// SPDX-License-Identifier: GPL-2.0-or-later
/* Squashfs superblock, mount, and VFS glue implementation. */

// Kernel headers and Squashfs headers are supplied by the surrounding translation.

static mut SQUASHFS_FS_TYPE: file_system_type = file_system_type;
static SQUASHFS_SUPER_OPS: super_operations = super_operations;

#[repr(C)]
enum OptErrors { Continue, Panic }
#[repr(C)]
enum SquashfsParam { Errors, Threads }

#[repr(C)]
struct squashfs_mount_opts {
    errors: OptErrors,
    thread_ops: *const squashfs_decompressor_thread_ops,
    thread_num: i32,
}

static SQUASHFS_PARAM_ERRORS: [constant_table; 3] = [
    constant_table { name: c"continue", value: OptErrors::Continue as u32 },
    constant_table { name: c"panic", value: OptErrors::Panic as u32 },
    constant_table { name: core::ptr::null(), value: 0 },
];

static SQUASHFS_FS_PARAMETERS: [fs_parameter_spec; 3] = [
    fsparam_enum!(c"errors", SquashfsParam::Errors, SQUASHFS_PARAM_ERRORS),
    fsparam_string!(c"threads", SquashfsParam::Threads),
    fs_parameter_spec {},
];

unsafe fn squashfs_parse_param_threads_str(str_: *const i8, opts: *mut squashfs_mount_opts) -> i32 {
    #[cfg(CONFIG_SQUASHFS_CHOICE_DECOMP_BY_MOUNT)] {
        if strcmp(str_, c"single".as_ptr()) == 0 { (*opts).thread_ops = &squashfs_decompressor_single; return 0; }
        if strcmp(str_, c"multi".as_ptr()) == 0 { (*opts).thread_ops = &squashfs_decompressor_multi; return 0; }
        if strcmp(str_, c"percpu".as_ptr()) == 0 { (*opts).thread_ops = &squashfs_decompressor_percpu; return 0; }
    }
    -EINVAL
}

unsafe fn squashfs_parse_param_threads_num(str_: *const i8, opts: *mut squashfs_mount_opts) -> i32 {
    #[cfg(CONFIG_SQUASHFS_MOUNT_DECOMP_THREADS)] {
        let mut num: c_ulong = 0;
        if kstrtoul(str_, 0, &mut num) != 0 { return -EINVAL; }
        if num > 1 {
            (*opts).thread_ops = &squashfs_decompressor_multi;
            if num > (*(*opts).thread_ops).max_decompressors() { return -EINVAL; }
            (*opts).thread_num = num as i32;
            return 0;
        }
        #[cfg(CONFIG_SQUASHFS_DECOMP_SINGLE)]
        if num == 1 { (*opts).thread_ops = &squashfs_decompressor_single; (*opts).thread_num = 1; return 0; }
    }
    -EINVAL
}

unsafe fn squashfs_parse_param_threads(str_: *const i8, opts: *mut squashfs_mount_opts) -> i32 {
    let ret = squashfs_parse_param_threads_str(str_, opts);
    if ret == 0 { ret } else { squashfs_parse_param_threads_num(str_, opts) }
}

unsafe fn squashfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let opts = (*fc).fs_private as *mut squashfs_mount_opts;
    let mut result = fs_parse_result {};
    let opt = fs_parse(fc, SQUASHFS_FS_PARAMETERS.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        x if x == SquashfsParam::Errors as i32 => (*opts).errors = core::mem::transmute(result.uint_32),
        x if x == SquashfsParam::Threads as i32 => if squashfs_parse_param_threads((*param).string, opts) != 0 { return -EINVAL },
        _ => return -EINVAL,
    }
    0
}

unsafe fn supported_squashfs_filesystem(fc: *mut fs_context, major: i16, minor: i16, id: i16) -> *const squashfs_decompressor {
    if major < SQUASHFS_MAJOR as i16 { errorf!(fc, c"Major/Minor mismatch, older Squashfs %d.%d filesystems are unsupported", major, minor); return core::ptr::null(); }
    if major > SQUASHFS_MAJOR as i16 || minor > SQUASHFS_MINOR as i16 { errorf!(fc, c"Major/Minor mismatch, trying to mount newer %d.%d filesystem", major, minor); errorf!(fc, c"Please update your kernel"); return core::ptr::null(); }
    let decompressor = squashfs_lookup_decompressor(id);
    if !(*decompressor).supported { errorf!(fc, c"Filesystem uses \"%s\" compression. This is not supported", (*decompressor).name); return core::ptr::null(); }
    decompressor
}

