// SPDX-License-Identifier: GPL-2.0
/*
 * Process version 2 NFSACL requests.
 *
 * Copyright (C) 2002-2003 Andreas Gruenbacher <agruen@suse.de>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// nfsd.h, linux/nfsacl.h, linux/gfp.h, cache.h, xdr3.h, and vfs.h.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PROC;

/* NULL call. */
unsafe fn nfsacld_proc_null(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

/* Get the Access and/or Default ACL of a file. */
unsafe fn nfsacld_proc_getacl(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_getaclargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_getaclres;
    let mut acl: *mut posix_acl;
    let inode: *mut inode;
    let fh: *mut svc_fh;

    dprintk(c"nfsd: GETACL(2acl)   %s\n", SVCFH_fmt(&(*argp).fh));
    fh = fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_NOP);
    if (*resp).status != nfs_ok { return rpc_success; }
    inode = d_inode((*fh).fh_dentry);
    if ((*argp).mask & !NFS_ACL_MASK) != 0 { (*resp).status = nfserr_io; return rpc_success; }
    (*resp).mask = (*argp).mask;
    (*resp).status = fh_getattr(fh, &mut (*resp).stat);
    if (*resp).status != nfs_ok { return rpc_success; }
    if ((*resp).mask & (NFS_ACL | NFS_ACLCNT)) != 0 {
        acl = get_inode_acl(inode, ACL_TYPE_ACCESS);
        if acl.is_null() { acl = posix_acl_from_mode((*inode).i_mode, GFP_KERNEL); }
        if IS_ERR(acl) { (*resp).status = nfserrno(PTR_ERR(acl)); posix_acl_release((*resp).acl_access); posix_acl_release((*resp).acl_default); (*resp).acl_access = core::ptr::null_mut(); (*resp).acl_default = core::ptr::null_mut(); return rpc_success; }
        (*resp).acl_access = acl;
    }
    if ((*resp).mask & (NFS_DFACL | NFS_DFACLCNT)) != 0 {
        acl = get_inode_acl(inode, ACL_TYPE_DEFAULT);
        if IS_ERR(acl) { (*resp).status = nfserrno(PTR_ERR(acl)); posix_acl_release((*resp).acl_access); posix_acl_release((*resp).acl_default); (*resp).acl_access = core::ptr::null_mut(); (*resp).acl_default = core::ptr::null_mut(); return rpc_success; }
        (*resp).acl_default = acl;
    }
    rpc_success
}

/* Set the Access and/or Default ACL of a file. */
unsafe fn nfsacld_proc_setacl(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_setaclargs;
    let resp = (*rqstp).rq_resp as *mut nfsd_attrstat;
    let fh: *mut svc_fh;
    let inode: *mut inode;
    let mut error: i32;
    dprintk(c"nfsd: SETACL(2acl)   %s\n", SVCFH_fmt(&(*argp).fh));
    fh = fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_SATTR);
    if (*resp).status != nfs_ok { return rpc_success; }
    inode = d_inode((*fh).fh_dentry);
    error = fh_want_write(fh);
    if error != 0 { (*resp).status = nfserrno(error); return rpc_success; }
    inode_lock(inode);
    error = 0;
    if ((*argp).mask & NFS_ACL) != 0 { error = set_posix_acl(&nop_mnt_idmap, (*fh).fh_dentry, ACL_TYPE_ACCESS, (*argp).acl_access); }
    if error == 0 && ((*argp).mask & NFS_DFACL) != 0 { error = set_posix_acl(&nop_mnt_idmap, (*fh).fh_dentry, ACL_TYPE_DEFAULT, (*argp).acl_default); }
    inode_unlock(inode);
    fh_drop_write(fh);
    if error != 0 { (*resp).status = nfserrno(error); return rpc_success; }
    (*resp).status = fh_getattr(fh, &mut (*resp).stat);
    rpc_success
}

/* Check file attributes. */
unsafe fn nfsacld_proc_getattr(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_fhandle;
    let resp = (*rqstp).rq_resp as *mut nfsd_attrstat;
    dprintk(c"nfsd: GETATTR  %s\n", SVCFH_fmt(&(*argp).fh));
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_NOP);
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    rpc_success
}

/* Check file access. */
unsafe fn nfsacld_proc_access(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_accessargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_accessres;
    dprintk(c"nfsd: ACCESS(2acl)   %s 0x%x\n", SVCFH_fmt(&(*argp).fh), (*argp).access);
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).access = (*argp).access;
    (*resp).status = nfsd_access(rqstp, &mut (*resp).fh, &mut (*resp).access, core::ptr::null_mut());
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    rpc_success
}

/* XDR decode functions. */
unsafe fn nfsaclsvc_decode_getaclargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let a = (*rqstp).rq_argp as *mut nfsd3_getaclargs;
    svcxdr_decode_fhandle(xdr, &mut (*a).fh) && xdr_stream_decode_u32(xdr, &mut (*a).mask) >= 0
}
unsafe fn nfsaclsvc_decode_setaclargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let a = (*rqstp).rq_argp as *mut nfsd3_setaclargs;
    if !svcxdr_decode_fhandle(xdr, &mut (*a).fh) || xdr_stream_decode_u32(xdr, &mut (*a).mask) < 0 || ((*a).mask & !NFS_ACL_MASK) != 0 { return false; }
    nfs_stream_decode_acl(xdr, core::ptr::null_mut(), if (*a).mask & NFS_ACL != 0 { &mut (*a).acl_access } else { core::ptr::null_mut() }) && nfs_stream_decode_acl(xdr, core::ptr::null_mut(), if (*a).mask & NFS_DFACL != 0 { &mut (*a).acl_default } else { core::ptr::null_mut() })
}
unsafe fn nfsaclsvc_decode_accessargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let a = (*rqstp).rq_argp as *mut nfsd3_accessargs;
    svcxdr_decode_fhandle(xdr, &mut (*a).fh) && xdr_stream_decode_u32(xdr, &mut (*a).access) >= 0
}

