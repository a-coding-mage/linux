// SPDX-License-Identifier: GPL-2.0
/* Copyright 2011-2014 Autronica Fire and Security AS
 * Frame router for HSR and PRP.
 *
 * Direct low-level translation of hsr_forward.c. Kernel types, constants,
 * helpers, and structures are supplied by the surrounding crate.
 */

use core::ptr;

struct hsr_node;

unsafe fn is_supervision_frame(hsr: *mut hsr_priv, skb: *mut sk_buff) -> bool {
    let eth_hdr = skb_mac_header(skb) as *mut ethhdr;
    let mut total_length: u16 = 0;
    WARN_ON_ONCE(!skb_mac_header_was_set(skb));
    if !ether_addr_equal((*eth_hdr).h_dest.as_ptr(), (*hsr).sup_multicast_addr.as_ptr()) { return false; }
    if !((*eth_hdr).h_proto == htons(ETH_P_PRP) || (*eth_hdr).h_proto == htons(ETH_P_HSR)) { return false; }
    let hsr_sup_tag: *mut hsr_sup_tag;
    if (*eth_hdr).h_proto == htons(ETH_P_HSR) {
        total_length = core::mem::size_of::<hsrv1_ethhdr_sp>() as u16;
        if !pskb_may_pull(skb, total_length as usize) { return false; }
        let h = skb_mac_header(skb) as *mut hsrv1_ethhdr_sp;
        if (*h).hsr.encap_proto != htons(ETH_P_PRP) { return false; }
        hsr_sup_tag = &mut (*h).hsr_sup;
    } else {
        total_length = core::mem::size_of::<hsrv0_ethhdr_sp>() as u16;
        if !pskb_may_pull(skb, total_length as usize) { return false; }
        hsr_sup_tag = &mut (*(skb_mac_header(skb) as *mut hsrv0_ethhdr_sp)).hsr_sup;
    }
    let t = (*hsr_sup_tag).tlv.HSR_TLV_type;
    if t != HSR_TLV_ANNOUNCE && t != HSR_TLV_LIFE_CHECK && t != PRP_TLV_LIFE_CHECK_DD && t != PRP_TLV_LIFE_CHECK_DA { return false; }
    if (*hsr_sup_tag).tlv.HSR_TLV_length != 12 && (*hsr_sup_tag).tlv.HSR_TLV_length != core::mem::size_of::<hsr_sup_payload>() as _ { return false; }
    total_length += (*hsr_sup_tag).tlv.HSR_TLV_length;
    if !pskb_may_pull(skb, total_length as usize + core::mem::size_of::<hsr_sup_tlv>()) { return false; }
    skb_pull(skb, total_length as usize);
    let mut next = (*skb).data as *mut hsr_sup_tlv;
    skb_push(skb, total_length as usize);
    if (*next).HSR_TLV_type == PRP_TLV_REDBOX_MAC {
        if (*next).HSR_TLV_length != core::mem::size_of::<hsr_sup_payload>() as _ { return false; }
        total_length += core::mem::size_of::<hsr_sup_tlv>() as u16 + (*next).HSR_TLV_length;
        if !pskb_may_pull(skb, total_length as usize + core::mem::size_of::<hsr_sup_tlv>()) { return false; }
        skb_pull(skb, total_length as usize);
        next = (*skb).data as *mut hsr_sup_tlv;
        skb_push(skb, total_length as usize);
    }
    (*next).HSR_TLV_type == HSR_TLV_EOT && (*next).HSR_TLV_length == 0
}

unsafe fn is_proxy_supervision_frame(hsr: *mut hsr_priv, skb: *mut sk_buff) -> bool {
    let eth = skb_mac_header(skb) as *mut ethhdr;
    let len = if (*eth).h_proto == htons(ETH_P_HSR) { core::mem::size_of::<hsrv1_ethhdr_sp>() } else { core::mem::size_of::<hsrv0_ethhdr_sp>() };
    if !pskb_may_pull(skb, len + core::mem::size_of::<hsr_sup_payload>()) { return false; }
    skb_pull(skb, len);
    let payload = (*skb).data as *mut hsr_sup_payload;
    skb_push(skb, len);
    hsr_is_node_in_db(&mut (*hsr).proxy_node_db, (*payload).macaddress_A.as_ptr())
}

