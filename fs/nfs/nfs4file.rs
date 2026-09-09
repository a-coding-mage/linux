// SPDX-License-Identifier: GPL-2.0
/* Direct translation of linux/fs/nfs/file.c NFSv4 implementation. */

const NFSDBG_FACILITY: u32 = NFSDBG_FILE;

unsafe fn nfs4_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut ctx: *mut nfs_open_context;
    let dentry = file_dentry(filp);
    let mut parent: *mut dentry = core::ptr::null_mut();
    let dir: *mut inode;
    let mut openflags = (*filp).f_flags;
    let mut attr: iattr = core::mem::zeroed();
    let mut err: c_int;

    dprintk!("NFS: open file(%pd2)\n", dentry);
    err = nfs_check_flags(openflags);
    if err != 0 { return err; }
    openflags &= !(O_CREAT | O_EXCL);
    parent = dget_parent(dentry);
    dir = d_inode(parent);
    ctx = alloc_nfs_open_context(file_dentry(filp), flags_to_mode(openflags), filp);
    err = PTR_ERR(ctx);
    if IS_ERR(ctx) { goto out; }
    attr.ia_valid = ATTR_OPEN;
    if (openflags & O_TRUNC) != 0 {
        attr.ia_valid |= ATTR_SIZE;
        attr.ia_size = 0;
        filemap_write_and_wait((*inode).i_mapping);
    }
    inode = (*NFS_PROTO(dir)).open_context(dir, ctx, openflags, &mut attr, core::ptr::null_mut());
    if IS_ERR(inode) {
        err = PTR_ERR(inode);
        match err { -ENOENT | -ESTALE | -EISDIR | -ENOTDIR | -ELOOP => goto out_drop, _ => goto out_put_ctx }
    }
    if inode != d_inode(dentry) { goto out_drop; }
    nfs_file_set_open_context(filp, ctx);
    nfs_fscache_open_file(inode, filp);
    err = 0;
    (*filp).f_mode |= FMODE_CAN_ODIRECT;
    if test_bit(NFS_CONTEXT_O_DIRECT, &(*ctx).flags) != 0 { (*filp).f_flags |= O_DIRECT; }
out_put_ctx:
    put_nfs_open_context(ctx);
out:
    dput(parent);
    return err;
out_drop:
    d_drop(dentry);
    err = -EOPENSTALE;
    goto out_put_ctx;
}

