// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/lockd/svc.c
 *
 * This is the central lockd service.
 *
 * FIXME: Separate the lockd NFS server functionality from the lockd NFS
 *        client functionality. Oh why didn't Sun create two separate
 *        services in the first place?
 *
 * Authors: Olaf Kirch (okir@monad.swb.de)
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// C dependencies supplied by the surrounding kernel translation unit.

const NLMDBG_FACILITY: u32 = NLMDBG_SVC;

static mut nlmsvc_program: svc_program = svc_program::default();
pub static mut nlmsvc_ops: *const nlmsvc_binding = core::ptr::null();
static mut nlmsvc_mutex: mutex = mutex::new();
static mut nlmsvc_users: u32 = 0;
static mut nlmsvc_serv: *mut svc_serv = core::ptr::null_mut();

unsafe fn nlmsvc_request_retry(_tl: *mut timer_list) {
    svc_wake_up(nlmsvc_serv);
}

static mut nlmsvc_retry: timer_list = timer_list::new(nlmsvc_request_retry);
pub static mut lockd_net_id: u32 = 0;

static mut nlm_grace_period: c_ulong = 0;
pub static mut nlm_timeout: c_ulong = LOCKD_DFLT_TIMEO as c_ulong;
static mut nlm_udpport: c_int = 0;
static mut nlm_tcpport: c_int = 0;

static nlm_grace_period_min: c_ulong = 0;
static nlm_grace_period_max: c_ulong = 240;
static nlm_timeout_min: c_ulong = 3;
static nlm_timeout_max: c_ulong = 20;

#[cfg(CONFIG_SYSCTL)]
static nlm_port_min: c_int = 0;
#[cfg(CONFIG_SYSCTL)]
static nlm_port_max: c_int = 65535;
#[cfg(CONFIG_SYSCTL)]
static mut nlm_sysctl_table: *mut ctl_table_header = core::ptr::null_mut();

unsafe fn get_lockd_grace_period(net: *mut net) -> c_ulong {
    let ln = net_generic(net, lockd_net_id);
    if (*ln).gracetime != 0 { return (*ln).gracetime as c_ulong * HZ as c_ulong; }
    if nlm_grace_period != 0 {
        return roundup(nlm_grace_period, nlm_timeout) * HZ as c_ulong;
    }
    nlm_timeout * 5 * HZ as c_ulong
}

unsafe fn grace_ender(grace: *mut work_struct) {
    let dwork = to_delayed_work(grace);
    let ln = container_of!(dwork, lockd_net, grace_period_end);
    locks_end_grace(&mut (*ln).lockd_manager);
}

unsafe fn set_grace_period(net: *mut net) {
    let grace_period = get_lockd_grace_period(net);
    let ln = net_generic(net, lockd_net_id);
    locks_start_grace(net, &mut (*ln).lockd_manager);
    cancel_delayed_work_sync(&mut (*ln).grace_period_end);
    schedule_delayed_work(&mut (*ln).grace_period_end, grace_period);
}

unsafe fn lockd(vrqstp: *mut c_void) -> c_int {
    let rqstp = vrqstp as *mut svc_rqst;
    let net = &mut init_net as *mut net;
    let ln = net_generic(net, lockd_net_id);
    svc_thread_init_status(rqstp, 0);
    set_freezable();
    dprintk!("NFS locking service started (ver {}).\\n", LOCKD_VERSION);
    while !svc_thread_should_stop(rqstp) {
        nlmsvc_retry_blocked(rqstp);
        svc_recv(rqstp, 0);
    }
    if !rcu_access_pointer(nlmsvc_ops).is_null() { nlmsvc_invalidate_all(); }
    nlm_shutdown_hosts();
    cancel_delayed_work_sync(&mut (*ln).grace_period_end);
    locks_end_grace(&mut (*ln).lockd_manager);
    dprintk!("lockd_down: service stopped\\n");
    svc_exit_thread(rqstp);
    0
}

