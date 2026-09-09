// SPDX-License-Identifier: GPL-2.0
/*
 * Common CPM GPIO wrapper for the CPM GPIO ports
 *
 * Author: Christophe Leroy <christophe.leroy@c-s.fr>
 *
 * Copyright 2017 CS Systemes d'Information.
 *
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

pub type GpioAdd = unsafe extern "C" fn(dev: *mut device) -> c_int;

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(ofdev: *mut platform_device) -> c_int>,
    pub driver: device_driver,
}

extern "C" {
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;

    // External GPIO registration functions supplied by the CPM GPIO support.
    fn cpm1_gpiochip_add16(dev: *mut device) -> c_int;
    fn cpm1_gpiochip_add32(dev: *mut device) -> c_int;
    fn cpm2_gpiochip_add32(dev: *mut device) -> c_int;
}

const ENODEV: c_int = 19;

unsafe extern "C" fn cpm_gpio_probe(ofdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*ofdev).dev;
    let gp_add: Option<GpioAdd> = {
        let data = of_device_get_match_data(dev);
        if data.is_null() {
            None
        } else {
            Some(core::mem::transmute(data))
        }
    };

    let gp_add = match gp_add {
        Some(gp_add) => gp_add,
        None => return -ENODEV,
    };

    gp_add(dev)
}

// The following entries are enabled when CONFIG_8xx_GPIO is configured.
#[cfg(feature = "CONFIG_8xx_GPIO")]
static CPM_GPIO_MATCH_8XX: [of_device_id; 6] = [
    of_device_id {
        compatible: b"fsl,cpm1-pario-bank-a\0".as_ptr() as *const c_char,
        data: cpm1_gpiochip_add16 as *const c_void,
    },
    of_device_id {
        compatible: b"fsl,cpm1-pario-bank-b\0".as_ptr() as *const c_char,
        data: cpm1_gpiochip_add32 as *const c_void,
    },
    of_device_id {
        compatible: b"fsl,cpm1-pario-bank-c\0".as_ptr() as *const c_char,
        data: cpm1_gpiochip_add16 as *const c_void,
    },
    of_device_id {
        compatible: b"fsl,cpm1-pario-bank-d\0".as_ptr() as *const c_char,
        data: cpm1_gpiochip_add16 as *const c_void,
    },
    // Port E uses CPM2 layout
    of_device_id {
        compatible: b"fsl,cpm1-pario-bank-e\0".as_ptr() as *const c_char,
        data: cpm2_gpiochip_add32 as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

#[cfg(not(feature = "CONFIG_8xx_GPIO"))]
static CPM_GPIO_MATCH_8XX: [of_device_id; 1] = [];

static mut cpm_gpio_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fsl,cpm2-pario-bank\0".as_ptr() as *const c_char,
        data: cpm2_gpiochip_add32 as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut cpm_gpio_driver: platform_driver = platform_driver {
    probe: Some(cpm_gpio_probe),
    driver: device_driver {
        name: b"cpm-gpio\0".as_ptr() as *const c_char,
        of_match_table: cpm_gpio_match.as_ptr(),
    },
};

unsafe extern "C" fn cpm_gpio_init() -> c_int {
    platform_driver_register(&mut cpm_gpio_driver)
}

// arch_initcall(cpm_gpio_init);
// MODULE_DEVICE_TABLE(of, cpm_gpio_match);
// MODULE_AUTHOR("Christophe Leroy <christophe.leroy@c-s.fr>");
// MODULE_DESCRIPTION("Driver for CPM GPIO");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:cpm-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
