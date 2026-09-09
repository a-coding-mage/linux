// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPV6 GSO/GRO offload support
 * Linux INET implementation
 *
 * Copyright (C) 2016 secunet Security Networks AG
 * Author: Steffen Klassert <steffen.klassert@secunet.com>
 *
 * ESP GRO support
 */

// C header dependencies are supplied by the surrounding kernel translation.

unsafe fn esp6_nexthdr_esp_offset(ipv6_hdr: *mut ipv6hdr, nhlen: c_int) -> u16 {
    let mut off: c_int = core::mem::size_of::<ipv6hdr>() as c_int;
    let mut exthdr: *mut ipv6_opt_hdr;
    if (*ipv6_hdr).nexthdr == NEXTHDR_ESP || (*ipv6_hdr).nexthdr == NEXTHDR_UDP {
        return core::mem::offset_of!(ipv6hdr, nexthdr) as u16;
    }
    while off < nhlen {
        exthdr = (ipv6_hdr as *mut u8).add(off as usize) as *mut ipv6_opt_hdr;
        if (*exthdr).nexthdr == NEXTHDR_ESP { return off as u16; }
        off += ipv6_optlen(exthdr) as c_int;
    }
    0
}

unsafe fn esp6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let offset = skb_gro_offset(skb);
    let mut xo: *mut xfrm_offload;
    let mut x: *mut xfrm_state;
    let mut encap_type = 0;
    let mut seq: __be32 = 0;
    let mut spi: __be32 = 0;
    let mut nhoff: c_int;
    if (*NAPI_GRO_CB(skb)).proto == IPPROTO_UDP { encap_type = UDP_ENCAP_ESPINUDP; }
    if pskb_pull(skb, offset) == 0 { return core::ptr::null_mut(); }
    if xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq) != 0 { goto_out(skb, offset); return core::ptr::null_mut(); }
    xo = xfrm_offload(skb);
    if xo.is_null() || (*xo).flags & CRYPTO_DONE == 0 {
        let sp = secpath_set(skb);
        if sp.is_null() || (*sp).len == XFRM_MAX_DEPTH { secpath_reset(skb); goto_out(skb, offset); return core::ptr::null_mut(); }
        x = xfrm_input_state_lookup(dev_net((*skb).dev), (*skb).mark,
            &mut (*ipv6_hdr(skb)).daddr as *mut _ as *mut xfrm_address_t,
            spi, IPPROTO_ESP, AF_INET6);
        if !x.is_null() && (*x).dir != 0 && (*x).dir != XFRM_SA_DIR_IN {
            xfrm_state_put(x); x = core::ptr::null_mut();
        }
        if x.is_null() { secpath_reset(skb); goto_out(skb, offset); return core::ptr::null_mut(); }
        (*skb).mark = xfrm_smark_get((*skb).mark, x);
        (*sp).xvec[(*sp).len as usize] = x; (*sp).len += 1; (*sp).olen += 1;
        xo = xfrm_offload(skb);
        if xo.is_null() { secpath_reset(skb); goto_out(skb, offset); return core::ptr::null_mut(); }
    }
    (*xo).flags |= XFRM_GRO;
    nhoff = esp6_nexthdr_esp_offset(ipv6_hdr(skb), offset);
    if nhoff == 0 { goto_out(skb, offset); return core::ptr::null_mut(); }
    (*IP6CB(skb)).nhoff = nhoff as _;
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip6 = core::ptr::null_mut();
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET6;
    (*XFRM_SPI_SKB_CB(skb)).daddroff = core::mem::offset_of!(ipv6hdr, daddr) as _;
    (*XFRM_SPI_SKB_CB(skb)).seq = seq;
    xfrm_input(skb, IPPROTO_ESP, spi, encap_type);
    ERR_PTR(-EINPROGRESS)
}

unsafe fn goto_out(skb: *mut sk_buff, offset: c_int) { skb_push(skb, offset); (*NAPI_GRO_CB(skb)).same_flow = 0; (*NAPI_GRO_CB(skb)).flush = 1; }

