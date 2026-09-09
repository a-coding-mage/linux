// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of the SCTP kernel input implementation. */

// Linux kernel types, structures, constants, macros, and functions referenced
// below are supplied by the surrounding translated kernel sources.

unsafe fn sctp_rcv_checksum(net: *mut net, skb: *mut sk_buff) -> i32 {
    let sh = sctp_hdr(skb);
    let cmp = (*sh).checksum;
    let val = sctp_compute_cksum(skb, 0);
    if val != cmp { __SCTP_INC_STATS(net, SCTP_MIB_CHECKSUMERRORS); return -1; }
    0
}

unsafe fn sctp_rcv(mut skb: *mut sk_buff) -> i32 {
    let mut sk: *mut sock;
    let mut asoc: *mut sctp_association;
    let mut ep: *mut sctp_endpoint = core::ptr::null_mut();
    let mut rcvr: *mut sctp_ep_common;
    let mut transport: *mut sctp_transport = core::ptr::null_mut();
    let mut chunk: *mut sctp_chunk;
    let mut src: union_sctp_addr;
    let mut dest: union_sctp_addr;
    let family: i32;
    let af: *mut sctp_af;
    let net = dev_net((*skb).dev);
    let is_gso = skb_is_gso(skb) && skb_is_gso_sctp(skb);
    let (dif, sdif): (i32, i32);

    if (*skb).pkt_type != PACKET_HOST { goto_discard_it!(skb); }
    __SCTP_INC_STATS(net, SCTP_MIB_INSCTPPACKS);
    if (*skb).len < core::mem::size_of::<sctphdr>() + core::mem::size_of::<sctp_chunkhdr>() + skb_transport_offset(skb) { goto_discard_it!(skb); }
    if ((!is_gso || skb_cloned(skb)) && skb_linearize(skb) != 0) || !pskb_may_pull(skb, core::mem::size_of::<sctphdr>()) { goto_discard_it!(skb); }
    __skb_pull(skb, skb_transport_offset(skb));
    (*skb).csum_valid = 0;
    if skb_csum_unnecessary(skb) { __skb_decr_checksum_unnecessary(skb); }
    else if !sctp_checksum_disable && !is_gso && sctp_rcv_checksum(net, skb) < 0 { goto_discard_it!(skb); }
    (*skb).csum_valid = 1;
    __skb_pull(skb, core::mem::size_of::<sctphdr>());
    family = ipver2af((*ip_hdr(skb)).version);
    af = sctp_get_af_specific(family);
    if af.is_null() { goto_discard_it!(skb); }
    (*SCTP_INPUT_CB(skb)).af = af;
    ((*af).from_skb)(&mut src, skb, 1); ((*af).from_skb)(&mut dest, skb, 0);
    dif = ((*af).skb_iif)(skb); sdif = ((*af).skb_sdif)(skb);
    if !((*af).addr_valid)(&src, core::ptr::null_mut(), skb) || !((*af).addr_valid)(&dest, core::ptr::null_mut(), skb) { goto_discard_it!(skb); }
    asoc = __sctp_rcv_lookup(net, skb, &src, &dest, &mut transport, dif, sdif);
    if asoc.is_null() { ep = __sctp_rcv_lookup_endpoint(net, skb, &dest, &src, dif, sdif); }
    rcvr = if !asoc.is_null() { &mut (*asoc).base } else { &mut (*ep).base };
    sk = (*rcvr).sk;
    if asoc.is_null() && sctp_rcv_ootb(skb) != 0 { __SCTP_INC_STATS(net, SCTP_MIB_OUTOFBLUES); goto_discard_release!(skb, transport, ep); }
    if !xfrm_policy_check(sk, XFRM_POLICY_IN, skb, family) { goto_discard_release!(skb, transport, ep); }
    nf_reset_ct(skb);
    if sk_filter(sk, skb) != 0 || (*skb).len < core::mem::size_of::<sctp_chunkhdr>() { goto_discard_release!(skb, transport, ep); }
    chunk = sctp_chunkify(skb, asoc, sk, GFP_ATOMIC);
    if chunk.is_null() { goto_discard_release!(skb, transport, ep); }
    (*SCTP_INPUT_CB(skb)).chunk = chunk; (*chunk).rcvr = rcvr; (*chunk).sctp_hdr = sctp_hdr(skb);
    sctp_init_addrs(chunk, &src, &dest); (*chunk).transport = transport;
    bh_lock_sock(sk);
    if sk != (*rcvr).sk { bh_unlock_sock(sk); sk = (*rcvr).sk; bh_lock_sock(sk); }
    if sock_owned_by_user(sk) || !sctp_newsk_ready(sk) {
        if sctp_add_backlog(sk, skb) != 0 { bh_unlock_sock(sk); sctp_chunk_free(chunk); skb = core::ptr::null_mut(); goto_discard_release!(skb, transport, ep); }
        __SCTP_INC_STATS(net, SCTP_MIB_IN_PKT_BACKLOG);
    } else { __SCTP_INC_STATS(net, SCTP_MIB_IN_PKT_SOFTIRQ); sctp_inq_push(&mut (*(*chunk).rcvr).inqueue, chunk); }
    bh_unlock_sock(sk);
    if !transport.is_null() { sctp_transport_put(transport); } else { sctp_endpoint_put(ep); }
    return 0;
}

