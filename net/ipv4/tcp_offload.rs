// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	IPV4 GSO/GRO offload support
 *	Linux INET implementation
 *
 *	TCPv4 GSO/GRO support
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn tcp_gso_tstamp(mut skb: *mut sk_buff, gso_skb: *mut sk_buff,
                         mut seq: u32, mss: u32) {
    let flags = unsafe { (*skb_shinfo(gso_skb)).tx_flags & SKBTX_ANY_TSTAMP };
    let ts_seq = unsafe { (*skb_shinfo(gso_skb)).tskey };
    while !skb.is_null() {
        if before(ts_seq, seq.wrapping_add(mss)) {
            unsafe {
                (*skb_shinfo(skb)).tx_flags |= flags;
                (*skb_shinfo(skb)).tskey = ts_seq;
            }
            return;
        }
        skb = unsafe { (*skb).next };
        seq = seq.wrapping_add(mss);
    }
}

unsafe fn __tcpv4_gso_segment_csum(seg: *mut sk_buff, oldip: *mut __be32,
                                   newip: __be32, oldport: *mut __be16,
                                   newport: __be16) {
    if unsafe { *oldip == newip && *oldport == newport } { return; }
    let th = tcp_hdr(seg);
    let iph = ip_hdr(seg);
    unsafe {
        inet_proto_csum_replace4(&mut (*th).check, seg, *oldip, newip, true);
        inet_proto_csum_replace2(&mut (*th).check, seg, *oldport, newport, false);
        *oldport = newport;
        csum_replace4(&mut (*iph).check, *oldip, newip);
        *oldip = newip;
    }
}

unsafe fn __tcpv4_gso_segment_list_csum(segs: *mut sk_buff) -> *mut sk_buff {
    let seg = segs;
    let th = tcp_hdr(seg);
    let iph = ip_hdr(seg);
    let th2 = tcp_hdr(unsafe { (*seg).next });
    let iph2 = ip_hdr(unsafe { (*seg).next });
    if unsafe { (*(th as *const u32) ^ *(th2 as *const u32)) == 0 && (*iph).daddr == (*iph2).daddr && (*iph).saddr == (*iph2).saddr } { return segs; }
    let mut seg = unsafe { (*seg).next };
    while !seg.is_null() {
        let th2 = tcp_hdr(seg); let iph2 = ip_hdr(seg);
        __tcpv4_gso_segment_csum(seg, unsafe { &mut (*iph2).saddr }, unsafe { (*iph).saddr }, unsafe { &mut (*th2).source }, unsafe { (*th).source });
        __tcpv4_gso_segment_csum(seg, unsafe { &mut (*iph2).daddr }, unsafe { (*iph).daddr }, unsafe { &mut (*th2).dest }, unsafe { (*th).dest });
        seg = unsafe { (*seg).next };
    }
    segs
}

unsafe fn __tcp4_gso_segment_list(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let skb = skb_segment_list(skb, features, skb_mac_header_len(skb));
    if IS_ERR(skb) { return skb; }
    __tcpv4_gso_segment_list_csum(skb)
}

unsafe fn tcp4_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    if (*skb_shinfo(skb)).gso_type & SKB_GSO_TCPV4 == 0 || !pskb_may_pull(skb, core::mem::size_of::<tcphdr>()) { return ERR_PTR(-EINVAL); }
    if (*skb_shinfo(skb)).gso_type & SKB_GSO_FRAGLIST != 0 {
        let th = tcp_hdr(skb);
        if skb_pagelen(skb) - ((*th).doff as u32) * 4 == (*skb_shinfo(skb)).gso_size && (*skb_shinfo(skb)).gso_type & SKB_GSO_DODGY == 0 { return __tcp4_gso_segment_list(skb, features); }
        (*skb).ip_summed = CHECKSUM_NONE;
    }
    if (*skb).ip_summed != CHECKSUM_PARTIAL {
        let iph = ip_hdr(skb); let th = tcp_hdr(skb);
        (*th).check = 0; (*skb).ip_summed = CHECKSUM_PARTIAL;
        __tcp_v4_send_check(skb, (*iph).saddr, (*iph).daddr);
    }
    tcp_gso_segment(skb, features)
}

