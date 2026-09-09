// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Bay Trail Crystal Cove PMIC operation region driver
 *
 * Copyright (C) 2014 Intel Corporation. All rights reserved.
 */

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct pmic_table {
    pub address: c_uint,
    pub reg: c_uint,
    pub bit: c_uint,
}

type GetPower = unsafe extern "C" fn(*mut regmap, c_int, c_int, *mut u64) -> c_int;
type UpdatePower = unsafe extern "C" fn(*mut regmap, c_int, c_int, bool) -> c_int;
type GetRawTemp = unsafe extern "C" fn(*mut regmap, c_int) -> c_int;
type UpdateAux = unsafe extern "C" fn(*mut regmap, c_int, c_int) -> c_int;
type GetPolicy = unsafe extern "C" fn(*mut regmap, c_int, c_int, *mut u64) -> c_int;
type UpdatePolicy = unsafe extern "C" fn(*mut regmap, c_int, c_int, c_int) -> c_int;
type LpatRawToTemp = unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int;

#[repr(C)]
pub struct intel_pmic_opregion_data {
    pub get_power: Option<GetPower>,
    pub update_power: Option<UpdatePower>,
    pub get_raw_temp: Option<GetRawTemp>,
    pub update_aux: Option<UpdateAux>,
    pub get_policy: Option<GetPolicy>,
    pub update_policy: Option<UpdatePolicy>,
    pub lpat_raw_to_temp: Option<LpatRawToTemp>,
    pub power_table: *const pmic_table,
    pub power_table_count: usize,
    pub thermal_table: *const pmic_table,
    pub thermal_table_count: usize,
    pub pmic_i2c_address: c_uint,
}

#[repr(C)]
pub struct intel_soc_pmic {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_int, val: *mut c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_int, val: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: c_int, val: c_int) -> c_int;
    fn acpi_lpat_raw_to_temp(arg: *mut c_void, raw: c_int, temp: *mut c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn acpi_handle(dev: *mut device) -> *mut c_void;
    fn intel_pmic_install_opregion_handler(
        dev: *mut device,
        handle: *mut c_void,
        regmap: *mut regmap,
        data: *const intel_pmic_opregion_data,
    ) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
}

const PWR_SOURCE_SELECT: c_int = 1 << 1;
const PMIC_A0LOCK_REG: c_int = 0xc5;

static POWER_TABLE: [pmic_table; 17] = [
    pmic_table { address: 0x04, reg: 0x63, bit: 0x00 },
    pmic_table { address: 0x08, reg: 0x62, bit: 0x00 },
    pmic_table { address: 0x0c, reg: 0x64, bit: 0x00 },
    pmic_table { address: 0x10, reg: 0x6a, bit: 0x00 },
    pmic_table { address: 0x14, reg: 0x6b, bit: 0x00 },
    pmic_table { address: 0x18, reg: 0x6c, bit: 0x00 },
    pmic_table { address: 0x1c, reg: 0x6d, bit: 0x00 },
    pmic_table { address: 0x24, reg: 0x66, bit: 0x00 },
    pmic_table { address: 0x2c, reg: 0x69, bit: 0x00 },
    pmic_table { address: 0x30, reg: 0x68, bit: 0x00 },
    pmic_table { address: 0x44, reg: 0x5c, bit: 0x00 },
    pmic_table { address: 0x48, reg: 0x5d, bit: 0x00 },
    pmic_table { address: 0x4c, reg: 0x5b, bit: 0x00 },
    pmic_table { address: 0x50, reg: 0x61, bit: 0x00 },
    pmic_table { address: 0x54, reg: 0x60, bit: 0x00 },
    pmic_table { address: 0x5c, reg: 0x56, bit: 0x00 },
    pmic_table { address: 0x60, reg: 0x57, bit: 0x00 },
];

static THERMAL_TABLE: [pmic_table; 12] = [
    pmic_table { address: 0x00, reg: 0x75, bit: 0 },
    pmic_table { address: 0x04, reg: 0x95, bit: 0 },
    pmic_table { address: 0x08, reg: 0x97, bit: 0 },
    pmic_table { address: 0x0c, reg: 0x77, bit: 0 },
    pmic_table { address: 0x10, reg: 0x9a, bit: 0 },
    pmic_table { address: 0x14, reg: 0x9c, bit: 0 },
    pmic_table { address: 0x18, reg: 0x79, bit: 0 },
    pmic_table { address: 0x1c, reg: 0x9f, bit: 0 },
    pmic_table { address: 0x20, reg: 0xa1, bit: 0 },
    pmic_table { address: 0x48, reg: 0x94, bit: 0 },
    pmic_table { address: 0x4c, reg: 0x99, bit: 0 },
    pmic_table { address: 0x50, reg: 0x9e, bit: 0 },
];

