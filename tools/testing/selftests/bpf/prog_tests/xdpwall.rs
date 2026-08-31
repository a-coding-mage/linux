// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include "test_progs.h"
// #include "xdpwall.skel.h"

#[repr(C)]
pub struct xdpwall {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn xdpwall__open_and_load() -> *mut xdpwall;
    fn xdpwall__destroy(skel: *mut xdpwall);
    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, msg: *const core::ffi::c_char);
}

pub unsafe fn test_xdpwall() {
    let skel: *mut xdpwall;

    skel = unsafe { xdpwall__open_and_load() };
    unsafe {
        ASSERT_OK_PTR(
            skel as *const core::ffi::c_void,
            c"Does LLVM have https://github.com/llvm/llvm-project/commit/ea72b0319d7b0f0c2fcf41d121afa5d031b319d5?".as_ptr(),
        );
    }

    unsafe { xdpwall__destroy(skel) };
}