unsafe fn create_lockd_listener(serv: *mut svc_serv, name: *const c_char,
    net: *mut net, family: c_int, port: u16, cred: *const cred) -> c_int {
    let xprt = svc_find_xprt(serv, name, net, family, 0);
    if xprt.is_null() { return svc_xprt_create(serv, name, net, family, port, SVC_SOCK_DEFAULTS, cred); }
    svc_xprt_put(xprt);
    0
}

unsafe fn create_lockd_family(serv: *mut svc_serv, net: *mut net, family: c_int,
    cred: *const cred) -> c_int {
    let ln = net_generic(net, lockd_net_id);
    let err = create_lockd_listener(serv, c"udp".as_ptr(), net, family,
        if (*ln).udp_port != 0 { (*ln).udp_port } else { nlm_udpport as u16 }, cred);
    if err < 0 { return err; }
    create_lockd_listener(serv, c"tcp".as_ptr(), net, family,
        if (*ln).tcp_port != 0 { (*ln).tcp_port } else { nlm_tcpport as u16 }, cred)
}

unsafe fn make_socks(serv: *mut svc_serv, net: *mut net, cred: *const cred) -> c_int {
    static mut warned: c_int = 0;
    let mut err = create_lockd_family(serv, net, PF_INET, cred);
    if err < 0 { return make_socks_error(serv, net, err, &mut warned); }
    err = create_lockd_family(serv, net, PF_INET6, cred);
    if err < 0 && err != -EAFNOSUPPORT { return make_socks_error(serv, net, err, &mut warned); }
    warned = 0;
    0
}

unsafe fn make_socks_error(serv: *mut svc_serv, net: *mut net, err: c_int, warned: *mut c_int) -> c_int {
    if *warned == 0 { printk!(KERN_WARNING, "lockd_up: makesock failed, error={}\\n", err); }
    *warned += 1;
    svc_xprt_destroy_all(serv, net, true);
    err
}

unsafe fn lockd_up_net(serv: *mut svc_serv, net: *mut net, cred: *const cred) -> c_int {
    let ln = net_generic(net, lockd_net_id);
    (*ln).nlmsvc_users += 1;
    if (*ln).nlmsvc_users != 1 { return 0; }
    let error = svc_bind(serv, net);
    if error != 0 { (*ln).nlmsvc_users -= 1; return error; }
    let error = make_socks(serv, net, cred);
    if error < 0 { (*ln).nlmsvc_users -= 1; return error; }
    set_grace_period(net);
    dprintk!("{}: per-net data created; net={:x}\\n", __func__, (*net).ns.inum);
    0
}

unsafe fn lockd_down_net(serv: *mut svc_serv, net: *mut net) {
    let ln = net_generic(net, lockd_net_id);
    if (*ln).nlmsvc_users != 0 {
        (*ln).nlmsvc_users -= 1;
        if (*ln).nlmsvc_users == 0 {
            nlm_shutdown_hosts_net(net);
            cancel_delayed_work_sync(&mut (*ln).grace_period_end);
            locks_end_grace(&mut (*ln).lockd_manager);
            svc_xprt_destroy_all(serv, net, true);
        }
    } else {
        pr_err!("{}: no users! net={:x}\\n", __func__, (*net).ns.inum);
        BUG!();
    }
}

unsafe fn lockd_inetaddr_event(_this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    if event != NETDEV_DOWN { return NOTIFY_DONE; }
    let ifa = ptr as *mut in_ifaddr;
    if !nlmsvc_serv.is_null() {
        dprintk!("lockd_inetaddr_event: removed %pI4\\n", &(*ifa).ifa_local);
        let mut sin: sockaddr_in = core::mem::zeroed();
        sin.sin_family = AF_INET;
        sin.sin_addr.s_addr = (*ifa).ifa_local;
        svc_age_temp_xprts_now(nlmsvc_serv, &sin as *const _ as *const sockaddr);
    }
    NOTIFY_DONE
}

static mut lockd_inetaddr_notifier: notifier_block = notifier_block { notifier_call: lockd_inetaddr_event };

