// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * udp_diag.c	Module for monitoring UDP transport protocols sockets.
 *
 * Authors:	Pavel Emelyanov, <xemul@parallels.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn sk_diag_dump(
    sk: *mut sock,
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
    net_admin: bool,
) -> c_int {
    if !inet_diag_bc_sk((*cb).data, sk) {
        return 0;
    }
    inet_sk_diag_fill(sk, core::ptr::null_mut(), skb, cb, req, NLM_F_MULTI, net_admin)
}

unsafe fn udp_diag_dump_one(
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
) -> c_int {
    let in_skb = (*cb).skb;
    let mut sk: *mut sock = core::ptr::null_mut();
    let rep: *mut sk_buff;
    let net: *mut net;
    let mut err: c_int;

    net = sock_net((*in_skb).sk);

    rcu_read_lock();
    if (*req).sdiag_family == AF_INET {
        // src and dst are swapped for historical reasons
        sk = __udp4_lib_lookup(
            net, (*req).id.idiag_src[0], (*req).id.idiag_sport,
            (*req).id.idiag_dst[0], (*req).id.idiag_dport,
            (*req).id.idiag_if, 0, core::ptr::null_mut(),
        );
    }
    // CONFIG_IPV6 conditional preserved from the C source.
    else if (*req).sdiag_family == AF_INET6 {
        sk = __udp6_lib_lookup(
            net, (*req).id.idiag_src as *mut in6_addr, (*req).id.idiag_sport,
            (*req).id.idiag_dst as *mut in6_addr, (*req).id.idiag_dport,
            (*req).id.idiag_if, 0, core::ptr::null_mut(),
        );
    }
    if !sk.is_null() && !refcount_inc_not_zero(&mut (*sk).sk_refcnt) {
        sk = core::ptr::null_mut();
    }
    rcu_read_unlock();
    err = -ENOENT;
    if sk.is_null() {
        return err;
    }

    err = sock_diag_check_cookie(sk, (*req).id.idiag_cookie);
    if err != 0 {
        sock_put(sk);
        return err;
    }

    err = -ENOMEM;
    rep = nlmsg_new(
        nla_total_size(core::mem::size_of::<inet_diag_msg>())
            + inet_diag_msg_attrs_size()
            + nla_total_size(core::mem::size_of::<inet_diag_meminfo>()) + 64,
        GFP_KERNEL,
    );
    if rep.is_null() {
        sock_put(sk);
        return err;
    }

    err = inet_sk_diag_fill(
        sk, core::ptr::null_mut(), rep, cb, req, 0,
        netlink_net_capable(in_skb, CAP_NET_ADMIN),
    );
    if err < 0 {
        WARN_ON(err == -EMSGSIZE);
        kfree_skb(rep);
        sock_put(sk);
        return err;
    }
    err = nlmsg_unicast((*net).diag_nlsk, rep, NETLINK_CB(in_skb).portid);
    sock_put(sk);
    err
}

unsafe fn udp_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) {
    let net_admin = netlink_net_capable((*cb).skb, CAP_NET_ADMIN);
    let net = sock_net((*skb).sk);
    let mut num: c_int;
    let mut s_num: c_int;
    let mut slot: c_int;
    let s_slot: c_int;
    let table: *mut udp_table;

    table = (*net).ipv4.udp_table;
    s_slot = (*cb).args[0];
    s_num = (*cb).args[1];
    slot = s_slot;

    while slot <= (*table).mask {
        let hslot = &mut (*table).hash[slot as usize];
        let mut sk: *mut sock;
        num = 0;
        if hlist_empty(&hslot.head) {
            s_num = 0;
            slot += 1;
            continue;
        }
        spin_lock_bh(&mut hslot.lock);
        sk_for_each!(sk, &hslot.head, {
            let inet = inet_sk(sk);
            if !net_eq(sock_net(sk), net) { continue; }
            if num < s_num { num += 1; continue; }
            if ((*r).idiag_states & (1u32 << (*sk).sk_state)) == 0 { num += 1; continue; }
            if (*r).sdiag_family != AF_UNSPEC && (*sk).sk_family != (*r).sdiag_family { num += 1; continue; }
            if (*r).id.idiag_sport != (*inet).inet_sport && (*r).id.idiag_sport != 0 { num += 1; continue; }
            if (*r).id.idiag_dport != (*inet).inet_dport && (*r).id.idiag_dport != 0 { num += 1; continue; }
            if sk_diag_dump(sk, skb, cb, r, net_admin) < 0 { spin_unlock_bh(&mut hslot.lock); return; }
            num += 1;
        });
        spin_unlock_bh(&mut hslot.lock);
        s_num = 0;
        slot += 1;
    }
    (*cb).args[0] = slot;
    (*cb).args[1] = num;
}

