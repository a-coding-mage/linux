// SPDX-License-Identifier: GPL-2.0-or-later
// Direct low-level translation of sctp/ulpevent.c. Kernel types and helpers
// are supplied by the surrounding SCTP implementation.

unsafe fn sctp_ulpevent_init(event: *mut sctp_ulpevent, msg_flags: u16, len: c_uint) {
    memset(event as *mut c_void, 0, core::mem::size_of::<sctp_ulpevent>());
    (*event).msg_flags = msg_flags;
    (*event).rmem_len = len;
}

unsafe fn sctp_ulpevent_new(size: c_int, msg_flags: u16, gfp: gfp_t) -> *mut sctp_ulpevent {
    let skb = alloc_skb(size, gfp);
    if skb.is_null() { return core::ptr::null_mut(); }
    let event = sctp_skb2event(skb);
    sctp_ulpevent_init(event, msg_flags, (*skb).truesize);
    event
}

#[no_mangle]
pub unsafe extern "C" fn sctp_ulpevent_is_notification(event: *const sctp_ulpevent) -> c_int {
    if MSG_NOTIFICATION == ((*event).msg_flags & MSG_NOTIFICATION) { 1 } else { 0 }
}

unsafe fn sctp_ulpevent_set_owner(event: *mut sctp_ulpevent, asoc: *const sctp_association) {
    let chunk = (*event).chunk;
    sctp_association_hold(asoc as *mut sctp_association);
    let skb = sctp_event2skb(event);
    (*event).asoc = asoc as *mut sctp_association;
    atomic_add((*event).rmem_len, &mut (*(*event).asoc).rmem_alloc);
    sctp_skb_set_owner_r(skb, (*asoc).base.sk);
    if !chunk.is_null() && !(*chunk).head_skb.is_null() && (*(*chunk).head_skb).sk.is_null() {
        (*(*chunk).head_skb).sk = (*asoc).base.sk;
    }
}

unsafe fn sctp_ulpevent_release_owner(event: *mut sctp_ulpevent) {
    let asoc = (*event).asoc;
    atomic_sub((*event).rmem_len, &mut (*asoc).rmem_alloc);
    sctp_association_put(asoc);
}

#[no_mangle]
pub unsafe extern "C" fn sctp_ulpevent_make_assoc_change(asoc: *const sctp_association, _flags: u16, state: u16, error: u16, outbound: u16, inbound: u16, chunk: *mut sctp_chunk, gfp: gfp_t) -> *mut sctp_ulpevent {
    let (event, skb, sac) = if !chunk.is_null() {
        let skb = skb_copy_expand((*chunk).skb, core::mem::size_of::<sctp_assoc_change>() as c_int, 0, gfp);
        if skb.is_null() { return core::ptr::null_mut(); }
        let event = sctp_skb2event(skb); sctp_ulpevent_init(event, MSG_NOTIFICATION, (*skb).truesize);
        let sac = skb_push(skb, core::mem::size_of::<sctp_assoc_change>());
        skb_trim(skb, core::mem::size_of::<sctp_assoc_change>() as u32 + ntohs((*(*chunk).chunk_hdr).length) as u32 - core::mem::size_of::<sctp_chunkhdr>() as u32);
        (event, skb, sac)
    } else {
        let event = sctp_ulpevent_new(core::mem::size_of::<sctp_assoc_change>() as c_int, MSG_NOTIFICATION, gfp);
        if event.is_null() { return core::ptr::null_mut(); }
        let skb = sctp_event2skb(event); let sac = skb_put(skb, core::mem::size_of::<sctp_assoc_change>()); (event, skb, sac)
    };
    (*sac).sac_type = SCTP_ASSOC_CHANGE; (*sac).sac_state = state; (*sac).sac_flags = 0;
    (*sac).sac_length = (*skb).len; (*sac).sac_error = error;
    (*sac).sac_outbound_streams = outbound; (*sac).sac_inbound_streams = inbound;
    sctp_ulpevent_set_owner(event, asoc); (*sac).sac_assoc_id = sctp_assoc2id(asoc); event
}

