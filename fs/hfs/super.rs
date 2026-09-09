/*
 *  linux/fs/hfs/super.c
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains hfs_read_super(), some of the super_ops and
 * init_hfs_fs() and exit_hfs_fs().  The remaining super_ops are in
 * inode.c since they deal with inodes.
 *
 * Based on the minix file system code, (C) 1991, 1992 by Linus Torvalds
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut hfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

MODULE_DESCRIPTION!("Apple Macintosh file system support");
MODULE_LICENSE!("GPL");

unsafe fn hfs_sync_fs(sb: *mut super_block, _wait: c_int) -> c_int {
    let ret;
    mutex_lock(&mut (*HFS_SB(sb)).mdb_lock);
    is_hfs_cnid_counts_valid(sb);
    ret = hfs_mdb_commit(sb);
    mutex_unlock(&mut (*HFS_SB(sb)).mdb_lock);
    ret
}

/* hfs_put_super() releases resources associated with the superblock. */
unsafe fn hfs_put_super(sb: *mut super_block) {
    cancel_delayed_work_sync(&mut (*HFS_SB(sb)).mdb_work);
    hfs_mdb_close(sb);
    /* release the MDB's resources */
    hfs_mdb_put(sb);
}

unsafe fn flush_mdb(work: *mut work_struct) {
    let sbi: *mut hfs_sb_info = container_of!(work, hfs_sb_info, mdb_work.work);
    let sb = (*sbi).sb;
    spin_lock(&mut (*sbi).work_lock);
    (*sbi).work_queued = 0;
    spin_unlock(&mut (*sbi).work_lock);
    mutex_lock(&mut (*sbi).mdb_lock);
    is_hfs_cnid_counts_valid(sb);
    hfs_mdb_commit(sb);
    mutex_unlock(&mut (*sbi).mdb_lock);
}

unsafe fn hfs_mark_mdb_dirty(sb: *mut super_block) {
    let sbi = HFS_SB(sb);
    let mut delay: c_ulong;
    if sb_rdonly(sb) { return; }
    spin_lock(&mut (*sbi).work_lock);
    if (*sbi).work_queued == 0 {
        delay = msecs_to_jiffies(dirty_writeback_interval * 10);
        queue_delayed_work(system_dfl_long_wq, &mut (*sbi).mdb_work, delay);
        (*sbi).work_queued = 1;
    }
    spin_unlock(&mut (*sbi).work_lock);
}

unsafe fn hfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let sb = (*dentry).d_sb;
    let id = huge_encode_dev((*(*sb).s_bdev).bd_dev);
    (*buf).f_type = HFS_SUPER_MAGIC;
    (*buf).f_bsize = (*sb).s_blocksize;
    (*buf).f_blocks = ((*HFS_SB(sb)).fs_ablocks as u32) * (*HFS_SB(sb)).fs_div;
    (*buf).f_bfree = ((*HFS_SB(sb)).free_ablocks as u32) * (*HFS_SB(sb)).fs_div;
    (*buf).f_bavail = (*buf).f_bfree;
    (*buf).f_files = (*HFS_SB(sb)).fs_ablocks;
    (*buf).f_ffree = (*HFS_SB(sb)).free_ablocks;
    (*buf).f_fsid = u64_to_fsid(id);
    (*buf).f_namelen = HFS_NAMELEN;
    0
}

unsafe fn hfs_reconfigure(fc: *mut fs_context) -> c_int {
    let sb = (*(*fc).root).d_sb;
    sync_filesystem(sb);
    (*fc).sb_flags |= SB_NODIRATIME;
    if (((*fc).sb_flags & SB_RDONLY) != 0) == sb_rdonly(sb) { return 0; }
    if ((*fc).sb_flags & SB_RDONLY) == 0 {
        if ((*HFS_SB(sb)).mdb.drAtrb & cpu_to_be16(HFS_SB_ATTRIB_UNMNT)) == 0 {
            pr_warn!("filesystem was not cleanly unmounted, running fsck.hfs is recommended.  leaving read-only.\n");
            (*sb).s_flags |= SB_RDONLY; (*fc).sb_flags |= SB_RDONLY;
        } else if ((*HFS_SB(sb)).mdb.drAtrb & cpu_to_be16(HFS_SB_ATTRIB_SLOCK)) != 0 {
            pr_warn!("filesystem is marked locked, leaving read-only.\n");
            (*sb).s_flags |= SB_RDONLY; (*fc).sb_flags |= SB_RDONLY;
        }
    }
    0
}

