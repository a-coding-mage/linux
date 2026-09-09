// SPDX-License-Identifier: GPL-2.0-only
// Dependencies are supplied by the surrounding kernel/netlink implementation.

unsafe fn sk_diag_dump_groups(sk: *mut sock, nlskb: *mut sk_buff) -> c_int {
    let nlk: *mut netlink_sock = nlk_sk(sk);

    if (*nlk).groups.is_null() {
        return 0;
    }

    nla_put(
        nlskb,
        NETLINK_DIAG_GROUPS,
        NLGRPSZ((*nlk).ngroups),
        (*nlk).groups,
    )
}

unsafe fn sk_diag_put_flags(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let nlk: *mut netlink_sock = nlk_sk(sk);
    let mut flags: u32 = 0;

    if (*nlk).cb_running {
        flags |= NDIAG_FLAG_CB_RUNNING;
    }
    if nlk_test_bit(RECV_PKTINFO, sk) {
        flags |= NDIAG_FLAG_PKTINFO;
    }
    if nlk_test_bit(BROADCAST_SEND_ERROR, sk) {
        flags |= NDIAG_FLAG_BROADCAST_ERROR;
    }
    if nlk_test_bit(RECV_NO_ENOBUFS, sk) {
        flags |= NDIAG_FLAG_NO_ENOBUFS;
    }
    if nlk_test_bit(LISTEN_ALL_NSID, sk) {
        flags |= NDIAG_FLAG_LISTEN_ALL_NSID;
    }
    if nlk_test_bit(CAP_ACK, sk) {
        flags |= NDIAG_FLAG_CAP_ACK;
    }

    nla_put_u32(skb, NETLINK_DIAG_FLAGS, flags)
}

unsafe fn sk_diag_fill(
    sk: *mut sock,
    skb: *mut sk_buff,
    req: *mut netlink_diag_req,
    portid: u32,
    seq: u32,
    flags: u32,
    sk_ino: u64,
) -> c_int {
    let mut nlh: *mut nlmsghdr;
    let rep: *mut netlink_diag_msg;
    let nlk: *mut netlink_sock = nlk_sk(sk);

    nlh = nlmsg_put(skb, portid, seq, SOCK_DIAG_BY_FAMILY, size_of::<netlink_diag_msg>(), flags);
    if nlh.is_null() {
        return -EMSGSIZE;
    }

    rep = nlmsg_data(nlh);
    (*rep).ndiag_family = AF_NETLINK;
    (*rep).ndiag_type = (*sk).sk_type;
    (*rep).ndiag_protocol = (*sk).sk_protocol;
    (*rep).ndiag_state = (*sk).sk_state;

    (*rep).ndiag_ino = sk_ino;
    (*rep).ndiag_portid = (*nlk).portid;
    (*rep).ndiag_dst_portid = (*nlk).dst_portid;
    (*rep).ndiag_dst_group = (*nlk).dst_group;
    sock_diag_save_cookie(sk, (*rep).ndiag_cookie.as_mut_ptr());

    if ((*req).ndiag_show & NDIAG_SHOW_GROUPS) != 0 && sk_diag_dump_groups(sk, skb) != 0 {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }
    if ((*req).ndiag_show & NDIAG_SHOW_MEMINFO) != 0
        && sock_diag_put_meminfo(sk, skb, NETLINK_DIAG_MEMINFO) != 0
    {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }
    if ((*req).ndiag_show & NDIAG_SHOW_FLAGS) != 0 && sk_diag_put_flags(sk, skb) != 0 {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }

    nlmsg_end(skb, nlh);
    0
}

