// SPDX-License-Identifier: GPL-2.0
/*
 * Process version 3 NFSACL requests.
 *
 * Copyright (C) 2002-2003 Andreas Gruenbacher <agruen@suse.de>
 */

// Dependencies supplied by the surrounding kernel NFS implementation:
// nfsd.h, linux/nfsacl.h, linux/gfp.h, cache.h, xdr3.h, and vfs.h.

/* NULL call. */
unsafe fn nfsd3_proc_null(_rqstp: *mut svc_rqst) -> __be32 {
    rpc_success
}

/* Get the Access and/or Default ACL of a file. */
unsafe fn nfsd3_proc_getacl(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_getaclargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_getaclres;
    let mut acl: *mut posix_acl;
    let inode: *mut inode;
    let fh: *mut svc_fh;

    fh = fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_NOP);
    if (*resp).status != nfs_ok {
        return rpc_success;
    }

    inode = d_inode((*fh).fh_dentry);

    if ((*argp).mask & !NFS_ACL_MASK) != 0 {
        (*resp).status = nfserr_inval;
        return rpc_success;
    }
    (*resp).mask = (*argp).mask;

    if ((*resp).mask & (NFS_ACL | NFS_ACLCNT)) != 0 {
        acl = get_inode_acl(inode, ACL_TYPE_ACCESS);
        if acl.is_null() {
            /* Solaris returns the inode's minimum ACL. */
            acl = posix_acl_from_mode((*inode).i_mode, GFP_KERNEL);
        }
        if IS_ERR(acl) {
            (*resp).status = nfserrno(PTR_ERR(acl));
            posix_acl_release((*resp).acl_access);
            posix_acl_release((*resp).acl_default);
            (*resp).acl_access = core::ptr::null_mut();
            (*resp).acl_default = core::ptr::null_mut();
            return rpc_success;
        }
        (*resp).acl_access = acl;
    }
    if ((*resp).mask & (NFS_DFACL | NFS_DFACLCNT)) != 0 {
        /* Check how Solaris handles requests for the Default ACL of a non-directory! */
        acl = get_inode_acl(inode, ACL_TYPE_DEFAULT);
        if IS_ERR(acl) {
            (*resp).status = nfserrno(PTR_ERR(acl));
            posix_acl_release((*resp).acl_access);
            posix_acl_release((*resp).acl_default);
            (*resp).acl_access = core::ptr::null_mut();
            (*resp).acl_default = core::ptr::null_mut();
            return rpc_success;
        }
        (*resp).acl_default = acl;
    }
    // resp->acl_{access,default} are released in nfs3svc_release_getacl.
    rpc_success
}

/* Set the Access and/or Default ACL of a file. */
unsafe fn nfsd3_proc_setacl(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_setaclargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_attrstat;
    let inode: *mut inode;
    let fh: *mut svc_fh;
    let mut error: int;

    fh = fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_SATTR);
    if (*resp).status != nfs_ok { return rpc_success; }
    inode = d_inode((*fh).fh_dentry);
    error = fh_want_write(fh);
    if error != 0 { (*resp).status = nfserrno(error); return rpc_success; }
    inode_lock(inode);
    error = 0;
    if ((*argp).mask & NFS_ACL) != 0 {
        error = set_posix_acl(&nop_mnt_idmap, (*fh).fh_dentry, ACL_TYPE_ACCESS, (*argp).acl_access);
        if error != 0 { inode_unlock(inode); fh_drop_write(fh); (*resp).status = nfserrno(error); return rpc_success; }
    }
    if ((*argp).mask & NFS_DFACL) != 0 {
        error = set_posix_acl(&nop_mnt_idmap, (*fh).fh_dentry, ACL_TYPE_DEFAULT, (*argp).acl_default);
    }
    inode_unlock(inode);
    fh_drop_write(fh);
    (*resp).status = nfserrno(error);
    // argp->acl_{access,default} are released in nfs3svc_release_setacl.
    rpc_success
}

/* XDR decode functions */
unsafe fn nfs3svc_decode_getaclargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let args = (*rqstp).rq_argp as *mut nfsd3_getaclargs;
    if !svcxdr_decode_nfs_fh3(xdr, &mut (*args).fh) { return false; }
    if xdr_stream_decode_u32(xdr, &mut (*args).mask) < 0 { return false; }
    true
}

