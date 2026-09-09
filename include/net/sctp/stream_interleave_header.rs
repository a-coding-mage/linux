/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * These are definitions used by the stream schedulers, defined in RFC
 * draft ndata (https://tools.ietf.org/html/draft-ietf-tsvwg-sctp-ndata-11)
 *
 * Please send any bug reports or fixes you make to the
 * email addresses:
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *   Xin Long <lucien.xin@gmail.com>
 */

#![allow(non_camel_case_types)]

pub struct sctp_association;
pub struct sctp_sndrcvinfo;
pub struct sctp_chunk;
pub struct sctp_ulpq;
pub struct sctp_ulpevent;
pub struct sctp_outq;
pub struct sctp_stream;
pub type gfp_t = usize;

#[repr(C)]
pub struct sctp_stream_interleave {
    pub data_chunk_len: u16,
    pub ftsn_chunk_len: u16,
    /* (I-)DATA process */
    pub make_datafrag: Option<unsafe extern "C" fn(
        asoc: *const sctp_association,
        sinfo: *const sctp_sndrcvinfo,
        len: i32,
        flags: u8,
        gfp: gfp_t,
    ) -> *mut sctp_chunk>,
    pub assign_number: Option<unsafe extern "C" fn(chunk: *mut sctp_chunk)>,
    pub validate_data: Option<unsafe extern "C" fn(chunk: *mut sctp_chunk) -> bool>,
    pub ulpevent_data: Option<unsafe extern "C" fn(
        ulpq: *mut sctp_ulpq,
        chunk: *mut sctp_chunk,
        gfp: gfp_t,
    ) -> i32>,
    pub enqueue_event: Option<unsafe extern "C" fn(
        ulpq: *mut sctp_ulpq,
        event: *mut sctp_ulpevent,
    ) -> i32>,
    pub renege_events: Option<unsafe extern "C" fn(
        ulpq: *mut sctp_ulpq,
        chunk: *mut sctp_chunk,
        gfp: gfp_t,
    )>,
    pub start_pd: Option<unsafe extern "C" fn(ulpq: *mut sctp_ulpq, gfp: gfp_t)>,
    pub abort_pd: Option<unsafe extern "C" fn(ulpq: *mut sctp_ulpq, gfp: gfp_t)>,
    /* (I-)FORWARD-TSN process */
    pub generate_ftsn: Option<unsafe extern "C" fn(q: *mut sctp_outq, ctsn: u32)>,
    pub validate_ftsn: Option<unsafe extern "C" fn(chunk: *mut sctp_chunk) -> bool>,
    pub report_ftsn: Option<unsafe extern "C" fn(ulpq: *mut sctp_ulpq, ftsn: u32)>,
    pub handle_ftsn: Option<unsafe extern "C" fn(
        ulpq: *mut sctp_ulpq,
        chunk: *mut sctp_chunk,
    )>,
}

unsafe extern "C" {
    pub fn sctp_stream_interleave_init(stream: *mut sctp_stream);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
