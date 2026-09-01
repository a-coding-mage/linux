// SPDX-License-Identifier: GPL-2.0-only
//
// CS35L56 ALSA SoC audio driver SPI binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Dependencies from:
// linux/acpi.h
// linux/module.h
// linux/moduleparam.h
// linux/regmap.h
// linux/spi/spi.h
// linux/types.h
// "cs35l56.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

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
pub struct spi_device {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct cs35l56_base {
    pub type_: c_uint,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub can_hibernate: bool,
}

#[repr(C)]
pub struct cs35l56_private {
    pub base: cs35l56_base,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

unsafe extern "C" {
    static cs35l56_regmap_spi: regmap_config;
    static cs35l56_pm_ops_i2c_spi: dev_pm_ops;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn cs35l56_init_config_for_spi(
        cs35l56: *mut cs35l56_base,
        spi: *mut spi_device,
    ) -> c_int;
    fn cs35l56_common_probe(cs35l56: *mut cs35l56_private, irq: c_int) -> c_int;
    fn spi_get_drvdata(spi: *mut spi_device) -> *mut c_void;
    fn cs35l56_remove(cs35l56: *mut cs35l56_private);
}

#[inline]
unsafe fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops {
    ptr
}

#[inline]
unsafe fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id {
    ptr
}

unsafe extern "C" fn cs35l56_spi_probe(spi: *mut spi_device) -> c_int {
    let regmap_config: *const regmap_config = unsafe { &cs35l56_regmap_spi };
    let cs35l56: *mut cs35l56_private;
    let ret: c_int;

    cs35l56 = unsafe {
        devm_kzalloc(
            &mut (*spi).dev,
            core::mem::size_of::<cs35l56_private>(),
            GFP_KERNEL,
        ) as *mut cs35l56_private
    };
    if cs35l56.is_null() {
        return -ENOMEM;
    }

    unsafe {
        spi_set_drvdata(spi, cs35l56 as *mut c_void);
    }

    unsafe {
        (*cs35l56).base.type_ = 0x56;
    }

    unsafe {
        (*cs35l56).base.regmap = devm_regmap_init_spi(spi, regmap_config);
    }
    if unsafe { IS_ERR((*cs35l56).base.regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*cs35l56).base.regmap as *const c_void) };
        return unsafe {
            dev_err_probe(
                &mut (*spi).dev,
                ret,
                c"Failed to allocate register map\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*cs35l56).base.dev = &mut (*spi).dev;
        (*cs35l56).base.can_hibernate = true;
    }
    ret = unsafe { cs35l56_init_config_for_spi(&mut (*cs35l56).base, spi) };
    if ret != 0 {
        return ret;
    }

    unsafe { cs35l56_common_probe(cs35l56, (*spi).irq) }
}

unsafe extern "C" fn cs35l56_spi_remove(spi: *mut spi_device) {
    let cs35l56: *mut cs35l56_private =
        unsafe { spi_get_drvdata(spi) as *mut cs35l56_private };

    unsafe {
        cs35l56_remove(cs35l56);
    }
}

static cs35l56_id_spi: [spi_device_id; 2] = [
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'6' as c_char,
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
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, cs35l56_id_spi);

// #ifdef CONFIG_ACPI
static cs35l56_asoc_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'5' as c_char,
            b'C' as c_char,
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
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs35l56_asoc_acpi_match);
// #endif

static mut cs35l56_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"cs35l56".as_ptr(),
        pm: unsafe { pm_ptr(&cs35l56_pm_ops_i2c_spi) },
        acpi_match_table: unsafe { ACPI_PTR(cs35l56_asoc_acpi_match.as_ptr()) },
    },
    id_table: cs35l56_id_spi.as_ptr(),
    probe: Some(cs35l56_spi_probe),
    remove: Some(cs35l56_spi_remove),
};

// module_spi_driver(cs35l56_spi_driver);

// MODULE_DESCRIPTION("ASoC CS35L56 SPI driver");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_CORE");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
