// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dlmfs.c
 *
 * Code which implements the kernel side of a minimal userspace
 * interface to our DLM. This file handles the virtual file system
 * used for communication with userspace. Credit should go to ramfs,
 * which was a template for the fs side of this module.
 *
 * Copyright (C) 2003, 2004 Oracle.  All rights reserved.
 */

/* Simple VFS hooks based on: */
/* Resizable simple ram filesystem for Linux. */

/* Kernel/project dependencies supplied externally. */

const DLMFS_CAPABILITIES: &[u8] = b"bast stackglue\0";

static mut DLMFS_INODE_CACHE: *mut kmem_cache = core::ptr::null_mut();
pub static mut USER_DLM_WORKER: *mut workqueue_struct = core::ptr::null_mut();

/* These are the ABI capabilities of dlmfs. */
unsafe fn param_set_dlmfs_capabilities(
    _val: *const c_char,
    kp: *const kernel_param,
) -> c_int {
    printk!(KERN_ERR, "%s: readonly parameter\n", (*kp).name);
    -EINVAL
}

unsafe fn param_get_dlmfs_capabilities(
    buffer: *mut c_char,
    _kp: *const kernel_param,
) -> c_int {
    sysfs_emit!(buffer, DLMFS_CAPABILITIES)
}

/* module_param_call(capabilities, param_set_dlmfs_capabilities,
 *                   param_get_dlmfs_capabilities, NULL, 0444);
 * MODULE_PARM_DESC(capabilities, DLMFS_CAPABILITIES);
 */

/* decodes a set of open flags into a valid lock level and a set of flags. */
unsafe fn dlmfs_decode_open_flags(open_flags: c_int, level: *mut c_int, flags: *mut c_int) -> c_int {
    if open_flags & (O_WRONLY | O_RDWR) != 0 {
        *level = DLM_LOCK_EX;
    } else {
        *level = DLM_LOCK_PR;
    }
    *flags = 0;
    if open_flags & O_NONBLOCK != 0 {
        *flags |= DLM_LKF_NOQUEUE;
    }
    0
}

unsafe fn dlmfs_file_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut status: c_int;
    let mut level = 0;
    let mut flags = 0;
    let mut fp: *mut dlmfs_filp_private = core::ptr::null_mut();
    let ip: *mut dlmfs_inode_private;

    if S_ISDIR((*inode).i_mode) { BUG!(); }
    mlog!(0, "open called on inode %llu, flags 0x%x\n", (*inode).i_ino, (*file).f_flags);
    status = dlmfs_decode_open_flags((*file).f_flags, &mut level, &mut flags);
    if status < 0 { return status; }
    (*file).f_flags &= !O_APPEND;
    fp = kmalloc_obj!(dlmfs_filp_private, GFP_NOFS);
    if fp.is_null() { return -ENOMEM; }
    (*fp).fp_lock_level = level;
    ip = DLMFS_I!(inode);
    status = user_dlm_cluster_lock(&mut (*ip).ip_lockres, level, flags);
    if status < 0 {
        if flags & DLM_LKF_NOQUEUE != 0 && status == -EAGAIN { status = -ETXTBSY; }
        kfree!(fp);
        return status;
    }
    (*file).private_data = fp as *mut c_void;
    status
}

unsafe fn dlmfs_file_release(inode: *mut inode, file: *mut file) -> c_int {
    let ip = DLMFS_I!(inode);
    let fp = (*file).private_data as *mut dlmfs_filp_private;
    if S_ISDIR((*inode).i_mode) { BUG!(); }
    mlog!(0, "close called on inode %llu\n", (*inode).i_ino);
    if !fp.is_null() {
        let level = (*fp).fp_lock_level;
        if level != DLM_LOCK_IV { user_dlm_cluster_unlock(&mut (*ip).ip_lockres, level); }
        kfree!(fp);
        (*file).private_data = core::ptr::null_mut();
    }
    0
}

unsafe fn dlmfs_file_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    let inode = d_inode!(dentry);
    (*attr).ia_valid &= !ATTR_SIZE;
    let error = setattr_prepare(&nop_mnt_idmap, dentry, attr);
    if error != 0 { return error; }
    setattr_copy(&nop_mnt_idmap, inode, attr);
    mark_inode_dirty(inode);
    0
}

unsafe fn dlmfs_file_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let inode = file_inode!(file);
    let ip = DLMFS_I!(inode);
    let mut event: __poll_t = 0;
    poll_wait(file, &mut (*ip).ip_lockres.l_event, wait);
    spin_lock(&mut (*ip).ip_lockres.l_lock);
    if (*ip).ip_lockres.l_flags & USER_LOCK_BLOCKED != 0 { event = EPOLLIN | EPOLLRDNORM; }
    spin_unlock(&mut (*ip).ip_lockres.l_lock);
    event
}

unsafe fn dlmfs_file_read(file: *mut file, buf: *mut c_char, count: usize, ppos: *mut loff_t) -> isize {
    let mut lvb = [0u8; DLM_LVB_LEN];
    if user_dlm_read_lvb(file_inode!(file), lvb.as_mut_ptr() as *mut c_char) == 0 { return 0; }
    simple_read_from_buffer(buf, count, ppos, lvb.as_ptr() as *const c_void, lvb.len())
}

