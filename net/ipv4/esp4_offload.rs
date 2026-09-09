// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPV4 GSO/GRO offload support
 * Linux INET implementation
 *
 * Copyright (C) 2016 secunet Security Networks AG
 * Author: Steffen Klassert <steffen.klassert@secunet.com>
 *
 * ESP GRO support
 */

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn esp4_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let offset: i32 = skb_gro_offset(skb);
    let mut xo: *mut xfrm_offload;
    let mut x: *mut xfrm_state;
    let mut encap_type: i32 = 0;
    let mut seq: __be32 = 0;
    let mut spi: __be32 = 0;

    if !pskb_pull(skb, offset) { return core::ptr::null_mut(); }
    if xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq) != 0 { goto_out!(skb, offset); }

    xo = xfrm_offload(skb);
    if xo.is_null() || ((*xo).flags & CRYPTO_DONE) == 0 {
        let sp: *mut sec_path = secpath_set(skb);
        if sp.is_null() { goto_out!(skb, offset); }
        if (*sp).len == XFRM_MAX_DEPTH { goto_out_reset!(skb, offset); }

        x = xfrm_input_state_lookup(dev_net((*skb).dev), (*skb).mark,
            &mut (*ip_hdr(skb)).daddr as *mut _ as *mut xfrm_address_t,
            spi, IPPROTO_ESP, AF_INET);
        if !x.is_null() && (*x).dir != 0 && (*x).dir != XFRM_SA_DIR_IN {
            xfrm_state_put(x); x = core::ptr::null_mut();
        }
        if x.is_null() { goto_out_reset!(skb, offset); }
        (*skb).mark = xfrm_smark_get((*skb).mark, x);
        (*sp).xvec[(*sp).len] = x; (*sp).len += 1; (*sp).olen += 1;
        xo = xfrm_offload(skb);
        if xo.is_null() { goto_out_reset!(skb, offset); }
    }
    (*xo).flags |= XFRM_GRO;
    if (*napi_gro_cb(skb)).proto == IPPROTO_UDP { encap_type = UDP_ENCAP_ESPINUDP; }
    (*xfrm_tunnel_skb_cb(skb)).tunnel.ip4 = core::ptr::null_mut();
    (*xfrm_spi_skb_cb(skb)).family = AF_INET;
    (*xfrm_spi_skb_cb(skb)).daddroff = core::mem::offset_of!(iphdr, daddr);
    (*xfrm_spi_skb_cb(skb)).seq = seq;
    xfrm_input(skb, IPPROTO_ESP, spi, encap_type);
    return ERR_PTR(-EINPROGRESS);

    macro_rules! goto_out_reset { ($s:expr, $o:expr) => {{ secpath_reset($s); skb_push($s, $o); (*napi_gro_cb($s)).same_flow = 0; (*napi_gro_cb($s)).flush = 1; return core::ptr::null_mut(); }}; }
    macro_rules! goto_out { ($s:expr, $o:expr) => {{ skb_push($s, $o); (*napi_gro_cb($s)).same_flow = 0; (*napi_gro_cb($s)).flush = 1; return core::ptr::null_mut(); }}; }
}

unsafe fn esp4_gso_encap(x: *mut xfrm_state, skb: *mut sk_buff) {
    let esph: *mut ip_esp_hdr; let iph = ip_hdr(skb); let xo = xfrm_offload(skb); let proto = (*iph).protocol;
    skb_push(skb, -skb_network_offset(skb)); esph = ip_esp_hdr(skb); *skb_mac_header(skb) = IPPROTO_ESP;
    (*esph).spi = (*x).id.spi; (*esph).seq_no = htonl((*xfrm_skb_cb(skb)).seq.output.low); (*xo).proto = proto;
}

unsafe fn xfrm4_tunnel_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); let inner_mode = xfrm_ip2inner_mode(x, (*xo).proto);
    let ty = if (*inner_mode).family == AF_INET6 { htons(ETH_P_IPV6) } else { htons(ETH_P_IP) };
    skb_eth_gso_segment(skb, features, ty)
}

unsafe fn xfrm4_transport_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let mut segs = ERR_PTR(-EINVAL); let xo = xfrm_offload(skb);
    (*skb).transport_header += (*x).props.header_len;
    let ops = rcu_dereference(inet_offloads[(*xo).proto as usize]);
    if !ops.is_null() && (*ops).callbacks.gso_segment.is_some() { segs = ((*ops).callbacks.gso_segment.unwrap())(skb, features); } segs
}

unsafe fn xfrm4_beet_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); let mut segs = ERR_PTR(-EINVAL); let mut proto: u8 = (*xo).proto;
    (*skb).transport_header += (*x).props.header_len;
    if (*x).sel.family != AF_INET6 { if proto == IPPROTO_BEETPH { let ph = (*skb).data as *mut ip_beet_phdr; (*skb).transport_header += (*ph).hdrlen * 8; proto = (*ph).nexthdr; } else { (*skb).transport_header -= IPV4_BEETPHMAXLEN; } }
    else { let mut frag: __be16 = 0; (*skb).transport_header += ipv6_skip_exthdr(skb, 0, &mut proto, &mut frag); if proto == IPPROTO_TCP { (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV4; } }
    if proto == IPPROTO_IPV6 { (*skb_shinfo(skb)).gso_type |= SKB_GSO_IPXIP4; }
    __skb_pull(skb, skb_transport_offset(skb)); let ops = rcu_dereference(inet_offloads[proto as usize]);
    if !ops.is_null() && (*ops).callbacks.gso_segment.is_some() { segs = ((*ops).callbacks.gso_segment.unwrap())(skb, features); } segs
}

