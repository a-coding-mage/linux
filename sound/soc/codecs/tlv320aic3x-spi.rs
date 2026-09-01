// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC TLV320AIC3x codec driver SPI interface
//
// Author:      Arun KS, <arunks@mistralsolutions.com>
// Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
//
// Based on sound/soc/codecs/wm8731.c by Richard Purdie
//

// C dependencies translated from:
// #include <linux/spi/spi.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "tlv320aic3x.h"

unsafe fn aic3x_spi_probe(spi: *mut spi_device) -> core::ffi::c_int {
    let mut regmap: *mut regmap;
    let mut config: regmap_config;
    let id: *const spi_device_id = spi_get_device_id(spi);

    config = aic3x_regmap;
    config.reg_bits = 7;
    config.pad_bits = 1;
    config.val_bits = 8;
    config.read_flag_mask = 0x01;

    dev_dbg(
        &mut (*spi).dev,
        c"probing tlv320aic3x spi device\n".as_ptr(),
    );

    regmap = devm_regmap_init_spi(spi, &mut config);
    aic3x_probe(&mut (*spi).dev, regmap, (*id).driver_data)
}

unsafe fn aic3x_spi_remove(spi: *mut spi_device) {
    aic3x_remove(&mut (*spi).dev);
}

static aic3x_spi_id: [spi_device_id; 6] = [
    spi_device_id {
        name: c"tlv320aic3x".as_ptr(),
        driver_data: AIC3X_MODEL_3X,
    },
    spi_device_id {
        name: c"tlv320aic33".as_ptr(),
        driver_data: AIC3X_MODEL_33,
    },
    spi_device_id {
        name: c"tlv320aic3007".as_ptr(),
        driver_data: AIC3X_MODEL_3007,
    },
    spi_device_id {
        name: c"tlv320aic3104".as_ptr(),
        driver_data: AIC3X_MODEL_3104,
    },
    spi_device_id {
        name: c"tlv320aic3106".as_ptr(),
        driver_data: AIC3X_MODEL_3106,
    },
    spi_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, aic3x_spi_id);

static aic3x_of_id: [of_device_id; 6] = [
    of_device_id {
        compatible: c"ti,tlv320aic3x".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tlv320aic33".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tlv320aic3007".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tlv320aic3104".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tlv320aic3106".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, aic3x_of_id);

static mut aic3x_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"tlv320aic3x".as_ptr(),
        of_match_table: aic3x_of_id.as_ptr(),
    },
    probe: Some(aic3x_spi_probe),
    remove: Some(aic3x_spi_remove),
    id_table: aic3x_spi_id.as_ptr(),
};

// module_spi_driver(aic3x_spi_driver);

// MODULE_DESCRIPTION("ASoC TLV320AIC3x codec driver SPI");
// MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
