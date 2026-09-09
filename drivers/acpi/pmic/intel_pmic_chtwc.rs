// SPDX-License-Identifier: GPL-2.0
/*
 * Intel CHT Whiskey Cove PMIC operation region driver
 * Copyright (C) 2017 Hans de Goede <hdegoede@redhat.com>
 *
 * Based on various non upstream patches to support the CHT Whiskey Cove PMIC:
 * Copyright (C) 2013-2015 Intel Corporation. All rights reserved.
 */

// C dependencies: linux/acpi.h, linux/init.h, linux/mfd/intel_soc_pmic.h,
// linux/platform_device.h, linux/regmap.h, and intel_pmic.h.

const CHT_WC_V1P05A_CTRL: u32 = 0x6e3b;
const CHT_WC_V1P15_CTRL: u32 = 0x6e3c;
const CHT_WC_V1P05A_VSEL: u32 = 0x6e3d;
const CHT_WC_V1P15_VSEL: u32 = 0x6e3e;
const CHT_WC_V1P8A_CTRL: u32 = 0x6e56;
const CHT_WC_V1P8SX_CTRL: u32 = 0x6e57;
const CHT_WC_VDDQ_CTRL: u32 = 0x6e58;
const CHT_WC_V1P2A_CTRL: u32 = 0x6e59;
const CHT_WC_V1P2SX_CTRL: u32 = 0x6e5a;
const CHT_WC_V1P8A_VSEL: u32 = 0x6e5b;
const CHT_WC_VDDQ_VSEL: u32 = 0x6e5c;
const CHT_WC_V2P8SX_CTRL: u32 = 0x6e5d;
const CHT_WC_V3P3A_CTRL: u32 = 0x6e5e;
const CHT_WC_V3P3SD_CTRL: u32 = 0x6e5f;
const CHT_WC_VSDIO_CTRL: u32 = 0x6e67;
const CHT_WC_V3P3A_VSEL: u32 = 0x6e68;
const CHT_WC_VPROG1A_CTRL: u32 = 0x6e90;
const CHT_WC_VPROG1B_CTRL: u32 = 0x6e91;
const CHT_WC_VPROG1F_CTRL: u32 = 0x6e95;
const CHT_WC_VPROG2D_CTRL: u32 = 0x6e99;
const CHT_WC_VPROG3A_CTRL: u32 = 0x6e9a;
const CHT_WC_VPROG3B_CTRL: u32 = 0x6e9b;
const CHT_WC_VPROG4A_CTRL: u32 = 0x6e9c;
const CHT_WC_VPROG4B_CTRL: u32 = 0x6e9d;
const CHT_WC_VPROG4C_CTRL: u32 = 0x6e9e;
const CHT_WC_VPROG4D_CTRL: u32 = 0x6e9f;
const CHT_WC_VPROG5A_CTRL: u32 = 0x6ea0;
const CHT_WC_VPROG5B_CTRL: u32 = 0x6ea1;
const CHT_WC_VPROG6A_CTRL: u32 = 0x6ea2;
const CHT_WC_VPROG6B_CTRL: u32 = 0x6ea3;
const CHT_WC_VPROG1A_VSEL: u32 = 0x6ec0;
const CHT_WC_VPROG1B_VSEL: u32 = 0x6ec1;
const CHT_WC_V1P8SX_VSEL: u32 = 0x6ec2;
const CHT_WC_V1P2SX_VSEL: u32 = 0x6ec3;
const CHT_WC_V1P2A_VSEL: u32 = 0x6ec4;
const CHT_WC_VPROG1F_VSEL: u32 = 0x6ec5;
const CHT_WC_VSDIO_VSEL: u32 = 0x6ec6;
const CHT_WC_V2P8SX_VSEL: u32 = 0x6ec7;
const CHT_WC_V3P3SD_VSEL: u32 = 0x6ec8;
const CHT_WC_VPROG2D_VSEL: u32 = 0x6ec9;
const CHT_WC_VPROG3A_VSEL: u32 = 0x6eca;
const CHT_WC_VPROG3B_VSEL: u32 = 0x6ecb;
const CHT_WC_VPROG4A_VSEL: u32 = 0x6ecc;
const CHT_WC_VPROG4B_VSEL: u32 = 0x6ecd;
const CHT_WC_VPROG4C_VSEL: u32 = 0x6ece;
const CHT_WC_VPROG4D_VSEL: u32 = 0x6ecf;
const CHT_WC_VPROG5A_VSEL: u32 = 0x6ed0;
const CHT_WC_VPROG5B_VSEL: u32 = 0x6ed1;
const CHT_WC_VPROG6A_VSEL: u32 = 0x6ed2;
const CHT_WC_VPROG6B_VSEL: u32 = 0x6ed3;