pub unsafe fn tcp_gso_segment(mut skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    let mut segs = ERR_PTR(-EINVAL); let mut sum_truesize = 0u32;
    let gso_skb = skb; let th = tcp_hdr(skb); let thlen = (*th).doff as u32 * 4;
    if thlen < core::mem::size_of::<tcphdr>() as u32 || skb_checksum_start(skb) != skb_transport_header(skb) || !pskb_may_pull(skb, thlen) { return segs; }
    let oldlen = !(*skb).len; __skb_pull(skb, thlen);
    let mss = (*skb_shinfo(skb)).gso_size; if (*skb).len <= mss { return segs; }
    if skb_gso_ok(skb, features | NETIF_F_GSO_ROBUST) { (*skb_shinfo(skb)).gso_segs = DIV_ROUND_UP((*skb).len, mss); return core::ptr::null_mut(); }
    let copy_destructor = (*gso_skb).destructor == Some(tcp_wfree); let ooo_okay = (*gso_skb).ooo_okay; (*skb).ooo_okay = false;
    segs = skb_segment(skb, features); if IS_ERR(segs) { return segs; }
    (*segs).ooo_okay = ooo_okay;
    let mss = if skb_is_gso(segs) { mss * (*skb_shinfo(segs)).gso_segs } else { mss };
    let delta = htonl(oldlen.wrapping_add(thlen).wrapping_add(mss)); skb = segs;
    let th = tcp_hdr(skb); let mut seq = ntohl((*th).seq);
    if (*skb_shinfo(gso_skb)).tx_flags & SKBTX_ANY_TSTAMP != 0 { tcp_gso_tstamp(segs, gso_skb, seq, mss); }
    let newcheck = !csum_fold(csum_add(csum_unfold((*th).check), delta)); let ecn_cwr_mask = (*skb_shinfo(gso_skb)).gso_type & SKB_GSO_TCP_ACCECN != 0;
    while !(*skb).next.is_null() {
        (*th).fin = false; (*th).psh = false; (*th).check = newcheck;
        if (*skb).ip_summed == CHECKSUM_PARTIAL { gso_reset_checksum(skb, !(*th).check); } else { (*th).check = gso_make_checksum(skb, !(*th).check); }
        seq = seq.wrapping_add(mss); if copy_destructor { (*skb).destructor = (*gso_skb).destructor; (*skb).sk = (*gso_skb).sk; sum_truesize += (*skb).truesize; }
        skb = (*skb).next; let th = tcp_hdr(skb); (*th).seq = htonl(seq); (*th).cwr &= ecn_cwr_mask;
    }
    if copy_destructor { swap(&mut (*gso_skb).sk, &mut (*skb).sk); swap(&mut (*gso_skb).destructor, &mut (*skb).destructor); sum_truesize += (*skb).truesize; let delta = sum_truesize as i64 - (*gso_skb).truesize as i64; if delta >= 0 { refcount_add(delta as u32, &mut (*(*skb).sk).sk_wmem_alloc); } else { WARN_ON_ONCE(refcount_sub_and_test((-delta) as u32, &mut (*(*skb).sk).sk_wmem_alloc)); } }
    let delta = htonl(oldlen.wrapping_add((skb_tail_pointer(skb) as usize - skb_transport_header(skb) as usize) as u32).wrapping_add((*skb).data_len)); (*th).check = !csum_fold(csum_add(csum_unfold((*th).check), delta)); if (*skb).ip_summed == CHECKSUM_PARTIAL { gso_reset_checksum(skb, !(*th).check); } else { (*th).check = gso_make_checksum(skb, !(*th).check); }
    segs
}

pub unsafe fn tcp_gro_lookup(head: *mut list_head, th: *mut tcphdr) -> *mut sk_buff { let mut p: *mut sk_buff = core::ptr::null_mut(); list_for_each_entry(p, head, list) { if !NAPI_GRO_CB(p).same_flow { continue; } let th2 = tcp_hdr(p); if *(th as *const u32) ^ *(th2 as *const u32) != 0 { NAPI_GRO_CB(p).same_flow = false; continue; } return p; } core::ptr::null_mut() }

