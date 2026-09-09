/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2003-2004, Instant802 Networks, Inc.
 * Copyright 2006, Devicescape Software, Inc.
 */

// Dependency intent from aead_api.h is preserved through the external symbols
// referenced below.

pub const CCM_AAD_LEN: usize = 32;

pub unsafe fn ieee80211_aes_key_setup_encrypt(
    key: *const u8,
    key_len: usize,
    mic_len: usize,
) -> *mut crate::crypto_aead {
    crate::aead_key_setup_encrypt(
        b"ccm(aes)\0".as_ptr() as *const i8,
        key,
        key_len,
        mic_len,
    )
}

pub unsafe fn ieee80211_aes_ccm_encrypt(
    tfm: *mut crate::crypto_aead,
    b_0: *mut u8,
    aad: *mut u8,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    crate::aead_encrypt(
        tfm,
        b_0,
        aad.add(2),
        crate::be16_to_cpup(aad as *const u16),
        data,
        data_len,
        mic,
    )
}

pub unsafe fn ieee80211_aes_ccm_decrypt(
    tfm: *mut crate::crypto_aead,
    b_0: *mut u8,
    aad: *mut u8,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    crate::aead_decrypt(
        tfm,
        b_0,
        aad.add(2),
        crate::be16_to_cpup(aad as *const u16),
        data,
        data_len,
        mic,
    )
}

pub unsafe fn ieee80211_aes_key_free(tfm: *mut crate::crypto_aead) {
    crate::aead_key_free(tfm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