// Regulator support is based on the non upstream Whiskey Cove VRF patch.
static POWER_TABLE: &[PmicTable] = &[
    PmicTable { address: 0x00, reg: CHT_WC_V1P8A_CTRL, bit: 0x01 },
    PmicTable { address: 0x04, reg: CHT_WC_V1P8SX_CTRL, bit: 0x07 },
    PmicTable { address: 0x08, reg: CHT_WC_VDDQ_CTRL, bit: 0x01 },
    PmicTable { address: 0x0c, reg: CHT_WC_V1P2A_CTRL, bit: 0x07 },
    PmicTable { address: 0x10, reg: CHT_WC_V1P2SX_CTRL, bit: 0x07 },
    PmicTable { address: 0x14, reg: CHT_WC_V2P8SX_CTRL, bit: 0x07 },
    PmicTable { address: 0x18, reg: CHT_WC_V3P3A_CTRL, bit: 0x01 },
    PmicTable { address: 0x1c, reg: CHT_WC_V3P3SD_CTRL, bit: 0x07 },
    PmicTable { address: 0x20, reg: CHT_WC_VSDIO_CTRL, bit: 0x07 },
    PmicTable { address: 0x34, reg: CHT_WC_VPROG1A_CTRL, bit: 0x07 },
    PmicTable { address: 0x38, reg: CHT_WC_VPROG1B_CTRL, bit: 0x07 },
    PmicTable { address: 0x3c, reg: CHT_WC_VPROG1F_CTRL, bit: 0x07 },
    PmicTable { address: 0x40, reg: CHT_WC_VPROG2D_CTRL, bit: 0x07 },
    PmicTable { address: 0x44, reg: CHT_WC_VPROG3A_CTRL, bit: 0x07 },
    PmicTable { address: 0x48, reg: CHT_WC_VPROG3B_CTRL, bit: 0x07 },
    PmicTable { address: 0x4c, reg: CHT_WC_VPROG4A_CTRL, bit: 0x07 },
    PmicTable { address: 0x50, reg: CHT_WC_VPROG4B_CTRL, bit: 0x07 },
    PmicTable { address: 0x54, reg: CHT_WC_VPROG4C_CTRL, bit: 0x07 },
    PmicTable { address: 0x58, reg: CHT_WC_VPROG4D_CTRL, bit: 0x07 },
    PmicTable { address: 0x5c, reg: CHT_WC_VPROG5A_CTRL, bit: 0x07 },
    PmicTable { address: 0x60, reg: CHT_WC_VPROG5B_CTRL, bit: 0x07 },
    PmicTable { address: 0x64, reg: CHT_WC_VPROG6A_CTRL, bit: 0x07 },
    PmicTable { address: 0x68, reg: CHT_WC_VPROG6B_CTRL, bit: 0x07 },
];

unsafe fn intel_cht_wc_pmic_get_power(regmap: *mut Regmap, reg: i32, bit: i32, value: *mut u64) -> i32 {
    let mut data: i32 = 0;
    if regmap_read(regmap, reg, &mut data) != 0 { return -5; }
    *value = if data & bit != 0 { 1 } else { 0 };
    0
}

unsafe fn intel_cht_wc_pmic_update_power(regmap: *mut Regmap, reg: i32, bitmask: i32, on: bool) -> i32 {
    regmap_update_bits(regmap, reg, bitmask as u32, if on { 1 } else { 0 })
}

unsafe fn intel_cht_wc_exec_mipi_pmic_seq_element(regmap: *mut Regmap, i2c_client_address: u16, reg_address: u32, value: u32, mask: u32) -> i32 {
    if i2c_client_address > 0xff || reg_address > 0xff { return -34; }
    let address = ((i2c_client_address as u32) << 8) | reg_address;
    regmap_update_bits(regmap, address as i32, mask, value)
}

// The thermal table and ops are empty; DPTF is unsupported due to lacking documentation.
static INTEL_CHT_WC_PMIC_OPREGION_DATA: IntelPmicOpregionData = IntelPmicOpregionData {
    get_power: Some(intel_cht_wc_pmic_get_power),
    update_power: Some(intel_cht_wc_pmic_update_power),
    exec_mipi_pmic_seq_element: Some(intel_cht_wc_exec_mipi_pmic_seq_element),
    lpat_raw_to_temp: Some(acpi_lpat_raw_to_temp),
    power_table: POWER_TABLE,
};

unsafe extern "C" {
    fn regmap_read(regmap: *mut Regmap, reg: i32, val: *mut i32) -> i32;
    fn regmap_update_bits(regmap: *mut Regmap, reg: i32, mask: u32, val: u32) -> i32;
    fn acpi_lpat_raw_to_temp(raw: i64, lpat: *mut core::ffi::c_void) -> i64;
}

// External kernel types and opregion installation routine are supplied by dependencies.
#[allow(improper_ctypes)]
unsafe extern "C" {
    type Regmap;
    type PmicTable;
    type IntelPmicOpregionData;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
