// SPDX-License-Identifier: GPL-2.0-only
/*
 * Delta TN48M CPLD GPIO driver
 *
 * Copyright (C) 2021 Sartura Ltd.
 *
 * Author: Robert Marko <robert.marko@sartura.hr>
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel types and functions supplied by other translation units.
#[repr(C)]
pub struct device {
    pub parent: *mut device,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct gpio_regmap_config {
    pub regmap: *mut regmap,
    pub parent: *mut device,
    pub ngpio: c_int,
    pub ngpio_per_reg: c_int,
    pub reg_set_base: u32,
    pub reg_dat_base: u32,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

extern "C" {
    fn device_get_match_data(dev: *const device) -> *const c_void;
    fn device_property_read_u32(dev: *const device, name: *const c_char, value: *mut u32) -> c_int;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn devm_gpio_regmap_register(dev: *mut device, config: *mut gpio_regmap_config) -> *mut c_void;
    fn ptr_err_or_zero(ptr: *mut c_void) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tn48m_gpio_type {
    TN48M_GP0 = 1,
    TN48M_GPI,
}

#[repr(C)]
pub struct tn48m_gpio_config {
    pub ngpio: c_int,
    pub ngpio_per_reg: c_int,
    pub type_: tn48m_gpio_type,
}

static tn48m_gpo_config: tn48m_gpio_config = tn48m_gpio_config {
    ngpio: 4,
    ngpio_per_reg: 4,
    type_: tn48m_gpio_type::TN48M_GP0,
};

static tn48m_gpi_config: tn48m_gpio_config = tn48m_gpio_config {
    ngpio: 4,
    ngpio_per_reg: 4,
    type_: tn48m_gpio_type::TN48M_GPI,
};

unsafe extern "C" fn tn48m_gpio_probe(pdev: *mut platform_device) -> c_int {
    let mut gpio_config: *const tn48m_gpio_config;
    let mut config: gpio_regmap_config = core::mem::zeroed();
    let regmap: *mut regmap;
    let mut base: u32 = 0;
    let ret: c_int;

    if (*pdev).dev.parent.is_null() {
        return -19; // -ENODEV
    }

    gpio_config = device_get_match_data(&(*pdev).dev) as *const tn48m_gpio_config;
    if gpio_config.is_null() {
        return -19; // -ENODEV
    }

    ret = device_property_read_u32(&(*pdev).dev, b"reg\0".as_ptr() as *const c_char, &mut base);
    if ret != 0 {
        return ret;
    }

    regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if regmap.is_null() {
        return -19; // -ENODEV
    }

    config.regmap = regmap;
    config.parent = &mut (*pdev).dev;
    config.ngpio = (*gpio_config).ngpio;
    config.ngpio_per_reg = (*gpio_config).ngpio_per_reg;
    match (*gpio_config).type_ {
        tn48m_gpio_type::TN48M_GP0 => {
            config.reg_set_base = base;
        }
        tn48m_gpio_type::TN48M_GPI => {
            config.reg_dat_base = base;
        }
    }

    ptr_err_or_zero(devm_gpio_regmap_register(&mut (*pdev).dev, &mut config))
}

static tn48m_gpio_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"delta,tn48m-gpo\0".as_ptr() as *const c_char,
        data: &tn48m_gpo_config as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"delta,tn48m-gpi\0".as_ptr() as *const c_char,
        data: &tn48m_gpi_config as *const _ as *const c_void,
    },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut tn48m_gpio_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"delta-tn48m-gpio\0".as_ptr() as *const c_char,
        of_match_table: tn48m_gpio_of_match.as_ptr(),
    },
    probe: Some(tn48m_gpio_probe),
};

// MODULE_DEVICE_TABLE(of, tn48m_gpio_of_match);
// module_platform_driver(tn48m_gpio_driver);
// MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
// MODULE_DESCRIPTION("Delta TN48M CPLD GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
