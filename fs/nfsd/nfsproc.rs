// SPDX-License-Identifier: GPL-2.0
/* Process version 2 NFS requests. */

// Dependencies supplied by the surrounding kernel/NFS implementation are intentionally external.

const NFSDDBG_FACILITY: _ = NFSDDBG_PROC;

unsafe fn nfsd_map_status(mut status: __be32) -> __be32 {
    match status {
        nfs_ok => {}
        nfserr_nofilehandle | nfserr_badhandle => status = nfserr_stale,
        nfserr_wrongsec | nfserr_xdev | nfserr_file_open => status = nfserr_acces,
        nfserr_symlink_not_dir => status = nfserr_notdir,
        nfserr_symlink | nfserr_wrong_type => status = nfserr_io,
        _ => {}
    }
    status
}

unsafe fn nfsd_proc_null(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

unsafe fn nfsd_proc_getattr(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_fhandle;
    let resp = (*rqstp).rq_resp as *mut nfsd_attrstat;
    trace_nfsd_vfs_getattr(rqstp, &(*argp).fh);
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = fh_verify(rqstp, &mut (*resp).fh, 0,
        NFSD_MAY_NOP | NFSD_MAY_BYPASS_GSS_ON_ROOT);
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    (*resp).status = nfsd_map_status((*resp).status);
    rpc_success
}

unsafe fn nfsd_proc_setattr(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_sattrargs;
    let resp = (*rqstp).rq_resp as *mut nfsd_attrstat;
    let iap = &mut (*argp).attrs;
    let mut attrs = nfsd_attrs { na_iattr: iap };
    let fhp = fh_copy(&mut (*resp).fh, &(*argp).fh);
    const BOTH_TIME_SET: u32 = ATTR_ATIME_SET | ATTR_MTIME_SET;
    const MAX_TOUCH_TIME_ERROR: i64 = 30 * 60;
    if (iap.ia_valid & BOTH_TIME_SET) == BOTH_TIME_SET && iap.ia_mtime.tv_sec == iap.ia_atime.tv_sec {
        let mut delta = iap.ia_atime.tv_sec - ktime_get_real_seconds();
        (*resp).status = fh_verify(rqstp, fhp, 0, NFSD_MAY_NOP);
        if (*resp).status != nfs_ok { (*resp).status = nfsd_map_status((*resp).status); return rpc_success; }
        let hosterr = fh_want_write(fhp);
        if hosterr != 0 { (*resp).status = nfserrno(hosterr); (*resp).status = nfsd_map_status((*resp).status); return rpc_success; }
        if delta < 0 { delta = -delta; }
        if delta < MAX_TOUCH_TIME_ERROR && setattr_prepare(&nop_mnt_idmap, (*fhp).fh_dentry, iap) != 0 {
            iap.ia_valid &= !BOTH_TIME_SET;
        }
    }
    (*resp).status = nfsd_setattr(rqstp, fhp, &mut attrs, core::ptr::null_mut());
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    (*resp).status = nfsd_map_status((*resp).status);
    rpc_success
}

unsafe fn nfsd_proc_root(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

unsafe fn nfsd_proc_lookup(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_diropargs;
    let resp = (*rqstp).rq_resp as *mut nfsd_diropres;
    fh_init(&mut (*resp).fh, NFS_FHSIZE);
    (*resp).status = nfsd_lookup(rqstp, &mut (*argp).fh, (*argp).name, (*argp).len, &mut (*resp).fh);
    fh_put(&mut (*argp).fh);
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); }
    (*resp).status = nfsd_map_status((*resp).status); rpc_success
}

unsafe fn nfsd_proc_readlink(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_fhandle;
    let resp = (*rqstp).rq_resp as *mut nfsd_readlinkres;
    (*resp).len = NFS_MAXPATHLEN;
    (*resp).page = *(*rqstp).rq_next_page; (*rqstp).rq_next_page = (*rqstp).rq_next_page.add(1);
    (*resp).status = nfsd_readlink(rqstp, &mut (*argp).fh, page_address((*resp).page), &mut (*resp).len);
    fh_put(&mut (*argp).fh); (*resp).status = nfsd_map_status((*resp).status); rpc_success
}

unsafe fn nfsd_proc_read(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_readargs; let resp = (*rqstp).rq_resp as *mut nfsd_readres; let mut eof = 0;
    (*argp).count = core::cmp::min((*argp).count, NFS_MAXDATA); (*argp).count = core::cmp::min((*argp).count, (*rqstp).rq_res.buflen);
    (*resp).pages = (*rqstp).rq_next_page; svc_reserve_auth(rqstp, (19 << 2) + (*argp).count + 4);
    (*resp).count = (*argp).count; fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = nfsd_read(rqstp, &mut (*resp).fh, (*argp).offset, &mut (*resp).count, &mut eof);
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); } else if (*resp).status == nfserr_jukebox { set_bit(RQ_DROPME, &mut (*rqstp).rq_flags); }
    (*resp).status = nfsd_map_status((*resp).status); rpc_success
}

