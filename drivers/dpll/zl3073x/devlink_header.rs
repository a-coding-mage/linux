/* SPDX-License-Identifier: GPL-2.0-only */

// The C header guard is omitted; Rust modules provide equivalent item scoping.

// External dependency supplied by another translation unit/header.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn zl3073x_devm_alloc(dev: *mut device) -> *mut zl3073x_dev;

    pub fn zl3073x_devlink_register(zldev: *mut zl3073x_dev) -> i32;

    pub fn zl3073x_devlink_flash_notify(
        zldev: *mut zl3073x_dev,
        msg: *const core::ffi::c_char,
        component: *const core::ffi::c_char,
        done: u32,
        total: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
