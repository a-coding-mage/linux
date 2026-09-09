// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation. Direct Rust translation of output.c. */

// Kernel and SCTP dependencies are supplied by the surrounding translation unit.

unsafe fn sctp_packet_reset(packet: *mut sctp_packet) {
    (*packet).size = (*packet).overhead;
    (*packet).has_cookie_echo = 0;
    (*packet).has_sack = 0;
    (*packet).has_data = 0;
    (*packet).has_auth = 0;
    (*packet).ipfragok = 0;
    (*packet).auth = core::ptr::null_mut();
}

pub unsafe fn sctp_packet_config(packet: *mut sctp_packet, vtag: u32, ecn_capable: i32) {
    let tp = (*packet).transport;
    let asoc = (*tp).asoc;
    let mut sp: *mut sctp_sock = core::ptr::null_mut();
    let mut sk: *mut sock;
    (*packet).vtag = vtag;
    if !sctp_packet_empty(packet) { return; }
    (*packet).max_size = (*tp).pathmtu;
    if !asoc.is_null() { sk = (*asoc).base.sk; sp = sctp_sk(sk); }
    (*packet).overhead = sctp_mtu_payload(sp, 0, 0);
    (*packet).size = (*packet).overhead;
    if asoc.is_null() { return; }
    if !sctp_transport_dst_check(tp) {
        sctp_transport_route(tp, core::ptr::null_mut(), sp);
        if (*asoc).param_flags & SPP_PMTUD_ENABLE != 0 { sctp_assoc_sync_pmtu(asoc); }
    } else if !sctp_transport_pl_enabled(tp) && (*asoc).param_flags & SPP_PMTUD_ENABLE != 0 && !sctp_transport_pmtu_check(tp) { sctp_assoc_sync_pmtu(asoc); }
    if (*asoc).pmtu_pending {
        if (*asoc).param_flags & SPP_PMTUD_ENABLE != 0 { sctp_assoc_sync_pmtu(asoc); }
        (*asoc).pmtu_pending = 0;
    }
    if ecn_capable != 0 {
        let chunk = sctp_get_ecne_prepend(asoc);
        if !chunk.is_null() { sctp_packet_append_chunk(packet, chunk); }
    }
    if (*tp).dst.is_null() { return; }
    rcu_read_lock();
    if __sk_dst_get(sk) != (*tp).dst { dst_hold((*tp).dst); sk_setup_caps(sk, (*tp).dst); }
    (*packet).max_size = if sk_can_gso(sk) { core::cmp::min(read_once((*(*tp).dst).dev.gso_max_size), GSO_LEGACY_MAX_SIZE) } else { (*asoc).pathmtu };
    rcu_read_unlock();
}

pub unsafe fn sctp_packet_init(packet: *mut sctp_packet, transport: *mut sctp_transport, sport: u16, dport: u16) {
    (*packet).transport = transport; (*packet).source_port = sport; (*packet).destination_port = dport;
    INIT_LIST_HEAD(&mut (*packet).chunk_list); (*packet).overhead = 0; sctp_packet_reset(packet); (*packet).vtag = 0;
}

pub unsafe fn sctp_packet_free(packet: *mut sctp_packet) {
    let mut chunk: *mut sctp_chunk = core::ptr::null_mut(); let mut tmp: *mut sctp_chunk = core::ptr::null_mut();
    list_for_each_entry_safe!(chunk, tmp, &mut (*packet).chunk_list, list, { list_del_init(&mut (*chunk).list); sctp_chunk_free(chunk); });
}

pub unsafe fn sctp_packet_transmit_chunk(packet: *mut sctp_packet, chunk: *mut sctp_chunk, one_packet: i32, gfp: gfp_t) -> sctp_xmit {
    let mut retval = sctp_packet_append_chunk(packet, chunk);
    if retval == SCTP_XMIT_PMTU_FULL && (*packet).has_cookie_echo == 0 {
        let error = sctp_packet_transmit(packet, gfp);
        if error < 0 { (*(*chunk).skb).sk.sk_err = -error; }
        if one_packet == 0 { retval = sctp_packet_append_chunk(packet, chunk); }
    }
    retval
}

unsafe fn sctp_packet_bundle_pad(pkt: *mut sctp_packet, chunk: *mut sctp_chunk) -> sctp_xmit {
    let t = (*pkt).transport; if !(*chunk).pmtu_probe { return SCTP_XMIT_OK; }
    let overhead = core::mem::size_of::<sctphdr>() + core::mem::size_of::<sctp_chunkhdr>() + core::mem::size_of::<sctp_sender_hb_info>() + core::mem::size_of::<sctp_pad_chunk>();
    let pad = sctp_make_pad((*t).asoc, (*t).pl.probe_size - overhead); if pad.is_null() { return SCTP_XMIT_DELAY; }
    list_add_tail(&mut (*pad).list, &mut (*pkt).chunk_list); (*pkt).size += SCTP_PAD4(ntohs((*pad).chunk_hdr).length); (*chunk).transport = t; SCTP_XMIT_OK
}