unsafe fn esp6_gso_encap(x: *mut xfrm_state, skb: *mut sk_buff) {
    let iph = ipv6_hdr(skb); let xo = xfrm_offload(skb); let mut proto = (*iph).nexthdr;
    skb_push(skb, -skb_network_offset(skb));
    if (*x).outer_mode.encap == XFRM_MODE_TRANSPORT { let mut frag: __be16 = 0; ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>() as _, &mut proto, &mut frag); }
    let esph = ip_esp_hdr(skb); *skb_mac_header(skb) = IPPROTO_ESP;
    (*esph).spi = (*x).id.spi; (*esph).seq_no = htonl((*XFRM_SKB_CB(skb)).seq.output.low); (*xo).proto = proto;
}

unsafe fn xfrm6_tunnel_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); let inner_mode = xfrm_ip2inner_mode(x, (*xo).proto);
    let ty = if (*inner_mode).family == AF_INET { htons(ETH_P_IP) } else { htons(ETH_P_IPV6) };
    skb_eth_gso_segment(skb, features, ty)
}

unsafe fn xfrm6_transport_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); (*skb).transport_header += (*x).props.header_len as _;
    let ops = rcu_dereference(inet6_offloads[(*xo).proto as usize]);
    if !ops.is_null() && (*ops).callbacks.gso_segment.is_some() { ((*ops).callbacks.gso_segment.unwrap())(skb, features) } else { ERR_PTR(-EINVAL) }
}

unsafe fn xfrm6_beet_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); let mut proto = (*xo).proto; (*skb).transport_header += (*x).props.header_len as _;
    if (*x).sel.family != AF_INET6 { (*skb).transport_header -= (core::mem::size_of::<ipv6hdr>() - core::mem::size_of::<iphdr>()) as _;
        if proto == IPPROTO_BEETPH { let ph = (*skb).data as *mut ip_beet_phdr; (*skb).transport_header += (*ph).hdrlen as u16 * 8; proto = (*ph).nexthdr; } else { (*skb).transport_header -= IPV4_BEET_PHMAXLEN as _; }
        if proto == IPPROTO_TCP { (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV6; }
    } else { let mut frag: __be16 = 0; (*skb).transport_header += ipv6_skip_exthdr(skb, 0, &mut proto, &mut frag) as _; }
    if proto == IPPROTO_IPIP { (*skb_shinfo(skb)).gso_type |= SKB_GSO_IPXIP6; }
    __skb_pull(skb, skb_transport_offset(skb)); let ops = rcu_dereference(inet6_offloads[proto as usize]);
    if !ops.is_null() && (*ops).callbacks.gso_segment.is_some() { ((*ops).callbacks.gso_segment.unwrap())(skb, features) } else { ERR_PTR(-EINVAL) }
}

unsafe fn xfrm6_outer_mode_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    match (*x).outer_mode.encap { XFRM_MODE_TUNNEL => xfrm6_tunnel_gso_segment(x, skb, features), XFRM_MODE_TRANSPORT => xfrm6_transport_gso_segment(x, skb, features), XFRM_MODE_BEET => xfrm6_beet_gso_segment(x, skb, features), _ => ERR_PTR(-EOPNOTSUPP) }
}
unsafe fn esp6_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); if xo.is_null() || (*skb_shinfo(skb)).gso_type & SKB_GSO_ESP == 0 { return ERR_PTR(-EINVAL); }
    let sp = skb_sec_path(skb); let x = (*sp).xvec[((*sp).len - 1) as usize]; let aead = (*x).data as *mut crypto_aead; let esph = ip_esp_hdr(skb);
    if (*esph).spi != (*x).id.spi || pskb_may_pull(skb, core::mem::size_of::<ip_esp_hdr>() as c_int + crypto_aead_ivsize(aead)) == 0 { return ERR_PTR(-EINVAL); }
    __skb_pull(skb, core::mem::size_of::<ip_esp_hdr>() as c_int + crypto_aead_ivsize(aead)); (*skb).encap_hdr_csum = 1;
    let mut esp_features = features;
    if features & NETIF_F_HW_ESP == 0 || (*x).xso.dev != (*skb).dev { esp_features = features & !(NETIF_F_SG | NETIF_F_CSUM_MASK | NETIF_F_SCTP_CRC); }
    else if features & NETIF_F_HW_ESP_TX_CSUM == 0 { esp_features = features & !(NETIF_F_CSUM_MASK | NETIF_F_SCTP_CRC); }
    (*xo).flags |= XFRM_GSO_SEGMENT; xfrm6_outer_mode_gso_segment(x, skb, esp_features)
}

