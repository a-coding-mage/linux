// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * TI PWM Subsystem driver
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel bindings:
// linux/module.h, linux/platform_device.h, linux/io.h, linux/err.h,
// linux/pm_runtime.h, linux/of_platform.h

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn of_platform_populate(
        root: *mut device_node,
        matches: *const core::ffi::c_void,
        lookup: *const core::ffi::c_void,
        parent: *mut device,
    ) -> i32;
    fn dev_err(dev: *mut device, format: *const u8, ...);
}

static pwmss_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,am33xx-pwmss\0".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

unsafe extern "C" fn pwmss_probe(pdev: *mut platform_device) -> i32 {
    let ret: i32;
    let node: *mut device_node = (*pdev).dev.of_node;

    pm_runtime_enable(&mut (*pdev).dev);

    /* Populate all the child nodes here... */
    ret = of_platform_populate(
        node,
        core::ptr::null(),
        core::ptr::null(),
        &mut (*pdev).dev,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"no child node found\n\0".as_ptr());
    }

    ret
}

unsafe extern "C" fn pwmss_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static mut pwmss_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"pwmss\0".as_ptr(),
        of_match_table: pwmss_of_match.as_ptr(),
    },
    probe: Some(pwmss_probe),
    remove: Some(pwmss_remove),
};

// Equivalent of module_platform_driver(pwmss_driver).

// MODULE_DESCRIPTION("PWM Subsystem driver");
// MODULE_AUTHOR("Texas Instruments");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
