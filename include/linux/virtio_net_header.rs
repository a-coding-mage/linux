/* SPDX-License-Identifier: GPL-2.0 */
/* Dependencies supplied by the corresponding Linux networking headers. */

#[inline]
pub unsafe fn virtio_net_hdr_match_proto(protocol: __be16, gso_type: __u8) -> bool {
    match gso_type & !VIRTIO_NET_HDR_GSO_ECN {
        VIRTIO_NET_HDR_GSO_TCPV4 => protocol == cpu_to_be16(ETH_P_IP),
        VIRTIO_NET_HDR_GSO_TCPV6 => protocol == cpu_to_be16(ETH_P_IPV6),
        VIRTIO_NET_HDR_GSO_UDP | VIRTIO_NET_HDR_GSO_UDP_L4 =>
            protocol == cpu_to_be16(ETH_P_IP) || protocol == cpu_to_be16(ETH_P_IPV6),
        _ => false,
    }
}

#[inline]
pub unsafe fn virtio_net_hdr_set_proto(skb: *mut sk_buff, hdr: *const virtio_net_hdr) -> c_int {
    if (*skb).protocol != 0 { return 0; }
    match (*hdr).gso_type & !VIRTIO_NET_HDR_GSO_ECN {
        VIRTIO_NET_HDR_GSO_TCPV4 | VIRTIO_NET_HDR_GSO_UDP | VIRTIO_NET_HDR_GSO_UDP_L4 => {
            (*skb).protocol = cpu_to_be16(ETH_P_IP);
        }
        VIRTIO_NET_HDR_GSO_TCPV6 => (*skb).protocol = cpu_to_be16(ETH_P_IPV6),
        _ => return -EINVAL,
    }
    0
}

#[inline]
pub unsafe fn __virtio_net_hdr_to_skb(skb: *mut sk_buff, hdr: *const virtio_net_hdr,
                                      little_endian: bool, hdr_gso_type: u8) -> c_int {
    let mut nh_min_len = core::mem::size_of::<iphdr>() as c_uint;
    let mut gso_type = 0u32;
    let mut thlen = 0u32;
    let mut p_off = 0u32;
    let mut ip_proto = 0u32;
    if hdr_gso_type != VIRTIO_NET_HDR_GSO_NONE {
        match hdr_gso_type & !VIRTIO_NET_HDR_GSO_ECN {
            VIRTIO_NET_HDR_GSO_TCPV4 => { gso_type=SKB_GSO_TCPV4; ip_proto=IPPROTO_TCP; thlen=core::mem::size_of::<tcphdr>() as u32; }
            VIRTIO_NET_HDR_GSO_TCPV6 => { gso_type=SKB_GSO_TCPV6; ip_proto=IPPROTO_TCP; thlen=core::mem::size_of::<tcphdr>() as u32; nh_min_len=core::mem::size_of::<ipv6hdr>() as u32; }
            VIRTIO_NET_HDR_GSO_UDP => { gso_type=SKB_GSO_UDP; ip_proto=IPPROTO_UDP; thlen=core::mem::size_of::<udphdr>() as u32; }
            VIRTIO_NET_HDR_GSO_UDP_L4 => { gso_type=SKB_GSO_UDP_L4; ip_proto=IPPROTO_UDP; thlen=core::mem::size_of::<udphdr>() as u32; }
            _ => return -EINVAL,
        }
        if hdr_gso_type & VIRTIO_NET_HDR_GSO_ECN != 0 { gso_type |= SKB_GSO_TCP_ECN; }
        if (*hdr).gso_size == 0 { return -EINVAL; }
    }
    skb_reset_mac_header(skb);
    if (*hdr).flags & VIRTIO_NET_HDR_F_NEEDS_CSUM != 0 {
        let start = __virtio16_to_cpu(little_endian, (*hdr).csum_start) as u32;
        let off = __virtio16_to_cpu(little_endian, (*hdr).csum_offset) as u32;
        let needed = start + core::cmp::max(thlen, off + core::mem::size_of::<__sum16>() as u32);
        if !pskb_may_pull(skb, needed) || !skb_partial_csum_set(skb, start, off) || skb_transport_offset(skb) < nh_min_len { return -EINVAL; }
        nh_min_len = skb_transport_offset(skb); p_off = nh_min_len + thlen;
        if !pskb_may_pull(skb, p_off) { return -EINVAL; }
    } else if gso_type != 0 && (*skb).network_header != 0 {
        let mut keys = core::mem::MaybeUninit::<flow_keys_basic>::uninit();
        if !(*skb).protocol { let protocol = dev_parse_header_protocol(skb); if protocol == 0 { virtio_net_hdr_set_proto(skb, hdr); } else if !virtio_net_hdr_match_proto(protocol, hdr_gso_type) { return -EINVAL; } else { (*skb).protocol=protocol; } }
        'retry: loop {
            if !skb_flow_dissect_flow_keys_basic(core::ptr::null_mut(), skb, keys.as_mut_ptr(), core::ptr::null_mut(), 0, 0, 0, 0) {
                if gso_type & SKB_GSO_UDP != 0 && (*skb).protocol == htons(ETH_P_IP) { (*skb).protocol=htons(ETH_P_IPV6); continue 'retry; }
                return -EINVAL;
            }
            let keys = keys.assume_init(); p_off=keys.control.thoff+thlen;
            if !pskb_may_pull(skb,p_off) || keys.basic.ip_proto != ip_proto { return -EINVAL; }
            skb_set_transport_header(skb, keys.control.thoff); break;
        }
    } else if gso_type != 0 { p_off=nh_min_len+thlen; if !pskb_may_pull(skb,p_off) { return -EINVAL; } }
    if hdr_gso_type != VIRTIO_NET_HDR_GSO_NONE {
        let mut gso_size=__virtio16_to_cpu(little_endian,(*hdr).gso_size) as u32; let mut nh_off=p_off; let shinfo=skb_shinfo(skb);
        match gso_type & !SKB_GSO_TCP_ECN {
            SKB_GSO_UDP => nh_off-=thlen,
            SKB_GSO_UDP_L4 => { if (*hdr).flags & VIRTIO_NET_HDR_F_NEEDS_CSUM == 0 || (*skb).csum_offset != core::mem::offset_of!(udphdr, check) as u32 || (*skb).len-p_off > gso_size*UDP_MAX_SEGMENTS || gso_type != SKB_GSO_UDP_L4 { return -EINVAL; } }
            SKB_GSO_TCPV4 | SKB_GSO_TCPV6 => { if (*skb).ip_summed==CHECKSUM_PARTIAL && (*skb).csum_offset != core::mem::offset_of!(tcphdr, check) as u32 { return -EINVAL; } gso_size=core::cmp::max(gso_size,TCP_MIN_GSO_SIZE); }
            _ => {}
        }
        if gso_size==GSO_BY_FRAGS || (*skb).len-nh_off <= gso_size { return 0; }
        (*shinfo).gso_size=gso_size; (*shinfo).gso_type=gso_type|SKB_GSO_DODGY; (*shinfo).gso_segs=0;
    }
    0
}