unsafe fn dlmfs_file_write(filp: *mut file, buf: *const c_char, mut count: usize, ppos: *mut loff_t) -> isize {
    let mut lvb_buf = [0u8; DLM_LVB_LEN];
    let inode = file_inode!(filp);
    mlog!(0, "inode %llu, count = %zu, *ppos = %llu\n", (*inode).i_ino, count, *ppos);
    if *ppos >= DLM_LVB_LEN as loff_t { return -ENOSPC; }
    if count > DLM_LVB_LEN - *ppos as usize { count = DLM_LVB_LEN - *ppos as usize; }
    if count == 0 { return 0; }
    let bytes_left = copy_from_user(lvb_buf.as_mut_ptr() as *mut c_void, buf, count);
    count -= bytes_left;
    if count != 0 { user_dlm_write_lvb(inode, lvb_buf.as_ptr() as *const c_char, count); }
    *ppos += count as loff_t;
    mlog!(0, "wrote %zu bytes\n", count);
    count as isize
}

unsafe fn dlmfs_init_once(foo: *mut c_void) {
    let ip = foo as *mut dlmfs_inode_private;
    (*ip).ip_conn = core::ptr::null_mut();
    (*ip).ip_parent = core::ptr::null_mut();
    inode_init_once(&mut (*ip).ip_vfs_inode);
}

unsafe fn dlmfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ip = alloc_inode_sb!(sb, DLMFS_INODE_CACHE, GFP_NOFS);
    if ip.is_null() { return core::ptr::null_mut(); }
    &mut (*ip).ip_vfs_inode
}

unsafe fn dlmfs_free_inode(inode: *mut inode) { kmem_cache_free(DLMFS_INODE_CACHE, DLMFS_I!(inode) as *mut c_void); }

unsafe fn dlmfs_evict_inode(inode: *mut inode) {
    let ip = DLMFS_I!(inode);
    let lockres = &mut (*ip).ip_lockres;
    clear_inode(inode);
    mlog!(0, "inode %llu\n", (*inode).i_ino);
    if S_ISREG((*inode).i_mode) {
        spin_lock(&mut lockres.l_lock);
        let teardown = (lockres.l_flags & USER_LOCK_IN_TEARDOWN) != 0;
        spin_unlock(&mut lockres.l_lock);
        if !teardown { let status = user_dlm_destroy_lock(lockres); if status < 0 { mlog_errno!(status); } }
        iput((*ip).ip_parent);
    } else {
        mlog!(0, "we're a directory, ip->ip_conn = 0x%p\n", (*ip).ip_conn);
        if !(*ip).ip_conn.is_null() { user_dlm_unregister((*ip).ip_conn); }
    }
    (*ip).ip_parent = core::ptr::null_mut();
    (*ip).ip_conn = core::ptr::null_mut();
}

unsafe fn dlmfs_get_root_inode(sb: *mut super_block) -> *mut inode {
    let inode = new_inode(sb);
    if !inode.is_null() {
        (*inode).i_ino = get_next_ino();
        inode_init_owner(&nop_mnt_idmap, inode, core::ptr::null_mut(), S_IFDIR | 0o755);
        simple_inode_init_ts(inode); inc_nlink(inode);
        (*inode).i_fop = &simple_dir_operations; (*inode).i_op = &dlmfs_root_inode_operations;
    }
    inode
}

unsafe fn dlmfs_get_inode(parent: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut inode {
    let inode = new_inode((*parent).i_sb);
    if inode.is_null() { return inode; }
    (*inode).i_ino = get_next_ino(); inode_init_owner(&nop_mnt_idmap, inode, parent, mode); simple_inode_init_ts(inode);
    let ip = DLMFS_I!(inode); (*ip).ip_conn = (*DLMFS_I!(parent)).ip_conn;
    match mode & S_IFMT {
        S_IFREG => { (*inode).i_op = &dlmfs_file_inode_operations; (*inode).i_fop = &dlmfs_file_operations; i_size_write(inode, DLM_LVB_LEN); user_dlm_lock_res_init(&mut (*ip).ip_lockres, dentry); (*ip).ip_parent = igrab(parent); BUG_ON!((*ip).ip_parent.is_null()); }
        S_IFDIR => { (*inode).i_op = &dlmfs_dir_inode_operations; (*inode).i_fop = &simple_dir_operations; inc_nlink(inode); }
        _ => BUG!(),
    }
    inode
}

unsafe fn dlmfs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let domain = &(*dentry).d_name;
    mlog!(0, "mkdir %.*s\n", domain.len, domain.name);
    if domain.len >= GROUP_NAME_MAX { mlog!(ML_ERROR, "invalid domain name for directory.\n"); return ERR_PTR!(-EINVAL); }
    let inode = dlmfs_get_inode(dir, dentry, mode);
    if inode.is_null() { mlog_errno!(-ENOMEM); return ERR_PTR!(-ENOMEM); }
    let ip = DLMFS_I!(inode); let conn = user_dlm_register(domain);
    if IS_ERR!(conn) { let status = PTR_ERR!(conn); mlog!(ML_ERROR, "Error %d could not register domain \"%.*s\"\n", status, domain.len, domain.name); iput(inode); return ERR_PTR!(status); }
    (*ip).ip_conn = conn; inc_nlink(dir); d_make_persistent(dentry, inode); dentry
}

