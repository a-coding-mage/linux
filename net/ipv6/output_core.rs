// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPv6 library code, needed by static components when full IPv6 support is
 * not configured or static.  These functions are needed by GSO/GRO implementation.
 */
// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn __ipv6_select_ident(
    _net: *mut net,
    _dst: *const in6_addr,
    _src: *const in6_addr,
) -> u32 {
    get_random_u32_above(0)
}

/* This function exists only for tap drivers that must support broken
 * clients requesting UFO without specifying an IPv6 fragment ID.
 *
 * This is similar to ipv6_select_ident() but we use an independent hash
 * seed to limit information leakage.
 *
 * The network header must be set before calling this.
 */
pub unsafe fn ipv6_proxy_select_ident(net: *mut net, skb: *mut sk_buff) -> __be32 {
    let mut buf: [in6_addr; 2] = core::mem::zeroed();
    let addrs: *mut in6_addr;
    let id: u32;

    addrs = skb_header_pointer(
        skb,
        skb_network_offset(skb) + core::mem::offset_of!(ipv6hdr, saddr),
        core::mem::size_of_val(&buf),
        buf.as_mut_ptr() as *mut core::ffi::c_void,
    ) as *mut in6_addr;
    if addrs.is_null() {
        return 0;
    }

    id = __ipv6_select_ident(net, addrs.add(1), addrs);
    htonl(id)
}

pub unsafe fn ipv6_select_ident(
    net: *mut net,
    daddr: *const in6_addr,
    saddr: *const in6_addr,
) -> __be32 {
    let id: u32;

    id = __ipv6_select_ident(net, daddr, saddr);
    htonl(id)
}

pub unsafe fn ip6_find_1stfragopt(skb: *mut sk_buff, nexthdr: *mut *mut u8) -> i32 {
    let mut offset: u32 = core::mem::size_of::<ipv6hdr>() as u32;
    let packet_len: u32 = (skb_tail_pointer(skb) as usize - skb_network_header(skb) as usize)
        as u32;
    let mut found_rhdr = 0;
    *nexthdr = &mut (*ipv6_hdr(skb)).nexthdr;

    while offset <= packet_len {
        let exthdr: *mut ipv6_opt_hdr;

        match **nexthdr {
            NEXTHDR_HOP => {}
            NEXTHDR_ROUTING => found_rhdr = 1,
            NEXTHDR_DEST => {
                // CONFIG_IPV6_MIP6 conditional retained from the C source.
                #[cfg(feature = "CONFIG_IPV6_MIP6")]
                if ipv6_find_tlv(skb, offset, IPV6_TLV_HAO) >= 0 {
                    continue;
                }
                if found_rhdr != 0 {
                    return offset as i32;
                }
            }
            _ => return offset as i32,
        }

        if offset + core::mem::size_of::<ipv6_opt_hdr>() as u32 > packet_len {
            return -EINVAL;
        }

        exthdr = (skb_network_header(skb) as *mut u8).add(offset as usize)
            as *mut ipv6_opt_hdr;
        offset += ipv6_optlen(exthdr);
        if offset > IPV6_MAXPLEN {
            return -EINVAL;
        }
        *nexthdr = &mut (*exthdr).nexthdr;
    }

    -EINVAL
}

pub unsafe fn __ip6_local_out(net: *mut net, sk: *mut sock, mut skb: *mut sk_buff) -> i32 {
    ipv6_set_payload_len(
        ipv6_hdr(skb),
        (*skb).len - core::mem::size_of::<ipv6hdr>(),
    );
    (*IP6CB(skb)).nhoff = core::mem::offset_of!(ipv6hdr, nexthdr) as _;

    /* if egress device is enslaved to an L3 master device pass the
     * skb to its handler for processing
     */
    skb = l3mdev_ip6_out(sk, skb);
    if skb.is_null() {
        return 0;
    }

    (*skb).protocol = htons(ETH_P_IPV6);

    nf_hook(
        NFPROTO_IPV6,
        NF_INET_LOCAL_OUT,
        net,
        sk,
        skb,
        core::ptr::null_mut(),
        skb_dst_dev(skb),
        dst_output,
    )
}

pub unsafe fn ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let mut err = __ip6_local_out(net, sk, skb);
    if err == 1 {
        err = dst_output(net, sk, skb);
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
