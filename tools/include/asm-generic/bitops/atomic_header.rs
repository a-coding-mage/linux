/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header dependencies:
 * #include <asm/types.h>
 * #include <asm/bitsperlong.h>
 */

/*
 * Just alias the test versions, all of the compiler built-in atomics "fetch",
 * and optimizing compile-time constants on x86 isn't worth the complexity.
 */
unsafe extern "C" {
    pub fn test_and_set_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn test_and_clear_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn set_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    unsafe { test_and_set_bit(nr, addr) }
}

#[inline]
pub unsafe fn clear_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    unsafe { test_and_clear_bit(nr, addr) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
