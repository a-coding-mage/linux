/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation; definitions needed by the state machine. */

/* C includes are supplied by other translated headers. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sctp_disposition {
    SCTP_DISPOSITION_DISCARD,
    SCTP_DISPOSITION_CONSUME,
    SCTP_DISPOSITION_NOMEM,
    SCTP_DISPOSITION_DELETE_TCB,
    SCTP_DISPOSITION_ABORT,
    SCTP_DISPOSITION_VIOLATION,
    SCTP_DISPOSITION_NOT_IMPL,
    SCTP_DISPOSITION_ERROR,
    SCTP_DISPOSITION_BUG,
}

pub type sctp_state_fn_t = unsafe extern "C" fn(
    net: *mut net,
    ep: *const sctp_endpoint,
    asoc: *const sctp_association,
    type_: sctp_subtype,
    arg: *mut core::ffi::c_void,
    commands: *mut sctp_cmd_seq,
) -> sctp_disposition;
pub type sctp_timer_event_t = unsafe extern "C" fn(*mut timer_list);

#[repr(C)]
pub struct sctp_sm_table_entry { pub fn_: *mut sctp_state_fn_t, pub name: *const core::ffi::c_char }

/* External types and constants are supplied by dependencies. */
#[allow(non_camel_case_types)] pub type __u8 = u8;
#[allow(non_camel_case_types)] pub type __u16 = u16;
#[allow(non_camel_case_types)] pub type __u32 = u32;
#[allow(non_camel_case_types)] pub type __s16 = i16;
#[allow(non_camel_case_types)] pub type __s32 = i32;
#[allow(non_camel_case_types)] pub type __be16 = u16;
pub type gfp_t = core::ffi::c_uint;
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sctp_endpoint { _private: [u8; 0] }
#[repr(C)] pub struct sctp_association { _private: [u8; 0] }
#[repr(C)] pub struct sctp_cmd_seq { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct sctp_chunk { _private: [u8; 0] }
#[repr(C)] pub struct sctp_bind_addr { _private: [u8; 0] }
#[repr(C)] pub struct sctp_ifwdtsn_skip { _private: [u8; 0] }
#[repr(C)] pub struct sctp_sndrcvinfo { _private: [u8; 0] }
#[repr(C)] pub struct sctp_fwdtsn_skip { _private: [u8; 0] }
#[repr(C)] pub struct sctp_transport { _private: [u8; 0] }
#[repr(C)] pub struct sctp_paramhdr { _private: [u8; 0] }
#[repr(C)] pub struct sctp_ulpevent { _private: [u8; 0] }
#[repr(C)] pub struct sctp_packet { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub union sctp_subtype { pub u: __u32 }
#[repr(C)] pub union sctp_params { pub u: __u32 }
#[repr(C)] pub union sctp_addr { pub u: __u32 }
pub type sctp_event_type = core::ffi::c_int;
pub type sctp_state = core::ffi::c_int;
pub const SCTP_NUM_TIMEOUT_TYPES: usize = 0; /* supplied by the SCTP dependency */

