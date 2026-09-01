// SPDX-License-Identifier: GPL-2.0
/*
 * Texas Instruments PCM186x Universal Audio ADC - I2C
 *
 * Copyright (C) 2015-2017 Texas Instruments Incorporated - https://www.ti.com
 *	Andreas Dannenberg <dannenberg@ti.com>
 *	Andrew F. Davis <afd@ti.com>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include "pcm186x.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

type uintptr_t = usize;
type pcm186x_type = c_uint;

type c_uint = u32;

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
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
    pub driver: device_driver,
}

unsafe extern "C" {
    static pcm186x_regmap: regmap_config;

    static PCM1862: pcm186x_type;
    static PCM1863: pcm186x_type;
    static PCM1864: pcm186x_type;
    static PCM1865: pcm186x_type;

    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pcm186x_probe(
        dev: *mut device,
        type_: pcm186x_type,
        irq: c_int,
        regmap: *mut regmap,
    ) -> c_int;
}

static pcm186x_of_match: [of_device_id; 5] = [
    of_device_id {
        compatible: c"ti,pcm1862".as_ptr(),
        data: unsafe { PCM1862 as uintptr_t as *const c_void },
    },
    of_device_id {
        compatible: c"ti,pcm1863".as_ptr(),
        data: unsafe { PCM1863 as uintptr_t as *const c_void },
    },
    of_device_id {
        compatible: c"ti,pcm1864".as_ptr(),
        data: unsafe { PCM1864 as uintptr_t as *const c_void },
    },
    of_device_id {
        compatible: c"ti,pcm1865".as_ptr(),
        data: unsafe { PCM1865 as uintptr_t as *const c_void },
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm186x_of_match);

static pcm186x_i2c_id: [i2c_device_id; 5] = [
    i2c_device_id {
        name: c"pcm1862".as_ptr(),
        driver_data: unsafe { PCM1862 as c_ulong },
    },
    i2c_device_id {
        name: c"pcm1863".as_ptr(),
        driver_data: unsafe { PCM1863 as c_ulong },
    },
    i2c_device_id {
        name: c"pcm1864".as_ptr(),
        driver_data: unsafe { PCM1864 as c_ulong },
    },
    i2c_device_id {
        name: c"pcm1865".as_ptr(),
        driver_data: unsafe { PCM1865 as c_ulong },
    },
    i2c_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, pcm186x_i2c_id);

unsafe extern "C" fn pcm186x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let type_: pcm186x_type = i2c_get_match_data(i2c) as uintptr_t as pcm186x_type;
    let irq: c_int = (*i2c).irq;
    let regmap: *mut regmap;

    regmap = devm_regmap_init_i2c(i2c, &pcm186x_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    return pcm186x_probe(&mut (*i2c).dev, type_, irq, regmap);
}

static mut pcm186x_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(pcm186x_i2c_probe),
    id_table: pcm186x_i2c_id.as_ptr(),
    driver: device_driver {
        name: c"pcm186x".as_ptr(),
        of_match_table: pcm186x_of_match.as_ptr(),
    },
};
// module_i2c_driver(pcm186x_i2c_driver);

// MODULE_AUTHOR("Andreas Dannenberg <dannenberg@ti.com>");
// MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
// MODULE_DESCRIPTION("PCM186x Universal Audio ADC I2C Interface Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
