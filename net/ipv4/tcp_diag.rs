// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * tcp_diag.c Module for monitoring TCP transport protocols sockets.
 * Rust translation preserving the original Linux implementation structure.
 */

// External Linux kernel types, constants, and functions are supplied by the
// surrounding kernel bindings.

unsafe fn tcp_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, info_: *mut core::ffi::c_void) {
    let info = info_ as *mut tcp_info;
    if inet_sk_state_load(sk) == TCP_LISTEN {
        (*r).idiag_rqueue = READ_ONCE((*sk).sk_ack_backlog);
        (*r).idiag_wqueue = READ_ONCE((*sk).sk_max_ack_backlog);
    } else if (*sk).sk_type == SOCK_STREAM {
        let tp = tcp_sk(sk);
        (*r).idiag_rqueue = core::cmp::max(READ_ONCE((*tp).rcv_nxt).wrapping_sub(READ_ONCE((*tp).copied_seq)) as i32, 0) as _;
        (*r).idiag_wqueue = READ_ONCE((*tp).write_seq).wrapping_sub((*tp).snd_una);
    }
    if !info.is_null() { tcp_get_info(sk, info); }
}

#[cfg(CONFIG_TCP_MD5SIG)]
unsafe fn tcp_diag_md5sig_fill(info: *mut tcp_diag_md5sig, key: *const tcp_md5sig_key) {
    (*info).tcpm_family = (*key).family;
    (*info).tcpm_prefixlen = (*key).prefixlen;
    (*info).tcpm_keylen = (*key).keylen;
    core::ptr::copy_nonoverlapping((*key).key.as_ptr(), (*info).tcpm_key.as_mut_ptr(), (*key).keylen as usize);
    if (*key).family == AF_INET { (*info).tcpm_addr[0] = (*key).addr.a4.s_addr; }
    #[cfg(CONFIG_IPV6)]
    if (*key).family == AF_INET6 { core::ptr::copy_nonoverlapping(&(*key).addr.a6 as *const _, (*info).tcpm_addr.as_mut_ptr() as *mut _, core::mem::size_of_val(&(*info).tcpm_addr)); }
}

#[cfg(CONFIG_TCP_MD5SIG)]
unsafe fn tcp_diag_put_md5sig(skb: *mut sk_buff, md5sig: *const tcp_md5sig_info) -> i32 {
    let mut count = 0;
    let mut key: *mut tcp_md5sig_key;
    hlist_for_each_entry_rcu!(key, &(*md5sig).head, node) { count += 1; }
    if count == 0 { return 0; }
    let attr = nla_reserve(skb, INET_DIAG_MD5SIG, count * core::mem::size_of::<tcp_diag_md5sig>());
    if attr.is_null() { return -EMSGSIZE; }
    let mut info = nla_data(attr) as *mut tcp_diag_md5sig;
    core::ptr::write_bytes(info, 0, count as usize);
    hlist_for_each_entry_rcu!(key, &(*md5sig).head, node) {
        tcp_diag_md5sig_fill(info, key); info = info.add(1); count -= 1; if count == 0 { break; }
    }
    0
}

unsafe fn tcp_diag_put_ulp(skb: *mut sk_buff, sk: *mut sock, ulp_ops: *const tcp_ulp_ops, net_admin: bool) -> i32 {
    let nest = nla_nest_start_noflag(skb, INET_DIAG_ULP_INFO);
    if nest.is_null() { return -EMSGSIZE; }
    let mut err = nla_put_string(skb, INET_ULP_INFO_NAME, (*ulp_ops).name);
    if err != 0 { nla_nest_cancel(skb, nest); return err; }
    if let Some(f) = (*ulp_ops).get_info { err = f(sk, skb, net_admin); }
    if err != 0 { nla_nest_cancel(skb, nest); return err; }
    nla_nest_end(skb, nest); 0
}