unsafe fn esp6_input_tail(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let aead = (*x).data as *mut crypto_aead; let xo = xfrm_offload(skb);
    if pskb_may_pull(skb, core::mem::size_of::<ip_esp_hdr>() as c_int + crypto_aead_ivsize(aead)) == 0 { return -EINVAL; }
    if (*xo).flags & CRYPTO_DONE == 0 { (*skb).ip_summed = CHECKSUM_NONE; }
    esp6_input_done2(skb, 0)
}

unsafe fn esp6_xmit(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> c_int {
    let xo = xfrm_offload(skb); if xo.is_null() { return -EINVAL; }
    let mut hw_offload = true; if features & NETIF_F_HW_ESP == 0 || (*x).xso.dev != (*skb).dev { (*xo).flags |= CRYPTO_FALLBACK; hw_offload = false; }
    let aead = (*x).data as *mut crypto_aead; let alen = crypto_aead_authsize(aead); let tfclen = 0; let blksize = ALIGN(crypto_aead_blocksize(aead), 4);
    let clen = ALIGN((*skb).len + 2 + tfclen, blksize); let plen = clen - (*skb).len - tfclen; let tailen = tfclen + plen + alen;
    let mut esp = esp_info { inplace: true, proto: (*xo).proto, tfclen, clen, plen, tailen, nfrags: 0, seqno: 0, esph: core::ptr::null_mut() };
    if !hw_offload || !skb_is_gso(skb) { esp.nfrags = esp6_output_head(x, skb, &mut esp); if esp.nfrags < 0 { return esp.nfrags; } }
    let seq = (*xo).seq.low; esp.esph = ip_esp_hdr(skb); (*esp.esph).spi = (*x).id.spi; skb_push(skb, -skb_network_offset(skb));
    if (*xo).flags & XFRM_GSO_SEGMENT != 0 { (*esp.esph).seq_no = htonl(seq); if !skb_is_gso(skb) { (*xo).seq.low += 1; } else { (*xo).seq.low += (*skb_shinfo(skb)).gso_segs; } }
    if (*xo).seq.low < seq { (*xo).seq.hi += 1; } esp.seqno = cpu_to_be64((*xo).seq.low as u64 + ((*xo).seq.hi as u64 << 32));
    let mut len = (*skb).len - core::mem::size_of::<ipv6hdr>(); if len > IPV6_MAXPLEN { len = 0; } (*ipv6_hdr(skb)).payload_len = htons(len as _);
    if hw_offload { if skb_ext_add(skb, SKB_EXT_SEC_PATH).is_null() { return -ENOMEM; } let xo2 = xfrm_offload(skb); if xo2.is_null() { return -EINVAL; } (*xo2).flags |= XFRM_XMIT; return 0; }
    let err = esp6_output_tail(x, skb, &mut esp); if err != 0 { return err; } secpath_reset(skb); if skb_needs_linearize(skb, (*(*skb).dev).features) != 0 && __skb_linearize(skb) != 0 { return -ENOMEM; } 0
}

static esp6_offload: net_offload = net_offload { callbacks: net_offload_callbacks { gro_receive: Some(esp6_gro_receive), gso_segment: Some(esp6_gso_segment) } };
static esp6_type_offload: xfrm_type_offload = xfrm_type_offload { owner: THIS_MODULE, proto: IPPROTO_ESP, input_tail: Some(esp6_input_tail), xmit: Some(esp6_xmit), encap: Some(esp6_gso_encap) };

unsafe fn esp6_offload_init() -> c_int { if xfrm_register_type_offload(&esp6_type_offload, AF_INET6) < 0 { pr_info!("%s: can't add xfrm type offload\n", "esp6_offload_init"); return -EAGAIN; } inet6_add_offload(&esp6_offload, IPPROTO_ESP) }
unsafe fn esp6_offload_exit() { xfrm_unregister_type_offload(&esp6_type_offload, AF_INET6); inet6_del_offload(&esp6_offload, IPPROTO_ESP); }

// module_init(esp6_offload_init); module_exit(esp6_offload_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Steffen Klassert <steffen.klassert@secunet.com>");
// MODULE_ALIAS_XFRM_OFFLOAD_TYPE(AF_INET6, XFRM_PROTO_ESP);
// MODULE_DESCRIPTION("IPV6 GSO/GRO offload support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
