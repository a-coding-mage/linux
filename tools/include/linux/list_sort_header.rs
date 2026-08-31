/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_void};

/* Dependency intent from C header: #include <linux/types.h> */

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

/* C attribute intent: callback arguments 2 and 3 are non-null. */
pub type list_cmp_func_t = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const list_head,
        *const list_head,
    ) -> c_int,
>;

unsafe extern "C" {
    /* C attribute intent: head and cmp are non-null. */
    pub fn list_sort(
        priv_: *mut c_void,
        head: *mut list_head,
        cmp: list_cmp_func_t,
    );
}
