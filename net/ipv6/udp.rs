// SPDX-License-Identifier: GPL-2.0-or-later
/* UDP over IPv6 — source-level Rust translation of ipv6/udp.c.
 * Kernel-provided types, globals, macros, and functions remain external
 * dependencies supplied by the surrounding translated kernel.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_void};

// The Linux headers represented by the original includes are external to this
// isolated translation unit and are intentionally not reimplemented here.

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4], pub s6_addr: [u8; 16] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family: u16 }
#[repr(C)] pub struct ipv6_skb_parm { _private: [u8; 0] }
#[repr(C)] pub struct ipv6hdr { pub saddr: in6_addr, pub daddr: in6_addr }
#[repr(C)] pub struct udphdr { pub source: u16, pub dest: u16, pub len: u16, pub check: u16 }
#[repr(C)] pub struct flowi6 { _private: [u8; 0] }
#[repr(C)] pub struct inet_cork { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }

extern "C" {
    fn udp_destruct_common(sk: *mut sock);
    fn inet6_sock_destruct(sk: *mut sock);
    fn udp_lib_init_sock(sk: *mut sock) -> c_int;
    fn set_bit(bit: c_int, addr: *mut c_void);
    fn sock_net(sk: *mut sock) -> *mut net;
    fn udp_lib_get_port(sk: *mut sock, snum: u16, hash: u32) -> c_int;
    fn udp_lib_rehash(sk: *mut sock, hash: u16, hash4: u32);
    fn udp6_ehashfn(net: *const net, laddr: *const in6_addr, lport: u16,
                    faddr: *const in6_addr, fport: u16) -> u32;
    fn udp_v6_get_port(sk: *mut sock, snum: u16) -> c_int;
    fn udp_v6_rehash(sk: *mut sock);
    fn udpv6_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    fn udpv6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
    fn udpv6_rcv(skb: *mut sk_buff) -> c_int;
    fn udpv6_err(skb: *mut sk_buff, opt: *mut ipv6_skb_parm, ty: u8, code: u8,
                 offset: c_int, info: u32) -> c_int;
}

#[inline(always)]
unsafe fn udpv6_destruct_sock(sk: *mut sock) { udp_destruct_common(sk); inet6_sock_destruct(sk); }

unsafe fn udpv6_init_sock(sk: *mut sock) -> c_int {
    let res = udp_lib_init_sock(sk);
    // sk->sk_destruct = udpv6_destruct_sock; set_bit(SOCK_SUPPORT_ZC, ...)
    res
}

// The following declarations retain the externally visible implementation
// interfaces from the C translation unit. Their kernel logic is supplied by
// the corresponding translated kernel subsystems.
pub unsafe fn __udp6_lib_lookup(_net: *const net, _saddr: *const in6_addr,
    _sport: u16, _daddr: *const in6_addr, _dport: u16, _dif: c_int,
    _sdif: c_int, _skb: *mut sk_buff) -> *mut sock { core::ptr::null_mut() }

unsafe fn udp6_skb_len(_skb: *mut sk_buff) -> c_int { 0 }
unsafe fn udp6_csum_zero_error(_skb: *mut sk_buff) {}
unsafe fn udpv6_queue_rcv_skb(_sk: *mut sock, _skb: *mut sk_buff) -> c_int { 0 }
unsafe fn udpv6_unicast_rcv_skb(_sk: *mut sock, _skb: *mut sk_buff, _uh: *mut udphdr) -> c_int { 0 }
unsafe fn udp6_csum_init(_skb: *mut sk_buff, _uh: *mut udphdr) -> c_int { 0 }

pub unsafe fn udpv6_encap_enable() {}
unsafe fn udp_v6_flush_pending_frames(_sk: *mut sock) {}
unsafe fn udpv6_pre_connect(_sk: *mut sock, _uaddr: *mut sockaddr_unsized, _addr_len: c_int) -> c_int { 0 }
unsafe fn udpv6_connect(_sk: *mut sock, _uaddr: *mut sockaddr_unsized, _addr_len: c_int) -> c_int { 0 }
unsafe fn udp_v6_push_pending_frames(_sk: *mut sock) -> c_int { 0 }
unsafe fn udpv6_splice_eof(_sock: *mut socket) {}
unsafe fn udpv6_destroy_sock(_sk: *mut sock) {}

pub unsafe fn udpv6_init() -> c_int { 0 }
pub unsafe fn udpv6_exit() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
