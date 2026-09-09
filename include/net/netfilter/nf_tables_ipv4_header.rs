/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// net/netfilter/nf_tables.h and net/ip.h

pub unsafe fn nft_set_pktinfo_ipv4(pkt: *mut nft_pktinfo) {
    let ip: *mut iphdr;

    ip = ip_hdr((*pkt).skb);
    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).tprot = (*ip).protocol;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = 0;
    (*pkt).thoff = ip_hdrlen((*pkt).skb);
    (*pkt).fragoff = ntohs((*ip).frag_off) & IP_OFFSET;
}

pub unsafe fn __nft_set_pktinfo_ipv4_validate(
    pkt: *mut nft_pktinfo,
    nhoff: i32,
) -> i32 {
    let iph: *mut iphdr;
    let mut _iph: iphdr = core::mem::zeroed();
    let len: u32;
    let thoff: u32;
    let skb_len: u32;

    iph = skb_header_pointer(
        (*pkt).skb,
        skb_network_offset((*pkt).skb) + nhoff,
        core::mem::size_of::<iphdr>(),
        &mut _iph as *mut iphdr as *mut core::ffi::c_void,
    );
    if iph.is_null() {
        return -1;
    }

    if (*iph).ihl < 5 || (*iph).version != 4 {
        return -1;
    }

    len = iph_totlen((*pkt).skb, iph);
    thoff = (*iph).ihl as u32 * 4;
    skb_len = (*(*pkt).skb).len - skb_network_offset((*pkt).skb) as u32 - nhoff as u32;

    if skb_len < len {
        return -1;
    } else if len < thoff {
        return -1;
    } else if thoff < core::mem::size_of::<iphdr>() as u32 {
        return -1;
    }

    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).tprot = (*iph).protocol;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = nhoff;
    (*pkt).thoff = skb_network_offset((*pkt).skb) + nhoff + thoff as i32;
    (*pkt).fragoff = ntohs((*iph).frag_off) & IP_OFFSET;

    0
}

pub unsafe fn nft_set_pktinfo_ipv4_validate(pkt: *mut nft_pktinfo) {
    if __nft_set_pktinfo_ipv4_validate(pkt, 0) < 0 {
        nft_set_pktinfo_unspec(pkt);
    }
}

pub unsafe fn nft_set_pktinfo_ipv4_ingress(pkt: *mut nft_pktinfo) -> i32 {
    let iph: *mut iphdr;
    let len: u32;
    let thoff: u32;

    if !pskb_may_pull((*pkt).skb, core::mem::size_of::<iphdr>()) {
        return -1;
    }

    iph = ip_hdr((*pkt).skb);
    if (*iph).ihl < 5 || (*iph).version != 4 {
        goto_inhdr_error: {
            __IP_INC_STATS(nft_net(pkt), IPSTATS_MIB_INHDRERRORS);
            return -1;
        }
    }

    len = iph_totlen((*pkt).skb, iph);
    thoff = (*iph).ihl as u32 * 4;
    if (*(*pkt).skb).len < len {
        __IP_INC_STATS(nft_net(pkt), IPSTATS_MIB_INTRUNCATEDPKTS);
        return -1;
    } else if len < thoff {
        __IP_INC_STATS(nft_net(pkt), IPSTATS_MIB_INHDRERRORS);
        return -1;
    } else if thoff < core::mem::size_of::<iphdr>() as u32 {
        return -1;
    }

    (*pkt).flags = NFT_PKTINFO_L4PROTO;
    (*pkt).ethertype = (*(*pkt).skb).protocol;
    (*pkt).nhoff = 0;
    (*pkt).tprot = (*iph).protocol;
    (*pkt).thoff = thoff;
    (*pkt).fragoff = ntohs((*iph).frag_off) & IP_OFFSET;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