#[inline] pub unsafe fn virtio_net_hdr_to_skb(skb:*mut sk_buff,hdr:*const virtio_net_hdr,little_endian:bool)->c_int { __virtio_net_hdr_to_skb(skb,hdr,little_endian,(*hdr).gso_type) }

#[inline] pub unsafe fn __virtio_net_set_hdrlen(skb:*const sk_buff,hdr:*mut virtio_net_hdr,little_endian:bool) { let mut hdr_len=skb_transport_offset(skb) as u16; if (*hdr).gso_type==VIRTIO_NET_HDR_GSO_UDP_L4 { hdr_len+=core::mem::size_of::<udphdr>() as u16; } else { hdr_len+=tcp_hdrlen(skb) as u16; } (*hdr).hdr_len=__cpu_to_virtio16(little_endian,hdr_len); }
#[inline] pub unsafe fn __virtio_net_set_tnl_hdrlen(skb:*const sk_buff,hdr:*mut virtio_net_hdr) { let mut hdr_len=skb_inner_transport_offset(skb) as u16; if (*hdr).gso_type==VIRTIO_NET_HDR_GSO_UDP_L4 { hdr_len+=core::mem::size_of::<udphdr>() as u16; } else { hdr_len+=inner_tcp_hdrlen(skb) as u16; } (*hdr).hdr_len=__cpu_to_virtio16(true,hdr_len); }