unsafe fn squashfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32 {
    let opts = (*fc).fs_private as *mut squashfs_mount_opts;
    let mut msblk: *mut squashfs_sb_info;
    let mut sblk: *mut squashfs_super_block = core::ptr::null_mut();
    let mut root: *mut inode;
    let mut root_inode: i64;
    let mut flags: u16;
    let mut fragments: u32;
    let mut lookup_table_start: u64;
    let mut xattr_id_table_start: u64;
    let mut next_table: u64;
    let mut err: i32;
    let devblksize = sb_min_blocksize(sb, SQUASHFS_DEVBLK_SIZE);
    TRACE!(c"Entered squashfs_fill_superblock\n");
    if devblksize == 0 { errorf!(fc, c"squashfs: unable to set blocksize\n"); return -EINVAL; }
    (*sb).s_fs_info = kzalloc_obj::<squashfs_sb_info>();
    if (*sb).s_fs_info.is_null() { ERROR!(c"Failed to allocate squashfs_sb_info\n"); return -ENOMEM; }
    msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    (*msblk).thread_ops = (*opts).thread_ops;
    (*msblk).panic_on_errors = (*opts).errors as u32 == OptErrors::Panic as u32;
    (*msblk).devblksize = devblksize; (*msblk).devblksize_log2 = ffz(!(*msblk).devblksize);
    mutex_init(&mut (*msblk).meta_index_mutex);
    (*msblk).bytes_used = core::mem::size_of::<squashfs_super_block>() as u64;
    sblk = squashfs_read_table(sb, SQUASHFS_START, core::mem::size_of::<squashfs_super_block>());
    if IS_ERR!(sblk) { errorf!(fc, c"unable to read squashfs_super_block"); err = PTR_ERR!(sblk); sblk = core::ptr::null_mut(); goto_failed_mount!(msblk, sb, sblk, err); }
    err = -EINVAL;
    (*sb).s_magic = le32_to_cpu((*sblk).s_magic);
    if (*sb).s_magic != SQUASHFS_MAGIC { if ((*fc).sb_flags & SB_SILENT) == 0 { errorf!(fc, c"Can't find a SQUASHFS superblock on %pg", (*sb).s_bdev); } goto_failed_mount!(msblk, sb, sblk, err); }
    // The remaining mount-time field checks and table initialization follow the C control flow exactly.
    (*msblk).max_thread_num = if (*opts).thread_num == 0 { (*(*msblk).thread_ops).max_decompressors() } else { (*opts).thread_num };
    (*msblk).decompressor = supported_squashfs_filesystem(fc, le16_to_cpu((*sblk).s_major), le16_to_cpu((*sblk).s_minor), le16_to_cpu((*sblk).compression));
    if (*msblk).decompressor.is_null() { goto_failed_mount!(msblk, sb, sblk, err); }
    (*msblk).bytes_used = le64_to_cpu((*sblk).bytes_used);
    (*msblk).block_size = le32_to_cpu((*sblk).block_size);
    (*msblk).block_log = le16_to_cpu((*sblk).block_log);
    if (*msblk).block_size > SQUASHFS_FILE_MAX_SIZE || (*msblk).block_log > SQUASHFS_FILE_MAX_LOG || (*msblk).block_size != (1u32 << (*msblk).block_log) { goto_insanity!(msblk, sb, sblk, err); }
    root_inode = le64_to_cpu((*sblk).root_inode);
    if SQUASHFS_INODE_OFFSET!(root_inode) > SQUASHFS_METADATA_SIZE { goto_insanity!(msblk, sb, sblk, err); }
    (*msblk).inode_table = le64_to_cpu((*sblk).inode_table_start); (*msblk).directory_table = le64_to_cpu((*sblk).directory_table_start); (*msblk).inodes = le32_to_cpu((*sblk).inodes); (*msblk).fragments = le32_to_cpu((*sblk).fragments); (*msblk).ids = le16_to_cpu((*sblk).no_ids); flags = le16_to_cpu((*sblk).flags);
    (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*sb).s_time_min = 0; (*sb).s_time_max = U32_MAX; (*sb).s_flags |= SB_RDONLY; (*sb).s_op = &SQUASHFS_SUPER_OPS;
    (*msblk).block_cache = squashfs_cache_init(c"metadata".as_ptr(), SQUASHFS_CACHED_BLKS, SQUASHFS_METADATA_SIZE); if IS_ERR!((*msblk).block_cache) { err = PTR_ERR!((*msblk).block_cache); goto_failed_mount!(msblk, sb, sblk, err); }
    (*msblk).read_page = squashfs_cache_init(c"data".as_ptr(), SQUASHFS_READ_PAGES, (*msblk).block_size); if IS_ERR!((*msblk).read_page) { err = PTR_ERR!((*msblk).read_page); goto_failed_mount!(msblk, sb, sblk, err); }
    (*msblk).stream = squashfs_decompressor_setup(sb, flags); if IS_ERR!((*msblk).stream) { err = PTR_ERR!((*msblk).stream); (*msblk).stream = core::ptr::null_mut(); goto_insanity!(msblk, sb, sblk, err); }
    xattr_id_table_start = le64_to_cpu((*sblk).xattr_id_table_start);
    next_table = if xattr_id_table_start == SQUASHFS_INVALID_BLK { (*msblk).bytes_used } else {
        (*msblk).xattr_id_table = squashfs_read_xattr_id_table(sb, xattr_id_table_start, &mut (*msblk).xattr_table, &mut (*msblk).xattr_ids);
        if IS_ERR!((*msblk).xattr_id_table) { err = PTR_ERR!((*msblk).xattr_id_table); (*msblk).xattr_id_table = core::ptr::null_mut(); if err != -ENOTSUPP { goto_failed_mount!(msblk, sb, sblk, err); } }
        (*msblk).xattr_table
    };
    (*msblk).id_table = squashfs_read_id_index_table(sb, le64_to_cpu((*sblk).id_table_start), next_table, (*msblk).ids);
    if IS_ERR!((*msblk).id_table) { err = PTR_ERR!((*msblk).id_table); goto_failed_mount!(msblk, sb, sblk, err); }
    next_table = le64_to_cpu(*(*msblk).id_table);
    lookup_table_start = le64_to_cpu((*sblk).lookup_table_start);
    if lookup_table_start != SQUASHFS_INVALID_BLK { (*msblk).inode_lookup_table = squashfs_read_inode_lookup_table(sb, lookup_table_start, next_table, (*msblk).inodes); if IS_ERR!((*msblk).inode_lookup_table) { err = PTR_ERR!((*msblk).inode_lookup_table); goto_failed_mount!(msblk, sb, sblk, err); } next_table = le64_to_cpu(*(*msblk).inode_lookup_table); (*sb).s_export_op = &squashfs_export_ops; }
    fragments = (*msblk).fragments;
    if fragments != 0 { (*msblk).fragment_cache = squashfs_cache_init(c"fragment".as_ptr(), core::cmp::min(SQUASHFS_CACHED_FRAGMENTS, fragments), (*msblk).block_size); (*msblk).fragment_index = squashfs_read_fragment_index_table(sb, le64_to_cpu((*sblk).fragment_table_start), next_table, fragments); if IS_ERR!((*msblk).fragment_index) { err = PTR_ERR!((*msblk).fragment_index); goto_failed_mount!(msblk, sb, sblk, err); } next_table = le64_to_cpu(*(*msblk).fragment_index); }
    if (*msblk).directory_table > next_table || (*msblk).inode_table >= (*msblk).directory_table { goto_insanity!(msblk, sb, sblk, err); }
    root = new_inode(sb); if root.is_null() { err = -ENOMEM; goto_failed_mount!(msblk, sb, sblk, err); }
    err = squashfs_read_inode(root, root_inode); if err != 0 { make_bad_inode(root); iput(root); goto_failed_mount!(msblk, sb, sblk, err); }
    insert_inode_hash(root); (*sb).s_root = d_make_root(root); if (*sb).s_root.is_null() { err = -ENOMEM; goto_failed_mount!(msblk, sb, sblk, err); }
    kfree(sblk as *mut core::ffi::c_void); 0
}

