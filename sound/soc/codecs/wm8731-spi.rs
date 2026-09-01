// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8731.c  --  WM8731 ALSA SoC Audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 * Copyright 2006-12 Wolfson Microelectronics, plc
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.c by Liam Girdwood
 */

// C dependencies: <linux/spi/spi.h>, <linux/module.h>, "wm8731.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm8731_priv {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" {
    static wm8731_regmap: regmap_config;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn wm8731_init(dev: *mut device, wm8731: *mut wm8731_priv) -> c_int;
}

#[used]
static wm8731_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"wlf,wm8731".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, wm8731_of_match);

unsafe extern "C" fn wm8731_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8731: *mut wm8731_priv;
    let ret: c_int;

    wm8731 = unsafe {
        devm_kzalloc(
            &raw mut (*spi).dev,
            core::mem::size_of::<wm8731_priv>(),
            GFP_KERNEL,
        ) as *mut wm8731_priv
    };
    if wm8731 == ptr::null_mut() {
        return -ENOMEM;
    }

    unsafe {
        spi_set_drvdata(spi, wm8731 as *mut c_void);
    }

    unsafe {
        (*wm8731).regmap = devm_regmap_init_spi(spi, &raw const wm8731_regmap);
    }
    if unsafe { IS_ERR((*wm8731).regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*wm8731).regmap as *const c_void) };
        unsafe {
            dev_err(
                &raw mut (*spi).dev,
                c"Failed to allocate register map: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    unsafe { wm8731_init(&raw mut (*spi).dev, wm8731) }
}

#[used]
static mut wm8731_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"wm8731".as_ptr(),
        of_match_table: wm8731_of_match.as_ptr(),
    },
    probe: Some(wm8731_spi_probe),
};

// module_spi_driver(wm8731_spi_driver);

// MODULE_DESCRIPTION("ASoC WM8731 driver - SPI");
// MODULE_AUTHOR("Richard Purdie");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