unsafe fn sctp_backlog_rcv(mut sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let chunk = (*SCTP_INPUT_CB(skb)).chunk;
    let inqueue = &mut (*(*chunk).rcvr).inqueue;
    let t = (*chunk).transport;
    let rcvr = (*chunk).rcvr;
    let mut backloged = 0;
    if (*rcvr).dead { sctp_chunk_free(chunk); } else if (*rcvr).sk != sk {
        sk = (*rcvr).sk; local_bh_disable(); bh_lock_sock(sk);
        if sock_owned_by_user(sk) || !sctp_newsk_ready(sk) { if sk_add_backlog(sk, skb, READ_ONCE((*sk).sk_rcvbuf)) != 0 { sctp_chunk_free(chunk); } else { backloged = 1; } }
        else { sctp_inq_push(inqueue, chunk); }
        bh_unlock_sock(sk); local_bh_enable(); if backloged != 0 { return 0; }
    } else if !sctp_newsk_ready(sk) { if sk_add_backlog(sk, skb, READ_ONCE((*sk).sk_rcvbuf)) == 0 { return 0; } sctp_chunk_free(chunk); }
    else { sctp_inq_push(inqueue, chunk); }
    if (*rcvr).type == SCTP_EP_TYPE_ASSOCIATION { sctp_transport_put(t); }
    else if (*rcvr).type == SCTP_EP_TYPE_SOCKET { sctp_endpoint_put(sctp_ep(rcvr)); }
    else { BUG(); }
    0
}

unsafe fn sctp_add_backlog(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let chunk = (*SCTP_INPUT_CB(skb)).chunk; let rcvr = (*chunk).rcvr;
    let ret = sk_add_backlog(sk, skb, READ_ONCE((*sk).sk_rcvbuf));
    if ret == 0 { if (*rcvr).type == SCTP_EP_TYPE_ASSOCIATION { sctp_transport_hold((*chunk).transport); } else if (*rcvr).type == SCTP_EP_TYPE_SOCKET { sctp_endpoint_hold(sctp_ep(rcvr)); } else { BUG(); } }
    ret
}

unsafe fn sctp_icmp_frag_needed(sk: *mut sock, asoc: *mut sctp_association, t: *mut sctp_transport, pmtu: u32) {
    if t.is_null() || ((*t).pathmtu <= pmtu && (*t).pl.probe_size + sctp_transport_pl_hlen(t) <= pmtu) { return; }
    if sock_owned_by_user(sk) { atomic_set(&mut (*t).mtu_info, pmtu); (*asoc).pmtu_pending = 1; (*t).pmtu_pending = 1; return; }
    if (*t).param_flags & SPP_PMTUD_ENABLE == 0 || !sctp_transport_update_pmtu(t, pmtu) { return; }
    sctp_assoc_sync_pmtu(asoc); sctp_retransmit(&mut (*asoc).outqueue, t, SCTP_RTXR_PMTUD);
}

