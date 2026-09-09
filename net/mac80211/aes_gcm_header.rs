/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2014-2015, Qualcomm Atheros, Inc.
 */

// Dependency: declarations from "aead_api.h" are supplied by other files.

pub const GCM_AAD_LEN: usize = 32;

#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

pub type __be16 = u16;

extern "C" {
    fn aead_encrypt(
        tfm: *mut crypto_aead,
        j_0: *mut u8,
        aad: *mut u8,
        aad_len: usize,
        data: *mut u8,
        data_len: usize,
        mic: *mut u8,
    ) -> i32;

    fn aead_decrypt(
        tfm: *mut crypto_aead,
        j_0: *mut u8,
        aad: *mut u8,
        aad_len: usize,
        data: *mut u8,
        data_len: usize,
        mic: *mut u8,
    ) -> i32;

    fn aead_key_setup_encrypt(
        cipher: *const u8,
        key: *const u8,
        key_len: usize,
        mic_len: usize,
    ) -> *mut crypto_aead;

    fn aead_key_free(tfm: *mut crypto_aead);

    fn be16_to_cpup(p: *const __be16) -> usize;
}

pub unsafe fn ieee80211_aes_gcm_encrypt(
    tfm: *mut crypto_aead,
    j_0: *mut u8,
    aad: *mut u8,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    aead_encrypt(
        tfm,
        j_0,
        aad.add(2),
        be16_to_cpup(aad as *const __be16),
        data,
        data_len,
        mic,
    )
}

pub unsafe fn ieee80211_aes_gcm_decrypt(
    tfm: *mut crypto_aead,
    j_0: *mut u8,
    aad: *mut u8,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    aead_decrypt(
        tfm,
        j_0,
        aad.add(2),
        be16_to_cpup(aad as *const __be16),
        data,
        data_len,
        mic,
    )
}

pub unsafe fn ieee80211_aes_gcm_key_setup_encrypt(
    key: *const u8,
    key_len: usize,
) -> *mut crypto_aead {
    aead_key_setup_encrypt(
        b"gcm(aes)\0".as_ptr(),
        key,
        key_len,
        IEEE80211_GCMP_MIC_LEN,
    )
}

pub unsafe fn ieee80211_aes_gcm_key_free(tfm: *mut crypto_aead) {
    aead_key_free(tfm)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