unsafe fn dlmfs_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    let name = &(*dentry).d_name;
    mlog!(0, "create %.*s\n", name.len, name.name);
    if name.len >= USER_DLM_LOCK_ID_MAX_LEN || *name.name == b'$' as c_char { mlog!(ML_ERROR, "invalid lock name, %.*s\n", name.len, name.name); return -EINVAL; }
    let inode = dlmfs_get_inode(dir, dentry, mode | S_IFREG);
    if inode.is_null() { mlog_errno!(-ENOMEM); return -ENOMEM; }
    d_make_persistent(dentry, inode); 0
}

unsafe fn dlmfs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode!(dentry); mlog!(0, "unlink inode %llu\n", (*inode).i_ino);
    let status = user_dlm_destroy_lock(&mut (*DLMFS_I!(inode)).ip_lockres);
    if status < 0 { mlog!(ML_ERROR, "unlink %pd, error %d from destroy\n", dentry, status); return status; }
    simple_unlink(dir, dentry)
}

unsafe fn dlmfs_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> c_int {
    (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*sb).s_blocksize = PAGE_SIZE; (*sb).s_blocksize_bits = PAGE_SHIFT; (*sb).s_magic = DLMFS_MAGIC; (*sb).s_op = &dlmfs_ops;
    (*sb).s_root = d_make_root(dlmfs_get_root_inode(sb)); if (*sb).s_root.is_null() { return -ENOMEM; } 0
}

/* VFS operation tables; function pointers and structures are supplied by the kernel ABI. */
static dlmfs_file_operations: file_operations = file_operations { open: Some(dlmfs_file_open), release: Some(dlmfs_file_release), poll: Some(dlmfs_file_poll), read: Some(dlmfs_file_read), write: Some(dlmfs_file_write), llseek: Some(default_llseek) };
static dlmfs_dir_inode_operations: inode_operations = inode_operations { create: Some(dlmfs_create), lookup: Some(simple_lookup), unlink: Some(dlmfs_unlink) };
static dlmfs_root_inode_operations: inode_operations = inode_operations { lookup: Some(simple_lookup), mkdir: Some(dlmfs_mkdir), rmdir: Some(simple_rmdir) };
static dlmfs_file_inode_operations: inode_operations = inode_operations { getattr: Some(simple_getattr), setattr: Some(dlmfs_file_setattr) };
static dlmfs_ops: super_operations = super_operations { statfs: Some(simple_statfs), alloc_inode: Some(dlmfs_alloc_inode), free_inode: Some(dlmfs_free_inode), evict_inode: Some(dlmfs_evict_inode), drop_inode: Some(inode_just_drop) };

unsafe fn dlmfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_nodev(fc, dlmfs_fill_super) }
unsafe fn dlmfs_init_fs_context(fc: *mut fs_context) -> c_int { (*fc).ops = &dlmfs_context_ops; 0 }
static dlmfs_context_ops: fs_context_operations = fs_context_operations { get_tree: Some(dlmfs_get_tree) };
static mut dlmfs_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: b"ocfs2_dlmfs\0".as_ptr() as *const c_char, kill_sb: Some(kill_anon_super), init_fs_context: Some(dlmfs_init_fs_context) };

unsafe fn init_dlmfs_fs() -> c_int {
    let mut status: c_int;
    let mut cleanup_inode = false;
    let mut cleanup_worker = false;
    DLMFS_INODE_CACHE = kmem_cache_create!(b"dlmfs_inode_cache\0", core::mem::size_of::<dlmfs_inode_private>(), 0, SLAB_HWCACHE_ALIGN | SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, dlmfs_init_once);
    if DLMFS_INODE_CACHE.is_null() { return -ENOMEM; }
    cleanup_inode = true;
    USER_DLM_WORKER = alloc_workqueue!(b"user_dlm\0", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if USER_DLM_WORKER.is_null() { status = -ENOMEM; } else { cleanup_worker = true; user_dlm_set_locking_protocol(); status = register_filesystem(&mut dlmfs_fs_type); }
    if status != 0 { if cleanup_inode { kmem_cache_destroy(DLMFS_INODE_CACHE); } if cleanup_worker { destroy_workqueue(USER_DLM_WORKER); } } else { printk!("OCFS2 User DLM kernel interface loaded\n"); }
    status
}

unsafe fn exit_dlmfs_fs() {
    unregister_filesystem(&mut dlmfs_fs_type); destroy_workqueue(USER_DLM_WORKER); rcu_barrier(); kmem_cache_destroy(DLMFS_INODE_CACHE);
}

/* MODULE_AUTHOR("Oracle"); MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("OCFS2 DLM-Filesystem"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
