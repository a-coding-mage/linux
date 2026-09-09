// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

// Dependencies supplied by protocol.h, orangefs-kernel.h, orangefs-bufmap.h,
// linux/hashtable.h, and linux/seq_file.h remain external.

/* a cache for orangefs-inode objects (i.e. orangefs inode private data) */
static mut orangefs_inode_cache: *mut kmem_cache = core::ptr::null_mut();

/* list for storing orangefs specific superblocks in use */
static mut orangefs_superblocks: list_head = list_head::new();

static mut orangefs_superblocks_lock: spinlock_t = spinlock_t::new();

enum {
    Opt_acl,
    Opt_intr,
    Opt_local_lock,
}

const orangefs_fs_param_spec: [fs_parameter_spec; 4] = [
    fsparam_flag!("acl", Opt_acl),
    fsparam_flag!("intr", Opt_intr),
    fsparam_flag!("local_lock", Opt_local_lock),
    fs_parameter_spec::default(),
];

static mut orangefs_features: u64 = 0;

static orangefs_s_ops: super_operations = super_operations {
    alloc_inode: Some(orangefs_alloc_inode), free_inode: Some(orangefs_free_inode),
    destroy_inode: Some(orangefs_destroy_inode), write_inode: Some(orangefs_write_inode),
    drop_inode: Some(inode_just_drop), statfs: Some(orangefs_statfs),
    show_options: Some(orangefs_show_options), ..super_operations::default()
};
static orangefs_export_ops: export_operations = export_operations {
    encode_fh: Some(orangefs_encode_fh), fh_to_dentry: Some(orangefs_fh_to_dentry),
    ..export_operations::default()
};

unsafe fn orangefs_show_options(m: *mut seq_file, root: *mut dentry) -> i32 {
    let orangefs_sb = ORANGEFS_SB((*(*root).d_sb));
    if (*(*root).d_sb).s_flags & SB_POSIXACL != 0 { seq_puts(m, ",acl"); }
    if (*orangefs_sb).flags & ORANGEFS_OPT_INTR != 0 { seq_puts(m, ",intr"); }
    if (*orangefs_sb).flags & ORANGEFS_OPT_LOCAL_LOCK != 0 { seq_puts(m, ",local_lock"); }
    0
}

unsafe fn orangefs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let orangefs_sb = (*fc).s_fs_info as *mut orangefs_sb_info_s;
    let mut result = fs_parse_result::default();
    let opt = fs_parse(fc, orangefs_fs_param_spec.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        Opt_acl => (*fc).sb_flags |= SB_POSIXACL,
        Opt_intr => (*orangefs_sb).flags |= ORANGEFS_OPT_INTR,
        Opt_local_lock => (*orangefs_sb).flags |= ORANGEFS_OPT_LOCAL_LOCK,
        _ => {}
    }
    0
}

unsafe fn orangefs_inode_cache_ctor(req: *mut core::ffi::c_void) {
    let orangefs_inode = req as *mut orangefs_inode_s;
    inode_init_once(&mut (*orangefs_inode).vfs_inode);
    init_rwsem(&mut (*orangefs_inode).xattr_sem);
}

unsafe fn orangefs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let orangefs_inode = alloc_inode_sb(sb, orangefs_inode_cache, GFP_KERNEL);
    if orangefs_inode.is_null() { return core::ptr::null_mut(); }
    /* We want to clear everything except for rw_semaphore and the vfs_inode. */
    core::ptr::write_bytes(&mut (*orangefs_inode).refn.khandle as *mut _, 0, 16);
    (*orangefs_inode).refn.fs_id = ORANGEFS_FS_ID_NULL;
    (*orangefs_inode).last_failed_block_index_read = 0;
    core::ptr::write_bytes((*orangefs_inode).link_target.as_mut_ptr(), 0,
                           (*orangefs_inode).link_target.len());
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_alloc_inode: allocated %p\n", &(*orangefs_inode).vfs_inode);
    &mut (*orangefs_inode).vfs_inode
}