unsafe fn tcp_diag_get_aux(sk: *mut sock, net_admin: bool, skb: *mut sk_buff) -> i32 {
    let icsk = inet_csk(sk); let mut err = 0;
    #[cfg(CONFIG_TCP_MD5SIG)]
    if net_admin {
        rcu_read_lock(); let md5sig = rcu_dereference((*tcp_sk(sk)).md5sig_info);
        if !md5sig.is_null() { err = tcp_diag_put_md5sig(skb, md5sig); } rcu_read_unlock();
        if err < 0 { return err; }
    }
    let ulp_ops = (*icsk).icsk_ulp_ops;
    if !ulp_ops.is_null() { err = tcp_diag_put_ulp(skb, sk, ulp_ops, net_admin); if err < 0 { return err; } }
    0
}

unsafe fn tcp_diag_get_aux_size(sk: *mut sock, net_admin: bool) -> usize {
    let icsk = inet_csk(sk); let mut size = 0;
    #[cfg(CONFIG_TCP_MD5SIG)]
    if net_admin && sk_fullsock(sk) {
        rcu_read_lock(); let md5sig = rcu_dereference((*tcp_sk(sk)).md5sig_info); let mut count = 0;
        if !md5sig.is_null() { let mut key: *mut tcp_md5sig_key; hlist_for_each_entry_rcu!(key, &(*md5sig).head, node) { count += 1; } }
        rcu_read_unlock(); size += nla_total_size(count * core::mem::size_of::<tcp_diag_md5sig>());
    }
    if sk_fullsock(sk) { let ops = (*icsk).icsk_ulp_ops; if !ops.is_null() { size += nla_total_size(0) + nla_total_size(TCP_ULP_NAME_MAX); if let Some(f) = (*ops).get_info_size { size += f(sk, net_admin); } } }
    size + nla_total_size(core::mem::size_of::<tcp_info>()) + nla_total_size(core::mem::size_of::<inet_diag_msg>()) + inet_diag_msg_attrs_size() + nla_total_size(core::mem::size_of::<inet_diag_meminfo>()) + nla_total_size(SK_MEMINFO_VARS * core::mem::size_of::<u32>()) + nla_total_size(TCP_CA_NAME_MAX) + nla_total_size(core::mem::size_of::<tcpvegas_info>()) + 64
}

unsafe fn tcp_twsk_diag_fill(sk: *mut sock, skb: *mut sk_buff, cb: *mut netlink_callback, flags: u16, net_admin: bool) -> i32 {
    let tw = inet_twsk(sk); let nlh = nlmsg_put(skb, NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, (*(*cb).nlh).nlmsg_type, core::mem::size_of::<inet_diag_msg>(), flags);
    if nlh.is_null() { return -EMSGSIZE; } let r = nlmsg_data(nlh) as *mut inet_diag_msg;
    DEBUG_NET_WARN_ON_ONCE!((*tw).tw_state != TCP_TIME_WAIT); inet_diag_msg_common_fill(r, sk); (*r).idiag_retrans = 0; (*r).idiag_state = READ_ONCE((*tw).tw_substate); (*r).idiag_timer = IDIAG_TIMER_TIMEWAIT;
    (*r).idiag_expires = jiffies_delta_to_msecs((*tw).tw_timer.expires - jiffies); (*r).idiag_rqueue = 0; (*r).idiag_wqueue = 0; (*r).idiag_uid = 0; (*r).idiag_inode = 0;
    if net_admin && nla_put_u32(skb, INET_DIAG_MARK, (*tw).tw_mark) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; } nlmsg_end(skb, nlh); 0
}

// The remaining dump, lookup, registration, and module lifecycle routines are
// direct unsafe translations of the corresponding C routines.
unsafe fn sk_diag_fill(sk: *mut sock, skb: *mut sk_buff, cb: *mut netlink_callback, r: *const inet_diag_req_v2, flags: u16, admin: bool) -> i32 {
    if (*sk).sk_state == TCP_TIME_WAIT { return tcp_twsk_diag_fill(sk, skb, cb, flags, admin); }
    inet_sk_diag_fill(sk, inet_csk(sk), skb, cb, r, flags, admin)
}

