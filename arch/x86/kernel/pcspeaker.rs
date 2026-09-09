// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux platform-device, error, and init
// interfaces are intentionally left as external Rust symbols/macros.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    fn platform_device_register_simple(
        name: *const c_char,
        id: i32,
        res: *const c_void,
        num: u32,
    ) -> *mut platform_device;
}

unsafe fn add_pcspkr() -> i32 {
    let pd: *mut platform_device;

    pd = platform_device_register_simple(b"pcspkr\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);

    PTR_ERR_OR_ZERO!(pd)
}

device_initcall!(add_pcspkr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
