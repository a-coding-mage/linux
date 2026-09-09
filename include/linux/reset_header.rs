/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of `linux/reset.h`.
//!
//! C-only includes and configuration guards are represented by Rust comments;
//! dependent kernel types and symbols are expected to be supplied externally.

use core::ffi::c_char;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)]
pub struct reset_control { _private: [u8; 0] }

pub type c_int = i32;
pub type c_uint = u32;
pub type bool_ = bool;

pub const ENOTSUPP: c_int = 524;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOENT: c_int = 2;

#[repr(C)]
pub struct reset_control_bulk_data {
    pub id: *const c_char,
    pub rstc: *mut reset_control,
}

pub const RESET_CONTROL_FLAGS_BIT_SHARED: c_uint = 1 << 0;
pub const RESET_CONTROL_FLAGS_BIT_OPTIONAL: c_uint = 1 << 1;
pub const RESET_CONTROL_FLAGS_BIT_ACQUIRED: c_uint = 1 << 2;
pub const RESET_CONTROL_FLAGS_BIT_DEASSERTED: c_uint = 1 << 3;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum reset_control_flags {
    RESET_CONTROL_EXCLUSIVE = RESET_CONTROL_FLAGS_BIT_ACQUIRED as i32,
    RESET_CONTROL_EXCLUSIVE_DEASSERTED = (RESET_CONTROL_FLAGS_BIT_ACQUIRED | RESET_CONTROL_FLAGS_BIT_DEASSERTED) as i32,
    RESET_CONTROL_EXCLUSIVE_RELEASED = 0,
    RESET_CONTROL_SHARED = RESET_CONTROL_FLAGS_BIT_SHARED as i32,
    RESET_CONTROL_SHARED_DEASSERTED = (RESET_CONTROL_FLAGS_BIT_SHARED | RESET_CONTROL_FLAGS_BIT_DEASSERTED) as i32,
    RESET_CONTROL_OPTIONAL_EXCLUSIVE = (RESET_CONTROL_FLAGS_BIT_OPTIONAL | RESET_CONTROL_FLAGS_BIT_ACQUIRED) as i32,
    RESET_CONTROL_OPTIONAL_EXCLUSIVE_DEASSERTED = (RESET_CONTROL_FLAGS_BIT_OPTIONAL | RESET_CONTROL_FLAGS_BIT_ACQUIRED | RESET_CONTROL_FLAGS_BIT_DEASSERTED) as i32,
    RESET_CONTROL_OPTIONAL_EXCLUSIVE_RELEASED = RESET_CONTROL_FLAGS_BIT_OPTIONAL as i32,
    RESET_CONTROL_OPTIONAL_SHARED = (RESET_CONTROL_FLAGS_BIT_OPTIONAL | RESET_CONTROL_FLAGS_BIT_SHARED) as i32,
    RESET_CONTROL_OPTIONAL_SHARED_DEASSERTED = (RESET_CONTROL_FLAGS_BIT_OPTIONAL | RESET_CONTROL_FLAGS_BIT_SHARED | RESET_CONTROL_FLAGS_BIT_DEASSERTED) as i32,
}

