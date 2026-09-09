// SPDX-License-Identifier: GPL-2.0
/* Process version 3 NFS requests. */

// Kernel and local header dependencies are supplied by the surrounding crate.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PROC;

static mut NFS3_FTYPES: [i32; 8] = [0, S_IFREG, S_IFDIR, S_IFBLK, S_IFCHR, S_IFLNK, S_IFSOCK, S_IFIFO];

/* Reject client supplied timestamps whose nanoseconds are out of range. */
unsafe fn nfsd3_time_in_range(iap: *const iattr) -> bool {
    if ((*iap).ia_valid & ATTR_ATIME_SET) != 0 && ((*iap).ia_atime.tv_nsec as c_ulong) >= NSEC_PER_SEC { return false; }
    if ((*iap).ia_valid & ATTR_MTIME_SET) != 0 && ((*iap).ia_mtime.tv_nsec as c_ulong) >= NSEC_PER_SEC { return false; }
    true
}

unsafe fn nfsd3_map_status(mut status: __be32) -> __be32 {
    match status {
        nfs_ok => (),
        nfserr_nofilehandle => status = nfserr_badhandle,
        nfserr_wrongsec | nfserr_file_open => status = nfserr_acces,
        nfserr_symlink_not_dir => status = nfserr_notdir,
        nfserr_symlink | nfserr_wrong_type => status = nfserr_inval,
        _ => (),
    }
    status
}

unsafe fn nfsd3_proc_null(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

unsafe fn nfsd3_proc_getattr(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_fhandle;
    let resp = (*rqstp).rq_resp as *mut nfsd3_attrstat;
    trace_nfsd_vfs_getattr(rqstp, &(*argp).fh);
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0, NFSD_MAY_NOP | NFSD_MAY_BYPASS_GSS_ON_ROOT);
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    (*resp).status = nfsd3_map_status((*resp).status); rpc_success
}

unsafe fn nfsd3_proc_setattr(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd3_sattrargs;
    let resp = (*rqstp).rq_resp as *mut nfsd3_attrstat;
    let attrs = nfsd_attrs { na_iattr: &mut (*argp).attrs };
    let mut guardtime: *const timespec64 = core::ptr::null();
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    if !nfsd3_time_in_range(&(*argp).attrs) { (*resp).status = nfserr_inval; }
    else {
        if (*argp).check_guard { guardtime = &(*argp).guardtime; }
        (*resp).status = nfsd_setattr(rqstp, &mut (*resp).fh, &attrs, guardtime);
    }
    (*resp).status = nfsd3_map_status((*resp).status); rpc_success
}

unsafe fn nfsd3_proc_lookup(rqstp: *mut svc_rqst) -> __be32 {
    let a = (*rqstp).rq_argp as *mut nfsd3_diropargs; let r = (*rqstp).rq_resp as *mut nfsd3_diropres;
    fh_copy(&mut (*r).dirfh, &(*a).fh); fh_init(&mut (*r).fh, NFS3_FHSIZE);
    (*r).status = nfsd_lookup(rqstp, &mut (*r).dirfh, (*a).name, (*a).len, &mut (*r).fh);
    (*r).status = nfsd3_map_status((*r).status); rpc_success
}

unsafe fn nfsd3_proc_access(rqstp: *mut svc_rqst) -> __be32 {
    let a = (*rqstp).rq_argp as *mut nfsd3_accessargs; let r = (*rqstp).rq_resp as *mut nfsd3_accessres;
    fh_copy(&mut (*r).fh, &(*a).fh); (*r).access = (*a).access;
    (*r).status = nfsd_access(rqstp, &mut (*r).fh, &mut (*r).access, core::ptr::null_mut());
    (*r).status = nfsd3_map_status((*r).status); rpc_success
}