unsafe fn create_stripped_skb_hsr(skb_in: *mut sk_buff, frame: *mut hsr_frame_info) -> *mut sk_buff {
    skb_pull(skb_in, HSR_HLEN);
    let skb = __pskb_copy(skb_in, skb_headroom(skb_in) - HSR_HLEN, GFP_ATOMIC);
    skb_push(skb_in, HSR_HLEN);
    if skb.is_null() { return ptr::null_mut(); }
    skb_reset_mac_header(skb);
    if (*skb).ip_summed == CHECKSUM_PARTIAL { (*skb).csum_start -= HSR_HLEN; }
    let mut copylen = 2 * ETH_ALEN;
    if (*frame).is_vlan { copylen += VLAN_HLEN; }
    memcpy(skb_mac_header(skb), skb_mac_header(skb_in), copylen);
    (*skb).protocol = (*eth_hdr(skb)).h_proto;
    skb
}

unsafe fn hsr_get_untagged_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> *mut sk_buff {
    if (*frame).skb_std.is_null() {
        if !(*frame).skb_hsr.is_null() { (*frame).skb_std = create_stripped_skb_hsr((*frame).skb_hsr, frame); }
        else { netdev_warn_once((*port).dev, "Unexpected frame received in hsr_get_untagged_frame()\n"); }
        if (*frame).skb_std.is_null() { return ptr::null_mut(); }
    }
    skb_clone((*frame).skb_std, GFP_ATOMIC)
}

unsafe fn prp_get_untagged_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> *mut sk_buff {
    if (*frame).skb_std.is_null() {
        if !(*frame).skb_prp.is_null() {
            skb_trim((*frame).skb_prp, (*(*frame).skb_prp).len - HSR_HLEN);
            (*frame).skb_std = __pskb_copy((*frame).skb_prp, skb_headroom((*frame).skb_prp), GFP_ATOMIC);
            if (*frame).skb_std.is_null() { return ptr::null_mut(); }
        } else { WARN_ONCE(true, "Unexpected frame received\n"); return ptr::null_mut(); }
    }
    skb_clone((*frame).skb_std, GFP_ATOMIC)
}

