// SPDX-License-Identifier: ISC
//
// Source-level Rust translation of wireless/reg.c.
// Kernel-provided symbols and types are intentionally referenced but not
// implemented here; they are supplied by the surrounding translation.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const REG_ENFORCE_GRACE_MS: u32 = 60000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulatory_request {
    pub wiphy_idx: c_int,
    pub alpha2: [c_char; 2],
    pub initiator: c_uint,
    pub intersect: bool,
    pub processed: bool,
    pub country_ie_env: c_uint,
    pub user_reg_hint_type: c_uint,
}

#[repr(C)]
pub struct ieee80211_regdomain {
    pub n_reg_rules: c_uint,
    pub alpha2: [c_char; 2],
    pub dfs_region: c_uint,
    pub reg_rules: [ieee80211_reg_rule; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_freq_range {
    pub start_freq_khz: u32,
    pub end_freq_khz: u32,
    pub max_bandwidth_khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_power_rule {
    pub max_antenna_gain: i32,
    pub max_eirp: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_reg_rule {
    pub freq_range: ieee80211_freq_range,
    pub power_rule: ieee80211_power_rule,
    pub flags: u32,
    pub dfs_cac_ms: u32,
    pub psd: i8,
    pub has_wmm: bool,
    pub wmm_rule: [u8; 64],
}

#[repr(C)]
pub struct wiphy { _private: [u8; 0] }

#[repr(C)]
pub struct ieee80211_channel { _private: [u8; 0] }

#[repr(C)]
pub struct faux_device { _private: [u8; 0] }

#[repr(C)]
pub struct fwdb_header { pub magic: u32, pub version: u32 }

#[repr(C, packed)]
pub struct fwdb_country { pub alpha2: [u8; 2], pub coll_ptr: u16 }

#[repr(C, packed)]
pub struct fwdb_collection { pub len: u8, pub n_rules: u8, pub dfs_region: u8 }

#[repr(C, packed)]
pub struct fwdb_rule {
    pub len: u8, pub flags: u8, pub max_eirp: u16,
    pub start: u32, pub end: u32, pub max_bw: u32,
    pub cac_timeout: u16, pub wmm_ptr: u16,
}

pub const FWDB_MAGIC: u32 = 0x5247_4442;
pub const FWDB_VERSION: u32 = 20;

pub const REG_REQ_OK: c_uint = 0;
pub const REG_REQ_IGNORE: c_uint = 1;
pub const REG_REQ_INTERSECT: c_uint = 2;
pub const REG_REQ_ALREADY_SET: c_uint = 3;

static mut core_request_world: regulatory_request = regulatory_request {
    wiphy_idx: 0, alpha2: [b'0' as c_char, b'0' as c_char],
    initiator: 0, intersect: false, processed: true,
    country_ie_env: 0, user_reg_hint_type: 0,
};

pub static mut cfg80211_regdomain: *const ieee80211_regdomain = core::ptr::null();
static mut last_request: *mut regulatory_request = core::ptr::null_mut();
static mut reg_fdev: *mut faux_device = core::ptr::null_mut();
static mut reg_num_devs_support_basehint: c_int = 0;
static mut reg_is_indoor: bool = false;
static mut reg_is_indoor_portid: u32 = 0;
static mut regdb: *const fwdb_header = core::ptr::null();

#[inline]
pub fn is_world_regdom(alpha2: *const c_char) -> bool {
    if alpha2.is_null() { return false; }
    unsafe { *alpha2 == b'0' as c_char && *alpha2.add(1) == b'0' as c_char }
}

#[inline]
pub unsafe fn alpha2_equal(x: *const c_char, y: *const c_char) -> bool {
    if x.is_null() || y.is_null() { return false; }
    *x == *y && *x.add(1) == *y.add(1)
}

pub unsafe fn reg_is_valid_request(alpha2: *const c_char) -> bool {
    if last_request.is_null() || (*last_request).processed { return false; }
    alpha2_equal((*last_request).alpha2.as_ptr(), alpha2)
}

// Remaining functions retain the C implementation's ABI and are provided by
// the kernel translation layer, including regulatory workqueues, RCU, lists,
// firmware loading, notifier callbacks, and module registration.
extern "C" {
    pub fn reg_query_regdb_wmm(alpha2: *mut c_char, freq: c_int,
                               rule: *mut ieee80211_reg_rule) -> c_int;
    pub fn reg_reload_regdb() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
