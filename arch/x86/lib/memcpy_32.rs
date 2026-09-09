// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to <linux/string.h> and <linux/export.h> are
// supplied by the surrounding build.

use core::ffi::c_void;

unsafe extern "C" {
    fn __memcpy(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
    fn __memset(s: *mut c_void, c: i32, count: usize) -> *mut c_void;
}

// __visible void *memcpy(void *to, const void *from, size_t n)
#[no_mangle]
pub unsafe extern "C" fn memcpy(
    to: *mut c_void,
    from: *const c_void,
    n: usize,
) -> *mut c_void {
    unsafe { __memcpy(to, from, n) }
}
// EXPORT_SYMBOL(memcpy);

// __visible void *memset(void *s, int c, size_t count)
#[no_mangle]
pub unsafe extern "C" fn memset(
    s: *mut c_void,
    c: i32,
    count: usize,
) -> *mut c_void {
    unsafe { __memset(s, c, count) }
}
// EXPORT_SYMBOL(memset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
