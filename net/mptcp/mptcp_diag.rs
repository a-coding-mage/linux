// SPDX-License-Identifier: GPL-2.0
/* MPTCP socket monitoring support
 *
 * Copyright (c) 2020 Red Hat
 *
 * Author: Paolo Abeni <pabeni@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn sk_diag_dump(
    sk: *mut sock,
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
    net_admin: bool,
) -> c_int {
    if inet_diag_bc_sk((*cb).data, sk) == 0 {
        return 0;
    }

    inet_sk_diag_fill(sk, inet_csk(sk), skb, cb, req, NLM_F_MULTI, net_admin)
}

unsafe fn mptcp_diag_dump_one(
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
) -> c_int {
    let in_skb: *mut sk_buff = (*cb).skb;
    let mut msk: *mut mptcp_sock = core::ptr::null_mut();
    let mut rep: *mut sk_buff;
    let mut err: c_int = -ENOENT;
    let net: *mut net;
    let sk: *mut sock;

    net = sock_net((*in_skb).sk);
    msk = mptcp_token_get_sock(net, (*req).id.idiag_cookie[0]);
    if msk.is_null() {
        return err;
    }

    err = -ENOMEM;
    sk = msk as *mut sock;
    rep = nlmsg_new(
        nla_total_size(core::mem::size_of::<inet_diag_msg>())
            + inet_diag_msg_attrs_size()
            + nla_total_size(core::mem::size_of::<mptcp_info>())
            + nla_total_size(core::mem::size_of::<inet_diag_meminfo>())
            + 64,
        GFP_KERNEL,
    );
    if rep.is_null() {
        sock_put(sk);
        return err;
    }

    err = inet_sk_diag_fill(
        sk,
        inet_csk(sk),
        rep,
        cb,
        req,
        0,
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

#[repr(C)]
struct mptcp_diag_ctx {
    s_slot: c_long,
    s_num: c_long,
    l_slot: c_uint,
    l_num: c_uint,
}

unsafe fn mptcp_diag_dump_listeners(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
    net_admin: bool,
) {
    let diag_ctx: *mut mptcp_diag_ctx = (*cb).ctx as *mut mptcp_diag_ctx;
    let net: *mut net = sock_net((*skb).sk);
    let hinfo: *mut inet_hashinfo;
    let mut i: c_int;

    hinfo = (*(*net).ipv4.tcp_death_row).hashinfo;

    i = (*diag_ctx).l_slot as c_int;
    while i <= (*hinfo).lhash2_mask as c_int {
        let ilb: *mut inet_listen_hashbucket = &mut (*hinfo).lhash2[i as usize];
        let mut num: c_int = 0;

        rcu_read_lock();
        spin_lock(&mut (*ilb).lock);
        let mut node = (*ilb).nulls_head;
        while let Some(sk) = sk_nulls_for_each(&mut node) {
            let ctx: *const mptcp_subflow_context = mptcp_subflow_ctx(sk);
            let inet: *mut inet_sock = inet_sk(sk);
            let mut ret: c_int;

            if num < (*diag_ctx).l_num as c_int {
                num += 1;
                continue;
            }
            if ctx.is_null() || strcmp((*(*inet_csk(sk)).icsk_ulp_ops).name, b"mptcp\0".as_ptr()) != 0 {
                num += 1;
                continue;
            }
            let target = (*ctx).conn;
            if target.is_null() || !net_eq(sock_net(target), net) {
                num += 1;
                continue;
            }
            if (*r).sdiag_family != AF_UNSPEC && (*target).sk_family != (*r).sdiag_family {
                num += 1;
                continue;
            }
            if (*r).id.idiag_sport != (*inet).inet_sport && (*r).id.idiag_sport != 0 {
                num += 1;
                continue;
            }
            if !refcount_inc_not_zero(&mut (*target).sk_refcnt) {
                num += 1;
                continue;
            }

            ret = sk_diag_dump(target, skb, cb, r, net_admin);
            sock_put(target);
            if ret < 0 {
                spin_unlock(&mut (*ilb).lock);
                rcu_read_unlock();
                (*diag_ctx).l_slot = i as c_uint;
                (*diag_ctx).l_num = num as c_uint;
                return;
            }
            (*diag_ctx).l_num = (num + 1) as c_uint;
            num = 0;
        }
        spin_unlock(&mut (*ilb).lock);
        rcu_read_unlock();
        cond_resched();
        (*diag_ctx).l_num = 0;
        i += 1;
    }
    (*diag_ctx).l_num = 0;
    (*diag_ctx).l_slot = i as c_uint;
}

unsafe fn mptcp_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) {
    let net_admin = netlink_net_capable((*cb).skb, CAP_NET_ADMIN);
    let diag_ctx: *mut mptcp_diag_ctx = (*cb).ctx as *mut mptcp_diag_ctx;
    let net = sock_net((*skb).sk);

    BUILD_BUG_ON(core::mem::size_of::<decltype!((*cb).ctx)>() < core::mem::size_of::<mptcp_diag_ctx>());

    loop {
        let msk = mptcp_token_iter_next(net, &mut (*diag_ctx).s_slot, &mut (*diag_ctx).s_num);
        if msk.is_null() { break; }
        let inet = msk as *mut inet_sock;
        let sk = msk as *mut sock;
        let mut ret: c_int = 0;

        if (*r).idiag_states & (1 << (*sk).sk_state) == 0 { sock_put(sk); continue; }
        if (*r).sdiag_family != AF_UNSPEC && (*sk).sk_family != (*r).sdiag_family { sock_put(sk); continue; }
        if (*r).id.idiag_sport != (*inet).inet_sport && (*r).id.idiag_sport != 0 { sock_put(sk); continue; }
        if (*r).id.idiag_dport != (*inet).inet_dport && (*r).id.idiag_dport != 0 { sock_put(sk); continue; }

        ret = sk_diag_dump(sk, skb, cb, r, net_admin);
        sock_put(sk);
        if ret < 0 {
            (*diag_ctx).s_num -= 1;
            break;
        }
        cond_resched();
    }

    if (*r).idiag_states & TCPF_LISTEN != 0 && (*r).id.idiag_dport == 0 {
        mptcp_diag_dump_listeners(skb, cb, r, net_admin);
    }
}

unsafe fn mptcp_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, info_ptr: *mut c_void) {
    let msk = mptcp_sk(sk);
    let info = info_ptr as *mut mptcp_info;

    (*r).idiag_rqueue = sk_rmem_alloc_get(sk) + READ_ONCE((*mptcp_sk(sk)).backlog_len);
    (*r).idiag_wqueue = sk_wmem_alloc_get(sk);

    if inet_sk_state_load(sk) == TCP_LISTEN {
        let lsk = READ_ONCE((*msk).first);
        if !lsk.is_null() {
            (*r).idiag_rqueue = READ_ONCE((*lsk).sk_ack_backlog);
            (*r).idiag_wqueue = READ_ONCE((*lsk).sk_max_ack_backlog);
        }
    }
    if info.is_null() { return; }
    mptcp_diag_fill_info(msk, info);
}

static mptcp_diag_handler: inet_diag_handler = inet_diag_handler {
    owner: THIS_MODULE,
    dump: mptcp_diag_dump,
    dump_one: mptcp_diag_dump_one,
    idiag_get_info: mptcp_diag_get_info,
    idiag_type: IPPROTO_MPTCP,
    idiag_info_size: core::mem::size_of::<mptcp_info>(),
};

unsafe fn mptcp_diag_init() -> c_int {
    inet_diag_register(&mptcp_diag_handler)
}

unsafe fn mptcp_diag_exit() {
    inet_diag_unregister(&mptcp_diag_handler);
}

module_init!(mptcp_diag_init);
module_exit!(mptcp_diag_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
