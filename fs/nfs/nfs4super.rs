// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012 Bryan Schumaker <bjschuma@netapp.com>
 */
// Linux kernel dependencies from the original C translation unit are supplied externally.

const NFSDBG_FACILITY: i32 = NFSDBG_VFS;

extern "C" {
    fn nfs4_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32;
    fn nfs4_evict_inode(inode: *mut inode);
}

static NFS4_SOPS: super_operations = super_operations {
    alloc_inode: Some(nfs_alloc_inode),
    free_inode: Some(nfs_free_inode),
    write_inode: Some(nfs4_write_inode),
    drop_inode: Some(nfs_drop_inode),
    statfs: Some(nfs_statfs),
    evict_inode: Some(nfs4_evict_inode),
    umount_begin: Some(nfs_umount_begin),
    show_options: Some(nfs_show_options),
    show_devname: Some(nfs_show_devname),
    show_path: Some(nfs_show_path),
    show_stats: Some(nfs_show_stats),
};

static mut NFS_V4: nfs_subversion = nfs_subversion {
    owner: THIS_MODULE,
    nfs_fs: &nfs4_fs_type,
    rpc_vers: &nfs_version4,
    rpc_ops: &nfs_v4_clientops,
    sops: &NFS4_SOPS,
    xattr: nfs4_xattr_handlers,
};

unsafe fn nfs4_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32 {
    let mut ret = nfs_write_inode(inode, wbc);
    if ret == 0 {
        ret = pnfs_layoutcommit_inode(inode, (*wbc).sync_mode == WB_SYNC_ALL);
    }
    ret
}

/* Clean out any remaining NFSv4 state left by failed nfs_atomic_lookup calls. */
unsafe fn nfs4_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data);
    clear_inode(inode);
    /* If we are holding a delegation, return and free it. */
    nfs_inode_evict_delegation(inode);
    /* The delegreturn above triggers pnfs return-on-close. */
    pnfs_return_layout(inode);
    pnfs_destroy_layout_final(NFS_I(inode));
    /* First call standard NFS clear_inode() code. */
    nfs_clear_inode(inode);
    nfs4_xattr_cache_zap(inode);
}

#[repr(C)]
struct nfs_referral_count {
    list: list_head,
    task: *const task_struct,
    referral_count: u32,
}

static mut NFS_REFERRAL_COUNT_LIST: list_head = LIST_HEAD_INIT;
static mut NFS_REFERRAL_COUNT_LIST_LOCK: spinlock_t = SPINLOCK_INIT;

unsafe fn nfs_find_referral_count() -> *mut nfs_referral_count {
    let mut p: *mut nfs_referral_count = core::ptr::null_mut();
    list_for_each_entry!(p, &mut NFS_REFERRAL_COUNT_LIST, list, nfs_referral_count);
    if !p.is_null() && (*p).task == current {
        return p;
    }
    core::ptr::null_mut()
}

const NFS_MAX_NESTED_REFERRALS: u32 = 2;

unsafe fn nfs_referral_loop_protect() -> i32 {
    let mut p: *mut nfs_referral_count;
    let mut new = kmalloc_obj::<nfs_referral_count>();
    let mut ret = -ENOMEM;
    if new.is_null() { return ret; }
    (*new).task = current;
    (*new).referral_count = 1;
    ret = 0;
    spin_lock(&mut NFS_REFERRAL_COUNT_LIST_LOCK);
    p = nfs_find_referral_count();
    if !p.is_null() {
        if (*p).referral_count >= NFS_MAX_NESTED_REFERRALS { ret = -ELOOP; }
        else { (*p).referral_count += 1; }
    } else {
        list_add(&mut (*new).list, &mut NFS_REFERRAL_COUNT_LIST);
        new = core::ptr::null_mut();
    }
    spin_unlock(&mut NFS_REFERRAL_COUNT_LIST_LOCK);
    kfree(new);
    ret
}

unsafe fn nfs_referral_loop_unprotect() {
    let mut p: *mut nfs_referral_count;
    spin_lock(&mut NFS_REFERRAL_COUNT_LIST_LOCK);
    p = nfs_find_referral_count();
    (*p).referral_count -= 1;
    if (*p).referral_count == 0 { list_del(&mut (*p).list); }
    else { p = core::ptr::null_mut(); }
    spin_unlock(&mut NFS_REFERRAL_COUNT_LIST_LOCK);
    kfree(p);
}

