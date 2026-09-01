// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8731-i2c.c  --  WM8731 ALSA SoC Audio driver I2C code
 *
 * Copyright 2005 Openedhand Ltd.
 * Copyright 2006-12 Wolfson Microelectronics, plc
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.c by Liam Girdwood
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include "wm8731.h"

use core::ffi::{c_char, c_int, c_void};

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static wm8731_regmap: regmap_config;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn wm8731_init(dev: *mut device, wm8731: *mut wm8731_priv) -> c_int;
}

static wm8731_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"wlf,wm8731".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, wm8731_of_match);

unsafe extern "C" fn wm8731_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8731: *mut wm8731_priv;
    let ret: c_int;

    wm8731 = unsafe {
        devm_kzalloc(
            &mut (*i2c).dev,
            core::mem::size_of::<wm8731_priv>(),
            GFP_KERNEL,
        ) as *mut wm8731_priv
    };
    if wm8731.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(i2c, wm8731 as *mut c_void);
    }

    unsafe {
        (*wm8731).regmap = devm_regmap_init_i2c(i2c, &wm8731_regmap);
    }
    if unsafe { IS_ERR((*wm8731).regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*wm8731).regmap as *const c_void) };
        unsafe {
            dev_err(
                &mut (*i2c).dev,
                c"Failed to allocate register map: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    unsafe { wm8731_init(&mut (*i2c).dev, wm8731) }
}

static wm8731_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'w' as c_char,
            b'm' as c_char,
            b'8' as c_char,
            b'7' as c_char,
            b'3' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];

// MODULE_DEVICE_TABLE(i2c, wm8731_i2c_id);

static mut wm8731_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"wm8731".as_ptr(),
        of_match_table: wm8731_of_match.as_ptr(),
    },
    probe: Some(wm8731_i2c_probe),
    id_table: wm8731_i2c_id.as_ptr(),
};

// module_i2c_driver(wm8731_i2c_driver);

// MODULE_DESCRIPTION("ASoC WM8731 driver - I2C");
// MODULE_AUTHOR("Richard Purdie");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
