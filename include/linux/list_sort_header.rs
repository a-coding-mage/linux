/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/types.h>.
use core::ffi::c_void;

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

// The C declaration carries __attribute__((nonnull(2,3))).
pub type list_cmp_func_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    a: *const list_head,
    b: *const list_head,
) -> i32;

// The C declaration carries __attribute__((nonnull(2,3))).
unsafe extern "C" {
    pub fn list_sort(
        priv_: *mut c_void,
        head: *mut list_head,
        cmp: list_cmp_func_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
