// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match packet length. */
/* (C) 1999-2001 James Morris <jmorros@intercode.com.au>
 */

// C dependencies supplied by the kernel and netfilter headers:
// linux/module.h, linux/skbuff.h, linux/ipv6.h, net/ip.h,
// linux/netfilter/xt_length.h, and linux/netfilter/x_tables.h

use crate::linux::netfilter::x_tables::{xt_action_param, xt_match};
use crate::linux::netfilter::xt_length::xt_length_info;
use crate::linux::skbuff::sk_buff;

extern "C" {
    fn skb_ip_totlen(skb: *const sk_buff) -> u32;
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
}

// MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");
// MODULE_DESCRIPTION("Xtables: Packet length (Layer3,4,5) match");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_length");
// MODULE_ALIAS("ip6t_length");

unsafe fn length_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_length_info;
    let pktlen: u32 = skb_ip_totlen(skb);

    (pktlen >= (*info).min && pktlen <= (*info).max) ^ (*info).invert
}

unsafe fn length_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_length_info;
    let pktlen: u32 = (*skb).len;

    (pktlen >= (*info).min && pktlen <= (*info).max) ^ (*info).invert
}

static mut length_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"length\0",
        family: crate::linux::netfilter::nfproto::NFPROTO_IPV4,
        match_: Some(length_mt),
        matchsize: core::mem::size_of::<xt_length_info>(),
        me: crate::THIS_MODULE,
        ..xt_match::DEFAULT
    },
    xt_match {
        name: *b"length\0",
        family: crate::linux::netfilter::nfproto::NFPROTO_IPV6,
        match_: Some(length_mt6),
        matchsize: core::mem::size_of::<xt_length_info>(),
        me: crate::THIS_MODULE,
        ..xt_match::DEFAULT
    },
];

unsafe fn length_mt_init() -> i32 {
    xt_register_matches(
        length_mt_reg.as_mut_ptr(),
        length_mt_reg.len(),
    )
}

unsafe fn length_mt_exit() {
    xt_unregister_matches(length_mt_reg.as_mut_ptr(), length_mt_reg.len());
}

// module_init(length_mt_init);
// module_exit(length_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
