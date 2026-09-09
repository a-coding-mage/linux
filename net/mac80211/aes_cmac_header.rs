/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2008, Jouni Malinen <j@w1.fi>
 */

// Dependency supplied by the translated crypto/aes-cbc-macs.h header.

/// Opaque AES-CMAC key type declared by the external dependency.
#[repr(C)]
pub struct aes_cmac_key;

extern "C" {
    pub fn ieee80211_aes_cmac(
        key: *const aes_cmac_key,
        aad: *const u8,
        data: *const u8,
        data_len: usize,
        mic: *mut u8,
        mic_len: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
