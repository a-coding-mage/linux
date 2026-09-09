/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * The ASB.1/BER parsing code is derived from ip_nat_snmp_basic.c which was in
 * turn derived from the gxsnmp package by Gregory McLean & Jochen Friedrich
 *
 * Copyright (c) 2000 RP Internet (www.rpi.net.au).
 * Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

use core::ffi::{c_char, c_int};

// External dependency supplied by the surrounding translation unit.
#[repr(C)]
pub struct ksmbd_conn {
    _private: [u8; 0],
}

extern "C" {
    pub fn ksmbd_decode_negTokenInit(
        security_blob: *mut u8,
        length: c_int,
        conn: *mut ksmbd_conn,
    ) -> c_int;

    pub fn ksmbd_decode_negTokenTarg(
        security_blob: *mut u8,
        length: c_int,
        conn: *mut ksmbd_conn,
    ) -> c_int;

    pub fn build_spnego_ntlmssp_neg_blob(
        pbuffer: *mut *mut u8,
        buflen: *mut u16,
        ntlm_blob: *mut c_char,
        ntlm_blob_len: c_int,
    ) -> c_int;

    pub fn build_spnego_ntlmssp_auth_blob(
        pbuffer: *mut *mut u8,
        buflen: *mut u16,
        neg_result: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