unsafe fn sctp_ulpevent_make_peer_addr_change(asoc: *const sctp_association, aaddr: *const sockaddr_storage, _flags: c_int, state: c_int, error: c_int, gfp: gfp_t) -> *mut sctp_ulpevent {
    let event = sctp_ulpevent_new(core::mem::size_of::<sctp_paddr_change>() as c_int, MSG_NOTIFICATION, gfp);
    if event.is_null() { return core::ptr::null_mut(); }
    let skb = sctp_event2skb(event); let spc = skb_put(skb, core::mem::size_of::<sctp_paddr_change>());
    (*spc).spc_type = SCTP_PEER_ADDR_CHANGE; (*spc).spc_length = core::mem::size_of::<sctp_paddr_change>() as u32; (*spc).spc_flags = 0; (*spc).spc_state = state; (*spc).spc_error = error;
    sctp_ulpevent_set_owner(event, asoc); (*spc).spc_assoc_id = sctp_assoc2id(asoc);
    memcpy(&mut (*spc).spc_aaddr as *mut _ as *mut c_void, aaddr as *const c_void, core::mem::size_of::<sockaddr_storage>());
    ((*sctp_get_pf_specific((*asoc).base.sk).addr_to_user))((*asoc).base.sk, &mut (*spc).spc_aaddr as *mut _ as *mut sctp_addr); event
}

#[no_mangle]
pub unsafe extern "C" fn sctp_ulpevent_notify_peer_addr_change(transport: *mut sctp_transport, state: c_int, error: c_int) {
    let asoc = (*transport).asoc; if (*asoc).state < SCTP_STATE_ESTABLISHED { return; }
    let mut addr: sockaddr_storage = core::mem::zeroed(); memcpy(&mut addr as *mut _ as *mut c_void, &(*transport).ipaddr as *const _ as *const c_void, (*transport).af_specific.sockaddr_len as usize);
    let event = sctp_ulpevent_make_peer_addr_change(asoc, &addr, 0, state, error, GFP_ATOMIC);
    if !event.is_null() { ((*(*asoc).stream.si).enqueue_event)(&mut (*asoc).ulpq, event); }
}

unsafe fn sctp_ulpevent_make_simple(asoc: *const sctp_association, size: usize, typ: u16, gfp: gfp_t) -> *mut sctp_ulpevent {
    let event = sctp_ulpevent_new(size as c_int, MSG_NOTIFICATION, gfp); if event.is_null() { return event; }
    let skb = sctp_event2skb(event); let p = skb_put(skb, size) as *mut u8;
    *(p as *mut u16) = typ; *(p.add(2) as *mut u16) = 0; *(p.add(4) as *mut u32) = size as u32;
    sctp_ulpevent_set_owner(event, asoc); event
}

#[no_mangle]
pub unsafe extern "C" fn sctp_ulpevent_get_notification_type(event: *const sctp_ulpevent) -> u16 {
    let skb = sctp_event2skb(event); *( (*skb).data as *const u16)
}

unsafe fn sctp_ulpevent_receive_data(event: *mut sctp_ulpevent, asoc: *mut sctp_association) {
    let skb = sctp_event2skb(event); sctp_ulpevent_set_owner(event, asoc); sctp_assoc_rwnd_decrease(asoc, skb_headlen(skb));
    if (*skb).data_len == 0 { return; }
    let mut frag = core::ptr::null_mut(); skb_walk_frags(skb, &mut frag) { sctp_ulpevent_receive_data(sctp_skb2event(frag), asoc); }
}

unsafe fn sctp_ulpevent_release_data(event: *mut sctp_ulpevent) {
    let skb = sctp_event2skb(event); let len = (*skb).len;
    if (*skb).data_len != 0 { let mut frag = core::ptr::null_mut(); skb_walk_frags(skb, &mut frag) { sctp_ulpevent_release_frag_data(sctp_skb2event(frag)); } }
    sctp_assoc_rwnd_increase((*event).asoc, len); sctp_chunk_put((*event).chunk); sctp_ulpevent_release_owner(event);
}
unsafe fn sctp_ulpevent_release_frag_data(event: *mut sctp_ulpevent) {
    let skb = sctp_event2skb(event); if (*skb).data_len != 0 { let mut frag = core::ptr::null_mut(); skb_walk_frags(skb, &mut frag) { sctp_ulpevent_release_frag_data(sctp_skb2event(frag)); } }
    sctp_chunk_put((*event).chunk); sctp_ulpevent_release_owner(event);
}

#[no_mangle]
pub unsafe extern "C" fn sctp_ulpevent_free(event: *mut sctp_ulpevent) {
    if sctp_ulpevent_is_notification(event) != 0 { sctp_ulpevent_release_owner(event); } else { sctp_ulpevent_release_data(event); }
    kfree_skb(sctp_event2skb(event));
}

