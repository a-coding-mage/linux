// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP and TWL PMIC specific initializations.
 *
 * Copyright (C) 2010 Texas Instruments Incorporated.
 * Thara Gopinath
 * Copyright (C) 2009 Texas Instruments Incorporated.
 * Nishanth Menon
 * Copyright (C) 2009 Nokia Corporation
 * Paul Walmsley
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn twl_i2c_read_u8(module: u8, value: *mut u8, reg: u8) -> c_int;
    fn pr_err(format: *const c_char, ...);
    fn cpu_is_omap44xx() -> bool;
    fn cpu_is_omap34xx() -> bool;
    fn of_find_compatible_node(from: *mut c_void, type_: *mut c_void, compatible: *const c_char) -> *mut c_void;
    fn voltdm_lookup(name: *const c_char) -> *mut voltagedomain;
    fn omap_voltage_register_pmic(voltdm: *mut voltagedomain, pmic: *mut omap_voltdm_pmic);
}

#[repr(C)]
pub struct voltagedomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_voltdm_pmic {
    pub slew_rate: u32,
    pub step_size: u32,
    pub vp_erroroffset: u32,
    pub vp_vstepmin: u32,
    pub vp_vstepmax: u32,
    pub vddmin: u32,
    pub vddmax: u32,
    pub vp_timeout_us: u32,
    pub i2c_slave_addr: u32,
    pub volt_reg_addr: u32,
    pub cmd_reg_addr: u32,
    pub i2c_high_speed: bool,
    pub i2c_pad_load: u32,
    pub vsel_to_uv: unsafe extern "C" fn(u8) -> c_ulong,
    pub uv_to_vsel: unsafe extern "C" fn(c_ulong) -> u8,
}

const TWL6030_MODULE_ID0: u8 = 0;
const ENODEV: c_int = 19;

const OMAP3_SRI2C_SLAVE_ADDR: u32 = 0x12;
const OMAP3_VDD_MPU_SR_CONTROL_REG: u32 = 0x00;
const OMAP3_VDD_CORE_SR_CONTROL_REG: u32 = 0x01;
const OMAP3_VP_CONFIG_ERROROFFSET: u32 = 0x00;
const OMAP3_VP_VSTEPMIN_VSTEPMIN: u32 = 0x1;
const OMAP3_VP_VSTEPMAX_VSTEPMAX: u32 = 0x04;
const OMAP3_VP_VLIMITTO_TIMEOUT_US: u32 = 200;

const OMAP4_SRI2C_SLAVE_ADDR: u32 = 0x12;
const OMAP4_VDD_MPU_SR_VOLT_REG: u32 = 0x55;
const OMAP4_VDD_MPU_SR_CMD_REG: u32 = 0x56;
const OMAP4_VDD_IVA_SR_VOLT_REG: u32 = 0x5B;
const OMAP4_VDD_IVA_SR_CMD_REG: u32 = 0x5C;
const OMAP4_VDD_CORE_SR_VOLT_REG: u32 = 0x61;
const OMAP4_VDD_CORE_SR_CMD_REG: u32 = 0x62;
const OMAP4_VP_CONFIG_ERROROFFSET: u32 = 0x00;
const OMAP4_VP_VSTEPMIN_VSTEPMIN: u32 = 0x1;
const OMAP4_VP_VSTEPMAX_VSTEPMAX: u32 = 0x04;
const OMAP4_VP_VLIMITTO_TIMEOUT_US: u32 = 200;

static mut is_offset_valid: bool = false;
static mut smps_offset: u8 = 0;
const REG_SMPS_OFFSET: u8 = 0xE0;

const fn div_round_up(x: c_ulong, d: c_ulong) -> c_ulong { (x + d - 1) / d }

unsafe extern "C" fn twl4030_vsel_to_uv(vsel: u8) -> c_ulong {
    (((vsel as c_ulong * 125) + 6000) * 100)
}

unsafe extern "C" fn twl4030_uv_to_vsel(uv: c_ulong) -> u8 {
    div_round_up(uv - 600000, 12500) as u8
}

unsafe extern "C" fn twl6030_vsel_to_uv(vsel: u8) -> c_ulong {
    if !is_offset_valid {
        twl_i2c_read_u8(TWL6030_MODULE_ID0, &mut smps_offset, REG_SMPS_OFFSET);
        is_offset_valid = true;
    }
    if vsel == 0 { return 0; }
    if vsel == 0x3A { return 1350000; }
    if smps_offset & 0x8 != 0 {
        (((vsel as c_ulong - 1) * 1266 + 70900) * 10)
    } else {
        (((vsel as c_ulong - 1) * 1266 + 60770) * 10)
    }
}