macro_rules! sctp_state_decls { ($($name:ident),* $(,)?) => { $(extern "C" { pub fn $name; })* }; }
sctp_state_decls!(
 sctp_sf_not_impl, sctp_sf_bug, sctp_sf_timer_ignore, sctp_sf_do_9_1_abort,
 sctp_sf_cookie_wait_abort, sctp_sf_cookie_echoed_abort, sctp_sf_shutdown_pending_abort,
 sctp_sf_shutdown_sent_abort, sctp_sf_shutdown_ack_sent_abort, sctp_sf_do_5_1B_init,
 sctp_sf_do_5_1C_ack, sctp_sf_do_5_1D_ce, sctp_sf_do_5_1E_ca, sctp_sf_do_4_C,
 sctp_sf_eat_data_6_2, sctp_sf_eat_data_fast_4_4, sctp_sf_eat_sack_6_2, sctp_sf_operr_notify,
 sctp_sf_t1_init_timer_expire, sctp_sf_t1_cookie_timer_expire, sctp_sf_t2_timer_expire,
 sctp_sf_t4_timer_expire, sctp_sf_t5_timer_expire, sctp_sf_sendbeat_8_3, sctp_sf_beat_8_3,
 sctp_sf_backbeat_8_3, sctp_sf_do_9_2_final, sctp_sf_do_9_2_shutdown, sctp_sf_do_9_2_shut_ctsn,
 sctp_sf_do_ecn_cwr, sctp_sf_do_ecne, sctp_sf_ootb, sctp_sf_pdiscard, sctp_sf_violation,
 sctp_sf_discard_chunk, sctp_sf_do_5_2_1_siminit, sctp_sf_do_5_2_2_dupinit,
 sctp_sf_do_5_2_3_initack, sctp_sf_do_5_2_4_dupcook, sctp_sf_unk_chunk, sctp_sf_do_8_5_1_E_sa,
 sctp_sf_cookie_echoed_err, sctp_sf_do_asconf, sctp_sf_do_asconf_ack, sctp_sf_do_reconf,
 sctp_sf_do_9_2_reshutack, sctp_sf_eat_fwd_tsn, sctp_sf_eat_fwd_tsn_fast, sctp_sf_eat_auth,
 sctp_sf_do_prm_asoc, sctp_sf_do_prm_send, sctp_sf_do_9_2_prm_shutdown,
 sctp_sf_cookie_wait_prm_shutdown, sctp_sf_cookie_echoed_prm_shutdown, sctp_sf_do_9_1_prm_abort,
 sctp_sf_cookie_wait_prm_abort, sctp_sf_cookie_echoed_prm_abort, sctp_sf_shutdown_pending_prm_abort,
 sctp_sf_shutdown_sent_prm_abort, sctp_sf_shutdown_ack_sent_prm_abort, sctp_sf_error_closed,
 sctp_sf_error_shutdown, sctp_sf_ignore_primitive, sctp_sf_do_prm_requestheartbeat,
 sctp_sf_do_prm_asconf, sctp_sf_do_prm_reconf, sctp_sf_do_no_pending_tsn,
 sctp_sf_do_9_2_start_shutdown, sctp_sf_do_9_2_shutdown_ack, sctp_sf_ignore_other,
 sctp_sf_cookie_wait_icmp_abort, sctp_sf_do_6_3_3_rtx, sctp_sf_send_reconf,
 sctp_sf_send_probe, sctp_sf_do_6_2_sack, sctp_sf_autoclose_timer_expire
);

/* The remaining header declarations retain the C ABI and exact parameter order. */
extern "C" {
    pub fn sctp_sm_lookup_event(*mut net, sctp_event_type, sctp_state, sctp_subtype) -> *const sctp_sm_table_entry;
    pub fn sctp_make_temp_asoc(*const sctp_endpoint, *mut sctp_chunk, gfp_t) -> *mut sctp_association;
    pub fn sctp_do_sm(*mut net, sctp_event_type, sctp_subtype, sctp_state, *mut sctp_endpoint, *mut sctp_association, *mut core::ffi::c_void, gfp_t) -> core::ffi::c_int;
    pub fn sctp_ootb_pkt_free(*mut sctp_packet);
    pub fn sctp_generate_tag(*const sctp_endpoint) -> __u32;
    pub fn sctp_generate_tsn(*const sctp_endpoint) -> __u32;
    pub static mut sctp_timer_events: *mut sctp_timer_event_t;
}

#[inline]
pub unsafe fn TSN_lt(a: __u32, b: __u32) -> bool { (a.wrapping_sub(b) as __s32) < 0 }
#[inline]
pub unsafe fn TSN_lte(a: __u32, b: __u32) -> bool { (a.wrapping_sub(b) as __s32) <= 0 }
#[inline]
pub unsafe fn MID_lt(a: __u32, b: __u32) -> bool { (a.wrapping_sub(b) as __s32) < 0 }
#[inline]
pub unsafe fn SSN_lt(a: __u16, b: __u16) -> bool { (a.wrapping_sub(b) as __s16) < 0 }
#[inline]
pub unsafe fn ADDIP_SERIAL_gte(a: __u32, b: __u32) -> bool { (b.wrapping_sub(a) as __s32) <= 0 }

/* Inline functions involving chunk internals are represented with the same external helpers. */
extern "C" {
    pub fn sctp_data_size(chunk: *mut sctp_chunk) -> __u16;
    pub fn sctp_vtag_verify(chunk: *const sctp_chunk, asoc: *const sctp_association) -> core::ffi::c_int;
    pub fn sctp_vtag_verify_either(chunk: *const sctp_chunk, asoc: *const sctp_association) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
