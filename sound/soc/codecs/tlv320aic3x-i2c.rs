/* SPDX-License-Identifier: GPL-2.0-only
 *
 * ALSA SoC TLV320AIC3x codec driver I2C interface
 *
 * Author:      Arun KS, <arunks@mistralsolutions.com>
 * Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
 *
 * Based on sound/soc/codecs/wm8731.c by Richard Purdie
 *
 */

/* C dependencies:
 * linux/i2c.h, linux/module.h, linux/of.h, linux/regmap.h, sound/soc.h,
 * and "tlv320aic3x.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

unsafe impl Sync for of_device_id {}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static aic3x_regmap: regmap_config;
    static AIC3X_MODEL_3X: c_ulong;
    static AIC3X_MODEL_33: c_ulong;
    static AIC3X_MODEL_3007: c_ulong;
    static AIC3X_MODEL_3104: c_ulong;
    static AIC3X_MODEL_3106: c_ulong;

    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn aic3x_probe(dev: *mut device, regmap: *mut regmap, driver_data: usize) -> c_int;
    fn aic3x_remove(dev: *mut device);
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
}

const fn i2c_name(name: &[u8]) -> [c_char; 20] {
    let mut out = [0 as c_char; 20];
    let mut i = 0;

    while i < name.len() && i < 19 {
        out[i] = name[i] as c_char;
        i += 1;
    }

    out
}

static aic3x_i2c_id: [i2c_device_id; 6] = unsafe {
    [
        i2c_device_id {
            name: i2c_name(b"tlv320aic3x"),
            driver_data: AIC3X_MODEL_3X,
        },
        i2c_device_id {
            name: i2c_name(b"tlv320aic33"),
            driver_data: AIC3X_MODEL_33,
        },
        i2c_device_id {
            name: i2c_name(b"tlv320aic3007"),
            driver_data: AIC3X_MODEL_3007,
        },
        i2c_device_id {
            name: i2c_name(b"tlv320aic3104"),
            driver_data: AIC3X_MODEL_3104,
        },
        i2c_device_id {
            name: i2c_name(b"tlv320aic3106"),
            driver_data: AIC3X_MODEL_3106,
        },
        i2c_device_id {
            name: [0 as c_char; 20],
            driver_data: 0,
        },
    ]
};
/* MODULE_DEVICE_TABLE(i2c, aic3x_i2c_id); */

unsafe extern "C" fn aic3x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut regmap: *mut regmap;
    let mut config: regmap_config;

    config = aic3x_regmap;
    config.reg_bits = 8;
    config.val_bits = 8;

    regmap = devm_regmap_init_i2c(i2c, &config);
    return aic3x_probe(
        &mut (*i2c).dev,
        regmap,
        i2c_get_match_data(i2c) as usize,
    );
}

unsafe extern "C" fn aic3x_i2c_remove(i2c: *mut i2c_client) {
    aic3x_remove(&mut (*i2c).dev);
}

static aic3x_of_id: [of_device_id; 6] = [
    of_device_id {
        compatible: b"ti,tlv320aic3x\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ti,tlv320aic33\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ti,tlv320aic3007\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ti,tlv320aic3104\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ti,tlv320aic3106\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, aic3x_of_id); */

static mut aic3x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tlv320aic3x\0".as_ptr() as *const c_char,
        of_match_table: aic3x_of_id.as_ptr(),
    },
    probe: Some(aic3x_i2c_probe),
    remove: Some(aic3x_i2c_remove),
    id_table: aic3x_i2c_id.as_ptr(),
};

/* module_i2c_driver(aic3x_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC TLV320AIC3x codec driver I2C"); */
/* MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
