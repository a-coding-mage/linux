/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by the surrounding kernel translation:
// linux/ipv6.h, net/dsfield.h, and net/xfrm.h.

#[inline]
pub unsafe fn xfrm4_extract_header(skb: *mut sk_buff) {
    let iph: *const iphdr = ip_hdr(skb);

    (*XFRM_MODE_SKB_CB(skb)).ihl = core::mem::size_of::<iphdr>();
    (*XFRM_MODE_SKB_CB(skb)).id = (*iph).id;
    (*XFRM_MODE_SKB_CB(skb)).frag_off = (*iph).frag_off;
    (*XFRM_MODE_SKB_CB(skb)).tos = (*iph).tos;
    (*XFRM_MODE_SKB_CB(skb)).ttl = (*iph).ttl;
    (*XFRM_MODE_SKB_CB(skb)).optlen = ((*iph).ihl as usize) * 4
        - core::mem::size_of::<iphdr>();
    core::ptr::write_bytes(
        (*XFRM_MODE_SKB_CB(skb)).flow_lbl.as_mut_ptr(),
        0,
        (*XFRM_MODE_SKB_CB(skb)).flow_lbl.len(),
    );
}

#[inline]
pub unsafe fn xfrm6_extract_header(skb: *mut sk_buff) {
    // C build-time condition: IS_ENABLED(CONFIG_IPV6).
    #[cfg(feature = "CONFIG_IPV6")]
    {
        let iph: *mut ipv6hdr = ipv6_hdr(skb);

        (*XFRM_MODE_SKB_CB(skb)).ihl = core::mem::size_of::<ipv6hdr>();
        (*XFRM_MODE_SKB_CB(skb)).id = 0;
        (*XFRM_MODE_SKB_CB(skb)).frag_off = htons(IP_DF);
        (*XFRM_MODE_SKB_CB(skb)).tos = ipv6_get_dsfield(iph);
        (*XFRM_MODE_SKB_CB(skb)).ttl = (*iph).hop_limit;
        (*XFRM_MODE_SKB_CB(skb)).optlen = 0;
        core::ptr::copy_nonoverlapping(
            (*iph).flow_lbl.as_ptr(),
            (*XFRM_MODE_SKB_CB(skb)).flow_lbl.as_mut_ptr(),
            (*XFRM_MODE_SKB_CB(skb)).flow_lbl.len(),
        );
    }
    #[cfg(not(feature = "CONFIG_IPV6"))]
    {
        WARN_ON_ONCE(1);
    }
}

#[inline]
pub unsafe fn xfrm6_beet_make_header(skb: *mut sk_buff) {
    let iph: *mut ipv6hdr = ipv6_hdr(skb);

    (*iph).version = 6;

    core::ptr::copy_nonoverlapping(
        (*XFRM_MODE_SKB_CB(skb)).flow_lbl.as_ptr(),
        (*iph).flow_lbl.as_mut_ptr(),
        (*iph).flow_lbl.len(),
    );
    (*iph).nexthdr = (*XFRM_MODE_SKB_CB(skb)).protocol;

    ipv6_change_dsfield(iph, 0, (*XFRM_MODE_SKB_CB(skb)).tos);
    (*iph).hop_limit = (*XFRM_MODE_SKB_CB(skb)).ttl;
}

#[inline]
pub unsafe fn xfrm4_beet_make_header(skb: *mut sk_buff) {
    let iph: *mut iphdr = ip_hdr(skb);

    (*iph).ihl = 5;
    (*iph).version = 4;

    (*iph).protocol = (*XFRM_MODE_SKB_CB(skb)).protocol;
    (*iph).tos = (*XFRM_MODE_SKB_CB(skb)).tos;

    (*iph).id = (*XFRM_MODE_SKB_CB(skb)).id;
    (*iph).frag_off = (*XFRM_MODE_SKB_CB(skb)).frag_off;
    (*iph).ttl = (*XFRM_MODE_SKB_CB(skb)).ttl;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
