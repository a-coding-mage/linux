// SPDX-License-Identifier: GPL-2.0-only
/*
 * xt_HMARK - Netfilter module to set mark by means of hashing
 *
 * (C) 2012 by Hans Schillstrom <hans.schillstrom@ericsson.com>
 * (C) 2012 by Pablo Neira Ayuso <pablo@netfilter.org>
 */

// C headers and build-time configuration dependencies are supplied externally.

#[repr(C)]
pub struct HmarkTuple {
    pub src: __be32,
    pub dst: __be32,
    pub uports: hmark_ports,
    pub proto: u8,
}

#[inline]
unsafe fn hmark_addr6_mask(addr32: *const __be32, mask: *const __be32) -> __be32 {
    (*addr32.add(0) & *mask.add(0))
        ^ (*addr32.add(1) & *mask.add(1))
        ^ (*addr32.add(2) & *mask.add(2))
        ^ (*addr32.add(3) & *mask.add(3))
}

#[inline]
unsafe fn hmark_addr_mask(l3num: c_int, addr32: *const __be32, mask: *const __be32) -> __be32 {
    match l3num {
        AF_INET => *addr32 & *mask,
        AF_INET6 => hmark_addr6_mask(addr32, mask),
        _ => 0,
    }
}

#[inline]
unsafe fn hmark_swap_ports(uports: *mut hmark_ports, info: *const xt_hmark_info) {
    let mut hp: hmark_ports = core::mem::zeroed();
    hp.b32 = ((*uports).b32 & (*info).port_mask.b32) | (*info).port_set.b32;
    let src: u16 = ntohs(hp.b16.src);
    let dst: u16 = ntohs(hp.b16.dst);

    if dst > src {
        (*uports).v32 = ((dst as u32) << 16) | src as u32;
    } else {
        (*uports).v32 = ((src as u32) << 16) | dst as u32;
    }
}

