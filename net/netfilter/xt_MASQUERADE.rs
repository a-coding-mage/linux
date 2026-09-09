// SPDX-License-Identifier: GPL-2.0-only
/* Masquerade. Simple mapping which alters range to a local IP address
 * (depending on route).
 *
 * (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 */

// C dependencies supplied by the surrounding kernel translation.
// Build-time CONFIG_IPV6 conditionals are preserved below.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: automatic-address SNAT");

/* FIXME: Multiple targets. --RR */
unsafe fn masquerade_tg_check(par: *const xt_tgchk_param) -> c_int {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;

    if (*mr).range[0].flags & NF_NAT_RANGE_MAP_IPS != 0 {
        pr_info_ratelimited!("bad MAP_IPS.\n");
        return -EINVAL;
    }
    if (*mr).rangesize != 1 {
        pr_info_ratelimited!("bad rangesize %u\n", (*mr).rangesize);
        return -EINVAL;
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn masquerade_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;

    range.flags = (*mr).range[0].flags;
    range.min_proto = (*mr).range[0].min;
    range.max_proto = (*mr).range[0].max;

    nf_nat_masquerade_ipv4(skb, xt_hooknum(par), &range, xt_out(par))
}

unsafe fn masquerade_tg_destroy(par: *const xt_tgdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

#[cfg(CONFIG_IPV6)]
unsafe fn masquerade_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    nf_nat_masquerade_ipv6(skb, (*par).targinfo, xt_out(par))
}

#[cfg(CONFIG_IPV6)]
unsafe fn masquerade_tg6_checkentry(par: *const xt_tgchk_param) -> c_int {
    let range = (*par).targinfo as *const nf_nat_range2;

    if (*range).flags & NF_NAT_RANGE_MAP_IPS != 0 {
        return -EINVAL;
    }

    nf_ct_netns_get((*par).net, (*par).family)
}

static mut masquerade_tg_reg: [xt_target; 2] = [
    #[cfg(CONFIG_IPV6)]
    xt_target {
        name: c_str!("MASQUERADE"),
        family: NFPROTO_IPV6,
        target: Some(masquerade_tg6),
        targetsize: core::mem::size_of::<nf_nat_range>(),
        table: c_str!("nat"),
        hooks: 1 << NF_INET_POST_ROUTING,
        checkentry: Some(masquerade_tg6_checkentry),
        destroy: Some(masquerade_tg_destroy),
        me: THIS_MODULE,
    },
    xt_target {
        name: c_str!("MASQUERADE"),
        family: NFPROTO_IPV4,
        target: Some(masquerade_tg),
        targetsize: core::mem::size_of::<nf_nat_ipv4_multi_range_compat>(),
        table: c_str!("nat"),
        hooks: 1 << NF_INET_POST_ROUTING,
        checkentry: Some(masquerade_tg_check),
        destroy: Some(masquerade_tg_destroy),
        me: THIS_MODULE,
    },
];

unsafe fn masquerade_tg_init() -> c_int {
    let mut ret = xt_register_targets(
        masquerade_tg_reg.as_mut_ptr(),
        masquerade_tg_reg.len(),
    );
    if ret != 0 {
        return ret;
    }

    ret = nf_nat_masquerade_inet_register_notifiers();
    if ret != 0 {
        xt_unregister_targets(masquerade_tg_reg.as_mut_ptr(), masquerade_tg_reg.len());
        return ret;
    }

    ret
}

unsafe fn masquerade_tg_exit() {
    xt_unregister_targets(masquerade_tg_reg.as_mut_ptr(), masquerade_tg_reg.len());
    nf_nat_masquerade_inet_unregister_notifiers();
}

// module_init(masquerade_tg_init);
// module_exit(masquerade_tg_exit);
#[cfg(CONFIG_IPV6)]
// MODULE_ALIAS("ip6t_MASQUERADE");
// MODULE_ALIAS("ipt_MASQUERADE");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
