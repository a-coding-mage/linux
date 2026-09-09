// SPDX-License-Identifier: GPL-2.0-only
/*
 *  iptables module to match inet_addr_type() of an ip.
 *
 *  Copyright (c) 2004 Patrick McHardy <kaber@trash.net>
 *  (C) 2007 Laszlo Attila Toth <panther@balabit.hu>
 */

// Kernel and netfilter dependencies are supplied by the surrounding crate.

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn match_lookup_rt6(
    net: *mut net,
    dev: *const net_device,
    addr: *const in6_addr,
    mask: u16,
) -> u32 {
    let mut flow: flowi6 = core::mem::zeroed();
    let mut rt: *mut rt6_info = core::ptr::null_mut();
    let mut ret: u32 = 0;
    let route_err: i32;

    flow.daddr = *addr;
    if !dev.is_null() {
        flow.flowi6_oif = (*dev).ifindex;
    }

    if !dev.is_null() && (mask & XT_ADDRTYPE_LOCAL) != 0 {
        if nf_ipv6_chk_addr(net, addr, dev, true) {
            ret = XT_ADDRTYPE_LOCAL;
        }
    }

    route_err = nf_ip6_route(
        net,
        &mut rt as *mut *mut rt6_info as *mut *mut dst_entry,
        flowi6_to_flowi(&mut flow),
        false,
    );
    if route_err != 0 {
        return XT_ADDRTYPE_UNREACHABLE;
    }

    if ((*rt).rt6i_flags & RTF_REJECT) != 0 {
        ret = XT_ADDRTYPE_UNREACHABLE;
    }
    if dev.is_null() && ((*rt).rt6i_flags & RTF_LOCAL) != 0 {
        ret |= XT_ADDRTYPE_LOCAL;
    }
    if ipv6_anycast_destination(&mut (*rt).dst as *mut dst_entry, addr) {
        ret |= XT_ADDRTYPE_ANYCAST;
    }

    dst_release(&mut (*rt).dst as *mut dst_entry);
    ret
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn match_type6(
    net: *mut net,
    dev: *const net_device,
    addr: *const in6_addr,
    mask: u16,
) -> bool {
    let addr_type = ipv6_addr_type(addr);

    if (mask & XT_ADDRTYPE_MULTICAST) != 0 && (addr_type & IPV6_ADDR_MULTICAST) == 0 {
        return false;
    }
    if (mask & XT_ADDRTYPE_UNICAST) != 0 && (addr_type & IPV6_ADDR_UNICAST) == 0 {
        return false;
    }
    if (mask & XT_ADDRTYPE_UNSPEC) != 0 && addr_type != IPV6_ADDR_ANY {
        return false;
    }

    if ((XT_ADDRTYPE_LOCAL | XT_ADDRTYPE_ANYCAST | XT_ADDRTYPE_UNREACHABLE) & mask) != 0 {
        return (mask & match_lookup_rt6(net, dev, addr, mask)) != 0;
    }
    true
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn addrtype_mt6(
    net: *mut net,
    dev: *const net_device,
    skb: *const sk_buff,
    info: *const xt_addrtype_info_v1,
) -> bool {
    let iph = ipv6_hdr(skb);
    let mut ret = true;

    if (*info).source != 0 {
        ret &= match_type6(net, dev, &(*iph).saddr, (*info).source)
            ^ (((*info).flags & XT_ADDRTYPE_INVERT_SOURCE) != 0);
    }
    if ret && (*info).dest != 0 {
        ret &= match_type6(net, dev, &(*iph).daddr, (*info).dest)
            ^ (((*info).flags & XT_ADDRTYPE_INVERT_DEST) != 0);
    }
    ret
}

unsafe fn match_type(net: *mut net, dev: *const net_device, addr: __be32, mask: u16) -> bool {
    (mask & (1u16 << inet_dev_addr_type(net, dev, addr))) != 0
}

unsafe fn addrtype_mt_v0(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let net = xt_net(par);
    let info = (*par).matchinfo as *const xt_addrtype_info;
    let iph = ip_hdr(skb);
    let mut ret = true;

    if (*info).source != 0 {
        ret &= match_type(net, core::ptr::null(), (*iph).saddr, (*info).source)
            ^ ((*info).invert_source != 0);
    }
    if (*info).dest != 0 {
        ret &= match_type(net, core::ptr::null(), (*iph).daddr, (*info).dest)
            ^ ((*info).invert_dest != 0);
    }
    ret
}

unsafe fn addrtype_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let net = xt_net(par);
    let info = (*par).matchinfo as *const xt_addrtype_info_v1;
    let mut dev: *const net_device = core::ptr::null();
    let mut ret = true;

    if ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_IN) != 0 {
        dev = xt_in(par);
    } else if ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_OUT) != 0 {
        dev = xt_out(par);
    }

    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    if xt_family(par) == NFPROTO_IPV6 {
        return addrtype_mt6(net, dev, skb, info);
    }

    let iph = ip_hdr(skb);
    if (*info).source != 0 {
        ret &= match_type(net, dev, (*iph).saddr, (*info).source)
            ^ (((*info).flags & XT_ADDRTYPE_INVERT_SOURCE) != 0);
    }
    if ret && (*info).dest != 0 {
        ret &= match_type(net, dev, (*iph).daddr, (*info).dest)
            ^ (((*info).flags & XT_ADDRTYPE_INVERT_DEST) != 0);
    }
    ret
}

