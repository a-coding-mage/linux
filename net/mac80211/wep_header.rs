/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Software WEP encryption implementation
 * Copyright 2002, Jouni Malinen <jkmaline@cc.hut.fi>
 * Copyright 2003, Instant802 Networks, Inc.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/skbuff.h, linux/types.h, ieee80211_i.h, and key.h.

#[repr(C)]
pub struct ieee80211_local {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arc4_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_rx_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_tx_data {
    _private: [u8; 0],
}

// These result types are supplied by ieee80211_i.h in the original source.
pub type ieee80211_rx_result = i32;
pub type ieee80211_tx_result = i32;

extern "C" {
    pub fn ieee80211_wep_init(local: *mut ieee80211_local);
    pub fn ieee80211_wep_encrypt_data(
        ctx: *mut arc4_ctx,
        rc4key: *mut u8,
        klen: usize,
        data: *mut u8,
        data_len: usize,
    ) -> i32;
    pub fn ieee80211_wep_encrypt(
        local: *mut ieee80211_local,
        skb: *mut sk_buff,
        key: *const u8,
        keylen: i32,
        keyidx: i32,
    ) -> i32;
    pub fn ieee80211_wep_decrypt_data(
        ctx: *mut arc4_ctx,
        rc4key: *mut u8,
        klen: usize,
        data: *mut u8,
        data_len: usize,
    ) -> i32;

    pub fn ieee80211_crypto_wep_decrypt(rx: *mut ieee80211_rx_data) -> ieee80211_rx_result;
    pub fn ieee80211_crypto_wep_encrypt(tx: *mut ieee80211_tx_data) -> ieee80211_tx_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
