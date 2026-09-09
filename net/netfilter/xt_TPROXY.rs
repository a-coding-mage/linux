// SPDX-License-Identifier: GPL-2.0-only
/*
 * Transparent proxy support for Linux/iptables
 *
 * Copyright (c) 2006-2010 BalaBit IT Ltd.
 * Author: Balazs Scheidler, Krisztian Kovacs
 */

// C headers and build-time configuration are supplied by the surrounding kernel bindings.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn ip_hdr(skb: *mut sk_buff) -> *const iphdr;
    fn ipv6_hdr(skb: *mut sk_buff) -> *const ipv6hdr;
    fn skb_header_pointer(skb: *mut sk_buff, offset: c_int, len: usize, buffer: *mut c_void) -> *mut udphdr;
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_in(par: *const xt_action_param) -> *mut net_device;
    fn nf_tproxy_get_sock_v4(net: *mut net, skb: *mut sk_buff, protocol: u8, saddr: __be32, daddr: __be32,
                             source: __be16, dest: __be16, dev: *mut net_device, lookup: c_int) -> *mut sock;
    fn nf_tproxy_laddr4(skb: *mut sk_buff, laddr: __be32, daddr: __be32) -> __be32;
    fn nf_tproxy_handle_time_wait4(net: *mut net, skb: *mut sk_buff, laddr: __be32, lport: __be16, sk: *mut sock) -> *mut sock;
    fn nf_tproxy_sk_is_transparent(sk: *mut sock) -> bool;
    fn nf_tproxy_assign_sock(skb: *mut sk_buff, sk: *mut sock);
    fn ipv6_find_hdr(skb: *mut sk_buff, offset: *mut c_int, target: c_int, fragoff: *mut u16, flags: *mut c_void) -> c_int;
    fn nf_tproxy_get_sock_v6(net: *mut net, skb: *mut sk_buff, thoff: c_int, protocol: c_int,
                             saddr: *const in6_addr, daddr: *const in6_addr, source: __be16, dest: __be16,
                             in_dev: *mut net_device, lookup: c_int) -> *mut sock;
    fn nf_tproxy_laddr6(skb: *mut sk_buff, laddr: *const in6_addr, daddr: *const in6_addr) -> *const in6_addr;
    fn nf_tproxy_handle_time_wait6(skb: *mut sk_buff, protocol: c_int, thoff: c_int, net: *mut net,
                                   laddr: *const in6_addr, lport: __be16, sk: *mut sock) -> *mut sock;
    fn nf_defrag_ipv4_enable(net: *mut net) -> c_int;
    fn nf_defrag_ipv4_disable(net: *mut net);
    fn nf_defrag_ipv6_enable(net: *mut net) -> c_int;
    fn nf_defrag_ipv6_disable(net: *mut net);
    fn pr_info_ratelimited(fmt: *const c_char);
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> c_int;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
}

type __be16 = u16;
type __be32 = u32;

#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct iphdr { pub protocol: u8, pub saddr: __be32, pub daddr: __be32 }
#[repr(C)] pub struct ipv6hdr { pub saddr: in6_addr, pub daddr: in6_addr }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct udphdr { pub source: __be16, pub dest: __be16 }
#[repr(C)] pub struct sock { pub sk_state: i32 }
#[repr(C)] pub struct sk_buff { pub mark: u32, pub dev: *mut net_device }
#[repr(C)] pub struct ipt_ip { pub proto: u8, pub invflags: u8 }
#[repr(C)] pub struct ip6t_ip6 { pub proto: u8, pub invflags: u8 }
#[repr(C)] pub struct xt_tproxy_target_info { pub laddr: __be32, pub lport: __be16, pub mark_mask: u32, pub mark_value: u32 }
#[repr(C)] pub struct xt_tproxy_target_info_v1 { pub laddr: xt_tproxy_addr, pub lport: __be16, pub mark_mask: u32, pub mark_value: u32 }
#[repr(C)] pub union xt_tproxy_addr { pub ip: __be32, pub in6: in6_addr }
#[repr(C)] pub struct xt_action_param { pub targinfo: *const c_void, pub fragoff: u16 }
#[repr(C)] pub struct xt_tgchk_param { pub entryinfo: *const c_void, pub net: *mut net }
#[repr(C)] pub struct xt_tgdtor_param { pub net: *mut net }
#[repr(C)] pub struct xt_target {
    pub name: *const c_char, pub family: u16, pub table: *const c_char,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    pub revision: u8, pub targetsize: usize,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>, pub hooks: u32, pub me: *mut c_void,
}

const NF_DROP: u32 = 0;
const NF_ACCEPT: u32 = 1;
const TCP_TIME_WAIT: i32 = 6;
const NF_TPROXY_LOOKUP_ESTABLISHED: c_int = 1;
const NF_TPROXY_LOOKUP_LISTENER: c_int = 2;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const IPT_INV_PROTO: u8 = 2;
const IP6T_INV_PROTO: u8 = 2;
const NFPROTO_IPV4: u16 = 2;
const NFPROTO_IPV6: u16 = 10;
const NF_INET_PRE_ROUTING: u32 = 0;

