/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _KUNIT_PLATFORM_DRIVER_H

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct completion {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device_info {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn kunit_platform_device_alloc(
        test: *mut kunit,
        name: *const c_char,
        id: c_int,
    ) -> *mut platform_device;

    pub fn kunit_platform_device_add(test: *mut kunit, pdev: *mut platform_device) -> c_int;

    pub fn kunit_platform_device_register_full(
        test: *mut kunit,
        pdevinfo: *const platform_device_info,
    ) -> *mut platform_device;

    pub fn kunit_platform_device_unregister(test: *mut kunit, pdev: *mut platform_device);

    pub fn kunit_platform_device_prepare_wait_for_probe(
        test: *mut kunit,
        pdev: *mut platform_device,
        x: *mut completion,
    ) -> c_int;

    pub fn kunit_platform_driver_register(test: *mut kunit, drv: *mut platform_driver) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
