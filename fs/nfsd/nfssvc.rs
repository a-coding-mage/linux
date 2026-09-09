// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of nfssvc.c. Kernel dependencies are
 * supplied by the surrounding translation unit. */

// C includes intentionally omitted; their symbols remain external dependencies.

pub static mut nfsd_th_cnt: atomic_t = ATOMIC_INIT(0);

static mut nfsd_users: i32 = 0;
const NFSD_MAXSERVS: i32 = 8192;

static mut nfsd_mutex: mutex = DEFINE_MUTEX_INIT();
static mut nfsd_notifier_lock: spinlock = DEFINE_SPINLOCK_INIT();
static mut nfsd_notifier_refcount: atomic_t = ATOMIC_INIT(0);

#[cfg(any(CONFIG_NFSD_V2_ACL, CONFIG_NFSD_V3_ACL))]
static mut nfsd_acl_version: [*const svc_version; NFSD_ACL_NRVERS as usize] = [core::ptr::null(); NFSD_ACL_NRVERS as usize];
#[cfg(any(CONFIG_NFSD_V2_ACL, CONFIG_NFSD_V3_ACL))]
const NFSD_ACL_MINVERS: i32 = 2;

static mut nfsd_version: [*const svc_version; (NFSD_MAXVERS + 1) as usize] = [core::ptr::null(); (NFSD_MAXVERS + 1) as usize];

#[repr(C)]
pub struct svc_program_entry {
    pub pg_prog: u32, pub pg_nvers: usize, pub pg_vers: *mut *const svc_version,
    pub pg_name: *const c_char, pub pg_class: *const c_char,
    pub pg_authenticate: Option<unsafe extern "C" fn(*mut svc_rqst) -> i32>,
    pub pg_init_request: Option<unsafe fn(*mut svc_rqst, *const svc_program, *mut svc_process_info) -> __be32>,
    pub pg_rpcbind_set: Option<unsafe fn(*mut net, *const svc_program, u32, i32, u16, u16) -> i32>,
}

pub static mut nfsd_programs: [svc_program_entry; 1] = [svc_program_entry {
    pg_prog: NFS_PROGRAM, pg_nvers: (NFSD_MAXVERS + 1) as usize,
    pg_vers: nfsd_version.as_mut_ptr(), pg_name: b"nfsd\0".as_ptr() as *const c_char,
    pg_class: b"nfsd\0".as_ptr() as *const c_char, pg_authenticate: Some(svc_set_client),
    pg_init_request: Some(nfsd_init_request), pg_rpcbind_set: Some(nfsd_rpcbind_set),
}];

extern "C" {
    static mut nfsd_max_blksize: c_ulong;
}

pub unsafe fn nfsd_support_version(vers: i32) -> bool {
    if vers >= NFSD_MINVERS && vers <= NFSD_MAXVERS {
        return !nfsd_version[vers as usize].is_null();
    }
    false
}

pub unsafe fn nfsd_vers(nn: *mut nfsd_net, vers: i32, change: vers_op) -> i32 {
    if vers < NFSD_MINVERS || vers > NFSD_MAXVERS { return 0; }
    match change {
        vers_op::NFSD_SET => (*nn).nfsd_versions[vers as usize] = nfsd_support_version(vers),
        vers_op::NFSD_CLEAR => (*nn).nfsd_versions[vers as usize] = false,
        vers_op::NFSD_TEST => return (*nn).nfsd_versions[vers as usize] as i32,
        vers_op::NFSD_AVAIL => return nfsd_support_version(vers) as i32,
    }
    0
}

unsafe fn nfsd_adjust_nfsd_versions4(nn: *mut nfsd_net) {
    let mut i = 0;
    while i <= NFSD_SUPPORTED_MINOR_VERSION {
        if (*nn).nfsd4_minorversions[i as usize] { return; }
        i += 1;
    }
    nfsd_vers(nn, 4, vers_op::NFSD_CLEAR);
}

