// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "test_progs.h"
// #include "core_kern_overflow.lskel.h"

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct core_kern_overflow_lskel {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn core_kern_overflow_lskel__open_and_load() -> *mut core_kern_overflow_lskel;
    fn core_kern_overflow_lskel__destroy(skel: *mut core_kern_overflow_lskel);
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
}

pub unsafe fn test_core_kern_overflow_lskel() {
    let skel: *mut core_kern_overflow_lskel;

    skel = unsafe { core_kern_overflow_lskel__open_and_load() };
    if !unsafe { ASSERT_NULL(skel as *const c_void, b"open_and_load\0".as_ptr() as *const c_char) } {
        unsafe { core_kern_overflow_lskel__destroy(skel) };
    }
}
