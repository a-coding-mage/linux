// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Microsoft Corporation
 *
 * Author: Lakshmi Ramasubramanian (nramas@linux.microsoft.com)
 *
 * File: ima_asymmetric_keys.c
 *       Defines an IMA hook to measure asymmetric keys on key
 *       create or update.
 */

// External includes: <keys/asymmetric-type.h>, <linux/user_namespace.h>, <linux/ima.h>, "ima.h"

use std::ffi::c_void;

#[repr(C)]
pub struct key_type {
    // Type definition from external header
}

#[repr(C)]
pub struct key {
    pub type_: *const key_type,
    pub description: *const i8,
}

extern "C" {
    pub static key_type_asymmetric: key_type;
    pub static nop_mnt_idmap: c_void;

    pub fn ima_should_queue_key() -> bool;
    pub fn ima_queue_key(keyring: *mut key, payload: *const c_void, payload_len: usize) -> bool;
    pub fn process_buffer_measurement(
        idmap: *const c_void,
        file: *mut c_void,
        buf: *const c_void,
        size: usize,
        eventname: *const i8,
        op: i32,
        xattr_value: i32,
        xattr_len: *const i8,
        update_flags: bool,
        filtered_rules: *mut c_void,
        pcr: i32,
    );
}

// KEY_CHECK constant from ima.h
const KEY_CHECK: i32 = 4;

/**
 * ima_post_key_create_or_update - measure asymmetric keys
 * @keyring: keyring to which the key is linked to
 * @key: created or updated key
 * @payload: The data used to instantiate or update the key.
 * @payload_len: The length of @payload.
 * @flags: key flags
 * @create: flag indicating whether the key was created or updated
 *
 * Keys can only be measured, not appraised.
 * The payload data used to instantiate or update the key is measured.
 */
pub extern "C" fn ima_post_key_create_or_update(
    keyring: *mut key,
    key: *mut key,
    payload: *const c_void,
    payload_len: usize,
    flags: usize,
    create: bool,
) {
    let mut queued = false;

    // Only asymmetric keys are handled by this hook.
    unsafe {
        if (*key).type_ != std::ptr::addr_of!(key_type_asymmetric) {
            return;
        }
    }

    if payload.is_null() || payload_len == 0 {
        return;
    }

    if unsafe { ima_should_queue_key() } {
        queued = unsafe { ima_queue_key(keyring, payload, payload_len) };
    }

    if queued {
        return;
    }

    /*
     * keyring->description points to the name of the keyring
     * (such as ".builtin_trusted_keys", ".ima", etc.) to
     * which the given key is linked to.
     *
     * The name of the keyring is passed in the "eventname"
     * parameter to process_buffer_measurement() and is set
     * in the "eventname" field in ima_event_data for
     * the key measurement IMA event.
     *
     * The name of the keyring is also passed in the "keyring"
     * parameter to process_buffer_measurement() to check
     * if the IMA policy is configured to measure a key linked
     * to the given keyring.
     */
    unsafe {
        process_buffer_measurement(
            std::ptr::addr_of!(nop_mnt_idmap) as *const c_void,
            std::ptr::null_mut(),
            payload,
            payload_len,
            (*keyring).description,
            KEY_CHECK,
            0,
            (*keyring).description,
            false,
            std::ptr::null_mut(),
            0,
        );
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
