// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8328-i2c.c  --  ES8328 ALSA SoC I2C Audio driver
 *
 * Copyright 2014 Sutajio Ko-Usagi PTE LTD
 *
 * Author: Sean Cross <xobs@kosagi.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "es8328.h"

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
pub struct i2c_device_id {
    pub name: [::core::ffi::c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub name: [::core::ffi::c_char; 32],
    pub type_: [::core::ffi::c_char; 32],
    pub compatible: [::core::ffi::c_char; 128],
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> ::core::ffi::c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static es8328_regmap_config: regmap_config;

    fn devm_regmap_init_i2c(
        i2c: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;

    fn es8328_probe(dev: *mut device, regmap: *mut regmap) -> ::core::ffi::c_int;
}

static es8328_id: [i2c_device_id; 3] = [
    i2c_device_id {
        name: [
            b'e' as ::core::ffi::c_char,
            b's' as ::core::ffi::c_char,
            b'8' as ::core::ffi::c_char,
            b'3' as ::core::ffi::c_char,
            b'2' as ::core::ffi::c_char,
            b'8' as ::core::ffi::c_char,
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
        driver_data: 0,
    },
    i2c_device_id {
        name: [
            b'e' as ::core::ffi::c_char,
            b's' as ::core::ffi::c_char,
            b'8' as ::core::ffi::c_char,
            b'3' as ::core::ffi::c_char,
            b'8' as ::core::ffi::c_char,
            b'8' as ::core::ffi::c_char,
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
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(i2c, es8328_id);

static es8328_of_match: [of_device_id; 3] = [
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: {
            let mut compatible = [0; 128];
            compatible[0] = b'e' as ::core::ffi::c_char;
            compatible[1] = b'v' as ::core::ffi::c_char;
            compatible[2] = b'e' as ::core::ffi::c_char;
            compatible[3] = b'r' as ::core::ffi::c_char;
            compatible[4] = b'e' as ::core::ffi::c_char;
            compatible[5] = b's' as ::core::ffi::c_char;
            compatible[6] = b't' as ::core::ffi::c_char;
            compatible[7] = b',' as ::core::ffi::c_char;
            compatible[8] = b'e' as ::core::ffi::c_char;
            compatible[9] = b's' as ::core::ffi::c_char;
            compatible[10] = b'8' as ::core::ffi::c_char;
            compatible[11] = b'3' as ::core::ffi::c_char;
            compatible[12] = b'2' as ::core::ffi::c_char;
            compatible[13] = b'8' as ::core::ffi::c_char;
            compatible
        },
        data: ::core::ptr::null(),
    },
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: {
            let mut compatible = [0; 128];
            compatible[0] = b'e' as ::core::ffi::c_char;
            compatible[1] = b'v' as ::core::ffi::c_char;
            compatible[2] = b'e' as ::core::ffi::c_char;
            compatible[3] = b'r' as ::core::ffi::c_char;
            compatible[4] = b'e' as ::core::ffi::c_char;
            compatible[5] = b's' as ::core::ffi::c_char;
            compatible[6] = b't' as ::core::ffi::c_char;
            compatible[7] = b',' as ::core::ffi::c_char;
            compatible[8] = b'e' as ::core::ffi::c_char;
            compatible[9] = b's' as ::core::ffi::c_char;
            compatible[10] = b'8' as ::core::ffi::c_char;
            compatible[11] = b'3' as ::core::ffi::c_char;
            compatible[12] = b'8' as ::core::ffi::c_char;
            compatible[13] = b'8' as ::core::ffi::c_char;
            compatible
        },
        data: ::core::ptr::null(),
    },
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: [0; 128],
        data: ::core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, es8328_of_match);

unsafe extern "C" fn es8328_i2c_probe(i2c: *mut i2c_client) -> ::core::ffi::c_int {
    unsafe {
        es8328_probe(
            &mut (*i2c).dev,
            devm_regmap_init_i2c(i2c, &es8328_regmap_config),
        )
    }
}

static mut es8328_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"es8328".as_ptr(),
        of_match_table: es8328_of_match.as_ptr(),
    },
    probe: Some(es8328_i2c_probe),
    id_table: es8328_id.as_ptr(),
};

// module_i2c_driver(es8328_i2c_driver);

// MODULE_DESCRIPTION("ASoC ES8328 audio CODEC I2C driver");
// MODULE_AUTHOR("Sean Cross <xobs@kosagi.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