unsafe fn hfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let i = alloc_inode_sb(sb, hfs_inode_cachep, GFP_KERNEL);
    if !i.is_null() { &mut (*i).vfs_inode } else { core::ptr::null_mut() }
}
unsafe fn hfs_free_inode(inode: *mut inode) { kmem_cache_free(hfs_inode_cachep, HFS_I(inode)); }

static hfs_super_operations: super_operations = super_operations {
    alloc_inode: Some(hfs_alloc_inode), free_inode: Some(hfs_free_inode),
    write_inode: Some(hfs_write_inode), evict_inode: Some(hfs_evict_inode),
    put_super: Some(hfs_put_super), sync_fs: Some(hfs_sync_fs),
    statfs: Some(hfs_statfs), show_options: Some(hfs_show_options),
};

enum { opt_uid, opt_gid, opt_umask, opt_file_umask, opt_dir_umask,
       opt_part, opt_session, opt_type, opt_creator, opt_quiet,
       opt_codepage, opt_iocharset }

static hfs_param_spec: [fs_parameter_spec; 13] = [
    fsparam_u32!("uid", opt_uid), fsparam_u32!("gid", opt_gid),
    fsparam_u32oct!("umask", opt_umask), fsparam_u32oct!("file_umask", opt_file_umask),
    fsparam_u32oct!("dir_umask", opt_dir_umask), fsparam_u32!("part", opt_part),
    fsparam_u32!("session", opt_session), fsparam_string!("type", opt_type),
    fsparam_string!("creator", opt_creator), fsparam_flag!("quiet", opt_quiet),
    fsparam_string!("codepage", opt_codepage), fsparam_string!("iocharset", opt_iocharset),
    fsparam_empty!(),
];

/* hfs_parse_param() is called by the vfs to parse mount options. */
unsafe fn hfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let hsb = (*fc).s_fs_info as *mut hfs_sb_info;
    let mut result = fs_parse_result::default();
    if (*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE { return 0; }
    let opt = fs_parse(fc, &hfs_param_spec, param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        opt_uid => (*hsb).s_uid = result.uid,
        opt_gid => (*hsb).s_gid = result.gid,
        opt_umask => { (*hsb).s_file_umask = result.uint_32 as umode_t; (*hsb).s_dir_umask = result.uint_32 as umode_t; },
        opt_file_umask => (*hsb).s_file_umask = result.uint_32 as umode_t,
        opt_dir_umask => (*hsb).s_dir_umask = result.uint_32 as umode_t,
        opt_part => (*hsb).part = result.uint_32,
        opt_session => (*hsb).session = result.uint_32,
        opt_type | opt_creator => {
            if strlen((*param).string) != 4 { pr_err!("value requires a 4 character value\n"); return -EINVAL; }
            if opt == opt_type { memcpy(&mut (*hsb).s_type as *mut _ as *mut c_void, (*param).string as *const c_void, 4); }
            else { memcpy(&mut (*hsb).s_creator as *mut _ as *mut c_void, (*param).string as *const c_void, 4); }
        },
        opt_quiet => (*hsb).s_quiet = 1,
        opt_codepage => { if !(*hsb).nls_disk.is_null() { return -EINVAL; } (*hsb).nls_disk = load_nls((*param).string); if (*hsb).nls_disk.is_null() { return -EINVAL; } },
        opt_iocharset => { if !(*hsb).nls_io.is_null() { return -EINVAL; } (*hsb).nls_io = load_nls((*param).string); if (*hsb).nls_io.is_null() { return -EINVAL; } },
        _ => return -EINVAL,
    }
    0
}

// The remaining mount, context, inode-cache, and module lifecycle routines
// retain the C implementation's sequencing and call the corresponding kernel
// interfaces supplied by the surrounding HFS translation.
unsafe fn hfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_bdev(fc, hfs_fill_super) }
unsafe fn hfs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }
static hfs_context_ops: fs_context_operations = fs_context_operations { parse_param: Some(hfs_parse_param), get_tree: Some(hfs_get_tree), reconfigure: Some(hfs_reconfigure), free: Some(hfs_free_fc) };

MODULE_ALIAS_FS!("hfs");