unsafe fn orangefs_free_inode(inode: *mut inode) {
    let orangefs_inode = ORANGEFS_I(inode);
    let mut i: i32 = 0;
    let mut tmp: *mut hlist_node = core::ptr::null_mut();
    let mut cx: *mut orangefs_cached_xattr = core::ptr::null_mut();
    hash_for_each_safe!((*orangefs_inode).xattr_cache, i, tmp, cx, node, {
        hlist_del(&mut (*cx).node);
        kfree(cx as *mut core::ffi::c_void);
    });
    kmem_cache_free(orangefs_inode_cache, orangefs_inode as *mut core::ffi::c_void);
}

unsafe fn orangefs_destroy_inode(inode: *mut inode) {
    let orangefs_inode = ORANGEFS_I(inode);
    gossip_debug(GOSSIP_SUPER_DEBUG, "%s: deallocated %p destroying inode %pU\n",
                 __func__, orangefs_inode, get_khandle_from_ino(inode));
}

unsafe fn orangefs_write_inode(inode: *mut inode, _wbc: *mut writeback_control) -> i32 {
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_write_inode\n");
    orangefs_inode_setattr(inode)
}

/* NOTE: information filled in here is typically reflected in the output of the system command 'df' */
unsafe fn orangefs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let mut ret = -ENOMEM;
    let mut new_op: *mut orangefs_kernel_op_s = core::ptr::null_mut();
    let mut flags = 0;
    let sb = (*dentry).d_sb;
    gossip_debug(GOSSIP_SUPER_DEBUG, "%s: called on sb %p (fs_id is %d)\n", __func__, sb, (*ORANGEFS_SB(sb)).fs_id as i32);
    new_op = op_alloc(ORANGEFS_VFS_OP_STATFS);
    if new_op.is_null() { return ret; }
    (*new_op).upcall.req.statfs.fs_id = (*ORANGEFS_SB(sb)).fs_id;
    if (*ORANGEFS_SB(sb)).flags & ORANGEFS_OPT_INTR != 0 { flags = ORANGEFS_OP_INTERRUPTIBLE; }
    ret = service_operation(new_op, "orangefs_statfs", flags);
    if (*new_op).downcall.status < 0 { op_release(new_op); return ret; }
    gossip_debug(GOSSIP_SUPER_DEBUG, "%s: got %ld blocks available | %ld blocks total | %ld block size | %ld files total | %ld files avail\n", __func__, (*new_op).downcall.resp.statfs.blocks_avail as i64, (*new_op).downcall.resp.statfs.blocks_total as i64, (*new_op).downcall.resp.statfs.block_size as i64, (*new_op).downcall.resp.statfs.files_total as i64, (*new_op).downcall.resp.statfs.files_avail as i64);
    (*buf).f_type = (*sb).s_magic;
    (*buf).f_fsid.val[0] = (*ORANGEFS_SB(sb)).fs_id;
    (*buf).f_fsid.val[1] = (*ORANGEFS_SB(sb)).id;
    (*buf).f_bsize = (*new_op).downcall.resp.statfs.block_size;
    (*buf).f_namelen = ORANGEFS_NAME_MAX;
    (*buf).f_blocks = (*new_op).downcall.resp.statfs.blocks_total as sector_t;
    (*buf).f_bfree = (*new_op).downcall.resp.statfs.blocks_avail as sector_t;
    (*buf).f_bavail = (*new_op).downcall.resp.statfs.blocks_avail as sector_t;
    (*buf).f_files = (*new_op).downcall.resp.statfs.files_total as sector_t;
    (*buf).f_ffree = (*new_op).downcall.resp.statfs.files_avail as sector_t;
    (*buf).f_frsize = 0;
    op_release(new_op);
    gossip_debug(GOSSIP_SUPER_DEBUG, "%s: returning %d\n", __func__, ret);
    ret
}

