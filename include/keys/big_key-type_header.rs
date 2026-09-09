/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Big capacity key type.
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency supplied by the translated kernel key-type declarations.

extern "C" {
    pub static mut key_type_big_key: key_type;

    pub fn big_key_preparse(prep: *mut key_preparsed_payload) -> core::ffi::c_int;
    pub fn big_key_free_preparse(prep: *mut key_preparsed_payload);
    pub fn big_key_revoke(key: *mut key);
    pub fn big_key_destroy(key: *mut key);
    pub fn big_key_describe(big_key: *const key, m: *mut seq_file);
    pub fn big_key_read(
        key: *const key,
        buffer: *mut core::ffi::c_char,
        buflen: usize,
    ) -> core::ffi::c_long;
    pub fn big_key_update(
        key: *mut key,
        prep: *mut key_preparsed_payload,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
