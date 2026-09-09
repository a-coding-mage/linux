/* SPDX-License-Identifier: GPL-2.0-or-later */
/* NET Generic infrastructure for INET connection oriented protocols. */
/* Translated from inet_connection_sock.h; Linux dependencies are external. */

use core::ffi::c_void;

// Opaque types and external helpers are supplied by the surrounding kernel translation.
#[repr(C)] pub struct inet_bind_bucket { _private: [u8; 0] }
#[repr(C)] pub struct inet_bind2_bucket { _private: [u8; 0] }
#[repr(C)] pub struct tcp_congestion_ops { _private: [u8; 0] }
#[repr(C)] pub struct inet_sock { _private: [u8; 0] }
#[repr(C)] pub struct request_sock_queue { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct flowi { _private: [u8; 0] }
#[repr(C)] pub struct request_sock { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct tcp_ulp_ops { _private: [u8; 0] }
#[repr(C)] pub struct proto_accept_arg { _private: [u8; 0] }
#[repr(C)] pub struct flowi4 { _private: [u8; 0] }

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type gfp_t = usize;
pub type sockptr_t = *mut c_void;
pub type __poll_t = u32;

#[repr(C)]
pub struct inet_connection_sock_af_ops {
    pub queue_xmit: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff, *mut flowi) -> i32>,
    pub net_header_len: u16,
    pub rebuild_header: Option<unsafe extern "C" fn(*mut sock) -> i32>,
    pub sk_rx_dst_set: Option<unsafe extern "C" fn(*mut sock, *const sk_buff)>,
    pub conn_request: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32>,
    pub syn_recv_sock: Option<unsafe extern "C" fn(*const sock, *mut sk_buff, *mut request_sock, *mut dst_entry, *mut request_sock, *mut bool, Option<unsafe extern "C" fn(*mut sock, *const sock)>) -> *mut sock>,
    pub setsockopt: Option<unsafe extern "C" fn(*mut sock, i32, i32, sockptr_t, u32) -> i32>,
    pub getsockopt: Option<unsafe extern "C" fn(*mut sock, i32, i32, *mut i8, *mut i32) -> i32>,
    pub mtu_reduced: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)]
pub struct inet_connection_sock_ack {
    pub pending: u8, pub quick: u8, pub pingpong: u8, pub retry: u8,
    pub ato: u32, pub lrcv_flowlabel: u32, pub dst_quick_ack: u32, pub unused: u32,
    pub lrcvtime: u32, pub last_seg_size: u16, pub rcv_mss: u16,
}
#[repr(C)]
pub struct inet_connection_sock_mtup {
    pub search_high: i32, pub search_low: i32, pub probe_size: u32,
    pub enabled: u32, pub probe_timestamp: u32,
}
#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_inet: inet_sock,
    pub icsk_accept_queue: request_sock_queue,
    pub icsk_bind_hash: *mut inet_bind_bucket,
    pub icsk_bind2_hash: *mut inet_bind2_bucket,
    pub icsk_delack_timer: timer_list,
    pub icsk_keepalive_timer: timer_list,
    pub icsk_rto: u32, pub icsk_rto_min: u32, pub icsk_rto_max: u32,
    pub icsk_delack_max: u32, pub icsk_pmtu_cookie: u32,
    pub icsk_ca_ops: *const tcp_congestion_ops,
    pub icsk_af_ops: *const inet_connection_sock_af_ops,
    pub icsk_ulp_ops: *const tcp_ulp_ops,
    pub icsk_ulp_data: *mut c_void,
    pub icsk_sync_mss: Option<unsafe extern "C" fn(*mut sock, u32) -> u32>,
    pub icsk_ca_state: u8, pub icsk_ca_initialized: u8, pub icsk_ca_setsockopt: u8, pub icsk_ca_dst_locked: u8,
    pub icsk_retransmits: u8, pub icsk_pending: u8, pub icsk_backoff: u8, pub icsk_syn_retries: u8,
    pub icsk_probes_out: u8, pub icsk_ext_hdr_len: u16,
    pub icsk_ack: inet_connection_sock_ack,
    pub icsk_mtup: inet_connection_sock_mtup,
    pub icsk_probes_tstamp: u32, pub icsk_user_timeout: u32,
    pub icsk_ca_priv: [u64; 13],
}

pub const ICSK_TIME_RETRANS: i32 = 1;
pub const ICSK_TIME_DACK: i32 = 2;
pub const ICSK_TIME_PROBE0: i32 = 3;
pub const ICSK_TIME_LOSS_PROBE: i32 = 5;
pub const ICSK_TIME_REO_TIMEOUT: i32 = 6;

pub unsafe fn inet_csk_ca(sk: *const sock) -> *mut c_void { inet_csk(sk).cast::<inet_connection_sock>().as_mut().unwrap().icsk_ca_priv.as_mut_ptr().cast() }
pub const ICSK_ACK_SCHED: u8 = 1;
pub const ICSK_ACK_TIMER: u8 = 2;
pub const ICSK_ACK_PUSHED: u8 = 4;
pub const ICSK_ACK_PUSHED2: u8 = 8;
pub const ICSK_ACK_NOW: u8 = 16;
pub const ICSK_ACK_NOMEM: u8 = 32;