/* Remount as initiated by VFS layer. We just need to reparse the mount options. */
unsafe fn orangefs_reconfigure(fc: *mut fs_context) -> i32 {
    let orangefs_sb = ORANGEFS_SB((*(*fc).root).d_sb);
    let revised = (*fc).s_fs_info as *mut orangefs_sb_info_s;
    let mut flags = (*orangefs_sb).flags;
    flags &= !(ORANGEFS_OPT_INTR | ORANGEFS_OPT_LOCAL_LOCK);
    flags |= (*revised).flags;
    WRITE_ONCE!((*orangefs_sb).flags, flags);
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_reconfigure: called\n");
    0
}

unsafe fn orangefs_remount(orangefs_sb: *mut orangefs_sb_info_s) -> i32 {
    let mut ret = -EINVAL;
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_remount: called\n");
    let mut new_op = op_alloc(ORANGEFS_VFS_OP_FS_MOUNT);
    if new_op.is_null() { return -ENOMEM; }
    strscpy((*new_op).upcall.req.fs_mount.orangefs_config_server.as_mut_ptr(), (*orangefs_sb).devname.as_ptr());
    gossip_debug(GOSSIP_SUPER_DEBUG, "Attempting ORANGEFS Remount via host %s\n", (*new_op).upcall.req.fs_mount.orangefs_config_server.as_ptr());
    ret = service_operation(new_op, "orangefs_remount", ORANGEFS_OP_PRIORITY | ORANGEFS_OP_NO_MUTEX);
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_remount: mount got return value of %d\n", ret);
    if ret == 0 { (*orangefs_sb).id = (*new_op).downcall.resp.fs_mount.id; (*orangefs_sb).mount_pending = 0; }
    op_release(new_op);
    if orangefs_userspace_version >= 20906 {
        new_op = op_alloc(ORANGEFS_VFS_OP_FEATURES);
        if new_op.is_null() { return -ENOMEM; }
        (*new_op).upcall.req.features.features = 0;
        ret = service_operation(new_op, "orangefs_features", ORANGEFS_OP_PRIORITY | ORANGEFS_OP_NO_MUTEX);
        orangefs_features = if ret == 0 { (*new_op).downcall.resp.features.features } else { 0 };
        op_release(new_op);
    } else { orangefs_features = 0; }
    ret
}

unsafe fn fsid_key_table_initialize() -> i32 { 0 }
unsafe fn fsid_key_table_finalize() {}

unsafe fn orangefs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if fh_len < 5 || fh_type > 2 { return core::ptr::null_mut(); }
    let mut refn: orangefs_object_kref = core::mem::zeroed();
    ORANGEFS_khandle_from!(&mut refn.khandle, (*fid).raw.as_ptr(), 16);
    refn.fs_id = (*fid).raw[4] as u32;
    gossip_debug(GOSSIP_SUPER_DEBUG, "fh_to_dentry: handle %pU, fs_id %d\n", &refn.khandle, refn.fs_id);
    d_obtain_alias(orangefs_iget(sb, &refn))
}

unsafe fn orangefs_encode_fh(inode: *mut inode, fh: *mut u32, max_len: *mut i32, parent: *mut inode) -> i32 {
    let len = if parent.is_null() { 5 } else { 10 };
    let mut ty = 1;
    let mut refn = (*ORANGEFS_I(inode)).refn;
    if *max_len < len { gossip_err!("fh buffer is too small for encoding\n"); *max_len = len; return 255; }
    ORANGEFS_khandle_to!(&mut refn.khandle, fh, 16); (*fh.add(4)) = refn.fs_id;
    gossip_debug(GOSSIP_SUPER_DEBUG, "Encoding fh: handle %pU, fsid %u\n", &refn.khandle, refn.fs_id);
    if !parent.is_null() { refn = (*ORANGEFS_I(parent)).refn; ORANGEFS_khandle_to!(&mut refn.khandle, (fh as *mut u8).add(20), 16); *fh.add(9) = refn.fs_id; ty = 2; gossip_debug(GOSSIP_SUPER_DEBUG, "Encoding parent: handle %pU, fsid %u\n", &refn.khandle, refn.fs_id); }
    *max_len = len; ty
}