unsafe extern "C" fn intel_crc_pmic_get_power(regmap: *mut regmap, reg: c_int, bit: c_int, value: *mut u64) -> c_int {
    let mut data = 0;
    if regmap_read(regmap, reg, &mut data) != 0 { return -5; }
    *value = if (data & PWR_SOURCE_SELECT) != 0 && (data & (1 << bit)) != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn intel_crc_pmic_update_power(regmap: *mut regmap, reg: c_int, bit: c_int, on: bool) -> c_int {
    let mut data = 0;
    if regmap_read(regmap, reg, &mut data) != 0 { return -5; }
    if on { data |= PWR_SOURCE_SELECT | (1 << bit); } else { data &= !(1 << bit); data |= PWR_SOURCE_SELECT; }
    if regmap_write(regmap, reg, data) != 0 { return -5; }
    0
}

unsafe extern "C" fn intel_crc_pmic_get_raw_temp(regmap: *mut regmap, reg: c_int) -> c_int {
    let (mut temp_l, mut temp_h) = (0, 0);
    if regmap_read(regmap, reg, &mut temp_l) != 0 || regmap_read(regmap, reg - 1, &mut temp_h) != 0 { return -5; }
    temp_l | ((temp_h & 0x3) << 8)
}

unsafe extern "C" fn intel_crc_pmic_update_aux(regmap: *mut regmap, reg: c_int, raw: c_int) -> c_int {
    if regmap_write(regmap, reg, raw) != 0 || regmap_update_bits(regmap, reg - 1, 0x3, raw >> 8) != 0 { -5 } else { 0 }
}

unsafe extern "C" fn intel_crc_pmic_get_policy(regmap: *mut regmap, reg: c_int, _bit: c_int, value: *mut u64) -> c_int {
    let mut pen = 0;
    if regmap_read(regmap, reg, &mut pen) != 0 { return -5; }
    *value = (pen >> 7) as u64;
    0
}

unsafe extern "C" fn intel_crc_pmic_update_policy(regmap: *mut regmap, reg: c_int, _bit: c_int, enable: c_int) -> c_int {
    let mut alert0 = 0;
    if regmap_read(regmap, PMIC_A0LOCK_REG, &mut alert0) != 0 { return -5; }
    if regmap_update_bits(regmap, PMIC_A0LOCK_REG, 0x01, 0) != 0 { return -5; }
    if regmap_update_bits(regmap, reg, 0x80, enable << 7) != 0 { return -5; }
    if regmap_write(regmap, PMIC_A0LOCK_REG, alert0) != 0 { return -5; }
    0
}

static INTEL_CRC_PMIC_OPREGION_DATA: intel_pmic_opregion_data = intel_pmic_opregion_data {
    get_power: Some(intel_crc_pmic_get_power), update_power: Some(intel_crc_pmic_update_power),
    get_raw_temp: Some(intel_crc_pmic_get_raw_temp), update_aux: Some(intel_crc_pmic_update_aux),
    get_policy: Some(intel_crc_pmic_get_policy), update_policy: Some(intel_crc_pmic_update_policy),
    lpat_raw_to_temp: Some(acpi_lpat_raw_to_temp), power_table: POWER_TABLE.as_ptr(), power_table_count: 17,
    thermal_table: THERMAL_TABLE.as_ptr(), thermal_table_count: 12, pmic_i2c_address: 0x6e,
};

unsafe extern "C" fn intel_crc_pmic_opregion_probe(pdev: *mut platform_device) -> c_int {
    let pmic = dev_get_drvdata(&mut (*pdev).dev as *mut device) as *mut intel_soc_pmic;
    intel_pmic_install_opregion_handler(&mut (*pdev).dev, acpi_handle(&mut (*pdev).dev), (*pmic).regmap, &INTEL_CRC_PMIC_OPREGION_DATA)
}

static mut INTEL_CRC_PMIC_OPREGION_DRIVER: platform_driver = platform_driver {
    probe: Some(intel_crc_pmic_opregion_probe),
    driver: driver { name: b"byt_crystal_cove_pmic\0".as_ptr() as *const c_char },
};

#[allow(dead_code)]
unsafe fn register_intel_crc_pmic_opregion_driver() -> c_int {
    platform_driver_register(&mut INTEL_CRC_PMIC_OPREGION_DRIVER)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
