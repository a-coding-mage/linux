// SPDX-License-Identifier: GPL-2.0
/*
 * Dollar Cove TI PMIC operation region driver
 * Copyright (C) 2014 Intel Corporation. All rights reserved.
 *
 * Rewritten and cleaned up
 * Copyright (C) 2017 Takashi Iwai <tiwai@suse.de>
 */

// Registers stored in 16-bit BE (high:low, total 10-bit).
const PMIC_REG_MASK: u16 = (1u16 << 10) - 1;

const CHTDC_TI_VBAT: i32 = 0x54;
const CHTDC_TI_DIETEMP: i32 = 0x56;
const CHTDC_TI_BPTHERM: i32 = 0x58;
const CHTDC_TI_GPADC: i32 = 0x5a;

#[repr(C)]
struct pmic_table {
    address: u32,
    reg: i32,
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct intel_pmic_opregion_data {
    get_power: Option<unsafe extern "C" fn(*mut regmap, i32, i32, *mut u64) -> i32>,
    update_power: Option<unsafe extern "C" fn(*mut regmap, i32, i32, bool) -> i32>,
    get_raw_temp: Option<unsafe extern "C" fn(*mut regmap, i32) -> i32>,
    lpat_raw_to_temp: Option<unsafe extern "C" fn(i32, i32) -> i32>,
    power_table: *const pmic_table,
    power_table_count: usize,
    thermal_table: *const pmic_table,
    thermal_table_count: usize,
    pmic_i2c_address: u16,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct intel_soc_pmic {
    regmap: *mut regmap,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: i32, val: *mut i32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: i32, mask: u32, val: bool) -> i32;
    fn regmap_bulk_read(map: *mut regmap, reg: i32, val: *mut u8, count: usize) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut intel_soc_pmic;
    fn intel_pmic_install_opregion_handler(
        dev: *mut device,
        handle: *mut core::ffi::c_void,
        regmap: *mut regmap,
        data: *const intel_pmic_opregion_data,
    ) -> i32;
    fn acpi_dev_clear_dependencies(companion: *mut core::ffi::c_void);
    fn acpi_lpat_raw_to_temp(raw: i32, lpat: i32) -> i32;
}

static chtdc_ti_power_table: [pmic_table; 13] = [
    pmic_table { address: 0x00, reg: 0x41 }, // LDO1
    pmic_table { address: 0x04, reg: 0x42 }, // LDO2
    pmic_table { address: 0x08, reg: 0x43 }, // LDO3
    pmic_table { address: 0x0c, reg: 0x45 }, // LDO5
    pmic_table { address: 0x10, reg: 0x46 }, // LDO6
    pmic_table { address: 0x14, reg: 0x47 }, // LDO7
    pmic_table { address: 0x18, reg: 0x48 }, // LDO8
    pmic_table { address: 0x1c, reg: 0x49 }, // LDO9
    pmic_table { address: 0x20, reg: 0x4a }, // LD10
    pmic_table { address: 0x24, reg: 0x4b }, // LD11
    pmic_table { address: 0x28, reg: 0x4c }, // LD12
    pmic_table { address: 0x2c, reg: 0x4d }, // LD13
    pmic_table { address: 0x30, reg: 0x4e }, // LD14
];

static chtdc_ti_thermal_table: [pmic_table; 6] = [
    pmic_table { address: 0x00, reg: CHTDC_TI_GPADC },
    pmic_table { address: 0x0c, reg: CHTDC_TI_GPADC },
    // TMP2 -> SYSTEMP
    pmic_table { address: 0x18, reg: CHTDC_TI_GPADC },
    // TMP3 -> BPTHERM
    pmic_table { address: 0x24, reg: CHTDC_TI_BPTHERM },
    pmic_table { address: 0x30, reg: CHTDC_TI_GPADC },
    // TMP5 -> DIETEMP
    pmic_table { address: 0x3c, reg: CHTDC_TI_DIETEMP },
];

unsafe extern "C" fn chtdc_ti_pmic_get_power(
    regmap: *mut regmap,
    reg: i32,
    _bit: i32,
    value: *mut u64,
) -> i32 {
    let mut data = 0i32;
    if regmap_read(regmap, reg, &mut data) != 0 {
        return -5; // -EIO
    }
    *value = (data & 1) as u64;
    0
}

unsafe extern "C" fn chtdc_ti_pmic_update_power(
    regmap: *mut regmap,
    reg: i32,
    _bit: i32,
    on: bool,
) -> i32 {
    regmap_update_bits(regmap, reg, 1, on)
}

unsafe extern "C" fn chtdc_ti_pmic_get_raw_temp(regmap: *mut regmap, reg: i32) -> i32 {
    let mut buf = [0u8; 2];
    if regmap_bulk_read(regmap, reg, buf.as_mut_ptr(), core::mem::size_of_val(&buf)) != 0 {
        return -5; // -EIO
    }
    i32::from(u16::from_be_bytes(buf) & PMIC_REG_MASK)
}

static chtdc_ti_pmic_opregion_data: intel_pmic_opregion_data = intel_pmic_opregion_data {
    get_power: Some(chtdc_ti_pmic_get_power),
    update_power: Some(chtdc_ti_pmic_update_power),
    get_raw_temp: Some(chtdc_ti_pmic_get_raw_temp),
    lpat_raw_to_temp: Some(acpi_lpat_raw_to_temp),
    power_table: chtdc_ti_power_table.as_ptr(),
    power_table_count: chtdc_ti_power_table.len(),
    thermal_table: chtdc_ti_thermal_table.as_ptr(),
    thermal_table_count: chtdc_ti_thermal_table.len(),
    pmic_i2c_address: 0x5e,
};

unsafe extern "C" fn chtdc_ti_pmic_opregion_probe(pdev: *mut platform_device) -> i32 {
    let pmic = dev_get_drvdata(&mut (*pdev).dev);
    let err = intel_pmic_install_opregion_handler(
        &mut (*pdev).dev,
        core::ptr::null_mut(),
        (*pmic).regmap,
        &chtdc_ti_pmic_opregion_data,
    );
    if err < 0 {
        return err;
    }

    // Re-enumerate devices depending on PMIC.
    acpi_dev_clear_dependencies(core::ptr::null_mut());
    0
}

#[repr(C)]
struct platform_device_id {
    name: *const core::ffi::c_char,
}

static chtdc_ti_pmic_opregion_id_table: [platform_device_id; 2] = [
    platform_device_id { name: b"chtdc_ti_region\0".as_ptr() as *const _ },
    platform_device_id { name: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    name: *const core::ffi::c_char,
    id_table: *const platform_device_id,
}

static mut chtdc_ti_pmic_opregion_driver: platform_driver = platform_driver {
    probe: Some(chtdc_ti_pmic_opregion_probe),
    name: b"cht_dollar_cove_ti_pmic\0".as_ptr() as *const _,
    id_table: chtdc_ti_pmic_opregion_id_table.as_ptr(),
};

// builtin_platform_driver(chtdc_ti_pmic_opregion_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