unsafe fn orangefs_unmount(id: i32, fs_id: i32, devname: *const i8) -> i32 {
    let op = op_alloc(ORANGEFS_VFS_OP_FS_UMOUNT); if op.is_null() { return -ENOMEM; }
    (*op).upcall.req.fs_umount.id = id; (*op).upcall.req.fs_umount.fs_id = fs_id;
    strscpy((*op).upcall.req.fs_umount.orangefs_config_server.as_mut_ptr(), devname);
    let r = service_operation(op, "orangefs_fs_umount", 0);
    if r != 0 { gossip_err!("orangefs_unmount: service_operation %d\n", r); }
    op_release(op); r
}

unsafe fn orangefs_fill_sb(sb: *mut super_block, _fc: *mut fs_context, fs_mount: *mut orangefs_fs_mount_response) -> i32 {
    (*ORANGEFS_SB(sb)).sb = sb;
    (*ORANGEFS_SB(sb)).root_khandle = (*fs_mount).root_khandle;
    (*ORANGEFS_SB(sb)).fs_id = (*fs_mount).fs_id;
    (*ORANGEFS_SB(sb)).id = (*fs_mount).id;
    (*sb).s_xattr = orangefs_xattr_handlers;
    (*sb).s_magic = ORANGEFS_SUPER_MAGIC;
    (*sb).s_op = &orangefs_s_ops;
    set_default_d_op(sb, &orangefs_dentry_operations);
    (*sb).s_blocksize = PAGE_SIZE;
    (*sb).s_blocksize_bits = PAGE_SHIFT;
    (*sb).s_maxbytes = MAX_LFS_FILESIZE;
    let ret = super_setup_bdi(sb); if ret != 0 { return ret; }
    let mut root_object: orangefs_object_kref = core::mem::zeroed();
    root_object.khandle = (*ORANGEFS_SB(sb)).root_khandle;
    root_object.fs_id = (*ORANGEFS_SB(sb)).fs_id;
    gossip_debug(GOSSIP_SUPER_DEBUG, "get inode %pU, fsid %d\n", &root_object.khandle, root_object.fs_id);
    let root = orangefs_iget(sb, &root_object); if IS_ERR(root) { return PTR_ERR(root); }
    gossip_debug(GOSSIP_SUPER_DEBUG, "Allocated root inode [%p] with mode %x\n", root, (*root).i_mode);
    let root_dentry = d_make_root(root); if root_dentry.is_null() { return -ENOMEM; }
    (*sb).s_export_op = &orangefs_export_ops; (*sb).s_root = root_dentry; 0
}

unsafe fn orangefs_get_tree(fc: *mut fs_context) -> i32 {
    if (*fc).source.is_null() { return invalf(fc, "Device name not specified.\n"); }
    let op = op_alloc(ORANGEFS_VFS_OP_FS_MOUNT); if op.is_null() { return -ENOMEM; }
    strscpy((*op).upcall.req.fs_mount.orangefs_config_server.as_mut_ptr(), (*fc).source);
    let mut ret = service_operation(op, "orangefs_mount", 0);
    if ret != 0 { op_release(op); return ret; }
    if (*op).downcall.resp.fs_mount.fs_id == ORANGEFS_FS_ID_NULL { op_release(op); return -EINVAL; }
    let sb = sget_fc(fc, core::ptr::null_mut(), set_anon_super_fc);
    if IS_ERR(sb) { ret = PTR_ERR(sb); orangefs_unmount((*op).downcall.resp.fs_mount.id, (*op).downcall.resp.fs_mount.fs_id, (*fc).source); op_release(op); return ret; }
    ret = orangefs_fill_sb(sb, fc, &mut (*op).downcall.resp.fs_mount);
    if ret != 0 { (*ORANGEFS_SB(sb)).no_list = 1; deactivate_locked_super(sb); op_release(op); return ret; }
    strscpy((*ORANGEFS_SB(sb)).devname.as_mut_ptr(), (*fc).source); (*ORANGEFS_SB(sb)).mount_pending = 0;
    spin_lock(&mut orangefs_superblocks_lock); list_add_tail(&mut (*ORANGEFS_SB(sb)).list, &mut orangefs_superblocks); spin_unlock(&mut orangefs_superblocks_lock);
    op_release(op); (*ORANGEFS_SB(sb)).no_list = 0;
    if orangefs_userspace_version >= 20906 { let fop = op_alloc(ORANGEFS_VFS_OP_FEATURES); if fop.is_null() { return -ENOMEM; } (*fop).upcall.req.features.features = 0; ret = service_operation(fop, "orangefs_features", 0); orangefs_features = (*fop).downcall.resp.features.features; op_release(fop); } else { orangefs_features = 0; }
    (*fc).root = dget((*sb).s_root); 0
}