#[inline] pub unsafe fn virtio_net_hdr_from_skb(skb:*const sk_buff,hdr:*mut virtio_net_hdr,little_endian:bool,has_data_valid:bool,vlan_hlen:c_int)->c_int {
    memset(hdr,0,core::mem::size_of::<virtio_net_hdr>());
    if skb_is_gso(skb) { let s=skb_shinfo(skb); (*hdr).hdr_len=__cpu_to_virtio16(little_endian,skb_headlen(skb) as u16); (*hdr).gso_size=__cpu_to_virtio16(little_endian,(*s).gso_size as u16); (*hdr).gso_type=if (*s).gso_type&SKB_GSO_TCPV4!=0 {VIRTIO_NET_HDR_GSO_TCPV4} else if (*s).gso_type&SKB_GSO_TCPV6!=0 {VIRTIO_NET_HDR_GSO_TCPV6} else if (*s).gso_type&SKB_GSO_UDP_L4!=0 {VIRTIO_NET_HDR_GSO_UDP_L4} else {return -EINVAL}; if (*s).gso_type&SKB_GSO_TCP_ECN!=0 {(*hdr).gso_type|=VIRTIO_NET_HDR_GSO_ECN;} } else {(*hdr).gso_type=VIRTIO_NET_HDR_GSO_NONE;}
    if (*skb).ip_summed==CHECKSUM_PARTIAL { (*hdr).flags=VIRTIO_NET_HDR_F_NEEDS_CSUM; (*hdr).csum_start=__cpu_to_virtio16(little_endian,(skb_checksum_start_offset(skb)+vlan_hlen) as u16); (*hdr).csum_offset=__cpu_to_virtio16(little_endian,(*skb).csum_offset as u16); } else if has_data_valid && (*skb).ip_summed==CHECKSUM_UNNECESSARY {(*hdr).flags=VIRTIO_NET_HDR_F_DATA_VALID;} 0
}

#[inline] pub unsafe fn virtio_l3min(is_ipv6:bool)->usize { if is_ipv6 {core::mem::size_of::<ipv6hdr>()} else {core::mem::size_of::<iphdr>()} }

#[inline] pub unsafe fn virtio_net_handle_csum_offload(skb:*mut sk_buff,hdr:*mut virtio_net_hdr,tnl_csum_negotiated:bool)->c_int { if (*hdr).gso_type&VIRTIO_NET_HDR_GSO_UDP_TUNNEL==0 {if (*hdr).flags&VIRTIO_NET_HDR_F_DATA_VALID==0{return 0;}(*skb).ip_summed=CHECKSUM_UNNECESSARY;if (*hdr).flags&VIRTIO_NET_HDR_F_UDP_TUNNEL_CSUM==0{return 0;}if !tnl_csum_negotiated{return -EINVAL;}(*skb).csum_level=1;return 0;}if (*hdr).flags&VIRTIO_NET_HDR_F_DATA_VALID!=0{return -EINVAL;}0 }

#[inline] pub unsafe fn virtio_net_hdr_tnl_to_skb(skb:*mut sk_buff,vhdr:*const virtio_net_hdr_v1_hash_tunnel,tnl_hdr_negotiated:bool,tnl_csum_negotiated:bool,little_endian:bool)->c_int { let hdr=vhdr as *const virtio_net_hdr;let t=(*hdr).gso_type&VIRTIO_NET_HDR_GSO_UDP_TUNNEL;if t==0{return virtio_net_hdr_to_skb(skb,hdr,little_endian);}if !tnl_hdr_negotiated||t==VIRTIO_NET_HDR_GSO_UDP_TUNNEL{return -EINVAL;}let r=__virtio_net_hdr_to_skb(skb,hdr,true,(*hdr).gso_type&!t);if r!=0{return r;}if (*hdr).flags&VIRTIO_NET_HDR_F_UDP_TUNNEL_CSUM!=0{if !tnl_csum_negotiated{return -EINVAL;}(*skb_shinfo(skb)).gso_type|=SKB_GSO_UDP_TUNNEL_CSUM;}else{(*skb_shinfo(skb)).gso_type|=SKB_GSO_UDP_TUNNEL;}0 }

#[inline] pub unsafe fn virtio_net_hdr_tnl_from_skb(skb:*const sk_buff,vhdr:*mut virtio_net_hdr_v1_hash_tunnel,tnl_hdr_negotiated:bool,little_endian:bool,vlan_hlen:c_int,has_data_valid:bool,feature_hdrlen:bool)->c_int {let hdr=vhdr as *mut virtio_net_hdr;let t=(*skb_shinfo(skb)).gso_type&(SKB_GSO_UDP_TUNNEL|SKB_GSO_UDP_TUNNEL_CSUM);if t==0{return virtio_net_hdr_from_skb(skb,hdr,little_endian,has_data_valid,vlan_hlen);}if !tnl_hdr_negotiated{return -EINVAL;}(*skb_shinfo(skb)).gso_type&=!t;let r=virtio_net_hdr_from_skb(skb,hdr,true,false,vlan_hlen);(*skb_shinfo(skb)).gso_type|=t;if r!=0{return r;}if feature_hdrlen&&(*hdr).hdr_len!=0{__virtio_net_set_tnl_hdrlen(skb,hdr);}if (*skb).protocol==htons(ETH_P_IPV6){(*hdr).gso_type|=VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6;}else{(*hdr).gso_type|=VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4;}0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
