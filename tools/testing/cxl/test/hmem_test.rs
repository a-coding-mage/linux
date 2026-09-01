// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2026 Intel Corporation */

// C dependencies:
// #include <linux/moduleparam.h>
// #include <linux/workqueue.h>
// #include "../../../drivers/dax/bus.h"
// #include "mock.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

pub static mut hmem_test: bool = false;

#[repr(C)]
pub struct work_struct {
    _data: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub release: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: device,
}

#[repr(C)]
pub struct hmem_platform_device {
    pub pdev: platform_device,
    pub work: work_struct,
}

unsafe extern "C" {
    fn platform_device_register(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
}

static HMEM_PLATFORM_NAME: &[u8; 14] = b"hmem_platform\0";

unsafe extern "C" fn hmem_test_work(_work: *mut work_struct) {}

unsafe extern "C" fn hmem_test_release(dev: *mut device) {
    let hpdev = (dev as *mut u8).sub(offset_of!(hmem_platform_device, pdev) + offset_of!(platform_device, dev))
        as *mut hmem_platform_device;

    ptr::write_bytes(hpdev as *mut c_void, 0, size_of::<hmem_platform_device>());
}

// C initializer used __WORK_INITIALIZER(hmem_test_device.work, hmem_test_work).
// The workqueue storage is external-kernel-defined; this preserves the file-local
// object shape and zero-initialized storage for the translated global.
static mut hmem_test_device: hmem_platform_device = hmem_platform_device {
    pdev: platform_device {
        name: HMEM_PLATFORM_NAME.as_ptr() as *const c_char,
        id: 1,
        dev: device {
            release: Some(hmem_test_release),
        },
    },
    work: work_struct { _data: [] },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hmem_test_init() -> c_int {
    if !hmem_test {
        return 0;
    }

    platform_device_register(ptr::addr_of_mut!(hmem_test_device.pdev))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hmem_test_exit() {
    if hmem_test {
        platform_device_unregister(ptr::addr_of_mut!(hmem_test_device.pdev));
    }
}

// module_param(hmem_test, bool, 0444);
const _HMEM_TEST_MODULE_PARAM_MODE: c_uint = 0o444;
// MODULE_PARM_DESC(hmem_test, "Enable/disable the dax_hmem test platform device");
const _HMEM_TEST_MODULE_PARAM_DESC: &[u8; 55] =
    b"Enable/disable the dax_hmem test platform device\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