unsafe fn nfsd3_proc_readlink(rqstp: *mut svc_rqst) -> __be32 {
    let a = (*rqstp).rq_argp as *mut nfsd_fhandle; let r = (*rqstp).rq_resp as *mut nfsd3_readlinkres;
    fh_copy(&mut (*r).fh, &(*a).fh); (*r).len = NFS3_MAXPATHLEN;
    (*r).pages = (*rqstp).rq_next_page; (*rqstp).rq_next_page = (*rqstp).rq_next_page.add(1);
    (*r).status = nfsd_readlink(rqstp, &mut (*r).fh, page_address(*(*r).pages), &mut (*r).len);
    (*r).status = nfsd3_map_status((*r).status); rpc_success
}

unsafe fn nfsd3_proc_read(rqstp: *mut svc_rqst) -> __be32 {
    let a = (*rqstp).rq_argp as *mut nfsd3_readargs; let r = (*rqstp).rq_resp as *mut nfsd3_readres;
    (*a).count = core::cmp::min((*a).count, svc_max_payload(rqstp));
    (*a).count = core::cmp::min((*a).count, (*rqstp).rq_res.buflen);
    if (*a).offset > OFFSET_MAX as u64 { (*a).offset = OFFSET_MAX as u64; }
    if (*a).offset + (*a).count as u64 > OFFSET_MAX as u64 { (*a).count = (OFFSET_MAX as u64 - (*a).offset) as u32; }
    (*r).pages = (*rqstp).rq_next_page; (*r).count = (*a).count;
    svc_reserve_auth(rqstp, ((1 + NFS3_POST_OP_ATTR_WORDS + 3) << 2) + (*r).count + 4);
    fh_copy(&mut (*r).fh, &(*a).fh); (*r).status = nfsd_read(rqstp, &mut (*r).fh, (*a).offset, &mut (*r).count, &mut (*r).eof);
    (*r).status = nfsd3_map_status((*r).status); rpc_success
}

unsafe fn nfsd3_proc_write(rqstp: *mut svc_rqst) -> __be32 {
    let a = (*rqstp).rq_argp as *mut nfsd3_writeargs; let r = (*rqstp).rq_resp as *mut nfsd3_writeres; let mut cnt = (*a).len as c_ulong;
    (*r).status = nfserr_fbig;
    if (*a).offset > OFFSET_MAX as u64 || (*a).offset + (*a).len as u64 > OFFSET_MAX as u64 { return rpc_success; }
    fh_copy(&mut (*r).fh, &(*a).fh); (*r).committed = (*a).stable;
    (*r).status = nfsd_write(rqstp, &mut (*r).fh, (*a).offset, &mut (*a).payload, &mut cnt, (*r).committed, (*r).verf);
    (*r).count = cnt; (*r).status = nfsd3_map_status((*r).status); rpc_success
}

/* The remaining handlers retain the kernel operation ordering and use the same external types. */
unsafe fn nfsd3_proc_remove(r: *mut svc_rqst) -> __be32 { let a=(*r).rq_argp as *mut nfsd3_diropargs; let o=(*r).rq_resp as *mut nfsd3_attrstat; fh_copy(&mut (*o).fh,&(*a).fh); (*o).status=nfsd_unlink(r,&mut (*o).fh,-S_IFDIR,(*a).name,(*a).len); (*o).status=nfsd3_map_status((*o).status); rpc_success }
unsafe fn nfsd3_proc_rmdir(r: *mut svc_rqst) -> __be32 { let a=(*r).rq_argp as *mut nfsd3_diropargs; let o=(*r).rq_resp as *mut nfsd3_attrstat; fh_copy(&mut (*o).fh,&(*a).fh); (*o).status=nfsd_unlink(r,&mut (*o).fh,S_IFDIR,(*a).name,(*a).len); (*o).status=nfsd3_map_status((*o).status); rpc_success }
unsafe fn nfsd3_proc_rename(r: *mut svc_rqst) -> __be32 { let a=(*r).rq_argp as *mut nfsd3_renameargs; let o=(*r).rq_resp as *mut nfsd3_renameres; fh_copy(&mut (*o).ffh,&(*a).ffh); fh_copy(&mut (*o).tfh,&(*a).tfh); (*o).status=nfsd_rename(r,&mut (*o).ffh,(*a).fname,(*a).flen,&mut (*o).tfh,(*a).tname,(*a).tlen); (*o).status=nfsd3_map_status((*o).status); rpc_success }
unsafe fn nfsd3_proc_link(r: *mut svc_rqst) -> __be32 { let a=(*r).rq_argp as *mut nfsd3_linkargs; let o=(*r).rq_resp as *mut nfsd3_linkres; fh_copy(&mut (*o).fh,&(*a).ffh); fh_copy(&mut (*o).tfh,&(*a).tfh); (*o).status=nfsd_link(r,&mut (*o).tfh,(*a).tname,(*a).tlen,&mut (*o).fh); (*o).status=nfsd3_map_status((*o).status); rpc_success }

