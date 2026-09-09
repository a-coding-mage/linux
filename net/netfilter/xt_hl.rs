// SPDX-License-Identifier: GPL-2.0-only
/*
 * IP tables module for matching the value of the TTL
 * (C) 2000,2001 by Harald Welte <laforge@netfilter.org>
 *
 * Hop Limit matching module
 * (C) 2001-2002 Maciej Soltysiak <solt@dns.toxicfilms.tv>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the Linux kernel and netfilter headers are external.

extern "C" {
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct ipt_ttl_info {
    pub mode: u8,
    pub ttl: u8,
}

#[repr(C)]
pub struct ip6t_hl_info {
    pub mode: u8,
    pub hop_limit: u8,
}

#[repr(C)]
pub struct ipv4_header {
    _private: [u8; 8],
    pub ttl: u8,
}

#[repr(C)]
pub struct ipv6hdr {
    _private: [u8; 7],
    pub hop_limit: u8,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const core::ffi::c_char,
    pub revision: u8,
    pub family: u8,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub match_fn: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub me: *mut core::ffi::c_void,
}

const EINVAL: i32 = 22;
const IPT_TTL_EQ: u8 = 0;
const IPT_TTL_NE: u8 = 1;
const IPT_TTL_LT: u8 = 2;
const IPT_TTL_GT: u8 = 3;
const IP6T_HL_EQ: u8 = 0;
const IP6T_HL_NE: u8 = 1;
const IP6T_HL_LT: u8 = 2;
const IP6T_HL_GT: u8 = 3;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;

unsafe fn ip_hdr(skb: *const sk_buff) -> *const ipv4_header {
    skb as *const ipv4_header
}

unsafe fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr {
    skb as *const ipv6hdr
}

unsafe extern "C" fn ttl_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ipt_ttl_info;

    if (*info).mode > IPT_TTL_GT {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn ttl_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const ipt_ttl_info;
    let ttl = (*ip_hdr(skb)).ttl;

    match (*info).mode {
        IPT_TTL_EQ => ttl == (*info).ttl,
        IPT_TTL_NE => ttl != (*info).ttl,
        IPT_TTL_LT => ttl < (*info).ttl,
        IPT_TTL_GT => ttl > (*info).ttl,
        _ => false,
    }
}

unsafe extern "C" fn hl_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ip6t_hl_info;

    if (*info).mode > IP6T_HL_GT {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn hl_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const ip6t_hl_info;
    let ip6h = ipv6_hdr(skb);

    match (*info).mode {
        IP6T_HL_EQ => (*ip6h).hop_limit == (*info).hop_limit,
        IP6T_HL_NE => (*ip6h).hop_limit != (*info).hop_limit,
        IP6T_HL_LT => (*ip6h).hop_limit < (*info).hop_limit,
        IP6T_HL_GT => (*ip6h).hop_limit > (*info).hop_limit,
        _ => false,
    }
}

static mut HL_MT_REG: [xt_match; 2] = [
    xt_match {
        name: b"ttl\0".as_ptr() as *const core::ffi::c_char,
        revision: 0,
        family: NFPROTO_IPV4,
        checkentry: Some(ttl_mt_check),
        match_fn: Some(ttl_mt),
        matchsize: core::mem::size_of::<ipt_ttl_info>(),
        me: core::ptr::null_mut(),
    },
    xt_match {
        name: b"hl\0".as_ptr() as *const core::ffi::c_char,
        revision: 0,
        family: NFPROTO_IPV6,
        checkentry: Some(hl_mt6_check),
        match_fn: Some(hl_mt6),
        matchsize: core::mem::size_of::<ip6t_hl_info>(),
        me: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn hl_mt_init() -> i32 {
    xt_register_matches(HL_MT_REG.as_mut_ptr(), HL_MT_REG.len())
}

unsafe extern "C" fn hl_mt_exit() {
    xt_unregister_matches(HL_MT_REG.as_mut_ptr(), HL_MT_REG.len());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