unsafe fn nfs3svc_decode_setaclargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let argp = (*rqstp).rq_argp as *mut nfsd3_setaclargs;
    if !svcxdr_decode_nfs_fh3(xdr, &mut (*argp).fh) { return false; }
    if xdr_stream_decode_u32(xdr, &mut (*argp).mask) < 0 { return false; }
    if ((*argp).mask & !NFS_ACL_MASK) != 0 { return false; }
    if !nfs_stream_decode_acl(xdr, core::ptr::null_mut(), if ((*argp).mask & NFS_ACL) != 0 { &mut (*argp).acl_access } else { core::ptr::null_mut() }) { return false; }
    if !nfs_stream_decode_acl(xdr, core::ptr::null_mut(), if ((*argp).mask & NFS_DFACL) != 0 { &mut (*argp).acl_default } else { core::ptr::null_mut() }) { return false; }
    true
}

/* XDR encode functions */
unsafe fn nfs3svc_encode_getaclres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let resp = (*rqstp).rq_resp as *mut nfsd3_getaclres;
    let dentry = (*resp).fh.fh_dentry;
    if !svcxdr_encode_nfsstat3(xdr, (*resp).status) { return false; }
    if (*resp).status == nfs_ok {
        let inode = d_inode(dentry);
        if !svcxdr_encode_post_op_attr(rqstp, xdr, &(*resp).fh) { return false; }
        if xdr_stream_encode_u32(xdr, (*resp).mask) < 0 { return false; }
        if !nfs_stream_encode_acl(xdr, inode, (*resp).acl_access, (*resp).mask & NFS_ACL, 0) { return false; }
        if !nfs_stream_encode_acl(xdr, inode, (*resp).acl_default, (*resp).mask & NFS_DFACL, NFS_ACL_DEFAULT) { return false; }
    } else if !svcxdr_encode_post_op_attr(rqstp, xdr, &(*resp).fh) { return false; }
    true
}

unsafe fn nfs3svc_encode_setaclres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let resp = (*rqstp).rq_resp as *mut nfsd3_attrstat;
    svcxdr_encode_nfsstat3(xdr, (*resp).status) && svcxdr_encode_post_op_attr(rqstp, xdr, &(*resp).fh)
}

/* XDR release functions */
unsafe fn nfs3svc_release_getacl(rqstp: *mut svc_rqst) {
    let resp = (*rqstp).rq_resp as *mut nfsd3_getaclres;
    fh_put(&mut (*resp).fh);
    posix_acl_release((*resp).acl_access);
    posix_acl_release((*resp).acl_default);
}

unsafe fn nfs3svc_release_setacl(rqstp: *mut svc_rqst) {
    let argp = (*rqstp).rq_argp as *mut nfsd3_setaclargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_attrstat;
    fh_put(&mut (*resp).fh);
    posix_acl_release((*argp).acl_access);
    posix_acl_release((*argp).acl_default);
}

const ST: usize = 1;
const AT: usize = 21;
const PAT: usize = 1 + AT;
const ACL: usize = 1 + NFS_ACL_MAX_ENTRIES * 3;

static nfsd_acl_procedures3: [svc_procedure; 3] = [
    svc_procedure { pc_func: Some(nfsd3_proc_null), pc_decode: Some(nfssvc_decode_voidarg), pc_encode: Some(nfssvc_encode_voidres), pc_release: None, pc_argsize: size_of::<nfsd_voidargs>(), pc_argzero: size_of::<nfsd_voidargs>(), pc_ressize: size_of::<nfsd_voidres>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST, pc_name: "NULL" },
    svc_procedure { pc_func: Some(nfsd3_proc_getacl), pc_decode: Some(nfs3svc_decode_getaclargs), pc_encode: Some(nfs3svc_encode_getaclres), pc_release: Some(nfs3svc_release_getacl), pc_argsize: size_of::<nfsd3_getaclargs>(), pc_argzero: size_of::<nfsd3_getaclargs>(), pc_ressize: size_of::<nfsd3_getaclres>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + 1 + 2 * (1 + ACL), pc_name: "GETACL" },
    svc_procedure { pc_func: Some(nfsd3_proc_setacl), pc_decode: Some(nfs3svc_decode_setaclargs), pc_encode: Some(nfs3svc_encode_setaclres), pc_release: Some(nfs3svc_release_setacl), pc_argsize: size_of::<nfsd3_setaclargs>(), pc_argzero: size_of::<nfsd3_setaclargs>(), pc_ressize: size_of::<nfsd3_attrstat>(), pc_cachetype: RC_NOCACHE, pc_xdrressize: ST + PAT, pc_name: "SETACL" },
];

static nfsd_acl_version3: svc_version = svc_version {
    vs_vers: 3,
    vs_nproc: nfsd_acl_procedures3.len(),
    vs_proc: nfsd_acl_procedures3.as_ptr(),
    vs_dispatch: Some(nfsd_dispatch),
    vs_xdrsize: NFS3_SVC_XDRSIZE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
