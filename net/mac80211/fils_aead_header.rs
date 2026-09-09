/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * FILS AEAD for (Re)Association Request/Response frames
 * Copyright 2016, Qualcomm Atheros, Inc.
 */

// C header dependency declarations; definitions are supplied by other files.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_mgd_assoc_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_sub_if_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn fils_encrypt_assoc_req(
        skb: *mut sk_buff,
        assoc_data: *mut ieee80211_mgd_assoc_data,
    ) -> ::std::os::raw::c_int;

    pub fn fils_decrypt_assoc_resp(
        sdata: *mut ieee80211_sub_if_data,
        frame: *mut u8,
        frame_len: *mut usize,
        assoc_data: *mut ieee80211_mgd_assoc_data,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
