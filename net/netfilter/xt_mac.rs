// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match MAC address parameters. */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Kernel dependencies supplied by the surrounding build.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: MAC address match");
// MODULE_ALIAS("ipt_mac");
// MODULE_ALIAS("ip6t_mac");

unsafe fn mac_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = unsafe { (*par).matchinfo as *const xt_mac_info };
    let mut ret: bool;

    if unsafe { (*skb).dev.is_null() }
        || unsafe { (*(*skb).dev).type_ != ARPHRD_ETHER }
    {
        return false;
    }
    if !unsafe { skb_mac_header_was_set(skb) }
        || unsafe { skb_mac_header_len(skb) } < ETH_HLEN
    {
        return false;
    }
    ret = unsafe {
        ether_addr_equal(
            (*eth_hdr(skb)).h_source.as_ptr(),
            (*info).srcaddr.as_ptr(),
        )
    };
    ret ^= unsafe { (*info).invert };
    ret
}

#[repr(C)]
static mut mac_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"mac\0",
        family: NFPROTO_IPV4,
        match_: Some(mac_mt),
        matchsize: core::mem::size_of::<xt_mac_info>(),
        hooks: (1 << NF_INET_PRE_ROUTING)
            | (1 << NF_INET_LOCAL_IN)
            | (1 << NF_INET_FORWARD),
        me: THIS_MODULE,
    },
    xt_match {
        name: *b"mac\0",
        family: NFPROTO_IPV6,
        match_: Some(mac_mt),
        matchsize: core::mem::size_of::<xt_mac_info>(),
        hooks: (1 << NF_INET_PRE_ROUTING)
            | (1 << NF_INET_LOCAL_IN)
            | (1 << NF_INET_FORWARD),
        me: THIS_MODULE,
    },
];

unsafe fn mac_mt_init() -> i32 {
    xt_register_matches(mac_mt_reg.as_mut_ptr(), mac_mt_reg.len())
}

unsafe fn mac_mt_exit() {
    xt_unregister_matches(mac_mt_reg.as_mut_ptr(), mac_mt_reg.len());
}

// module_init(mac_mt_init);
// module_exit(mac_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
