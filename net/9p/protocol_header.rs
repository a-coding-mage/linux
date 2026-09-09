/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 9P Protocol Support Code
 *
 *  Copyright (C) 2008 by Eric Van Hensbergen <ericvh@gmail.com>
 *
 *  Base on code from Anthony Liguori <aliguori@us.ibm.com>
 *  Copyright (C) 2008 by IBM, Corp.
 */

use core::ffi::{c_char, c_void};

// C types and declarations supplied by the surrounding translation unit.
pub type size_t = usize;
pub type va_list = *mut c_void;

extern "C" {
    pub fn p9_msg_buf_size(
        c: *mut p9_client,
        type_: p9_msg_t,
        fmt: *const c_char,
        ap: va_list,
    ) -> size_t;
    pub fn p9pdu_vwritef(
        pdu: *mut p9_fcall,
        proto_version: i32,
        fmt: *const c_char,
        ap: va_list,
    ) -> i32;
    pub fn p9pdu_readf(
        pdu: *mut p9_fcall,
        proto_version: i32,
        fmt: *const c_char,
        ...,
    ) -> i32;
    pub fn p9pdu_prepare(pdu: *mut p9_fcall, tag: i16, type_: i8) -> i32;
    pub fn p9pdu_finalize(clnt: *mut p9_client, pdu: *mut p9_fcall) -> i32;
    pub fn p9pdu_reset(pdu: *mut p9_fcall);
    pub fn pdu_read(pdu: *mut p9_fcall, data: *mut c_void, size: size_t) -> size_t;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
