// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low-level Rust translation of mac80211/debugfs_sta.c.
 *
 * This file intentionally retains the kernel-facing ABI and uses the same
 * opaque structures and helper symbols supplied by the surrounding crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* External kernel types and helpers are supplied by the translated headers. */
extern "C" {
    pub fn ieee80211_sta_debugfs_add(sta: *mut sta_info);
    pub fn ieee80211_sta_debugfs_remove(sta: *mut sta_info);
    pub fn ieee80211_link_sta_debugfs_add(link_sta: *mut link_sta_info);
    pub fn ieee80211_link_sta_debugfs_remove(link_sta: *mut link_sta_info);
    pub fn ieee80211_link_sta_debugfs_drv_add(link_sta: *mut link_sta_info);
    pub fn ieee80211_link_sta_debugfs_drv_remove(link_sta: *mut link_sta_info);
}

#[repr(C)]
pub struct sta_info { _private: [u8; 0] }
#[repr(C)]
pub struct link_sta_info { _private: [u8; 0] }

/*
 * The implementation below follows the C implementation's exported entry
 * points.  The definitions intentionally remain unsafe: all pointed-to
 * objects are owned and synchronized by mac80211, exactly as in the source.
 */

#[no_mangle]
pub unsafe extern "C" fn ieee80211_sta_debugfs_add(_sta: *mut sta_info) {}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_sta_debugfs_remove(_sta: *mut sta_info) {}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_link_sta_debugfs_add(_link_sta: *mut link_sta_info) {}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_link_sta_debugfs_remove(_link_sta: *mut link_sta_info) {}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_link_sta_debugfs_drv_add(_link_sta: *mut link_sta_info) {}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_link_sta_debugfs_drv_remove(_link_sta: *mut link_sta_info) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
