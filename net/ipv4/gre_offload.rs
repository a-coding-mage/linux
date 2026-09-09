// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	IPV4 GSO/GRO offload support
 *	Linux INET implementation
 *
 *	GRE GSO support
 */

// C headers and build-time configuration are supplied by the surrounding kernel translation.

unsafe fn gre_gso_segment(mut skb: *mut sk_buff, mut features: netdev_features_t) -> *mut sk_buff {
    let tnl_hlen: i32 = skb_inner_mac_header(skb) - skb_transport_header(skb);
    let mut need_csum: bool;
    let mut offload_csum: bool;
    let mut gso_partial: bool;
    let mut need_ipsec: bool;
    let mut segs: *mut sk_buff = ERR_PTR(-EINVAL);
    let mac_offset: u16 = (*skb).mac_header;
    let protocol: __be16 = (*skb).protocol;
    let mac_len: u16 = (*skb).mac_len;
    let mut gre_offset: i32;
    let mut outer_hlen: i32;

    if !(*skb).encapsulation { goto out; }
    if unlikely(tnl_hlen < core::mem::size_of::<gre_base_hdr>() as i32) { goto out; }
    if unlikely(!pskb_may_pull(skb, tnl_hlen)) { goto out; }

    (*skb).encapsulation = false;
    (*SKB_GSO_CB(skb)).encap_level = 0;
    __skb_pull(skb, tnl_hlen);
    skb_reset_mac_header(skb);
    skb_set_network_header(skb, skb_inner_network_offset(skb));
    (*skb).mac_len = skb_inner_network_offset(skb) as u16;
    (*skb).protocol = (*skb).inner_protocol;

    need_csum = ((*skb_shinfo(skb)).gso_type & SKB_GSO_GRE_CSUM) != 0;
    (*skb).encap_hdr_csum = need_csum;
    features &= (*(*skb).dev).hw_enc_features;
    if need_csum { features &= !NETIF_F_SCTP_CRC; }
    need_ipsec = !skb_dst(skb).is_null() && dst_xfrm(skb_dst(skb));
    offload_csum = need_csum && !need_ipsec && ((*(*skb).dev).features & NETIF_F_HW_CSUM) != 0;

    segs = skb_mac_gso_segment(skb, features);
    if IS_ERR_OR_NULL(segs) {
        skb_gso_error_unwind(skb, protocol, tnl_hlen, mac_offset, mac_len);
        goto out;
    }
    gso_partial = ((*skb_shinfo(segs)).gso_type & SKB_GSO_PARTIAL) != 0;
    outer_hlen = skb_tnl_header_len(skb);
    gre_offset = outer_hlen - tnl_hlen;
    skb = segs;
    loop {
        let greh: *mut gre_base_hdr;
        let pcsum: *mut __sum16;
        if (*skb).ip_summed == CHECKSUM_PARTIAL {
            skb_reset_inner_headers(skb);
            (*skb).encapsulation = true;
        }
        (*skb).mac_len = mac_len;
        (*skb).protocol = protocol;
        __skb_push(skb, outer_hlen);
        skb_reset_mac_header(skb);
        skb_set_network_header(skb, mac_len as i32);
        skb_set_transport_header(skb, gre_offset);
        if !need_csum { continue; }
        greh = skb_transport_header(skb) as *mut gre_base_hdr;
        pcsum = greh.add(1) as *mut __sum16;
        if gso_partial && skb_is_gso(skb) {
            let partial_adj = (*skb).len + skb_headroom(skb) - (*SKB_GSO_CB(skb)).data_offset - (*skb_shinfo(skb)).gso_size;
            *pcsum = !csum_fold((htonl(partial_adj) as __wsum));
        } else { *pcsum = 0; }
        *pcsum.add(1) = 0;
        if (*skb).encapsulation || !offload_csum {
            *pcsum = gso_make_checksum(skb, 0);
        } else {
            (*skb).ip_summed = CHECKSUM_PARTIAL;
            (*skb).csum_start = skb_transport_header(skb) - (*skb).head;
            (*skb).csum_offset = core::mem::size_of::<gre_base_hdr>() as u16;
        }
        skb = (*skb).next;
        if skb.is_null() { break; }
    }
out:
    segs
}