unsafe fn do_nfs4_mount(server: *mut nfs_server, fc: *mut fs_context, hostname: *const c_char, export_path: *const c_char) -> i32 {
    if IS_ERR(server) { return PTR_ERR(server); }
    let root_fc = vfs_dup_fs_context(fc);
    if IS_ERR(root_fc) { nfs_free_server(server); return PTR_ERR(root_fc); }
    kfree((*root_fc).source); (*root_fc).source = core::ptr::null_mut();
    let ctx = nfs_fc2context(fc); let root_ctx = nfs_fc2context(root_fc);
    (*root_ctx).internal = true; (*root_ctx).server = server;
    if !(*ctx).fscache_uniq.is_null() {
        let ret = vfs_parse_fs_string(root_fc, c"fsc".as_ptr(), (*ctx).fscache_uniq);
        if ret < 0 { put_fs_context(root_fc); return ret; }
    }
    let source = if strchr(hostname, b':' as c_int) != core::ptr::null() {
        kasprintf(GFP_KERNEL, c"[%s]:/".as_ptr(), hostname)
    } else { kasprintf(GFP_KERNEL, c"%s:/".as_ptr(), hostname) };
    if source.is_null() { put_fs_context(root_fc); return -ENOMEM; }
    let mut ret = vfs_parse_fs_string(root_fc, c"source".as_ptr(), source);
    kfree(source); if ret < 0 { put_fs_context(root_fc); return ret; }
    let root_mnt = fc_mount(root_fc); put_fs_context(root_fc);
    if IS_ERR(root_mnt) { return PTR_ERR(root_mnt); }
    ret = nfs_referral_loop_protect();
    if ret != 0 { mntput(root_mnt); return ret; }
    let dentry = mount_subtree(root_mnt, export_path);
    nfs_referral_loop_unprotect();
    if IS_ERR(dentry) { return PTR_ERR(dentry); }
    (*fc).root = dentry; 0
}

pub unsafe fn nfs4_try_get_tree(fc: *mut fs_context) -> i32 {
    let ctx = nfs_fc2context(fc);
    dfprintk!(MOUNT, "--> nfs4_try_get_tree()\n");
    let err = do_nfs4_mount(nfs4_create_server(fc), fc, (*ctx).nfs_server.hostname, (*ctx).nfs_server.export_path);
    if err != 0 { nfs_ferrorf!(fc, MOUNT, "NFS4: Couldn't follow remote path"); dfprintk!(MOUNT, "<-- nfs4_try_get_tree() = %d [error]\n", err); }
    else { dfprintk!(MOUNT, "<-- nfs4_try_get_tree() = 0\n"); }
    err
}

/* Create an NFS4 server record on referral traversal. */
pub unsafe fn nfs4_get_referral_tree(fc: *mut fs_context) -> i32 {
    let ctx = nfs_fc2context(fc);
    dprintk!("--> nfs4_referral_mount()\n");
    let err = do_nfs4_mount(nfs4_create_referral_server(fc), fc, (*ctx).nfs_server.hostname, (*ctx).nfs_server.export_path);
    if err != 0 { nfs_ferrorf!(fc, MOUNT, "NFS4: Couldn't follow remote path"); dfprintk!(MOUNT, "<-- nfs4_get_referral_tree() = %d [error]\n", err); }
    else { dfprintk!(MOUNT, "<-- nfs4_get_referral_tree() = 0\n"); }
    err
}

unsafe fn init_nfs_v4() -> i32 {
    let mut err = nfs_dns_resolver_init(); if err != 0 { return err; }
    err = nfs_idmap_init(); if err != 0 { nfs_dns_resolver_destroy(); return err; }
    // CONFIG_NFS_V4_2: nfs4_xattr_cache_init() and nfs42_ssc_register_ops() are conditional.
    err = nfs4_register_sysctl(); if err != 0 { nfs_idmap_quit(); nfs_dns_resolver_destroy(); return err; }
    register_nfs_version(&mut NFS_V4); 0
}

unsafe fn exit_nfs_v4() {
    /* Not called in the _init(), conditionally loaded. */
    nfs4_pnfs_v3_ds_connect_unload(); unregister_nfs_version(&mut NFS_V4);
    // CONFIG_NFS_V4_2: nfs4_xattr_cache_exit() and nfs42_ssc_unregister_ops() are conditional.
    nfs4_unregister_sysctl(); nfs_idmap_quit(); nfs_dns_resolver_destroy();
}

// MODULE_DESCRIPTION("NFSv4 client support");
// MODULE_LICENSE("GPL");
// module_init(init_nfs_v4);
// module_exit(exit_nfs_v4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