/* Procedure descriptors and aliases are supplied with the same ABI-visible layout. */
const ST: usize=1; const FH: usize=17; const AT: usize=21; const PAT: usize=1+AT; const WC: usize=7+PAT;
const NFS3_PROCEDURES3: [svc_procedure; 22] = [
    svc_procedure::new(nfsd3_proc_null, "NULL", RC_NOCACHE, ST),
    svc_procedure::new(nfsd3_proc_getattr, "GETATTR", RC_NOCACHE, ST+AT),
    svc_procedure::new(nfsd3_proc_setattr, "SETATTR", RC_REPLBUFF, ST+WC),
    svc_procedure::new(nfsd3_proc_lookup, "LOOKUP", RC_NOCACHE, ST+FH+PAT+PAT),
    svc_procedure::new(nfsd3_proc_access, "ACCESS", RC_NOCACHE, ST+PAT+1),
    svc_procedure::new(nfsd3_proc_readlink, "READLINK", RC_NOCACHE, ST+PAT+1+NFS3_MAXPATHLEN/4),
    svc_procedure::new(nfsd3_proc_read, "READ", RC_NOCACHE, ST+PAT+4+NFSSVC_MAXBLKSIZE/4),
    svc_procedure::new(nfsd3_proc_write, "WRITE", RC_REPLBUFF, ST+WC+4),
    svc_procedure::new(nfsd3_proc_create, "CREATE", RC_REPLBUFF, ST+(1+FH+PAT)+WC),
    svc_procedure::new(nfsd3_proc_mkdir, "MKDIR", RC_REPLBUFF, ST+(1+FH+PAT)+WC),
    svc_procedure::new(nfsd3_proc_symlink, "SYMLINK", RC_REPLBUFF, ST+(1+FH+PAT)+WC),
    svc_procedure::new(nfsd3_proc_mknod, "MKNOD", RC_REPLBUFF, ST+(1+FH+PAT)+WC),
    svc_procedure::new(nfsd3_proc_remove, "REMOVE", RC_REPLBUFF, ST+WC),
    svc_procedure::new(nfsd3_proc_rmdir, "RMDIR", RC_REPLBUFF, ST+WC),
    svc_procedure::new(nfsd3_proc_rename, "RENAME", RC_REPLBUFF, ST+WC+WC),
    svc_procedure::new(nfsd3_proc_link, "LINK", RC_REPLBUFF, ST+PAT+WC),
    svc_procedure::new(nfsd3_proc_readdir, "READDIR", RC_NOCACHE, 0),
    svc_procedure::new(nfsd3_proc_readdirplus, "READDIRPLUS", RC_NOCACHE, 0),
    svc_procedure::new(nfsd3_proc_fsstat, "FSSTAT", RC_NOCACHE, ST+PAT+2*6+1),
    svc_procedure::new(nfsd3_proc_fsinfo, "FSINFO", RC_NOCACHE, ST+PAT+12),
    svc_procedure::new(nfsd3_proc_pathconf, "PATHCONF", RC_NOCACHE, ST+PAT+6),
    svc_procedure::new(nfsd3_proc_commit, "COMMIT", RC_NOCACHE, ST+WC+2),
];

pub static nfsd_version3: svc_version = svc_version { vs_vers: 3, vs_nproc: NFS3_PROCEDURES3.len(), vs_proc: NFS3_PROCEDURES3.as_ptr(), vs_dispatch: nfsd_dispatch, vs_xdrsize: NFS3_SVC_XDRSIZE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
