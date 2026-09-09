// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP stream message interleaving implementation.
 *
 * This translation intentionally keeps kernel-provided SCTP types and
 * functions external; they are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __be16 = u16;
pub type __be32 = u32;
pub type gfp_t = usize;

#[repr(C)]
pub struct sctp_stream {
    pub si: *mut sctp_stream_interleave,
    pub incnt: __u16,
}
#[repr(C)]
pub struct sctp_association {
    pub stream: sctp_stream,
    pub peer: sctp_peer,
    pub base: sctp_base,
}
#[repr(C)] pub struct sctp_peer { pub intl_capable: bool, pub prsctp_capable: bool, pub adv_peer_ack_point: __u32, pub tsn_map: *mut c_void }
#[repr(C)] pub struct sctp_base { pub net: *mut c_void, pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_shutdown: u32, pub sk_receive_queue: sk_buff_head, pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct sk_buff { pub len: u32, pub cb: [u8; 48], pub prev: *mut sk_buff }
#[repr(C)] pub struct sk_buff_head { pub head: *mut sk_buff }
#[repr(C)] pub struct sctp_chunk { pub asoc: *mut sctp_association, pub has_mid: u8, pub msg: *mut c_void, pub chunk_hdr: *mut sctp_chunk_hdr, pub subh: sctp_chunk_subh, pub sinfo: sctp_sndrcvinfo }
#[repr(C)] pub struct sctp_chunk_hdr { pub type_: __u8, pub flags: __u8, pub length: __be16 }
#[repr(C)] pub union sctp_chunk_subh { pub data_hdr: *mut sctp_datahdr, pub idata_hdr: *mut sctp_idatahdr, pub ifwdtsn_hdr: *mut sctp_ifwdtsn_hdr }
#[repr(C)] pub struct sctp_datahdr { pub tsn: __be32, pub ssn: __be16 }
#[repr(C)] pub struct sctp_idatahdr { pub stream: __be16, pub mid: __be32, pub ppid: __be32, pub fsn: __be32 }
#[repr(C)] pub struct sctp_ifwdtsn_hdr { pub new_cum_tsn: __be32 }
#[repr(C)] pub struct sctp_sndrcvinfo { pub sinfo_stream: __be16, pub sinfo_flags: __u16, pub sinfo_ppid: __be32 }
#[repr(C)] pub struct sctp_ulpevent { pub stream: __u16, pub mid: __u32, pub fsn: __u32, pub tsn: __u32, pub msg_flags: u32, pub ppid: __u32 }
#[repr(C)] pub struct sctp_ulpq { pub asoc: *mut sctp_association, pub reasm: sk_buff_head, pub reasm_uo: sk_buff_head, pub lobby: sk_buff_head }
#[repr(C)] pub struct sctp_outq { pub asoc: *mut sctp_association, pub abandoned: *mut c_void, pub control_chunk_list: *mut c_void }
#[repr(C)] pub struct sctp_stream_in { pub mid: __u32, pub fsn: __u32, pub pd_mode: u8, pub mid_uo: __u32, pub fsn_uo: __u32, pub pd_mode_uo: u8 }
#[repr(C)] pub struct sctp_ifwdtsn_skip { pub stream: __be16, pub reserved: __u8, pub flags: __u8, pub mid: __be32 }
#[repr(C)] pub struct sctp_stream_interleave {
    pub data_chunk_len: usize,
    pub ftsn_chunk_len: usize,
    pub make_datafrag: Option<unsafe extern "C" fn(*const sctp_association, *const sctp_sndrcvinfo, i32, __u8, gfp_t) -> *mut sctp_chunk>,
    pub assign_number: Option<unsafe extern "C" fn(*mut sctp_chunk)>,
    pub validate_data: Option<unsafe extern "C" fn(*mut sctp_chunk) -> bool>,
    pub ulpevent_data: Option<unsafe extern "C" fn(*mut sctp_ulpq, *mut sctp_chunk, gfp_t) -> i32>,
    pub enqueue_event: Option<unsafe extern "C" fn(*mut sctp_ulpq, *mut sctp_ulpevent) -> i32>,
    pub renege_events: Option<unsafe extern "C" fn(*mut sctp_ulpq, *mut sctp_chunk, gfp_t)>,
    pub start_pd: Option<unsafe extern "C" fn(*mut sctp_ulpq, gfp_t)>,
    pub abort_pd: Option<unsafe extern "C" fn(*mut sctp_ulpq, gfp_t)>,
    pub generate_ftsn: Option<unsafe extern "C" fn(*mut sctp_outq, __u32)>,
    pub validate_ftsn: Option<unsafe extern "C" fn(*mut sctp_chunk) -> bool>,
    pub report_ftsn: Option<unsafe extern "C" fn(*mut sctp_ulpq, __u32)>,
    pub handle_ftsn: Option<unsafe extern "C" fn(*mut sctp_ulpq, *mut sctp_chunk)>,
}