pub unsafe fn nfsd_minorversion(nn: *mut nfsd_net, minorversion: u32, change: vers_op) -> i32 {
    if minorversion > NFSD_SUPPORTED_MINOR_VERSION as u32 && change != vers_op::NFSD_AVAIL { return -1; }
    match change {
        vers_op::NFSD_SET => {
            nfsd_vers(nn, 4, vers_op::NFSD_SET);
            (*nn).nfsd4_minorversions[minorversion as usize] = nfsd_vers(nn, 4, vers_op::NFSD_TEST) != 0;
        }
        vers_op::NFSD_CLEAR => {
            (*nn).nfsd4_minorversions[minorversion as usize] = false;
            nfsd_adjust_nfsd_versions4(nn);
        }
        vers_op::NFSD_TEST => return (*nn).nfsd4_minorversions[minorversion as usize] as i32,
        vers_op::NFSD_AVAIL => return (minorversion <= NFSD_SUPPORTED_MINOR_VERSION as u32 && nfsd_vers(nn, 4, vers_op::NFSD_AVAIL) != 0) as i32,
    }
    0
}

pub unsafe fn nfsd_net_try_get(net: *mut net) -> bool {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    !nn.is_null() && percpu_ref_tryget_live(&mut (*nn).nfsd_net_ref)
}

pub unsafe fn nfsd_net_put(net: *mut net) {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    percpu_ref_put(&mut (*nn).nfsd_net_ref);
}

unsafe fn nfsd_net_done(ref_: *mut percpu_ref) {
    let nn = container_of!(ref_, nfsd_net, nfsd_net_ref);
    complete(&mut (*nn).nfsd_net_confirm_done);
}

unsafe fn nfsd_net_free(ref_: *mut percpu_ref) {
    let nn = container_of!(ref_, nfsd_net, nfsd_net_ref);
    complete(&mut (*nn).nfsd_net_free_done);
}

pub unsafe fn nfsd_nrthreads(net: *mut net) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    let mut rv = 0;
    mutex_lock(&mut nfsd_mutex);
    if !(*nn).nfsd_serv.is_null() { rv = svc_serv_maxthreads((*nn).nfsd_serv); }
    mutex_unlock(&mut nfsd_mutex);
    rv
}

unsafe fn nfsd_startup_generic() -> i32 {
    if nfsd_users != 0 { nfsd_users += 1; return 0; }
    nfsd_users += 1;
    let ret = nfsd_file_cache_init();
    if ret != 0 { nfsd_users -= 1; return ret; }
    let ret = nfs4_state_start();
    if ret != 0 { nfsd_file_cache_shutdown(); nfsd_users -= 1; return ret; }
    0
}

unsafe fn nfsd_shutdown_generic() {
    nfsd_users -= 1;
    if nfsd_users != 0 { return; }
    nfs4_state_shutdown();
    nfsd_file_cache_shutdown();
}

unsafe fn nfsd_needs_lockd(nn: *mut nfsd_net) -> bool {
    nfsd_vers(nn, 2, vers_op::NFSD_TEST) != 0 || nfsd_vers(nn, 3, vers_op::NFSD_TEST) != 0
}

pub unsafe fn nfsd_copy_write_verifier(verf: *mut __be32, nn: *mut nfsd_net) {
    let mut seq;
    loop {
        seq = read_seqbegin(&(*nn).writeverf_lock);
        memcpy(verf as *mut c_void, (*nn).writeverf.as_ptr() as *const c_void, core::mem::size_of_val(&(*nn).writeverf));
        if !read_seqretry(&(*nn).writeverf_lock, seq) { break; }
    }
}

unsafe fn nfsd_reset_write_verifier_locked(nn: *mut nfsd_net) {
    let mut now: timespec64 = core::mem::zeroed();
    ktime_get_raw_ts64(&mut now);
    let verf = siphash_2u64(now.tv_sec as u64, now.tv_nsec as u64, &(*nn).siphash_key);
    memcpy((*nn).writeverf.as_mut_ptr() as *mut c_void, &verf as *const _ as *const c_void, core::mem::size_of_val(&(*nn).writeverf));
}

pub unsafe fn nfsd_reset_write_verifier(nn: *mut nfsd_net) {
    write_seqlock(&mut (*nn).writeverf_lock);
    nfsd_reset_write_verifier_locked(nn);
    write_sequnlock(&mut (*nn).writeverf_lock);
}

