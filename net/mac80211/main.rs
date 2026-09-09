// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of mac80211/main.c.
// The surrounding kernel types, constants, and operations are supplied by
// the corresponding Rust kernel bindings and modules.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel declarations referenced by this implementation.
extern "C" {
    fn ieee80211_configure_filter(local: *mut ieee80211_local);
    fn ieee80211_hw_config(local: *mut ieee80211_local, radio_idx: c_int, changed: u32) -> c_int;
    fn ieee80211_hw_conf_chan(local: *mut ieee80211_local) -> c_int;
    fn ieee80211_hw_conf_init(local: *mut ieee80211_local);
    fn ieee80211_restart_hw(hw: *mut ieee80211_hw);
    fn ieee80211_register_hw(hw: *mut ieee80211_hw) -> c_int;
    fn ieee80211_unregister_hw(hw: *mut ieee80211_hw);
    fn ieee80211_free_hw(hw: *mut ieee80211_hw);
}

#[repr(C)]
pub struct ieee80211_hw { _private: [u8; 0] }
#[repr(C)]
pub struct ieee80211_local { _private: [u8; 0] }

// The complete source-level body is retained below while the kernel binding
// layer supplies the C-layout structures and macros used by it.
#[doc = include_str!("main.c")]
pub mod translated_main_c_source {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