unsafe extern "C" fn twl6030_uv_to_vsel(uv: c_ulong) -> u8 {
    if !is_offset_valid {
        twl_i2c_read_u8(TWL6030_MODULE_ID0, &mut smps_offset, REG_SMPS_OFFSET);
        is_offset_valid = true;
    }
    if uv == 0 { return 0x00; }
    if uv > twl6030_vsel_to_uv(0x39) {
        if uv == 1350000 { return 0x3A; }
        // pr_err("%s:OUT OF RANGE! non mapped vsel for %ld Vs max %ld\n", __func__, uv, twl6030_vsel_to_uv(0x39));
        return 0x3A;
    }
    if smps_offset & 0x8 != 0 {
        (div_round_up(uv - 709000, 12660) + 1) as u8
    } else {
        (div_round_up(uv - 607700, 12660) + 1) as u8
    }
}

static mut omap3_mpu_pmic: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500, vp_erroroffset: OMAP3_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP3_VP_VSTEPMIN_VSTEPMIN, vp_vstepmax: OMAP3_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 600000, vddmax: 1450000, vp_timeout_us: OMAP3_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: OMAP3_SRI2C_SLAVE_ADDR, volt_reg_addr: OMAP3_VDD_MPU_SR_CONTROL_REG,
    cmd_reg_addr: 0, i2c_high_speed: true, i2c_pad_load: 0,
    vsel_to_uv: twl4030_vsel_to_uv, uv_to_vsel: twl4030_uv_to_vsel,
};

static mut omap3_core_pmic: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500, vp_erroroffset: OMAP3_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP3_VP_VSTEPMIN_VSTEPMIN, vp_vstepmax: OMAP3_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 600000, vddmax: 1450000, vp_timeout_us: OMAP3_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: OMAP3_SRI2C_SLAVE_ADDR, volt_reg_addr: OMAP3_VDD_CORE_SR_CONTROL_REG,
    cmd_reg_addr: 0, i2c_high_speed: true, i2c_pad_load: 0,
    vsel_to_uv: twl4030_vsel_to_uv, uv_to_vsel: twl4030_uv_to_vsel,
};

static mut omap4_mpu_pmic: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12660, vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN, vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 0, vddmax: 2100000, vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: OMAP4_SRI2C_SLAVE_ADDR, volt_reg_addr: OMAP4_VDD_MPU_SR_VOLT_REG,
    cmd_reg_addr: OMAP4_VDD_MPU_SR_CMD_REG, i2c_high_speed: true, i2c_pad_load: 3,
    vsel_to_uv: twl6030_vsel_to_uv, uv_to_vsel: twl6030_uv_to_vsel,
};

static mut omap4_iva_pmic: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12660, vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN, vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 0, vddmax: 2100000, vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: OMAP4_SRI2C_SLAVE_ADDR, volt_reg_addr: OMAP4_VDD_IVA_SR_VOLT_REG,
    cmd_reg_addr: OMAP4_VDD_IVA_SR_CMD_REG, i2c_high_speed: true, i2c_pad_load: 3,
    vsel_to_uv: twl6030_vsel_to_uv, uv_to_vsel: twl6030_uv_to_vsel,
};

static mut omap4_core_pmic: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12660, vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN, vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 0, vddmax: 2100000, vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: OMAP4_SRI2C_SLAVE_ADDR, volt_reg_addr: OMAP4_VDD_CORE_SR_VOLT_REG,
    cmd_reg_addr: OMAP4_VDD_CORE_SR_CMD_REG, i2c_high_speed: true, i2c_pad_load: 3,
    vsel_to_uv: twl6030_vsel_to_uv, uv_to_vsel: twl6030_uv_to_vsel,
};

#[no_mangle]
pub unsafe extern "C" fn omap4_twl_init() -> c_int {
    if !cpu_is_omap44xx() || !of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"motorola,cpcap\0".as_ptr() as *const c_char).is_null() { return -ENODEV; }
    let mut voltdm = voltdm_lookup(b"mpu\0".as_ptr() as *const c_char); omap_voltage_register_pmic(voltdm, &mut omap4_mpu_pmic);
    voltdm = voltdm_lookup(b"iva\0".as_ptr() as *const c_char); omap_voltage_register_pmic(voltdm, &mut omap4_iva_pmic);
    voltdm = voltdm_lookup(b"core\0".as_ptr() as *const c_char); omap_voltage_register_pmic(voltdm, &mut omap4_core_pmic);
    0
}

#[no_mangle]
pub unsafe extern "C" fn omap3_twl_init() -> c_int {
    if !cpu_is_omap34xx() { return -ENODEV; }
    let mut voltdm = voltdm_lookup(b"mpu_iva\0".as_ptr() as *const c_char); omap_voltage_register_pmic(voltdm, &mut omap3_mpu_pmic);
    voltdm = voltdm_lookup(b"core\0".as_ptr() as *const c_char); omap_voltage_register_pmic(voltdm, &mut omap3_core_pmic);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
