// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC TLV320AIC23 codec driver I2C interface
 *
 * Author:      Arun KS, <arunks@mistralsolutions.com>
 * Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
 *
 * Based on sound/soc/codecs/wm8731.c by Richard Purdie
 */

// Dependencies from:
// linux/i2c.h, linux/module.h, linux/of.h, linux/regmap.h, sound/soc.h,
// and "tlv320aic23.h".

use core::ptr;

extern "C" {
    static tlv320aic23_regmap: regmap_config;

    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: u32) -> i32;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn tlv320aic23_probe(dev: *mut device, regmap: *mut regmap) -> i32;
}

const EINVAL: i32 = 22;
const I2C_FUNC_SMBUS_BYTE_DATA: u32 = 0x0002_0000;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
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
    pub adapter: *mut i2c_adapter,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [u8; 20],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct of_device_id {
    pub name: [u8; 32],
    pub type_: [u8; 32],
    pub compatible: [u8; 128],
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> i32>,
    pub id_table: *const i2c_device_id,
}

#[allow(non_camel_case_types)]
pub type kernel_ulong_t = core::ffi::c_ulong;

unsafe extern "C" fn tlv320aic23_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let regmap: *mut regmap;

    if i2c_check_functionality((*i2c).adapter, I2C_FUNC_SMBUS_BYTE_DATA) == 0 {
        return -EINVAL;
    }

    regmap = devm_regmap_init_i2c(i2c, &tlv320aic23_regmap);
    tlv320aic23_probe(&mut (*i2c).dev, regmap)
}

static tlv320aic23_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: {
            let mut name = [0u8; 20];
            name[0] = b't';
            name[1] = b'l';
            name[2] = b'v';
            name[3] = b'3';
            name[4] = b'2';
            name[5] = b'0';
            name[6] = b'a';
            name[7] = b'i';
            name[8] = b'c';
            name[9] = b'2';
            name[10] = b'3';
            name
        },
        driver_data: 0,
    },
    i2c_device_id {
        name: [0u8; 20],
        driver_data: 0,
    },
];

module_device_table!(i2c, tlv320aic23_id);

// CONFIG_OF: Open Firmware match table for compatible "ti,tlv320aic23".
static tlv320aic23_of_match: [of_device_id; 2] = [
    of_device_id {
        name: [0u8; 32],
        type_: [0u8; 32],
        compatible: {
            let mut compatible = [0u8; 128];
            compatible[0] = b't';
            compatible[1] = b'i';
            compatible[2] = b',';
            compatible[3] = b't';
            compatible[4] = b'l';
            compatible[5] = b'v';
            compatible[6] = b'3';
            compatible[7] = b'2';
            compatible[8] = b'0';
            compatible[9] = b'a';
            compatible[10] = b'i';
            compatible[11] = b'c';
            compatible[12] = b'2';
            compatible[13] = b'3';
            compatible
        },
        data: ptr::null(),
    },
    of_device_id {
        name: [0u8; 32],
        type_: [0u8; 32],
        compatible: [0u8; 128],
        data: ptr::null(),
    },
];

module_device_table!(of, tlv320aic23_of_match);

static mut tlv320aic23_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tlv320aic23-codec\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_ptr!(tlv320aic23_of_match),
    },
    probe: Some(tlv320aic23_i2c_probe),
    id_table: tlv320aic23_id.as_ptr(),
};

module_i2c_driver!(tlv320aic23_i2c_driver);

module_description!("ASoC TLV320AIC23 codec driver I2C");
module_author!("Arun KS <arunks@mistralsolutions.com>");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
