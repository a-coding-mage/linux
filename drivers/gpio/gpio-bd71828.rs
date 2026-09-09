// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2018 ROHM Semiconductors

// Dependencies supplied by the Linux kernel and the BD71828 MFD driver.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct gpio_chip {
    pub parent: *mut device,
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_ulong) -> c_int>,
    pub can_sleep: bool,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int)>,
    pub base: c_int,
    pub ngpio: c_uint,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
}

extern "C" {
    static mut THIS_MODULE: c_void;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn pinconf_to_config_param(config: c_ulong) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut c_void) -> c_int;
}

const GPIO_OUT_REG: unsafe fn(c_uint) -> c_uint = |off| BD71828_REG_GPIO_CTRL1 + off;
const HALL_GPIO_OFFSET: c_uint = 3;

const BD71828_REG_GPIO_CTRL1: c_uint = 0;
const BD71828_REG_IO_STAT: c_uint = 0;
const BD71828_GPIO_OUT_HI: c_uint = 0;
const BD71828_GPIO_OUT_LO: c_uint = 0;
const BD71828_GPIO_OUT_MASK: c_uint = 0;
const BD71828_GPIO_DRIVE_MASK: c_uint = 0;
const BD71828_GPIO_OPEN_DRAIN: c_uint = 0;
const BD71828_GPIO_PUSH_PULL: c_uint = 0;
const PIN_CONFIG_DRIVE_OPEN_DRAIN: c_uint = 0;
const PIN_CONFIG_DRIVE_PUSH_PULL: c_uint = 0;
const GPIO_LINE_DIRECTION_IN: c_int = 0;
const GPIO_LINE_DIRECTION_OUT: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOTSUPP: c_int = 524;

#[repr(C)]
struct bd71828_gpio {
    regmap: *mut regmap,
    dev: *mut device,
    gpio: gpio_chip,
}

unsafe extern "C" fn bd71828_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) {
    let bdgpio = gpiochip_get_data(chip) as *mut bd71828_gpio;
    let val = if value != 0 { BD71828_GPIO_OUT_HI } else { BD71828_GPIO_OUT_LO };

    /* The HALL input pin can only be used as input. */
    if offset == HALL_GPIO_OFFSET {
        return;
    }

    let _ = regmap_update_bits((*bdgpio).regmap, GPIO_OUT_REG(offset),
                               BD71828_GPIO_OUT_MASK, val);
}

unsafe extern "C" fn bd71828_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let bdgpio = gpiochip_get_data(chip) as *mut bd71828_gpio;
    let mut val: c_uint = 0;
    let mut ret;
    if offset == HALL_GPIO_OFFSET {
        ret = regmap_read((*bdgpio).regmap, BD71828_REG_IO_STAT, &mut val);
    } else {
        ret = regmap_read((*bdgpio).regmap, GPIO_OUT_REG(offset), &mut val);
    }
    if ret == 0 {
        ret = (val & BD71828_GPIO_OUT_MASK) as c_int;
    }
    ret
}

unsafe extern "C" fn bd71828_gpio_set_config(chip: *mut gpio_chip, offset: c_uint,
                                               config: c_ulong) -> c_int {
    let bdgpio = gpiochip_get_data(chip) as *mut bd71828_gpio;
    if offset == HALL_GPIO_OFFSET { return -ENOTSUPP; }
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits((*bdgpio).regmap, GPIO_OUT_REG(offset),
            BD71828_GPIO_DRIVE_MASK, BD71828_GPIO_OPEN_DRAIN),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits((*bdgpio).regmap, GPIO_OUT_REG(offset),
            BD71828_GPIO_DRIVE_MASK, BD71828_GPIO_PUSH_PULL),
        _ => -ENOTSUPP,
    }
}

unsafe extern "C" fn bd71828_get_direction(_chip: *mut gpio_chip, offset: c_uint) -> c_int {
    if offset == HALL_GPIO_OFFSET { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn bd71828_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let bdgpio = devm_kzalloc(dev, core::mem::size_of::<bd71828_gpio>(), GFP_KERNEL)
        as *mut bd71828_gpio;
    if bdgpio.is_null() { return -ENOMEM; }
    (*bdgpio).dev = dev;
    (*bdgpio).gpio.parent = core::ptr::null_mut();
    (*bdgpio).gpio.label = b"bd71828-gpio\0".as_ptr() as *const c_char;
    (*bdgpio).gpio.owner = &mut THIS_MODULE;
    (*bdgpio).gpio.get_direction = Some(bd71828_get_direction);
    (*bdgpio).gpio.set_config = Some(bd71828_gpio_set_config);
    (*bdgpio).gpio.can_sleep = true;
    (*bdgpio).gpio.get = Some(bd71828_gpio_get);
    (*bdgpio).gpio.set = Some(bd71828_gpio_set);
    (*bdgpio).gpio.base = -1;
    (*bdgpio).gpio.ngpio = 4;
    (*bdgpio).regmap = dev_get_regmap(core::ptr::null_mut(), core::ptr::null());
    if (*bdgpio).regmap.is_null() { return -ENODEV; }
    devm_gpiochip_add_data(dev, &mut (*bdgpio).gpio, bdgpio as *mut c_void)
}

static mut bd71828_gpio: platform_driver = platform_driver {
    driver: driver { name: b"bd71828-gpio\0".as_ptr() as *const c_char },
    probe: Some(bd71828_probe),
};

// module_platform_driver(bd71828_gpio);
// MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
// MODULE_DESCRIPTION("BD71828 voltage regulator driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:bd71828-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
