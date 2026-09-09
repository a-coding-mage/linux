// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 * Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// Dependencies supplied by the surrounding kernel/9p translation units.

static V9FS_SUPER_OPS: super_operations = super_operations {
    alloc_inode: Some(v9fs_alloc_inode), free_inode: Some(v9fs_free_inode),
    statfs: Some(simple_statfs), drop_inode: Some(v9fs_drop_inode),
    evict_inode: Some(v9fs_evict_inode), show_options: Some(v9fs_show_options),
    umount_begin: Some(v9fs_umount_begin), write_inode: Some(v9fs_write_inode),
};
static V9FS_SUPER_OPS_DOTL: super_operations = super_operations {
    alloc_inode: Some(v9fs_alloc_inode), free_inode: Some(v9fs_free_inode),
    statfs: Some(v9fs_statfs), drop_inode: Some(v9fs_drop_inode),
    evict_inode: Some(v9fs_evict_inode), show_options: Some(v9fs_show_options),
    umount_begin: Some(v9fs_umount_begin), write_inode: Some(v9fs_write_inode_dotl),
};

unsafe fn v9fs_fill_super(sb: *mut super_block) -> c_int {
    let mut ret: c_int;
    let v9ses: *mut v9fs_session_info = (*sb).s_fs_info as *mut v9fs_session_info;

    (*sb).s_maxbytes = MAX_LFS_FILESIZE;
    (*sb).s_blocksize_bits = fls((*v9ses).maxdata.wrapping_sub(1));
    (*sb).s_blocksize = 1u64 << (*sb).s_blocksize_bits;
    (*sb).s_magic = V9FS_MAGIC;
    if v9fs_proto_dotl(v9ses) {
        (*sb).s_op = &V9FS_SUPER_OPS_DOTL;
        if ((*v9ses).flags & V9FS_NO_XATTR) == 0 {
            (*sb).s_xattr = v9fs_xattr_handlers;
        }
    } else {
        (*sb).s_op = &V9FS_SUPER_OPS;
        (*sb).s_time_max = U32_MAX;
    }
    (*sb).s_time_min = 0;

    ret = super_setup_bdi(sb);
    if ret != 0 { return ret; }
    if !(*v9ses).cache {
        (*(*sb).s_bdi).ra_pages = 0;
        (*(*sb).s_bdi).io_pages = 0;
    } else {
        (*(*sb).s_bdi).ra_pages = (*v9ses).maxdata >> PAGE_SHIFT;
        (*(*sb).s_bdi).io_pages = (*v9ses).maxdata >> PAGE_SHIFT;
    }
    (*sb).s_flags |= SB_ACTIVE;
    // CONFIG_9P_FS_POSIX_ACL conditional retained from the C source.
    if ((*v9ses).flags & V9FS_ACL_MASK) == V9FS_POSIX_ACL { (*sb).s_flags |= SB_POSIXACL; }
    0
}

unsafe fn v9fs_get_tree(fc: *mut fs_context) -> c_int {
    let mut sb: *mut super_block = core::ptr::null_mut();
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut root: *mut dentry = core::ptr::null_mut();
    let mut v9ses: *mut v9fs_session_info = core::ptr::null_mut();
    let mut fid: *mut p9_fid;
    let mut retval: c_int = 0;
    p9_debug(P9_DEBUG_VFS, b"\n\0".as_ptr());
    v9ses = kzalloc_obj::<v9fs_session_info>();
    if v9ses.is_null() { return -ENOMEM; }
    fid = v9fs_session_init(v9ses, fc);
    if IS_ERR(fid) { retval = PTR_ERR(fid); goto free_session; }
    (*fc).s_fs_info = v9ses as *mut core::ffi::c_void;
    sb = sget_fc(fc, None, set_anon_super_fc);
    if IS_ERR(sb) { retval = PTR_ERR(sb); goto clunk_fid; }
    retval = v9fs_fill_super(sb);
    if retval != 0 { goto release_sb; }
    if (*v9ses).cache & (CACHE_META | CACHE_LOOSE) != 0 {
        set_default_d_op(sb, &v9fs_cached_dentry_operations);
    } else {
        set_default_d_op(sb, &v9fs_dentry_operations);
        (*sb).s_d_flags |= DCACHE_DONTCACHE;
    }
    inode = v9fs_get_new_inode_from_fid(v9ses, fid, sb);
    if IS_ERR(inode) { retval = PTR_ERR(inode); goto release_sb; }
    root = d_make_root(inode);
    if root.is_null() { retval = -ENOMEM; goto release_sb; }
    (*sb).s_root = root;
    retval = v9fs_get_acl(inode, fid);
    if retval != 0 { goto release_sb; }
    v9fs_fid_add(root, &mut fid);
    p9_debug(P9_DEBUG_VFS, b" simple set mount, return 0\n\0".as_ptr());
    (*fc).root = dget((*sb).s_root);
    return 0;
clunk_fid:
    p9_fid_put(fid); v9fs_session_close(v9ses);
free_session:
    kfree(v9ses as *mut core::ffi::c_void); return retval;
release_sb:
    p9_fid_put(fid); deactivate_locked_super(sb); retval
}