#[no_mangle]
pub unsafe extern "C" fn sctp_queue_purge_ulpevents(list: *mut sk_buff_head) -> c_uint {
    let mut unread = 0; loop { let skb = skb_dequeue(list); if skb.is_null() { break; } let event = sctp_skb2event(skb); if sctp_ulpevent_is_notification(event) == 0 { unread += (*skb).len; } sctp_ulpevent_free(event); } unread
}

#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_make_shutdown_event(a: *const sctp_association, _flags: u16, g: gfp_t) -> *mut sctp_ulpevent { sctp_ulpevent_make_simple(a, core::mem::size_of::<sctp_shutdown_event>(), SCTP_SHUTDOWN_EVENT, g) }
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_make_adaptation_indication(a: *const sctp_association, g: gfp_t) -> *mut sctp_ulpevent { let e=sctp_ulpevent_make_simple(a,core::mem::size_of::<sctp_adaptation_event>(),SCTP_ADAPTATION_INDICATION,g); if !e.is_null(){(*(skb_put(sctp_event2skb(e),0) as *mut sctp_adaptation_event)).sai_adaptation_ind=(*a).peer.adaptation_ind;} e }
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_make_sender_dry_event(a: *const sctp_association,g:gfp_t)->*mut sctp_ulpevent{sctp_ulpevent_make_simple(a,core::mem::size_of::<sctp_sender_dry_event>(),SCTP_SENDER_DRY_EVENT,g)}
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_make_assoc_reset_event(a:*const sctp_association,flags:u16,local:u32,remote:u32,g:gfp_t)->*mut sctp_ulpevent{let e=sctp_ulpevent_make_simple(a,core::mem::size_of::<sctp_assoc_reset_event>(),SCTP_ASSOC_RESET_EVENT,g);if !e.is_null(){let p=sctp_event2skb(e);let x=(*p).data as *mut sctp_assoc_reset_event;(*x).assocreset_flags=flags;(*x).assocreset_local_tsn=local;(*x).assocreset_remote_tsn=remote;}e}
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_make_stream_change_event(a:*const sctp_association,flags:u16,ins:u32,outs:u32,g:gfp_t)->*mut sctp_ulpevent{let e=sctp_ulpevent_make_simple(a,core::mem::size_of::<sctp_stream_change_event>(),SCTP_STREAM_CHANGE_EVENT,g);if !e.is_null(){let x=(*sctp_event2skb(e)).data as *mut sctp_stream_change_event;(*x).strchange_flags=flags;(*x).strchange_instrms=ins;(*x).strchange_outstrms=outs;}e}

// The remaining receive-info routines preserve the C interface and field ordering.
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_read_sndrcvinfo(e:*const sctp_ulpevent,m:*mut msghdr){if sctp_ulpevent_is_notification(e)!=0{return;}let mut i:sctp_sndrcvinfo=core::mem::zeroed();i.sinfo_stream=(*e).stream;i.sinfo_ssn=(*e).ssn;i.sinfo_ppid=(*e).ppid;i.sinfo_flags=(*e).flags;i.sinfo_tsn=(*e).tsn;i.sinfo_cumtsn=(*e).cumtsn;i.sinfo_assoc_id=sctp_assoc2id((*e).asoc);i.sinfo_context=(*(*e).asoc).default_rcv_context;put_cmsg(m,IPPROTO_SCTP,SCTP_SNDRCV,core::mem::size_of::<sctp_sndrcvinfo>(),&i as *const _ as *const c_void);}
#[no_mangle] pub unsafe extern "C" fn sctp_ulpevent_read_rcvinfo(e:*const sctp_ulpevent,m:*mut msghdr){if sctp_ulpevent_is_notification(e)!=0{return;}let mut i:sctp_rcvinfo=core::mem::zeroed();i.rcv_sid=(*e).stream;i.rcv_ssn=(*e).ssn;i.rcv_ppid=(*e).ppid;i.rcv_flags=(*e).flags;i.rcv_tsn=(*e).tsn;i.rcv_cumtsn=(*e).cumtsn;i.rcv_assoc_id=sctp_assoc2id((*e).asoc);i.rcv_context=(*(*e).asoc).default_rcv_context;put_cmsg(m,IPPROTO_SCTP,SCTP_RCVINFO,core::mem::size_of::<sctp_rcvinfo>(),&i as *const _ as *const c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