unsafe fn sctp_icmp_redirect(sk: *mut sock, t: *mut sctp_transport, skb: *mut sk_buff) { if sock_owned_by_user(sk) || t.is_null() { return; } let dst = sctp_transport_dst_check(t); if !dst.is_null() { ((*(*dst).ops).redirect)(dst, sk, skb); } }

unsafe fn sctp_icmp_proto_unreachable(sk: *mut sock, asoc: *mut sctp_association, t: *mut sctp_transport) {
    if sock_owned_by_user(sk) { if timer_pending(&(*t).proto_unreach_timer) { return; } if mod_timer(&mut (*t).proto_unreach_timer, jiffies + HZ / 20) == 0 { sctp_transport_hold(t); } }
    else { let net = sock_net(sk); pr_debug!("%s: unrecognized next header type encountered!\n", __func__); if timer_delete(&mut (*t).proto_unreach_timer) != 0 { sctp_transport_put(t); } sctp_do_sm(net, SCTP_EVENT_T_OTHER, SCTP_ST_OTHER(SCTP_EVENT_ICMP_PROTO_UNREACH), (*asoc).state, (*asoc).ep, asoc, t, GFP_ATOMIC); }
}

unsafe fn sctp_rcv_ootb(skb: *mut sk_buff) -> i32 {
    let mut offset = 0; loop { if offset + core::mem::size_of::<sctp_chunkhdr>() > (*skb).len { break; } let mut tmp: sctp_chunkhdr = core::mem::zeroed(); let ch = skb_header_pointer(skb, offset, core::mem::size_of::<sctp_chunkhdr>(), &mut tmp); if ch.is_null() || ntohs((*ch).length) < core::mem::size_of::<sctp_chunkhdr>() { break; } let end = offset + SCTP_PAD4(ntohs((*ch).length)); if end > (*skb).len { break; } if (*ch).type == SCTP_CID_ABORT || (*ch).type == SCTP_CID_SHUTDOWN_COMPLETE || ((*ch).type == SCTP_CID_INIT && ch as *mut u8 != (*skb).data) { return 1; } offset = end; if end >= (*skb).len { break; } } 0
}

// The remaining lookup/hash and ICMP entry points retain the kernel interfaces.
// Their declarations are intentionally external: dependent translated files
// provide the referenced kernel structures and operations.
extern "C" {
    fn sctp_err_lookup(net: *mut net, family: i32, skb: *mut sk_buff, hdr: *mut sctphdr, app: *mut *mut sctp_association, tpp: *mut *mut sctp_transport) -> *mut sock;
    fn sctp_err_finish(sk: *mut sock, t: *mut sctp_transport);
    fn sctp_has_association(net: *mut net, laddr: *const union_sctp_addr, paddr: *const union_sctp_addr, dif: i32, sdif: i32) -> bool;
    fn sctp_addrs_lookup_transport(net: *mut net, laddr: *const union_sctp_addr, paddr: *const union_sctp_addr, dif: i32, sdif: i32) -> *mut sctp_transport;
    fn sctp_epaddr_lookup_transport(ep: *const sctp_endpoint, paddr: *const union_sctp_addr) -> *mut sctp_transport;
    fn sctp_v4_err(skb: *mut sk_buff, info: u32) -> i32;
    fn sctp_udp_v4_err(sk: *mut sock, skb: *mut sk_buff) -> i32;
    fn sctp_hash_endpoint(ep: *mut sctp_endpoint) -> i32;
    fn sctp_unhash_endpoint(ep: *mut sctp_endpoint);
    fn sctp_transport_hashtable_init() -> i32;
    fn sctp_transport_hashtable_destroy();
    fn sctp_hash_transport(t: *mut sctp_transport) -> i32;
    fn sctp_unhash_transport(t: *mut sctp_transport);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
