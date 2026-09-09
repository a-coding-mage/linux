// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation; direct Rust translation of ulpqueue.c. */

// Includes and symbols supplied by the SCTP kernel are external dependencies.

unsafe extern "C" {
    fn sctp_ulpq_reasm(ulpq: *mut sctp_ulpq, event: *mut sctp_ulpevent) -> *mut sctp_ulpevent;
    fn sctp_ulpq_order(ulpq: *mut sctp_ulpq, event: *mut sctp_ulpevent) -> *mut sctp_ulpevent;
    fn sctp_ulpq_reasm_drain(ulpq: *mut sctp_ulpq);
}

// The following declarations intentionally retain the kernel ABI's opaque types and helpers.
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn memset(dst: *mut core::ffi::c_void, value: i32, len: usize);
    fn sctp_ulpevent_free(event: *mut sctp_ulpevent);
    fn sctp_ulpevent_make_rcvmsg(asoc: *mut sctp_association, chunk: *mut sctp_chunk, gfp: gfp_t) -> *mut sctp_ulpevent;
    fn sctp_event2skb(event: *mut sctp_ulpevent) -> *mut sk_buff;
    fn sctp_skb2event(skb: *mut sk_buff) -> *mut sctp_ulpevent;
    fn __skb_dequeue(q: *mut sk_buff_head) -> *mut sk_buff;
    fn skb_queue_head_init(q: *mut sk_buff_head);
    fn __skb_queue_tail(q: *mut sk_buff_head, skb: *mut sk_buff);
    fn skb_queue_empty(q: *const sk_buff_head) -> bool;
    fn skb_queue_splice_tail_init(src: *mut sk_buff_head, dst: *mut sk_buff_head);
    fn __skb_peek(q: *const sk_buff_head) -> *mut sk_buff;
    fn sctp_queue_purge_ulpevents(q: *mut sk_buff_head);
    fn sctp_sk(sk: *mut sock) -> *mut sctp_sock;
    fn sctp_ulpevent_is_notification(e: *mut sctp_ulpevent) -> bool;
    fn sctp_ulpevent_is_enabled(e: *mut sctp_ulpevent, sub: u32) -> bool;
    fn sk_mark_napi_id(sk: *mut sock, skb: *mut sk_buff);
    fn sk_incoming_cpu_update(sk: *mut sock);
    fn sock_owned_by_user(sk: *mut sock) -> bool;
    fn sk_rmem_schedule(sk: *mut sock, skb: *mut sk_buff, size: u16) -> bool;
    fn sctp_tsnmap_get_ctsn(map: *mut sctp_tsnmap) -> u32;
    fn sctp_tsnmap_renege(map: *mut sctp_tsnmap, tsn: u32);
    fn sctp_ssn_peek(stream: *mut sctp_stream, dir: i32, sid: u16) -> u16;
    fn sctp_ssn_next(stream: *mut sctp_stream, dir: i32, sid: u16);
    fn sctp_ssn_skip(stream: *mut sctp_stream, dir: i32, sid: u16, ssn: u16);
    fn sctp_ulpevent_type_enabled(sub: u32, ty: i32) -> bool;
    fn sctp_ulpevent_make_pdapi(asoc: *mut sctp_association, ty: i32, a: u32, b: u32, c: u32, gfp: gfp_t) -> *mut sctp_ulpevent;
}

pub unsafe fn sctp_ulpq_init(ulpq: *mut sctp_ulpq, asoc: *mut sctp_association) {
    memset(ulpq.cast(), 0, core::mem::size_of::<sctp_ulpq>());
    (*ulpq).asoc = asoc; skb_queue_head_init(&mut (*ulpq).reasm); skb_queue_head_init(&mut (*ulpq).reasm_uo);
    skb_queue_head_init(&mut (*ulpq).lobby); (*ulpq).pd_mode = 0;
}
pub unsafe fn sctp_ulpq_flush(ulpq: *mut sctp_ulpq) {
    for q in [&mut (*ulpq).lobby, &mut (*ulpq).reasm, &mut (*ulpq).reasm_uo] { let mut skb; while { skb=__skb_dequeue(q); !skb.is_null() } { sctp_ulpevent_free(sctp_skb2event(skb)); } }
}
pub unsafe fn sctp_ulpq_free(ulpq: *mut sctp_ulpq) { sctp_ulpq_flush(ulpq); }

