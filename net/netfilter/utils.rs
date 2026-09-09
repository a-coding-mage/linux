// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_INET)]
pub unsafe fn nf_ip_checksum(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    protocol: u8,
) -> __sum16 {
    let iph = ip_hdr(skb);
    let mut csum: __sum16 = 0;

    match (*skb).ip_summed {
        CHECKSUM_COMPLETE => {
            if hook != NF_INET_PRE_ROUTING && hook != NF_INET_LOCAL_IN {
                return csum;
            }
            if ((protocol != IPPROTO_TCP && protocol != IPPROTO_UDP
                && !csum_fold((*skb).csum))
                || !csum_tcpudp_magic(
                    (*iph).saddr,
                    (*iph).daddr,
                    (*skb).len - dataoff,
                    protocol,
                    (*skb).csum,
                )) {
                (*skb).ip_summed = CHECKSUM_UNNECESSARY;
                return csum;
            }
            // fallthrough
            if protocol != IPPROTO_TCP && protocol != IPPROTO_UDP {
                (*skb).csum = 0;
            } else {
                (*skb).csum = csum_tcpudp_nofold(
                    (*iph).saddr,
                    (*iph).daddr,
                    (*skb).len - dataoff,
                    protocol,
                    0,
                );
            }
            csum = __skb_checksum_complete(skb);
        }
        CHECKSUM_NONE => {
            if protocol != IPPROTO_TCP && protocol != IPPROTO_UDP {
                (*skb).csum = 0;
            } else {
                (*skb).csum = csum_tcpudp_nofold(
                    (*iph).saddr,
                    (*iph).daddr,
                    (*skb).len - dataoff,
                    protocol,
                    0,
                );
            }
            csum = __skb_checksum_complete(skb);
        }
        _ => {}
    }
    csum
}

#[allow(non_snake_case)]
static unsafe fn nf_ip_checksum_partial(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
    protocol: u8,
) -> __sum16 {
    let iph = ip_hdr(skb);
    let mut csum: __sum16 = 0;

    match (*skb).ip_summed {
        CHECKSUM_COMPLETE => {
            if len == (*skb).len - dataoff {
                return nf_ip_checksum(skb, hook, dataoff, protocol);
            }
            // fallthrough
            (*skb).csum = csum_tcpudp_nofold(
                (*iph).saddr, (*iph).daddr, protocol, (*skb).len - dataoff, 0,
            );
            (*skb).ip_summed = CHECKSUM_NONE;
            return __skb_checksum_complete_head(skb, dataoff + len);
        }
        CHECKSUM_NONE => {
            (*skb).csum = csum_tcpudp_nofold(
                (*iph).saddr, (*iph).daddr, protocol, (*skb).len - dataoff, 0,
            );
            (*skb).ip_summed = CHECKSUM_NONE;
            return __skb_checksum_complete_head(skb, dataoff + len);
        }
        _ => {}
    }
    csum
}

pub unsafe fn nf_ip6_checksum(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    protocol: u8,
) -> __sum16 {
    let ip6h = ipv6_hdr(skb);
    let mut csum: __sum16 = 0;

    match (*skb).ip_summed {
        CHECKSUM_COMPLETE => {
            if hook != NF_INET_PRE_ROUTING && hook != NF_INET_LOCAL_IN {
                return csum;
            }
            if !csum_ipv6_magic(
                &(*ip6h).saddr,
                &(*ip6h).daddr,
                (*skb).len - dataoff,
                protocol,
                csum_sub((*skb).csum, skb_checksum(skb, 0, dataoff, 0)),
            ) {
                (*skb).ip_summed = CHECKSUM_UNNECESSARY;
                return csum;
            }
            // fallthrough
            (*skb).csum = !csum_unfold(csum_ipv6_magic(
                &(*ip6h).saddr,
                &(*ip6h).daddr,
                (*skb).len - dataoff,
                protocol,
                csum_sub(0, skb_checksum(skb, 0, dataoff, 0)),
            ));
            csum = __skb_checksum_complete(skb);
        }
        CHECKSUM_NONE => {
            (*skb).csum = !csum_unfold(csum_ipv6_magic(
                &(*ip6h).saddr,
                &(*ip6h).daddr,
                (*skb).len - dataoff,
                protocol,
                csum_sub(0, skb_checksum(skb, 0, dataoff, 0)),
            ));
            csum = __skb_checksum_complete(skb);
        }
        _ => {}
    }
    csum
}