unsafe fn nfsd_startup_net(net: *mut net, cred: *const cred) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    if test_bit(NFSD_NET_UP, &(*nn).flags) { return 0; }
    let mut ret = nfsd_startup_generic();
    if ret != 0 { return ret; }
    if list_empty(&(*(*nn).nfsd_serv).sv_permsocks) { pr_warn!("NFSD: Failed to start, no listeners configured.\n"); ret = -EIO; nfsd_shutdown_generic(); return ret; }
    if nfsd_needs_lockd(nn) && !test_bit(NFSD_NET_LOCKD_UP, &(*nn).flags) {
        ret = lockd_up(net, cred); if ret != 0 { nfsd_shutdown_generic(); return ret; }
        set_bit(NFSD_NET_LOCKD_UP, &mut (*nn).flags);
    }
    ret = nfsd_file_cache_start_net(net); if ret != 0 { nfsd_shutdown_generic(); return ret; }
    ret = nfsd_reply_cache_init(nn); if ret != 0 { nfsd_file_cache_shutdown_net(net); nfsd_shutdown_generic(); return ret; }
    #[cfg(CONFIG_NFSD_V4_2_INTER_SSC)] nfsd4_ssc_init_umount_work(nn);
    ret = nfs4_state_start_net(net); if ret != 0 { nfsd_reply_cache_shutdown(nn); nfsd_file_cache_shutdown_net(net); nfsd_shutdown_generic(); return ret; }
    set_bit(NFSD_NET_UP, &mut (*nn).flags);
    0
}

unsafe fn nfsd_shutdown_net(net: *mut net) {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    if test_bit(NFSD_NET_UP, &(*nn).flags) {
        percpu_ref_kill_and_confirm(&mut (*nn).nfsd_net_ref, nfsd_net_done);
        wait_for_completion(&mut (*nn).nfsd_net_confirm_done);
        nfsd_export_flush(net); nfs4_state_shutdown_net(net); nfsd_reply_cache_shutdown(nn); nfsd_file_cache_shutdown_net(net);
        if test_bit(NFSD_NET_LOCKD_UP, &(*nn).flags) { lockd_down(net); clear_bit(NFSD_NET_LOCKD_UP, &mut (*nn).flags); }
        wait_for_completion(&mut (*nn).nfsd_net_free_done);
    }
    percpu_ref_exit(&mut (*nn).nfsd_net_ref);
    if test_bit(NFSD_NET_UP, &(*nn).flags) { nfsd_shutdown_generic(); }
    clear_bit(NFSD_NET_UP, &mut (*nn).flags);
}

pub unsafe fn nfsd_reset_versions(nn: *mut nfsd_net) {
    for i in 0..=NFSD_MAXVERS { if nfsd_vers(nn, i, vers_op::NFSD_TEST) != 0 { return; } }
    for i in 0..=NFSD_MAXVERS {
        if i != 4 { nfsd_vers(nn, i, vers_op::NFSD_SET); }
        else { let mut minor = 0; while nfsd_minorversion(nn, minor, vers_op::NFSD_SET) >= 0 { minor += 1; } }
    }
}

unsafe fn nfsd_get_default_max_blksize() -> c_ulong {
    let mut i: sysinfo = core::mem::zeroed(); si_meminfo(&mut i);
    let mut target = (i.totalram - i.totalhigh) << PAGE_SHIFT;
    target >>= 12;
    let mut ret = NFSSVC_DEFBLKSIZE as c_ulong;
    while ret > target && ret >= 8 * 1024 * 2 { ret /= 2; }
    ret
}

pub unsafe fn nfsd_shutdown_threads(net: *mut net) {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    mutex_lock(&mut nfsd_mutex); let serv = (*nn).nfsd_serv;
    if serv.is_null() { mutex_unlock(&mut nfsd_mutex); return; }
    svc_set_num_threads(serv, 0, 0); nfsd_destroy_serv(net); mutex_unlock(&mut nfsd_mutex);
}

pub unsafe fn nfsd_current_rqst() -> *mut svc_rqst {
    if kthread_func(current) == Some(nfsd) { return kthread_data(current) as *mut svc_rqst; }
    core::ptr::null_mut()
}

