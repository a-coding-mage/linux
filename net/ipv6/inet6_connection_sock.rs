// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET        An implementation of the TCP/IP protocol suite for the LINUX
 *             operating system.  INET is implemented using the  BSD Socket
 *             interface as the means of communication with the user level.
 *
 *             Support for INET6 connection oriented protocols.
 *
 * Authors:    See the TCPv6 sources
 */

// External kernel declarations supplied by the corresponding Linux headers:
// linux/module.h, linux/in6.h, linux/ipv6.h, linux/jhash.h, linux/slab.h,
// net/addrconf.h, net/inet_connection_sock.h, net/inet_ecn.h,
// net/inet_hashtables.h, net/ip6_route.h, net/sock.h,
// net/inet6_connection_sock.h, and net/sock_reuseport.h.

pub unsafe fn inet6_csk_route_req(
    sk: *const sock,
    mut dst: *mut dst_entry,
    fl6: *mut flowi6,
    req: *const request_sock,
    proto: u8,
) -> *mut dst_entry {
    let ireq: *const inet_request_sock = inet_rsk(req);
    let np: *const ipv6_pinfo = inet6_sk(sk);
    let mut final_p: *mut in6_addr;
    let mut final_addr: in6_addr = core::mem::zeroed();

    core::ptr::write_bytes(fl6, 0, 1);
    (*fl6).flowi6_proto = proto;
    (*fl6).daddr = (*ireq).ir_v6_rmt_addr;
    rcu_read_lock();
    final_p = fl6_update_dst(fl6, rcu_dereference((*np).opt), &mut final_addr);
    rcu_read_unlock();
    (*fl6).saddr = (*ireq).ir_v6_loc_addr;
    (*fl6).flowi6_oif = (*ireq).ir_iif;
    (*fl6).flowi6_mark = (*ireq).ir_mark;
    (*fl6).fl6_dport = (*ireq).ir_rmt_port;
    (*fl6).fl6_sport = htons((*ireq).ir_num);
    (*fl6).flowi6_uid = sk_uid(sk);
    security_req_classify_flow(req, flowi6_to_flowi_common(fl6));

    ip6_ecmp_set_mp_hash(sock_net(sk), fl6, (*tcp_rsk(req)).txhash);

    if dst.is_null() {
        dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, final_p);
        if IS_ERR(dst) {
            return core::ptr::null_mut();
        }
    }
    dst
}

pub unsafe fn inet6_csk_route_socket(
    sk: *mut sock,
    fl6: *mut flowi6,
) -> *mut dst_entry {
    let inet: *mut inet_sock = inet_sk(sk);
    let np: *mut ipv6_pinfo = inet6_sk(sk);
    let final_p: *mut in6_addr;
    let dst: *mut dst_entry;

    core::ptr::write_bytes(fl6, 0, 1);
    (*fl6).flowi6_proto = (*sk).sk_protocol;
    (*fl6).daddr = (*sk).sk_v6_daddr;
    (*fl6).saddr = (*np).saddr;
    (*fl6).flowlabel = (*np).flow_label;
    IP6_ECN_flow_xmit(sk, (*fl6).flowlabel);

    if (*sk).sk_protocol == IPPROTO_TCP {
        ip6_ecmp_set_mp_hash(sock_net(sk), fl6, (*sk).sk_txhash);
    }
    (*fl6).flowi6_oif = (*sk).sk_bound_dev_if;
    (*fl6).flowi6_mark = (*sk).sk_mark;
    (*fl6).fl6_sport = (*inet).inet_sport;
    (*fl6).fl6_dport = (*inet).inet_dport;
    (*fl6).flowi6_uid = sk_uid(sk);
    security_sk_classify_flow(sk, flowi6_to_flowi_common(fl6));

    rcu_read_lock();
    final_p = fl6_update_dst(fl6, rcu_dereference((*np).opt), &mut (*np).r#final);
    rcu_read_unlock();

    dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, final_p);

    if !IS_ERR(dst) {
        ip6_dst_store(sk, dst, false, false);
    }

    dst
}

pub unsafe fn inet6_csk_xmit(
    sk: *mut sock,
    skb: *mut sk_buff,
    _fl_unused: *mut flowi,
) -> i32 {
    let fl6: *mut flowi6 = &mut (*inet_sk(sk)).cork.fl.u.ip6;
    let np: *mut ipv6_pinfo = inet6_sk(sk);
    let mut dst: *mut dst_entry;
    let res: i32;

    dst = __sk_dst_check(sk, (*np).dst_cookie);
    if unlikely(dst.is_null()) {
        dst = inet6_csk_route_socket(sk, fl6);
        if IS_ERR(dst) {
            WRITE_ONCE((*sk).sk_err_soft, -PTR_ERR(dst));
            (*sk).sk_route_caps = 0;
            sk_skb_reason_drop(sk, skb, SKB_DROP_REASON_IP_OUTNOROUTES);
            return PTR_ERR(dst);
        }
        /* Restore final destination back after routing done */
        (*fl6).daddr = (*sk).sk_v6_daddr;
    }

    rcu_read_lock();
    skb_dst_set_noref(skb, dst);

    res = ip6_xmit(
        sk,
        skb,
        fl6,
        (*sk).sk_mark,
        rcu_dereference((*np).opt),
        (*np).tclass,
        READ_ONCE((*sk).sk_priority),
    );
    rcu_read_unlock();
    res
}

// EXPORT_SYMBOL_GPL(inet6_csk_xmit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