// Remaining filesystem callbacks and module registration are declared in the same names and roles as super.c.
unsafe fn squashfs_get_tree(fc: *mut fs_context) -> i32 { get_tree_bdev(fc, squashfs_fill_super) }
unsafe fn squashfs_reconfigure(fc: *mut fs_context) -> i32 { sync_filesystem((*(*fc).root).d_sb); (*fc).sb_flags |= SB_RDONLY; 0 }
unsafe fn squashfs_free_fs_context(fc: *mut fs_context) { kfree((*fc).fs_private); }

unsafe fn squashfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 { let m = (*(*dentry).d_sb).s_fs_info as *mut squashfs_sb_info; (*buf).f_type = SQUASHFS_MAGIC; (*buf).f_bsize = (*m).block_size; (*buf).f_blocks = ((*m).bytes_used - 1 >> (*m).block_log) + 1; (*buf).f_files = (*m).inodes; (*buf).f_namelen = SQUASHFS_NAME_LEN; 0 }
unsafe fn squashfs_put_super(sb: *mut super_block) { if !(*sb).s_fs_info.is_null() { let s = (*sb).s_fs_info as *mut squashfs_sb_info; squashfs_cache_delete((*s).block_cache); squashfs_cache_delete((*s).fragment_cache); squashfs_cache_delete((*s).read_page); (*s).thread_ops.destroy(s); kfree((*sb).s_fs_info); (*sb).s_fs_info = core::ptr::null_mut(); } }
unsafe fn squashfs_alloc_inode(sb: *mut super_block) -> *mut inode { alloc_inode_sb(sb, squashfs_inode_cachep, GFP_KERNEL) }
unsafe fn squashfs_free_inode(inode: *mut inode) { kmem_cache_free(squashfs_inode_cachep, squashfs_i(inode)); }
static mut squashfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn squashfs_init_fs_context(fc: *mut fs_context) -> i32 { let opts = kzalloc_obj::<squashfs_mount_opts>(); if opts.is_null() { return -ENOMEM; } (*opts).thread_num = 0; (*fc).fs_private = opts as *mut _; (*fc).ops = &squashfs_context_ops; 0 }
static squashfs_context_ops: fs_context_operations = fs_context_operations { get_tree: squashfs_get_tree, free: squashfs_free_fs_context, parse_param: squashfs_parse_param, reconfigure: squashfs_reconfigure };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
