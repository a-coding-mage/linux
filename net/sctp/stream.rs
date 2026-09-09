// SPDX-License-Identifier: GPL-2.0-or-later
// Rust translation of the SCTP stream manipulation implementation.
// Kernel types, constants, macros, and helper functions are supplied externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

#[repr(C)]
pub struct sctp_stream { pub outcnt: u16, pub incnt: u16, pub out: genradix, pub r#in: genradix, pub si: *mut sctp_stream_in }
#[repr(C)] pub struct genradix { pub tree: genradix_tree }
#[repr(C)] pub struct genradix_tree { pub root: *mut core::ffi::c_void }
#[repr(C)] pub struct sctp_association { pub stream: sctp_stream, pub outqueue: sctp_outq, pub peer: sctp_peer, pub strreset_enable: u32, pub strreset_outstanding: u32, pub strreset_chunk: *mut sctp_chunk, pub strreset_inseq: u32, pub strreset_outseq: u32, pub strreset_result: [u32; 2], pub ctsn_ack_point: u32, pub adv_peer_ack_point: u32, pub next_tsn: u32, pub ulpq: sctp_ulpq }
#[repr(C)] pub struct sctp_outq { pub out_qlen: u32, pub out_chunk_list: list_head }
#[repr(C)] pub struct sctp_peer { pub reconf_capable: bool, pub prsctp_capable: bool, pub tsn_map: sctp_tsnmap }
#[repr(C)] pub struct sctp_tsnmap;
#[repr(C)] pub struct sctp_ulpq;
#[repr(C)] pub struct sctp_chunk { pub transport: *mut sctp_transport, pub chunk_hdr: *mut core::ffi::c_void }
#[repr(C)] pub struct sctp_transport { pub reconf_timer: timer_list }
#[repr(C)] pub struct timer_list;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct sctp_stream_out { pub mid: u16, pub mid_uo: u16, pub state: u32, pub ext: *mut sctp_stream_out_ext }
#[repr(C)] pub struct sctp_stream_in { pub mid: u16 }
#[repr(C)] pub struct sctp_stream_out_ext { pub outq: list_head }
#[repr(C)] pub struct sctp_sched_ops { pub free_sid: Option<unsafe extern "C" fn(*mut sctp_stream, u16)> }
#[repr(C)] pub struct sctp_chunk_param;
#[repr(C)] pub struct sctp_paramhdr { pub r#type: u16, pub length: u16 }
#[repr(C)] pub union sctp_params { pub v: *mut sctp_paramhdr, pub p: *mut sctp_paramhdr }
#[repr(C)] pub struct sctp_ulpevent;
#[repr(C)] pub struct sctp_reset_streams { pub srs_flags: u16, pub srs_number_streams: u16, pub srs_stream_list: *mut u16 }
#[repr(C)] pub struct sctp_add_streams { pub sas_outstrms: u16, pub sas_instrms: u16 }
#[repr(C)] pub struct sctp_strreset_outreq { pub request_seq: u32, pub response_seq: u32, pub send_reset_at_tsn: u32, pub list_of_streams: *mut u16 }
#[repr(C)] pub struct sctp_strreset_inreq { pub request_seq: u32, pub list_of_streams: *mut u16 }
#[repr(C)] pub struct sctp_strreset_tsnreq { pub request_seq: u32 }
#[repr(C)] pub struct sctp_strreset_addstrm { pub request_seq: u32, pub number_of_streams: u16 }
#[repr(C)] pub struct sctp_strreset_resp { pub response_seq: u32, pub result: u32 }
#[repr(C)] pub struct sctp_strreset_resptsn { pub senders_next_tsn: u32, pub receivers_next_tsn: u32 }

extern "C" {
    fn sctp_sched_ops_from_stream(s: *mut sctp_stream) -> *const sctp_sched_ops;
    fn sctp_sched_dequeue_common(q: *mut sctp_outq, c: *mut sctp_chunk);
    fn sctp_sched_init_sid(s: *mut sctp_stream, sid: u16, gfp: u32) -> i32;
    fn sctp_sched_all(s: *mut sctp_stream); fn sctp_unsched_all(s: *mut sctp_stream);
    fn sctp_chunk_stream_no(c: *mut sctp_chunk) -> u16; fn sctp_chunk_fail(c: *mut sctp_chunk, e: u32); fn sctp_chunk_free(c: *mut sctp_chunk);
    fn sctp_chunk_hold(c: *mut sctp_chunk); fn sctp_chunk_put(c: *mut sctp_chunk);
    fn genradix_prealloc(r: *mut genradix, n: u16, gfp: u32) -> i32; fn genradix_free(r: *mut genradix);
    fn sctp_stream_interleave_init(s: *mut sctp_stream);
    fn sctp_primitive_RECONF(net: *mut core::ffi::c_void, a: *mut sctp_association, c: *mut sctp_chunk) -> i32;
    fn sctp_make_strreset_req(a: *mut sctp_association,n:u16,l:*mut u16,o:bool,i:bool)->*mut sctp_chunk;
    fn sctp_make_strreset_tsnreq(a:*mut sctp_association)->*mut sctp_chunk; fn sctp_make_strreset_addstrm(a:*mut sctp_association,o:u16,i:u16)->*mut sctp_chunk;
    fn sctp_make_strreset_resp(a:*mut sctp_association,r:u32,s:u32)->*mut sctp_chunk; fn sctp_make_strreset_tsnresp(a:*mut sctp_association,r:u32,s:u32,n:u32,i:u32)->*mut sctp_chunk;
    fn sctp_outq_is_empty(q:*mut sctp_outq)->bool; fn sctp_outq_free(q:*mut sctp_outq);
    fn sctp_tsnmap_get_ctsn(m:*mut sctp_tsnmap)->u32; fn sctp_tsnmap_get_max_tsn_seen(m:*mut sctp_tsnmap)->u32; fn sctp_tsnmap_init(m:*mut sctp_tsnmap,n:u32,i:u32,g:u32);
    fn sctp_transport_put(t:*mut sctp_transport); fn timer_delete(t:*mut timer_list)->bool;
}

const GFP_KERNEL:u32=0; const GFP_ATOMIC:u32=0; const SCTP_STREAM_OPEN:u32=0; const SCTP_STREAM_CLOSED:u32=1;
const SCTP_PARAM_RESET_OUT_REQUEST:u16=13; const SCTP_PARAM_RESET_IN_REQUEST:u16=14; const SCTP_PARAM_RESET_TSN_REQUEST:u16=15; const SCTP_PARAM_RESET_ADD_OUT_STREAMS:u16=16; const SCTP_PARAM_RESET_ADD_IN_STREAMS:u16=17;
const SCTP_STRRESET_DENIED:u32=0; const SCTP_STRRESET_PERFORMED:u32=1; const SCTP_STRRESET_IN_PROGRESS:u32=2; const SCTP_STRRESET_ERR_BAD_SEQNO:u32=3; const SCTP_STRRESET_ERR_IN_PROGRESS:u32=4; const SCTP_STRRESET_ERR_WRONG_SSN:u32=5;

unsafe fn so(s:*mut sctp_stream,i:u16)->*mut sctp_stream_out { (*s).out.tree.root.add(i as usize) as *mut sctp_stream_out }
unsafe fn si(s:*mut sctp_stream,i:u16)->*mut sctp_stream_in { (*s).r#in.tree.root.add(i as usize) as *mut sctp_stream_in }
unsafe fn stream_alloc_out(s:*mut sctp_stream,n:u16,g:u32)->i32 { if n>(*s).outcnt { let r=genradix_prealloc(&mut (*s).out,n,g); if r!=0{return r;} } (*s).outcnt=n; 0 }
unsafe fn stream_alloc_in(s:*mut sctp_stream,n:u16,g:u32)->i32 { if n>(*s).incnt { let r=genradix_prealloc(&mut (*s).r#in,n,g); if r!=0{return r;} } (*s).incnt=n; 0 }

pub unsafe fn sctp_stream_init(s:*mut sctp_stream,outcnt:u16,incnt:u16,gfp:u32)->i32 { let r=stream_alloc_out(s,outcnt,gfp|0); if r!=0{return r;} for i in 0..outcnt {(*so(s,i)).state=SCTP_STREAM_OPEN;} sctp_stream_interleave_init(s); if incnt!=0 {stream_alloc_in(s,incnt,gfp)} else {0} }
pub unsafe fn sctp_stream_init_ext(s:*mut sctp_stream,sid:u16)->i32 { let p=libc::calloc(1,core::mem::size_of::<sctp_stream_out_ext>()) as *mut sctp_stream_out_ext; if p.is_null(){return -12;} (*so(s,sid)).ext=p; let r=sctp_sched_init_sid(s,sid,GFP_KERNEL); if r!=0 {libc::free(p as *mut _);(*so(s,sid)).ext=ptr::null_mut();} r }
pub unsafe fn sctp_stream_free(s:*mut sctp_stream) { sctp_unsched_all(s); genradix_free(&mut (*s).out); genradix_free(&mut (*s).r#in); }
pub unsafe fn sctp_stream_clear(s:*mut sctp_stream) { for i in 0..(*s).outcnt {(*so(s,i)).mid=0;(*so(s,i)).mid_uo=0;} for i in 0..(*s).incnt {(*si(s,i)).mid=0;} }

// The remaining protocol handlers retain the C implementation's externally visible
// entry points and are intentionally expressed with raw pointers for kernel ABI fidelity.
pub unsafe fn sctp_send_reset_assoc(a:*mut sctp_association)->i32 { if !(*a).peer.reconf_capable{return -92;} if (*a).strreset_outstanding!=0{return -115;} if !sctp_outq_is_empty(&mut (*a).outqueue){return -11;} let c=sctp_make_strreset_tsnreq(a); if c.is_null(){return -12;} (*a).strreset_chunk=c;sctp_chunk_hold(c);0 }
pub unsafe fn sctp_send_add_streams(a:*mut sctp_association,p:*mut sctp_add_streams)->i32 { let s=&mut (*a).stream; let o=(*p).sas_outstrms; let i=(*p).sas_instrms; if o==0&&i==0{return -22;} let r=stream_alloc_out(s,(s.outcnt as u32+o as u32) as u16,GFP_KERNEL); if r!=0{return r;} let c=sctp_make_strreset_addstrm(a,o,i); if c.is_null(){return -12;} (*a).strreset_chunk=c;sctp_chunk_hold(c);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
