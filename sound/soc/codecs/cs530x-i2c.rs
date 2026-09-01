// SPDX-License-Identifier: GPL-2.0
//
// CS530x CODEC driver
//
// Copyright (C) 2024-2025 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.

// C includes translated as external dependencies:
// linux/device.h, linux/module.h, linux/i2c.h, linux/regmap.h, "cs530x.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
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
pub struct cs530x_priv {
    pub regmap: *mut regmap,
    pub devtype: usize,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
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
    static cs530x_regmap_i2c: regmap_config;

    static CS4282: c_uint;
    static CS4302: c_uint;
    static CS4304: c_uint;
    static CS4308: c_uint;
    static CS5302: c_uint;
    static CS5304: c_uint;
    static CS5308: c_uint;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn cs530x_probe(cs530x: *mut cs530x_priv) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

static cs530x_of_match: [of_device_id; 8] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs4282".as_ptr(),
        data: unsafe { CS4282 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs4302".as_ptr(),
        data: unsafe { CS4302 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs4304".as_ptr(),
        data: unsafe { CS4304 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs4308".as_ptr(),
        data: unsafe { CS4308 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs5302".as_ptr(),
        data: unsafe { CS5302 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs5304".as_ptr(),
        data: unsafe { CS5304 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs5308".as_ptr(),
        data: unsafe { CS5308 as usize as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs530x_of_match);

static cs530x_i2c_id: [i2c_device_id; 8] = [
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            b'2' as c_char,
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
        driver_data: unsafe { CS4282 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'2' as c_char,
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
        driver_data: unsafe { CS4302 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'4' as c_char,
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
        driver_data: unsafe { CS4304 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'8' as c_char,
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
        driver_data: unsafe { CS4308 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'5' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'2' as c_char,
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
        driver_data: unsafe { CS5302 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'5' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'4' as c_char,
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
        driver_data: unsafe { CS5304 as usize },
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'5' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'8' as c_char,
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
        driver_data: unsafe { CS5308 as usize },
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, cs530x_i2c_id);

unsafe extern "C" fn cs530x_i2c_probe(client: *mut i2c_client) -> c_int {
    let cs530x: *mut cs530x_priv;

    cs530x = devm_kzalloc(
        unsafe { &mut (*client).dev },
        core::mem::size_of::<cs530x_priv>(),
        GFP_KERNEL,
    ) as *mut cs530x_priv;
    if cs530x.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(client, cs530x as *mut c_void);
    }

    unsafe {
        (*cs530x).regmap = devm_regmap_init_i2c(client, &cs530x_regmap_i2c);
    }
    if unsafe { IS_ERR((*cs530x).regmap as *const c_void) } {
        return unsafe {
            dev_err_probe(
                &mut (*client).dev,
                PTR_ERR((*cs530x).regmap as *const c_void),
                c"Failed to allocate register map\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*cs530x).devtype = i2c_get_match_data(client) as usize;
        (*cs530x).dev = &mut (*client).dev;
    }

    unsafe { cs530x_probe(cs530x) }
}

static mut cs530x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs530x".as_ptr(),
        of_match_table: cs530x_of_match.as_ptr(),
    },
    probe: Some(cs530x_i2c_probe),
    id_table: cs530x_i2c_id.as_ptr(),
};
// module_i2c_driver(cs530x_i2c_driver);

// MODULE_DESCRIPTION("I2C CS530X driver");
// MODULE_IMPORT_NS("SND_SOC_CS530X");
// MODULE_AUTHOR("Paul Handrigan, Cirrus Logic Inc, <paulha@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