/* XDR encode functions. */
unsafe fn nfsaclsvc_encode_getaclres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let r = (*rqstp).rq_resp as *mut nfsd3_getaclres;
    if !svcxdr_encode_stat(xdr, (*r).status) { return false; }
    if (*r).status == nfs_ok { let i = d_inode((*r).fh.fh_dentry); return svcxdr_encode_fattr(rqstp,xdr,&mut (*r).fh,&(*r).stat) && xdr_stream_encode_u32(xdr,(*r).mask) >= 0 && nfs_stream_encode_acl(xdr,i,(*r).acl_access,(*r).mask & NFS_ACL,0) && nfs_stream_encode_acl(xdr,i,(*r).acl_default,(*r).mask & NFS_DFACL,NFS_ACL_DEFAULT); }
    true
}
unsafe fn nfsaclsvc_encode_accessres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let r = (*rqstp).rq_resp as *mut nfsd3_accessres;
    if !svcxdr_encode_stat(xdr,(*r).status) { return false; }
    (*r).status != nfs_ok || (svcxdr_encode_fattr(rqstp,xdr,&mut (*r).fh,&(*r).stat) && xdr_stream_encode_u32(xdr,(*r).access) >= 0)
}

/* XDR release functions. */
unsafe fn nfsaclsvc_release_getacl(rqstp: *mut svc_rqst) { let r=(*rqstp).rq_resp as *mut nfsd3_getaclres; fh_put(&mut (*r).fh); posix_acl_release((*r).acl_access); posix_acl_release((*r).acl_default); }
unsafe fn nfsaclsvc_release_access(rqstp: *mut svc_rqst) { fh_put(&mut *((*rqstp).rq_resp as *mut nfsd3_accessres).as_mut().unwrap().fh); }
unsafe fn nfsaclsvc_release_setacl(rqstp: *mut svc_rqst) { let a=(*rqstp).rq_argp as *mut nfsd3_setaclargs; let r=(*rqstp).rq_resp as *mut nfsd_attrstat; fh_put(&mut (*r).fh); posix_acl_release((*a).acl_access); posix_acl_release((*a).acl_default); }

const ST: usize = 1;
const AT: usize = 21;
const PAT: usize = 1 + AT;
const ACL: usize = 1 + NFS_ACL_MAX_ENTRIES * 3;

// The procedure table retains the C ABI layout and callback ordering.
pub static nfsd_acl_procedures2: [svc_procedure; 5] = [
    svc_procedure { pc_func: nfsacld_proc_null, pc_decode: nfssvc_decode_voidarg, pc_encode: nfssvc_encode_voidres, pc_release: None, pc_argsize: size_of::<nfsd_voidargs>(), pc_argzero: size_of::<nfsd_voidargs>(), pc_ressize: size_of::<nfsd_voidres>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST, pc_name: c"NULL" },
    svc_procedure { pc_func: nfsacld_proc_getacl, pc_decode: nfsaclsvc_decode_getaclargs, pc_encode: nfsaclsvc_encode_getaclres, pc_release: Some(nfsaclsvc_release_getacl), pc_argsize: size_of::<nfsd3_getaclargs>(), pc_argzero: size_of::<nfsd3_getaclargs>(), pc_ressize: size_of::<nfsd3_getaclres>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + 1 + 2 * (1 + ACL), pc_name: c"GETACL" },
    svc_procedure { pc_func: nfsacld_proc_setacl, pc_decode: nfsaclsvc_decode_setaclargs, pc_encode: nfssvc_encode_attrstatres, pc_release: Some(nfsaclsvc_release_setacl), pc_argsize: size_of::<nfsd3_setaclargs>(), pc_argzero: size_of::<nfsd3_setaclargs>(), pc_ressize: size_of::<nfsd_attrstat>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + AT, pc_name: c"SETACL" },
    svc_procedure { pc_func: nfsacld_proc_getattr, pc_decode: nfssvc_decode_fhandleargs, pc_encode: nfssvc_encode_attrstatres, pc_release: nfssvc_release_attrstat, pc_argsize: size_of::<nfsd_fhandle>(), pc_argzero: size_of::<nfsd_fhandle>(), pc_ressize: size_of::<nfsd_attrstat>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + AT, pc_name: c"GETATTR" },
    svc_procedure { pc_func: nfsacld_proc_access, pc_decode: nfsaclsvc_decode_accessargs, pc_encode: nfsaclsvc_encode_accessres, pc_release: Some(nfsaclsvc_release_access), pc_argsize: size_of::<nfsd3_accessargs>(), pc_argzero: size_of::<nfsd3_accessargs>(), pc_ressize: size_of::<nfsd3_accessres>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + AT + 1, pc_name: c"SETATTR" },
];
pub static nfsd_acl_version2: svc_version = svc_version {
    vs_vers: 2, vs_nproc: nfsd_acl_procedures2.len(), vs_proc: nfsd_acl_procedures2.as_ptr(),
    vs_dispatch: nfsd_dispatch, vs_xdrsize: NFS3_SVC_XDRSIZE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
