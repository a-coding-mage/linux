// SPDX-License-Identifier: GPL-2.0-or-later
/* Local endpoint object management
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

use core::mem;

// Kernel/networking declarations are supplied by the surrounding translation.

/*
 * Handle an ICMP/ICMP6 error turning up at the tunnel.  Push it through the
 * usual mechanism so that it gets parsed and presented through the UDP
 * socket's error_report().
 */
unsafe fn rxrpc_encap_err_rcv(
    sk: *mut sock,
    skb: *mut sk_buff,
    err: i32,
    port: __be16,
    info: u32,
    payload: *mut u8,
) {
    if (*ip_hdr(skb)).version == IPVERSION {
        return ip_icmp_error(sk, skb, err, port, info, payload);
    }
    if IS_ENABLED(CONFIG_AF_RXRPC_IPV6) {
        return ipv6_icmp_error(sk, skb, err, port, info, payload);
    }
}

/* Set or clear the Don't Fragment flag on a socket. */
pub unsafe fn rxrpc_local_dont_fragment(local: *const rxrpc_local, set: bool) {
    if set {
        ip_sock_set_mtu_discover((*(*local).socket).sk, IP_PMTUDISC_DO);
    } else {
        ip_sock_set_mtu_discover((*(*local).socket).sk, IP_PMTUDISC_DONT);
    }
}

/*
 * Compare a local to an address.  Return -ve, 0 or +ve to indicate less than,
 * same or greater than.
 */
unsafe fn rxrpc_local_cmp_key(
    local: *const rxrpc_local,
    srx: *const sockaddr_rxrpc,
) -> isize {
    let diff = ((*local).srx.transport_type - (*srx).transport_type)
        .then((*local).srx.transport_len - (*srx).transport_len)
        .then((*local).srx.transport.family - (*srx).transport.family);
    if diff != 0 { return diff as isize; }

    match (*srx).transport.family {
        AF_INET => {
            ((*local).srx.transport.sin.sin_port as u16)
                .wrapping_sub((*srx).transport.sin.sin_port as u16)
                .then(unsafe { memcmp(
                    &(*local).srx.transport.sin.sin_addr as *const _ as *const _,
                    &(*srx).transport.sin.sin_addr as *const _ as *const _,
                    mem::size_of::<in_addr>(),
                ) }) as isize
        }
        #[cfg(CONFIG_AF_RXRPC_IPV6)]
        AF_INET6 => {
            ((*local).srx.transport.sin6.sin6_port as u16)
                .wrapping_sub((*srx).transport.sin6.sin6_port as u16)
                .then(unsafe { memcmp(
                    &(*local).srx.transport.sin6.sin6_addr as *const _ as *const _,
                    &(*srx).transport.sin6.sin6_addr as *const _ as *const _,
                    mem::size_of::<in6_addr>(),
                ) }) as isize
        }
        _ => { BUG(); 0 }
    }
}

unsafe fn rxrpc_client_conn_reap_timeout(timer: *mut timer_list) {
    let local = container_of!(timer, rxrpc_local, client_conn_reap_timer);
    if !(*local).kill_all_client_conns
        && test_and_set_bit(RXRPC_CLIENT_CONN_REAP_TIMER, &mut (*local).client_conn_flags)
    {
        rxrpc_wake_up_io_thread(local);
    }
}

/* Allocate a new local endpoint. */
unsafe fn rxrpc_alloc_local(net: *mut net, srx: *const sockaddr_rxrpc) -> *mut rxrpc_local {
    let local = kzalloc_obj::<rxrpc_local>();
    if !local.is_null() {
        refcount_set(&mut (*local).ref_, 1);
        atomic_set(&mut (*local).active_users, 1);
        (*local).net = net;
        (*local).rxnet = rxrpc_net(net);
        INIT_HLIST_NODE(&mut (*local).link);
        init_completion(&mut (*local).io_thread_ready);
        #[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)]
        skb_queue_head_init(&mut (*local).rx_delay_queue);
        skb_queue_head_init(&mut (*local).rx_queue);
        INIT_LIST_HEAD(&mut (*local).conn_attend_q);
        INIT_LIST_HEAD(&mut (*local).call_attend_q);
        (*local).client_bundles = RB_ROOT;
        spin_lock_init(&mut (*local).client_bundles_lock);
        (*local).kill_all_client_conns = false;
        INIT_LIST_HEAD(&mut (*local).idle_client_conns);
        timer_setup(&mut (*local).client_conn_reap_timer, rxrpc_client_conn_reap_timeout, 0);
        spin_lock_init(&mut (*local).lock);
        rwlock_init(&mut (*local).services_lock);
        (*local).debug_id = atomic_inc_return(&mut rxrpc_debug_id);
        memcpy(&mut (*local).srx as *mut _, srx as *const _, mem::size_of::<sockaddr_rxrpc>());
        (*local).srx.srx_service = 0;
        idr_init(&mut (*local).conn_ids);
        let mut tmp: u32 = 0;
        get_random_bytes(&mut tmp as *mut _ as *mut _, mem::size_of::<u32>());
        tmp &= 0x3fffffff;
        if tmp == 0 { tmp = 1; }
        idr_set_cursor(&mut (*local).conn_ids, tmp);
        INIT_LIST_HEAD(&mut (*local).new_client_calls);
        spin_lock_init(&mut (*local).client_call_lock);
        trace_rxrpc_local((*local).debug_id, rxrpc_local_new, 1, 1);
    }
    _leave!(" = %p", local);
    local
}