unsafe fn nfs4_file_flush(file: *mut file, _id: fl_owner_t) -> c_int {
    let inode = file_inode(file);
    let since: errseq_t;
    dprintk!("NFS: flush(%pD2)\n", file);
    nfs_inc_stats(inode, NFSIOS_VFSFLUSH);
    if ((*file).f_mode & FMODE_WRITE) == 0 { return 0; }
    if !nfs4_delegation_flush_on_close(inode) { return filemap_fdatawrite((*file).f_mapping); }
    since = filemap_sample_wb_err((*file).f_mapping);
    nfs_wb_all(inode);
    filemap_check_wb_err((*file).f_mapping, since)
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn __nfs4_copy_file_range(file_in: *mut file, pos_in: loff_t, file_out: *mut file, pos_out: loff_t, count: size_t, _flags: c_uint) -> ssize_t {
    let mut cn_resp: *mut nfs42_copy_notify_res = core::ptr::null_mut();
    let mut nss: *mut nl4_server = core::ptr::null_mut();
    let mut cnrs: *mut nfs4_stateid = core::ptr::null_mut();
    let mut sync = false;
    if (*file_in).f_op != &nfs4_file_operations { return -EXDEV as ssize_t; }
    if !nfs_server_capable(file_inode(file_out), NFS_CAP_COPY) || !nfs_server_capable(file_inode(file_in), NFS_CAP_COPY) { return -EOPNOTSUPP as ssize_t; }
    if file_inode(file_in) == file_inode(file_out) { return -EOPNOTSUPP as ssize_t; }
    if count <= 2 * (*NFS_SERVER(file_inode(file_in))).rsize { sync = true; }
retry:
    if !nfs42_files_from_same_server(file_in, file_out) {
        if sync { return -EOPNOTSUPP as ssize_t; }
        cn_resp = kzalloc_obj::<nfs42_copy_notify_res>();
        if cn_resp.is_null() { return -ENOMEM as ssize_t; }
        let mut ret = nfs42_proc_copy_notify(file_in, file_out, cn_resp);
        if ret != 0 { ret = -EOPNOTSUPP; goto out; }
        nss = &mut (*cn_resp).cnr_src;
        cnrs = &mut (*cn_resp).cnr_stateid;
    }
    let ret = nfs42_proc_copy(file_in, pos_in, file_out, pos_out, count, nss, cnrs, sync);
out:
    kfree(cn_resp as *mut c_void);
    if ret == -EAGAIN { goto retry; }
    ret
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs4_copy_file_range(a: *mut file, b: loff_t, c: *mut file, d: loff_t, e: size_t, f: c_uint) -> ssize_t {
    let mut ret = __nfs4_copy_file_range(a,b,c,d,e,f);
    if ret == -EOPNOTSUPP as ssize_t || ret == -EXDEV as ssize_t { ret = splice_copy_file_range(a,b,c,d,e); }
    ret
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs4_file_llseek(filep: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    match whence { SEEK_HOLE | SEEK_DATA => { let ret = nfs42_proc_llseek(filep, offset, whence); if ret != -EOPNOTSUPP { ret } else { nfs_file_llseek(filep, offset, whence) } }, _ => nfs_file_llseek(filep, offset, whence) }
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs42_fallocate(filep: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    let inode = file_inode(filep);
    if !S_ISREG((*inode).i_mode) { return -EOPNOTSUPP as c_long; }
    match mode { 0 | (FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE) | FALLOC_FL_ZERO_RANGE => (), _ => return -EOPNOTSUPP as c_long }
    let ret = inode_newsize_ok(inode, offset + len); if ret < 0 { return ret as c_long; }
    if mode & FALLOC_FL_PUNCH_HOLE != 0 { nfs42_proc_deallocate(filep, offset, len) }
    else if mode & FALLOC_FL_ZERO_RANGE != 0 { nfs42_proc_zero_range(filep, offset, len) }
    else { nfs42_proc_allocate(filep, offset, len) }
}

unsafe fn nfs4_setlease(file: *mut file, arg: c_int, lease: *mut *mut file_lease, priv_: *mut *mut c_void) -> c_int { nfs4_proc_setlease(file, arg, lease, priv_) }

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs42_remap_file_range(src_file: *mut file, src_off: loff_t, dst_file: *mut file, dst_off: loff_t, count: loff_t, remap_flags: c_uint) -> loff_t {
    let dst_inode = file_inode(dst_file); let src_inode = file_inode(src_file); let bs = (*NFS_SERVER(dst_inode)).clone_blksize; let mut ret = -EINVAL;
    if remap_flags & REMAP_FILE_DEDUP != 0 { return -EOPNOTSUPP as loff_t; }
    if remap_flags & !REMAP_FILE_ADVISORY != 0 { return -EINVAL as loff_t; }
    if IS_SWAPFILE(dst_inode) || IS_SWAPFILE(src_inode) { return -ETXTBSY as loff_t; }
    if bs != 0 { if !IS_ALIGNED(src_off, bs) || !IS_ALIGNED(dst_off, bs) { return ret as loff_t; } if !IS_ALIGNED(count, bs) && i_size_read(src_inode) != src_off + count { return ret as loff_t; } }
    lock_two_nondirectories(src_inode, dst_inode); nfs_file_block_o_direct(NFS_I(src_inode)); ret = nfs_sync_inode(src_inode); if ret != 0 { goto out_unlock; }
    nfs_file_block_o_direct(NFS_I(dst_inode)); ret = nfs_sync_inode(dst_inode); if ret != 0 { goto out_unlock; }
    ret = nfs42_proc_clone(src_file, dst_file, src_off, dst_off, count);
    if ret == 0 { truncate_inode_pages_range(&mut (*dst_inode).i_data, dst_off, dst_off + count - 1); }
out_unlock: unlock_two_nondirectories(src_inode, dst_inode); if ret < 0 { ret as loff_t } else { count }
}

#[cfg(CONFIG_NFS_V4_2)]
static mut read_name_gen: c_int = 1;
#[cfg(CONFIG_NFS_V4_2)]
const SSC_READ_NAME_BODY: &[u8] = b"ssc_read_%d\0";

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn __nfs42_ssc_close(filep: *mut file) { (*nfs_file_open_context(filep)).state.as_mut().unwrap().flags = 0; }

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn __nfs42_ssc_open(_ss_mnt: *mut vfsmount, _src_fh: *mut nfs_fh, _stateid: *mut nfs4_stateid) -> *mut file { unimplemented!("direct translation requires external SSC/kernel declarations") }

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs42_ssc_register_ops() { nfs42_ssc_register(&nfs4_ssc_clnt_ops_tbl); }
#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs42_ssc_unregister_ops() { nfs42_ssc_unregister(&nfs4_ssc_clnt_ops_tbl); }

#[cfg(CONFIG_NFS_V4_2)]
static nfs4_ssc_clnt_ops_tbl: nfs4_ssc_client_ops = nfs4_ssc_client_ops { sco_open: Some(__nfs42_ssc_open), sco_close: Some(__nfs42_ssc_close) };

#[no_mangle]
pub static nfs4_file_operations: file_operations = file_operations {
    read_iter: Some(nfs_file_read), write_iter: Some(nfs_file_write), mmap_prepare: Some(nfs_file_mmap_prepare),
    open: Some(nfs4_file_open), flush: Some(nfs4_file_flush), release: Some(nfs_file_release), fsync: Some(nfs_file_fsync),
    lock: Some(nfs_lock), flock: Some(nfs_flock), splice_read: Some(nfs_file_splice_read), splice_write: Some(iter_file_splice_write),
    check_flags: Some(nfs_check_flags), setlease: Some(nfs4_setlease),
    #[cfg(CONFIG_NFS_V4_2)] copy_file_range: Some(nfs4_copy_file_range),
    #[cfg(CONFIG_NFS_V4_2)] llseek: Some(nfs4_file_llseek),
    #[cfg(CONFIG_NFS_V4_2)] fallocate: Some(nfs42_fallocate),
    #[cfg(CONFIG_NFS_V4_2)] remap_file_range: Some(nfs42_remap_file_range),
    #[cfg(not(CONFIG_NFS_V4_2))] llseek: Some(nfs_file_llseek),
    fop_flags: FOP_DONTCACHE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