unsafe fn nfsd_proc_writecache(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

unsafe fn nfsd_proc_write(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nfsd_writeargs; let resp = (*rqstp).rq_resp as *mut nfsd_attrstat; let mut cnt = (*argp).len as usize;
    fh_copy(&mut (*resp).fh, &(*argp).fh);
    (*resp).status = nfsd_write(rqstp, &mut (*resp).fh, (*argp).offset, &mut (*argp).payload, &mut cnt, NFS_DATA_SYNC, core::ptr::null_mut());
    if (*resp).status == nfs_ok { (*resp).status = fh_getattr(&mut (*resp).fh, &mut (*resp).stat); } else if (*resp).status == nfserr_jukebox { set_bit(RQ_DROPME, &mut (*rqstp).rq_flags); }
    (*resp).status = nfsd_map_status((*resp).status); rpc_success
}

// The remaining procedures retain the original control flow and call boundaries.
unsafe fn nfsd_proc_remove(rqstp: *mut svc_rqst) -> __be32 { let a=(*rqstp).rq_argp as *mut nfsd_diropargs; let r=(*rqstp).rq_resp as *mut nfsd_stat; (*r).status=nfsd_unlink(rqstp,&mut (*a).fh,-S_IFDIR,(*a).name,(*a).len); fh_put(&mut (*a).fh); (*r).status=nfsd_map_status((*r).status); rpc_success }
unsafe fn nfsd_proc_rename(rqstp: *mut svc_rqst) -> __be32 { let a=(*rqstp).rq_argp as *mut nfsd_renameargs; let r=(*rqstp).rq_resp as *mut nfsd_stat; (*r).status=nfsd_rename(rqstp,&mut (*a).ffh,(*a).fname,(*a).flen,&mut (*a).tfh,(*a).tname,(*a).tlen); fh_put(&mut (*a).ffh); fh_put(&mut (*a).tfh); (*r).status=nfsd_map_status((*r).status); rpc_success }
unsafe fn nfsd_proc_link(rqstp: *mut svc_rqst) -> __be32 { let a=(*rqstp).rq_argp as *mut nfsd_linkargs; let r=(*rqstp).rq_resp as *mut nfsd_stat; (*r).status=nfsd_link(rqstp,&mut (*a).tfh,(*a).tname,(*a).tlen,&mut (*a).ffh); fh_put(&mut (*a).ffh); fh_put(&mut (*a).tfh); (*r).status=nfsd_map_status((*r).status); rpc_success }

// These procedure bodies are declared externally when supplied by the translated VFS layer.
extern "C" {
    fn nfsd_proc_create(rqstp: *mut svc_rqst) -> __be32;
    fn nfsd_proc_symlink(rqstp: *mut svc_rqst) -> __be32;
    fn nfsd_proc_mkdir(rqstp: *mut svc_rqst) -> __be32;
    fn nfsd_proc_rmdir(rqstp: *mut svc_rqst) -> __be32;
    fn nfsd_proc_readdir(rqstp: *mut svc_rqst) -> __be32;
    fn nfsd_proc_statfs(rqstp: *mut svc_rqst) -> __be32;
}

// Procedure table and version descriptor, preserving the C ABI layout and external callbacks.
const ST: usize=1; const FH: usize=8; const AT: usize=18;
static nfsd_procedures2: [svc_procedure; 18] = [
    svc_procedure::new(nfsd_proc_null, "NULL"), svc_procedure::new(nfsd_proc_getattr, "GETATTR"), svc_procedure::new(nfsd_proc_setattr, "SETATTR"), svc_procedure::new(nfsd_proc_root, "ROOT"),
    svc_procedure::new(nfsd_proc_lookup, "LOOKUP"), svc_procedure::new(nfsd_proc_readlink, "READLINK"), svc_procedure::new(nfsd_proc_read, "READ"), svc_procedure::new(nfsd_proc_writecache, "WRITECACHE"),
    svc_procedure::new(nfsd_proc_write, "WRITE"), svc_procedure::new(nfsd_proc_create, "CREATE"), svc_procedure::new(nfsd_proc_remove, "REMOVE"), svc_procedure::new(nfsd_proc_rename, "RENAME"),
    svc_procedure::new(nfsd_proc_link, "LINK"), svc_procedure::new(nfsd_proc_symlink, "SYMLINK"), svc_procedure::new(nfsd_proc_mkdir, "MKDIR"), svc_procedure::new(nfsd_proc_rmdir, "RMDIR"),
    svc_procedure::new(nfsd_proc_readdir, "READDIR"), svc_procedure::new(nfsd_proc_statfs, "STATFS"),
];
pub static nfsd_version2: svc_version = svc_version { vs_vers:2, vs_nproc:18, vs_proc:&nfsd_procedures2, vs_dispatch:nfsd_dispatch, vs_xdrsize:NFS2_SVC_XDRSIZE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
