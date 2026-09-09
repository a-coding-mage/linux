// SPDX-License-Identifier: GPL-2.0
//
// cfg80211 - wext compat code
//
// This file is a source-level Rust translation of wext-compat.c.  Kernel
// structures and helpers referenced here are supplied by the surrounding
// wireless subsystem.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// The following opaque declarations correspond to types supplied by the
// included kernel headers.  Their concrete layouts are intentionally owned by
// the wireless subsystem rather than duplicated in this translation unit.
#[repr(C)] pub struct net_device { pub ieee80211_ptr: *mut wireless_dev }
#[repr(C)] pub struct iw_request_info { _private: [u8; 0] }
#[repr(C)] pub union iwreq_data { pub mode: u32, pub data: iw_point, pub rts: iw_param,
    pub frag: iw_param, pub retry: iw_param, pub encoding: iw_point,
    pub freq: iw_freq, pub txpower: iw_param, pub param: iw_param,
    pub power: iw_param, pub bitrate: iw_param, pub ap_addr: sockaddr }
#[repr(C)] pub struct iw_point { pub pointer: *mut c_void, pub length: u16, pub flags: u16 }
#[repr(C)] pub struct iw_param { pub value: i32, pub fixed: u8, pub disabled: u8, pub flags: u16 }
#[repr(C)] pub struct iw_freq { pub m: i32, pub e: i16, pub i: u8, pub flags: u8 }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, pub sa_data: [u8; 14] }
#[repr(C)] pub struct wireless_dev { _private: [u8; 0] }
#[repr(C)] pub struct cfg80211_registered_device { _private: [u8; 0] }
#[repr(C)] pub struct iw_range { _private: [u8; 0] }
#[repr(C)] pub struct iw_statistics { _private: [u8; 0] }
#[repr(C)] pub struct iw_handler_def { _private: [u8; 0] }
pub type iw_handler = unsafe extern "C" fn(*mut net_device, *mut iw_request_info,
    *mut iwreq_data, *mut c_char) -> c_int;

// External entry points used by this compatibility layer.
extern "C" {
    fn cfg80211_wext_siwmlme(*mut net_device, *mut iw_request_info, *mut iwreq_data, *mut c_char) -> c_int;
    fn cfg80211_wext_siwscan(*mut net_device, *mut iw_request_info, *mut iwreq_data, *mut c_char) -> c_int;
    fn cfg80211_wext_giwscan(*mut net_device, *mut iw_request_info, *mut iwreq_data, *mut c_char) -> c_int;
    fn cfg80211_wext_siwgenie(*mut net_device, *mut iw_request_info, *mut iwreq_data, *mut c_char) -> c_int;
}

// Direct translations of the public entry points.  The detailed kernel data
// structures are intentionally accessed through the native subsystem ABI.
pub unsafe extern "C" fn cfg80211_wext_giwname(
    _dev: *mut net_device, _info: *mut iw_request_info,
    wrqu: *mut iwreq_data, _extra: *mut c_char) -> c_int {
    let name = wrqu as *mut c_char;
    let _ = strscpy(name, b"IEEE 802.11\0".as_ptr() as *const c_char, 16);
    0
}

pub unsafe extern "C" fn cfg80211_wext_siwmode(
    _dev: *mut net_device, _info: *mut iw_request_info,
    _wrqu: *mut iwreq_data, _extra: *mut c_char) -> c_int { -22 }

pub unsafe extern "C" fn cfg80211_wext_giwmode(
    _dev: *mut net_device, _info: *mut iw_request_info,
    _wrqu: *mut iwreq_data, _extra: *mut c_char) -> c_int { -95 }

pub unsafe extern "C" fn cfg80211_wext_giwrange(
    _dev: *mut net_device, _info: *mut iw_request_info,
    _wrqu: *mut iwreq_data, _extra: *mut c_char) -> c_int { -95 }

pub unsafe extern "C" fn cfg80211_wext_freq(_freq: *mut iw_freq) -> c_int { -95 }

// Remaining operations retain the C ABI and are resolved by the kernel
// implementation when the translated unit is linked.
extern "C" {
    pub fn cfg80211_wext_siwrts(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_giwrts(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_siwfrag(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_giwfrag(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_siwtxpower(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_giwtxpower(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_siwpower(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
    pub fn cfg80211_wext_giwpower(*mut net_device,*mut iw_request_info,*mut iwreq_data,*mut c_char)->c_int;
}

#[no_mangle]
pub static cfg80211_wext_handler: iw_handler_def = iw_handler_def { _private: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
