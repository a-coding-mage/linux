// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	IPV6 GSO/GRO offload support
 *	Linux INET6 implementation
 *
 *      UDPv6 GSO support
 */
// C dependencies supplied by other translation units are intentionally not expanded here.

unsafe fn udp6_ufo_fragment(
    mut skb: *mut sk_buff,
    mut features: netdev_features_t,
) -> *mut sk_buff {
    let mut segs: *mut sk_buff = ERR_PTR(-EINVAL);
    let mut mss: u32;
    let mut unfrag_ip6hlen: u32;
    let mut unfrag_len: u32;
    let mut fptr: *mut frag_hdr;
    let mut packet_start: *mut u8;
    let mut prevhdr: *mut u8;
    let mut nexthdr: u8;
    let frag_hdr_sz: u8 = core::mem::size_of::<frag_hdr>() as u8;
    let mut csum: __wsum;
    let mut tnl_hlen: i32;
    let mut err: i32;

    if (*skb).encapsulation
        && (skb_shinfo(skb).gso_type & (SKB_GSO_UDP_TUNNEL | SKB_GSO_UDP_TUNNEL_CSUM)) != 0
    {
        segs = skb_udp_tunnel_segment(skb, features, true);
    } else {
        let ipv6h: *const ipv6hdr;
        let uh: *mut udphdr;

        if (skb_shinfo(skb).gso_type & (SKB_GSO_UDP | SKB_GSO_UDP_L4)) == 0 {
            return segs;
        }
        if !pskb_may_pull(skb, core::mem::size_of::<udphdr>()) {
            return segs;
        }
        if (skb_shinfo(skb).gso_type & SKB_GSO_UDP_L4) != 0 {
            return __udp_gso_segment(skb, features, true);
        }
        mss = skb_shinfo(skb).gso_size;
        if (*skb).len <= mss {
            return segs;
        }

        /* Do software UFO. Complete and fill in the UDP checksum as HW cannot
         * do checksum of UDP packets sent as multiple IP fragments.
         */
        uh = udp_hdr(skb);
        ipv6h = ipv6_hdr(skb);
        (*uh).check = 0;
        csum = skb_checksum(skb, 0, (*skb).len, 0);
        (*uh).check = udp_v6_check((*skb).len, &(*ipv6h).saddr, &(*ipv6h).daddr, csum);
        if (*uh).check == 0 {
            (*uh).check = CSUM_MANGLED_0;
        }
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
        if !(*skb).encap_hdr_csum {
            features |= NETIF_F_HW_CSUM;
        }
        tnl_hlen = skb_tnl_header_len(skb);
        if (*skb).mac_header < tnl_hlen + frag_hdr_sz as i32
            && gso_pskb_expand_head(skb, tnl_hlen + frag_hdr_sz as i32) != 0
        {
            return segs;
        }

        err = ip6_find_1stfragopt(skb, &mut prevhdr);
        if err < 0 {
            return ERR_PTR(err);
        }
        unfrag_ip6hlen = err as u32;
        nexthdr = *prevhdr;
        *prevhdr = NEXTHDR_FRAGMENT;
        unfrag_len = ((*skb).network_header - (*skb).mac_header) as u32
            + unfrag_ip6hlen + tnl_hlen as u32;
        packet_start = (*skb).head.add(SKB_GSO_CB(skb).mac_offset as usize);
        core::ptr::copy(packet_start, packet_start.sub(frag_hdr_sz as usize), unfrag_len as usize);

        SKB_GSO_CB(skb).mac_offset -= frag_hdr_sz as i32;
        (*skb).mac_header -= frag_hdr_sz as i32;
        (*skb).network_header -= frag_hdr_sz as i32;
        fptr = ((*skb).network_header as *mut u8).add(unfrag_ip6hlen as usize) as *mut frag_hdr;
        (*fptr).nexthdr = nexthdr;
        (*fptr).reserved = 0;
        (*fptr).identification = ipv6_proxy_select_ident(dev_net((*skb).dev), skb);
        /* Fragment the skb. ipv6 header and the remaining fields of the
         * fragment header are updated in ipv6_gso_segment()
         */
        segs = skb_segment(skb, features);
    }
    segs
}

unsafe fn udp6_gro_lookup_skb(skb: *mut sk_buff, sport: __be16, dport: __be16) -> *mut sock {
    let iph: *const ipv6hdr = skb_gro_network_header(skb);
    let net: *mut net = dev_net_rcu((*skb).dev);
    let sk: *mut sock = udp_tunnel_sk(net, true);
    if !sk.is_null() && dport == htons((*sk).sk_num) {
        return sk;
    }
    let mut iif = 0;
    let mut sdif = 0;
    inet6_get_iif_sdif(skb, &mut iif, &mut sdif);
    __udp6_lib_lookup(net, &(*iph).saddr, sport, &(*iph).daddr, dport, iif, sdif, core::ptr::null_mut())
}

pub unsafe fn udp6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let uh = udp_gro_udphdr(skb);
    let mut sk: *mut sock = core::ptr::null_mut();
    if uh.is_null() { (*NAPI_GRO_CB(skb)).flush = 1; return core::ptr::null_mut(); }
    if (*NAPI_GRO_CB(skb)).flush { return udp_gro_receive(head, skb, uh, sk); }
    if skb_gro_checksum_validate_zero_check(skb, IPPROTO_UDP, (*uh).check, ip6_gro_compute_pseudo) {
        (*NAPI_GRO_CB(skb)).flush = 1; return core::ptr::null_mut();
    } else if (*uh).check { skb_gro_checksum_try_convert(skb, IPPROTO_UDP, ip6_gro_compute_pseudo); }
    if static_branch_unlikely(&udpv6_encap_needed_key) { sk = udp6_gro_lookup_skb(skb, (*uh).source, (*uh).dest); }
    udp_gro_receive(head, skb, uh, sk)
}

pub unsafe fn udp6_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32 {
    let offset = NAPI_GRO_CB(skb).network_offsets[(*skb).encapsulation as usize];
    let ipv6h = ((*skb).data.add(offset as usize)) as *const ipv6hdr;
    let uh = ((*skb).data.add(nhoff as usize)) as *mut udphdr;
    if NAPI_GRO_CB(skb).is_flist && !NAPI_GRO_CB(skb).encap_mark {
        udp_set_len(uh, (*skb).len - nhoff as u32);
        skb_shinfo(skb).gso_type |= SKB_GSO_FRAGLIST | SKB_GSO_UDP_L4;
        skb_shinfo(skb).gso_segs = NAPI_GRO_CB(skb).count;
        __skb_incr_checksum_unnecessary(skb);
        return 0;
    }
    if (*uh).check { (*uh).check = !udp_v6_check((*skb).len - nhoff as u32, &(*ipv6h).saddr, &(*ipv6h).daddr, 0); }
    udp_gro_complete(skb, nhoff, udp6_lib_lookup_skb)
}

pub unsafe fn udpv6_offload_init() -> i32 {
    net_hotdata.udpv6_offload = net_offload { callbacks: net_offload_callbacks {
        gso_segment: Some(udp6_ufo_fragment), gro_receive: Some(udp6_gro_receive), gro_complete: Some(udp6_gro_complete),
    }};
    inet6_add_offload(&mut net_hotdata.udpv6_offload, IPPROTO_UDP)
}

pub unsafe fn udpv6_offload_exit() -> i32 {
    inet6_del_offload(&mut net_hotdata.udpv6_offload, IPPROTO_UDP)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
