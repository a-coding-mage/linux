// SPDX-License-Identifier: GPL-2.0-or-later

use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    pub fn crypto_aegis128_init_neon(
        state: *mut c_void,
        key: *const c_void,
        iv: *const c_void,
    );
    pub fn crypto_aegis128_update_neon(state: *mut c_void, msg: *const c_void);
    pub fn crypto_aegis128_encrypt_chunk_neon(
        state: *mut c_void,
        dst: *mut c_void,
        src: *const c_void,
        size: c_uint,
    );
    pub fn crypto_aegis128_decrypt_chunk_neon(
        state: *mut c_void,
        dst: *mut c_void,
        src: *const c_void,
        size: c_uint,
    );
    pub fn crypto_aegis128_final_neon(
        state: *mut c_void,
        tag_xor: *mut c_void,
        assoclen: c_uint,
        cryptlen: c_uint,
        authsize: c_uint,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