unsafe fn sctp_packet_bundle_auth(pkt: *mut sctp_packet, chunk: *mut sctp_chunk) -> sctp_xmit {
    let asoc = (*(*pkt).transport).asoc; if asoc.is_null() || (*chunk).chunk_hdr.type_ == SCTP_CID_AUTH || (*pkt).has_auth != 0 || !(*chunk).auth { return SCTP_XMIT_OK; }
    let auth = sctp_make_auth(asoc, (*(*chunk).shkey).key_id); if auth.is_null() { return SCTP_XMIT_OK; }
    (*auth).shkey = (*chunk).shkey; sctp_auth_shkey_hold((*auth).shkey);
    let ret = __sctp_packet_append_chunk(pkt, auth); if ret != SCTP_XMIT_OK { sctp_chunk_free(auth); } ret
}

unsafe fn sctp_packet_bundle_sack(pkt: *mut sctp_packet, chunk: *mut sctp_chunk) -> sctp_xmit {
    if !sctp_chunk_is_data(chunk) || (*pkt).has_sack != 0 || (*pkt).has_cookie_echo != 0 { return SCTP_XMIT_OK; }
    let asoc = (*(*pkt).transport).asoc; let timer = &mut (*asoc).timers[SCTP_EVENT_TIMEOUT_SACK];
    if timer_pending(timer) {
        if (*(*pkt).transport).sack_generation != (*asoc).peer.sack_generation { return SCTP_XMIT_OK; }
        (*asoc).a_rwnd = (*asoc).rwnd; let sack = sctp_make_sack(asoc);
        if !sack.is_null() { let ret = __sctp_packet_append_chunk(pkt, sack); if ret != SCTP_XMIT_OK { sctp_chunk_free(sack); return ret; } (*asoc).peer.sack_needed = 0; if timer_delete(timer) { sctp_association_put(asoc); } }
    } SCTP_XMIT_OK
}

unsafe fn __sctp_packet_append_chunk(packet: *mut sctp_packet, chunk: *mut sctp_chunk) -> sctp_xmit {
    let len = SCTP_PAD4(ntohs((*chunk).chunk_hdr.length)); let ret = sctp_packet_will_fit(packet, chunk, len); if ret != SCTP_XMIT_OK { return ret; }
    match (*chunk).chunk_hdr.type_ { SCTP_CID_DATA | SCTP_CID_I_DATA => { sctp_packet_append_data(packet, chunk); (*packet).has_sack=1; (*packet).has_auth=1; (*packet).has_data=1; (*chunk).sent_at=jiffies; (*chunk).sent_count += 1; }, SCTP_CID_COOKIE_ECHO => (*packet).has_cookie_echo=1, SCTP_CID_SACK => { (*packet).has_sack=1; }, SCTP_CID_AUTH => { (*packet).has_auth=1; (*packet).auth=chunk; }, _ => {} }
    list_add_tail(&mut (*chunk).list, &mut (*packet).chunk_list); (*packet).size += len; (*chunk).transport=(*packet).transport; SCTP_XMIT_OK
}

pub unsafe fn sctp_packet_append_chunk(packet: *mut sctp_packet, chunk: *mut sctp_chunk) -> sctp_xmit {
    if sctp_chunk_is_data(chunk) { let r=sctp_packet_can_append_data(packet,chunk); if r!=SCTP_XMIT_OK{return r;} }
    let r=sctp_packet_bundle_auth(packet,chunk); if r!=SCTP_XMIT_OK{return r;} let r=sctp_packet_bundle_sack(packet,chunk); if r!=SCTP_XMIT_OK{return r;} let r=__sctp_packet_append_chunk(packet,chunk); if r!=SCTP_XMIT_OK{return r;} sctp_packet_bundle_pad(packet,chunk)
}

unsafe fn sctp_packet_can_append_data(packet:*mut sctp_packet, chunk:*mut sctp_chunk)->sctp_xmit { let transport=(*packet).transport; let asoc=(*transport).asoc; let q=&mut (*asoc).outqueue; let rwnd=(*asoc).peer.rwnd; let inflight=q.outstanding_bytes; let flight=(*transport).flight_size; let data=sctp_data_size(chunk); if data>rwnd&&inflight>0{return SCTP_XMIT_RWND_FULL;} if (*chunk).fast_retransmit!=SCTP_NEED_FRTX&&flight>=(*transport).cwnd{return SCTP_XMIT_RWND_FULL;} if (sctp_sk((*asoc).base.sk).nodelay||inflight==0)&&!(*asoc).force_delay{return SCTP_XMIT_OK;} if !sctp_packet_empty(packet)||!sctp_state(asoc,ESTABLISHED)||!(*chunk).msg.can_delay{return SCTP_XMIT_OK;} if (*chunk).skb.len+q.out_qlen> (*transport).pathmtu-(*packet).overhead-sctp_datachk_len(&(*chunk).asoc.stream)-4{return SCTP_XMIT_OK;} SCTP_XMIT_DELAY }