extern "C" {
    pub fn reset_control_reset(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_rearm(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_status(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_acquire(rstc: *mut reset_control) -> c_int;
    pub fn reset_control_release(rstc: *mut reset_control);
    pub fn reset_control_bulk_reset(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data) -> c_int;
    pub fn reset_control_bulk_assert(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data) -> c_int;
    pub fn reset_control_bulk_deassert(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data) -> c_int;
    pub fn reset_control_bulk_acquire(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data) -> c_int;
    pub fn reset_control_bulk_release(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data);
    pub fn __fwnode_reset_control_get(fwnode: *mut fwnode_handle, id: *const c_char, index: c_int, flags: reset_control_flags) -> *mut reset_control;
    pub fn __reset_control_get(dev: *mut device, id: *const c_char, index: c_int, flags: reset_control_flags) -> *mut reset_control;
    pub fn reset_control_put(rstc: *mut reset_control);
    pub fn __reset_control_bulk_get(dev: *mut device, num_rstcs: c_int, rstcs: *mut reset_control_bulk_data, flags: reset_control_flags) -> c_int;
    pub fn reset_control_bulk_put(num_rstcs: c_int, rstcs: *mut reset_control_bulk_data);
    pub fn __device_reset(dev: *mut device, optional: bool) -> c_int;
    pub fn __devm_reset_control_get(dev: *mut device, id: *const c_char, index: c_int, flags: reset_control_flags) -> *mut reset_control;
    pub fn __devm_reset_control_bulk_get(dev: *mut device, num_rstcs: c_int, rstcs: *mut reset_control_bulk_data, flags: reset_control_flags) -> c_int;
    pub fn devm_reset_control_array_get(dev: *mut device, flags: reset_control_flags) -> *mut reset_control;
    pub fn fwnode_reset_control_array_get(fwnode: *mut fwnode_handle, flags: reset_control_flags) -> *mut reset_control;
    pub fn reset_control_get_count(dev: *mut device) -> c_int;
}

extern "C" { pub fn of_fwnode_handle(node: *mut device_node) -> *mut fwnode_handle; }

#[inline] pub unsafe fn device_reset(dev: *mut device) -> c_int { __device_reset(dev, false) }
#[inline] pub unsafe fn device_reset_optional(dev: *mut device) -> c_int { __device_reset(dev, true) }

#[inline] pub unsafe fn reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control { __reset_control_get(dev, id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn reset_control_bulk_get_exclusive(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn reset_control_get_exclusive_released(dev: *mut device, id: *const c_char) -> *mut reset_control { __reset_control_get(dev, id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn reset_control_bulk_get_exclusive_released(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn reset_control_bulk_get_optional_exclusive_released(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn reset_control_get_shared(dev: *mut device, id: *const c_char) -> *mut reset_control { __reset_control_get(dev, id, 0, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn reset_control_bulk_get_shared(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control { __reset_control_get(dev, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn reset_control_bulk_get_optional_exclusive(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn reset_control_get_optional_shared(dev: *mut device, id: *const c_char) -> *mut reset_control { __reset_control_get(dev, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }
#[inline] pub unsafe fn reset_control_bulk_get_optional_shared(dev: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __reset_control_bulk_get(dev, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }

#[inline] pub unsafe fn of_reset_control_get_exclusive(n: *mut device_node, id: *const c_char) -> *mut reset_control { __fwnode_reset_control_get(of_fwnode_handle(n), id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn of_reset_control_get_optional_exclusive(n: *mut device_node, id: *const c_char) -> *mut reset_control { __fwnode_reset_control_get(of_fwnode_handle(n), id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn of_reset_control_get_shared(n: *mut device_node, id: *const c_char) -> *mut reset_control { __fwnode_reset_control_get(of_fwnode_handle(n), id, 0, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn of_reset_control_get_exclusive_by_index(n: *mut device_node, i: c_int) -> *mut reset_control { __fwnode_reset_control_get(of_fwnode_handle(n), core::ptr::null(), i, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn of_reset_control_get_shared_by_index(n: *mut device_node, i: c_int) -> *mut reset_control { __fwnode_reset_control_get(of_fwnode_handle(n), core::ptr::null(), i, reset_control_flags::RESET_CONTROL_SHARED) }

#[inline] pub unsafe fn devm_reset_control_get_exclusive(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_get_exclusive_deasserted(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE_DEASSERTED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_exclusive(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_get_exclusive_released(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_exclusive_released(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn devm_reset_control_get_optional_exclusive_released(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_optional_exclusive_released(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn devm_reset_control_get_shared(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn devm_reset_control_get_shared_deasserted(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_SHARED_DEASSERTED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_shared(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_shared_deasserted(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_SHARED_DEASSERTED) }
#[inline] pub unsafe fn devm_reset_control_get_optional_exclusive(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_get_optional_exclusive_deasserted(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE_DEASSERTED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_optional_exclusive(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_get_optional_shared(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }
#[inline] pub unsafe fn devm_reset_control_get_optional_shared_deasserted(d: *mut device, id: *const c_char) -> *mut reset_control { __devm_reset_control_get(d, id, 0, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED_DEASSERTED) }
#[inline] pub unsafe fn devm_reset_control_bulk_get_optional_shared(d: *mut device, n: c_int, r: *mut reset_control_bulk_data) -> c_int { __devm_reset_control_bulk_get(d, n, r, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }
#[inline] pub unsafe fn devm_reset_control_get_exclusive_by_index(d: *mut device, i: c_int) -> *mut reset_control { __devm_reset_control_get(d, core::ptr::null(), i, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_get_shared_by_index(d: *mut device, i: c_int) -> *mut reset_control { __devm_reset_control_get(d, core::ptr::null(), i, reset_control_flags::RESET_CONTROL_SHARED) }

#[inline] pub unsafe fn of_reset_control_get(n: *mut device_node, id: *const c_char) -> *mut reset_control { of_reset_control_get_exclusive(n, id) }
#[inline] pub unsafe fn of_reset_control_get_by_index(n: *mut device_node, i: c_int) -> *mut reset_control { of_reset_control_get_exclusive_by_index(n, i) }
#[inline] pub unsafe fn devm_reset_control_get(d: *mut device, id: *const c_char) -> *mut reset_control { devm_reset_control_get_exclusive(d, id) }
#[inline] pub unsafe fn devm_reset_control_get_optional(d: *mut device, id: *const c_char) -> *mut reset_control { devm_reset_control_get_optional_exclusive(d, id) }
#[inline] pub unsafe fn devm_reset_control_get_by_index(d: *mut device, i: c_int) -> *mut reset_control { devm_reset_control_get_exclusive_by_index(d, i) }

#[inline] pub unsafe fn devm_reset_control_array_get_exclusive(d: *mut device) -> *mut reset_control { devm_reset_control_array_get(d, reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_array_get_exclusive_released(d: *mut device) -> *mut reset_control { devm_reset_control_array_get(d, reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn devm_reset_control_array_get_shared(d: *mut device) -> *mut reset_control { devm_reset_control_array_get(d, reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn devm_reset_control_array_get_optional_exclusive(d: *mut device) -> *mut reset_control { devm_reset_control_array_get(d, reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn devm_reset_control_array_get_optional_shared(d: *mut device) -> *mut reset_control { devm_reset_control_array_get(d, reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }
#[inline] pub unsafe fn of_reset_control_array_get_exclusive(n: *mut device_node) -> *mut reset_control { fwnode_reset_control_array_get(of_fwnode_handle(n), reset_control_flags::RESET_CONTROL_EXCLUSIVE) }
#[inline] pub unsafe fn of_reset_control_array_get_exclusive_released(n: *mut device_node) -> *mut reset_control { fwnode_reset_control_array_get(of_fwnode_handle(n), reset_control_flags::RESET_CONTROL_EXCLUSIVE_RELEASED) }
#[inline] pub unsafe fn of_reset_control_array_get_shared(n: *mut device_node) -> *mut reset_control { fwnode_reset_control_array_get(of_fwnode_handle(n), reset_control_flags::RESET_CONTROL_SHARED) }
#[inline] pub unsafe fn of_reset_control_array_get_optional_exclusive(n: *mut device_node) -> *mut reset_control { fwnode_reset_control_array_get(of_fwnode_handle(n), reset_control_flags::RESET_CONTROL_OPTIONAL_EXCLUSIVE) }
#[inline] pub unsafe fn of_reset_control_array_get_optional_shared(n: *mut device_node) -> *mut reset_control { fwnode_reset_control_array_get(of_fwnode_handle(n), reset_control_flags::RESET_CONTROL_OPTIONAL_SHARED) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
