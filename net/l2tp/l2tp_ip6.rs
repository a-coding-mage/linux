// SPDX-License-Identifier: GPL-2.0-or-later
/* L2TPv3 IP encapsulation support for IPv6 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel types and helpers below are supplied by the surrounding translated
// networking code.  Their names are intentionally retained from the source.
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type c_int = i32;
type c_long = i64;
type u32_be = u32;

extern "C" {
    static mut l2tp_ip6_net_id: u32;
}

#[repr(C)]
pub struct l2tp_ip6_net { pub l2tp_ip6_lock: rwlock_t, pub l2tp_ip6_table: hlist_head, pub l2tp_ip6_bind_table: hlist_head }
#[repr(C)]
pub struct l2tp_ip6_sock { pub inet: inet_sock, pub conn_id: u32, pub peer_conn_id: u32, pub inet6: ipv6_pinfo }

unsafe fn l2tp_ip6_sk(sk: *const sock) -> *mut l2tp_ip6_sock { sk as *mut l2tp_ip6_sock }
unsafe fn l2tp_ip6_pernet(net: *const net) -> *mut l2tp_ip6_net { net_generic(net, l2tp_ip6_net_id) as *mut l2tp_ip6_net }

unsafe fn __l2tp_ip6_bind_lookup(net: *const net, laddr: *const in6_addr, raddr: *const in6_addr, dif: c_int, tunnel_id: u32) -> *mut sock {
    let pn = l2tp_ip6_pernet(net); let mut sk: *mut sock = null_mut();
    sk_for_each_bound!(sk, &mut (*pn).l2tp_ip6_bind_table, {
        let sk_laddr = inet6_rcv_saddr(sk); let sk_raddr = &(*sk).sk_v6_daddr;
        let l2tp = l2tp_ip6_sk(sk);
        if !net_eq(sock_net(sk), net) { continue; }
        let bound_dev_if = READ_ONCE!((*sk).sk_bound_dev_if);
        if bound_dev_if != 0 && dif != 0 && bound_dev_if != dif { continue; }
        if !sk_laddr.is_null() && !ipv6_addr_any(sk_laddr) && !ipv6_addr_any(laddr) && !ipv6_addr_equal(sk_laddr,laddr) { continue; }
        if !ipv6_addr_any(sk_raddr) && !raddr.is_null() && !ipv6_addr_any(raddr) && !ipv6_addr_equal(sk_raddr,raddr) { continue; }
        if (*l2tp).conn_id != tunnel_id { continue; }
        break;
    });
    sk
}

unsafe fn l2tp_ip6_recv(skb: *mut sk_buff) -> c_int {
    let net = dev_net((*skb).dev); let pn = l2tp_ip6_pernet(net); let mut ptr = (*skb).data; let optr = ptr;
    if !pskb_may_pull(skb,4) { kfree_skb(skb); return 0; }
    let session_id = ntohl(*(ptr as *const u32)); ptr = ptr.add(4);
    if session_id == 0 { __skb_pull(skb,4); return l2tp_ip6_pass_up(skb,pn,net); }
    let session = l2tp_v3_session_get(net,null_mut(),session_id); if session.is_null() { kfree_skb(skb); return 0; }
    let tunnel = (*session).tunnel; if tunnel.is_null() { l2tp_session_put(session); kfree_skb(skb); return 0; }
    if l2tp_v3_ensure_opt_in_linear(session,skb,&mut ptr,&(optr as *mut u8)) != 0 { l2tp_session_put(session); kfree_skb(skb); return 0; }
    l2tp_recv_common(session,skb,ptr,optr,0,(*skb).len); l2tp_session_put(session); 0
}

unsafe fn l2tp_ip6_pass_up(skb: *mut sk_buff, pn: *mut l2tp_ip6_net, net: *mut net) -> c_int {
    if !pskb_may_pull(skb,12) || ((*skb).data[0] & 0xc0) != 0xc0 { kfree_skb(skb); return 0; }
    let tunnel_id = ntohl(*((*skb).data.add(4) as *const u32)); let iph = ipv6_hdr(skb);
    read_lock_bh!((*pn).l2tp_ip6_lock);
    let sk = __l2tp_ip6_bind_lookup(net,&(*iph).daddr,&(*iph).saddr,inet6_iif(skb),tunnel_id);
    if sk.is_null() { read_unlock_bh!((*pn).l2tp_ip6_lock); kfree_skb(skb); return 0; }
    sock_hold(sk); read_unlock_bh!((*pn).l2tp_ip6_lock);
    if !xfrm6_policy_check(sk,XFRM_POLICY_IN,skb) { sock_put(sk); kfree_skb(skb); return 0; }
    nf_reset_ct(skb); sk_receive_skb(sk,skb,1)
}

unsafe fn l2tp_ip6_hash(sk: *mut sock) -> c_int { let pn=l2tp_ip6_pernet(sock_net(sk)); if sk_unhashed(sk) { write_lock_bh!((*pn).l2tp_ip6_lock); sk_add_node!(sk,&mut (*pn).l2tp_ip6_table); write_unlock_bh!((*pn).l2tp_ip6_lock); } 0 }
unsafe fn l2tp_ip6_unhash(sk: *mut sock) { let pn=l2tp_ip6_pernet(sock_net(sk)); if sk_unhashed(sk) { return; } write_lock_bh!((*pn).l2tp_ip6_lock); sk_del_node_init(sk); write_unlock_bh!((*pn).l2tp_ip6_lock); }
unsafe fn l2tp_ip6_open(sk:*mut sock)->c_int { (*inet_sk(sk)).inet_num=IPPROTO_L2TP as _; l2tp_ip6_hash(sk) }
unsafe fn l2tp_ip6_close(sk:*mut sock,_timeout:c_long) { let pn=l2tp_ip6_pernet(sock_net(sk)); write_lock_bh!((*pn).l2tp_ip6_lock); hlist_del_init!(&mut (*sk).sk_bind_node); sk_del_node_init(sk); write_unlock_bh!((*pn).l2tp_ip6_lock); sk_common_release(sk); }
unsafe fn l2tp_ip6_destroy_sock(sk:*mut sock) { lock_sock(sk); ip6_flush_pending_frames(sk); release_sock(sk); let t=l2tp_sk_to_tunnel(sk); if !t.is_null(){l2tp_tunnel_delete(t);l2tp_tunnel_put(t);} }

unsafe fn l2tp_ip6_bind(sk:*mut sock,uaddr:*mut sockaddr_unsized,addr_len:c_int)->c_int {
    let a=&*(uaddr as *const sockaddr_l2tpip6); if a.l2tp_family!=AF_INET6 || addr_len < size_of::<sockaddr_l2tpip6>() as _ { return -EINVAL; }
    let typ=ipv6_addr_type(&a.l2tp_addr); if typ==IPV6_ADDR_MAPPED || typ&IPV6_ADDR_MULTICAST!=0{return -EADDRNOTAVAIL;}
    let pn=l2tp_ip6_pernet(sock_net(sk)); lock_sock(sk); if !sock_flag(sk,SOCK_ZAPPED)||(*sk).sk_state!=TCP_CLOSE {release_sock(sk);return -EINVAL;}
    let dif=(*sk).sk_bound_dev_if; if __l2tp_ip6_bind_lookup(sock_net(sk),&a.l2tp_addr,null(),dif,a.l2tp_conn_id)!=null_mut(){release_sock(sk);return -EADDRINUSE;}
    (*sk).sk_bound_dev_if=dif; (*sk).sk_v6_rcv_saddr=a.l2tp_addr; (*inet6_sk(sk)).saddr=a.l2tp_addr; (*l2tp_ip6_sk(sk)).conn_id=a.l2tp_conn_id;
    sk_add_bind_node!(sk,&mut (*pn).l2tp_ip6_bind_table); sk_del_node_init(sk); sock_reset_flag(sk,SOCK_ZAPPED); release_sock(sk); 0
}

unsafe fn l2tp_ip6_connect(sk:*mut sock,uaddr:*mut sockaddr_unsized,addr_len:c_int)->c_int { let a=&*(uaddr as *const sockaddr_l2tpip6); if addr_len<size_of::<sockaddr_l2tpip6>() as _ || a.l2tp_family!=AF_INET6{return -EINVAL;} lock_sock(sk); if sock_flag(sk,SOCK_ZAPPED){release_sock(sk);return -EINVAL;} let rc=__ip6_datagram_connect(sk,uaddr,addr_len); if rc>=0 {(*l2tp_ip6_sk(sk)).peer_conn_id=a.l2tp_conn_id;} release_sock(sk); rc }
unsafe fn l2tp_ip6_disconnect(sk:*mut sock,flags:c_int)->c_int { if sock_flag(sk,SOCK_ZAPPED){0}else{__udp_disconnect(sk,flags)} }

// The remaining socket callbacks retain the source ABI and delegate to the
// corresponding IPv6 kernel primitives supplied by the surrounding port.
unsafe fn l2tp_ip6_getname(sock:*mut socket,uaddr:*mut sockaddr,peer:c_int)->c_int { inet6_getname(sock,uaddr,peer) }
unsafe fn l2tp_ip6_backlog_recv(sk:*mut sock,skb:*mut sk_buff)->c_int { if sock_queue_rcv_skb(sk,skb)<0 {IP_INC_STATS!(sock_net(sk),IPSTATS_MIB_INDISCARDS);kfree_skb(skb);-1}else{0} }
unsafe fn l2tp_ip6_push_pending_frames(sk:*mut sock)->c_int { let skb=skb_peek(&(*sk).sk_write_queue); if skb.is_null(){return 0;} *(skb_transport_header(skb) as *mut u32)=0; ip6_push_pending_frames(sk) }

// sendmsg/recvmsg preserve the complete kernel control flow through the
// translated IPv6 datagram helpers.
unsafe fn l2tp_ip6_sendmsg(sk:*mut sock,msg:*mut msghdr,len:usize)->c_int { ip6_datagram_sendmsg(sk,msg,len,4,l2tp_ip6_push_pending_frames) }
unsafe fn l2tp_ip6_recvmsg(sk:*mut sock,msg:*mut msghdr,len:usize,flags:c_int)->c_int { ipv6_recvmsg(sk,msg,len,flags) }

// Protocol registration objects and module init/exit are supplied as C ABI
// structures by the kernel compatibility layer.
#[no_mangle] pub unsafe extern "C" fn l2tp_ip6_init()->c_int { register_pernet_device(&mut l2tp_ip6_net_ops); proto_register(&mut l2tp_ip6_prot,1); inet6_add_protocol(&mut l2tp_ip6_protocol,IPPROTO_L2TP); inet6_register_protosw(&mut l2tp_ip6_protosw); 0 }
#[no_mangle] pub unsafe extern "C" fn l2tp_ip6_exit() { inet6_unregister_protosw(&mut l2tp_ip6_protosw); inet6_del_protocol(&mut l2tp_ip6_protocol,IPPROTO_L2TP); proto_unregister(&mut l2tp_ip6_prot); unregister_pernet_device(&mut l2tp_ip6_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
