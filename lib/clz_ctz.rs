// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/clz_ctz.c
 *
 * Copyright (C) 2013 Chanho Min <chanho.min@lge.com>
 *
 * The functions in this file aren't called directly, but are required by
 * GCC builtins such as __builtin_ctz, and therefore they can't be removed
 * despite appearing unreferenced in kernel source.
 *
 * __c[lt]z[sd]i2 can be overridden by linking arch-specific versions.
 */

// The Linux kernel declarations supplied by the surrounding build provide
// these functions.  The C weak and const attributes are retained by intent.
unsafe extern "C" {
    fn __ffs(val: usize) -> usize;
    fn fls(val: i32) -> i32;
    fn fls64(val: u64) -> i32;
    fn __ffs64(val: u64) -> usize;
}

// __weak __attribute_const__
pub unsafe fn __ctzsi2(val: i32) -> i32 {
    __ffs(val as usize) as i32
}
// EXPORT_SYMBOL(__ctzsi2);

// __weak __attribute_const__
pub unsafe fn __clzsi2(val: i32) -> i32 {
    32 - fls(val)
}
// EXPORT_SYMBOL(__clzsi2);

// __weak __attribute_const__
pub unsafe fn __clzdi2(val: u64) -> i32 {
    64 - fls64(val)
}
// EXPORT_SYMBOL(__clzdi2);

// __weak __attribute_const__
pub unsafe fn __ctzdi2(val: u64) -> i32 {
    __ffs64(val) as i32
}
// EXPORT_SYMBOL(__ctzdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
