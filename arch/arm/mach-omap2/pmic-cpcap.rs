// SPDX-License-Identifier: GPL-2.0-only
/*
 * pmic-cpcap.c - CPCAP-specific functions for the OPP code
 *
 * Adapted from Motorola Mapphone Android Linux kernel
 * Copyright (C) 2011 Motorola, Inc.
 */

// Linux dependencies supplied by the surrounding translation unit.

/**
 * omap_cpcap_vsel_to_uv - convert CPCAP VSEL value to microvolts DC
 * @vsel: CPCAP VSEL value to convert
 *
 * Returns: the microvolts DC that the CPCAP PMIC should generate when
 * programmed with @vsel.
 */
unsafe fn omap_cpcap_vsel_to_uv(mut vsel: u8) -> libc::c_ulong {
    if vsel > 0x44 { vsel = 0x44; }
    (((vsel as libc::c_ulong * 125) + 6000) * 100)
}

unsafe fn omap_cpcap_uv_to_vsel(mut uv: libc::c_ulong) -> u8 {
    if uv < 600000 { uv = 600000; }
    else if uv > 1450000 { uv = 1450000; }
    ((uv - 600000 + 12500 - 1) / 12500) as u8
}

static mut omap_cpcap_core: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500,
    vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN,
    vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 900000, vddmax: 1350000,
    vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: 0x02, volt_reg_addr: 0x00, cmd_reg_addr: 0x01,
    i2c_high_speed: false, vsel_to_uv: Some(omap_cpcap_vsel_to_uv),
    uv_to_vsel: Some(omap_cpcap_uv_to_vsel),
};

static mut omap_cpcap_iva: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500,
    vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN,
    vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 900000, vddmax: 1375000,
    vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: 0x44, volt_reg_addr: 0x0, cmd_reg_addr: 0x01,
    i2c_high_speed: false, vsel_to_uv: Some(omap_cpcap_vsel_to_uv),
    uv_to_vsel: Some(omap_cpcap_uv_to_vsel),
};

unsafe fn omap_max8952_vsel_to_uv(mut vsel: u8) -> libc::c_ulong {
    if vsel > 0x3F { vsel = 0x3F; }
    (((vsel as libc::c_ulong * 100) + 7700) * 100)
}

unsafe fn omap_max8952_uv_to_vsel(mut uv: libc::c_ulong) -> u8 {
    if uv < 770000 { uv = 770000; }
    else if uv > 1400000 { uv = 1400000; }
    ((uv - 770000 + 10000 - 1) / 10000) as u8
}

static mut omap443x_max8952_mpu: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 16000, step_size: 10000,
    vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN,
    vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 900000, vddmax: 1400000,
    vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: 0x60, volt_reg_addr: 0x03, cmd_reg_addr: 0x03,
    i2c_high_speed: false, vsel_to_uv: Some(omap_max8952_vsel_to_uv),
    uv_to_vsel: Some(omap_max8952_uv_to_vsel),
};

unsafe fn omap_fan535503_vsel_to_uv(mut vsel: u8) -> libc::c_ulong {
    /* Extract bits[5:0] */
    vsel &= 0x3F;
    (((vsel as libc::c_ulong * 125) + 7500) * 100)
}

unsafe fn omap_fan535508_vsel_to_uv(mut vsel: u8) -> libc::c_ulong {
    /* Extract bits[5:0] */
    vsel &= 0x3F;
    if vsel > 0x37 { vsel = 0x37; }
    (((vsel as libc::c_ulong * 125) + 7500) * 100)
}

unsafe fn omap_fan535503_uv_to_vsel(mut uv: libc::c_ulong) -> u8 {
    if uv < 750000 { uv = 750000; }
    else if uv > 1537500 { uv = 1537500; }
    (((uv - 750000 + 12500 - 1) / 12500) as u8) | 0xC0
}

unsafe fn omap_fan535508_uv_to_vsel(mut uv: libc::c_ulong) -> u8 {
    if uv < 750000 { uv = 750000; }
    else if uv > 1437500 { uv = 1437500; }
    (((uv - 750000 + 12500 - 1) / 12500) as u8) | 0xC0
}

/* fan5335-core */
static mut omap4_fan_core: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500,
    vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN,
    vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 850000, vddmax: 1375000,
    vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: 0x4A, i2c_high_speed: false,
    volt_reg_addr: 0x01, cmd_reg_addr: 0x01,
    vsel_to_uv: Some(omap_fan535508_vsel_to_uv), uv_to_vsel: Some(omap_fan535508_uv_to_vsel),
};

/* fan5335 iva */
static mut omap4_fan_iva: omap_voltdm_pmic = omap_voltdm_pmic {
    slew_rate: 4000, step_size: 12500,
    vp_erroroffset: OMAP4_VP_CONFIG_ERROROFFSET,
    vp_vstepmin: OMAP4_VP_VSTEPMIN_VSTEPMIN,
    vp_vstepmax: OMAP4_VP_VSTEPMAX_VSTEPMAX,
    vddmin: 850000, vddmax: 1375000,
    vp_timeout_us: OMAP4_VP_VLIMITTO_TIMEOUT_US,
    i2c_slave_addr: 0x48, volt_reg_addr: 0x01, cmd_reg_addr: 0x01,
    i2c_high_speed: false, vsel_to_uv: Some(omap_fan535503_vsel_to_uv),
    uv_to_vsel: Some(omap_fan535503_uv_to_vsel),
};

pub unsafe fn omap4_cpcap_init() -> i32 {
    let mut voltdm: *mut voltagedomain;
    if of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"motorola,cpcap\0".as_ptr() as *const i8).is_null() { return -19; }
    voltdm = voltdm_lookup(b"mpu\0".as_ptr() as *const i8);
    omap_voltage_register_pmic(voltdm, &raw mut omap443x_max8952_mpu);
    if of_machine_is_compatible(b"motorola,droid-bionic\0".as_ptr() as *const i8) {
        voltdm = voltdm_lookup(b"core\0".as_ptr() as *const i8); omap_voltage_register_pmic(voltdm, &raw mut omap_cpcap_core);
        voltdm = voltdm_lookup(b"iva\0".as_ptr() as *const i8); omap_voltage_register_pmic(voltdm, &raw mut omap_cpcap_iva);
    } else {
        voltdm = voltdm_lookup(b"core\0".as_ptr() as *const i8); omap_voltage_register_pmic(voltdm, &raw mut omap4_fan_core);
        voltdm = voltdm_lookup(b"iva\0".as_ptr() as *const i8); omap_voltage_register_pmic(voltdm, &raw mut omap4_fan_iva);
    }
    0
}

unsafe fn cpcap_late_init() -> i32 {
    if of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"motorola,cpcap\0".as_ptr() as *const i8).is_null() { return 0; }
    if soc_is_omap443x() || soc_is_omap446x() || soc_is_omap447x() { omap4_vc_set_pmic_signaling(PWRDM_POWER_RET); }
    0
}

// omap_late_initcall(cpcap_late_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
