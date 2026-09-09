// SPDX-License-Identifier: GPL-2.0
/*
 * Intel BXT WhiskeyCove PMIC operation region driver
 *
 * Copyright (C) 2015 Intel Corporation. All rights reserved.
 */

// External Linux/ACPI and Intel PMIC dependencies are supplied by the surrounding crate.

const WHISKEY_COVE_ALRT_HIGH_BIT_MASK: u8 = 0x0F;
#[inline]
const fn whiskey_cove_adc_high_bit(x: u8) -> u16 { ((x & 0x0F) as u16) << 8 }
#[inline]
const fn whiskey_cove_adc_cursrc(x: u8) -> u8 { (x & 0xF0) >> 4 }
const VR_MODE_DISABLED: u8 = 0;
const VR_MODE_AUTO: u8 = 1 << 0;
const VR_MODE_NORMAL: u8 = 1 << 1;
const VR_MODE_SWITCH: u8 = 1 << 2;
const VR_MODE_ECO: u8 = (1 << 0) | (1 << 1);
const VSWITCH2_OUTPUT: u8 = 1 << 5;
const VSWITCH1_OUTPUT: u8 = 1 << 4;
const VUSBPHY_CHARGE: u8 = 1 << 1;

static POWER_TABLE: [pmic_table; 30] = [
    pmic_table { address: 0x0, reg: 0x63, bit: VR_MODE_AUTO }, pmic_table { address: 0x04, reg: 0x65, bit: VR_MODE_AUTO },
    pmic_table { address: 0x08, reg: 0x67, bit: VR_MODE_AUTO }, pmic_table { address: 0x0c, reg: 0x6d, bit: VR_MODE_AUTO },
    pmic_table { address: 0x10, reg: 0x6f, bit: VR_MODE_NORMAL }, pmic_table { address: 0x14, reg: 0x70, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x18, reg: 0x71, bit: VR_MODE_NORMAL }, pmic_table { address: 0x1c, reg: 0x72, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x20, reg: 0x73, bit: VR_MODE_NORMAL }, pmic_table { address: 0x24, reg: 0x74, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x28, reg: 0x75, bit: VR_MODE_NORMAL }, pmic_table { address: 0x2c, reg: 0x76, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x30, reg: 0x77, bit: VR_MODE_NORMAL }, pmic_table { address: 0x34, reg: 0x78, bit: VSWITCH2_OUTPUT },
    pmic_table { address: 0x38, reg: 0x78, bit: VSWITCH1_OUTPUT }, pmic_table { address: 0x3c, reg: 0x78, bit: VUSBPHY_CHARGE },
    pmic_table { address: 0x40, reg: 0x7b, bit: VR_MODE_NORMAL }, pmic_table { address: 0x44, reg: 0xA0, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x48, reg: 0xA1, bit: VR_MODE_NORMAL }, pmic_table { address: 0x4c, reg: 0xA2, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x50, reg: 0xA3, bit: VR_MODE_NORMAL }, pmic_table { address: 0x54, reg: 0xA4, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x58, reg: 0xA5, bit: VR_MODE_NORMAL }, pmic_table { address: 0x5c, reg: 0xA6, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x60, reg: 0xA7, bit: VR_MODE_NORMAL }, pmic_table { address: 0x64, reg: 0xA8, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x68, reg: 0xA9, bit: VR_MODE_NORMAL }, pmic_table { address: 0x6c, reg: 0xAA, bit: VR_MODE_NORMAL },
    pmic_table { address: 0x70, reg: 0x36, bit: 1 << 2 }, pmic_table { address: 0x74, reg: 0x36, bit: 1 << 0 },
];

static THERMAL_TABLE: [pmic_table; 21] = [
    pmic_table { address: 0x00, reg: 0x4F39, bit: 0 }, pmic_table { address: 0x04, reg: 0x4F24, bit: 0 },
    pmic_table { address: 0x08, reg: 0x4F26, bit: 0 }, pmic_table { address: 0x0c, reg: 0x4F3B, bit: 0 },
    pmic_table { address: 0x10, reg: 0x4F28, bit: 0 }, pmic_table { address: 0x14, reg: 0x4F2A, bit: 0 },
    pmic_table { address: 0x18, reg: 0x4F3D, bit: 0 }, pmic_table { address: 0x1c, reg: 0x4F2C, bit: 0 },
    pmic_table { address: 0x20, reg: 0x4F2E, bit: 0 }, pmic_table { address: 0x24, reg: 0x4F3F, bit: 0 },
    pmic_table { address: 0x28, reg: 0x4F30, bit: 0 }, pmic_table { address: 0x30, reg: 0x4F41, bit: 0 },
    pmic_table { address: 0x34, reg: 0x4F32, bit: 0 }, pmic_table { address: 0x3c, reg: 0x4F43, bit: 0 },
    pmic_table { address: 0x40, reg: 0x4F34, bit: 0 }, pmic_table { address: 0x48, reg: 0x4F6A, bit: 0 },
    pmic_table { address: 0x4C, reg: 0x4F6A, bit: 1 }, pmic_table { address: 0x50, reg: 0x4F6A, bit: 2 },
    pmic_table { address: 0x54, reg: 0x4F6A, bit: 4 }, pmic_table { address: 0x58, reg: 0x4F6A, bit: 5 },
    pmic_table { address: 0x5C, reg: 0x4F6A, bit: 3 },
];