unsafe fn v9fs_kill_super(s: *mut super_block) {
    let v9ses = (*s).s_fs_info as *mut v9fs_session_info;
    p9_debug(P9_DEBUG_VFS, b" %p\0".as_ptr(), s);
    kill_anon_super(s); v9fs_session_cancel(v9ses); v9fs_session_close(v9ses);
    kfree(v9ses as *mut core::ffi::c_void); (*s).s_fs_info = core::ptr::null_mut();
    p9_debug(P9_DEBUG_VFS, b"exiting kill_super\n\0".as_ptr());
}

unsafe fn v9fs_umount_begin(sb: *mut super_block) { v9fs_session_begin_cancel((*sb).s_fs_info as *mut v9fs_session_info); }

unsafe fn v9fs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let fid = v9fs_fid_lookup(dentry); if IS_ERR(fid) { return PTR_ERR(fid); }
    let v9ses = v9fs_dentry2v9ses(dentry); let mut rs = p9_rstatfs::zeroed();
    let mut res = if v9fs_proto_dotl(v9ses) { p9_client_statfs(fid, &mut rs) } else { -ENOSYS };
    if res == 0 { (*buf).f_type=rs.type_; (*buf).f_bsize=rs.bsize; (*buf).f_blocks=rs.blocks; (*buf).f_bfree=rs.bfree; (*buf).f_bavail=rs.bavail; (*buf).f_files=rs.files; (*buf).f_ffree=rs.ffree; (*buf).f_fsid=u64_to_fsid(rs.fsid); (*buf).f_namelen=rs.namelen; }
    if res == -ENOSYS { res = simple_statfs(dentry, buf); }
    p9_fid_put(fid); res
}

unsafe fn v9fs_drop_inode(inode: *mut inode) -> c_int {
    let v9ses=v9fs_inode2v9ses(inode); if (*v9ses).cache & (CACHE_META|CACHE_LOOSE) != 0 { inode_generic_drop(inode) } else { 1 }
}
unsafe fn v9fs_write_inode(inode:*mut inode,wbc:*mut writeback_control)->c_int { p9_debug(P9_DEBUG_VFS,b"%s: inode %p\n\0".as_ptr(),__func__,inode); netfs_unpin_writeback(inode,wbc) }
unsafe fn v9fs_write_inode_dotl(inode:*mut inode,wbc:*mut writeback_control)->c_int { p9_debug(P9_DEBUG_VFS,b"%s: inode %p\n\0".as_ptr(),__func__,inode); netfs_unpin_writeback(inode,wbc) }

static V9FS_CONTEXT_OPS: fs_context_operations = fs_context_operations { parse_param: Some(v9fs_parse_param), get_tree: Some(v9fs_get_tree), free: Some(v9fs_free_fc) };

unsafe fn v9fs_free_fc(fc:*mut fs_context) { let ctx=(*fc).fs_private as *mut v9fs_context; if ctx.is_null(){return;} kfree((*ctx).session_opts.uname); kfree((*ctx).session_opts.aname); if !(*ctx).client_opts.trans_mod.is_null(){v9fs_put_trans((*ctx).client_opts.trans_mod);} kfree(ctx as *mut core::ffi::c_void); }
unsafe fn v9fs_init_fs_context(fc:*mut fs_context)->c_int { let ctx=kzalloc_obj::<v9fs_context>(); if ctx.is_null(){return -ENOMEM;} (*fc).ops=&V9FS_CONTEXT_OPS; (*fc).fs_private=ctx as *mut core::ffi::c_void; (*ctx).session_opts.afid=!0; (*ctx).session_opts.cache=CACHE_NONE; (*ctx).session_opts.session_lock_timeout=P9_LOCK_TIMEOUT; (*ctx).session_opts.uname=kstrdup(V9FS_DEFUSER,GFP_KERNEL); if (*ctx).session_opts.uname.is_null(){(*fc).need_free=1;return -ENOMEM;} (*ctx).session_opts.aname=kstrdup(V9FS_DEFANAME,GFP_KERNEL); if (*ctx).session_opts.aname.is_null(){(*fc).need_free=1;return -ENOMEM;} (*ctx).session_opts.uid=INVALID_UID; (*ctx).session_opts.dfltuid=V9FS_DEFUID; (*ctx).session_opts.dfltgid=V9FS_DEFGID; (*ctx).session_opts.ndentry_timeout_ms=0; (*ctx).client_opts.proto_version=p9_proto_2000L; (*ctx).client_opts.msize=DEFAULT_MSIZE; (*ctx).fd_opts.port=P9_FD_PORT; (*ctx).fd_opts.rfd=!0; (*ctx).fd_opts.wfd=!0; (*ctx).fd_opts.privport=false; (*ctx).rdma_opts.port=P9_RDMA_PORT; (*ctx).rdma_opts.sq_depth=P9_RDMA_SQ_DEPTH; (*ctx).rdma_opts.rq_depth=P9_RDMA_RQ_DEPTH; (*ctx).rdma_opts.timeout=P9_RDMA_TIMEOUT; (*ctx).rdma_opts.privport=false; 0 }

#[no_mangle]
static mut v9fs_fs_type: file_system_type = file_system_type {
    name: b"9p\0".as_ptr(), kill_sb: Some(v9fs_kill_super), owner: THIS_MODULE,
    fs_flags: FS_RENAME_DOES_D_MOVE, init_fs_context: Some(v9fs_init_fs_context),
    parameters: v9fs_param_spec,
};
// MODULE_ALIAS_FS("9p")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
