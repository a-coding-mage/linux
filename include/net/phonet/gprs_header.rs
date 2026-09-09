/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * File: pep_gprs.h
 *
 * GPRS over Phonet pipe end point socket
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Author: Rémi Denis-Courmont
 */

// C forward declarations.
#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pep_writeable(sk: *mut sock) -> ::core::ffi::c_int;
    pub fn pep_write(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn pep_read(sk: *mut sock) -> *mut sk_buff;

    pub fn gprs_attach(sk: *mut sock) -> ::core::ffi::c_int;
    pub fn gprs_detach(sk: *mut sock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
