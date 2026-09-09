// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2000-2001 Svenning Soerensen <svenning@post5.tele.dk>
 * Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the Linux kernel and netfilter environment.

unsafe fn netmap_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let range = (*par).targinfo as *const nf_nat_range2;
    let mut newrange: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct: *mut nf_conn = nf_ct_get(skb, &mut ctinfo);
    let mut new_addr: nf_inet_addr = core::mem::zeroed();
    let mut netmask: nf_inet_addr = core::mem::zeroed();

    for i in 0..core::mem::size_of_val(&(*range).min_addr.ip6) /
        core::mem::size_of_val(&(*range).min_addr.ip6[0])
    {
        netmask.ip6[i] = !((*range).min_addr.ip6[i] ^ (*range).max_addr.ip6[i]);
    }

    if xt_hooknum(par) == NF_INET_PRE_ROUTING || xt_hooknum(par) == NF_INET_LOCAL_OUT {
        new_addr.in6 = (*ipv6_hdr(skb)).daddr;
    } else {
        new_addr.in6 = (*ipv6_hdr(skb)).saddr;
    }

    for i in 0..core::mem::size_of_val(&new_addr.ip6) /
        core::mem::size_of_val(&new_addr.ip6[0])
    {
        new_addr.ip6[i] &= !netmask.ip6[i];
        new_addr.ip6[i] |= (*range).min_addr.ip6[i] & netmask.ip6[i];
    }

    newrange.flags = (*range).flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr = new_addr;
    newrange.max_addr = new_addr;
    newrange.min_proto = (*range).min_proto;
    newrange.max_proto = (*range).max_proto;

    nf_nat_setup_info(ct, &mut newrange, HOOK2MANIP(xt_hooknum(par)))
}

unsafe fn netmap_tg6_checkentry(par: *const xt_tgchk_param) -> c_int {
    let range = (*par).targinfo as *const nf_nat_range2;

    if (*range).flags & NF_NAT_RANGE_MAP_IPS == 0 {
        return -EINVAL;
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn netmap_tg_destroy(par: *const xt_tgdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

unsafe fn netmap_tg4(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct: *mut nf_conn;
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;
    let mut newrange: nf_nat_range2 = core::mem::zeroed();
    let mut new_ip: __be32;
    let netmask: __be32;

    WARN_ON(xt_hooknum(par) != NF_INET_PRE_ROUTING &&
        xt_hooknum(par) != NF_INET_POST_ROUTING &&
        xt_hooknum(par) != NF_INET_LOCAL_OUT &&
        xt_hooknum(par) != NF_INET_LOCAL_IN);
    ct = nf_ct_get(skb, &mut ctinfo);

    netmask = !((*mr).range[0].min_ip ^ (*mr).range[0].max_ip);

    if xt_hooknum(par) == NF_INET_PRE_ROUTING || xt_hooknum(par) == NF_INET_LOCAL_OUT {
        new_ip = (*ip_hdr(skb)).daddr & !netmask;
    } else {
        new_ip = (*ip_hdr(skb)).saddr & !netmask;
    }
    new_ip |= (*mr).range[0].min_ip & netmask;

    newrange.min_addr = core::mem::zeroed();
    newrange.max_addr = core::mem::zeroed();
    newrange.flags = (*mr).range[0].flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.ip = new_ip;
    newrange.max_addr.ip = new_ip;
    newrange.min_proto = (*mr).range[0].min;
    newrange.max_proto = (*mr).range[0].max;

    /* Hand modified range to generic setup. */
    nf_nat_setup_info(ct, &mut newrange, HOOK2MANIP(xt_hooknum(par)))
}

unsafe fn netmap_tg4_check(par: *const xt_tgchk_param) -> c_int {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;

    if (*mr).range[0].flags & NF_NAT_RANGE_MAP_IPS == 0 {
        pr_info_ratelimited!("bad MAP_IPS.\n");
        return -EINVAL;
    }
    if (*mr).rangesize != 1 {
        pr_info_ratelimited!("bad rangesize %u.\n", (*mr).rangesize);
        return -EINVAL;
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

static mut netmap_tg_reg: [xt_target; 2] = [
    xt_target {
        name: "NETMAP\0".as_ptr() as *const c_char,
        family: NFPROTO_IPV6,
        revision: 0,
        target: Some(netmap_tg6),
        targetsize: core::mem::size_of::<nf_nat_range>(),
        table: "nat\0".as_ptr() as *const c_char,
        hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING) |
            (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_LOCAL_IN),
        checkentry: Some(netmap_tg6_checkentry),
        destroy: Some(netmap_tg_destroy),
        me: THIS_MODULE,
    },
    xt_target {
        name: "NETMAP\0".as_ptr() as *const c_char,
        family: NFPROTO_IPV4,
        revision: 0,
        target: Some(netmap_tg4),
        targetsize: core::mem::size_of::<nf_nat_ipv4_multi_range_compat>(),
        table: "nat\0".as_ptr() as *const c_char,
        hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING) |
            (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_LOCAL_IN),
        checkentry: Some(netmap_tg4_check),
        destroy: Some(netmap_tg_destroy),
        me: THIS_MODULE,
    },
];

unsafe fn netmap_tg_init() -> c_int {
    xt_register_targets(netmap_tg_reg.as_mut_ptr(), netmap_tg_reg.len())
}

unsafe fn netmap_tg_exit() {
    xt_unregister_targets(netmap_tg_reg.as_mut_ptr(), netmap_tg_reg.len());
}

module_init!(netmap_tg_init);
module_exit!(netmap_tg_exit);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Xtables: 1:1 NAT mapping of subnets");
MODULE_AUTHOR!("Patrick McHardy <kaber@trash.net>");
MODULE_ALIAS!("ip6t_NETMAP");
MODULE_ALIAS!("ipt_NETMAP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
