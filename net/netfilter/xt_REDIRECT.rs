// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 * Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
 *
 * Based on Rusty Russell's IPv4 REDIRECT target. Development of IPv6
 * NAT funded by Astaro.
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe extern "C" {
    fn nf_nat_redirect_ipv6(
        skb: *mut sk_buff,
        targinfo: *const core::ffi::c_void,
        hooknum: u8,
    ) -> u32;
    fn nf_ct_netns_get(net: *mut net, family: u8) -> i32;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn pr_info_ratelimited(fmt: *const core::ffi::c_char, ...);
    fn nf_nat_redirect_ipv4(
        skb: *mut sk_buff,
        range: *const nf_nat_range2,
        hooknum: u8,
    ) -> u32;
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> i32;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;
}

unsafe fn xt_hooknum(par: *const xt_action_param) -> u8;

const NF_NAT_RANGE_MAP_IPS: u32 = 1 << 0;
const NFPROTO_IPV6: u8 = 10;
const NFPROTO_IPV4: u8 = 2;
const NF_INET_PRE_ROUTING: u32 = 0;
const NF_INET_LOCAL_OUT: u32 = 3;

#[repr(C)]
struct nf_nat_range2 {
    flags: u32,
    min_proto: nf_nat_proto_range,
    max_proto: nf_nat_proto_range,
}

#[repr(C)]
struct nf_nat_ipv4_multi_range_compat {
    rangesize: u32,
    range: [nf_nat_range_compat; 1],
}

#[repr(C)]
struct nf_nat_range_compat {
    flags: u32,
    min: nf_nat_proto_range,
    max: nf_nat_proto_range,
}

#[repr(C)]
struct nf_nat_proto_range {
    all: [u8; 4],
}

#[repr(C)]
struct xt_action_param {
    targinfo: *const core::ffi::c_void,
}

#[repr(C)]
struct xt_tgchk_param {
    targinfo: *const core::ffi::c_void,
    net: *mut net,
    family: u8,
}

#[repr(C)]
struct xt_tgdtor_param {
    net: *mut net,
    family: u8,
}

#[repr(C)]
struct sk_buff;
#[repr(C)]
struct net;
#[repr(C)]
struct module;

#[repr(C)]
struct xt_target {
    name: [core::ffi::c_char; 29],
    revision: u8,
    family: u8,
    table: *const core::ffi::c_char,
    checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>,
    target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    targetsize: usize,
    hooks: u32,
    me: *mut module,
}

unsafe fn redirect_tg6(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    nf_nat_redirect_ipv6((*skb,).0, (*par).targinfo, xt_hooknum(par))
}

unsafe fn redirect_tg6_checkentry(par: *const xt_tgchk_param) -> i32 {
    let range = (*par).targinfo as *const nf_nat_range2;

    if (*range).flags & NF_NAT_RANGE_MAP_IPS != 0 {
        return -22;
    }

    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn redirect_tg_destroy(par: *const xt_tgdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

unsafe fn redirect_tg4_check(par: *const xt_tgchk_param) -> i32 {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;

    if (*mr).range[0].flags & NF_NAT_RANGE_MAP_IPS != 0 {
        pr_info_ratelimited(c"bad MAP_IPS.\n".as_ptr());
        return -22;
    }
    if (*mr).rangesize != 1 {
        pr_info_ratelimited(c"bad rangesize %u.\n".as_ptr(), (*mr).rangesize);
        return -22;
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn redirect_tg4(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;
    let range = nf_nat_range2 {
        flags: (*mr).range[0].flags,
        min_proto: (*mr).range[0].min,
        max_proto: (*mr).range[0].max,
    };

    nf_nat_redirect_ipv4(skb, &range, xt_hooknum(par))
}

static mut redirect_tg_reg: [xt_target; 2] = [
    xt_target {
        name: *b"REDIRECT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        family: NFPROTO_IPV6,
        revision: 0,
        table: c"nat".as_ptr(),
        checkentry: Some(redirect_tg6_checkentry),
        destroy: Some(redirect_tg_destroy),
        target: Some(redirect_tg6),
        targetsize: core::mem::size_of::<nf_nat_range2>(),
        hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT),
        me: THIS_MODULE,
    },
    xt_target {
        name: *b"REDIRECT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        family: NFPROTO_IPV4,
        revision: 0,
        table: c"nat".as_ptr(),
        target: Some(redirect_tg4),
        checkentry: Some(redirect_tg4_check),
        destroy: Some(redirect_tg_destroy),
        targetsize: core::mem::size_of::<nf_nat_ipv4_multi_range_compat>(),
        hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT),
        me: THIS_MODULE,
    },
];

unsafe fn redirect_tg_init() -> i32 {
    xt_register_targets(redirect_tg_reg.as_mut_ptr(), redirect_tg_reg.len())
}

unsafe fn redirect_tg_exit() {
    xt_unregister_targets(redirect_tg_reg.as_mut_ptr(), redirect_tg_reg.len());
}

// module_init(redirect_tg_init);
// module_exit(redirect_tg_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("Xtables: Connection redirection to localhost");
// MODULE_ALIAS("ip6t_REDIRECT");
// MODULE_ALIAS("ipt_REDIRECT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
