/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from the original header:
// #include <linux/compiler.h>
// #include <linux/types.h>
// #include <asm/errno.h>

use core::ffi::{c_int, c_long, c_ulong, c_void};

/*
 * Original kernel header comment:
 *
 * Kernel pointers have redundant information, so we can use a
 * scheme where we can return either an error code or a normal
 * pointer with the same return value.
 *
 * This should be a per-architecture thing, to allow different
 * error and pointer decisions.
 *
 * Userspace note:
 * The same principle works for userspace, because 'error' pointers
 * fall down to the unused hole far from user space, as described
 * in Documentation/arch/x86/x86_64/mm.rst for x86_64 arch:
 *
 * 0000000000000000 - 00007fffffffffff (=47 bits) user space, different per mm hole caused by [48:63] sign extension
 * ffffffffffe00000 - ffffffffffffffff (=2 MB) unused hole
 *
 * It should be the same case for other architectures, because
 * this code is used in generic kernel code.
 */
pub const MAX_ERRNO: c_ulong = 4095;

#[inline]
pub const fn IS_ERR_VALUE(x: c_ulong) -> bool {
    x >= (0 as c_ulong).wrapping_sub(MAX_ERRNO)
}

#[inline]
pub fn ERR_PTR(error_: c_long) -> *mut c_void {
    error_ as isize as *mut c_void
}

#[inline]
pub fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as isize as c_long
}

#[inline]
pub fn IS_ERR(ptr: *const c_void) -> bool {
    IS_ERR_VALUE(ptr as c_ulong)
}

#[inline]
pub fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool {
    ptr.is_null() || IS_ERR_VALUE(ptr as c_ulong)
}

#[inline]
pub fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int {
    if IS_ERR(ptr) {
        PTR_ERR(ptr) as c_int
    } else {
        0
    }
}

/**
 * ERR_CAST - Explicitly cast an error-valued pointer to another pointer type
 * @ptr: The pointer to cast.
 *
 * Explicitly cast an error-valued pointer to another pointer type in such a
 * way as to make it clear that's what's going on.
 */
#[inline]
pub fn ERR_CAST(ptr: *const c_void) -> *mut c_void {
    /* cast away the const */
    ptr as *mut c_void
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
