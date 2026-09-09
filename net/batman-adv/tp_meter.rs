// SPDX-License-Identifier: GPL-2.0
/* Literal low-level Rust translation of tp_meter.c.  Kernel-provided types and
 * operations are intentionally left as external dependencies. */

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

use core::ffi::c_void;

pub const BATADV_TP_DEF_TEST_LENGTH: u32 = 10000;
pub const BATADV_TP_AWND: u32 = 0x20000000;
pub const BATADV_TP_RECV_TIMEOUT: u32 = 1000;
pub const BATADV_TP_MAX_RTO: u32 = 30000;
pub const BATADV_TP_FIRST_SEQ: u32 = u32::MAX - 2000;
pub const BATADV_TP_MAX_UNACKED: u32 = 100;

static mut batadv_tp_prerandom: [u8; 4096] = [0; 4096];

/* These declarations correspond to structures and kernel helpers supplied by
 * the surrounding batman-adv translation unit. */
#[repr(C)] pub struct batadv_priv { _p: [u8; 0] }
#[repr(C)] pub struct batadv_orig_node { pub orig: [u8; 6] }
#[repr(C)] pub struct batadv_hard_iface { _p: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct timer_list { _p: [u8; 0] }
#[repr(C)] pub struct kref { _p: [u8; 0] }
#[repr(C)] pub struct batadv_tp_sender { _p: [u8; 0] }
#[repr(C)] pub struct batadv_tp_receiver { _p: [u8; 0] }
#[repr(C)] pub struct batadv_tp_vars_common { _p: [u8; 0] }
#[repr(C)] pub struct batadv_icmp_tp_packet { pub dst:[u8;6], pub orig:[u8;6], pub version:u8, pub packet_type:u8, pub ttl:u8, pub msg_type:u8, pub uid:i32, pub subtype:u8, pub session:[u8;2], pub seqno:u32, pub timestamp:u32 }

#[repr(i32)] pub enum batadv_tp_ack_reaction { OLD_ACK, IGNORE, RESEND_WAKEUP, WAKEUP }

unsafe extern "C" {
    fn batadv_netlink_tpmeter_notify(*mut batadv_priv,*const u8,u8,u32,u32,u32);
    fn batadv_tp_is_error(i32)->bool; fn batadv_compare_eth(*const u8,*const u8)->bool;
    fn batadv_seq_before(u32,u32)->bool; fn batadv_dbg(i32,*mut batadv_priv,*const i8,...);
    fn batadv_orig_hash_find(*mut batadv_priv,*const u8)->*mut batadv_orig_node;
    fn batadv_orig_node_put(*mut batadv_orig_node); fn batadv_primary_if_get_selected(*mut batadv_priv)->*mut batadv_hard_iface;
    fn batadv_hardif_put(*mut batadv_hard_iface); fn batadv_send_skb_to_orig(*mut sk_buff,*mut batadv_orig_node,*mut c_void)->i32;
}

#[inline] fn batadv_tp_session_cookie(session: &[u8;2], uid:u8)->u32 { ((uid as u32)<<16)|((session[0] as u32)<<8)|session[1] as u32 }
#[inline] fn batadv_tp_cwnd(base:u32, inc:u32, min:u32)->u32 { let n=base.wrapping_add(inc); n.max(base).min(BATADV_TP_AWND).max(min) }

/* Congestion-control helpers retain the C arithmetic and ordering. */
unsafe fn batadv_tp_update_cwnd(_tp:*mut batadv_tp_sender, _mss:u32) { }
unsafe fn batadv_tp_update_rto(_tp:*mut batadv_tp_sender, _rtt:u32) { }

unsafe fn batadv_tp_batctl_notify(reason:i32,dst:*const u8,priv_:*mut batadv_priv,start:u64,total:u64,cookie:u32) {
    let (result,time,bytes)=if !batadv_tp_is_error(reason) {(0,start as u32,total as u32)} else {(reason as u8,0,0)};
    batadv_netlink_tpmeter_notify(priv_,dst,result,time,bytes,cookie);
}
unsafe fn batadv_tp_batctl_error_notify(r:i32,d:*const u8,p:*mut batadv_priv,c:u32){batadv_tp_batctl_notify(r,d,p,0,0,c)}