unsafe fn addrtype_mt_check_hooks(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_addrtype_info_v1;
    let mut errmsg: *const c_char;

    if ((*par).hook_mask & ((1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN))) != 0
        && ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_OUT) != 0
    {
        errmsg = b"output interface limitation not valid in PREROUTING and INPUT\0".as_ptr() as *const c_char;
        pr_info_ratelimited(errmsg);
        return -EINVAL;
    }
    if ((*par).hook_mask & ((1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_OUT))) != 0
        && ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_IN) != 0
    {
        errmsg = b"input interface limitation not valid in POSTROUTING and OUTPUT\0".as_ptr() as *const c_char;
        pr_info_ratelimited(errmsg);
        return -EINVAL;
    }
    0
}

unsafe fn addrtype_mt_checkentry_v1(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_addrtype_info_v1;
    let mut errmsg = b"both incoming and outgoing interface limitation cannot be selected\0".as_ptr() as *const c_char;

    if ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_IN) != 0
        && ((*info).flags & XT_ADDRTYPE_LIMIT_IFACE_OUT) != 0
    {
        pr_info_ratelimited(errmsg);
        return -EINVAL;
    }

    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    if (*par).family == NFPROTO_IPV6 {
        if (((*info).source | (*info).dest) & XT_ADDRTYPE_BLACKHOLE) != 0 {
            errmsg = b"ipv6 BLACKHOLE matching not supported\0".as_ptr() as *const c_char;
        } else if ((*info).source | (*info).dest) >= XT_ADDRTYPE_PROHIBIT {
            errmsg = b"ipv6 PROHIBIT (THROW, NAT ..) matching not supported\0".as_ptr() as *const c_char;
        } else if (((*info).source | (*info).dest) & XT_ADDRTYPE_BROADCAST) != 0 {
            errmsg = b"ipv6 does not support BROADCAST matching\0".as_ptr() as *const c_char;
        } else {
            return 0;
        }
        pr_info_ratelimited(errmsg);
        return -EINVAL;
    }
    0
}

static mut addrtype_mt_reg: [xt_match; 3] = [
    xt_match { name: *b"addrtype\0", family: NFPROTO_IPV4, revision: 0, match: Some(addrtype_mt_v0), check_hooks: None, checkentry: None, matchsize: core::mem::size_of::<xt_addrtype_info>(), me: THIS_MODULE },
    xt_match { name: *b"addrtype\0", family: NFPROTO_IPV4, revision: 1, match: Some(addrtype_mt_v1), check_hooks: Some(addrtype_mt_check_hooks), checkentry: Some(addrtype_mt_checkentry_v1), matchsize: core::mem::size_of::<xt_addrtype_info_v1>(), me: THIS_MODULE },
    xt_match { name: *b"addrtype\0", family: NFPROTO_IPV6, revision: 1, match: Some(addrtype_mt_v1), check_hooks: Some(addrtype_mt_check_hooks), checkentry: Some(addrtype_mt_checkentry_v1), matchsize: core::mem::size_of::<xt_addrtype_info_v1>(), me: THIS_MODULE },
];

unsafe fn addrtype_mt_init() -> i32 {
    xt_register_matches(addrtype_mt_reg.as_mut_ptr(), addrtype_mt_reg.len())
}

unsafe fn addrtype_mt_exit() {
    xt_unregister_matches(addrtype_mt_reg.as_mut_ptr(), addrtype_mt_reg.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
