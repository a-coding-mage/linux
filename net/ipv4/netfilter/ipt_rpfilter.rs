// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Florian Westphal <fw@strlen.de>
 *
 * based on fib_frontend.c; Author: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies supplied by the surrounding kernel/netfilter environment.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_DESCRIPTION("iptables: ipv4 reverse path filter match");

/* don't try to find route from mcast/bcast/zeronet */
unsafe fn rpfilter_get_saddr(addr: __be32) -> __be32 {
    if ipv4_is_multicast(addr) || ipv4_is_lbcast(addr) || ipv4_is_zeronet(addr) {
        return 0;
    }
    addr
}

unsafe fn rpfilter_lookup_reverse(
    net: *mut net,
    fl4: *mut flowi4,
    dev: *const net_device,
    flags: u8,
) -> bool {
    let mut res: fib_result = core::mem::zeroed();

    if fib_lookup(net, fl4, &mut res, FIB_LOOKUP_IGNORE_LINKSTATE) != 0 {
        return false;
    }

    if res.type_ != RTN_UNICAST {
        if res.type_ != RTN_LOCAL || (flags & XT_RPFILTER_ACCEPT_LOCAL) == 0 {
            return false;
        }
    }
    fib_info_nh_uses_dev(res.fi, dev) || (flags & XT_RPFILTER_LOOSE) != 0
}

unsafe fn rpfilter_is_loopback(skb: *const sk_buff, input: *const net_device) -> bool {
    (*skb).pkt_type == PACKET_LOOPBACK || ((*input).flags & IFF_LOOPBACK) != 0
}

unsafe fn rpfilter_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info: *const xt_rpfilter_info;
    let iph: *const iphdr;
    let mut flow: flowi4 = core::mem::zeroed();
    let invert: bool;

    info = (*par).matchinfo as *const xt_rpfilter_info;
    invert = ((*info).flags & XT_RPFILTER_INVERT) != 0;

    if rpfilter_is_loopback(skb, xt_in(par)) {
        return true ^ invert;
    }

    iph = ip_hdr(skb);
    if ipv4_is_zeronet((*iph).saddr) {
        if ipv4_is_lbcast((*iph).daddr) || ipv4_is_local_multicast((*iph).daddr) {
            return true ^ invert;
        }
    }

    flow.flowi4_iif = LOOPBACK_IFINDEX;
    flow.daddr = (*iph).saddr;
    flow.saddr = rpfilter_get_saddr((*iph).daddr);
    flow.flowi4_mark = if ((*info).flags & XT_RPFILTER_VALID_MARK) != 0 {
        (*skb).mark
    } else {
        0
    };
    flow.flowi4_dscp = ip4h_dscp(iph);
    flow.flowi4_scope = RT_SCOPE_UNIVERSE;
    flow.flowi4_l3mdev = l3mdev_master_ifindex_rcu(xt_in(par));
    flow.flowi4_uid = sock_net_uid(xt_net(par), core::ptr::null_mut());

    rpfilter_lookup_reverse(xt_net(par), &mut flow, xt_in(par), (*info).flags) ^ invert
}

unsafe fn rpfilter_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const xt_rpfilter_info;
    let options: u32 = !XT_RPFILTER_OPTION_MASK;
    if ((*info).flags as u32 & options) != 0 {
        pr_info_ratelimited!("unknown options\n");
        return -EINVAL;
    }

    if strcmp((*par).table, b"mangle\0".as_ptr()) != 0
        && strcmp((*par).table, b"raw\0".as_ptr()) != 0
    {
        pr_info_ratelimited!(
            "only valid in \'raw\' or \'mangle\' table, not \'%s\'\n",
            (*par).table
        );
        return -EINVAL;
    }

    0
}

static mut rpfilter_mt_reg: xt_match = xt_match {
    name: b"rpfilter\0".as_ptr(),
    family: NFPROTO_IPV4,
    checkentry: Some(rpfilter_check),
    r#match: Some(rpfilter_mt),
    matchsize: core::mem::size_of::<xt_rpfilter_info>(),
    hooks: 1 << NF_INET_PRE_ROUTING,
    me: THIS_MODULE,
};

unsafe fn rpfilter_mt_init() -> c_int {
    xt_register_match(&mut rpfilter_mt_reg)
}

unsafe fn rpfilter_mt_exit() {
    xt_unregister_match(&mut rpfilter_mt_reg);
}

// module_init(rpfilter_mt_init);
// module_exit(rpfilter_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