unsafe fn hmark_ct_set_htuple(
    skb: *const sk_buff,
    t: *mut HmarkTuple,
    info: *const xt_hmark_info,
) -> c_int {
    #[cfg(CONFIG_NF_CONNTRACK)]
    {
        let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
        let ct = nf_ct_get(skb, &mut ctinfo);
        if ct.is_null() {
            return -1;
        }
        let otuple = &(*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;
        let rtuple = &(*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
        (*t).src = hmark_addr_mask((*otuple).src.l3num, (*otuple).src.u3.ip6.as_ptr(), (*info).src_mask.ip6.as_ptr());
        (*t).dst = hmark_addr_mask((*otuple).src.l3num, (*rtuple).src.u3.ip6.as_ptr(), (*info).dst_mask.ip6.as_ptr());
        if (*info).flags & XT_HMARK_FLAG(XT_HMARK_METHOD_L3) != 0 {
            return 0;
        }
        (*t).proto = nf_ct_protonum(ct);
        if (*t).proto != IPPROTO_ICMP as u8 {
            (*t).uports.b16.src = (*otuple).src.u.all;
            (*t).uports.b16.dst = (*rtuple).src.u.all;
            hmark_swap_ports(&mut (*t).uports, info);
        }
        return 0;
    }
    #[cfg(not(CONFIG_NF_CONNTRACK))]
    { let _ = (skb, t, info); -1 }
}

#[inline]
unsafe fn hmark_hash(t: *mut HmarkTuple, info: *const xt_hmark_info) -> u32 {
    let mut src = ntohl((*t).src);
    let mut dst = ntohl((*t).dst);
    if dst < src { core::mem::swap(&mut src, &mut dst); }
    let hash = jhash_3words(src, dst, (*t).uports.v32, (*info).hashrnd);
    reciprocal_scale(hash ^ ((*t).proto as u32 & (*info).proto_mask), (*info).hmodulus) + (*info).hoffset
}

unsafe fn hmark_set_tuple_ports(skb: *const sk_buff, mut nhoff: c_uint, t: *mut HmarkTuple, info: *const xt_hmark_info) {
    let protoff = proto_ports_offset((*t).proto);
    if protoff < 0 { return; }
    nhoff += protoff as c_uint;
    if skb_copy_bits(skb, nhoff, &mut (*t).uports as *mut _ as *mut c_void, core::mem::size_of::<hmark_ports>()) < 0 { return; }
    hmark_swap_ports(&mut (*t).uports, info);
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn get_inner6_hdr(skb: *const sk_buff, offset: *mut c_int) -> c_int {
    let mut ih6: icmp6hdr = core::mem::zeroed();
    let icmp6h = skb_header_pointer(skb, *offset, core::mem::size_of::<icmp6hdr>(), &mut ih6 as *mut _ as *mut c_void);
    if icmp6h.is_null() { return 0; }
    if (*icmp6h).icmp6_type != 0 && (*icmp6h).icmp6_type < 128 { *offset += core::mem::size_of::<icmp6hdr>() as c_int; return 1; }
    0
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn hmark_pkt_set_htuple_ipv6(skb: *const sk_buff, t: *mut HmarkTuple, info: *const xt_hmark_info) -> c_int {
    let mut ip6: *mut ipv6hdr = ((*skb).data.add(skb_network_offset(skb) as usize)) as *mut ipv6hdr;
    let mut nhoff: c_uint = 0;
    let mut fragoff: u16 = 0;
    let mut flag: c_int = IP6_FH_F_AUTH;
    let mut nexthdr = ipv6_find_hdr(skb, &mut nhoff, -1, &mut fragoff, &mut flag);
    if nexthdr < 0 { return 0; }
    if (flag & IP6_FH_F_FRAG) != 0 || nexthdr != IPPROTO_ICMPV6 as c_int { return hmark_pkt_set_htuple_ipv6_noicmp(skb, t, info, ip6, nexthdr, nhoff); }
    if get_inner6_hdr(skb, &mut nhoff) != 0 {
        let mut ih6: ipv6hdr = core::mem::zeroed();
        ip6 = skb_header_pointer(skb, nhoff, core::mem::size_of::<ipv6hdr>(), &mut ih6 as *mut _ as *mut c_void) as *mut ipv6hdr;
        if ip6.is_null() { return -1; }
        flag = IP6_FH_F_AUTH;
        nexthdr = ipv6_find_hdr(skb, &mut nhoff, -1, &mut fragoff, &mut flag);
        if nexthdr < 0 { return -1; }
    }
    hmark_pkt_set_htuple_ipv6_noicmp(skb, t, info, ip6, nexthdr, nhoff)
}

unsafe fn hmark_pkt_set_htuple_ipv6_noicmp(skb: *const sk_buff, t: *mut HmarkTuple, info: *const xt_hmark_info, ip6: *mut ipv6hdr, nexthdr: c_int, nhoff: c_uint) -> c_int {
    (*t).src = hmark_addr6_mask((*ip6).saddr.s6_addr32.as_ptr(), (*info).src_mask.ip6.as_ptr());
    (*t).dst = hmark_addr6_mask((*ip6).daddr.s6_addr32.as_ptr(), (*info).dst_mask.ip6.as_ptr());
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_METHOD_L3) != 0 { return 0; }
    (*t).proto = nexthdr as u8;
    if (*t).proto == IPPROTO_ICMPV6 as u8 { return 0; }
    hmark_set_tuple_ports(skb, nhoff, t, info); 0
}

unsafe fn get_inner_hdr(skb: *const sk_buff, iphsz: c_int, nhoff: *mut c_int) -> c_int {
    let mut ih: icmphdr = core::mem::zeroed();
    let icmph = skb_header_pointer(skb, *nhoff + iphsz, core::mem::size_of::<icmphdr>(), &mut ih as *mut _ as *mut c_void);
    if icmph.is_null() || (*icmph).type_ > NR_ICMP_TYPES { return 0; }
    if !icmp_is_err((*icmph).type_) { return 0; }
    *nhoff += iphsz + core::mem::size_of::<icmphdr>() as c_int; 1
}

unsafe fn hmark_pkt_set_htuple_ipv4(skb: *const sk_buff, t: *mut HmarkTuple, info: *const xt_hmark_info) -> c_int {
    let mut nhoff = skb_network_offset(skb) as c_int;
    let mut ip = ((*skb).data.add(nhoff as usize)) as *mut iphdr;
    if (*ip).protocol == IPPROTO_ICMP as u8 && get_inner_hdr(skb, ((*ip).ihl as c_int) * 4, &mut nhoff) != 0 {
        let mut ih: iphdr = core::mem::zeroed();
        ip = skb_header_pointer(skb, nhoff, core::mem::size_of::<iphdr>(), &mut ih as *mut _ as *mut c_void) as *mut iphdr;
        if ip.is_null() { return -1; }
    }
    (*t).src = (*ip).saddr & (*info).src_mask.ip;
    (*t).dst = (*ip).daddr & (*info).dst_mask.ip;
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_METHOD_L3) != 0 { return 0; }
    (*t).proto = (*ip).protocol;
    if (*t).proto == IPPROTO_ICMP as u8 || ip_is_fragment(ip) { return 0; }
    hmark_set_tuple_ports(skb, ((*ip).ihl as c_uint) * 4 + nhoff as c_uint, t, info); 0
}

unsafe fn hmark_tg_v4(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const xt_hmark_info;
    let mut t: HmarkTuple = core::mem::zeroed();
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_CT) != 0 { if hmark_ct_set_htuple(skb, &mut t, info) < 0 { return XT_CONTINUE; } }
    else if hmark_pkt_set_htuple_ipv4(skb, &mut t, info) < 0 { return XT_CONTINUE; }
    (*skb).mark = hmark_hash(&mut t, info); XT_CONTINUE
}

unsafe fn hmark_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const xt_hmark_info;
    if (*info).hmodulus == 0 { return -EINVAL; }
    if (*info).proto_mask != 0 && (*info).flags & XT_HMARK_FLAG(XT_HMARK_METHOD_L3) != 0 { pr_info_ratelimited!("proto mask must be zero with L3 mode\n"); return -EINVAL; }
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_SPI_MASK) != 0 && (*info).flags & (XT_HMARK_FLAG(XT_HMARK_SPORT_MASK) | XT_HMARK_FLAG(XT_HMARK_DPORT_MASK)) != 0 { return -EINVAL; }
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_SPI) != 0 && (*info).flags & (XT_HMARK_FLAG(XT_HMARK_SPORT) | XT_HMARK_FLAG(XT_HMARK_DPORT)) != 0 { pr_info_ratelimited!("spi-set and port-set can't be combined\n"); return -EINVAL; }
    0
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn hmark_tg_v6(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const xt_hmark_info;
    let mut t: HmarkTuple = core::mem::zeroed();
    if (*info).flags & XT_HMARK_FLAG(XT_HMARK_CT) != 0 { if hmark_ct_set_htuple(skb, &mut t, info) < 0 { return XT_CONTINUE; } }
    else if hmark_pkt_set_htuple_ipv6(skb, &mut t, info) < 0 { return XT_CONTINUE; }
    (*skb).mark = hmark_hash(&mut t, info); XT_CONTINUE
}

// The target registration table, module metadata, and init/exit registration
// use kernel-provided declarations and are preserved as external integration points.
unsafe fn hmark_tg_init() -> c_int { xt_register_targets(hmark_tg_reg.as_ptr(), hmark_tg_reg.len()) }
unsafe fn hmark_tg_exit() { xt_unregister_targets(hmark_tg_reg.as_ptr(), hmark_tg_reg.len()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
