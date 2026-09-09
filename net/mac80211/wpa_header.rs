/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2002-2004, Instant802 Networks, Inc.
 * Copyright (C) 2022 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit:
// linux/skbuff.h, linux/types.h, and ieee80211_i.h.

extern "C" {
    pub fn ieee80211_tx_h_michael_mic_add(
        tx: *mut ieee80211_tx_data,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_rx_h_michael_mic_verify(
        rx: *mut ieee80211_rx_data,
    ) -> ieee80211_rx_result;

    pub fn ieee80211_crypto_tkip_encrypt(
        tx: *mut ieee80211_tx_data,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_crypto_tkip_decrypt(
        rx: *mut ieee80211_rx_data,
    ) -> ieee80211_rx_result;

    pub fn ieee80211_crypto_ccmp_encrypt(
        tx: *mut ieee80211_tx_data,
        mic_len: core::ffi::c_uint,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_crypto_ccmp_decrypt(
        rx: *mut ieee80211_rx_data,
        mic_len: core::ffi::c_uint,
    ) -> ieee80211_rx_result;

    pub fn ieee80211_crypto_aes_cmac_encrypt(
        tx: *mut ieee80211_tx_data,
        mic_len: core::ffi::c_uint,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_crypto_aes_cmac_decrypt(
        rx: *mut ieee80211_rx_data,
        mic_len: core::ffi::c_uint,
    ) -> ieee80211_rx_result;
    pub fn ieee80211_crypto_aes_gmac_encrypt(
        tx: *mut ieee80211_tx_data,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_crypto_aes_gmac_decrypt(
        rx: *mut ieee80211_rx_data,
    ) -> ieee80211_rx_result;

    pub fn ieee80211_crypto_gcmp_encrypt(
        tx: *mut ieee80211_tx_data,
    ) -> ieee80211_tx_result;
    pub fn ieee80211_crypto_gcmp_decrypt(
        rx: *mut ieee80211_rx_data,
    ) -> ieee80211_rx_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