static unsafe fn nf_ip6_checksum_partial(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
    protocol: u8,
) -> __sum16 {
    let ip6h = ipv6_hdr(skb);
    let hsum: __wsum;
    let mut csum: __sum16 = 0;

    match (*skb).ip_summed {
        CHECKSUM_COMPLETE => {
            if len == (*skb).len - dataoff {
                return nf_ip6_checksum(skb, hook, dataoff, protocol);
            }
            // fallthrough
            hsum = skb_checksum(skb, 0, dataoff, 0);
            (*skb).csum = !csum_unfold(csum_ipv6_magic(
                &(*ip6h).saddr,
                &(*ip6h).daddr,
                (*skb).len - dataoff,
                protocol,
                csum_sub(0, hsum),
            ));
            (*skb).ip_summed = CHECKSUM_NONE;
            return __skb_checksum_complete_head(skb, dataoff + len);
        }
        CHECKSUM_NONE => {
            hsum = skb_checksum(skb, 0, dataoff, 0);
            (*skb).csum = !csum_unfold(csum_ipv6_magic(
                &(*ip6h).saddr,
                &(*ip6h).daddr,
                (*skb).len - dataoff,
                protocol,
                csum_sub(0, hsum),
            ));
            (*skb).ip_summed = CHECKSUM_NONE;
            return __skb_checksum_complete_head(skb, dataoff + len);
        }
        _ => {}
    }
    csum
}

pub unsafe fn nf_checksum(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    protocol: u8,
    family: u16,
) -> __sum16 {
    match family {
        AF_INET => nf_ip_checksum(skb, hook, dataoff, protocol),
        AF_INET6 => nf_ip6_checksum(skb, hook, dataoff, protocol),
        _ => 0,
    }
}

pub unsafe fn nf_checksum_partial(
    skb: *mut struct sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
    protocol: u8,
    family: u16,
) -> __sum16 {
    match family {
        AF_INET => nf_ip_checksum_partial(skb, hook, dataoff, len, protocol),
        AF_INET6 => nf_ip6_checksum_partial(skb, hook, dataoff, len, protocol),
        _ => 0,
    }
}

pub unsafe fn nf_route(
    net: *mut struct net,
    dst: *mut *mut struct dst_entry,
    fl: *mut struct flowi,
    strict: bool,
    family: u16,
) -> ::core::ffi::c_int {
    match family {
        AF_INET => nf_ip_route(net, dst, fl, strict),
        AF_INET6 => nf_ip6_route(net, dst, fl, strict),
        _ => 0,
    }
}

/* Only get and check the lengths, not do any hop-by-hop stuff. */
pub unsafe fn nf_ip6_check_hbh_len(skb: *mut struct sk_buff, plen: *mut u32) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int;
    let mut off: ::core::ffi::c_int = core::mem::size_of::<struct ipv6hdr>() as ::core::ffi::c_int;
    let mut nh: *mut u8;

    if !pskb_may_pull(skb, (off + 8) as _) { return -ENOMEM; }
    nh = (ipv6_hdr(skb).add(1)) as *mut u8;
    len = ((*nh.add(1) as u32 + 1) << 3) as _;

    if !pskb_may_pull(skb, (off + len) as _) { return -ENOMEM; }
    nh = skb_network_header(skb);
    off += 2;
    len -= 2;
    while len > 0 {
        let optlen: ::core::ffi::c_int;
        if *nh.add(off as usize) == IPV6_TLV_PAD1 { off += 1; len -= 1; continue; }
        if len < 2 { return -EBADMSG; }
        optlen = *nh.add((off + 1) as usize) as ::core::ffi::c_int + 2;
        if optlen > len { return -EBADMSG; }
        if *nh.add(off as usize) == IPV6_TLV_JUMBO {
            let pkt_len: u32;
            if *nh.add((off + 1) as usize) != 4 || (off & 3) != 2 { return -EBADMSG; }
            pkt_len = ntohl(*(nh.add((off + 2) as usize) as *const __be32));
            if pkt_len <= IPV6_MAXPLEN || (*ipv6_hdr(skb)).payload_len != 0 { return -EBADMSG; }
            if pkt_len > (*skb).len - core::mem::size_of::<struct ipv6hdr>() as u32 { return -EBADMSG; }
            *plen = pkt_len;
        }
        off += optlen;
        len -= optlen;
    }
    if len != 0 { -EBADMSG } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
