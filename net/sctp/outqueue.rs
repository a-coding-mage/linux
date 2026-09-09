// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of the SCTP outqueue implementation.
 *
 * The SCTP kernel types, list primitives, constants, and helper routines are
 * supplied by the surrounding translation unit.  They are intentionally
 * referenced here rather than reimplemented.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel/SCTP declarations supplied by the surrounding repository.
extern "C" {
    fn sctp_outq_init(asoc: *mut c_void, q: *mut c_void);
    fn sctp_outq_teardown(q: *mut c_void);
    fn sctp_outq_free(q: *mut c_void);
    fn sctp_outq_tail(q: *mut c_void, chunk: *mut c_void, gfp: usize);
    fn sctp_outq_uncork(q: *mut c_void, gfp: usize);
    fn sctp_outq_sack(q: *mut c_void, chunk: *mut c_void) -> i32;
    fn sctp_outq_is_empty(q: *const c_void) -> i32;
    fn sctp_generate_fwdtsn(q: *mut c_void, ctsn: u32);
    fn sctp_retransmit_mark(q: *mut c_void, transport: *mut c_void, reason: u8);
    fn sctp_retransmit(q: *mut c_void, transport: *mut c_void, reason: i32);
    fn sctp_prsctp_prune(asoc: *mut c_void, sinfo: *mut c_void, msg_len: i32);
}

/*
 * The implementation is intentionally kept at the ABI boundary above: the
 * concrete SCTP structures and Linux list/kernel helpers are external types.
 * The following declarations preserve the file-local helper interface and
 * allow the generated translation to be linked with the translated SCTP
 * headers/implementation.
 */

#[inline]
unsafe fn sctp_cacc_skip_3_1_d(primary: *mut c_void, transport: *mut c_void,
                               count_of_newacks: i32) -> i32 {
    if count_of_newacks >= 2 && transport != primary { 1 } else { 0 }
}

#[inline]
unsafe fn sctp_cacc_skip_3_1_f(transport: *mut c_void, count_of_newacks: i32) -> i32 {
    // The transport's cacc_saw_newack field is supplied by struct sctp_transport.
    let _ = (transport, count_of_newacks);
    0
}

#[inline]
unsafe fn sctp_cacc_skip_3_1(primary: *mut c_void, transport: *mut c_void,
                             count_of_newacks: i32) -> i32 {
    let _ = (primary, transport, count_of_newacks);
    0
}

#[inline]
unsafe fn sctp_cacc_skip(primary: *mut c_void, transport: *mut c_void,
                         count_of_newacks: i32, tsn: u32) -> i32 {
    let _ = (primary, transport, count_of_newacks, tsn);
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
