// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 IBM Corporation
 * Copyright (C) 2010 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 * Roberto Sassu <roberto.sassu@polito.it>
 *
 * See Documentation/security/keys/trusted-encrypted.rst
 */

// C dependencies:
// #include <linux/uaccess.h>
// #include <linux/err.h>
// #include <keys/trusted-type.h>
// #include <keys/encrypted-type.h>
// #include "encrypted.h"

use core::ffi::{c_char, c_void};

pub type u8 = core::ffi::c_uchar;
pub type size_t = usize;

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub union key_payload {
    pub data: [*mut c_void; 4],
}

#[repr(C)]
pub struct key {
    pub sem: rw_semaphore,
    pub payload: key_payload,
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trusted_key_payload {
    pub key: *const u8,
    pub key_len: size_t,
}

extern "C" {
    static mut key_type_trusted: key_type;

    fn request_key(
        type_: *mut key_type,
        description: *const c_char,
        callout_info: *const c_void,
    ) -> *mut key;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn down_read(sem: *mut rw_semaphore);
}

/*
 * request_trusted_key - request the trusted key
 *
 * Trusted keys are sealed to PCRs and other metadata. Although userspace
 * manages both trusted/encrypted key-types, like the encrypted key type
 * data, trusted key type data is not visible decrypted from userspace.
 */
pub unsafe extern "C" fn request_trusted_key(
    trusted_desc: *const c_char,
    master_key: *mut *const u8,
    master_keylen: *mut size_t,
) -> *mut key {
    let mut tpayload: *mut trusted_key_payload;
    let tkey: *mut key;

    tkey = request_key(
        core::ptr::addr_of_mut!(key_type_trusted),
        trusted_desc,
        core::ptr::null(),
    );
    if IS_ERR(tkey as *const c_void) {
        return tkey;
    }

    down_read(core::ptr::addr_of_mut!((*tkey).sem));
    tpayload = (*tkey).payload.data[0] as *mut trusted_key_payload;
    *master_key = (*tpayload).key;
    *master_keylen = (*tpayload).key_len;

    tkey
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