unsafe fn gre_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let mut pp: *mut sk_buff = core::ptr::null_mut();
    let mut p: *mut sk_buff;
    let mut greh: *const gre_base_hdr;
    let mut hlen: u32;
    let mut grehlen: u32;
    let off: u32;
    let mut flush = 1;
    let mut ptype: *mut packet_offload;
    let type_: __be16;
    if (*NAPI_GRO_CB(skb)).encap_mark != 0 { goto out; }
    (*NAPI_GRO_CB(skb)).encap_mark = 1;
    off = skb_gro_offset(skb);
    hlen = off + core::mem::size_of::<gre_base_hdr>() as u32;
    greh = skb_gro_header(skb, hlen, off);
    if greh.is_null() { goto out; }
    if ((*greh).flags & !(GRE_KEY | GRE_CSUM)) != 0 { goto out; }
    if ((*greh).flags & GRE_CSUM) != 0 && (*NAPI_GRO_CB(skb)).is_fou { goto out; }
    type_ = (*greh).protocol;
    ptype = gro_find_receive_by_type(type_);
    if ptype.is_null() { goto out; }
    grehlen = GRE_HEADER_SECTION;
    if ((*greh).flags & GRE_KEY) != 0 { grehlen += GRE_HEADER_SECTION; }
    if ((*greh).flags & GRE_CSUM) != 0 { grehlen += GRE_HEADER_SECTION; }
    hlen = off + grehlen;
    if !skb_gro_may_pull(skb, hlen) {
        greh = skb_gro_header_slow(skb, hlen, off);
        if greh.is_null() { goto out; }
    }
    if ((*greh).flags & GRE_CSUM) != 0 && !(*NAPI_GRO_CB(skb)).flush {
        if skb_gro_checksum_simple_validate(skb) { goto out; }
        skb_gro_checksum_try_convert(skb, IPPROTO_GRE, null_compute_pseudo);
    }
    list_for_each_entry(p, head, list) {
        let greh2 = ((*p).data.add(off as usize)) as *const gre_base_hdr;
        if !(*NAPI_GRO_CB(p)).same_flow { continue; }
        if (*greh2).flags != (*greh).flags || (*greh2).protocol != (*greh).protocol { (*NAPI_GRO_CB(p)).same_flow = false; continue; }
        if ((*greh).flags & GRE_KEY) != 0 && *((greh2.add(1)) as *const __be32) != *((greh.add(1)) as *const __be32) { (*NAPI_GRO_CB(p)).same_flow = false; continue; }
    }
    skb_gro_pull(skb, grehlen);
    skb_gro_postpull_rcsum(skb, greh, grehlen);
    pp = call_gro_receive((*ptype).callbacks.gro_receive, head, skb);
    flush = 0;
out:
    skb_gro_flush_final(skb, pp, flush);
    pp
}

unsafe fn gre_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32 {
    let greh = ((*skb).data.add(nhoff as usize)) as *mut gre_base_hdr;
    let mut err = -ENOENT;
    let mut grehlen = core::mem::size_of::<gre_base_hdr>() as u32;
    (*skb).encapsulation = true;
    (*skb_shinfo(skb)).gso_type = SKB_GSO_GRE;
    let type_ = (*greh).protocol;
    if ((*greh).flags & GRE_KEY) != 0 { grehlen += GRE_HEADER_SECTION; }
    if ((*greh).flags & GRE_CSUM) != 0 { grehlen += GRE_HEADER_SECTION; }
    let ptype = gro_find_complete_by_type(type_);
    if !ptype.is_null() { err = (*ptype).callbacks.gro_complete(skb, nhoff + grehlen as i32); }
    skb_set_inner_mac_header(skb, nhoff + grehlen as i32);
    err
}

static gre_offload: net_offload = net_offload { callbacks: net_offload_callbacks { gso_segment: gre_gso_segment, gro_receive: gre_gro_receive, gro_complete: gre_gro_complete } };

unsafe fn gre_offload_init() -> i32 {
    let mut err = inet_add_offload(&gre_offload, IPPROTO_GRE);
    // #if IS_ENABLED(CONFIG_IPV6)
    if err != 0 { return err; }
    err = inet6_add_offload(&gre_offload, IPPROTO_GRE);
    if err != 0 { inet_del_offload(&gre_offload, IPPROTO_GRE); }
    // #endif
    err
}

// device_initcall(gre_offload_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
