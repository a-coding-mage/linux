/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by linux/netfilter_ipv6/ip6_tables.h, net/ipv6.h,
// and net/netfilter/nf_tables.h are referenced externally.

pub unsafe fn nft_set_pktinfo_ipv6(pkt: *mut nft_pktinfo) {
    let mut flags: ::core::ffi::c_uint = IP6_FH_F_AUTH;
    let mut protohdr: ::core::ffi::c_int;
    let mut thoff: ::core::ffi::c_int = 0;
    let mut frag_off: ::core::ffi::c_ushort;

    protohdr = ipv6_find_hdr((*pkt).skb, &mut thoff, -1, &mut frag_off, &mut flags);
    if protohdr < 0 || thoff > U16_MAX {
        nft_set_pktinfo_unspec(pkt);
        return;
    }

    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).tprot = protohdr;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = 0;
    (*pkt).thoff = thoff;
    (*pkt).fragoff = frag_off;
}

pub unsafe fn __nft_set_pktinfo_ipv6_validate(
    pkt: *mut nft_pktinfo,
    nhoff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // Equivalent to #if IS_ENABLED(CONFIG_IPV6):
    // The body is retained here; builds without IPv6 should select the -1 branch below.
    let mut flags: ::core::ffi::c_uint = IP6_FH_F_AUTH;
    let mut ip6h: *mut ipv6hdr;
    let mut _ip6h: ipv6hdr = ::core::mem::zeroed();
    let mut thoff: ::core::ffi::c_uint = nhoff as ::core::ffi::c_uint;
    let mut frag_off: ::core::ffi::c_ushort;
    let mut pkt_len: u32;
    let mut skb_len: u32;
    let mut protohdr: ::core::ffi::c_int;

    ip6h = skb_header_pointer(
        (*pkt).skb,
        skb_network_offset((*pkt).skb) + nhoff,
        ::core::mem::size_of::<ipv6hdr>(),
        &mut _ip6h as *mut ipv6hdr as *mut ::core::ffi::c_void,
    );
    if ip6h.is_null() {
        return -1;
    }

    if (*ip6h).version != 6 {
        return -1;
    }

    pkt_len = ipv6_payload_len((*pkt).skb, ip6h);
    skb_len = (*(*pkt).skb).len - skb_network_offset((*pkt).skb) - nhoff as u32;
    if pkt_len + ::core::mem::size_of::<ipv6hdr>() as u32 > skb_len {
        return -1;
    }

    protohdr = ipv6_find_hdr((*pkt).skb, &mut thoff, -1, &mut frag_off, &mut flags);
    if protohdr < 0 || thoff > U16_MAX {
        return -1;
    }

    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).tprot = protohdr;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = nhoff;
    (*pkt).thoff = thoff;
    (*pkt).fragoff = frag_off;

    0
}

pub unsafe fn nft_set_pktinfo_ipv6_validate(pkt: *mut nft_pktinfo) {
    if __nft_set_pktinfo_ipv6_validate(pkt, 0) < 0 {
        nft_set_pktinfo_unspec(pkt);
    }
}

pub unsafe fn nft_set_pktinfo_ipv6_ingress(
    pkt: *mut nft_pktinfo,
) -> ::core::ffi::c_int {
    // Equivalent to #if IS_ENABLED(CONFIG_IPV6):
    let mut flags: ::core::ffi::c_uint = IP6_FH_F_AUTH;
    let mut frag_off: ::core::ffi::c_ushort;
    let mut thoff: ::core::ffi::c_uint = 0;
    let mut idev: *mut inet6_dev;
    let mut ip6h: *mut ipv6hdr;
    let mut protohdr: ::core::ffi::c_int;
    let mut pkt_len: u32;

    if !pskb_may_pull((*pkt).skb, ::core::mem::size_of::<ipv6hdr>()) {
        return -1;
    }

    ip6h = ipv6_hdr((*pkt).skb);
    if (*ip6h).version != 6 {
        goto_inhdr_error(pkt);
        return -1;
    }

    pkt_len = ipv6_payload_len((*pkt).skb, ip6h);
    if pkt_len + ::core::mem::size_of::<ipv6hdr>() as u32 > (*(*pkt).skb).len {
        idev = __in6_dev_get(nft_in(pkt));
        __IP6_INC_STATS(nft_net(pkt), idev, IPSTATS_MIB_INTRUNCATEDPKTS);
        return -1;
    }

    protohdr = ipv6_find_hdr((*pkt).skb, &mut thoff, -1, &mut frag_off, &mut flags);
    if protohdr < 0 || thoff > U16_MAX {
        goto_inhdr_error(pkt);
        return -1;
    }

    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).tprot = protohdr;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = 0;
    (*pkt).thoff = thoff;
    (*pkt).fragoff = frag_off;

    return 0;
}

unsafe fn goto_inhdr_error(pkt: *mut nft_pktinfo) {
    let idev = __in6_dev_get(nft_in(pkt));
    __IP6_INC_STATS(nft_net(pkt), idev, IPSTATS_MIB_INHDRERRORS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
