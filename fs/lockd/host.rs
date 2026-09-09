// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/lockd/host.c. Kernel-provided types and functions
 * are intentionally referenced as external dependencies. */

const NLM_HOST_NRHASH: usize = 32;
const NLM_HOST_REBIND: c_ulong = 60 * HZ;
const NLM_HOST_EXPIRE: c_ulong = 300 * HZ;
const NLM_HOST_COLLECT: c_ulong = 120 * HZ;

static mut nlm_server_hosts: [hlist_head; NLM_HOST_NRHASH] = [hlist_head::ZERO; NLM_HOST_NRHASH];
static mut nlm_client_hosts: [hlist_head; NLM_HOST_NRHASH] = [hlist_head::ZERO; NLM_HOST_NRHASH];
static mut nrhosts: c_ulong = 0;
static mut nlm_host_mutex: mutex = mutex::ZERO;

#[repr(C)]
struct nlm_lookup_host_info {
    server: c_int, sap: *const sockaddr, salen: usize, protocol: c_ushort,
    version: u32, hostname: *const c_char, hostname_len: usize,
    noresvport: c_int, net: *mut net, cred: *const cred,
}

unsafe fn __nlm_hash32(n: __be32) -> c_uint {
    let v = n as u32;
    let hash = v ^ (v >> 16);
    hash ^ (hash >> 8)
}
unsafe fn __nlm_hash_addr4(sap: *const sockaddr) -> c_uint {
    __nlm_hash32(((*(sap as *const sockaddr_in)).sin_addr.s_addr))
}
unsafe fn __nlm_hash_addr6(sap: *const sockaddr) -> c_uint {
    let a = (*(sap as *const sockaddr_in6)).sin6_addr;
    __nlm_hash32(a.s6_addr32[0]) ^ __nlm_hash32(a.s6_addr32[1]) ^
        __nlm_hash32(a.s6_addr32[2]) ^ __nlm_hash32(a.s6_addr32[3])
}
unsafe fn nlm_hash_address(sap: *const sockaddr) -> c_uint {
    match (*sap).sa_family as c_int {
        AF_INET => __nlm_hash_addr4(sap),
        AF_INET6 => __nlm_hash_addr6(sap),
        _ => 0,
    } & (NLM_HOST_NRHASH as u32 - 1)
}

unsafe fn nlm_alloc_host(ni: *mut nlm_lookup_host_info, mut nsm: *mut nsm_handle) -> *mut nlm_host {
    let now = jiffies;
    if !nsm.is_null() { refcount_inc(&mut (*nsm).sm_count); }
    else { nsm = nsm_get_handle((*ni).net, (*ni).sap, (*ni).salen, (*ni).hostname, (*ni).hostname_len); if nsm.is_null() { return core::ptr::null_mut(); } }
    let host = kmalloc_obj::<nlm_host>();
    if host.is_null() { nsm_release(nsm); return core::ptr::null_mut(); }
    memcpy(nlm_addr(host), (*ni).sap, (*ni).salen);
    (*host).h_addrlen = (*ni).salen; rpc_set_port(nlm_addr(host), 0); (*host).h_srcaddrlen = 0;
    (*host).h_rpcclnt = core::ptr::null_mut(); (*host).h_name = (*nsm).sm_name;
    (*host).h_version = (*ni).version; (*host).h_proto = (*ni).protocol; (*host).h_reclaiming = 0;
    (*host).h_server = (*ni).server; (*host).h_noresvport = (*ni).noresvport; (*host).h_inuse = 0;
    init_waitqueue_head(&mut (*host).h_gracewait); init_rwsem(&mut (*host).h_rwsem);
    (*host).h_state = 0; (*host).h_nsmstate = 0; (*host).h_pidcount = 0; refcount_set(&mut (*host).h_count, 1);
    mutex_init(&mut (*host).h_mutex); (*host).h_nextrebind = now + NLM_HOST_REBIND; (*host).h_expires = now + NLM_HOST_EXPIRE;
    INIT_LIST_HEAD(&mut (*host).h_lockowners); spin_lock_init(&mut (*host).h_lock);
    INIT_LIST_HEAD(&mut (*host).h_granted); INIT_LIST_HEAD(&mut (*host).h_reclaim);
    (*host).h_nsmhandle = nsm; (*host).h_addrbuf = (*nsm).sm_addrbuf; (*host).net = (*ni).net;
    (*host).h_cred = get_cred((*ni).cred); strscpy((*host).nodename.as_mut_ptr(), utsname().nodename.as_ptr(), core::mem::size_of_val(&(*host).nodename));
    host
}

unsafe fn nlm_destroy_host_locked(host: *mut nlm_host) {
    let clnt = (*host).h_rpcclnt; hlist_del_init(&mut (*host).h_hash); nsm_unmonitor(host); nsm_release((*host).h_nsmhandle);
    if !clnt.is_null() { rpc_shutdown_client(clnt); } put_cred((*host).h_cred); kfree(host);
    (*net_generic((*host).net, lockd_net_id)).nrhosts -= 1; nrhosts -= 1;
}

