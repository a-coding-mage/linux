// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Florian Westphal <fw@strlen.de>
 */
// pr_fmt(fmt) = KBUILD_MODNAME ": " fmt
// Kernel and netfilter dependencies are supplied by the surrounding build.

const IPV6_ADDR_UNICAST: i32 = 0x0001;
const IPV6_ADDR_LINKLOCAL: i32 = 0x0020;
const IPV6_ADDR_ANY: i32 = 0x0100;
const IPV6_FLOWINFO_MASK: u32 = 0x000f_ffff;
const RT6_LOOKUP_F_HAS_SADDR: i32 = 0x0001;
const RT6_LOOKUP_F_IFACE: i32 = 0x0002;
const RTF_REJECT: u32 = 0x0200;
const RTF_ANYCAST: u32 = 0x0002;
const RTF_LOCAL: u32 = 0x0400;
const XT_RPFILTER_VALID_MARK: u8 = 1 << 1;
const XT_RPFILTER_LOOSE: u8 = 1 << 0;
const XT_RPFILTER_ACCEPT_LOCAL: u8 = 1 << 2;
const XT_RPFILTER_INVERT: u8 = 1 << 3;
const XT_RPFILTER_OPTION_MASK: u8 = XT_RPFILTER_VALID_MARK | XT_RPFILTER_LOOSE | XT_RPFILTER_ACCEPT_LOCAL | XT_RPFILTER_INVERT;
const LOOPBACK_IFINDEX: i32 = 1;
const PACKET_LOOPBACK: u32 = 5;
const IFF_LOOPBACK: u32 = 0x8;
const NFPROTO_IPV6: u16 = 10;
const NF_INET_PRE_ROUTING: u32 = 0;

extern "C" {
    fn ipv6_addr_type(addr: *const in6_addr) -> i32;
    fn ipv6_hdr(skb: *const sk_buff) -> *mut ipv6hdr;
    fn l3mdev_master_ifindex_rcu(dev: *const net_device) -> i32;
    fn sock_net_uid(net: *mut net, sk: *mut core::ffi::c_void) -> u32;
    fn ip6_route_lookup(net: *mut net, fl6: *mut flowi6, skb: *const sk_buff, flags: i32) -> *mut rt6_info;
    fn ip6_rt_put(rt: *mut rt6_info);
    fn xt_in(par: *const xt_action_param) -> *const net_device;
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
}

#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub mark: u32, pub pkt_type: u32 }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub flags: u32 }
#[repr(C)] pub struct ipv6hdr { pub saddr: in6_addr, pub daddr: in6_addr, pub nexthdr: u8 }
#[repr(C)] pub struct dst_entry { pub error: i32 }
#[repr(C)] pub struct inet6_dev { pub dev: *const net_device }
#[repr(C)] pub struct rt6_info { pub dst: dst_entry, pub rt6i_flags: u32, pub rt6i_idev: *const inet6_dev }
#[repr(C)] pub struct flowi6 { pub flowi6_iif: i32, pub flowi6_l3mdev: i32, pub flowlabel: u32, pub flowi6_proto: u8, pub flowi6_uid: u32, pub daddr: in6_addr, pub saddr: in6_addr, pub flowi6_mark: u32, pub flowi6_oif: i32 }
#[repr(C)] pub struct xt_rpfilter_info { pub flags: u8 }
#[repr(C)] pub struct xt_action_param { pub matchinfo: *const core::ffi::c_void }
#[repr(C)] pub struct xt_mtchk_param { pub matchinfo: *const core::ffi::c_void, pub table: *const core::ffi::c_char }
#[repr(C)] pub struct xt_match { pub name: *const core::ffi::c_char, pub family: u16, pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>, pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>, pub matchsize: usize, pub hooks: u32, pub me: *mut core::ffi::c_void }

unsafe fn rpfilter_addr_unicast(addr: *const in6_addr) -> bool { ipv6_addr_type(addr) & IPV6_ADDR_UNICAST != 0 }
unsafe fn rpfilter_addr_linklocal(addr: *const in6_addr) -> bool { ipv6_addr_type(addr) & IPV6_ADDR_LINKLOCAL != 0 }

