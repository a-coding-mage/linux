/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency declarations from <crypto/aead.h> and <linux/crypto.h> are
// supplied by other translated files.

use core::ffi::c_char;

#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

extern "C" {
    pub fn aead_key_setup_encrypt(
        alg: *const c_char,
        key: *const u8,
        key_len: usize,
        mic_len: usize,
    ) -> *mut crypto_aead;

    pub fn aead_encrypt(
        tfm: *mut crypto_aead,
        b_0: *mut u8,
        aad: *mut u8,
        aad_len: usize,
        data: *mut u8,
        data_len: usize,
        mic: *mut u8,
    ) -> core::ffi::c_int;

    pub fn aead_decrypt(
        tfm: *mut crypto_aead,
        b_0: *mut u8,
        aad: *mut u8,
        aad_len: usize,
        data: *mut u8,
        data_len: usize,
        mic: *mut u8,
    ) -> core::ffi::c_int;

    pub fn aead_key_free(tfm: *mut crypto_aead);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
