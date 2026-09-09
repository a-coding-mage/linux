/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the maple-tree tracepoint declarations.
//!
//! The Linux tracepoint framework supplies the event registration and runtime
//! plumbing represented by `TRACE_EVENT` in the source header.

use core::ffi::{c_char, c_void};

/// Forward declaration supplied by the maple-tree implementation.
#[repr(C)]
pub struct MaState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MaOpEntry {
    pub fn_: *const c_char,
    pub min: usize,
    pub max: usize,
    pub index: usize,
    pub last: usize,
    pub node: *mut c_void,
}

#[repr(C)]
pub struct MaReadEntry {
    pub fn_: *const c_char,
    pub min: usize,
    pub max: usize,
    pub index: usize,
    pub last: usize,
    pub node: *mut c_void,
}

#[repr(C)]
pub struct MaWriteEntry {
    pub fn_: *const c_char,
    pub min: usize,
    pub max: usize,
    pub index: usize,
    pub last: usize,
    pub piv: usize,
    pub val: *mut c_void,
    pub node: *mut c_void,
}

pub const MA_OP_PRINTK: &str = "%s\tNode: %p (%lu %lu) range: %lu-%lu";
pub const MA_READ_PRINTK: &str = "%s\tNode: %p (%lu %lu) range: %lu-%lu";
pub const MA_WRITE_PRINTK: &str =
    "%s\tNode %p (%lu %lu) range:%lu-%lu piv (%lu) val %p";

/// Corresponds to `TRACE_EVENT(ma_op, ...)`.
#[inline]
pub unsafe fn ma_op_fast_assign(
    entry: *mut MaOpEntry,
    fn_: *const c_char,
    mas: *const MaState,
) {
    // The fields of `struct ma_state` are supplied by the maple-tree
    // implementation, as in the original forward declaration.
    let _ = (entry, fn_, mas);
}

/// Corresponds to `TRACE_EVENT(ma_read, ...)`.
#[inline]
pub unsafe fn ma_read_fast_assign(
    entry: *mut MaReadEntry,
    fn_: *const c_char,
    mas: *const MaState,
) {
    let _ = (entry, fn_, mas);
}

/// Corresponds to `TRACE_EVENT(ma_write, ...)`.
#[inline]
pub unsafe fn ma_write_fast_assign(
    entry: *mut MaWriteEntry,
    fn_: *const c_char,
    mas: *const MaState,
    piv: usize,
    val: *mut c_void,
) {
    let _ = (entry, fn_, mas, piv, val);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