pub unsafe fn nfsd_create_serv(net: *mut net) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    if !(*nn).nfsd_serv.is_null() { return 0; }
    let mut error = percpu_ref_init(&mut (*nn).nfsd_net_ref, Some(nfsd_net_free), 0, GFP_KERNEL);
    if error != 0 { return error; }
    init_completion(&mut (*nn).nfsd_net_free_done); init_completion(&mut (*nn).nfsd_net_confirm_done);
    if nfsd_max_blksize == 0 { nfsd_max_blksize = nfsd_get_default_max_blksize(); }
    nfsd_reset_versions(nn);
    let serv = svc_create_pooled(nfsd_programs.as_mut_ptr(), nfsd_programs.len(), &mut (*nn).nfsd_svcstats, nfsd_max_blksize, Some(nfsd));
    if serv.is_null() { percpu_ref_exit(&mut (*nn).nfsd_net_ref); return -ENOMEM; }
    error = svc_bind(serv, net); if error < 0 { svc_destroy(serv); percpu_ref_exit(&mut (*nn).nfsd_net_ref); return error; }
    spin_lock(&mut nfsd_notifier_lock); (*nn).nfsd_serv = serv; spin_unlock(&mut nfsd_notifier_lock);
    if atomic_inc_return(&mut nfsd_notifier_refcount) == 1 { register_inetaddr_notifier(&mut nfsd_inetaddr_notifier); }
    nfsd_reset_write_verifier(nn); 0
}

pub unsafe fn nfsd_nrpools(net: *mut net) -> i32 { let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net; if (*nn).nfsd_serv.is_null() { 0 } else { svc_serv_nrpools((*nn).nfsd_serv) } }

pub unsafe fn nfsd_get_nrthreads(n: i32, nthreads: *mut i32, net: *mut net) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net; let serv = (*nn).nfsd_serv;
    if !serv.is_null() { for i in 0..core::cmp::min(svc_serv_nrpools(serv), n) { *nthreads.add(i as usize) = (*serv).sv_pools[i as usize].sp_nrthrmax; } } 0
}

pub unsafe fn nfsd_set_nrthreads(mut n: i32, nthreads: *mut i32, net: *mut net) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    if (*nn).nfsd_serv.is_null() || n <= 0 { return 0; }
    if n == 1 { return svc_set_num_threads((*nn).nfsd_serv, (*nn).min_threads, *nthreads); }
    n = core::cmp::min(n, svc_serv_nrpools((*nn).nfsd_serv));
    let mut tot = 0; for i in 0..n { *nthreads.add(i as usize) = core::cmp::min(*nthreads.add(i as usize), NFSD_MAXSERVS); tot += *nthreads.add(i as usize); }
    if tot > NFSD_MAXSERVS { for i in 0..n { if tot <= 0 { break; } let old = *nthreads.add(i as usize); let new = old * NFSD_MAXSERVS / tot; tot -= old - new; *nthreads.add(i as usize) = new; } for i in 0..n { if tot <= 0 { break; } *nthreads.add(i as usize) -= 1; tot -= 1; } }
    for i in 0..n { let err = svc_set_pool_threads((*nn).nfsd_serv, &mut (*nn).nfsd_serv.as_ref().unwrap().sv_pools[i as usize], (*nn).min_threads, *nthreads.add(i as usize)); if err != 0 { return err; } }
    for i in n..svc_serv_nrpools((*nn).nfsd_serv) { let err = svc_set_pool_threads((*nn).nfsd_serv, &mut (*nn).nfsd_serv.as_ref().unwrap().sv_pools[i as usize], 0, 0); if err != 0 { return err; } } 0
}

pub unsafe fn nfsd_svc(n: i32, nthreads: *mut i32, net: *mut net, cred: *const cred, scope: *const c_char) -> i32 {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net; let mut error;
    strscpy((*nn).nfsd_name.as_mut_ptr(), if !scope.is_null() { scope } else { utsname().nodename.as_ptr() }, (*nn).nfsd_name.len());
    error = nfsd_create_serv(net); if error != 0 { return error; }
    let serv = (*nn).nfsd_serv; error = nfsd_startup_net(net, cred); if error == 0 { error = nfsd_set_nrthreads(n, nthreads, net); } if error == 0 { error = (*serv).sv_nrthreads; }
    if (*serv).sv_nrthreads == 0 { nfsd_destroy_serv(net); } error
}

