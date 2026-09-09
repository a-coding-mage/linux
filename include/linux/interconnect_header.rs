/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018-2019, Linaro Ltd.
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Dependency intent from the C header: linux/mutex.h and linux/types.h.

/* macros for converting to icc units */
macro_rules! Bps_to_icc { ($x:expr) => { ($x) / 1000 }; }
macro_rules! kBps_to_icc { ($x:expr) => { $x }; }
macro_rules! MBps_to_icc { ($x:expr) => { ($x) * 1000 }; }
macro_rules! GBps_to_icc { ($x:expr) => { ($x) * 1000 * 1000 }; }
macro_rules! bps_to_icc { ($x:expr) => { 1 }; }
macro_rules! kbps_to_icc { ($x:expr) => { (($x) + 7) / 8 }; }
macro_rules! Mbps_to_icc { ($x:expr) => { ($x) * 1000 / 8 }; }
macro_rules! Gbps_to_icc { ($x:expr) => { ($x) * 1000 * 1000 / 8 }; }

/* macro to indicate dynamic id allocation */
const ICC_ALLOC_DYN_ID: i32 = -1;

#[repr(C)]
pub struct icc_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct icc_bulk_data - Data used for bulk icc operations.
 *
 * @path: reference to the interconnect path (internal use)
 * @name: the name from the "interconnect-names" DT property
 * @avg_bw: average bandwidth in icc units
 * @peak_bw: peak bandwidth in icc units
 */
#[repr(C)]
pub struct icc_bulk_data {
    pub path: *mut icc_path,
    pub name: *const core::ffi::c_char,
    pub avg_bw: u32,
    pub peak_bw: u32,
}

// The C condition is IS_ENABLED(CONFIG_INTERCONNECT). Set the Rust feature
// CONFIG_INTERCONNECT when the interconnect implementation is enabled.
#[cfg(feature = "CONFIG_INTERCONNECT")]
extern "C" {
    pub fn of_icc_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut icc_path;
    pub fn devm_of_icc_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut icc_path;
    pub fn devm_of_icc_bulk_get(dev: *mut device, num_paths: i32, paths: *mut icc_bulk_data) -> i32;
    pub fn of_icc_get_by_index(dev: *mut device, idx: i32) -> *mut icc_path;
    pub fn devm_of_icc_get_by_index(dev: *mut device, idx: i32) -> *mut icc_path;
    pub fn icc_put(path: *mut icc_path);
    pub fn icc_enable(path: *mut icc_path) -> i32;
    pub fn icc_disable(path: *mut icc_path) -> i32;
    pub fn icc_set_bw(path: *mut icc_path, avg_bw: u32, peak_bw: u32) -> i32;
    pub fn icc_set_tag(path: *mut icc_path, tag: u32);
    pub fn icc_get_name(path: *mut icc_path) -> *const core::ffi::c_char;
    pub fn of_icc_bulk_get(dev: *mut device, num_paths: i32, paths: *mut icc_bulk_data) -> i32;
    pub fn icc_bulk_put(num_paths: i32, paths: *mut icc_bulk_data);
    pub fn icc_bulk_set_bw(num_paths: i32, paths: *const icc_bulk_data) -> i32;
    pub fn icc_bulk_enable(num_paths: i32, paths: *const icc_bulk_data) -> i32;
    pub fn icc_bulk_disable(num_paths: i32, paths: *const icc_bulk_data);
}

#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn of_icc_get(_dev: *mut device, _name: *const core::ffi::c_char) -> *mut icc_path { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn devm_of_icc_get(_dev: *mut device, _name: *const core::ffi::c_char) -> *mut icc_path { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn of_icc_get_by_index(_dev: *mut device, _idx: i32) -> *mut icc_path { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn devm_of_icc_get_by_index(_dev: *mut device, _idx: i32) -> *mut icc_path { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_put(_path: *mut icc_path) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_enable(_path: *mut icc_path) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_disable(_path: *mut icc_path) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_set_bw(_path: *mut icc_path, _avg_bw: u32, _peak_bw: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_set_tag(_path: *mut icc_path, _tag: u32) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_get_name(_path: *mut icc_path) -> *const core::ffi::c_char { core::ptr::null() }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn of_icc_bulk_get(_dev: *mut device, _num_paths: i32, _paths: *mut icc_bulk_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn devm_of_icc_bulk_get(_dev: *mut device, _num_paths: i32, _paths: *mut icc_bulk_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_bulk_put(_num_paths: i32, _paths: *mut icc_bulk_data) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_bulk_set_bw(_num_paths: i32, _paths: *const icc_bulk_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_bulk_enable(_num_paths: i32, _paths: *const icc_bulk_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
pub unsafe fn icc_bulk_disable(_num_paths: i32, _paths: *const icc_bulk_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