#[cfg(IS_ENABLED_CONFIG_IPV6)]
unsafe fn lockd_inet6addr_event(_this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    if event != NETDEV_DOWN { return NOTIFY_DONE; }
    let ifa = ptr as *mut inet6_ifaddr;
    if !nlmsvc_serv.is_null() {
        dprintk!("lockd_inet6addr_event: removed %pI6\\n", &(*ifa).addr);
        let mut sin6: sockaddr_in6 = core::mem::zeroed();
        sin6.sin6_family = AF_INET6;
        sin6.sin6_addr = (*ifa).addr;
        if ipv6_addr_type(&sin6.sin6_addr) & IPV6_ADDR_LINKLOCAL != 0 { sin6.sin6_scope_id = (*(*ifa).idev).dev.ifindex; }
        svc_age_temp_xprts_now(nlmsvc_serv, &sin6 as *const _ as *const sockaddr);
    }
    NOTIFY_DONE
}

#[cfg(IS_ENABLED_CONFIG_IPV6)]
static mut lockd_inet6addr_notifier: notifier_block = notifier_block { notifier_call: lockd_inet6addr_event };

unsafe fn lockd_get() -> c_int {
    if !nlmsvc_serv.is_null() { nlmsvc_users += 1; return 0; }
    if nlmsvc_users != 0 { printk!(KERN_WARNING, "lockd_up: no pid, {} users??\\n", nlmsvc_users); }
    let bufsize = 1024 + max(nlmsvc_version1.vs_xdrsize, nlmsvc_version3.vs_xdrsize);
    let serv = svc_create(&nlmsvc_program, bufsize, lockd);
    if serv.is_null() { printk!(KERN_WARNING, "lockd_up: create service failed\\n"); return -ENOMEM; }
    let error = svc_set_num_threads(serv, 0, 1);
    if error < 0 { svc_destroy(&mut (serv as *mut svc_serv)); return error; }
    nlmsvc_serv = serv;
    register_inetaddr_notifier(&mut lockd_inetaddr_notifier);
    #[cfg(IS_ENABLED_CONFIG_IPV6)] register_inet6addr_notifier(&mut lockd_inet6addr_notifier);
    dprintk!("lockd_up: service created\\n");
    nlmsvc_users += 1;
    0
}

unsafe fn lockd_put() {
    if WARN!(nlmsvc_users <= 0, "lockd_down: no users!\\n") { return; }
    nlmsvc_users -= 1;
    if nlmsvc_users != 0 { return; }
    unregister_inetaddr_notifier(&mut lockd_inetaddr_notifier);
    #[cfg(IS_ENABLED_CONFIG_IPV6)] unregister_inet6addr_notifier(&mut lockd_inet6addr_notifier);
    svc_set_num_threads(nlmsvc_serv, 0, 0);
    timer_delete_sync(&mut nlmsvc_retry);
    svc_destroy(&mut nlmsvc_serv);
    dprintk!("lockd_down: service destroyed\\n");
}

pub unsafe fn lockd_up(net: *mut net, cred: *const cred) -> c_int {
    mutex_lock(&mut nlmsvc_mutex);
    let mut error = lockd_get();
    if error == 0 { error = lockd_up_net(nlmsvc_serv, net, cred); if error < 0 { lockd_put(); } }
    mutex_unlock(&mut nlmsvc_mutex);
    error
}

pub unsafe fn lockd_down(net: *mut net) {
    mutex_lock(&mut nlmsvc_mutex);
    lockd_down_net(nlmsvc_serv, net);
    lockd_put();
    mutex_unlock(&mut nlmsvc_mutex);
}

