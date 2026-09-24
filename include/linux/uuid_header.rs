/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * UUID/GUID definition
 *
 * Copyright (C) 2010, 2016 Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

//! UUID/GUID declarations using the genuine kernel binding types.

pub use kernel::bindings::{guid_t, uuid_t};
use kernel::ffi::{c_char, c_int, c_uchar, c_void};

/// Size in bytes of either binary identifier.
pub const UUID_SIZE: usize = 16;

/// Constructs a binary identifier in the C header's byte order.
/// Each occurrence is widened before shifting, matching C integer promotion
/// and byte truncation without changing repeated argument evaluation.
#[macro_export]
macro_rules! GUID_INIT {
    ($a:expr, $b:expr, $c:expr, $d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr, $d7:expr) => {
        ::kernel::bindings::guid_t {
            b: [
                ((($a) as u32) & 0xff) as u8,
                (((($a) as u32) >> 8) & 0xff) as u8,
                (((($a) as u32) >> 16) & 0xff) as u8,
                (((($a) as u32) >> 24) & 0xff) as u8,
                ((($b) as u32) & 0xff) as u8,
                (((($b) as u32) >> 8) & 0xff) as u8,
                ((($c) as u32) & 0xff) as u8,
                (((($c) as u32) >> 8) & 0xff) as u8,
                ($d0) as u8,
                ($d1) as u8,
                ($d2) as u8,
                ($d3) as u8,
                ($d4) as u8,
                ($d5) as u8,
                ($d6) as u8,
                ($d7) as u8,
            ],
        }
    };
}

/// Constructs a binary identifier in the C header's byte order.
/// Each occurrence is widened before shifting, matching C integer promotion
/// and byte truncation without changing repeated argument evaluation.
#[macro_export]
macro_rules! UUID_INIT {
    ($a:expr, $b:expr, $c:expr, $d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr, $d7:expr) => {
        ::kernel::bindings::uuid_t {
            b: [
                (((($a) as u32) >> 24) & 0xff) as u8,
                (((($a) as u32) >> 16) & 0xff) as u8,
                (((($a) as u32) >> 8) & 0xff) as u8,
                ((($a) as u32) & 0xff) as u8,
                (((($b) as u32) >> 8) & 0xff) as u8,
                ((($b) as u32) & 0xff) as u8,
                (((($c) as u32) >> 8) & 0xff) as u8,
                ((($c) as u32) & 0xff) as u8,
                ($d0) as u8,
                ($d1) as u8,
                ($d2) as u8,
                ($d3) as u8,
                ($d4) as u8,
                ($d5) as u8,
                ($d6) as u8,
                ($d7) as u8,
            ],
        }
    };
}

/* The length of a UUID string ("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
 * not including trailing NUL. */
/// Number of validated text positions, excluding any terminator.
pub const UUID_STRING_LEN: usize = 36;

unsafe extern "C" {
    /// All-zero GUID.
    pub static guid_null: guid_t;
    /// All-zero UUID.
    pub static uuid_null: uuid_t;

    /// Compares readable byte regions under the C memcmp contract.
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    /// Copies nonoverlapping byte regions under the C memcpy contract.
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn guid_equal(u1: *const guid_t, u2: *const guid_t) -> bool {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe { memcmp(u1.cast(), u2.cast(), core::mem::size_of::<guid_t>()) == 0 }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn guid_copy(dst: *mut guid_t, src: *const guid_t) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn import_guid(dst: *mut guid_t, src: *const u8) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn export_guid(dst: *mut u8, src: *const guid_t) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn guid_is_null(guid: *const guid_t) -> bool {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe { guid_equal(guid, core::ptr::addr_of!(guid_null)) }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn uuid_equal(u1: *const uuid_t, u2: *const uuid_t) -> bool {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe { memcmp(u1.cast(), u2.cast(), core::mem::size_of::<uuid_t>()) == 0 }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn uuid_copy(dst: *mut uuid_t, src: *const uuid_t) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn import_uuid(dst: *mut uuid_t, src: *const u8) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn export_uuid(dst: *mut u8, src: *const uuid_t) {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe {
        memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
    }
}

/// Equivalent to the corresponding C UUID header helper.
///
/// # Safety
/// Pointers must satisfy the original C helper's readable/writable bounds and
/// memcpy nonoverlap requirements where applicable.
#[inline]
pub unsafe fn uuid_is_null(uuid: *const uuid_t) -> bool {
    // SAFETY: The caller supplies valid memory for the corresponding C helper.
    unsafe { uuid_equal(uuid, core::ptr::addr_of!(uuid_null)) }
}

unsafe extern "C" {
    /// Writes 16 random UUID bytes using the kernel RNG.
    pub fn generate_random_uuid(uuid: *mut c_uchar);
    /// Writes 16 random GUID bytes using the kernel RNG.
    pub fn generate_random_guid(guid: *mut c_uchar);
    /// Generates a GUID into a writable object.
    pub fn guid_gen(u: *mut guid_t);
    /// Generates a UUID into a writable object.
    pub fn uuid_gen(u: *mut uuid_t);
    /// Reads up to 36 positions, stopping at the first invalid byte.
    pub fn uuid_is_valid(uuid: *const c_char) -> bool;

    /// Text-pair to GUID byte mapping; global but not a module export.
    pub static guid_index: [u8; 16];
    /// Text-pair to UUID byte mapping; global but not a module export.
    pub static uuid_index: [u8; 16];

    /// Validates first, then writes each GUID byte in the C order.
    pub fn guid_parse(uuid: *const c_char, u: *mut guid_t) -> c_int;
    /// Validates first, then writes each UUID byte in the C order.
    pub fn uuid_parse(uuid: *const c_char, u: *mut uuid_t) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
