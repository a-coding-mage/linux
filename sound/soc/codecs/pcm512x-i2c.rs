// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the PCM512x CODECs
 *
 * Author:	Mark Brown <broonie@kernel.org>
 *		Copyright 2014 Linaro Ltd
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include <linux/acpi.h>
// #include "pcm512x.h"

unsafe fn pcm512x_i2c_probe(i2c: *mut i2c_client) -> core::ffi::c_int {
    let mut regmap: *mut regmap;
    let mut config: regmap_config = unsafe { pcm512x_regmap };

    /* msb needs to be set to enable auto-increment of addresses */
    config.read_flag_mask = 0x80;
    config.write_flag_mask = 0x80;

    regmap = unsafe { devm_regmap_init_i2c(i2c, &mut config) };
    if unsafe { IS_ERR(regmap) } {
        return unsafe { PTR_ERR(regmap) };
    }

    unsafe { pcm512x_probe(&mut (*i2c).dev, regmap) }
}

unsafe fn pcm512x_i2c_remove(i2c: *mut i2c_client) {
    unsafe {
        pcm512x_remove(&mut (*i2c).dev);
    }
}

static pcm512x_i2c_id: [i2c_device_id; 8] = [
    i2c_device_id {
        name: c"pcm5121".as_ptr(),
    },
    i2c_device_id {
        name: c"pcm5122".as_ptr(),
    },
    i2c_device_id {
        name: c"pcm5141".as_ptr(),
    },
    i2c_device_id {
        name: c"pcm5142".as_ptr(),
    },
    i2c_device_id {
        name: c"pcm5242".as_ptr(),
    },
    i2c_device_id {
        name: c"tas5754".as_ptr(),
    },
    i2c_device_id {
        name: c"tas5756".as_ptr(),
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];
module_device_table!(i2c, pcm512x_i2c_id);

// Original C condition: #if defined(CONFIG_OF)
#[cfg(CONFIG_OF)]
static pcm512x_of_match: [of_device_id; 8] = [
    of_device_id {
        compatible: c"ti,pcm5121".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,pcm5122".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,pcm5141".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,pcm5142".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,pcm5242".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tas5754".as_ptr(),
    },
    of_device_id {
        compatible: c"ti,tas5756".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
module_device_table!(of, pcm512x_of_match);

// Original C condition: #ifdef CONFIG_ACPI
#[cfg(CONFIG_ACPI)]
static pcm512x_acpi_match: [acpi_device_id; 5] = [
    acpi_device_id {
        id: c"104C5121".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: c"104C5122".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: c"104C5141".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: c"104C5142".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: core::ptr::null(),
        driver_data: 0,
    },
];
#[cfg(CONFIG_ACPI)]
module_device_table!(acpi, pcm512x_acpi_match);

static mut pcm512x_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(pcm512x_i2c_probe),
    remove: Some(pcm512x_i2c_remove),
    id_table: pcm512x_i2c_id.as_ptr(),
    driver: device_driver {
        name: c"pcm512x".as_ptr(),
        of_match_table: of_match_ptr!(pcm512x_of_match),
        acpi_match_table: ACPI_PTR!(pcm512x_acpi_match),
        pm: pm_ptr!(&pcm512x_pm_ops),
    },
};

module_i2c_driver!(pcm512x_i2c_driver);

MODULE_DESCRIPTION!("ASoC PCM512x codec driver - I2C");
MODULE_AUTHOR!("Mark Brown <broonie@kernel.org>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
