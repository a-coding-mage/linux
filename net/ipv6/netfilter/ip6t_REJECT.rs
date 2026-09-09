// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IP6 tables REJECT target module
 * Linux INET6 implementation
 *
 * Copyright (C)2003 USAGI/WIDE Project
 *
 * Authors:
 *\tYasuyuki Kozakai\t<yasuyuki.kozakai@toshiba.co.jp>
 *
 * Copyright (c) 2005-2007 Patrick McHardy <kaber@trash.net>
 *
 * Based on net/ipv4/netfilter/ipt_REJECT.c
 */

// C dependencies:
// linux/gfp.h, linux/module.h, linux/skbuff.h, linux/icmpv6.h,
// linux/netdevice.h, net/icmp.h, net/flow.h,
// linux/netfilter/x_tables.h, linux/netfilter_ipv6/ip6_tables.h,
// linux/netfilter_ipv6/ip6t_REJECT.h, net/netfilter/ipv6/nf_reject.h

extern "C" {
    fn nf_send_unreach6(net: *mut net, skb: *mut sk_buff, code: i32, hooknum: u8);
    fn nf_send_reset6(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        hooknum: u8,
    );
    fn xt_register_target(target: *mut xt_target) -> i32;
    fn xt_unregister_target(target: *mut xt_target);
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_hooknum(par: *const xt_action_param) -> u8;
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    pub targinfo: *const core::ffi::c_void,
    pub state: *const xt_nft_action_param_state,
}

#[repr(C)]
pub struct xt_nft_action_param_state {
    pub net: *mut net,
    pub sk: *mut sock,
}

#[repr(C)]
pub struct xt_tgchk_param {
    pub targinfo: *const core::ffi::c_void,
    pub entryinfo: *const ip6t_entry,
}

#[repr(C)]
pub struct ip6t_reject_info {
    pub with: u8,
}

#[repr(C)]
pub struct ip6t_entry_ipv6 {
    pub flags: u8,
    pub proto: u8,
    pub invflags: u8,
}

#[repr(C)]
pub struct ip6t_entry {
    pub ipv6: ip6t_entry_ipv6,
}

#[repr(C)]
pub struct xt_target {
    pub name: *const u8,
    pub family: u16,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    pub targetsize: usize,
    pub table: *const u8,
    pub hooks: u32,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    pub me: *mut core::ffi::c_void,
}

const IP6T_ICMP6_NO_ROUTE: u8 = 0;
const IP6T_ICMP6_ADM_PROHIBITED: u8 = 1;
const IP6T_ICMP6_NOT_NEIGHBOUR: u8 = 2;
const IP6T_ICMP6_ADDR_UNREACH: u8 = 3;
const IP6T_ICMP6_PORT_UNREACH: u8 = 4;
const IP6T_ICMP6_ECHOREPLY: u8 = 5;
const IP6T_TCP_RESET: u8 = 6;
const IP6T_ICMP6_POLICY_FAIL: u8 = 7;
const IP6T_ICMP6_REJECT_ROUTE: u8 = 8;

const ICMPV6_NOROUTE: i32 = 0;
const ICMPV6_ADM_PROHIBITED: i32 = 1;
const ICMPV6_NOT_NEIGHBOUR: i32 = 2;
const ICMPV6_ADDR_UNREACH: i32 = 3;
const ICMPV6_PORT_UNREACH: i32 = 4;
const ICMPV6_POLICY_FAIL: i32 = 5;
const ICMPV6_REJECT_ROUTE: i32 = 6;
const IPPROTO_TCP: u8 = 6;
const IP6T_F_PROTO: u8 = 1;
const XT_INV_PROTO: u8 = 2;
const NF_DROP: u32 = 0;
const NFPROTO_IPV6: u16 = 10;
const NF_INET_LOCAL_IN: u32 = 1;
const NF_INET_FORWARD: u32 = 2;
const NF_INET_LOCAL_OUT: u32 = 3;

unsafe extern "C" fn reject_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let reject = (*par).targinfo as *const ip6t_reject_info;
    let net = xt_net(par);

    match (*reject).with {
        IP6T_ICMP6_NO_ROUTE => nf_send_unreach6(net, skb, ICMPV6_NOROUTE, xt_hooknum(par)),
        IP6T_ICMP6_ADM_PROHIBITED => {
            nf_send_unreach6(net, skb, ICMPV6_ADM_PROHIBITED, xt_hooknum(par))
        }
        IP6T_ICMP6_NOT_NEIGHBOUR => {
            nf_send_unreach6(net, skb, ICMPV6_NOT_NEIGHBOUR, xt_hooknum(par))
        }
        IP6T_ICMP6_ADDR_UNREACH => {
            nf_send_unreach6(net, skb, ICMPV6_ADDR_UNREACH, xt_hooknum(par))
        }
        IP6T_ICMP6_PORT_UNREACH => {
            nf_send_unreach6(net, skb, ICMPV6_PORT_UNREACH, xt_hooknum(par))
        }
        IP6T_ICMP6_ECHOREPLY => {}
        IP6T_TCP_RESET => nf_send_reset6(net, (*(*par).state).sk, skb, xt_hooknum(par)),
        IP6T_ICMP6_POLICY_FAIL => {
            nf_send_unreach6(net, skb, ICMPV6_POLICY_FAIL, xt_hooknum(par))
        }
        IP6T_ICMP6_REJECT_ROUTE => {
            nf_send_unreach6(net, skb, ICMPV6_REJECT_ROUTE, xt_hooknum(par))
        }
        _ => {}
    }

    NF_DROP
}

unsafe extern "C" fn reject_tg6_check(par: *const xt_tgchk_param) -> i32 {
    let rejinfo = (*par).targinfo as *const ip6t_reject_info;
    let e = (*par).entryinfo;

    if (*rejinfo).with == IP6T_ICMP6_ECHOREPLY {
        // pr_info_ratelimited("ECHOREPLY is not supported\\n");
        return -22;
    } else if (*rejinfo).with == IP6T_TCP_RESET {
        if ((*e).ipv6.flags & IP6T_F_PROTO) == 0
            || (*e).ipv6.proto != IPPROTO_TCP
            || ((*e).ipv6.invflags & XT_INV_PROTO) != 0
        {
            // pr_info_ratelimited("TCP_RESET illegal for non-tcp\\n");
            return -22;
        }
    }
    0
}

static mut reject_tg6_reg: xt_target = xt_target {
    name: b"REJECT\\0".as_ptr(),
    family: NFPROTO_IPV6,
    target: Some(reject_tg6),
    targetsize: core::mem::size_of::<ip6t_reject_info>(),
    table: b"filter\\0".as_ptr(),
    hooks: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD) | (1 << NF_INET_LOCAL_OUT),
    checkentry: Some(reject_tg6_check),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn reject_tg6_init() -> i32 {
    xt_register_target(&raw mut reject_tg6_reg)
}

unsafe extern "C" fn reject_tg6_exit() {
    xt_unregister_target(&raw mut reject_tg6_reg);
}

// module_init(reject_tg6_init);
// module_exit(reject_tg6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