unsafe fn sctp_packet_append_data(packet:*mut sctp_packet, chunk:*mut sctp_chunk){let t=(*packet).transport;let a=(*t).asoc;let n=sctp_data_size(chunk);(*t).flight_size+=n;(*a).outqueue.outstanding_bytes+=n;(*a).peer.rwnd=(*a).peer.rwnd.saturating_sub(n);sctp_chunk_assign_tsn(chunk);(*a).stream.si.assign_number(chunk);}

unsafe fn sctp_packet_will_fit(packet:*mut sctp_packet, chunk:*mut sctp_chunk, len:u16)->sctp_xmit {if ((*packet).auth&&!core::ptr::eq((*chunk).shkey,(*(*packet).auth).shkey))||((*packet).auth.is_null()&&(*chunk).shkey&&!((*chunk).chunk_hdr.type_==SCTP_CID_AUTH)){return SCTP_XMIT_PMTU_FULL;}let p=(*packet).size;let mtu=if !(*packet).transport.asoc.is_null(){(*(*packet).transport).asoc.pathmtu}else{(*(*packet).transport).pathmtu};if p+len as usize>mtu{if sctp_packet_empty(packet)||(!(*packet).has_data&&(*chunk).auth){(*packet).ipfragok=1;return SCTP_XMIT_OK;}let mut max=mtu-(*packet).overhead;if !(*packet).auth.is_null(){max-=SCTP_PAD4((*(*packet).auth).skb.len);}if len as usize>max||(!sctp_chunk_is_data(chunk)&&(*packet).has_data!=0)||p+len as usize>(*packet).max_size{return SCTP_XMIT_PMTU_FULL;}}SCTP_XMIT_OK}

unsafe fn sctp_packet_gso_append(head: *mut sk_buff, skb: *mut sk_buff) {
    if SCTP_OUTPUT_CB(head).last == head { skb_shinfo(head).frag_list = skb; } else { (*SCTP_OUTPUT_CB(head).last).next = skb; }
    SCTP_OUTPUT_CB(head).last = skb; (*head).truesize += (*skb).truesize; (*head).data_len += (*skb).len; (*head).len += (*skb).len;
    refcount_add((*skb).truesize, &mut (*(*head).sk).sk_wmem_alloc); __skb_header_release(skb);
}

unsafe fn sctp_packet_pack(packet: *mut sctp_packet, head: *mut sk_buff, gso: i32, gfp: gfp_t) -> i32 {
    let tp=(*packet).transport; let mut count=0; if gso!=0 { skb_shinfo(head).gso_type=(*(*head).sk).sk_gso_type; SCTP_OUTPUT_CB(head).last=head; }
    while !list_empty(&(*packet).chunk_list) {
        let chunk=list_first_entry!(&mut (*packet).chunk_list, sctp_chunk, list); list_del_init(&mut (*chunk).list);
        if sctp_chunk_is_data(chunk) && !sctp_chunk_retransmitted(chunk) && !(*tp).rto_pending {(*chunk).rtt_in_progress=1;(*tp).rto_pending=1;}
        let padding=SCTP_PAD4((*chunk).skb.len)-(*chunk).skb.len; if padding!=0 {skb_put_zero((*chunk).skb,padding);}
        skb_put_data(head,(*chunk).skb.data,(*chunk).skb.len); if !sctp_chunk_is_data(chunk){sctp_chunk_free(chunk);} count+=1;
    }
    if gso!=0 {sctp_packet_gso_append(head,head); skb_shinfo(head).gso_segs=count; skb_shinfo(head).gso_size=GSO_BY_FRAGS;} else if !sctp_checksum_disable {(*head).ip_summed=CHECKSUM_PARTIAL;(*head).csum_not_inet=1;}
    let _=gfp; count
}

pub unsafe fn sctp_packet_transmit(packet:*mut sctp_packet, gfp:gfp_t)->i32 {
    let tp=(*packet).transport; if list_empty(&(*packet).chunk_list){return 0;} let chunk=list_first_entry!(&mut (*packet).chunk_list,sctp_chunk,list); let sk=(*chunk).skb.sk; let gso=if (*packet).size>(*tp).pathmtu&&!(*packet).ipfragok&&!(*chunk).pmtu_probe {if sk_can_gso(sk){1}else{0}}else{0};
    let head=alloc_skb((if gso!=0{(*packet).overhead}else{(*packet).size})+MAX_HEADER,gfp); if head.is_null(){sctp_packet_reset(packet);return 0;} skb_reserve(head,(*packet).overhead+MAX_HEADER);skb_set_owner_w(head,sk);let sh=skb_push(head,core::mem::size_of::<sctphdr>());skb_reset_transport_header(head);(*sh).source=htons((*packet).source_port);(*sh).dest=htons((*packet).destination_port);(*sh).vtag=htonl((*packet).vtag);(*sh).checksum=0;let n=sctp_packet_pack(packet,head,gso,gfp);if n>0{(*tp).af_specific.ecn_capable(sk);(*head).ignore_df=(*packet).ipfragok;(*tp).af_specific.sctp_xmit(head,tp);}else{kfree_skb(head);}sctp_packet_reset(packet);0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