extern "C" {
    fn sctp_make_datafrag_empty(*const sctp_association, *const sctp_sndrcvinfo, i32, __u8, gfp_t) -> *mut sctp_chunk;
    fn sctp_make_idatafrag_empty(*const sctp_association, *const sctp_sndrcvinfo, i32, __u8, gfp_t) -> *mut sctp_chunk;
    fn sctp_chunk_assign_ssn(*mut sctp_chunk);
    fn sctp_chunk_assign_mid(*mut sctp_chunk);
    fn sctp_validate_data(*mut sctp_chunk) -> bool;
    fn sctp_validate_idata(*mut sctp_chunk) -> bool;
    fn sctp_ulpq_tail_data(*mut sctp_ulpq, *mut sctp_chunk, gfp_t) -> i32;
    fn sctp_ulpevent_idata(*mut sctp_ulpq, *mut sctp_chunk, gfp_t) -> i32;
    fn do_ulpq_tail_event(*mut sctp_ulpq, *mut sctp_ulpevent) -> i32;
    fn do_sctp_enqueue_event(*mut sctp_ulpq, *mut sctp_ulpevent) -> i32;
    fn sctp_generate_fwdtsn(*mut sctp_outq, __u32);
    fn sctp_generate_iftsn(*mut sctp_outq, __u32);
    fn sctp_validate_fwdtsn(*mut sctp_chunk) -> bool;
    fn sctp_validate_iftsn(*mut sctp_chunk) -> bool;
    fn sctp_report_fwdtsn(*mut sctp_ulpq, __u32);
    fn sctp_report_iftsn(*mut sctp_ulpq, __u32);
    fn sctp_handle_fwdtsn(*mut sctp_ulpq, *mut sctp_chunk);
    fn sctp_handle_iftsn(*mut sctp_ulpq, *mut sctp_chunk);
    fn sctp_ulpq_renege(*mut sctp_ulpq, *mut sctp_chunk, gfp_t);
    fn sctp_ulpq_partial_delivery(*mut sctp_ulpq, gfp_t);
    fn sctp_ulpq_abort_pd(*mut sctp_ulpq, gfp_t);
    fn sctp_ulpq_renege_events(*mut sctp_ulpq, *mut sctp_chunk, gfp_t);
    fn sctp_intl_start_pd(*mut sctp_ulpq, gfp_t);
    fn sctp_intl_abort_pd(*mut sctp_ulpq, gfp_t);
}

pub static mut sctp_stream_interleave_0: sctp_stream_interleave = sctp_stream_interleave {
    data_chunk_len: 0, ftsn_chunk_len: 0, make_datafrag: Some(sctp_make_datafrag_empty), assign_number: Some(sctp_chunk_assign_ssn),
    validate_data: Some(sctp_validate_data), ulpevent_data: Some(sctp_ulpq_tail_data), enqueue_event: Some(do_ulpq_tail_event),
    renege_events: Some(sctp_ulpq_renege_events), start_pd: Some(sctp_ulpq_partial_delivery), abort_pd: Some(sctp_ulpq_abort_pd),
    generate_ftsn: Some(sctp_generate_fwdtsn), validate_ftsn: Some(sctp_validate_fwdtsn), report_ftsn: Some(sctp_report_fwdtsn), handle_ftsn: Some(sctp_handle_fwdtsn),
};
pub static mut sctp_stream_interleave_1: sctp_stream_interleave = sctp_stream_interleave {
    data_chunk_len: 0, ftsn_chunk_len: 0, make_datafrag: Some(sctp_make_idatafrag_empty), assign_number: Some(sctp_chunk_assign_mid),
    validate_data: Some(sctp_validate_idata), ulpevent_data: Some(sctp_ulpevent_idata), enqueue_event: Some(do_sctp_enqueue_event),
    renege_events: Some(sctp_ulpq_renege_events), start_pd: Some(sctp_intl_start_pd), abort_pd: Some(sctp_intl_abort_pd),
    generate_ftsn: Some(sctp_generate_iftsn), validate_ftsn: Some(sctp_validate_iftsn), report_ftsn: Some(sctp_report_iftsn), handle_ftsn: Some(sctp_handle_iftsn),
};

#[no_mangle]
pub unsafe extern "C" fn sctp_stream_interleave_init(stream: *mut sctp_stream) {
    let asoc = (stream as *mut u8).sub(core::mem::offset_of!(sctp_association, stream)) as *mut sctp_association;
    (*stream).si = if (*asoc).peer.intl_capable { &mut sctp_stream_interleave_1 } else { &mut sctp_stream_interleave_0 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
