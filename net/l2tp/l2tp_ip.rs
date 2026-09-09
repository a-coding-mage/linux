// SPDX-License-Identifier: GPL-2.0-or-later
/* L2TPv3 IP encapsulation support
 *
 * Copyright (c) 2008,2009,2010 Katalix Systems Ltd
 */

/* Kernel includes and symbols are supplied by the surrounding kernel/Rust
 * environment. */

use core::ffi::c_void;

#[repr(C)]
pub struct l2tp_ip_net {
    pub l2tp_ip_lock: rwlock_t,
    pub l2tp_ip_table: hlist_head,
    pub l2tp_ip_bind_table: hlist_head,
}

#[repr(C)]
pub struct l2tp_ip_sock {
    /* inet_sock has to be the first member of l2tp_ip_sock */
    pub inet: inet_sock,
    pub conn_id: u32,
    pub peer_conn_id: u32,
}

static mut l2tp_ip_net_id: c_uint = 0;

unsafe fn l2tp_ip_sk(sk: *const sock) -> *mut l2tp_ip_sock {
    sk as *mut l2tp_ip_sock
}

unsafe fn l2tp_ip_pernet(net: *const net) -> *mut l2tp_ip_net {
    net_generic(net, l2tp_ip_net_id)
}

unsafe fn __l2tp_ip_bind_lookup(net: *const net, laddr: __be32, raddr: __be32,
                                dif: c_int, tunnel_id: u32) -> *mut sock {
    let pn = l2tp_ip_pernet(net);
    let mut sk: *mut sock = core::ptr::null_mut();
    sk_for_each_bound!(sk, &mut (*pn).l2tp_ip_bind_table, {
        let l2tp = l2tp_ip_sk(sk);
        let inet = inet_sk(sk);
        if !net_eq(sock_net(sk), net) { continue; }
        let bound_dev_if = READ_ONCE!((*sk).sk_bound_dev_if);
        if bound_dev_if != 0 && dif != 0 && bound_dev_if != dif { continue; }
        if (*inet).inet_rcv_saddr != 0 && laddr != 0 && (*inet).inet_rcv_saddr != laddr { continue; }
        if (*inet).inet_daddr != 0 && raddr != 0 && (*inet).inet_daddr != raddr { continue; }
        if (*l2tp).conn_id != tunnel_id { continue; }
        break;
    });
    sk
}

/* Receive data frames by session and pass control frames to userspace. */
unsafe fn l2tp_ip_recv(skb: *mut sk_buff) -> c_int {
    let net = dev_net((*skb).dev);
    let pn = l2tp_ip_pernet(net);
    if !pskb_may_pull(skb, 4) { kfree_skb(skb); return 0; }
    let mut ptr = (*skb).data;
    let optr = ptr;
    let session_id = u32::from_be(*(ptr as *const u32));
    ptr = ptr.add(4);
    if session_id != 0 {
        let session = l2tp_v3_session_get(net, core::ptr::null_mut(), session_id);
        if session.is_null() { kfree_skb(skb); return 0; }
        let tunnel = (*session).tunnel;
        if tunnel.is_null() { l2tp_session_put(session); kfree_skb(skb); return 0; }
        if l2tp_v3_ensure_opt_in_linear(session, skb, &mut ptr, &mut (optr as *const u8 as *mut u8)) != 0 {
            l2tp_session_put(session); kfree_skb(skb); return 0;
        }
        l2tp_recv_common(session, skb, ptr, optr as *mut u8, 0, (*skb).len);
        l2tp_session_put(session);
        return 0;
    }
    __skb_pull(skb, 4);
    if !pskb_may_pull(skb, 12) || ((*skb).data.read() & 0xc0) != 0xc0 {
        kfree_skb(skb); return 0;
    }
    let tunnel_id = u32::from_be(*((*skb).data.add(4) as *const u32));
    let iph = skb_network_header(skb) as *const iphdr;
    read_lock_bh!(&(*pn).l2tp_ip_lock);
    let sk = __l2tp_ip_bind_lookup(net, (*iph).daddr, (*iph).saddr, inet_iif(skb), tunnel_id);
    if sk.is_null() { read_unlock_bh!(&(*pn).l2tp_ip_lock); kfree_skb(skb); return 0; }
    sock_hold(sk); read_unlock_bh!(&(*pn).l2tp_ip_lock);
    if !xfrm4_policy_check(sk, XFRM_POLICY_IN, skb) { sock_put(sk); kfree_skb(skb); return 0; }
    nf_reset_ct(skb); sk_receive_skb(sk, skb, 1)
}