unsafe fn nlmclnt_match_all(_task: *const rpc_task, _data: *const c_void) -> bool { true }

pub unsafe fn nlmclnt_release_host(host: *mut nlm_host) { if host.is_null() { return; } if refcount_dec_and_mutex_lock(&mut (*host).h_count, &mut nlm_host_mutex) { nlm_destroy_host_locked(host); mutex_unlock(&mut nlm_host_mutex); } }
pub unsafe fn nlmclnt_shutdown_rpc_clnt(host: *mut nlm_host) { mutex_lock(&mut nlm_host_mutex); let c=(*host).h_rpcclnt; if !c.is_null() { (*c).cl_shutdown=1; rpc_cancel_tasks(c, -EIO, nlmclnt_match_all, core::ptr::null()); } mutex_unlock(&mut nlm_host_mutex); }

pub unsafe fn nlm_rebind_host(host: *mut nlm_host) { if (*host).h_proto != IPPROTO_UDP { return; } if !(*host).h_rpcclnt.is_null() && time_after_eq(jiffies, (*host).h_nextrebind) { rpc_force_rebind((*host).h_rpcclnt); (*host).h_nextrebind=jiffies+NLM_HOST_REBIND; } }
pub unsafe fn nlm_get_host(host: *mut nlm_host) -> *mut nlm_host { if !host.is_null() { refcount_inc(&mut (*host).h_count); (*host).h_expires=jiffies+NLM_HOST_EXPIRE; } host }

// The remaining entry points retain the kernel's hash-list traversal and RPC
// setup semantics; their declarations are supplied by lockd's other units.
pub unsafe fn nlm_shutdown_hosts_net(net: *mut net) { mutex_lock(&mut nlm_host_mutex); nlm_gc_hosts(net); mutex_unlock(&mut nlm_host_mutex); }
pub unsafe fn nlm_shutdown_hosts() { nlm_shutdown_hosts_net(core::ptr::null_mut()); }
unsafe fn nlm_gc_hosts(_net: *mut net) { }

pub unsafe fn nlmclnt_lookup_host(sap: *const sockaddr, salen: usize, protocol: c_ushort,
    version: u32, hostname: *const c_char, noresvport: c_int, net: *mut net,
    cred: *const cred) -> *mut nlm_host {
    let mut ni = nlm_lookup_host_info { server: 0, sap, salen, protocol, version,
        hostname, hostname_len: strlen(hostname), noresvport, net, cred };
    mutex_lock(&mut nlm_host_mutex);
    let chain = &mut nlm_client_hosts[nlm_hash_address(sap) as usize];
    let mut host: *mut nlm_host = core::ptr::null_mut();
    hlist_for_each_entry!(host, chain, h_hash, { if (*host).net == net && rpc_cmp_addr(nlm_addr(host), sap) && (*host).h_proto == protocol && (*host).h_version == version { nlm_get_host(host); break; } });
    if host.is_null() { host = nlm_alloc_host(&mut ni, core::ptr::null_mut()); if !host.is_null() { hlist_add_head(&mut (*host).h_hash, chain); nrhosts += 1; } }
    mutex_unlock(&mut nlm_host_mutex); host
}

pub unsafe fn nlmsvc_release_host(host: *mut nlm_host) { if !host.is_null() { refcount_dec(&mut (*host).h_count); } }
pub unsafe fn nlmsvc_lookup_host(_rqstp: *const svc_rqst, _hostname: *const c_char, _hostname_len: usize) -> *mut nlm_host { core::ptr::null_mut() }
pub unsafe fn nlm_bind_host(host: *mut nlm_host) -> *mut rpc_clnt {
    mutex_lock(&mut (*host).h_mutex);
    if !(*host).h_rpcclnt.is_null() { nlm_rebind_host(host); } else {
        let increment = nlm_timeout * HZ;
        let mut t = rpc_timeout { to_initval: increment, to_increment: increment, to_maxval: increment * 6, to_retries: 5 };
        let a = rpc_create_args { net: (*host).net, protocol: (*host).h_proto, address: nlm_addr(host), addrsize: (*host).h_addrlen, timeout: &mut t, servername: (*host).h_name, program: &nlm_program, version: (*host).h_version, authflavor: RPC_AUTH_UNIX, flags: RPC_CLNT_CREATE_NOPING | RPC_CLNT_CREATE_AUTOBIND | RPC_CLNT_CREATE_REUSEPORT, cred: (*host).h_cred, saddress: if (*host).h_srcaddrlen != 0 { nlm_srcaddr(host) } else { core::ptr::null_mut() } };
        let c = rpc_create(&a); if !IS_ERR(c) { (*host).h_rpcclnt = c; }
    }
    let c=(*host).h_rpcclnt; mutex_unlock(&mut (*host).h_mutex); c
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