unsafe fn udp_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, _info: *mut c_void) {
    (*r).idiag_rqueue = udp_rqueue_get(sk);
    (*r).idiag_wqueue = sk_wmem_alloc_get(sk);
}

// CONFIG_INET_DIAG_DESTROY conditional preserved from the C source.
unsafe fn udp_diag_destroy(in_skb: *mut sk_buff, req: *const inet_diag_req_v2) -> c_int {
    let net = sock_net((*in_skb).sk);
    let mut sk: *mut sock;
    rcu_read_lock();
    if (*req).sdiag_family == AF_INET {
        sk = __udp4_lib_lookup(net, (*req).id.idiag_dst[0], (*req).id.idiag_dport,
            (*req).id.idiag_src[0], (*req).id.idiag_sport, (*req).id.idiag_if, 0, core::ptr::null_mut());
    } else if (*req).sdiag_family == AF_INET6 {
        if ipv6_addr_v4mapped((*req).id.idiag_dst as *mut in6_addr) && ipv6_addr_v4mapped((*req).id.idiag_src as *mut in6_addr) {
            sk = __udp4_lib_lookup(net, (*req).id.idiag_dst[3], (*req).id.idiag_dport, (*req).id.idiag_src[3], (*req).id.idiag_sport, (*req).id.idiag_if, 0, core::ptr::null_mut());
        } else {
            sk = __udp6_lib_lookup(net, (*req).id.idiag_dst as *mut in6_addr, (*req).id.idiag_dport, (*req).id.idiag_src as *mut in6_addr, (*req).id.idiag_sport, (*req).id.idiag_if, 0, core::ptr::null_mut());
        }
    } else { rcu_read_unlock(); return -EINVAL; }
    if !sk.is_null() && !refcount_inc_not_zero(&mut (*sk).sk_refcnt) { sk = core::ptr::null_mut(); }
    rcu_read_unlock();
    if sk.is_null() { return -ENOENT; }
    if sock_diag_check_cookie(sk, (*req).id.idiag_cookie) != 0 { sock_put(sk); return -ENOENT; }
    let err = sock_diag_destroy(sk, ECONNABORTED);
    sock_put(sk);
    err
}

// The C registration object and module init/exit declarations depend on kernel bindings.
static udp_diag_handler: inet_diag_handler = inet_diag_handler {
    owner: THIS_MODULE,
    dump: Some(udp_diag_dump),
    dump_one: Some(udp_diag_dump_one),
    idiag_get_info: Some(udp_diag_get_info),
    idiag_type: IPPROTO_UDP,
    idiag_info_size: 0,
    destroy: Some(udp_diag_destroy),
};

unsafe fn udp_diag_init() -> c_int { inet_diag_register(&udp_diag_handler) }
unsafe fn udp_diag_exit() { inet_diag_unregister(&udp_diag_handler); }

// module_init(udp_diag_init); module_exit(udp_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("UDP socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 2-17 /* AF_INET - IPPROTO_UDP */);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