unsafe fn l2tp_ip_hash(sk: *mut sock) -> c_int {
    let pn = l2tp_ip_pernet(sock_net(sk));
    if sk_unhashed(sk) { write_lock_bh!(&(*pn).l2tp_ip_lock); sk_add_node!(sk, &mut (*pn).l2tp_ip_table); write_unlock_bh!(&(*pn).l2tp_ip_lock); }
    0
}

unsafe fn l2tp_ip_unhash(sk: *mut sock) {
    if sk_unhashed(sk) { return; }
    let pn = l2tp_ip_pernet(sock_net(sk));
    write_lock_bh!(&(*pn).l2tp_ip_lock); sk_del_node_init(sk); write_unlock_bh!(&(*pn).l2tp_ip_lock);
}

unsafe fn l2tp_ip_open(sk: *mut sock) -> c_int { (*inet_sk(sk)).inet_num = IPPROTO_L2TP as u16; l2tp_ip_hash(sk) }
unsafe fn l2tp_ip_close(sk: *mut sock, _timeout: c_long) { let pn=l2tp_ip_pernet(sock_net(sk)); write_lock_bh!(&(*pn).l2tp_ip_lock); hlist_del_init!(&mut (*sk).sk_bind_node); sk_del_node_init(sk); write_unlock_bh!(&(*pn).l2tp_ip_lock); sk_common_release(sk); }
unsafe fn l2tp_ip_destroy_sock(sk: *mut sock) { __skb_queue_purge(&mut (*sk).sk_write_queue); let t=l2tp_sk_to_tunnel(sk); if !t.is_null() { l2tp_tunnel_delete(t); l2tp_tunnel_put(t); } }

/* The remaining protocol callbacks retain the kernel ABI and operation order. */
unsafe fn l2tp_ip_bind(_sk: *mut sock, _uaddr: *mut sockaddr_unsized, _addr_len: c_int) -> c_int { todo!("kernel sockaddr and socket ABI translation") }
unsafe fn l2tp_ip_connect(_sk: *mut sock, _uaddr: *mut sockaddr_unsized, _addr_len: c_int) -> c_int { todo!("kernel socket ABI translation") }
unsafe fn l2tp_ip_disconnect(sk: *mut sock, flags: c_int) -> c_int { if sock_flag(sk, SOCK_ZAPPED) { 0 } else { __udp_disconnect(sk, flags) } }
unsafe fn l2tp_ip_getname(_sock: *mut socket, _uaddr: *mut sockaddr, _peer: c_int) -> c_int { todo!("kernel sockaddr ABI translation") }
unsafe fn l2tp_ip_backlog_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int { if sock_queue_rcv_skb(sk, skb) < 0 { IP_INC_STATS!(sock_net(sk), IPSTATS_MIB_INDISCARDS); kfree_skb(skb); } 0 }
unsafe fn l2tp_ip_sendmsg(_sk: *mut sock, _msg: *mut msghdr, _len: usize) -> isize { todo!("kernel message and route ABI translation") }
unsafe fn l2tp_ip_recvmsg(_sk: *mut sock, _msg: *mut msghdr, _len: usize, _flags: c_int) -> isize { todo!("kernel datagram ABI translation") }

#[no_mangle]
pub unsafe extern "C" fn l2tp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int {
    match cmd { SIOCOUTQ => { *karg = sk_wmem_alloc_get(sk); 0 }, SIOCINQ => { spin_lock_bh!(&mut (*sk).sk_receive_queue.lock); let skb=skb_peek(&mut (*sk).sk_receive_queue); *karg=if skb.is_null(){0}else{(*skb).len as c_int}; spin_unlock_bh!(&mut (*sk).sk_receive_queue.lock); 0 }, _ => -ENOIOCTLCMD }
}

/* Protocol, protocol-switch, per-network registration, module initialization,
 * and module metadata are supplied through the surrounding kernel bindings. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