// The remaining filesystem-context, superblock teardown, and inode-cache routines
// retain the same external kernel operations and ordering as the C implementation.
unsafe fn orangefs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }

static orangefs_context_ops: fs_context_operations = fs_context_operations {
    free: Some(orangefs_free_fc), parse_param: Some(orangefs_parse_param),
    get_tree: Some(orangefs_get_tree), reconfigure: Some(orangefs_reconfigure),
    ..fs_context_operations::default()
};

unsafe fn orangefs_init_fs_context(fc: *mut fs_context) -> i32 {
    let osi = kzalloc_obj::<orangefs_sb_info_s>(); if osi.is_null() { return -ENOMEM; }
    (*fc).sb_flags_mask &= !SB_POSIXACL; (*osi).flags &= !ORANGEFS_OPT_INTR; (*osi).flags &= !ORANGEFS_OPT_LOCAL_LOCK;
    (*fc).s_fs_info = osi as *mut _; (*fc).ops = &orangefs_context_ops; 0
}

unsafe fn orangefs_kill_sb(sb: *mut super_block) {
    gossip_debug(GOSSIP_SUPER_DEBUG, "orangefs_kill_sb: called\n");
    kill_anon_super(sb);
    if ORANGEFS_SB(sb).is_null() { mutex_lock(&mut orangefs_request_mutex); mutex_unlock(&mut orangefs_request_mutex); return; }
    let r = orangefs_unmount((*ORANGEFS_SB(sb)).id, (*ORANGEFS_SB(sb)).fs_id, (*ORANGEFS_SB(sb)).devname.as_ptr());
    if r == 0 { (*ORANGEFS_SB(sb)).mount_pending = 1; }
    if (*ORANGEFS_SB(sb)).no_list == 0 { spin_lock(&mut orangefs_superblocks_lock); __list_del_entry(&mut (*ORANGEFS_SB(sb)).list); (*ORANGEFS_SB(sb)).list.prev = core::ptr::null_mut(); spin_unlock(&mut orangefs_superblocks_lock); }
    mutex_lock(&mut orangefs_request_mutex); mutex_unlock(&mut orangefs_request_mutex); kfree(ORANGEFS_SB(sb) as *mut _);
}

unsafe fn orangefs_inode_cache_initialize() -> i32 {
    orangefs_inode_cache = kmem_cache_create_usercopy!("orangefs_inode_cache", core::mem::size_of::<orangefs_inode_s>(), 0, 0, offset_of!(orangefs_inode_s, link_target), size_of_field!(orangefs_inode_s, link_target), orangefs_inode_cache_ctor);
    if orangefs_inode_cache.is_null() { gossip_err!("Cannot create orangefs_inode_cache\n"); return -ENOMEM; } 0
}

unsafe fn orangefs_inode_cache_finalize() -> i32 { kmem_cache_destroy(orangefs_inode_cache); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