unsafe fn rpfilter_lookup_reverse6(net: *mut net, skb: *const sk_buff, dev: *const net_device, flags: u8) -> bool {
    let iph = ipv6_hdr(skb);
    let mut ret = false;
    let mut fl6 = flowi6 { flowi6_iif: LOOPBACK_IFINDEX, flowi6_l3mdev: l3mdev_master_ifindex_rcu(dev), flowlabel: (*(iph as *const u32)) & IPV6_FLOWINFO_MASK, flowi6_proto: (*iph).nexthdr, flowi6_uid: sock_net_uid(net, core::ptr::null_mut()), daddr: (*iph).saddr, saddr: in6_addr { s6_addr: [0; 16] }, flowi6_mark: 0, flowi6_oif: 0 };
    let mut lookup_flags;
    if rpfilter_addr_unicast(&(*iph).daddr) { fl6.saddr = (*iph).daddr; lookup_flags = RT6_LOOKUP_F_HAS_SADDR; } else { lookup_flags = 0; }
    fl6.flowi6_mark = if flags & XT_RPFILTER_VALID_MARK != 0 { (*skb).mark } else { 0 };
    if rpfilter_addr_linklocal(&(*iph).saddr) { lookup_flags |= RT6_LOOKUP_F_IFACE; fl6.flowi6_oif = (*dev).ifindex; } else if flags & XT_RPFILTER_LOOSE == 0 { fl6.flowi6_oif = (*dev).ifindex; }
    let rt = ip6_route_lookup(net, &mut fl6, skb, lookup_flags);
    if (*rt).dst.error != 0 { ip6_rt_put(rt); return ret; }
    if (*rt).rt6i_flags & (RTF_REJECT | RTF_ANYCAST) != 0 { ip6_rt_put(rt); return ret; }
    if (*rt).rt6i_flags & RTF_LOCAL != 0 { ret = flags & XT_RPFILTER_ACCEPT_LOCAL != 0; ip6_rt_put(rt); return ret; }
    if (*(*rt).rt6i_idev).dev == dev || l3mdev_master_ifindex_rcu((*rt).rt6i_idev).eq(&(*dev).ifindex) || flags & XT_RPFILTER_LOOSE != 0 { ret = true; }
    ip6_rt_put(rt); ret
}

unsafe fn rpfilter_is_loopback(skb: *const sk_buff, input: *const net_device) -> bool { (*skb).pkt_type == PACKET_LOOPBACK || (*input).flags & IFF_LOOPBACK != 0 }
unsafe fn rpfilter_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_rpfilter_info; let invert = (*info).flags & XT_RPFILTER_INVERT != 0;
    if rpfilter_is_loopback(skb, xt_in(par)) { return true ^ invert; }
    let iph = ipv6_hdr(skb); let saddrtype = ipv6_addr_type(&(*iph).saddr);
    if saddrtype == IPV6_ADDR_ANY { return true ^ invert; }
    rpfilter_lookup_reverse6(xt_net(par), skb, xt_in(par), (*info).flags) ^ invert
}

unsafe fn rpfilter_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_rpfilter_info;
    if (*info).flags & !XT_RPFILTER_OPTION_MASK != 0 { return -22; }
    let raw = b"raw\0"; let mangle = b"mangle\0";
    if strcmp((*par).table, mangle.as_ptr() as *const _) != 0 && strcmp((*par).table, raw.as_ptr() as *const _) != 0 { return -22; }
    0
}

static mut rpfilter_mt_reg: xt_match = xt_match { name: b"rpfilter\0".as_ptr() as *const _, family: NFPROTO_IPV6, checkentry: Some(rpfilter_check), r#match: Some(rpfilter_mt), matchsize: core::mem::size_of::<xt_rpfilter_info>(), hooks: 1 << NF_INET_PRE_ROUTING, me: core::ptr::null_mut() };
unsafe fn rpfilter_mt_init() -> i32 { xt_register_match(&mut rpfilter_mt_reg) }
unsafe fn rpfilter_mt_exit() { xt_unregister_match(&mut rpfilter_mt_reg); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
