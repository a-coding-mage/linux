// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l42-i2c.c -- CS42L42 ALSA SoC audio driver for I2C
 *
 * Copyright 2016, 2022 Cirrus Logic, Inc.
 */

// C dependencies:
// linux/i2c.h, linux/module.h, linux/regmap.h, linux/slab.h, linux/types.h
// "cs42l42.h"

use core::ffi::{c_char, c_int, c_void};

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const CS42L42_CHIP_ID: c_int = 0;

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
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs42l42_private {
    pub devid: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: c_int,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: *const c_char,
    pub driver_data: usize,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static cs42l42_regmap: regmap_config;
    static cs42l42_soc_component: snd_soc_component_driver;
    static cs42l42_dai: snd_soc_dai_driver;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c_client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn cs42l42_common_probe(
        cs42l42: *mut cs42l42_private,
        component_drv: *const snd_soc_component_driver,
        dai: *const snd_soc_dai_driver,
    ) -> c_int;
    fn cs42l42_init(cs42l42: *mut cs42l42_private) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn cs42l42_common_remove(cs42l42: *mut cs42l42_private);
    fn cs42l42_resume(dev: *mut device) -> c_int;
    fn cs42l42_resume_restore(dev: *mut device);
    fn cs42l42_suspend(dev: *mut device) -> c_int;
}

unsafe extern "C" fn cs42l42_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let dev: *mut device = unsafe { &mut (*i2c_client).dev };
    let cs42l42: *mut cs42l42_private;
    let regmap: *mut regmap;
    let ret: c_int;

    cs42l42 = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<cs42l42_private>(),
            GFP_KERNEL,
        ) as *mut cs42l42_private
    };
    if cs42l42.is_null() {
        return -ENOMEM;
    }

    regmap = unsafe { devm_regmap_init_i2c(i2c_client, &raw const cs42l42_regmap) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        return unsafe {
            dev_err_probe(
                &mut (*i2c_client).dev,
                PTR_ERR(regmap as *const c_void),
                c"regmap_init() failed\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*cs42l42).devid = CS42L42_CHIP_ID;
        (*cs42l42).dev = dev;
        (*cs42l42).regmap = regmap;
        (*cs42l42).irq = (*i2c_client).irq;
    }

    ret = unsafe {
        cs42l42_common_probe(
            cs42l42,
            &raw const cs42l42_soc_component,
            &raw const cs42l42_dai,
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe { cs42l42_init(cs42l42) }
}

unsafe extern "C" fn cs42l42_i2c_remove(i2c_client: *mut i2c_client) {
    let cs42l42: *mut cs42l42_private =
        unsafe { dev_get_drvdata(&mut (*i2c_client).dev) as *mut cs42l42_private };

    unsafe {
        cs42l42_common_remove(cs42l42);
    }
}

unsafe extern "C" fn cs42l42_i2c_resume(dev: *mut device) -> c_int {
    let ret: c_int;

    ret = unsafe { cs42l42_resume(dev) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        cs42l42_resume_restore(dev);
    }

    0
}

static cs42l42_i2c_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(cs42l42_suspend, cs42l42_i2c_resume)
    suspend: Some(cs42l42_suspend),
    resume: Some(cs42l42_i2c_resume),
};

// __maybe_unused
static cs42l42_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cirrus,cs42l42".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs42l42_of_match);

// __maybe_unused
static cs42l42_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: c"10134242".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs42l42_acpi_match);

static cs42l42_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"cs42l42".as_ptr(),
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(i2c, cs42l42_id);

static mut cs42l42_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs42l42".as_ptr(),
        pm: &raw const cs42l42_i2c_pm_ops,
        of_match_table: cs42l42_of_match.as_ptr(),
        acpi_match_table: cs42l42_acpi_match.as_ptr(),
    },
    id_table: cs42l42_id.as_ptr(),
    probe: Some(cs42l42_i2c_probe),
    remove: Some(cs42l42_i2c_remove),
};

// module_i2c_driver(cs42l42_i2c_driver);

// MODULE_DESCRIPTION("ASoC CS42L42 I2C driver");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_CS42L42_CORE");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