#[cfg(CONFIG_SYSCTL)]
static nlm_sysctls: [ctl_table; 6] = [
    ctl_table::new("nlm_grace_period", &nlm_grace_period, core::mem::size_of::<c_ulong>(), 0o644, proc_doulongvec_minmax, &nlm_grace_period_min, &nlm_grace_period_max),
    ctl_table::new("nlm_timeout", &nlm_timeout, core::mem::size_of::<c_ulong>(), 0o644, proc_doulongvec_minmax, &nlm_timeout_min, &nlm_timeout_max),
    ctl_table::new("nlm_udpport", &nlm_udpport, core::mem::size_of::<c_int>(), 0o644, proc_dointvec_minmax, &nlm_port_min, &nlm_port_max),
    ctl_table::new("nlm_tcpport", &nlm_tcpport, core::mem::size_of::<c_int>(), 0o644, proc_dointvec_minmax, &nlm_port_min, &nlm_port_max),
    ctl_table::new("nsm_use_hostnames", &nsm_use_hostnames, core::mem::size_of::<bool>(), 0o644, proc_dobool, core::ptr::null(), core::ptr::null()),
    ctl_table::new("nsm_local_state", &nsm_local_state, core::mem::size_of_val(&nsm_local_state), 0o644, proc_douintvec, SYSCTL_ZERO, core::ptr::null()),
];

unsafe fn is_callback(proc: u32) -> bool {
    proc == NLMPROC_GRANTED || proc == NLMPROC_GRANTED_MSG || proc == NLMPROC_TEST_RES ||
    proc == NLMPROC_LOCK_RES || proc == NLMPROC_CANCEL_RES || proc == NLMPROC_UNLOCK_RES || proc == NLMPROC_NSM_NOTIFY
}

unsafe fn lockd_authenticate(rqstp: *mut svc_rqst) -> svc_auth_status {
    (*rqstp).rq_client = core::ptr::null_mut();
    match (*(*rqstp).rq_authop).flavour {
        RPC_AUTH_NULL | RPC_AUTH_UNIX => {
            (*rqstp).rq_auth_stat = rpc_auth_ok;
            if (*rqstp).rq_proc == 0 || is_callback((*rqstp).rq_proc) { return SVC_OK; }
            svc_set_client(rqstp)
        }
        _ => { (*rqstp).rq_auth_stat = rpc_autherr_badcred; SVC_DENIED }
    }
}

unsafe fn lockd_init_net(net: *mut net) -> c_int {
    let ln = net_generic(net, lockd_net_id);
    INIT_DELAYED_WORK!(&mut (*ln).grace_period_end, grace_ender);
    INIT_LIST_HEAD!(&mut (*ln).lockd_manager.list);
    (*ln).lockd_manager.block_opens = false;
    INIT_LIST_HEAD!(&mut (*ln).nsm_handles);
    0
}

unsafe fn lockd_exit_net(net: *mut net) {
    let ln = net_generic(net, lockd_net_id);
    WARN_ONCE!(!list_empty(&(*ln).lockd_manager.list), "net {:x} {}: lockd_manager.list is not empty\\n", (*net).ns.inum, __func__);
    WARN_ONCE!(!list_empty(&(*ln).nsm_handles), "net {:x} {}: nsm_handles list is not empty\\n", (*net).ns.inum, __func__);
    WARN_ONCE!(delayed_work_pending(&(*ln).grace_period_end), "net {:x} {}: grace_period_end was not cancelled\\n", (*net).ns.inum, __func__);
}

static mut lockd_net_ops: pernet_operations = pernet_operations { init: lockd_init_net, exit: lockd_exit_net, id: &mut lockd_net_id, size: core::mem::size_of::<lockd_net>() };

