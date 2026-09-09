/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn copy_page(to: *mut c_void, from: *mut c_void);
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut c_void,
    from: *mut c_void,
    vaddr: c_ulong,
    page: *mut page,
) {
    let _ = vaddr;
    let _ = page;
    copy_page(to, from);
}

type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
