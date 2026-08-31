// SPDX-License-Identifier: GPL-2.0-or-later
// C source included <linux/mmzone.h>; pglist_data and atomic_long_t are supplied externally.

extern "C" {
    pub type pglist_data;
    pub type atomic_long_t;
}

#[no_mangle]
pub unsafe extern "C" fn first_online_pgdat() -> *mut pglist_data {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn next_online_pgdat(pgdat: *mut pglist_data) -> *mut pglist_data {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn atomic_long_set(v: *mut atomic_long_t, i: core::ffi::c_long) {}
