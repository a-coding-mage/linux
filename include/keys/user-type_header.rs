/* SPDX-License-Identifier: GPL-2.0-or-later */
/* user-type.h: User-defined key type
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* The declarations below are conditional on CONFIG_KEYS in the C header. */

#[repr(C)]
pub struct user_key_payload {
    pub rcu: rcu_head,
    pub datalen: u16,
    pub data: [core::ffi::c_char; 0],
}

extern "C" {
    pub static mut key_type_user: key_type;
    pub static mut key_type_logon: key_type;

    pub fn user_preparse(prep: *mut key_preparsed_payload) -> core::ffi::c_int;
    pub fn user_free_preparse(prep: *mut key_preparsed_payload);
    pub fn user_update(
        key: *mut key,
        prep: *mut key_preparsed_payload,
    ) -> core::ffi::c_int;
    pub fn user_revoke(key: *mut key);
    pub fn user_destroy(key: *mut key);
    pub fn user_describe(user: *const key, m: *mut seq_file);
    pub fn user_read(
        key: *const key,
        buffer: *mut core::ffi::c_char,
        buflen: usize,
    ) -> isize;
}

#[inline]
pub unsafe fn user_key_payload_rcu(
    key: *const key,
) -> *const user_key_payload {
    dereference_key_rcu(key) as *const user_key_payload
}

#[inline]
pub unsafe fn user_key_payload_locked(
    key: *const key,
) -> *mut user_key_payload {
    dereference_key_locked(key as *mut key) as *mut user_key_payload
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