unsafe fn intel_bxtwc_pmic_get_power(regmap: *mut regmap, reg: i32, bit: i32, value: *mut u64) -> i32 {
    let mut data = 0i32;
    if regmap_read(regmap, reg, &mut data) != 0 { return -5; }
    *value = if (data & bit) != 0 { 1 } else { 0 }; 0
}

unsafe fn intel_bxtwc_pmic_update_power(regmap: *mut regmap, reg: i32, bit: i32, on: bool) -> i32 {
    let val: u8 = if on { 0xFF } else { 0 };
    regmap_update_bits(regmap, reg, bit as u8, val)
}

unsafe fn intel_bxtwc_pmic_get_raw_temp(regmap: *mut regmap, reg: i32) -> i32 {
    let mut val = 0u32;
    let rlsb_array: [u32; 12] = [0, 260420, 130210, 65100, 32550, 16280, 8140, 4070, 2030, 0, 260420, 130210];
    if regmap_read(regmap, reg, &mut val) != 0 { return -5; }
    let temp_l = val as u8;
    if regmap_read(regmap, reg - 1, &mut val) != 0 { return -5; }
    let temp_h = val as u8;
    let reg_val = temp_l as u32 | whiskey_cove_adc_high_bit(temp_h) as u32;
    let cursrc = whiskey_cove_adc_cursrc(temp_h) as usize;
    (reg_val * rlsb_array[cursrc] / 1000) as i32
}

unsafe fn intel_bxtwc_pmic_update_aux(regmap: *mut regmap, reg: i32, raw: i32) -> i32 {
    let bsr_num = (raw as u32) / (1 << 5);
    let count = 31 - bsr_num.leading_zeros();
    let cursel = ((count as i32 - 7).clamp(0, 7)) as u16;
    let thrsh = (raw as u32) / (1 << (4 + cursel));
    let resi_val = (cursel << 9) | thrsh as u16;
    let alrt_h = ((resi_val >> 8) as u8) & WHISKEY_COVE_ALRT_HIGH_BIT_MASK;
    if regmap_update_bits(regmap, reg - 1, WHISKEY_COVE_ALRT_HIGH_BIT_MASK, alrt_h) != 0 { return -5; }
    regmap_write(regmap, reg, resi_val as u8)
}

unsafe fn intel_bxtwc_pmic_get_policy(regmap: *mut regmap, reg: i32, bit: i32, value: *mut u64) -> i32 {
    let mut val = 0u32;
    if regmap_read(regmap, reg, &mut val) != 0 { return -5; }
    *value = ((val & (1 << bit)) >> bit) as u64; 0
}

unsafe fn intel_bxtwc_pmic_update_policy(regmap: *mut regmap, reg: i32, bit: i32, enable: i32) -> i32 {
    regmap_update_bits(regmap, reg, (1 << bit) as u8, (enable << bit) as u8)
}

// The remaining operation-region and platform-driver aggregate definitions use the
// corresponding external kernel bindings; their field values mirror the C source.
static INTEL_BXTWC_PMIC_OPREGION_DATA: intel_pmic_opregion_data = intel_pmic_opregion_data {
    get_power: Some(intel_bxtwc_pmic_get_power), update_power: Some(intel_bxtwc_pmic_update_power),
    get_raw_temp: Some(intel_bxtwc_pmic_get_raw_temp), update_aux: Some(intel_bxtwc_pmic_update_aux),
    get_policy: Some(intel_bxtwc_pmic_get_policy), update_policy: Some(intel_bxtwc_pmic_update_policy),
    lpat_raw_to_temp: Some(acpi_lpat_raw_to_temp), power_table: POWER_TABLE.as_ptr(), power_table_count: POWER_TABLE.len(),
    thermal_table: THERMAL_TABLE.as_ptr(), thermal_table_count: THERMAL_TABLE.len(),
};

unsafe fn intel_bxtwc_pmic_opregion_probe(pdev: *mut platform_device) -> i32 {
    let pmic = dev_get_drvdata((*pdev).dev.parent);
    intel_pmic_install_opregion_handler(&mut (*pdev).dev, acpi_handle((*pdev).dev.parent), (*pmic).regmap, &INTEL_BXTWC_PMIC_OPREGION_DATA)
}

static BXT_WC_OPREGION_ID_TABLE: [platform_device_id; 2] = [platform_device_id { name: "bxt_wcove_region" }, platform_device_id { name: "" }];
static mut INTEL_BXTWC_PMIC_OPREGION_DRIVER: platform_driver = platform_driver {
    probe: Some(intel_bxtwc_pmic_opregion_probe), name: "bxt_whiskey_cove_pmic", id_table: BXT_WC_OPREGION_ID_TABLE.as_ptr(),
};

// builtin_platform_driver(intel_bxtwc_pmic_opregion_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