unsafe fn xfrm4_outer_mode_gso_segment(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    match (*x).outer_mode.encap { XFRM_MODE_TUNNEL => xfrm4_tunnel_gso_segment(x, skb, features), XFRM_MODE_TRANSPORT => xfrm4_transport_gso_segment(x, skb, features), XFRM_MODE_BEET => xfrm4_beet_gso_segment(x, skb, features), _ => ERR_PTR(-EOPNOTSUPP) }
}

unsafe fn esp4_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let xo = xfrm_offload(skb); if xo.is_null() || ((*skb_shinfo(skb)).gso_type & SKB_GSO_ESP) == 0 { return ERR_PTR(-EINVAL); }
    let sp = skb_sec_path(skb); let x = (*sp).xvec[(*sp).len - 1]; let aead = (*x).data as *mut crypto_aead; let esph = ip_esp_hdr(skb);
    if (*esph).spi != (*x).id.spi || !pskb_may_pull(skb, core::mem::size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead)) { return ERR_PTR(-EINVAL); }
    __skb_pull(skb, core::mem::size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead)); (*skb).encap_hdr_csum = 1; (*xo).flags |= XFRM_GSO_SEGMENT; xfrm4_outer_mode_gso_segment(x, skb, features)
}

unsafe fn esp_input_tail(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 { let aead = (*x).data as *mut crypto_aead; let xo = xfrm_offload(skb); if !pskb_may_pull(skb, core::mem::size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead)) { return -EINVAL; } if ((*xo).flags & CRYPTO_DONE) == 0 { (*skb).ip_summed = CHECKSUM_NONE; } esp_input_done2(skb, 0) }

unsafe fn esp_xmit(x: *mut xfrm_state, skb: *mut sk_buff, features: netdev_features_t) -> i32 {
    let xo = xfrm_offload(skb); if xo.is_null() { return -EINVAL; } let aead = (*x).data as *mut crypto_aead; let mut hw_offload = true;
    if ((features & NETIF_F_HW_ESP) == 0 && ((*skb).dev).gso_partial_features & NETIF_F_HW_ESP == 0) || (*x).xso.dev != (*skb).dev { (*xo).flags |= CRYPTO_FALLBACK; hw_offload = false; }
    let alen = crypto_aead_authsize(aead); let blksize = ALIGN(crypto_aead_blocksize(aead), 4); let clen = ALIGN((*skb).len + 2, blksize); let _plen = clen - (*skb).len; let _tailen = _plen + alen;
    let esph = ip_esp_hdr(skb); (*esph).spi = (*x).id.spi; skb_push(skb, -skb_network_offset(skb));
    if ((*xo).flags & XFRM_GSO_SEGMENT) != 0 { (*esph).seq_no = htonl((*xo).seq.low); (*xo).seq.low = (*xo).seq.low.wrapping_add(if !skb_is_gso(skb) { 1 } else { (*skb_shinfo(skb)).gso_segs }); }
    if !hw_offload { secpath_reset(skb); } if hw_offload { (*xo).flags |= XFRM_XMIT; return 0; } 0
}

#[repr(C)]
struct net_offload {
    callbacks: net_offload_callbacks,
}
#[repr(C)]
struct net_offload_callbacks {
    gro_receive: Option<unsafe fn(*mut list_head, *mut sk_buff) -> *mut sk_buff>,
    gso_segment: Option<unsafe fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff>,
}
#[repr(C)]
struct xfrm_type_offload {
    owner: *mut core::ffi::c_void,
    proto: i32,
    input_tail: Option<unsafe fn(*mut xfrm_state, *mut sk_buff) -> i32>,
    xmit: Option<unsafe fn(*mut xfrm_state, *mut sk_buff, netdev_features_t) -> i32>,
    encap: Option<unsafe fn(*mut xfrm_state, *mut sk_buff)>,
}

static ESP4_OFFLOAD: net_offload = net_offload { callbacks: net_offload_callbacks {
    gro_receive: Some(esp4_gro_receive), gso_segment: Some(esp4_gso_segment)
} };
static mut ESP_TYPE_OFFLOAD: xfrm_type_offload = xfrm_type_offload {
    owner: core::ptr::null_mut(), proto: IPPROTO_ESP,
    input_tail: Some(esp_input_tail), xmit: Some(esp_xmit), encap: Some(esp4_gso_encap)
};

unsafe fn esp4_offload_init() -> i32 {
    if xfrm_register_type_offload(&ESP_TYPE_OFFLOAD, AF_INET) < 0 {
        pr_info!("%s: can't add xfrm type offload\n", "esp4_offload_init");
        return -EAGAIN;
    }
    inet_add_offload(&ESP4_OFFLOAD, IPPROTO_ESP)
}

unsafe fn esp4_offload_exit() {
    xfrm_unregister_type_offload(&ESP_TYPE_OFFLOAD, AF_INET);
    inet_del_offload(&ESP4_OFFLOAD, IPPROTO_ESP);
}

#[allow(dead_code)]
const MODULE_LICENSE: &str = "GPL";
#[allow(dead_code)]
const MODULE_AUTHOR: &str = "Steffen Klassert <steffen.klassert@secunet.com>";
#[allow(dead_code)]
const MODULE_DESCRIPTION: &str = "IPV4 GSO/GRO offload support";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
