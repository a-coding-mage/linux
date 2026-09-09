/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2015, Qualcomm Atheros, Inc.
 */

// Dependency supplied by the Linux crypto subsystem: `struct crypto_aead`.

#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

pub const GMAC_AAD_LEN: usize = 20;
pub const GMAC_NONCE_LEN: usize = 12;

extern "C" {
    pub fn ieee80211_aes_gmac_key_setup(
        key: *const u8,
        key_len: usize,
    ) -> *mut crypto_aead;

    pub fn ieee80211_aes_gmac(
        tfm: *mut crypto_aead,
        aad: *const u8,
        nonce: *mut u8,
        data: *const u8,
        data_len: usize,
        mic: *mut u8,
    ) -> i32;

    pub fn ieee80211_aes_gmac_key_free(tfm: *mut crypto_aead);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
