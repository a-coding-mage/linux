// SPDX-License-Identifier: GPL-2.0-only
// Translated from raw_diag.c. Kernel declarations and symbols are supplied by
// the surrounding repository.

unsafe fn raw_get_hashinfo(r: *const inet_diag_req_v2) -> *mut raw_hashinfo {
    if (*r).sdiag_family == AF_INET {
        &raw_v4_hashinfo as *const _ as *mut _
    } else if (*r).sdiag_family == AF_INET6 {
        &raw_v6_hashinfo as *const _ as *mut _
    } else {
        ERR_PTR(-EINVAL)
    }
}

/*
 * Due to requirement of not breaking user API we can't simply
 * rename @pad field in inet_diag_req_v2 structure, instead
 * use helper to figure it out.
 */
unsafe fn raw_lookup(
    net: *mut net,
    sk: *const sock,
    req: *const inet_diag_req_v2,
) -> bool {
    let r = req as *const inet_diag_req_raw;

    if (*r).sdiag_family == AF_INET {
        raw_v4_match(
            net,
            sk,
            (*r).sdiag_raw_protocol,
            (*r).id.idiag_dst[0],
            (*r).id.idiag_src[0],
            (*r).id.idiag_if,
            0,
        )
    } else {
        raw_v6_match(
            net,
            sk,
            (*r).sdiag_raw_protocol,
            &(*r).id.idiag_src as *const _ as *const in6_addr,
            &(*r).id.idiag_dst as *const _ as *const in6_addr,
            (*r).id.idiag_if,
            0,
        )
    }
}

unsafe fn raw_sock_get(
    net: *mut net,
    r: *const inet_diag_req_v2,
) -> *mut sock {
    let hashinfo = raw_get_hashinfo(r);
    let mut sk: *mut sock;
    let mut slot: i32;

    if IS_ERR(hashinfo) {
        return ERR_CAST(hashinfo);
    }

    rcu_read_lock();
    slot = 0;
    while slot < RAW_HTABLE_SIZE {
        let hlist = &mut (*hashinfo).ht[slot as usize] as *mut _;
        sk_for_each_rcu!(sk, hlist, {
            if raw_lookup(net, sk, r) {
                /*
                 * Grab it and keep until we fill
                 * diag message to be reported, so
                 * caller should call sock_put then.
                 */
                if refcount_inc_not_zero(&mut (*sk).sk_refcnt) {
                    rcu_read_unlock();
                    return sk;
                }
            }
        });
        slot += 1;
    }
    sk = ERR_PTR(-ENOENT);
    rcu_read_unlock();
    sk
}

unsafe fn raw_diag_dump_one(
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) -> i32 {
    let in_skb = (*cb).skb;
    let net = sock_net((*in_skb).sk);
    let sk = raw_sock_get(net, r);
    if IS_ERR(sk) {
        return PTR_ERR(sk);
    }

    let rep = nlmsg_new(
        nla_total_size(core::mem::size_of::<inet_diag_msg>())
            + inet_diag_msg_attrs_size()
            + nla_total_size(core::mem::size_of::<inet_diag_meminfo>())
            + 64,
        GFP_KERNEL,
    );
    if rep.is_null() {
        sock_put(sk);
        return -ENOMEM;
    }

    let err = inet_sk_diag_fill(
        sk,
        core::ptr::null_mut(),
        rep,
        cb,
        r,
        0,
        netlink_net_capable(in_skb, CAP_NET_ADMIN),
    );
    sock_put(sk);
    if err < 0 {
        kfree_skb(rep);
        return err;
    }
    nlmsg_unicast((*net).diag_nlsk, rep, (*in_skb).cb.portid)
}

unsafe fn sk_diag_dump(
    sk: *mut sock,
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
    net_admin: bool,
) -> i32 {
    if !inet_diag_bc_sk((*cb).data, sk) {
        return 0;
    }
    inet_sk_diag_fill(sk, core::ptr::null_mut(), skb, cb, r, NLM_F_MULTI, net_admin)
}

unsafe fn raw_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) {
    let net_admin = netlink_net_capable((*cb).skb, CAP_NET_ADMIN);
    let hashinfo = raw_get_hashinfo(r);
    let net = sock_net((*skb).sk);
    let mut num: i32;
    let mut s_num: i32;
    let mut slot: i32;
    let s_slot = (*cb).args[0];
    let mut sk: *mut sock = core::ptr::null_mut();

    if IS_ERR(hashinfo) {
        return;
    }
    s_num = (*cb).args[1];
    num = s_num;
    rcu_read_lock();
    slot = s_slot;
    while slot < RAW_HTABLE_SIZE {
        s_num = 0;
        num = 0;
        let hlist = &mut (*hashinfo).ht[slot as usize] as *mut _;
        sk_for_each_rcu!(sk, hlist, {
            let inet = inet_sk(sk);
            if !net_eq(sock_net(sk), net) { continue; }
            if num < s_num { continue; }
            if (*sk).sk_family != (*r).sdiag_family { continue; }
            if (*r).id.idiag_sport != (*inet).inet_sport && (*r).id.idiag_sport != 0 { continue; }
            if (*r).id.idiag_dport != (*inet).inet_dport && (*r).id.idiag_dport != 0 { continue; }
            if sk_diag_dump(sk, skb, cb, r, net_admin) < 0 {
                rcu_read_unlock();
                (*cb).args[0] = slot as _;
                (*cb).args[1] = num as _;
                return;
            }
            num += 1;
        });
        slot += 1;
    }
    rcu_read_unlock();
    (*cb).args[0] = slot as _;
    (*cb).args[1] = num as _;
}

unsafe fn raw_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, _info: *mut core::ffi::c_void) {
    (*r).idiag_rqueue = sk_rmem_alloc_get(sk);
    (*r).idiag_wqueue = sk_wmem_alloc_get(sk);
}

#[cfg(CONFIG_INET_DIAG_DESTROY)]
unsafe fn raw_diag_destroy(in_skb: *mut sk_buff, r: *const inet_diag_req_v2) -> i32 {
    let net = sock_net((*in_skb).sk);
    let sk = raw_sock_get(net, r);
    if IS_ERR(sk) { return PTR_ERR(sk); }
    let err = sock_diag_destroy(sk, ECONNABORTED);
    sock_put(sk);
    err
}

static raw_diag_handler: inet_diag_handler = inet_diag_handler {
    owner: THIS_MODULE,
    dump: Some(raw_diag_dump),
    dump_one: Some(raw_diag_dump_one),
    idiag_get_info: Some(raw_diag_get_info),
    idiag_type: IPPROTO_RAW,
    idiag_info_size: 0,
    #[cfg(CONFIG_INET_DIAG_DESTROY)]
    destroy: Some(raw_diag_destroy),
};

unsafe fn raw_diag_init() -> i32 {
    inet_diag_register(&raw_diag_handler)
}

unsafe fn raw_diag_exit() {
    inet_diag_unregister(&raw_diag_handler);
}

// module_init(raw_diag_init);
// module_exit(raw_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("RAW socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 2-255 /* AF_INET - IPPROTO_RAW */);
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 10-255 /* AF_INET6 - IPPROTO_RAW */);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