unsafe extern "C" fn tproxy_tg4(net: *mut net, skb: *mut sk_buff, mut laddr: __be32, mut lport: __be16,
                                 mark_mask: u32, mark_value: u32) -> u32 {
    let iph = ip_hdr(skb); let mut header = udphdr { source: 0, dest: 0 };
    let hp = skb_header_pointer(skb, 0, core::mem::size_of::<udphdr>(), &mut header as *mut _ as *mut c_void);
    if hp.is_null() { return NF_DROP; }
    let mut sk = nf_tproxy_get_sock_v4(net, skb, (*iph).protocol, (*iph).saddr, (*iph).daddr, (*hp).source, (*hp).dest, (*skb).dev, NF_TPROXY_LOOKUP_ESTABLISHED);
    laddr = nf_tproxy_laddr4(skb, laddr, (*iph).daddr); if lport == 0 { lport = (*hp).dest; }
    if !sk.is_null() && (*sk).sk_state == TCP_TIME_WAIT { sk = nf_tproxy_handle_time_wait4(net, skb, laddr, lport, sk); }
    else if sk.is_null() { sk = nf_tproxy_get_sock_v4(net, skb, (*iph).protocol, (*iph).saddr, laddr, (*hp).source, lport, (*skb).dev, NF_TPROXY_LOOKUP_LISTENER); }
    if !sk.is_null() && nf_tproxy_sk_is_transparent(sk) { (*skb).mark = ((*skb).mark & !mark_mask) ^ mark_value; nf_tproxy_assign_sock(skb, sk); return NF_ACCEPT; }
    NF_DROP
}

unsafe extern "C" fn tproxy_tg4_v0(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let tgi = (*par).targinfo as *const xt_tproxy_target_info; if (*par).fragoff != 0 { return NF_DROP; }
    tproxy_tg4(xt_net(par), skb, (*tgi).laddr, (*tgi).lport, (*tgi).mark_mask, (*tgi).mark_value)
}
unsafe extern "C" fn tproxy_tg4_v1(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let tgi = (*par).targinfo as *const xt_tproxy_target_info_v1; if (*par).fragoff != 0 { return NF_DROP; }
    tproxy_tg4(xt_net(par), skb, unsafe { (*tgi).laddr.ip }, (*tgi).lport, (*tgi).mark_mask, (*tgi).mark_value)
}

unsafe extern "C" fn tproxy_tg4_check(par: *const xt_tgchk_param) -> c_int {
    let i = (*par).entryinfo as *const ipt_ip; let err = nf_defrag_ipv4_enable((*par).net); if err != 0 { return err; }
    if ((*i).proto == IPPROTO_TCP || (*i).proto == IPPROTO_UDP) && ((*i).invflags & IPT_INV_PROTO) == 0 { return 0; }
    -22
}
unsafe extern "C" fn tproxy_tg4_destroy(par: *const xt_tgdtor_param) { nf_defrag_ipv4_disable((*par).net); }

// IPv6 implementation is conditionally compiled when CONFIG_IP6_NF_IPTABLES is enabled in the C source.
unsafe extern "C" fn tproxy_tg6_check(par: *const xt_tgchk_param) -> c_int {
    let i = (*par).entryinfo as *const ip6t_ip6; let err = nf_defrag_ipv6_enable((*par).net); if err != 0 { return err; }
    if ((*i).proto == IPPROTO_TCP || (*i).proto == IPPROTO_UDP) && ((*i).invflags & IP6T_INV_PROTO) == 0 { return 0; }
    -22
}
unsafe extern "C" fn tproxy_tg6_destroy(par: *const xt_tgdtor_param) { nf_defrag_ipv6_disable((*par).net); }

static mut tproxy_tg_reg: [xt_target; 2] = [
    xt_target { name: b"TPROXY\0".as_ptr() as *const c_char, family: NFPROTO_IPV4, table: b"mangle\0".as_ptr() as *const c_char,
        target: Some(tproxy_tg4_v0), revision: 0, targetsize: core::mem::size_of::<xt_tproxy_target_info>(),
        checkentry: Some(tproxy_tg4_check), destroy: Some(tproxy_tg4_destroy), hooks: 1 << NF_INET_PRE_ROUTING, me: core::ptr::null_mut() },
    xt_target { name: b"TPROXY\0".as_ptr() as *const c_char, family: NFPROTO_IPV4, table: b"mangle\0".as_ptr() as *const c_char,
        target: Some(tproxy_tg4_v1), revision: 1, targetsize: core::mem::size_of::<xt_tproxy_target_info_v1>(),
        checkentry: Some(tproxy_tg4_check), destroy: Some(tproxy_tg4_destroy), hooks: 1 << NF_INET_PRE_ROUTING, me: core::ptr::null_mut() },
];

#[no_mangle] pub unsafe extern "C" fn tproxy_tg_init() -> c_int {
    xt_register_targets(tproxy_tg_reg.as_mut_ptr(), tproxy_tg_reg.len())
}
#[no_mangle] pub unsafe extern "C" fn tproxy_tg_exit() {
    xt_unregister_targets(tproxy_tg_reg.as_mut_ptr(), tproxy_tg_reg.len());
}

// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
// MODULE_DESCRIPTION("Netfilter transparent proxy (TPROXY) target module.");
// MODULE_ALIAS("ipt_TPROXY"); MODULE_ALIAS("ip6t_TPROXY");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
