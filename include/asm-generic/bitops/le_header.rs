/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <asm/types.h> and <asm/byteorder.h>.

#[cfg(target_endian = "little")]
const BITOP_LE_SWIZZLE: usize = 0;

// On big-endian targets, BITS_PER_LONG is supplied by the surrounding
// low-level environment, as it is in the original C header.
#[cfg(target_endian = "big")]
const BITOP_LE_SWIZZLE: usize = (BITS_PER_LONG - 1) & !0x7;

unsafe extern "C" {
    fn test_bit(nr: usize, addr: *const core::ffi::c_void) -> i32;
    fn set_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn clear_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn __set_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn __clear_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn test_and_set_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32;
    fn test_and_clear_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32;
    fn __test_and_set_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32;
    fn __test_and_clear_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32;
}

pub unsafe fn test_bit_le(nr: usize, addr: *const core::ffi::c_void) -> i32 {
    unsafe { test_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn set_bit_le(nr: usize, addr: *mut core::ffi::c_void) {
    unsafe { set_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn clear_bit_le(nr: usize, addr: *mut core::ffi::c_void) {
    unsafe { clear_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn __set_bit_le(nr: usize, addr: *mut core::ffi::c_void) {
    unsafe { __set_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn __clear_bit_le(nr: usize, addr: *mut core::ffi::c_void) {
    unsafe { __clear_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn test_and_set_bit_le(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    unsafe { test_and_set_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn test_and_clear_bit_le(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    unsafe { test_and_clear_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn __test_and_set_bit_le(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    unsafe { __test_and_set_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

pub unsafe fn __test_and_clear_bit_le(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    unsafe { __test_and_clear_bit(nr ^ BITOP_LE_SWIZZLE, addr) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