unsafe fn init_nlm() -> c_int {
    #[cfg(CONFIG_SYSCTL)] { nlm_sysctl_table = register_sysctl(c"fs/nfs".as_ptr(), nlm_sysctls.as_ptr()); if nlm_sysctl_table.is_null() { return -ENOMEM; } }
    let err = register_pernet_subsys(&mut lockd_net_ops); if err != 0 { #[cfg(CONFIG_SYSCTL)] unregister_sysctl_table(nlm_sysctl_table); return err; }
    let err = genl_register_family(&mut lockd_nl_family); if err != 0 { unregister_pernet_subsys(&mut lockd_net_ops); #[cfg(CONFIG_SYSCTL)] unregister_sysctl_table(nlm_sysctl_table); return err; }
    let err = lockd_create_procfs(); if err != 0 { genl_unregister_family(&mut lockd_nl_family); unregister_pernet_subsys(&mut lockd_net_ops); #[cfg(CONFIG_SYSCTL)] unregister_sysctl_table(nlm_sysctl_table); return err; }
    0
}

unsafe fn exit_nlm() {
    nlm_shutdown_hosts();
    genl_unregister_family(&mut lockd_nl_family);
    lockd_remove_procfs();
    unregister_pernet_subsys(&mut lockd_net_ops);
    #[cfg(CONFIG_SYSCTL)] unregister_sysctl_table(nlm_sysctl_table);
}

module_init!(init_nlm);
module_exit!(exit_nlm);

pub unsafe fn nlmsvc_dispatch(rqstp: *mut svc_rqst) -> c_int {
    let procp = (*rqstp).rq_procinfo;
    let statp = (*rqstp).rq_accept_statp;
    if !((*procp).pc_decode)(rqstp, &mut (*rqstp).rq_arg_stream) { *statp = rpc_garbage_args; return 1; }
    *statp = ((*procp).pc_func)(rqstp);
    if *statp == rpc_drop_reply { return 0; }
    if *statp != rpc_success { return 1; }
    if !((*procp).pc_encode)(rqstp, &mut (*rqstp).rq_res_stream) { *statp = rpc_system_err; }
    1
}

static nlmsvc_version: [*const svc_version; 5] = [core::ptr::null(), &nlmsvc_version1, core::ptr::null(), &nlmsvc_version3, &nlmsvc_version4];
const NLM_NRVERS: usize = core::mem::size_of_val(&nlmsvc_version) / core::mem::size_of::<*const svc_version>();

static mut nlmsvc_program: svc_program = svc_program {
    pg_prog: NLM_PROGRAM, pg_nvers: NLM_NRVERS, pg_vers: nlmsvc_version.as_ptr(), pg_name: c"lockd".as_ptr(), pg_class: c"nfsd".as_ptr(),
    pg_authenticate: Some(lockd_authenticate), pg_init_request: svc_generic_init_request, pg_rpcbind_set: svc_generic_rpcbind_set,
};

pub unsafe fn lockd_nl_server_set_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let net = genl_info_net(info); let ln = net_generic(net, lockd_net_id);
    if GENL_REQ_ATTR_CHECK(info, LOCKD_A_SERVER_GRACETIME) { return -EINVAL; }
    if let Some(attr) = (*info).attrs[LOCKD_A_SERVER_GRACETIME] { let v = nla_get_u32(attr); if v as c_ulong > nlm_grace_period_max { return -EINVAL; } (*ln).gracetime = v; if net == &mut init_net { nlm_grace_period = v as c_ulong; } }
    if let Some(attr) = (*info).attrs[LOCKD_A_SERVER_TCP_PORT] { (*ln).tcp_port = nla_get_u16(attr); if net == &mut init_net { nlm_tcpport = (*ln).tcp_port as c_int; } }
    if let Some(attr) = (*info).attrs[LOCKD_A_SERVER_UDP_PORT] { (*ln).udp_port = nla_get_u16(attr); if net == &mut init_net { nlm_udpport = (*ln).udp_port as c_int; } }
    0
}

pub unsafe fn lockd_nl_server_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let net = genl_info_net(info); let ln = net_generic(net, lockd_net_id);
    let skb = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL); if skb.is_null() { return -ENOMEM; }
    let hdr = genlmsg_iput(skb, info); if hdr.is_null() { nlmsg_free(skb); return -EMSGSIZE; }
    let err = nla_put_u32(skb, LOCKD_A_SERVER_GRACETIME, (*ln).gracetime) || nla_put_u16(skb, LOCKD_A_SERVER_TCP_PORT, (*ln).tcp_port) || nla_put_u16(skb, LOCKD_A_SERVER_UDP_PORT, (*ln).udp_port);
    if err { nlmsg_free(skb); return err as c_int; }
    genlmsg_end(skb, hdr); genlmsg_reply(skb, info)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
