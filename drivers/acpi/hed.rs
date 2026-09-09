// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI Hardware Error Device (PNP0C33) Driver
 *
 * Copyright (C) 2010, Intel Corp.
 *	Author: Huang Ying <ying.huang@intel.com>
 *
 * ACPI Hardware Error Device is used to report some hardware errors
 * notified via SCI, mainly the corrected errors.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct acpi_device_id {
    pub id: *const u8,
    pub driver_data: usize,
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

pub type acpi_handle = *mut c_void;

const ACPI_DEVICE_NOTIFY: u32 = 0;

unsafe extern "C" {
    fn blocking_notifier_chain_register(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> i32;
    fn blocking_notifier_chain_unregister(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> i32;
    fn blocking_notifier_call_chain(
        nh: *mut blocking_notifier_head,
        val: u64,
        v: *mut c_void,
    ) -> i32;
    fn devm_acpi_install_notify_handler(
        dev: *mut device,
        handler_type: u32,
        handler: Option<unsafe extern "C" fn(acpi_handle, u32, *mut c_void)>,
        context: *mut c_void,
    ) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
}

static acpi_hed_ids: [acpi_device_id; 3] = [
    acpi_device_id { id: b"PNP0C33\0".as_ptr(), driver_data: 0 },
    acpi_device_id { id: b"\0".as_ptr(), driver_data: 0 },
    acpi_device_id { id: core::ptr::null(), driver_data: 0 },
];

static mut hed_present: bool = false;

static mut acpi_hed_notify_list: blocking_notifier_head = blocking_notifier_head {
    _private: [],
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_acpi_hed_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_register(&raw mut acpi_hed_notify_list, nb)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unregister_acpi_hed_notifier(nb: *mut notifier_block) {
    blocking_notifier_chain_unregister(&raw mut acpi_hed_notify_list, nb);
}

/*
 * SCI to report hardware error is forwarded to the listeners of HED,
 * it is used by HEST Generic Hardware Error Source with notify type
 * SCI.
 */
unsafe extern "C" fn acpi_hed_notify(_handle: acpi_handle, _event: u32, _data: *mut c_void) {
    blocking_notifier_call_chain(&raw mut acpi_hed_notify_list, 0, core::ptr::null_mut());
}

unsafe extern "C" fn acpi_hed_probe(pdev: *mut platform_device) -> i32 {
    let err: i32;

    /* Only one hardware error device */
    if hed_present {
        return -22;
    }

    err = devm_acpi_install_notify_handler(
        pdev as *mut device,
        ACPI_DEVICE_NOTIFY,
        Some(acpi_hed_notify),
        core::ptr::null_mut(),
    );
    if err != 0 {
        return err;
    }

    hed_present = true;
    0
}

unsafe extern "C" fn acpi_hed_remove(_pdev: *mut platform_device) {
    hed_present = false;
}

static mut acpi_hed_driver: platform_driver = platform_driver {
    probe: Some(acpi_hed_probe),
    remove: Some(acpi_hed_remove),
    driver: driver {
        name: b"acpi-hardware-error-device\0".as_ptr(),
        acpi_match_table: acpi_hed_ids.as_ptr(),
    },
};

unsafe extern "C" fn acpi_hed_driver_init() -> i32 {
    platform_driver_register(&raw mut acpi_hed_driver)
}

// subsys_initcall(acpi_hed_driver_init);

// MODULE_AUTHOR("Huang Ying");
// MODULE_DESCRIPTION("ACPI Hardware Error Device Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
