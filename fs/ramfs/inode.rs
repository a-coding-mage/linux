/*
 * Resizable simple ram filesystem for Linux.
 *
 * Copyright (C) 2000 Linus Torvalds.
 *               2000 Transmeta Corp.
 *
 * Usage limits added by David Gibson, Linuxcare Australia.
 * This file is released under the GPL.
 */

/*
 * NOTE! This filesystem is probably most useful
 * not as a real filesystem, but as an example of
 * how virtual filesystems can be written.
 *
 * It doesn't get much simpler than this. Consider
 * that this file implements the full semantics of
 * a POSIX-compliant read-write filesystem.
 *
 * Note in particular how the filesystem does not
 * need to implement any data structures of its own
 * to keep track of the virtual data: using the VFS
 * caches is sufficient.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct ramfs_mount_opts {
    pub mode: umode_t,
}

#[repr(C)]
pub struct ramfs_fs_info {
    pub mount_opts: ramfs_mount_opts,
}

pub const RAMFS_DEFAULT_MODE: umode_t = 0o755;

extern "C" {
    static ramfs_ops: struct_super_operations;
    static ramfs_dir_inode_operations: struct_inode_operations;
}

pub unsafe fn ramfs_get_inode(
    sb: *mut super_block,
    dir: *const inode,
    mode: umode_t,
    dev: dev_t,
) -> *mut inode {
    let inode = new_inode(sb);

    if !inode.is_null() {
        (*inode).i_ino = get_next_ino();
        inode_init_owner(&nop_mnt_idmap, inode, dir, mode);
        (*inode).i_mapping.a_ops = &ram_aops;
        mapping_set_gfp_mask((*inode).i_mapping, GFP_HIGHUSER);
        mapping_set_unevictable((*inode).i_mapping);
        simple_inode_init_ts(inode);
        match mode & S_IFMT {
            _ => init_special_inode(inode, mode, dev),
            S_IFREG => {
                (*inode).i_op = &ramfs_file_inode_operations;
                (*inode).i_fop = &ramfs_file_operations;
            }
            S_IFDIR => {
                (*inode).i_op = &ramfs_dir_inode_operations;
                (*inode).i_fop = &simple_dir_operations;
                // directory inodes start off with i_nlink == 2 (for "." entry)
                inc_nlink(inode);
            }
            S_IFLNK => {
                (*inode).i_op = &page_symlink_inode_operations;
                inode_nohighmem(inode);
            }
        }
    }
    inode
}

// File creation. Allocate an inode, and we're done..
// SMP-safe
unsafe fn ramfs_mknod(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
    dev: dev_t,
) -> c_int {
    let inode = ramfs_get_inode((*dir).i_sb, dir, mode, dev);
    let mut error = -ENOSPC;

    if !inode.is_null() {
        error = security_inode_init_security(inode, dir, &(*dentry).d_name, core::ptr::null_mut(), core::ptr::null_mut());
        if error != 0 {
            iput(inode);
            return error;
        }
        d_make_persistent(dentry, inode);
        error = 0;
        inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    }
    error
}

unsafe fn ramfs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let retval = ramfs_mknod(&nop_mnt_idmap, dir, dentry, mode, 0);
    if retval == 0 { inc_nlink(dir); }
    ERR_PTR(retval as isize)
}

unsafe fn ramfs_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    ramfs_mknod(&nop_mnt_idmap, dir, dentry, mode | S_IFREG, 0)
}

unsafe fn ramfs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const c_char) -> c_int {
    let inode = ramfs_get_inode((*dir).i_sb, dir, S_IFLNK | S_IRWXUGO, 0);
    let mut error = -ENOSPC;
    if !inode.is_null() {
        let l = strlen(symname) + 1;
        error = security_inode_init_security(inode, dir, &(*dentry).d_name, core::ptr::null_mut(), core::ptr::null_mut());
        if error != 0 { iput(inode); return error; }
        error = page_symlink(inode, symname, l);
        if error == 0 {
            d_make_persistent(dentry, inode);
            inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
        } else { iput(inode); }
    }
    error
}

unsafe fn ramfs_tmpfile(idmap: *mut mnt_idmap, dir: *mut inode, file: *mut file, mode: umode_t) -> c_int {
    let inode = ramfs_get_inode((*dir).i_sb, dir, mode, 0);
    if inode.is_null() { return -ENOSPC; }
    let mut error = security_inode_init_security(inode, dir, &(*file_dentry(file)).d_name, core::ptr::null_mut(), core::ptr::null_mut());
    if error != 0 { iput(inode); } else { d_tmpfile(file, inode); }
    finish_open_simple(file, error)
}

#[repr(C)]
static ramfs_dir_inode_operations: struct_inode_operations = struct_inode_operations {
    create: Some(ramfs_create), lookup: Some(simple_lookup), link: Some(simple_link),
    unlink: Some(simple_unlink), symlink: Some(ramfs_symlink), mkdir: Some(ramfs_mkdir),
    rmdir: Some(simple_rmdir), mknod: Some(ramfs_mknod), rename: Some(simple_rename),
    tmpfile: Some(ramfs_tmpfile),
};

unsafe fn ramfs_show_options(m: *mut seq_file, root: *mut dentry) -> c_int {
    let fsi = (*(*root).d_sb).s_fs_info as *mut ramfs_fs_info;
    if (*fsi).mount_opts.mode != RAMFS_DEFAULT_MODE { seq_printf(m, ",mode=%o", (*fsi).mount_opts.mode); }
    0
}

static ramfs_ops: struct_super_operations = struct_super_operations {
    statfs: Some(simple_statfs), drop_inode: Some(inode_just_drop), show_options: Some(ramfs_show_options),
};

pub enum ramfs_param { Opt_mode }

pub static ramfs_fs_parameters: [fs_parameter_spec; 2] = [
    fsparam_u32oct("mode", Opt_mode), fs_parameter_spec {},
];

unsafe fn ramfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut result = fs_parse_result {};
    let fsi = (*fc).s_fs_info as *mut ramfs_fs_info;
    let mut opt = fs_parse(fc, ramfs_fs_parameters.as_ptr(), param, &mut result);
    if opt == -ENOPARAM { opt = vfs_parse_fs_param_source(fc, param); if opt != -ENOPARAM { return opt; } return 0; }
    if opt < 0 { return opt; }
    match opt { Opt_mode => (*fsi).mount_opts.mode = result.uint_32 & S_IALLUGO, }
    0
}

unsafe fn ramfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let fsi = (*sb).s_fs_info as *mut ramfs_fs_info;
    (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*sb).s_blocksize = PAGE_SIZE; (*sb).s_blocksize_bits = PAGE_SHIFT;
    (*sb).s_magic = RAMFS_MAGIC; (*sb).s_op = &ramfs_ops; (*sb).s_d_flags = DCACHE_DONTCACHE; (*sb).s_time_gran = 1;
    let inode = ramfs_get_inode(sb, core::ptr::null(), S_IFDIR | (*fsi).mount_opts.mode, 0);
    (*sb).s_root = d_make_root(inode);
    if (*sb).s_root.is_null() { return -ENOMEM; }
    0
}

unsafe fn ramfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_nodev(fc, Some(ramfs_fill_super)) }
unsafe fn ramfs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }

static ramfs_context_ops: fs_context_operations = fs_context_operations {
    free: Some(ramfs_free_fc), parse_param: Some(ramfs_parse_param), get_tree: Some(ramfs_get_tree),
};

pub unsafe fn ramfs_init_fs_context(fc: *mut fs_context) -> c_int {
    let fsi = kzalloc_obj::<ramfs_fs_info>();
    if fsi.is_null() { return -ENOMEM; }
    (*fsi).mount_opts.mode = RAMFS_DEFAULT_MODE; (*fc).s_fs_info = fsi as *mut c_void; (*fc).ops = &ramfs_context_ops;
    0
}

pub unsafe fn ramfs_kill_sb(sb: *mut super_block) { kfree((*sb).s_fs_info); kill_anon_super(sb); }

static mut ramfs_fs_type: file_system_type = file_system_type {
    name: "ramfs", init_fs_context: Some(ramfs_init_fs_context), parameters: ramfs_fs_parameters.as_ptr(),
    kill_sb: Some(ramfs_kill_sb), fs_flags: FS_USERNS_MOUNT,
};

unsafe fn init_ramfs_fs() -> c_int { register_filesystem(&mut ramfs_fs_type) }
fs_initcall!(init_ramfs_fs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
