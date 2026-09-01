// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC TLV320AIC23 codec driver SPI interface
 *
 * Author:      Arun KS, <arunks@mistralsolutions.com>
 * Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
 *
 * Based on sound/soc/codecs/wm8731.c by Richard Purdie
 */

// C dependencies: <linux/module.h>, <linux/regmap.h>, <linux/spi/spi.h>,
// <sound/soc.h>, and "tlv320aic23.h".

use core::ffi::{c_char, c_int};

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
pub struct spi_device {
    pub dev: device,
    pub mode: u32,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

pub const SPI_MODE_0: u32 = 0;

unsafe extern "C" {
    static tlv320aic23_regmap: regmap_config;

    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn tlv320aic23_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn __dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn __module_spi_driver(driver: *mut spi_driver);
}

unsafe extern "C" fn aic23_spi_probe(spi: *mut spi_device) -> c_int {
    let ret: c_int;
    let regmap: *mut regmap;

    unsafe {
        __dev_dbg(
            &mut (*spi).dev,
            c"probing tlv320aic23 spi device\n".as_ptr(),
        );

        (*spi).mode = SPI_MODE_0;
        ret = spi_setup(spi);
        if ret < 0 {
            return ret;
        }

        regmap = devm_regmap_init_spi(spi, &tlv320aic23_regmap);
        tlv320aic23_probe(&mut (*spi).dev, regmap)
    }
}

static mut aic23_spi: spi_driver = spi_driver {
    driver: device_driver {
        name: c"tlv320aic23".as_ptr(),
    },
    probe: Some(aic23_spi_probe),
};

// module_spi_driver(aic23_spi);
#[used]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __register_aic23_spi_driver() {
    unsafe {
        __module_spi_driver(&raw mut aic23_spi);
    }
}

// MODULE_DESCRIPTION("ASoC TLV320AIC23 codec driver SPI");
#[used]
#[unsafe(no_mangle)]
pub static __MODULE_DESCRIPTION: &[u8; 36] = b"ASoC TLV320AIC23 codec driver SPI\0";

// MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
#[used]
#[unsafe(no_mangle)]
pub static __MODULE_AUTHOR: &[u8; 39] = b"Arun KS <arunks@mistralsolutions.com>\0";

// MODULE_LICENSE("GPL");
#[used]
#[unsafe(no_mangle)]
pub static __MODULE_LICENSE: &[u8; 4] = b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
