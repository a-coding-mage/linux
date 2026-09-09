/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2002-2004, Instant802 Networks, Inc.
 * Copyright 2005, Devicescape Software, Inc.
 * Copyright (C) 2019, 2022-2023 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel/mac80211 translation.

pub const NUM_DEFAULT_KEYS: usize = 4;
pub const NUM_DEFAULT_MGMT_KEYS: usize = 2;
pub const NUM_DEFAULT_BEACON_KEYS: usize = 2;
pub const INVALID_PTK_KEYIDX: usize = 2; /* Keyidx always pointing to a NULL key for PTK */

pub const KEY_FLAG_UPLOADED_TO_HARDWARE: u32 = 1u32 << 0;
pub const KEY_FLAG_TAINTED: u32 = 1u32 << 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_internal_key_flags {
    KEY_FLAG_UPLOADED_TO_HARDWARE = 1 << 0,
    KEY_FLAG_TAINTED = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_internal_tkip_state {
    TKIP_STATE_NOT_INIT,
    TKIP_STATE_PHASE1_DONE,
    TKIP_STATE_PHASE1_HW_UPLOADED,
}

#[repr(C)]
pub struct tkip_ctx {
    pub p1k: [u16; 5], /* p1k cache */
    pub p1k_iv32: u32, /* iv32 for which p1k computed */
    pub state: ieee80211_internal_tkip_state,
}

#[repr(C)]
pub struct tkip_ctx_rx {
    pub ctx: tkip_ctx,
    pub iv32: u32, /* current iv32 */
    pub iv16: u16, /* current iv16 */
}

#[repr(C)]
pub struct ieee80211_key_tkip {
    /* protects tx context */
    pub txlock: spinlock_t,
    /* last used TSC */
    pub tx: tkip_ctx,
    /* last received RSC */
    pub rx: [tkip_ctx_rx; IEEE80211_NUM_TIDS],
    /* number of mic failures */
    pub mic_failures: u32,
}

#[repr(C)]
pub struct ieee80211_key_ccmp {
    /* Last received packet number. The first IEEE80211_NUM_TIDS counters
     * are used with Data frames and the last counter is used with Robust
     * Management frames. */
    pub rx_pn: [[u8; IEEE80211_CCMP_PN_LEN]; IEEE80211_NUM_TIDS + 1],
    pub tfm: *mut crypto_aead,
    pub replays: u32, /* dot11RSNAStatsCCMPReplays */
}

#[repr(C)]
pub struct ieee80211_key_aes_cmac {
    pub rx_pn: [u8; IEEE80211_CMAC_PN_LEN],
    pub key: aes_cmac_key,
    pub replays: u32, /* dot11RSNAStatsCMACReplays */
    pub icverrors: u32, /* dot11RSNAStatsCMACICVErrors */
}

#[repr(C)]
pub struct ieee80211_key_aes_gmac {
    pub rx_pn: [u8; IEEE80211_GMAC_PN_LEN],
    pub tfm: *mut crypto_aead,
    pub replays: u32, /* dot11RSNAStatsCMACReplays */
    pub icverrors: u32, /* dot11RSNAStatsCMACICVErrors */
}

#[repr(C)]
pub struct ieee80211_key_gcmp {
    /* Last received packet number. The first IEEE80211_NUM_TIDS counters
     * are used with Data frames and the last counter is used with Robust
     * Management frames. */
    pub rx_pn: [[u8; IEEE80211_GCMP_PN_LEN]; IEEE80211_NUM_TIDS + 1],
    pub tfm: *mut crypto_aead,
    pub replays: u32, /* dot11RSNAStatsGCMPReplays */
}

#[repr(C)]
pub struct ieee80211_key_gen {
    /* generic cipher scheme */
    pub rx_pn: [[u8; IEEE80211_MAX_PN_LEN]; IEEE80211_NUM_TIDS + 1],
}

#[repr(C)]
pub union ieee80211_key_u {
    pub tkip: core::mem::ManuallyDrop<ieee80211_key_tkip>,
    pub ccmp: core::mem::ManuallyDrop<ieee80211_key_ccmp>,
    pub aes_cmac: core::mem::ManuallyDrop<ieee80211_key_aes_cmac>,
    pub aes_gmac: core::mem::ManuallyDrop<ieee80211_key_aes_gmac>,
    pub gcmp: core::mem::ManuallyDrop<ieee80211_key_gcmp>,
    pub gen: core::mem::ManuallyDrop<ieee80211_key_gen>,
}

#[repr(C)]
pub struct ieee80211_key {
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub sta: *mut sta_info,
    /* for sdata list */
    pub list: list_head,
    /* protected by key mutex */
    pub flags: core::ffi::c_uint,
    pub u: ieee80211_key_u,
    #[cfg(feature = "CONFIG_MAC80211_DEBUGFS")]
    pub debugfs: ieee80211_key_debugfs,
    pub color: core::ffi::c_uint,
    /* key config, must be last because it contains key material as variable length member */
    pub conf: ieee80211_key_conf,
}

#[cfg(feature = "CONFIG_MAC80211_DEBUGFS")]
#[repr(C)]
pub struct ieee80211_key_debugfs {
    pub stalink: *mut dentry,
    pub dir: *mut dentry,
    pub cnt: i32,
}

extern "C" {
    pub fn ieee80211_key_alloc(cipher: u32, idx: i32, key_len: usize,
                               key_data: *const u8, seq_len: usize,
                               seq: *const u8) -> *mut ieee80211_key;
    pub fn ieee80211_key_link(key: *mut ieee80211_key, link: *mut ieee80211_link_data,
                              sta: *mut sta_info) -> i32;
    pub fn ieee80211_set_tx_key(key: *mut ieee80211_key) -> i32;
    pub fn ieee80211_key_free(key: *mut ieee80211_key, delay_tailroom: bool);
    pub fn ieee80211_key_free_unused(key: *mut ieee80211_key);
    pub fn ieee80211_set_default_key(link: *mut ieee80211_link_data, idx: i32, uni: bool, multi: bool);
    pub fn ieee80211_set_default_mgmt_key(link: *mut ieee80211_link_data, idx: i32);
    pub fn ieee80211_set_default_beacon_key(link: *mut ieee80211_link_data, idx: i32);
    pub fn ieee80211_remove_link_keys(link: *mut ieee80211_link_data, keys: *mut list_head);
    pub fn ieee80211_free_key_list(local: *mut ieee80211_local, keys: *mut list_head);
    pub fn ieee80211_free_keys(sdata: *mut ieee80211_sub_if_data, force_synchronize: bool);
    pub fn ieee80211_free_sta_keys(local: *mut ieee80211_local, sta: *mut sta_info);
    pub fn ieee80211_reenable_keys(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_key_switch_links(sdata: *mut ieee80211_sub_if_data,
                                      del_links_mask: c_ulong, add_links_mask: c_ulong) -> i32;
    pub fn ieee80211_delayed_tailroom_dec(wiphy: *mut wiphy, wk: *mut wiphy_work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