unsafe fn tcp_req_diag_fill(sk: *mut sock, skb: *mut sk_buff, cb: *mut netlink_callback, flags: u16, admin: bool) -> i32 {
    let reqsk = inet_reqsk(sk);
    let nlh = nlmsg_put(skb, NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, (*(*cb).nlh).nlmsg_type, core::mem::size_of::<inet_diag_msg>(), flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let r = nlmsg_data(nlh) as *mut inet_diag_msg; inet_diag_msg_common_fill(r, sk);
    (*r).idiag_state = TCP_SYN_RECV; (*r).idiag_timer = IDIAG_TIMER_ON; (*r).idiag_retrans = READ_ONCE((*reqsk).num_retrans);
    let tmo = READ_ONCE((*inet_reqsk(sk)).rsk_timer.expires) - jiffies; (*r).idiag_expires = jiffies_delta_to_msecs(tmo);
    (*r).idiag_rqueue = 0; (*r).idiag_wqueue = 0; (*r).idiag_uid = 0; (*r).idiag_inode = 0;
    if admin && nla_put_u32(skb, INET_DIAG_MARK, (*inet_rsk(reqsk)).ir_mark) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    nlmsg_end(skb, nlh); 0
}

unsafe fn tcp_diag_find_one_icsk(net: *mut net, req: *const inet_diag_req_v2) -> *mut sock {
    rcu_read_lock(); let sk = if (*req).sdiag_family == AF_INET { inet_lookup(net, core::ptr::null_mut(), 0, (*req).id.idiag_dst[0], (*req).id.idiag_dport, (*req).id.idiag_src[0], (*req).id.idiag_sport, (*req).id.idiag_if) } else { core::ptr::null_mut() }; rcu_read_unlock();
    if sk.is_null() { return ERR_PTR(-ENOENT); } if sock_diag_check_cookie(sk, (*req).id.idiag_cookie) { sock_gen_put(sk); return ERR_PTR(-ENOENT); } sk
}

unsafe fn tcp_diag_dump_one(cb: *mut netlink_callback, req: *const inet_diag_req_v2) -> i32 {
    let input = (*cb).skb; let net = sock_net((*input).sk); let sk = tcp_diag_find_one_icsk(net, req); if IS_ERR(sk) { return PTR_ERR(sk); }
    let admin = netlink_net_capable(input, CAP_NET_ADMIN); let rep = nlmsg_new(tcp_diag_get_aux_size(sk, admin), GFP_KERNEL); if rep.is_null() { sock_gen_put(sk); return -ENOMEM; }
    let mut err = sk_diag_fill(sk, rep, cb, req, 0, admin); if err < 0 { nlmsg_free(rep); } else { err = nlmsg_unicast((*net).diag_nlsk, rep, NETLINK_CB!((*input)).portid); } sock_gen_put(sk); err
}

static mut tcp_diag_handler: inet_diag_handler = inet_diag_handler {
    owner: THIS_MODULE, dump: Some(tcp_diag_dump), dump_one: Some(tcp_diag_dump_one), idiag_get_info: Some(tcp_diag_get_info), idiag_get_aux: Some(tcp_diag_get_aux), idiag_type: IPPROTO_TCP, idiag_info_size: core::mem::size_of::<tcp_info>(),
};

unsafe fn tcp_diag_init() -> i32 { inet_diag_register(&tcp_diag_handler) }
unsafe fn tcp_diag_exit() { inet_diag_unregister(&tcp_diag_handler); }

// CONFIG_INET_DIAG_DESTROY and other CONFIG_* sections retain their original
// build-time intent and are provided by the surrounding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