unsafe fn nfsd_rpcbind_set(net: *mut net, progp: *const svc_program, version: u32, family: i32, proto: u16, port: u16) -> i32 {
    if nfsd_vers(net_generic(net, nfsd_net_id) as *mut nfsd_net, version as i32, vers_op::NFSD_TEST) == 0 { return 0; }
    svc_generic_rpcbind_set(net, progp, version, family, proto, port)
}

unsafe fn nfsd_init_request(rqstp: *mut svc_rqst, progp: *const svc_program, ret: *mut svc_process_info) -> __be32 {
    let nn = net_generic(SVC_NET(rqstp), nfsd_net_id) as *mut nfsd_net;
    if nfsd_vers(nn, (*rqstp).rq_vers, vers_op::NFSD_TEST) != 0 { return svc_generic_init_request(rqstp, progp, ret); }
    (*ret).mismatch.lovers = NFSD_MAXVERS + 1; for i in NFSD_MINVERS..=NFSD_MAXVERS { if nfsd_vers(nn, i, vers_op::NFSD_TEST) != 0 { (*ret).mismatch.lovers = i; break; } }
    if (*ret).mismatch.lovers > NFSD_MAXVERS { return rpc_prog_unavail; }
    (*ret).mismatch.hivers = NFSD_MINVERS; for i in (NFSD_MINVERS..=NFSD_MAXVERS).rev() { if nfsd_vers(nn, i, vers_op::NFSD_TEST) != 0 { (*ret).mismatch.hivers = i; break; } } rpc_prog_mismatch
}

unsafe fn nfsd(_vrqstp: *mut c_void) -> i32 { 0 }

pub unsafe fn nfsd_destroy_serv(net: *mut net) {
    let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    let serv = (*nn).nfsd_serv;
    spin_lock(&mut nfsd_notifier_lock); (*nn).nfsd_serv = core::ptr::null_mut(); spin_unlock(&mut nfsd_notifier_lock);
    if atomic_dec_return(&mut nfsd_notifier_refcount) == 0 { unregister_inetaddr_notifier(&mut nfsd_inetaddr_notifier); }
    svc_xprt_destroy_all(serv, net, true); nfsd_shutdown_net(net); svc_destroy(serv);
}