/* create the local socket - must be called with rxrpc_local_mutex locked */
unsafe fn rxrpc_open_socket(local: *mut rxrpc_local, net: *mut net) -> i32 {
    let mut tuncfg = udp_tunnel_sock_cfg { encap_type: 0, encap_rcv: None, encap_err_rcv: None, sk_user_data: core::ptr::null_mut() };
    let srx = &mut (*local).srx;
    let mut udp_conf: udp_port_cfg = mem::zeroed();
    let usk: *mut sock;
    let mut ret: i32;
    _enter!("%p{%d,%d}", local, srx.transport_type, srx.transport.family);
    udp_conf.family = srx.transport.family;
    udp_conf.use_udp_checksums = true;
    if udp_conf.family == AF_INET {
        udp_conf.local_ip = srx.transport.sin.sin_addr;
        udp_conf.local_udp_port = srx.transport.sin.sin_port;
    } else {
        udp_conf.local_ip6 = srx.transport.sin6.sin6_addr;
        udp_conf.local_udp_port = srx.transport.sin6.sin6_port;
        udp_conf.use_udp6_tx_checksums = true;
        udp_conf.use_udp6_rx_checksums = true;
    }
    ret = udp_sock_create(net, &mut udp_conf, &mut (*local).socket);
    if ret < 0 { _leave!(" = %d [socket]", ret); return ret; }
    tuncfg.encap_type = UDP_ENCAP_RXRPC;
    tuncfg.encap_rcv = Some(rxrpc_encap_rcv);
    tuncfg.encap_err_rcv = Some(rxrpc_encap_err_rcv);
    tuncfg.sk_user_data = local as *mut _;
    setup_udp_tunnel_sock(net, (*local).socket.sk, &mut tuncfg);
    usk = (*local).socket.sk;
    (*usk).sk_error_report = Some(rxrpc_error_report);
    match srx.transport.family {
        AF_INET6 => { ip6_sock_set_recverr(usk); ip_sock_set_recverr(usk); rxrpc_local_dont_fragment(local, true); }
        AF_INET => { ip_sock_set_recverr(usk); rxrpc_local_dont_fragment(local, true); }
        _ => BUG(),
    }
    let io_thread = kthread_run(rxrpc_io_thread, local, "krxrpcio/%u", ntohs(udp_conf.local_udp_port));
    if IS_ERR(io_thread) { ret = PTR_ERR(io_thread); goto!(error_sock); }
    wait_for_completion(&mut (*local).io_thread_ready);
    WRITE_ONCE!((*local).io_thread, io_thread);
    _leave!(" = 0");
    return 0;
error_sock:
    kernel_sock_shutdown((*local).socket, SHUT_RDWR);
    (*local).socket.sk.sk_user_data = core::ptr::null_mut();
    sock_release((*local).socket);
    (*local).socket = core::ptr::null_mut();
    ret
}

/* Look up or create a new local endpoint using the specified local address. */
pub unsafe fn rxrpc_lookup_local(net: *mut net, srx: *const sockaddr_rxrpc) -> *mut rxrpc_local {
    let rxnet = rxrpc_net(net);
    let mut local: *mut rxrpc_local = core::ptr::null_mut();
    let mut cursor: *mut hlist_node = core::ptr::null_mut();
    let mut diff: isize;
    mutex_lock(&mut (*rxnet).local_mutex);
    hlist_for_each!(cursor, &mut (*rxnet).local_endpoints, {
        local = hlist_entry!(cursor, rxrpc_local, link);
        diff = rxrpc_local_cmp_key(local, srx);
        if diff != 0 { continue; }
        if (*srx).srx_service != 0 { local = core::ptr::null_mut(); goto!(addr_in_use); }
        if !rxrpc_use_local(local, rxrpc_local_use_lookup).is_null() { break; }
        goto!(found);
    });
    local = rxrpc_alloc_local(net, srx);
    if local.is_null() { goto!(nomem); }
    let ret = rxrpc_open_socket(local, net);
    if ret < 0 { goto!(sock_error); }
    if !cursor.is_null() { hlist_replace_rcu(cursor, &mut (*local).link); (*cursor).pprev = core::ptr::null_mut(); }
    else { hlist_add_head_rcu(&mut (*local).link, &mut (*rxnet).local_endpoints); }
found:
    mutex_unlock(&mut (*rxnet).local_mutex); _leave!(" = %p", local); return local;
nomem:
    let ret = -ENOMEM;
sock_error:
    mutex_unlock(&mut (*rxnet).local_mutex);
    if !local.is_null() { call_rcu(&mut (*local).rcu, rxrpc_local_rcu); }
    _leave!(" = %d", ret); return ERR_PTR(ret);
addr_in_use:
    mutex_unlock(&mut (*rxnet).local_mutex); _leave!(" = -EADDRINUSE"); ERR_PTR(-EADDRINUSE)
}

