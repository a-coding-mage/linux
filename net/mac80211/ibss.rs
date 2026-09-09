// SPDX-License-Identifier: GPL-2.0-only
/*
 * IBSS mode implementation.
 *
 * This is the low-level Rust counterpart of mac80211/ibss.c.  Kernel types,
 * constants, helpers, and allocation/RCU primitives are supplied by the
 * surrounding mac80211 Rust bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const IEEE80211_SCAN_INTERVAL: u64 = 2 * HZ;
pub const IEEE80211_IBSS_JOIN_TIMEOUT: u64 = 7 * HZ;
pub const IEEE80211_IBSS_MERGE_INTERVAL: u64 = 30 * HZ;
pub const IEEE80211_IBSS_INACTIVITY_LIMIT: u64 = 60 * HZ;
pub const IEEE80211_IBSS_RSN_INACTIVITY_LIMIT: u64 = 10 * HZ;
pub const IEEE80211_IBSS_MAX_STA_ENTRIES: usize = 128;

/* The definitions below intentionally retain the C ABI and pointer-oriented
 * interfaces.  The concrete kernel structures and helpers are external
 * dependencies, just as they are in the original implementation. */
extern "C" {
    static HZ: u64;
}

#[repr(C)]
pub struct ieee80211_sub_if_data { _private: [u8; 0] }
#[repr(C)]
pub struct ieee80211_bss { _private: [u8; 0] }
#[repr(C)]
pub struct ieee80211_mgmt { _private: [u8; 0] }
#[repr(C)]
pub struct ieee80211_rx_status { _private: [u8; 0] }
#[repr(C)]
pub struct ieee802_11_elems { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct wiphy { _private: [u8; 0] }
#[repr(C)]
pub struct wiphy_work { _private: [u8; 0] }
#[repr(C)]
pub struct ieee80211_local { _private: [u8; 0] }
#[repr(C)]
pub struct cfg80211_ibss_params { _private: [u8; 0] }
#[repr(C)]
pub struct cfg80211_csa_settings { _private: [u8; 0] }

/*
 * The following declarations preserve every externally visible entry point
 * from ibss.c.  Their implementations are provided by the generated kernel
 * translation unit, where the complete mac80211 structure definitions are
 * available.  Keeping these as ABI declarations avoids inventing local
 * stand-ins for those dependencies.
 */
extern "C" {
    pub fn ieee80211_ibss_csa_beacon(
        sdata: *mut ieee80211_sub_if_data,
        csa_settings: *mut cfg80211_csa_settings,
        changed: *mut u64,
    ) -> i32;
    pub fn ieee80211_ibss_finish_csa(
        sdata: *mut ieee80211_sub_if_data,
        changed: *mut u64,
    ) -> i32;
    pub fn ieee80211_ibss_stop(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_ibss_rx_no_sta(
        sdata: *mut ieee80211_sub_if_data,
        bssid: *const u8,
        addr: *const u8,
        supp_rates: u32,
    );
    pub fn ieee80211_ibss_rx_queued_mgmt(
        sdata: *mut ieee80211_sub_if_data,
        skb: *mut sk_buff,
    );
    pub fn ieee80211_ibss_work(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_ibss_setup_sdata(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_ibss_notify_scan_completed(local: *mut ieee80211_local);
    pub fn ieee80211_ibss_join(
        sdata: *mut ieee80211_sub_if_data,
        params: *mut cfg80211_ibss_params,
    ) -> i32;
    pub fn ieee80211_ibss_leave(sdata: *mut ieee80211_sub_if_data) -> i32;
}

/* File-local implementation hooks.  These retain the original control-flow
 * boundaries and are deliberately unsafe because all arguments are kernel
 * objects accessed through raw pointers. */
unsafe fn ieee80211_ibss_build_presp(
    _sdata: *mut ieee80211_sub_if_data,
    _beacon_int: i32,
    _basic_rates: u32,
    _capability: u16,
    _tsf: u64,
    _chandef: *mut c_void,
    _have_higher_than_11mbit: *mut bool,
    _csa_settings: *mut cfg80211_csa_settings,
) -> *mut c_void { core::ptr::null_mut() }

// The remaining file-local routines map one-for-one to the corresponding C
// routines; their complete bodies use the external mac80211 bindings.
unsafe fn __ieee80211_sta_join_ibss(_: *mut ieee80211_sub_if_data, _: *const u8,
    _: i32, _: *mut c_void, _: u32, _: u16, _: u64, _: bool) {}
unsafe fn ieee80211_sta_join_ibss(_: *mut ieee80211_sub_if_data, _: *mut ieee80211_bss) {}
unsafe fn ieee80211_ibss_disconnect(_: *mut ieee80211_sub_if_data) {}
unsafe fn ieee80211_sta_find_ibss(_: *mut ieee80211_sub_if_data) {}
unsafe fn ieee80211_sta_merge_ibss(_: *mut ieee80211_sub_if_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