unsafe fn __netlink_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    protocol: c_int,
    mut s_num: c_int,
) -> c_int {
    let hti: *mut rhashtable_iter = (*cb).args[2] as *mut rhashtable_iter;
    let tbl: *mut netlink_table = &mut nl_table[protocol as usize];
    let net: *mut net = sock_net((*skb).sk);
    let req: *mut netlink_diag_req = nlmsg_data((*cb).nlh);
    let mut ret: c_int = 0;
    let mut num: c_int = 2;
    let mut sk: *mut sock;

    if s_num > 1 {
        // continue at mc_list
    } else {
    num -= 1;
    let mut hti = (*cb).args[2] as *mut rhashtable_iter;
    if hti.is_null() {
        hti = kmalloc_obj::<rhashtable_iter>();
        if hti.is_null() { return -ENOMEM; }
        (*cb).args[2] = hti as long;
    }
    if s_num == 0 { rhashtable_walk_enter(&(*tbl).hash, hti); }
    rhashtable_walk_start(hti);
    loop {
        let nlsk: *mut netlink_sock = rhashtable_walk_next(hti);
        if nlsk.is_null() { break; }
        if IS_ERR(nlsk) {
            ret = PTR_ERR(nlsk);
            if ret == -EAGAIN { ret = 0; continue; }
            break;
        }
        sk = nlsk as *mut sock;
        if !net_eq(sock_net(sk), net) { continue; }
        if sk_diag_fill(sk, skb, req, NETLINK_CB((*cb).skb).portid,
                        (*(*cb).nlh).nlmsg_seq, NLM_F_MULTI, sock_i_ino(sk)) < 0 {
            ret = 1; break;
        }
    }
    rhashtable_walk_stop(hti);
    if ret != 0 { (*cb).args[0] = num; return ret; }
    rhashtable_walk_exit(hti);
    num += 1;
    }

    let mut irq_flags: ulong = 0;
    read_lock_irqsave(&nl_table_lock, &mut irq_flags);
    sk_for_each_bound(sk, &(*tbl).mc_list) {
        if sk_hashed(sk) { continue; }
        if !net_eq(sock_net(sk), net) { continue; }
        if num < s_num { num += 1; continue; }
        if sk_diag_fill(sk, skb, req, NETLINK_CB((*cb).skb).portid,
                        (*(*cb).nlh).nlmsg_seq, NLM_F_MULTI, sock_i_ino(sk)) < 0 {
            ret = 1; break;
        }
        num += 1;
    }
    read_unlock_irqrestore(&nl_table_lock, irq_flags);
    (*cb).args[0] = num;
    ret
}

unsafe fn netlink_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let req: *mut netlink_diag_req = nlmsg_data((*cb).nlh);
    let mut s_num = (*cb).args[0] as c_int;
    let mut err = 0;
    if (*req).sdiag_protocol == NDIAG_PROTO_ALL {
        let mut i = (*cb).args[1] as c_int;
        while i < MAX_LINKS {
            err = __netlink_diag_dump(skb, cb, i, s_num);
            if err != 0 { break; }
            s_num = 0;
            i += 1;
        }
        (*cb).args[1] = i as long;
    } else {
        if (*req).sdiag_protocol >= MAX_LINKS { return -ENOENT; }
        err = __netlink_diag_dump(skb, cb, (*req).sdiag_protocol as c_int, s_num);
    }
    if err <= 0 { err } else { (*skb).len as c_int }
}

unsafe fn netlink_diag_dump_done(cb: *mut netlink_callback) -> c_int {
    let hti = (*cb).args[2] as *mut rhashtable_iter;
    if (*cb).args[0] == 1 { rhashtable_walk_exit(hti); }
    kfree(hti as *mut c_void);
    0
}

unsafe fn netlink_diag_handler_dump(skb: *mut sk_buff, h: *mut nlmsghdr) -> c_int {
    let hdrlen = size_of::<netlink_diag_req>();
    let net = sock_net((*skb).sk);
    if nlmsg_len(h) < hdrlen { return -EINVAL; }
    if (*h).nlmsg_flags & NLM_F_DUMP != 0 {
        let c = netlink_dump_control { dump: Some(netlink_diag_dump), done: Some(netlink_diag_dump_done) };
        netlink_dump_start((*net).diag_nlsk, skb, h, &c)
    } else { -EOPNOTSUPP }
}

static netlink_diag_handler: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE,
    family: AF_NETLINK,
    dump: Some(netlink_diag_handler_dump),
};

unsafe fn netlink_diag_init() -> c_int {
    sock_diag_register(&netlink_diag_handler)
}

unsafe fn netlink_diag_exit() {
    sock_diag_unregister(&netlink_diag_handler);
}

// module_init(netlink_diag_init);
// module_exit(netlink_diag_exit);
// MODULE_DESCRIPTION("Netlink-based socket monitoring/diagnostic interface (sock_diag)");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 16 /* AF_NETLINK */);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
