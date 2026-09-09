// SPDX-License-Identifier: GPL-2.0-only
/* IP tables module for matching the value of the IPv4/IPv6 DSCP field
 *
 * (C) 2002 by Harald Welte <laforge@netfilter.org>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_int, c_void};

extern "C" {
    static THIS_MODULE: c_void;
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> c_int;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn ipv4_get_dsfield(iph: *const iphdr) -> u8;
    fn ipv6_get_dsfield(iph: *const ipv6hdr) -> u8;
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    pub tos: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipv6hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
    pub family: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const c_void,
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_dscp_info {
    pub dscp: u8,
    pub invert: u8,
}

#[repr(C)]
pub struct xt_tos_match_info {
    pub tos_value: u8,
    pub tos_mask: u8,
    pub invert: u8,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> c_int>,
    pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub me: *const c_void,
}

const XT_DSCP_SHIFT: u32 = 2;
const XT_DSCP_MAX: u8 = 0x3f;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;

unsafe extern "C" fn xt_family(par: *const xt_action_param) -> u8 {
    (*par).family
}

unsafe extern "C" fn dscp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_dscp_info;
    let dscp = ipv4_get_dsfield(ip_hdr(skb)) >> XT_DSCP_SHIFT;

    (dscp == (*info).dscp) ^ ((*info).invert != 0)
}

unsafe extern "C" fn dscp_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_dscp_info;
    let dscp = ipv6_get_dsfield(ipv6_hdr(skb)) >> XT_DSCP_SHIFT;

    (dscp == (*info).dscp) ^ ((*info).invert != 0)
}

unsafe extern "C" fn dscp_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const xt_dscp_info;

    if (*info).dscp > XT_DSCP_MAX {
        return -33; // -EDOM
    }

    0
}

unsafe extern "C" fn tos_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const xt_tos_match_info;

    if (*info).invert > 1 {
        return -22; // -EINVAL
    }

    0
}

unsafe extern "C" fn tos_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_tos_match_info;

    if xt_family(par) == NFPROTO_IPV4 {
        ((*ip_hdr(skb)).tos & (*info).tos_mask == (*info).tos_value)
            ^ ((*info).invert != 0)
    } else {
        (ipv6_get_dsfield(ipv6_hdr(skb)) & (*info).tos_mask == (*info).tos_value)
            ^ ((*info).invert != 0)
    }
}

static mut dscp_mt_reg: [xt_match; 4] = [
    xt_match {
        name: b"dscp\0".as_ptr(), revision: 0, family: NFPROTO_IPV4 as u16,
        checkentry: Some(dscp_mt_check), r#match: Some(dscp_mt),
        matchsize: core::mem::size_of::<xt_dscp_info>(), me: unsafe { &THIS_MODULE },
    },
    xt_match {
        name: b"dscp\0".as_ptr(), revision: 0, family: NFPROTO_IPV6 as u16,
        checkentry: Some(dscp_mt_check), r#match: Some(dscp_mt6),
        matchsize: core::mem::size_of::<xt_dscp_info>(), me: unsafe { &THIS_MODULE },
    },
    xt_match {
        name: b"tos\0".as_ptr(), revision: 1, family: NFPROTO_IPV4 as u16,
        checkentry: Some(tos_mt_check), r#match: Some(tos_mt),
        matchsize: core::mem::size_of::<xt_tos_match_info>(), me: unsafe { &THIS_MODULE },
    },
    xt_match {
        name: b"tos\0".as_ptr(), revision: 1, family: NFPROTO_IPV6 as u16,
        checkentry: Some(tos_mt_check), r#match: Some(tos_mt),
        matchsize: core::mem::size_of::<xt_tos_match_info>(), me: unsafe { &THIS_MODULE },
    },
];

unsafe extern "C" fn dscp_mt_init() -> c_int {
    xt_register_matches(dscp_mt_reg.as_mut_ptr(), dscp_mt_reg.len())
}

unsafe extern "C" fn dscp_mt_exit() {
    xt_unregister_matches(dscp_mt_reg.as_mut_ptr(), dscp_mt_reg.len());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
