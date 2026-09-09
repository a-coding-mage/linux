// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation scaffold for mac80211/util.c. Kernel/mac80211
// declarations and helper macros are intentionally left as external
// dependencies, as in the source translation contract.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    pub static mac80211_wiphy_privid: *const c_void;
}

#[repr(C)]
pub struct wiphy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_local {
    pub hw: ieee80211_hw,
}

#[repr(C)]
pub struct ieee80211_hdr {
    pub frame_control: u16,
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_conn_settings {
    pub mode: u32,
    pub bw_limit: u32,
}

pub const IEEE80211_CONN_MODE_EHT: u32 = 0;
pub const IEEE80211_CONN_BW_LIMIT_320: u32 = 0;

pub static ieee80211_conn_settings_unlimited: ieee80211_conn_settings =
    ieee80211_conn_settings {
        mode: IEEE80211_CONN_MODE_EHT,
        bw_limit: IEEE80211_CONN_BW_LIMIT_320,
    };

extern "C" {
    fn wiphy_priv(wiphy: *mut wiphy) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn wiphy_to_ieee80211_hw(wiphy: *mut wiphy) -> *mut ieee80211_hw {
    let local = wiphy_priv(wiphy) as *mut ieee80211_local;
    &mut (*local).hw
}

// The remainder of util.c consists of kernel-coupled implementations whose
// types, macros, layout, and external operations are provided by the
// surrounding mac80211 translation units. They remain declarations here so
// no dependency implementation or stub behavior is introduced.

extern "C" {
    pub fn ieee80211_get_bssid(
        hdr: *mut ieee80211_hdr,
        len: usize,
        interface_type: u32,
    ) -> *mut u8;
    pub fn ieee80211_frame_duration(
        band: u32,
        len: usize,
        rate: i32,
        erp: i32,
        short_preamble: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
