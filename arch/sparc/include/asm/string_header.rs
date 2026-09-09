/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header selects asm/string_64.h on sparc64 and asm/string_32.h
 * otherwise.  Those build-time dependencies are supplied by the surrounding
 * translation unit.
 */

use core::ffi::{c_char, c_int, c_void};

pub type __kernel_size_t = usize;

/* First the mem*() things. */
pub const __HAVE_ARCH_MEMMOVE: bool = true;
pub const __HAVE_ARCH_MEMCPY: bool = true;
pub const __HAVE_ARCH_MEMSET: bool = true;
pub const __HAVE_ARCH_MEMSCAN: bool = true;

unsafe extern "C" {
    pub fn memmove(
        destination: *mut c_void,
        source: *const c_void,
        size: __kernel_size_t,
    ) -> *mut c_void;

    pub fn memcmp(
        left: *const c_void,
        right: *const c_void,
        size: __kernel_size_t,
    ) -> c_int;

    pub fn strlen(string: *const c_char) -> __kernel_size_t;

    pub fn strncmp(
        left: *const c_char,
        right: *const c_char,
        size: __kernel_size_t,
    ) -> c_int;
}

/* The C macros expand to compiler builtins at the call site. */
#[inline]
pub unsafe fn memcpy(
    destination: *mut c_void,
    source: *const c_void,
    size: __kernel_size_t,
) -> *mut c_void {
    unsafe { core::ptr::copy_nonoverlapping(source as *const u8, destination as *mut u8, size) };
    destination
}

#[inline]
pub unsafe fn memset(
    destination: *mut c_void,
    value: c_int,
    count: __kernel_size_t,
) -> *mut c_void {
    unsafe { core::ptr::write_bytes(destination as *mut u8, value as u8, count) };
    destination
}

unsafe extern "C" {
    fn __memscan_zero(address: *mut c_void, size: usize) -> *mut c_void;
    fn __memscan_generic(address: *mut c_void, value: c_int, size: usize) -> *mut c_void;
}

/*
 * C's __builtin_constant_p(__char) condition is a compile-time property.
 * Callers needing that exact macro expansion should preserve it at their call
 * site; this function retains the same zero-specialized and generic branches.
 */
#[inline]
pub unsafe fn memscan(address: *mut c_void, value: c_int, size: usize) -> *mut c_void {
    if value == 0 {
        unsafe { __memscan_zero(address, size) }
    } else {
        unsafe { __memscan_generic(address, value, size) }
    }
}

/* Now the str*() stuff... */
pub const __HAVE_ARCH_STRLEN: bool = true;
pub const __HAVE_ARCH_STRNCMP: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
