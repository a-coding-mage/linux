/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SMB2 compression support for ksmbd.
 *
 * Copyright (C) 2026 Namjae Jeon <linkinjeon@kernel.org>
 */

// Dependencies supplied by connection.h and ../common/compress/compress.h.

use std::os::raw::c_int;

#[repr(C)]
pub struct ksmbd_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_work {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ksmbd_decompress_request(conn: *mut ksmbd_conn) -> c_int;
    pub fn ksmbd_decompress_work_request(work: *mut ksmbd_work) -> c_int;
    pub fn ksmbd_compress_response(work: *mut ksmbd_work) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
