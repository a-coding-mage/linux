/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 IBM Corporation
 * Copyright (C) 2010 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 * Roberto Sassu <roberto.sassu@polito.it>
 */

// Dependencies supplied by the corresponding Linux headers:
// #include <linux/key.h>
// #include <linux/rcupdate.h>

use core::ffi::c_char;

#[repr(C)]
pub struct encrypted_key_payload {
    pub rcu: rcu_head,
    pub format: *mut c_char,          /* datablob: format */
    pub master_desc: *mut c_char,     /* datablob: master key name */
    pub datalen: *mut c_char,         /* datablob: decrypted key length */
    pub iv: *mut u8,                  /* datablob: iv */
    pub encrypted_data: *mut u8,      /* datablob: encrypted data */
    pub datablob_len: u16,            /* length of datablob */
    pub decrypted_datalen: u16,       /* decrypted data length */
    pub payload_datalen: u16,         /* payload data length */
    pub encrypted_key_format: u16,    /* encrypted key format */
    pub decrypted_data: *mut u8,      /* decrypted data */
    pub payload_data: [u8; 0],         /* payload data + datablob + hmac */
}

extern "C" {
    pub static mut key_type_encrypted: key_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
