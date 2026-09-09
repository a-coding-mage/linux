// SPDX-License-Identifier: GPL-2.0-only
/*
 *	xt_iprange - Netfilter module to match IP address ranges
 *
 *	(C) 2003 Jozsef Kadlecsik <kadlec@netfilter.org>
 *	(C) CC Computer Consultants GmbH, 2008
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// External Linux kernel and netfilter declarations are supplied by the surrounding build.

unsafe fn iprange_mt4(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const xt_iprange_mtinfo = unsafe { (*par).matchinfo as *const xt_iprange_mtinfo };
    let iph: *const iphdr = unsafe { ip_hdr(skb) };
    let mut m: bool;

    if unsafe { (*info).flags & IPRANGE_SRC } != 0 {
        m = unsafe { ntohl((*iph).saddr) < ntohl((*info).src_min.ip) };
        m |= unsafe { ntohl((*iph).saddr) > ntohl((*info).src_max.ip) };
        m ^= unsafe { ((*info).flags & IPRANGE_SRC_INV) != 0 };
        if m {
            return false;
        }
    }
    if unsafe { (*info).flags & IPRANGE_DST } != 0 {
        m = unsafe { ntohl((*iph).daddr) < ntohl((*info).dst_min.ip) };
        m |= unsafe { ntohl((*iph).daddr) > ntohl((*info).dst_max.ip) };
        m ^= unsafe { ((*info).flags & IPRANGE_DST_INV) != 0 };
        if m {
            return false;
        }
    }
    true
}

#[inline]
unsafe fn iprange_ipv6_lt(a: *const in6_addr, b: *const in6_addr) -> i32 {
    let mut i: u32 = 0;

    while i < 4 {
        if unsafe { (*a).s6_addr32[i as usize] != (*b).s6_addr32[i as usize] } {
            return unsafe {
                (ntohl((*a).s6_addr32[i as usize]) < ntohl((*b).s6_addr32[i as usize])) as i32
            };
        }
        i += 1;
    }

    0
}

unsafe fn iprange_mt6(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const xt_iprange_mtinfo = unsafe { (*par).matchinfo as *const xt_iprange_mtinfo };
    let iph: *const ipv6hdr = unsafe { ipv6_hdr(skb) };
    let mut m: bool;

    if unsafe { (*info).flags & IPRANGE_SRC } != 0 {
        m = unsafe { iprange_ipv6_lt(&(*iph).saddr, &(*info).src_min.in6) != 0 };
        m |= unsafe { iprange_ipv6_lt(&(*info).src_max.in6, &(*iph).saddr) != 0 };
        m ^= unsafe { ((*info).flags & IPRANGE_SRC_INV) != 0 };
        if m {
            return false;
        }
    }
    if unsafe { (*info).flags & IPRANGE_DST } != 0 {
        m = unsafe { iprange_ipv6_lt(&(*iph).daddr, &(*info).dst_min.in6) != 0 };
        m |= unsafe { iprange_ipv6_lt(&(*info).dst_max.in6, &(*iph).daddr) != 0 };
        m ^= unsafe { ((*info).flags & IPRANGE_DST_INV) != 0 };
        if m {
            return false;
        }
    }
    true
}

static mut iprange_mt_reg: [xt_match; 2] = [
    xt_match {
        name: "iprange",
        revision: 1,
        family: NFPROTO_IPV4,
        match_fn: Some(iprange_mt4),
        matchsize: core::mem::size_of::<xt_iprange_mtinfo>(),
        me: THIS_MODULE,
    },
    xt_match {
        name: "iprange",
        revision: 1,
        family: NFPROTO_IPV6,
        match_fn: Some(iprange_mt6),
        matchsize: core::mem::size_of::<xt_iprange_mtinfo>(),
        me: THIS_MODULE,
    },
];

unsafe fn iprange_mt_init() -> i32 {
    xt_register_matches(iprange_mt_reg.as_mut_ptr(), iprange_mt_reg.len())
}

unsafe fn iprange_mt_exit() {
    xt_unregister_matches(iprange_mt_reg.as_mut_ptr(), iprange_mt_reg.len());
}

// module_init!(iprange_mt_init);
// module_exit!(iprange_mt_exit);
// MODULE_LICENSE!("GPL");
// MODULE_AUTHOR!("Jozsef Kadlecsik <kadlec@netfilter.org>");
// MODULE_AUTHOR!("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION!("Xtables: arbitrary IPv4 range matching");
// MODULE_ALIAS!("ipt_iprange");
// MODULE_ALIAS!("ip6t_iprange");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
