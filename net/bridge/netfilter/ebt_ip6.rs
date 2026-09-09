// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_ip6
 *
 *	Authors:
 *	Manohar Castelino <manohar.r.castelino@intel.com>
 *	Kuo-Lang Tseng <kuo-lang.tseng@intel.com>
 *	Jan Engelhardt <jengelh@medozas.de>
 *
 * Summary:
 * This is just a modification of the IPv4 code written by
 * Bart De Schuymer <bdschuym@pandora.be>
 * with the changes required to support IPv6
 *
 *  Jan, 2008
 */

// External kernel declarations supplied by the surrounding translation unit.

#[repr(C)]
pub union Pkthdr {
    pub tcpudphdr: Tcpudphdr,
    pub icmphdr: Icmphdr,
}

#[repr(C)]
pub struct Tcpudphdr {
    pub src: __be16,
    pub dst: __be16,
}

#[repr(C)]
pub struct Icmphdr {
    pub type_: u8,
    pub code: u8,
}

unsafe fn ebt_ip6_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const ebt_ip6_info;
    let mut ip6h = core::mem::MaybeUninit::<ipv6hdr>::uninit();
    let mut pptr: *const Pkthdr;
    let mut pkthdr = core::mem::MaybeUninit::<Pkthdr>::uninit();

    let ih6 = skb_header_pointer(
        skb,
        0,
        core::mem::size_of::<ipv6hdr>(),
        ip6h.as_mut_ptr() as *mut core::ffi::c_void,
    );
    if ih6.is_null() {
        return false;
    }
    if ((*info).bitmask & EBT_IP6_TCLASS) != 0
        && NF_INVF(info, EBT_IP6_TCLASS, (*info).tclass != ipv6_get_dsfield(ih6))
    {
        return false;
    }
    if (((*info).bitmask & EBT_IP6_SOURCE) != 0
        && NF_INVF(
            info,
            EBT_IP6_SOURCE,
            ipv6_masked_addr_cmp(&(*ih6).saddr, &(*info).smsk, &(*info).saddr),
        ))
        || (((*info).bitmask & EBT_IP6_DEST) != 0
            && NF_INVF(
                info,
                EBT_IP6_DEST,
                ipv6_masked_addr_cmp(&(*ih6).daddr, &(*info).dmsk, &(*info).daddr),
            ))
    {
        return false;
    }
    if ((*info).bitmask & EBT_IP6_PROTO) != 0 {
        let mut nexthdr: u8 = (*ih6).nexthdr;
        let mut frag_off: __be16 = 0;
        let offset_ph = ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>(), &mut nexthdr, &mut frag_off);
        if offset_ph == -1 {
            return false;
        }
        if NF_INVF(info, EBT_IP6_PROTO, (*info).protocol != nexthdr) {
            return false;
        }
        if ((*info).bitmask & (EBT_IP6_DPORT | EBT_IP6_SPORT | EBT_IP6_ICMP6)) == 0 {
            return true;
        }

        // min icmpv6 headersize is 4, so sizeof(_pkthdr) is ok.
        pptr = skb_header_pointer(
            skb,
            offset_ph as usize,
            core::mem::size_of::<Pkthdr>(),
            pkthdr.as_mut_ptr() as *mut core::ffi::c_void,
        );
        if pptr.is_null() {
            return false;
        }
        if ((*info).bitmask & EBT_IP6_DPORT) != 0 {
            let dst = u16::from_be((*pptr).tcpudphdr.dst);
            if NF_INVF(info, EBT_IP6_DPORT, dst < (*info).dport[0] || dst > (*info).dport[1]) {
                return false;
            }
        }
        if ((*info).bitmask & EBT_IP6_SPORT) != 0 {
            let src = u16::from_be((*pptr).tcpudphdr.src);
            if NF_INVF(info, EBT_IP6_SPORT, src < (*info).sport[0] || src > (*info).sport[1]) {
                return false;
            }
        }
        if ((*info).bitmask & EBT_IP6_ICMP6) != 0
            && NF_INVF(
                info,
                EBT_IP6_ICMP6,
                (*pptr).icmphdr.type_ < (*info).icmpv6_type[0]
                    || (*pptr).icmphdr.type_ > (*info).icmpv6_type[1]
                    || (*pptr).icmphdr.code < (*info).icmpv6_code[0]
                    || (*pptr).icmphdr.code > (*info).icmpv6_code[1],
            )
        {
            return false;
        }
    }
    true
}

unsafe fn ebt_ip6_mt_check(par: *const xt_mtchk_param) -> i32 {
    let e = (*par).entryinfo as *const ebt_entry;
    let info = (*par).matchinfo as *mut ebt_ip6_info;

    if (*e).ethproto != htons(ETH_P_IPV6) || ((*e).invflags & EBT_IPROTO) != 0 {
        return -EINVAL;
    }
    if ((*info).bitmask & !EBT_IP6_MASK) != 0 || ((*info).invflags & !EBT_IP6_MASK) != 0 {
        return -EINVAL;
    }
    if ((*info).bitmask & (EBT_IP6_DPORT | EBT_IP6_SPORT)) != 0 {
        if ((*info).invflags & EBT_IP6_PROTO) != 0 {
            return -EINVAL;
        }
        if (*info).protocol != IPPROTO_TCP
            && *info.protocol != IPPROTO_UDP
            && *info.protocol != IPPROTO_UDPLITE
            && *info.protocol != IPPROTO_SCTP
            && *info.protocol != IPPROTO_DCCP
        {
            return -EINVAL;
        }
    }
    if ((*info).bitmask & EBT_IP6_DPORT) != 0 && (*info).dport[0] > (*info).dport[1] {
        return -EINVAL;
    }
    if ((*info).bitmask & EBT_IP6_SPORT) != 0 && (*info).sport[0] > (*info).sport[1] {
        return -EINVAL;
    }
    if ((*info).bitmask & EBT_IP6_ICMP6) != 0 {
        if ((*info).invflags & EBT_IP6_PROTO) != 0 || (*info).protocol != IPPROTO_ICMPV6 {
            return -EINVAL;
        }
        if (*info.icmpv6_type[0] > *info.icmpv6_type[1]
            || *info.icmpv6_code[0] > *info.icmpv6_code[1])
        {
            return -EINVAL;
        }
    }
    0
}

static mut ebt_ip6_mt_reg: xt_match = xt_match {
    name: "ip6",
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_ip6_mt),
    checkentry: Some(ebt_ip6_mt_check),
    matchsize: core::mem::size_of::<ebt_ip6_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_ip6_init() -> i32 {
    xt_register_match(&mut ebt_ip6_mt_reg)
}

unsafe fn ebt_ip6_fini() {
    xt_unregister_match(&mut ebt_ip6_mt_reg);
}

// module_init(ebt_ip6_init);
// module_exit(ebt_ip6_fini);
// MODULE_DESCRIPTION("Ebtables: IPv6 protocol packet match");
// MODULE_AUTHOR("Kuo-Lang Tseng <kuo-lang.tseng@intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
