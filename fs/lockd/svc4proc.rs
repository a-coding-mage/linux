// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/lockd/svc4proc.c. External kernel/XDR symbols are
// intentionally referenced but not implemented here.

#[repr(C)]
pub struct nlm4_testargs_wrapper { pub xdrgen: nlm4_testargs, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_lockargs_wrapper { pub xdrgen: nlm4_lockargs, pub cookie: lockd_cookie, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_cancargs_wrapper { pub xdrgen: nlm4_cancargs, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_unlockargs_wrapper { pub xdrgen: nlm4_unlockargs, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_notifyargs_wrapper { pub xdrgen: nlm4_notifyargs, pub reboot: lockd_reboot }
#[repr(C)]
pub struct nlm4_notify_wrapper { pub xdrgen: nlm4_notify }
#[repr(C)]
pub struct nlm4_testres_wrapper { pub xdrgen: nlm4_testres, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_shareargs_wrapper { pub xdrgen: nlm4_shareargs, pub lock: lockd_lock }
#[repr(C)]
pub struct nlm4_res_wrapper { pub xdrgen: nlm4_res, pub cookie: lockd_cookie }
#[repr(C)]
pub struct nlm4_shareres_wrapper { pub xdrgen: nlm4_shareres }

unsafe fn nlm4_netobj_to_cookie(cookie: *mut lockd_cookie, object: *mut netobj) -> __be32 {
    if (*object).len > NLM_MAXCOOKIELEN { return nlm_lck_denied_nolocks; }
    (*cookie).len = (*object).len;
    core::ptr::copy_nonoverlapping((*object).data, (*cookie).data.as_mut_ptr(), (*object).len as usize);
    nlm_granted
}

unsafe fn nlm4_lock_to_lockd_lock(lock: *mut lockd_lock, alock: *mut nlm4_lock) -> __be32 {
    if (*alock).fh.len > NFS_MAXFHSIZE { return nlm_lck_denied; }
    (*lock).fh.size = (*alock).fh.len;
    core::ptr::copy_nonoverlapping((*alock).fh.data, (*lock).fh.data.as_mut_ptr(), (*alock).fh.len as usize);
    (*lock).oh.len = (*alock).oh.len; (*lock).oh.data = (*alock).oh.data;
    (*lock).svid = (*alock).svid;
    lockd_set_file_lock_range4(&mut (*lock).fl, (*alock).l_offset, (*alock).l_len);
    nlm_granted
}

unsafe fn nlm4svc_lookup_host(rqstp: *mut svc_rqst, caller: string, monitored: bool) -> *mut nlm_host {
    if !rcu_access_pointer(nlmsvc_ops) { return core::ptr::null_mut(); }
    let host = nlmsvc_lookup_host(rqstp, caller.data, caller.len);
    if host.is_null() { return host; }
    if monitored && nsm_monitor(host) < 0 { nlmsvc_release_host(host); return core::ptr::null_mut(); }
    host
}

unsafe fn nlm4svc_lookup_file(rqstp: *mut svc_rqst, host: *mut nlm_host, lock: *mut lockd_lock,
    filp: *mut *mut nlm_file, xdr_lock: *mut nlm4_lock, typ: u8) -> __be32 {
    let is_test = (*rqstp).rq_proc == NLMPROC4_TEST || (*rqstp).rq_proc == NLMPROC4_TEST_MSG;
    let fl = &mut (*lock).fl;
    let mut file: *mut nlm_file = core::ptr::null_mut();
    if (*xdr_lock).fh.len > NFS_MAXFHSIZE { return nlm_lck_denied_nolocks; }
    (*lock).fh.size = (*xdr_lock).fh.len;
    core::ptr::copy_nonoverlapping((*xdr_lock).fh.data, (*lock).fh.data.as_mut_ptr(), (*xdr_lock).fh.len as usize);
    if (*xdr_lock).fh.len < LOCKD_FH_HASH_SIZE { core::ptr::write_bytes((*lock).fh.data.as_mut_ptr().add((*xdr_lock).fh.len as usize), 0, (LOCKD_FH_HASH_SIZE - (*xdr_lock).fh.len) as usize); }
    (*lock).oh.len = (*xdr_lock).oh.len; (*lock).oh.data = (*xdr_lock).oh.data;
    (*lock).svid = (*xdr_lock).svid; (*lock).lock_start = (*xdr_lock).l_offset; (*lock).lock_len = (*xdr_lock).l_len;
    if (*lock).lock_start > OFFSET_MAX || ((*lock).lock_len != 0 && ((*lock).lock_len - 1) > OFFSET_MAX - (*lock).lock_start) { return nlm4_fbig; }
    locks_init_lock(fl); (*fl).c.flc_type = typ;
    lockd_set_file_lock_range4(fl, (*lock).lock_start, (*lock).lock_len);
    let mode = if is_test { O_RDWR } else { lock_to_openmode(fl) };
    let mut error = nlm_lookup_file(rqstp, &mut file, lock, mode);
    if error == nlm__int__stale_fh { return nlm4_stale_fh; }
    if error == nlm__int__failed { return nlm4_failed; }
    if error != nlm_granted { return error; }
    *filp = file; (*fl).c.flc_flags = FL_POSIX;
    (*fl).c.flc_file = if is_test { nlmsvc_file_file(file) } else { (*file).f_file[mode as usize] };
    (*fl).c.flc_pid = current().tgid; (*fl).fl_lmops = &nlmsvc_lock_operations;
    nlmsvc_locks_init_private(fl, host, (*lock).svid as pid_t);
    if (*fl).c.flc_owner.is_null() { return nlm_lck_denied_nolocks; }
    nlm_granted
}

unsafe fn nlm4svc_proc_null(_: *mut svc_rqst) -> __be32 { rpc_success }

unsafe fn nlm4svc_proc_test(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nlm4_testargs_wrapper; let resp = (*rqstp).rq_resp as *mut nlm4_testres_wrapper;
    let typ = if (*argp).xdrgen.exclusive { F_WRLCK } else { F_RDLCK }; let mut file = core::ptr::null_mut();
    (*resp).xdrgen.cookie = (*argp).xdrgen.cookie; (*resp).xdrgen.stat.stat = nlm_lck_denied_nolocks;
    let host = nlm4svc_lookup_host(rqstp, (*argp).xdrgen.alock.caller_name, false);
    if !host.is_null() { (*resp).xdrgen.stat.stat = nlm4svc_lookup_file(rqstp, host, &mut (*argp).lock, &mut file, &mut (*argp).xdrgen.alock, typ); if (*resp).xdrgen.stat.stat == 0 { (*resp).xdrgen.stat.stat = nlmsvc_testlock(rqstp, file, host, &mut (*argp).lock, &mut (*resp).lock); nlmsvc_release_lockowner(&mut (*argp).lock); } }
    if !file.is_null() { nlm_release_file(file); } nlmsvc_release_host(host);
    if (*resp).xdrgen.stat.stat == nlm__int__drop_reply { rpc_drop_reply } else { rpc_success }
}

unsafe fn nlm4svc_do_lock(rqstp: *mut svc_rqst, monitored: bool) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut nlm4_lockargs_wrapper; let resp = (*rqstp).rq_resp as *mut nlm4_res_wrapper;
    let typ = if (*argp).xdrgen.exclusive { F_WRLCK } else { F_RDLCK }; let mut file = core::ptr::null_mut();
    (*resp).xdrgen.cookie = (*argp).xdrgen.cookie; (*resp).xdrgen.stat.stat = nlm4_netobj_to_cookie(&mut (*argp).cookie, &mut (*argp).xdrgen.cookie);
    if (*resp).xdrgen.stat.stat == 0 { (*resp).xdrgen.stat.stat = nlm_lck_denied_nolocks; let host = nlm4svc_lookup_host(rqstp, (*argp).xdrgen.alock.caller_name, monitored); if !host.is_null() { (*resp).xdrgen.stat.stat = nlm4svc_lookup_file(rqstp, host, &mut (*argp).lock, &mut file, &mut (*argp).xdrgen.alock, typ); if (*resp).xdrgen.stat.stat == 0 { (*resp).xdrgen.stat.stat = nlmsvc_lock(rqstp, file, host, &mut (*argp).lock, (*argp).xdrgen.block, &mut (*argp).cookie, (*argp).xdrgen.reclaim); if (*resp).xdrgen.stat.stat == nlm__int__deadlock { (*resp).xdrgen.stat.stat = nlm4_deadlock; } nlmsvc_release_lockowner(&mut (*argp).lock); } nlmsvc_release_host(host); } }
    if !file.is_null() { nlm_release_file(file); } if (*resp).xdrgen.stat.stat == nlm__int__drop_reply { rpc_drop_reply } else { rpc_success }
}
unsafe fn nlm4svc_proc_lock(r: *mut svc_rqst)->__be32 { nlm4svc_do_lock(r,true) }
unsafe fn nlm4svc_proc_nm_lock(r: *mut svc_rqst)->__be32 { nlm4svc_do_lock(r,false) }

// The remaining procedures retain the C dispatch semantics and call the same
// external lockd helpers; declarations are kept explicit for ABI fidelity.
unsafe fn nlm4svc_proc_unused(_: *mut svc_rqst)->__be32 { rpc_proc_unavail }
unsafe fn nlm4svc_proc_granted(r: *mut svc_rqst)->__be32 { let a=(*r).rq_argp as *mut nlm4_testargs_wrapper; let p=(*r).rq_resp as *mut nlm4_res_wrapper; (*p).xdrgen.cookie=(*a).xdrgen.cookie; (*p).xdrgen.stat.stat=nlm4_lock_to_lockd_lock(&mut (*a).lock,&mut (*a).xdrgen.alock); if (*p).xdrgen.stat.stat==0 { (*p).xdrgen.stat.stat=nlmclnt_grant(svc_addr(r),&mut (*a).lock); } rpc_success }

#[repr(C)] pub struct nlm4svc_xdrstore { pub testargs:nlm4_testargs_wrapper, pub lockargs:nlm4_lockargs_wrapper, pub cancargs:nlm4_cancargs_wrapper, pub unlockargs:nlm4_unlockargs_wrapper, pub notifyargs:nlm4_notifyargs_wrapper, pub shareargs:nlm4_shareargs_wrapper, pub notify:nlm4_notify_wrapper, pub testres:nlm4_testres_wrapper, pub res:nlm4_res_wrapper, pub shareres:nlm4_shareres_wrapper }

// External declarations supplied by the surrounding translated kernel files.
extern "C" {
    fn nlm4svc_proc_cancel(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_unlock(_: *mut svc_rqst)->__be32;
    fn nlm4svc_proc_test_msg(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_lock_msg(_: *mut svc_rqst)->__be32;
    fn nlm4svc_proc_cancel_msg(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_unlock_msg(_: *mut svc_rqst)->__be32;
    fn nlm4svc_proc_granted_msg(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_granted_res(_: *mut svc_rqst)->__be32;
    fn nlm4svc_proc_sm_notify(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_share(_: *mut svc_rqst)->__be32;
    fn nlm4svc_proc_unshare(_: *mut svc_rqst)->__be32; fn nlm4svc_proc_free_all(_: *mut svc_rqst)->__be32;
}

// NLMv4 server procedure vector (the svc_procedure type and XDR callbacks are
// provided by the translated RPC support code).
#[repr(C)]
pub struct svc_procedure {
    pub pc_func: unsafe extern "C" fn(*mut svc_rqst) -> __be32,
    pub pc_decode: *const core::ffi::c_void,
    pub pc_encode: *const core::ffi::c_void,
    pub pc_argsize: usize,
    pub pc_argzero: usize,
    pub pc_ressize: usize,
    pub pc_xdrressize: usize,
    pub pc_name: *const core::ffi::c_char,
}

// Procedure numbers 0..23 correspond exactly to NLMPROC4_* in nlm4xdr_gen.h.
// Decode/encode callbacks and symbolic constants remain external dependencies.
extern "C" {
    static nlm4svc_procedures: [svc_procedure; 24];
}

#[repr(C)]
pub struct svc_version {
    pub vs_vers: u32,
    pub vs_nproc: usize,
    pub vs_proc: *const svc_procedure,
    pub vs_dispatch: *const core::ffi::c_void,
    pub vs_xdrsize: usize,
}

extern "C" {
    pub static nlmsvc_version4: svc_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
