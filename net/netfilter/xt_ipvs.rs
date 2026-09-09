// SPDX-License-Identifier: GPL-2.0-only
/*
 * xt_ipvs - kernel module to match IPVS connection properties
 *
 * Author: Hannes Eder <heder@google.com>
 */

// Dependency declarations and build-time configuration supplied by the kernel
// headers are intentionally left external to this translation.

use core::ffi::c_void;

extern "C" {
    fn ipv6_masked_addr_cmp(kaddr: *const in6_addr, mask: *const in6_addr,
                            uaddr: *const in6_addr) -> i32;
    fn net_ipvs(net: *mut net) -> *mut netns_ipvs;
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_family(par: *const xt_action_param) -> u8;
    fn ip_vs_fill_iph_skb(family: u8, skb: *const sk_buff, inverse: bool,
                          iph: *mut ip_vs_iphdr);
    fn ip_vs_proto_get(protocol: u8) -> *mut ip_vs_protocol;
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn;
    fn __ip_vs_conn_put(cp: *mut ip_vs_conn);
    fn xt_register_match(mt: *mut xt_match) -> i32;
    fn xt_unregister_match(mt: *mut xt_match);
}

#[repr(C)]
pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)]
pub union nf_inet_addr { pub ip: u32, pub in6: in6_addr }
#[repr(C)] pub struct net;
#[repr(C)] pub struct netns_ipvs;
#[repr(C)] pub struct sk_buff { pub ipvs_property: bool }
#[repr(C)] pub struct xt_action_param { pub matchinfo: *const xt_ipvs_mtinfo }
#[repr(C)] pub struct ip_vs_iphdr { pub protocol: u8 }
#[repr(C)] pub struct ip_vs_protocol {
    pub conn_out_get: unsafe extern "C" fn(*mut netns_ipvs, u8, *const sk_buff,
                                             *const ip_vs_iphdr) -> *mut ip_vs_conn,
}
#[repr(C)] pub struct ip_vs_conn {
    pub vport: u16,
    pub control: *mut ip_vs_conn,
    pub flags: u32,
    pub vaddr: nf_inet_addr,
}
#[repr(C)] pub struct nf_conn;
#[repr(C)] pub struct xt_mtchk_param { pub family: u8 }
#[repr(C)] pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub matchsize: usize,
    pub me: *mut c_void,
}
#[repr(C)] pub struct xt_ipvs_mtinfo {
    pub bitmask: u8,
    pub invert: u8,
    pub l4proto: u8,
    pub vport: u16,
    pub vportctl: u16,
    pub fwd_method: u32,
    pub vaddr: nf_inet_addr,
    pub vmask: nf_inet_addr,
}
#[repr(C)] pub enum ip_conntrack_info { IP_CT_NEW = 0, IP_CT_IS_REPLY = 1 }

const NFPROTO_UNSPEC: u16 = 0;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const XT_IPVS_IPVS_PROPERTY: u8 = 1;
const XT_IPVS_PROTO: u8 = 2;
const XT_IPVS_VPORT: u8 = 4;
const XT_IPVS_VPORTCTL: u8 = 8;
const XT_IPVS_DIR: u8 = 16;
const XT_IPVS_METHOD: u8 = 32;
const XT_IPVS_VADDR: u8 = 64;
const XT_ALIGN: usize = 1;
const IP_VS_CONN_F_FWD_MASK: u32 = 0xffff_ffff;

unsafe fn ipvs_mt_addrcmp(kaddr: *const nf_inet_addr, uaddr: *const nf_inet_addr,
                           umask: *const nf_inet_addr, l3proto: u32) -> bool {
    if l3proto == NFPROTO_IPV4 as u32 {
        ((*kaddr).ip ^ (*uaddr).ip) & (*umask).ip == 0
    } else if l3proto == NFPROTO_IPV6 as u32 {
        ipv6_masked_addr_cmp(&raw const (*kaddr).in6, &raw const (*umask).in6,
                             &raw const (*uaddr).in6) == 0
    } else { false }
}

unsafe extern "C" fn ipvs_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let data = (*par).matchinfo;
    let ipvs = net_ipvs(xt_net(par));
    let family = xt_family(par);
    let mut iph = ip_vs_iphdr { protocol: 0 };
    let mut match_ = true;

    if (*data).bitmask == XT_IPVS_IPVS_PROPERTY {
        match_ = (*skb).ipvs_property ^ ((*data).invert & XT_IPVS_IPVS_PROPERTY != 0);
        return match_;
    }
    if !(*skb).ipvs_property { return false; }
    ip_vs_fill_iph_skb(family, skb, true, &mut iph);
    if (*data).bitmask & XT_IPVS_PROTO != 0 &&
       ((iph.protocol == (*data).l4proto) ^ !((*data).invert & XT_IPVS_PROTO != 0)) { return false; }
    let pp = ip_vs_proto_get(iph.protocol);
    if pp.is_null() { return false; }
    let cp = ((*pp).conn_out_get)(ipvs, family, skb, &iph);
    if cp.is_null() { return false; }
    if (*data).bitmask & XT_IPVS_VPORT != 0 &&
       (((*cp).vport == (*data).vport) ^ !((*data).invert & XT_IPVS_VPORT != 0)) { __ip_vs_conn_put(cp); return false; }
    if (*data).bitmask & XT_IPVS_VPORTCTL != 0 &&
       (((!(*cp).control.is_null() && (*(*cp).control).vport == (*data).vportctl)) ^ !((*data).invert & XT_IPVS_VPORTCTL != 0)) { __ip_vs_conn_put(cp); return false; }
    if (*data).bitmask & XT_IPVS_DIR != 0 {
        let mut ctinfo = ip_conntrack_info::IP_CT_NEW;
        let ct = nf_ct_get(skb, &mut ctinfo);
        if ct.is_null() || ((ctinfo as i32 >= ip_conntrack_info::IP_CT_IS_REPLY as i32) ^ ((*data).invert & XT_IPVS_DIR != 0)) { __ip_vs_conn_put(cp); return false; }
    }
    if (*data).bitmask & XT_IPVS_METHOD != 0 &&
       (((*cp).flags & IP_VS_CONN_F_FWD_MASK) == (*data).fwd_method) ^ !((*data).invert & XT_IPVS_METHOD != 0) { __ip_vs_conn_put(cp); return false; }
    if (*data).bitmask & XT_IPVS_VADDR != 0 &&
       (ipvs_mt_addrcmp(&raw const (*cp).vaddr, &raw const (*data).vaddr, &raw const (*data).vmask, family as u32) ^ !((*data).invert & XT_IPVS_VADDR != 0)) { __ip_vs_conn_put(cp); return false; }
    __ip_vs_conn_put(cp);
    match_
}

unsafe extern "C" fn ipvs_mt_check(par: *const xt_mtchk_param) -> i32 {
    if (*par).family != NFPROTO_IPV4 && (*par).family != NFPROTO_IPV6 { return -22; }
    0
}

static mut xt_ipvs_mt_reg: xt_match = xt_match {
    name: b"ipvs\0".as_ptr(), revision: 0, family: NFPROTO_UNSPEC,
    match_: Some(ipvs_mt), checkentry: Some(ipvs_mt_check),
    matchsize: XT_ALIGN, me: core::ptr::null_mut(),
};

pub unsafe extern "C" fn ipvs_mt_init() -> i32 { xt_register_match(&raw mut xt_ipvs_mt_reg) }
pub unsafe extern "C" fn ipvs_mt_exit() { xt_unregister_match(&raw mut xt_ipvs_mt_reg); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