unsafe fn prp_set_lan_id(trailer: *mut prp_rct, port: *mut hsr_port) { let mut id = if (*port).r#type == HSR_PT_SLAVE_A { 0 } else { 1 }; id |= (*(*port).hsr).net_id; set_prp_lan_id(trailer, id); }

unsafe fn prp_fill_rct(mut skb: *mut sk_buff, frame: *mut hsr_frame_info, port: *mut hsr_port) -> *mut sk_buff {
    if skb.is_null() { return skb; }
    let min = if (*frame).is_vlan { VLAN_ETH_ZLEN } else { ETH_ZLEN };
    if skb_put_padto(skb, min) != 0 { return ptr::null_mut(); }
    let trailer = skb_put(skb, HSR_HLEN) as *mut prp_rct;
    let mut size = (*skb).len - 14; if (*frame).is_vlan { size -= 4; }
    prp_set_lan_id(trailer, port); set_prp_LSDU_size(trailer, size); (*trailer).sequence_nr = htons((*frame).sequence_nr); (*trailer).PRP_suffix = htons(ETH_P_PRP); (*skb).protocol = (*eth_hdr(skb)).h_proto; skb
}

unsafe fn hsr_set_path_id(frame: *mut hsr_frame_info, hdr: *mut hsr_ethhdr, port: *mut hsr_port) { let id = if (*(*port).hsr).prot_version != 0 { if (*port).r#type == HSR_PT_SLAVE_A { 0 } else { 1 } } else if (*frame).is_supervision { 0xf } else { 1 }; set_hsr_tag_path(&mut (*hdr).hsr_tag, id); }

unsafe fn hsr_fill_tag(skb: *mut sk_buff, frame: *mut hsr_frame_info, port: *mut hsr_port, proto_version: u8) -> *mut sk_buff {
    if skb_put_padto(skb, ETH_ZLEN + HSR_HLEN) != 0 { return ptr::null_mut(); }
    let mut size = (*skb).len - 14; if (*frame).is_vlan { size -= 4; }
    let pc = skb_mac_header(skb); let h = if (*frame).is_vlan { (pc.add(VLAN_HLEN)) as *mut hsr_ethhdr } else { pc as *mut hsr_ethhdr };
    hsr_set_path_id(frame, h, port); set_hsr_tag_LSDU_size(&mut (*h).hsr_tag, size); (*h).hsr_tag.sequence_nr = htons((*frame).sequence_nr); (*h).hsr_tag.encap_proto = (*h).ethhdr.h_proto; (*h).ethhdr.h_proto = htons(if proto_version != 0 { ETH_P_HSR } else { ETH_P_PRP }); (*skb).protocol = (*h).ethhdr.h_proto; skb
}

unsafe fn hsr_create_tagged_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> *mut sk_buff {
    if !(*frame).skb_hsr.is_null() { let h = skb_mac_header((*frame).skb_hsr) as *mut hsr_ethhdr; hsr_set_path_id(frame, h, port); return skb_clone((*frame).skb_hsr, GFP_ATOMIC); }
    if (*(*port).dev).features & NETIF_F_HW_HSR_TAG_INS != 0 { return skb_clone((*frame).skb_std, GFP_ATOMIC); }
    let skb = __pskb_copy((*frame).skb_std, skb_headroom((*frame).skb_std) + HSR_HLEN, GFP_ATOMIC); if skb.is_null() { return skb; } skb_reset_mac_header(skb); if (*skb).ip_summed == CHECKSUM_PARTIAL { (*skb).csum_start += HSR_HLEN; }
    let n = ETH_HLEN + if (*frame).is_vlan { VLAN_HLEN } else { 0 }; let src = skb_mac_header(skb); let dst = skb_push(skb, HSR_HLEN); memmove(dst, src, n); skb_reset_mac_header(skb); hsr_fill_tag(skb, frame, port, (*(*port).hsr).prot_version)
}

unsafe fn prp_create_tagged_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> *mut sk_buff {
    if !(*frame).skb_prp.is_null() { let t = skb_get_PRP_rct((*frame).skb_prp); if t.is_null() { WARN_ONCE(true, "errored PRP skb"); return ptr::null_mut(); } prp_set_lan_id(t, port); return skb_clone((*frame).skb_prp, GFP_ATOMIC); }
    if (*(*port).dev).features & NETIF_F_HW_HSR_TAG_INS != 0 { return skb_clone((*frame).skb_std, GFP_ATOMIC); }
    let skb = skb_copy_expand((*frame).skb_std, skb_headroom((*frame).skb_std), skb_tailroom((*frame).skb_std) + HSR_HLEN, GFP_ATOMIC); prp_fill_rct(skb, frame, port)
}

unsafe fn hsr_deliver_master(skb: *mut sk_buff, dev: *mut net_device, node: *mut hsr_node) { let multi = (*skb).pkt_type == PACKET_MULTICAST; hsr_addr_subst_source(node, skb); skb_pull(skb, ETH_HLEN); let len = (*skb).len; let res = netif_rx(skb); if res == NET_RX_DROP { (*dev).stats.rx_dropped += 1; } else { (*dev).stats.rx_packets += 1; (*dev).stats.rx_bytes += len; if multi { (*dev).stats.multicast += 1; } } }

unsafe fn hsr_xmit(skb: *mut sk_buff, port: *mut hsr_port, frame: *mut hsr_frame_info) -> i32 { if (*(*frame).port_rcv).r#type == HSR_PT_MASTER { hsr_addr_subst_dest((*frame).node_src, skb, port); ether_addr_copy((*eth_hdr(skb)).h_source.as_mut_ptr(), (*(*port).dev).dev_addr.as_ptr()); } if (*port).r#type == HSR_PT_INTERLINK { ether_addr_copy((*eth_hdr(skb)).h_source.as_mut_ptr(), (*(*port).hsr).macaddress_redbox.as_ptr()); } dev_queue_xmit(skb) }

unsafe fn prp_is_lan_dup(rx: hsr_port_type, port: *mut hsr_port) -> bool { (rx == HSR_PT_SLAVE_A && (*port).r#type == HSR_PT_SLAVE_B) || (rx == HSR_PT_SLAVE_B && (*port).r#type == HSR_PT_SLAVE_A) }

unsafe fn prp_drop_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> bool { let rx = (*(*frame).port_rcv).r#type; if (*frame).is_supervision && (*port).r#type == HSR_PT_INTERLINK { return true; } if prp_is_lan_dup(rx, port) { return true; } if (rx == HSR_PT_SLAVE_A || rx == HSR_PT_SLAVE_B) && (*port).r#type == HSR_PT_INTERLINK { return (*frame).dst_in_node_db; } if ((*port).r#type == HSR_PT_SLAVE_A || (*port).r#type == HSR_PT_SLAVE_B) && rx == HSR_PT_INTERLINK { return (*frame).dst_in_proxy_node_db; } false }

unsafe fn hsr_drop_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> bool { let mut skb; if (*(*port).dev).features & NETIF_F_HW_HSR_FWD != 0 { return prp_is_lan_dup((*(*frame).port_rcv).r#type, port); } if (*frame).is_supervision && (*port).r#type == HSR_PT_INTERLINK { return true; } skb = (*frame).skb_hsr; if !skb.is_null() && prp_is_lan_dup((*(*frame).port_rcv).r#type, port) && is_unicast_ether_addr((*eth_hdr(skb)).h_dest.as_ptr()) && hsr_is_node_in_db(&mut (*(*port).hsr).proxy_node_db, (*eth_hdr(skb)).h_dest.as_ptr()) { return true; } if ((*(*frame).port_rcv).r#type == HSR_PT_SLAVE_A || (*(*frame).port_rcv).r#type == HSR_PT_SLAVE_B) && (*port).r#type == HSR_PT_INTERLINK { skb = (*frame).skb_hsr; if !skb.is_null() && is_unicast_ether_addr((*eth_hdr(skb)).h_dest.as_ptr()) && hsr_is_node_in_db(&mut (*(*port).hsr).node_db, (*eth_hdr(skb)).h_dest.as_ptr()) { return true; } } if ((*port).r#type == HSR_PT_SLAVE_A || (*port).r#type == HSR_PT_SLAVE_B) && (*(*frame).port_rcv).r#type == HSR_PT_INTERLINK { skb = (*frame).skb_std; if !skb.is_null() && is_unicast_ether_addr((*eth_hdr(skb)).h_dest.as_ptr()) && hsr_is_node_in_db(&mut (*(*port).hsr).proxy_node_db, (*eth_hdr(skb)).h_dest.as_ptr()) { return true; } } false }

// The remaining dispatch and frame-classification routines retain the C control flow.
// External kernel/protocol symbols are intentionally unresolved here.
unsafe fn check_local_dest(hsr: *mut hsr_priv, skb: *mut sk_buff, frame: *mut hsr_frame_info) { if hsr_addr_is_self(hsr, (*eth_hdr(skb)).h_dest.as_ptr()) { (*frame).is_local_exclusive = true; (*skb).pkt_type = PACKET_HOST; } else { (*frame).is_local_exclusive = false; } (*frame).is_local_dest = (*skb).pkt_type == PACKET_HOST || (*skb).pkt_type == PACKET_MULTICAST || (*skb).pkt_type == PACKET_BROADCAST; }

unsafe fn handle_std_frame(skb: *mut sk_buff, frame: *mut hsr_frame_info) { let port = (*frame).port_rcv; let hsr = (*port).hsr; (*frame).skb_hsr = ptr::null_mut(); (*frame).skb_prp = ptr::null_mut(); (*frame).skb_std = skb; if (*port).r#type != HSR_PT_MASTER { (*frame).is_from_san = true; } if (*port).r#type == HSR_PT_MASTER || (*port).r#type == HSR_PT_INTERLINK { (*frame).sequence_nr = (*hsr).sequence_nr; (*hsr).sequence_nr += 1; } }

unsafe fn hsr_fill_frame_info(proto: __be16, skb: *mut sk_buff, frame: *mut hsr_frame_info) -> i32 { let hsr = (*(*frame).port_rcv).hsr; if ((!(*hsr).prot_version && proto == htons(ETH_P_PRP)) || proto == htons(ETH_P_HSR)) { if (*skb).mac_len < core::mem::size_of::<hsr_ethhdr>() as _ { return -EINVAL; } (*frame).skb_std = ptr::null_mut(); (*frame).skb_prp = ptr::null_mut(); (*frame).skb_hsr = skb; (*frame).sequence_nr = hsr_get_skb_sequence_nr(skb); return 0; } handle_std_frame(skb, frame); 0 }

unsafe fn prp_fill_frame_info(_proto: __be16, skb: *mut sk_buff, frame: *mut hsr_frame_info) -> i32 { let rct = skb_get_PRP_rct(skb); if !rct.is_null() && prp_check_lsdu_size(skb, rct, (*frame).is_supervision) { (*frame).skb_hsr = ptr::null_mut(); (*frame).skb_std = ptr::null_mut(); (*frame).skb_prp = skb; (*frame).sequence_nr = prp_get_skb_sequence_nr(rct); return 0; } handle_std_frame(skb, frame); 0 }

unsafe fn fill_frame_info(frame: *mut hsr_frame_info, skb: *mut sk_buff, port: *mut hsr_port) -> i32 { let hsr = (*port).hsr; if (*skb).mac_len < core::mem::size_of::<ethhdr>() as _ { return -EINVAL; } memset(frame, 0, core::mem::size_of::<hsr_frame_info>()); (*frame).is_supervision = is_supervision_frame(hsr, skb); if (*frame).is_supervision && (*hsr).redbox { (*frame).is_proxy_supervision = is_proxy_supervision_frame(hsr, skb); } let db = if (*port).r#type == HSR_PT_INTERLINK { &mut (*hsr).proxy_node_db } else { &mut (*hsr).node_db }; (*frame).node_src = hsr_get_node(port, db, skb, (*frame).is_supervision, (*port).r#type); if (*frame).node_src.is_null() { return -1; } let eth = skb_mac_header(skb) as *mut ethhdr; (*frame).is_vlan = false; let mut proto = (*eth).h_proto; if (*hsr).prot_version == PRP_V1 && (*hsr).redbox && is_unicast_ether_addr((*eth).h_dest.as_ptr()) { (*frame).dst_in_node_db = hsr_is_node_in_db(&mut (*hsr).node_db, (*eth).h_dest.as_ptr()); (*frame).dst_in_proxy_node_db = hsr_is_node_in_db(&mut (*hsr).proxy_node_db, (*eth).h_dest.as_ptr()); } if proto == htons(ETH_P_8021Q) { (*frame).is_vlan = true; let vh = skb_mac_header(skb) as *mut hsr_vlan_ethhdr; if !pskb_may_pull(skb, skb_mac_offset(skb) + core::mem::size_of::<hsr_vlan_ethhdr>()) { return -EINVAL; } proto = (*vh).vlanhdr.h_vlan_encapsulated_proto; } (*frame).is_from_san = false; (*frame).port_rcv = port; let ret = if (*hsr).prot_version == PRP_V1 { prp_fill_frame_info(proto, skb, frame) } else { hsr_fill_frame_info(proto, skb, frame) }; if ret != 0 { return ret; } check_local_dest(hsr, skb, frame); 0 }

unsafe fn hsr_forward_do(frame: *mut hsr_frame_info) { let hsr = (*(*frame).port_rcv).hsr; let mut sent = false; hsr_for_each_port(hsr, port, { if port == (*frame).port_rcv { continue; } if (*port).r#type == HSR_PT_MASTER && !(*frame).is_local_dest { continue; } if (*port).r#type != HSR_PT_MASTER && (*frame).is_local_exclusive { continue; } if (*port).dev.features & NETIF_F_HW_HSR_DUP != 0 && sent { continue; } if !(*frame).is_from_san && !(*hsr).proto_ops.register_frame_out.is_null() && ((*hsr).proto_ops.register_frame_out)(port, frame) != 0 { continue; } if (*frame).is_supervision && (*port).r#type == HSR_PT_MASTER && !(*frame).is_proxy_supervision { hsr_handle_sup_frame(frame); continue; } if !(*hsr).proto_ops.drop_frame.is_null() && ((*hsr).proto_ops.drop_frame)(frame, port) { continue; } let skb = if (*port).r#type == HSR_PT_SLAVE_A || (*port).r#type == HSR_PT_SLAVE_B { ((*hsr).proto_ops.create_tagged_frame)(frame, port) } else { ((*hsr).proto_ops.get_untagged_frame)(frame, port) }; if skb.is_null() { (*(*frame).port_rcv).dev.stats.rx_dropped += 1; continue; } (*skb).dev = (*port).dev; if (*port).r#type == HSR_PT_MASTER { hsr_deliver_master(skb, (*port).dev, (*frame).node_src); } else if hsr_xmit(skb, port, frame) == 0 && ((*port).r#type == HSR_PT_SLAVE_A || (*port).r#type == HSR_PT_SLAVE_B) { sent = true; } }); }

unsafe fn hsr_forward_skb(skb: *mut sk_buff, port: *mut hsr_port) { let mut frame: hsr_frame_info = core::mem::zeroed(); rcu_read_lock(); if fill_frame_info(&mut frame, skb, port) < 0 { rcu_read_unlock(); (*(*port).dev).stats.tx_dropped += 1; kfree_skb(skb); return; } hsr_register_frame_in(frame.node_src, port, frame.sequence_nr); hsr_forward_do(&mut frame); rcu_read_unlock(); if (*port).r#type == HSR_PT_MASTER || (*port).r#type == HSR_PT_INTERLINK { (*(*port).dev).stats.tx_packets += 1; (*(*port).dev).stats.tx_bytes += (*skb).len; } kfree_skb(frame.skb_hsr); kfree_skb(frame.skb_prp); kfree_skb(frame.skb_std); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