unsafe fn hfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let sbi = HFS_SB(sb);
    let mut fd = core::mem::MaybeUninit::<hfs_find_data>::uninit();
    let mut rec = core::mem::MaybeUninit::<hfs_cat_rec>::uninit();
    let mut root_inode: *mut inode;
    let silent = (*fc).sb_flags & SB_SILENT;
    let mut res: c_int;
    atomic64_set(&mut (*sbi).file_count, 0); atomic64_set(&mut (*sbi).folder_count, 0); atomic64_set(&mut (*sbi).next_id, 0);
    if !(*sbi).nls_disk.is_null() && (*sbi).nls_io.is_null() { (*sbi).nls_io = load_nls_default(); }
    (*sbi).s_dir_umask &= 0o777; (*sbi).s_file_umask &= 0o577;
    spin_lock_init(&mut (*sbi).work_lock); INIT_DELAYED_WORK!(&mut (*sbi).mdb_work, flush_mdb);
    (*sbi).sb = sb; (*sb).s_op = &hfs_super_operations; (*sb).s_xattr = hfs_xattr_handlers;
    (*sb).s_flags |= SB_NOATIME | SB_NODIRATIME; mutex_init(&mut (*sbi).mdb_lock); mutex_init(&mut (*sbi).bitmap_lock);
    mutex_lock(&mut (*sbi).mdb_lock); res = hfs_mdb_get(sb); mutex_unlock(&mut (*sbi).mdb_lock);
    if res != 0 { if silent == 0 { pr_warn!("can't find a HFS filesystem on dev %s\n", hfs_mdb_name(sb)); } res = -EINVAL; hfs_mdb_put(sb); return res; }
    res = hfs_find_init((*HFS_SB(sb)).cat_tree, fd.as_mut_ptr()); if res != 0 { hfs_mdb_put(sb); return res; }
    res = hfs_cat_find_brec(sb, HFS_ROOT_CNID, fd.as_mut_ptr());
    if res == 0 { if (*fd.as_ptr()).entrylength != core::mem::size_of_val(&(*rec.as_ptr()).dir) { res = -EIO; } else { hfs_bnode_read((*fd.as_ptr()).bnode, rec.as_mut_ptr(), (*fd.as_ptr()).entryoffset, (*fd.as_ptr()).entrylength); if (*rec.as_ptr()).type_ != HFS_CDR_DIR { res = -EIO; } } }
    if res != 0 { hfs_find_exit(fd.as_mut_ptr()); hfs_mdb_put(sb); return res; }
    root_inode = hfs_iget(sb, &(*fd.as_ptr()).search_key.cat, rec.as_ptr()); hfs_find_exit(fd.as_mut_ptr());
    if root_inode.is_null() || is_bad_inode(root_inode) { if !root_inode.is_null() { iput(root_inode); } hfs_mdb_put(sb); return -EINVAL; }
    set_default_d_op(sb, &hfs_dentry_operations); (*sb).s_root = d_make_root(root_inode);
    if (*sb).s_root.is_null() { hfs_mdb_put(sb); return -ENOMEM; } 0
}

unsafe fn hfs_init_fs_context(fc: *mut fs_context) -> c_int {
    let hsb = kzalloc_obj::<hfs_sb_info>(); if hsb.is_null() { return -ENOMEM; }
    (*fc).s_fs_info = hsb as *mut c_void; (*fc).ops = &hfs_context_ops;
    if (*fc).purpose != FS_CONTEXT_FOR_RECONFIGURE { (*hsb).s_uid = current_uid(); (*hsb).s_gid = current_gid(); (*hsb).s_file_umask = 0o133; (*hsb).s_dir_umask = 0o022; (*hsb).s_type = cpu_to_be32(0x3f3f3f3f); (*hsb).s_creator = cpu_to_be32(0x3f3f3f3f); (*hsb).part = -1; (*hsb).session = -1; }
    0
}
unsafe fn hfs_kill_super(sb: *mut super_block) { let hsb = HFS_SB(sb); kill_block_super(sb); kfree(hsb); }
static mut hfs_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: "hfs", kill_sb: Some(hfs_kill_super), fs_flags: FS_REQUIRES_DEV, init_fs_context: Some(hfs_init_fs_context) };
unsafe fn hfs_init_once(p: *mut c_void) { inode_init_once(&mut (*((p as *mut hfs_inode_info))).vfs_inode); }
unsafe fn init_hfs_fs() -> c_int { hfs_inode_cachep = kmem_cache_create!("hfs_inode_cache", core::mem::size_of::<hfs_inode_info>(), SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT, hfs_init_once); if hfs_inode_cachep.is_null() { return -ENOMEM; } let err = register_filesystem(&mut hfs_fs_type); if err != 0 { kmem_cache_destroy(hfs_inode_cachep); } err }
unsafe fn exit_hfs_fs() { unregister_filesystem(&mut hfs_fs_type); rcu_barrier(); kmem_cache_destroy(hfs_inode_cachep); }
module_init!(init_hfs_fs); module_exit!(exit_hfs_fs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