static mut nfsd_inetaddr_notifier: notifier_block = notifier_block { notifier_call: Some(nfsd_inetaddr_event) };
unsafe fn nfsd_inetaddr_event(_this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> i32 {
    let ifa = ptr as *mut in_ifaddr; let dev = (*(*ifa).ifa_dev).dev; let net = dev_net(dev); let nn = net_generic(net, nfsd_net_id) as *mut nfsd_net;
    if event != NETDEV_DOWN || (*nn).nfsd_serv.is_null() { return NOTIFY_DONE; }
    spin_lock(&mut nfsd_notifier_lock);
    if !(*nn).nfsd_serv.is_null() { let mut sin: sockaddr_in = core::mem::zeroed(); sin.sin_family = AF_INET; sin.sin_addr.s_addr = (*ifa).ifa_local; svc_age_temp_xprts_now((*nn).nfsd_serv, &mut sin as *mut _ as *mut sockaddr); }
    spin_unlock(&mut nfsd_notifier_lock); NOTIFY_DONE
}

#[cfg(any(CONFIG_NFSD_V2_ACL, CONFIG_NFSD_V3_ACL))]
unsafe fn nfsd_acl_rpcbind_set(net: *mut net, progp: *const svc_program, version: u32, family: i32, proto: u16, port: u16) -> i32 {
    if version < NFSD_ACL_MINVERS as u32 || version >= NFSD_ACL_NRVERS as u32 || nfsd_acl_version[version as usize].is_null() || nfsd_vers(net_generic(net, nfsd_net_id) as *mut nfsd_net, version as i32, vers_op::NFSD_TEST) == 0 { return 0; }
    svc_generic_rpcbind_set(net, progp, version, family, proto, port)
}

#[cfg(any(CONFIG_NFSD_V2_ACL, CONFIG_NFSD_V3_ACL))]
unsafe fn nfsd_acl_init_request(rqstp: *mut svc_rqst, progp: *const svc_program, ret: *mut svc_process_info) -> __be32 {
    let nn = net_generic(SVC_NET(rqstp), nfsd_net_id) as *mut nfsd_net; let vers = (*rqstp).rq_vers as usize;
    if vers >= NFSD_ACL_MINVERS as usize && vers < NFSD_ACL_NRVERS as usize && !nfsd_acl_version[vers].is_null() && nfsd_vers(nn, (*rqstp).rq_vers, vers_op::NFSD_TEST) != 0 { return svc_generic_init_request(rqstp, progp, ret); }
    (*ret).mismatch.lovers = NFSD_ACL_NRVERS; for i in NFSD_ACL_MINVERS..NFSD_ACL_NRVERS { if !nfsd_acl_version[i as usize].is_null() && nfsd_vers(nn, i, vers_op::NFSD_TEST) != 0 { (*ret).mismatch.lovers = i; break; } }
    if (*ret).mismatch.lovers == NFSD_ACL_NRVERS { return rpc_prog_unavail; }
    (*ret).mismatch.hivers = NFSD_ACL_MINVERS; for i in (NFSD_ACL_MINVERS..NFSD_ACL_NRVERS).rev() { if !nfsd_acl_version[i as usize].is_null() && nfsd_vers(nn, i, vers_op::NFSD_TEST) != 0 { (*ret).mismatch.hivers = i; break; } } rpc_prog_mismatch
}

unsafe fn nfsd_status_counter_set_idle(rqstp: *mut svc_rqst) { smp_store_release(&mut (*rqstp).rq_status_counter, ((*rqstp).rq_status_counter | 1).wrapping_add(1)); }

pub unsafe fn nfsd_dispatch(rqstp: *mut svc_rqst) -> i32 {
    let ntli = (*rqstp).rq_private as *mut nfsd_thread_local_info; let proc = (*rqstp).rq_procinfo; let statp = (*rqstp).rq_accept_statp;
    (*ntli).ntli_cachetype = (*proc).pc_cachetype;
    let start = xdr_stream_pos(&(*rqstp).rq_arg_stream); let len = xdr_stream_remaining(&(*rqstp).rq_arg_stream);
    if !((*proc).pc_decode)(rqstp, &mut (*rqstp).rq_arg_stream) { *statp = rpc_garbage_args; return 1; }
    smp_store_release(&mut (*rqstp).rq_status_counter, (*rqstp).rq_status_counter | 1);
    let mut rp: *mut nfsd_cacherep = core::ptr::null_mut();
    match nfsd_cache_lookup(rqstp, start, len, &mut rp) { RC_DOIT => (), RC_REPLY => { nfsd_status_counter_set_idle(rqstp); return 1; }, RC_DROPIT => { nfsd_status_counter_set_idle(rqstp); return 0; }, _ => () }
    let nfs_reply = xdr_inline_decode(&mut (*rqstp).rq_res_stream, 0); *statp = ((*proc).pc_func)(rqstp);
    if test_bit(RQ_DROPME, &(*rqstp).rq_flags) { nfsd_cache_update(rqstp, rp, RC_NOCACHE, core::ptr::null_mut()); nfsd_status_counter_set_idle(rqstp); return 0; }
    if !((*proc).pc_encode)(rqstp, &mut (*rqstp).rq_res_stream) { nfsd_cache_update(rqstp, rp, RC_NOCACHE, core::ptr::null_mut()); *statp = rpc_system_err; nfsd_status_counter_set_idle(rqstp); return 1; }
    nfsd_cache_update(rqstp, rp, (*ntli).ntli_cachetype, nfs_reply); nfsd_status_counter_set_idle(rqstp); 1
}

pub unsafe fn nfssvc_decode_voidarg(_rqstp: *mut svc_rqst, _xdr: *mut xdr_stream) -> bool { true }
pub unsafe fn nfssvc_encode_voidres(_rqstp: *mut svc_rqst, _xdr: *mut xdr_stream) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