extern "C" {
    pub fn inet_csk_clone_lock(sk: *const sock, req: *const request_sock, priority: gfp_t) -> *mut sock;
    pub fn inet_csk_init_xmit_timers(sk: *mut sock, retransmit_handler: Option<unsafe extern "C" fn(*mut timer_list)>, delack_handler: Option<unsafe extern "C" fn(*mut timer_list)>, keepalive_handler: Option<unsafe extern "C" fn(*mut timer_list)>);
    pub fn inet_csk_clear_xmit_timers(sk: *mut sock);
    pub fn inet_csk_clear_xmit_timers_sync(sk: *mut sock);
    pub fn inet_csk_accept(sk: *mut sock, arg: *mut proto_accept_arg) -> *mut sock;
    pub fn inet_csk_get_port(sk: *mut sock, snum: u16) -> i32;
    pub fn inet_csk_route_req(sk: *const sock, fl4: *const flowi4, req: *const request_sock) -> *mut dst_entry;
    pub fn inet_csk_route_child_sock(sk: *const sock, newsk: *mut sock, req: *const request_sock) -> *mut dst_entry;
    pub fn inet_csk_reqsk_queue_add(sk: *mut sock, req: *mut request_sock, child: *mut sock) -> *mut sock;
    pub fn inet_csk_reqsk_queue_hash_add(sk: *mut sock, req: *mut request_sock) -> bool;
    pub fn inet_csk_complete_hashdance(sk: *mut sock, child: *mut sock, req: *mut request_sock, own_req: bool) -> *mut sock;
    pub fn inet_csk_reqsk_queue_drop(sk: *mut sock, req: *mut request_sock) -> bool;
    pub fn inet_csk_reqsk_queue_drop_and_put(sk: *mut sock, req: *mut request_sock);
    pub fn inet_csk_destroy_sock(sk: *mut sock);
    pub fn inet_csk_prepare_for_destroy_sock(sk: *mut sock);
    pub fn inet_csk_prepare_forced_close(sk: *mut sock);
    pub fn inet_csk_listen_start(sk: *mut sock) -> i32;
    pub fn inet_csk_listen_stop(sk: *mut sock);
    pub fn inet_csk_update_fastreuse(sk: *const sock, tb: *mut inet_bind_bucket, tb2: *mut inet_bind2_bucket);
    pub fn inet_csk_update_pmtu(sk: *mut sock, mtu: u32) -> *mut dst_entry;
}

#[repr(i32)] pub enum inet_csk_ack_state_t { ICSK_ACK_SCHED_ENUM = 1, ICSK_ACK_TIMER_ENUM = 2, ICSK_ACK_PUSHED_ENUM = 4, ICSK_ACK_PUSHED2_ENUM = 8, ICSK_ACK_NOW_ENUM = 16, ICSK_ACK_NOMEM_ENUM = 32 }

// The following inline operations retain the original kernel expressions and rely on external helpers/macros.
pub unsafe fn inet_csk_schedule_ack(sk: *mut sock) { (*inet_csk(sk)).icsk_ack.pending |= ICSK_ACK_SCHED; }
pub unsafe fn inet_csk_ack_scheduled(sk: *const sock) -> i32 { ((*inet_csk(sk)).icsk_ack.pending & ICSK_ACK_SCHED) as i32 }
pub unsafe fn inet_csk_delack_init(sk: *mut sock) { core::ptr::write_bytes(&mut (*inet_csk(sk)).icsk_ack as *mut _, 0, 1); }
pub unsafe fn tcp_timeout_expires(_sk: *const sock) -> usize { 0 }
pub unsafe fn icsk_delack_timeout(_icsk: *const inet_connection_sock) -> usize { 0 }
pub unsafe fn inet_csk_clear_xmit_timer(sk: *mut sock, what: i32) {
    let icsk = &mut *inet_csk(sk);
    if what == ICSK_TIME_RETRANS || what == ICSK_TIME_PROBE0 { icsk.icsk_pending = 0; }
    else if what == ICSK_TIME_DACK { icsk.icsk_ack.pending = 0; icsk.icsk_ack.retry = 0; }
}
pub unsafe fn inet_csk_reset_xmit_timer(sk: *mut sock, what: i32, when: usize, max_when: usize) {
    let icsk = &mut *inet_csk(sk); let when = core::cmp::min(when, max_when);
    if what == ICSK_TIME_RETRANS || what == ICSK_TIME_PROBE0 || what == ICSK_TIME_LOSS_PROBE || what == ICSK_TIME_REO_TIMEOUT { icsk.icsk_pending = what as u8; }
    else if what == ICSK_TIME_DACK { icsk.icsk_ack.pending |= ICSK_ACK_TIMER; }
    let _ = (sk, when);
}
pub unsafe fn inet_csk_rto_backoff(icsk: *const inet_connection_sock, max_when: usize) -> usize { core::cmp::min(((*icsk).icsk_rto as usize) << (*icsk).icsk_backoff, max_when) }
pub unsafe fn inet_csk_reqsk_queue_added(_sk: *mut sock) {}
pub unsafe fn inet_csk_reqsk_queue_len(_sk: *const sock) -> i32 { 0 }
pub unsafe fn inet_csk_reqsk_queue_is_full(_sk: *const sock) -> bool { false }
pub unsafe fn inet_csk_listen_poll(_sk: *const sock) -> __poll_t { 0 }
pub unsafe fn inet_csk_enter_pingpong_mode(_sk: *mut sock) {}
pub unsafe fn inet_csk_exit_pingpong_mode(_sk: *mut sock) {}
pub unsafe fn inet_csk_in_pingpong_mode(_sk: *mut sock) -> bool { false }
pub unsafe fn inet_csk_inc_pingpong_cnt(_sk: *mut sock) {}
pub unsafe fn inet_csk_has_ulp(_sk: *const sock) -> bool { false }
pub unsafe fn inet_init_csk_locks(_sk: *mut sock) {}

extern "C" { pub fn inet_csk(sk: *const sock) -> *mut inet_connection_sock; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