/* Get a ref on a local endpoint. */
pub unsafe fn rxrpc_get_local(local: *mut rxrpc_local, why: rxrpc_local_trace) -> *mut rxrpc_local {
    let u = atomic_read(&(*local).active_users); let mut r = 0;
    __refcount_inc(&mut (*local).ref_, &mut r); trace_rxrpc_local((*local).debug_id, why, r + 1, u); local
}

/* Get a ref on a local endpoint unless its usage has already reached 0. */
pub unsafe fn rxrpc_get_local_maybe(local: *mut rxrpc_local, why: rxrpc_local_trace) -> *mut rxrpc_local {
    if !local.is_null() { let mut r = 0; if __refcount_inc_not_zero(&mut (*local).ref_, &mut r) { let u = atomic_read(&(*local).active_users); trace_rxrpc_local((*local).debug_id, why, r + 1, u); return local; } } core::ptr::null_mut()
}

/* Drop a ref on a local endpoint. */
pub unsafe fn rxrpc_put_local(local: *mut rxrpc_local, why: rxrpc_local_trace) {
    if !local.is_null() { let id = (*local).debug_id; let u = atomic_read(&(*local).active_users); let mut r = 0; let dead = __refcount_dec_and_test(&mut (*local).ref_, &mut r); trace_rxrpc_local(id, why, r, u); if dead { call_rcu(&mut (*local).rcu, rxrpc_local_rcu); } }
}

/* Start using a local endpoint. */
pub unsafe fn rxrpc_use_local(mut local: *mut rxrpc_local, why: rxrpc_local_trace) -> *mut rxrpc_local {
    local = rxrpc_get_local_maybe(local, rxrpc_local_get_for_use); if local.is_null() { return core::ptr::null_mut(); }
    if !__rxrpc_use_local(local, why) { rxrpc_put_local(local, rxrpc_local_put_for_use); return core::ptr::null_mut(); } local
}

/* Cease using a local endpoint. */
pub unsafe fn rxrpc_unuse_local(local: *mut rxrpc_local, why: rxrpc_local_trace) {
    if !local.is_null() { let id = (*local).debug_id; let r = refcount_read(&(*local).ref_); let u = atomic_dec_return(&mut (*local).active_users); trace_rxrpc_local(id, why, r, u); if u == 0 { kthread_stop((*local).io_thread); } }
}

/* Destroy a local endpoint's socket and then hand the record to RCU to dispose of. */
pub unsafe fn rxrpc_destroy_local(local: *mut rxrpc_local) {
    let socket = (*local).socket; let rxnet = (*local).rxnet; _enter!("%d", (*local).debug_id); (*local).dead = true;
    mutex_lock(&mut (*rxnet).local_mutex); hlist_del_init_rcu(&mut (*local).link); mutex_unlock(&mut (*rxnet).local_mutex);
    rxrpc_clean_up_local_conns(local); rxrpc_service_connection_reaper(&mut (*rxnet).service_conn_reaper); ASSERT!((*local).service.is_null());
    if !socket.is_null() { (*local).socket = core::ptr::null_mut(); kernel_sock_shutdown(socket, SHUT_RDWR); (*socket).sk.sk_user_data = core::ptr::null_mut(); sock_release(socket); }
    #[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)] rxrpc_purge_queue(&mut (*local).rx_delay_queue);
    rxrpc_purge_queue(&mut (*local).rx_queue); rxrpc_purge_client_connections(local); page_frag_cache_drain(&mut (*local).tx_alloc);
}

/* Destroy a local endpoint after the RCU grace period expires. */
unsafe fn rxrpc_local_rcu(rcu: *mut rcu_head) {
    let local = container_of!(rcu, rxrpc_local, rcu); rxrpc_see_local(local, rxrpc_local_free); kfree(local);
}

/* Verify the local endpoint list is empty by this point. */
pub unsafe fn rxrpc_destroy_all_locals(rxnet: *mut rxrpc_net) {
    _enter!(""); flush_workqueue(rxrpc_workqueue);
    if !hlist_empty(&(*rxnet).local_endpoints) { mutex_lock(&mut (*rxnet).local_mutex); hlist_for_each_entry!(local, &(*rxnet).local_endpoints, link, { pr_err!("AF_RXRPC: Leaked local %p {%d}\n", local, refcount_read(&(*local).ref_)); }); mutex_unlock(&mut (*rxnet).local_mutex); BUG(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