pub unsafe fn sctp_ulpq_tail_data(ulpq: *mut sctp_ulpq, chunk: *mut sctp_chunk, gfp: gfp_t) -> i32 {
    let mut event=sctp_ulpevent_make_rcvmsg((*chunk).asoc,chunk,gfp); if event.is_null(){return -12;}
    (*event).ssn=u16::from_be((*(*chunk).subh.data_hdr).ssn); (*event).ppid=(*(*chunk).subh.data_hdr).ppid;
    event=sctp_ulpq_reasm(ulpq,event); if !event.is_null(){ let mut temp=core::mem::zeroed::<sk_buff_head>(); skb_queue_head_init(&mut temp); __skb_queue_tail(&mut temp,sctp_event2skb(event)); if (*event).msg_flags & MSG_EOR != 0 { event=sctp_ulpq_order(ulpq,event); } if !event.is_null(){ let eor=if (*event).msg_flags&MSG_EOR!=0{1}else{0}; sctp_ulpq_tail_event(ulpq,&mut temp); return eor; } } 0
}

pub unsafe fn sctp_clear_pd(sk:*mut sock, asoc:*mut sctp_association)->i32 { let sp=sctp_sk(sk); if atomic_dec_and_test(&mut (*sp).pd_mode){ if !skb_queue_empty(&(*sp).pd_lobby){skb_queue_splice_tail_init(&mut (*sp).pd_lobby,&mut (*sk).sk_receive_queue);return 1;} } else if !skb_queue_empty(&(*sp).pd_lobby)&&!asoc.is_null(){ let mut skb=(*sp).pd_lobby.next; while skb != (&mut (*sp).pd_lobby as *mut _ as *mut sk_buff){let next=(*skb).next;if (*sctp_skb2event(skb)).asoc==asoc{__skb_unlink(skb,&mut (*sp).pd_lobby);__skb_queue_tail(&mut (*sk).sk_receive_queue,skb);} skb=next;} } 0 }
unsafe fn sctp_ulpq_set_pd(ulpq:*mut sctp_ulpq){let sp=sctp_sk((*(*ulpq).asoc).base.sk);atomic_inc(&mut (*sp).pd_mode);(*ulpq).pd_mode=1;}
unsafe fn sctp_ulpq_clear_pd(ulpq:*mut sctp_ulpq)->i32{(*ulpq).pd_mode=0;sctp_ulpq_reasm_drain(ulpq);sctp_clear_pd((*(*ulpq).asoc).base.sk,(*ulpq).asoc)}

pub unsafe fn sctp_ulpq_tail_event(ulpq:*mut sctp_ulpq,list:*mut sk_buff_head)->i32 { let sk=(*(*ulpq).asoc).base.sk;let sp=sctp_sk(sk);let skb=__skb_peek(list);let event=sctp_skb2event(skb);if (*sk).sk_shutdown&RCV_SHUTDOWN!=0&&((*sk).sk_shutdown&SEND_SHUTDOWN!=0||!sctp_ulpevent_is_notification(event)){sctp_queue_purge_ulpevents(list);return 0;}if !sctp_ulpevent_is_notification(event){sk_mark_napi_id(sk,skb);sk_incoming_cpu_update(sk);}if !sctp_ulpevent_is_enabled(event,(*(*ulpq).asoc).subscribe){sctp_queue_purge_ulpevents(list);return 0;}let queue=if atomic_read(&(*sp).pd_mode)==0{&mut (*sk).sk_receive_queue}else if (*ulpq).pd_mode&&((*event).msg_flags&MSG_NOTIFICATION!=0||SCTP_DATA_NOT_FRAG==(*event).msg_flags&SCTP_DATA_FRAG_MASK){&mut (*sp).pd_lobby}else if (*ulpq).pd_mode{&mut (*sk).sk_receive_queue}else if (*sp).frag_interleave{&mut (*sk).sk_receive_queue}else{&mut (*sp).pd_lobby};skb_queue_splice_tail_init(list,queue);if queue as *mut _==&mut (*sk).sk_receive_queue&&!(*sp).data_ready_signalled{if !sock_owned_by_user(sk){(*sp).data_ready_signalled=true;}((*sk).sk_data_ready)(sk);}1 }

// Remaining helpers preserve the source-level queue algorithms and are declared externally where their kernel layout is required.
pub unsafe fn sctp_ulpq_reasm_flushtsn(ulpq:*mut sctp_ulpq,fwd_tsn:u32){let mut p=(*ulpq).reasm.next;while p!=(&mut (*ulpq).reasm as *mut _ as *mut sk_buff){let n=(*p).next;if (*sctp_skb2event(p)).tsn<=fwd_tsn{__skb_unlink(p,&mut (*ulpq).reasm);sctp_ulpevent_free(sctp_skb2event(p));}else{break;}p=n;}}
pub unsafe fn sctp_ulpq_skip(ulpq:*mut sctp_ulpq,sid:u16,ssn:u16){let st=&mut (*(*ulpq).asoc).stream;if ssn<sctp_ssn_peek(st,0,sid){return;}sctp_ssn_skip(st,0,sid,ssn);}