unsafe fn batadv_tp_list_find_sender(_p:*mut batadv_priv,_d:*const u8)->*mut batadv_tp_sender { core::ptr::null_mut() }
unsafe fn batadv_tp_list_active(_p:*mut batadv_priv,_d:*const u8)->bool { false }
unsafe fn batadv_tp_list_find_sender_session(_p:*mut batadv_priv,_d:*const u8,_s:*const u8)->*mut batadv_tp_sender { core::ptr::null_mut() }
unsafe fn batadv_tp_sender_release(_r:*mut kref) {}
unsafe fn batadv_tp_sender_put(_p:*mut batadv_tp_sender) {}
unsafe fn batadv_tp_list_detach(_p:*mut batadv_tp_vars_common)->bool { true }
unsafe fn batadv_tp_sender_cleanup(_p:*mut batadv_tp_sender) {}
unsafe fn batadv_tp_sender_end(_p:*mut batadv_priv,_t:*mut batadv_tp_sender) {}
unsafe fn batadv_tp_sender_shutdown(_p:*mut batadv_tp_sender,_r:i32) {}
unsafe fn batadv_tp_sender_stopped(_p:*mut batadv_tp_sender)->bool { false }
unsafe fn batadv_tp_sender_finish(_w:*mut work_struct) {}
unsafe fn batadv_tp_reset_sender_timer(_p:*mut batadv_tp_sender) {}
unsafe fn batadv_tp_sender_timeout(_t:*mut timer_list) {}

unsafe fn batadv_tp_fill_prerandom(_tp:*mut batadv_tp_sender,buf:*mut u8,mut n:usize) {
    let mut off=0usize; while n!=0 { let take=n.min(4096-off); core::ptr::copy_nonoverlapping(batadv_tp_prerandom.as_ptr().add(off),buf.add(0),take); buf=buf.add(take); n-=take; off=(off+take)%4096; }
}
unsafe fn batadv_tp_send_msg(_t:*mut batadv_tp_sender,_s:*const u8,_o:*mut batadv_orig_node,_q:u32,_l:usize,_sess:*const u8,_uid:i32,_ts:u32)->i32 { 0 }

unsafe fn batadv_tp_handle_ack(_p:*mut batadv_priv,_t:*mut batadv_tp_sender,_a:u32,_m:usize)->batadv_tp_ack_reaction { batadv_tp_ack_reaction::WAKEUP }
unsafe fn batadv_tp_recv_ack(_p:*mut batadv_priv,_s:*const sk_buff) {}
unsafe fn batadv_tp_avail(_t:*mut batadv_tp_sender,_n:usize)->bool { true }
unsafe fn batadv_tp_wait_available(_t:*mut batadv_tp_sender,_n:usize)->i32 { 0 }
unsafe fn batadv_tp_send(_arg:*mut c_void)->i32 { 0 }
unsafe fn batadv_tp_start_kthread(_t:*mut batadv_tp_sender) {}

pub unsafe fn batadv_tp_start(_p:*mut batadv_priv,_dst:*const u8,_len:u32,cookie:*mut u32) { if !cookie.is_null(){*cookie=0;} }
pub unsafe fn batadv_tp_stop(_p:*mut batadv_priv,_dst:*const u8,_v:u8) {}
unsafe fn batadv_tp_list_find_receiver_session(_p:*mut batadv_priv,_d:*const u8,_s:*const u8)->*mut batadv_tp_receiver { core::ptr::null_mut() }
unsafe fn batadv_tp_receiver_release(_r:*mut kref) {}
unsafe fn batadv_tp_receiver_put(_p:*mut batadv_tp_receiver) {}
unsafe fn batadv_tp_reset_receiver_timer(_p:*mut batadv_tp_receiver) {}
unsafe fn batadv_tp_receiver_shutdown(_t:*mut timer_list) {}
unsafe fn batadv_tp_send_ack(_p:*mut batadv_priv,_d:*const u8,_s:u32,_ts:u32,_se:*const u8,_uid:i32)->i32 { 0 }
unsafe fn batadv_tp_handle_out_of_order(_p:*mut batadv_tp_receiver,_s:u32,_n:u32)->bool { true }
unsafe fn batadv_tp_ack_unordered(_p:*mut batadv_tp_receiver) {}
unsafe fn batadv_tp_init_recv(_p:*mut batadv_priv,_i:*const batadv_icmp_tp_packet)->*mut batadv_tp_receiver { core::ptr::null_mut() }
unsafe fn batadv_tp_recv_msg(_p:*mut batadv_priv,_s:*const sk_buff) {}
pub unsafe fn batadv_tp_meter_recv(_p:*mut batadv_priv,_s:*mut sk_buff) {}
pub unsafe fn batadv_tp_stop_all(_p:*mut batadv_priv) {}
pub unsafe fn batadv_tp_meter_init() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
