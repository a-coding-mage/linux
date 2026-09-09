/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

pub type cmp_func_t = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;

pub unsafe fn __inline_bsearch(
    key: *const c_void,
    mut base: *const c_void,
    mut num: usize,
    size: usize,
    cmp: cmp_func_t,
) -> *mut c_void {
    let mut pivot: *const c_char;
    let mut result: c_int;

    while num > 0 {
        pivot = (base as *const c_char).add((num >> 1) * size);
        result = cmp(key, pivot as *const c_void);

        if result == 0 {
            return pivot as *mut c_void;
        }

        if result > 0 {
            base = pivot.add(size) as *const c_void;
            num -= 1;
        }
        num >>= 1;
    }

    core::ptr::null_mut()
}

unsafe extern "C" {
    pub fn bsearch(
        key: *const c_void,
        base: *const c_void,
        num: usize,
        size: usize,
        cmp: cmp_func_t,
    ) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