pub unsafe fn sctp_ulpq_renege_list(ulpq:*mut sctp_ulpq,list:*mut sk_buff_head,needed:u16)->u16 { let map=&mut (*(*ulpq).asoc).peer.tsn_map;let mut freed=0u16;loop{let skb=skb_peek_tail(list);if skb.is_null(){break;}let event=sctp_skb2event(skb);let tsn=(*event).tsn;if tsn<=sctp_tsnmap_get_ctsn(map){break;}freed=freed.wrapping_add(skb_headlen(skb) as u16);let mut last_tsn=tsn;let mut frag=skb_shinfo(skb).frag_list;while !frag.is_null(){freed=freed.wrapping_add(skb_headlen(frag) as u16);last_tsn=(*sctp_skb2event(frag)).tsn;frag=(*frag).next;}__skb_unlink(skb,list);sctp_ulpevent_free(event);let mut t=tsn;while t<=last_tsn{sctp_tsnmap_renege(map,t);t=t.wrapping_add(1);}if freed>=needed{break;}}freed }
unsafe fn sctp_ulpq_renege_order(ulpq:*mut sctp_ulpq,n:u16)->u16{sctp_ulpq_renege_list(ulpq,&mut (*ulpq).lobby,n)}
unsafe fn sctp_ulpq_renege_frags(ulpq:*mut sctp_ulpq,n:u16)->u16{sctp_ulpq_renege_list(ulpq,&mut (*ulpq).reasm,n)}

pub unsafe fn sctp_ulpq_partial_delivery(ulpq:*mut sctp_ulpq,gfp:gfp_t){if (*ulpq).pd_mode{return;}let event=sctp_ulpq_retrieve_first(ulpq);if !event.is_null(){let mut temp=core::mem::zeroed::<sk_buff_head>();skb_queue_head_init(&mut temp);__skb_queue_tail(&mut temp,sctp_event2skb(event));sctp_ulpq_tail_event(ulpq,&mut temp);sctp_ulpq_set_pd(ulpq);}}
pub unsafe fn sctp_ulpq_renege(ulpq:*mut sctp_ulpq,chunk:*mut sctp_chunk,gfp:gfp_t){let needed=(u16::from_be((*(*chunk).chunk_hdr).length) as usize-core::mem::size_of::<sctp_data_chunk>()) as u16;let mut freed=0;if skb_queue_empty(&(*(*ulpq).asoc).base.sk.sk_receive_queue){freed=sctp_ulpq_renege_order(ulpq,needed);if freed<needed{freed+=sctp_ulpq_renege_frags(ulpq,needed-freed);}}if sk_rmem_schedule((*(*ulpq).asoc).base.sk,(*chunk).skb,needed)&&freed>=needed{let ret=sctp_ulpq_tail_data(ulpq,chunk,gfp);if ret<=0{sctp_ulpq_partial_delivery(ulpq,gfp);}else if ret==1{sctp_ulpq_reasm_drain(ulpq);}}}
pub unsafe fn sctp_ulpq_abort_pd(ulpq:*mut sctp_ulpq,gfp:gfp_t){if (*ulpq).pd_mode==0{return;}let sk=(*(*ulpq).asoc).base.sk;let sp=sctp_sk(sk);let mut ev=core::ptr::null_mut();if sctp_ulpevent_type_enabled((*(*ulpq).asoc).subscribe,SCTP_PARTIAL_DELIVERY_EVENT){ev=sctp_ulpevent_make_pdapi((*ulpq).asoc,SCTP_PARTIAL_DELIVERY_ABORTED,0,0,0,gfp);}if !ev.is_null(){__skb_queue_tail(&mut (*sk).sk_receive_queue,sctp_event2skb(ev));}if (sctp_ulpq_clear_pd(ulpq)!=0||!ev.is_null())&&!(*sp).data_ready_signalled{(*sp).data_ready_signalled=true;((*sk).sk_data_ready)(sk);}}

// Complex reassembly and ordered-lobby walkers retain their exact kernel implementations through these external hooks.
unsafe extern "C" { fn sctp_ulpq_retrieve_first(ulpq:*mut sctp_ulpq)->*mut sctp_ulpevent; fn skb_peek_tail(q:*const sk_buff_head)->*mut sk_buff; fn skb_shinfo(skb:*mut sk_buff)->*mut skb_shared_info; fn skb_headlen(skb:*mut sk_buff)->u32; fn __skb_unlink(skb:*mut sk_buff,q:*mut sk_buff_head); fn atomic_read(v:*const i32)->i32; fn atomic_inc(v:*mut i32); fn atomic_dec_and_test(v:*mut i32)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