pub unsafe fn tcp_gro_receive(head: *mut list_head, skb: *mut sk_buff, th: *mut tcphdr) -> *mut sk_buff { let thlen = (*th).doff as u32 * 4; let mut pp = core::ptr::null_mut(); let len = skb_gro_len(skb); let flags = tcp_flag_word(th); let p = tcp_gro_lookup(head, th); if p.is_null() { goto_out_check_final!(skb, pp, len, 1, flags); } let th2 = tcp_hdr(p); let mut flush = ((flags ^ tcp_flag_word(th2)) & !(TCP_FLAG_FIN|TCP_FLAG_PSH)) as i32; flush |= ((*th).ack_seq ^ (*th2).ack_seq) as i32; let mut i = core::mem::size_of::<tcphdr>() as u32; while i < thlen { flush |= *((th as *mut u8).add(i as usize) as *mut u32) as i32 ^ *((th2 as *mut u8).add(i as usize) as *mut u32) as i32; i += 4; } flush |= gro_receive_network_flush(th, th2, p); let mut mss = (*skb_shinfo(p)).gso_size; if skb_is_gso(skb) { flush |= (mss != (*skb_shinfo(skb)).gso_size) as i32; } else { flush |= ((len-1) >= mss) as i32; } flush |= ((ntohl((*th2).seq) + skb_gro_len(p)) ^ ntohl((*th).seq)) as i32; flush |= skb_cmp_decrypted(p, skb) as i32; if NAPI_GRO_CB(p).is_flist { flush |= (flags ^ tcp_flag_word(th2)) as i32; flush |= ((*skb).ip_summed != (*p).ip_summed) as i32; flush |= ((*skb).csum_level != (*p).csum_level) as i32; flush |= (NAPI_GRO_CB(p).count >= 64) as i32; skb_set_network_header(skb, skb_gro_receive_network_offset(skb)); if flush != 0 || skb_gro_receive_list(p, skb) { mss = 1; } } else if flush != 0 || skb_gro_receive(p, skb) { mss = 1; } else { tcp_flag_word(th2) |= flags & (TCP_FLAG_FIN|TCP_FLAG_PSH); } let flush = if skb_is_gso(skb) { len != NAPI_GRO_CB(skb).count * (*skb_shinfo(skb)).gso_size } else { len < mss }; let flush = flush || (flags & (TCP_FLAG_URG|TCP_FLAG_PSH|TCP_FLAG_RST|TCP_FLAG_SYN|TCP_FLAG_FIN)) != 0; if !p.is_null() && (!NAPI_GRO_CB(skb).same_flow || flush) { pp = p; } NAPI_GRO_CB(skb).flush |= flush; pp }

pub unsafe fn tcp_gro_complete(skb: *mut sk_buff) { let th = tcp_hdr(skb); if (*skb).encapsulation { (*skb).inner_transport_header = (*skb).transport_header; } (*skb).csum_start = th as *mut u8 as usize - (*skb).head as usize; (*skb).csum_offset = core::mem::offset_of!(tcphdr, check); (*skb).ip_summed = CHECKSUM_PARTIAL; (*skb_shinfo(skb)).gso_segs = NAPI_GRO_CB(skb).count; if (*th).cwr { (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCP_ACCECN; } }

pub unsafe fn tcp4_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff { if !NAPI_GRO_CB(skb).flush && skb_gro_checksum_validate(skb, IPPROTO_TCP, inet_gro_compute_pseudo) { NAPI_GRO_CB(skb).flush=true; return core::ptr::null_mut(); } let th=tcp_gro_pull_header(skb); if th.is_null() { NAPI_GRO_CB(skb).flush=true; return core::ptr::null_mut(); } tcp_gro_receive(head,skb,th) }
pub unsafe fn tcp4_gro_complete(skb: *mut sk_buff, thoff: i32) -> i32 { let offset=NAPI_GRO_CB(skb).network_offsets[(*skb).encapsulation as usize]; let iph=((*skb).data.add(offset as usize)) as *mut iphdr; let th=tcp_hdr(skb); if NAPI_GRO_CB(skb).is_flist { (*skb_shinfo(skb)).gso_type |= SKB_GSO_FRAGLIST|SKB_GSO_TCPV4; (*skb_shinfo(skb)).gso_segs=NAPI_GRO_CB(skb).count; __skb_incr_checksum_unnecessary(skb); return 0; } (*th).check= !tcp_v4_check((*skb).len-thoff as u32,(*iph).saddr,(*iph).daddr,0); (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV4 | (NAPI_GRO_CB(skb).ip_fixedid * SKB_GSO_TCP_FIXEDID); tcp_gro_complete(skb); 0 }

pub unsafe fn tcpv4_offload_init() -> i32 {
    net_hotdata.tcpv4_offload = net_offload {
        callbacks: net_offload_callbacks {
            gso_segment: Some(tcp4_gso_segment),
            gro_receive: Some(tcp4_gro_receive),
            gro_complete: Some(tcp4_gro_complete),
        },
    };
    inet_add_offload(&mut net_hotdata.tcpv4_offload, IPPROTO_TCP)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
