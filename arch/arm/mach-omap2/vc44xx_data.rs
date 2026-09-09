// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 Voltage Controller (VC) data
 *
 * Copyright (C) 2007, 2010 Texas Instruments, Inc.
 * Rajendra Nayak <rnayak@ti.com>
 * Lesly A M <x0080970@ti.com>
 * Thara Gopinath <thara@ti.com>
 *
 * Copyright (C) 2008, 2011 Nokia Corporation
 * Kalle Jokiniemi
 * Paul Walmsley
 */

// VC data common to 44xx chips
// XXX This stuff presumably belongs in the vc3xxx.c or vc.c file.
static omap4_vc_common: omap_vc_common = omap_vc_common {
    bypass_val_reg: OMAP4_PRM_VC_VAL_BYPASS_OFFSET,
    data_shift: OMAP4430_DATA_SHIFT,
    slaveaddr_shift: OMAP4430_SLAVEADDR_SHIFT,
    regaddr_shift: OMAP4430_REGADDR_SHIFT,
    valid: OMAP4430_VALID_MASK,
    cmd_on_shift: OMAP4430_ON_SHIFT,
    cmd_on_mask: OMAP4430_ON_MASK,
    cmd_onlp_shift: OMAP4430_ONLP_SHIFT,
    cmd_ret_shift: OMAP4430_RET_SHIFT,
    cmd_off_shift: OMAP4430_OFF_SHIFT,
    i2c_cfg_reg: OMAP4_PRM_VC_CFG_I2C_MODE_OFFSET,
    i2c_cfg_clear_mask: OMAP4430_SRMODEEN_MASK | OMAP4430_HSMODEEN_MASK,
    i2c_cfg_hsen_mask: OMAP4430_HSMODEEN_MASK,
    i2c_mcode_mask: OMAP4430_HSMCODE_MASK,
    ..unsafe { core::mem::zeroed() }
};

// VC instance data for each controllable voltage line
pub static mut omap4_vc_mpu: omap_vc_channel = omap_vc_channel {
    flags: OMAP_VC_CHANNEL_DEFAULT | OMAP_VC_CHANNEL_CFG_MUTANT,
    common: &omap4_vc_common,
    smps_sa_reg: OMAP4_PRM_VC_SMPS_SA_OFFSET,
    smps_volra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_VOL_OFFSET,
    smps_cmdra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_CMD_OFFSET,
    cfg_channel_reg: OMAP4_PRM_VC_CFG_CHANNEL_OFFSET,
    cmdval_reg: OMAP4_PRM_VC_VAL_CMD_VDD_MPU_L_OFFSET,
    smps_sa_mask: OMAP4430_SA_VDD_MPU_L_PRM_VC_SMPS_SA_MASK,
    smps_volra_mask: OMAP4430_VOLRA_VDD_MPU_L_MASK,
    smps_cmdra_mask: OMAP4430_CMDRA_VDD_MPU_L_MASK,
    cfg_channel_sa_shift: OMAP4430_SA_VDD_MPU_L_SHIFT,
    ..unsafe { core::mem::zeroed() }
};

pub static mut omap4_vc_iva: omap_vc_channel = omap_vc_channel {
    common: &omap4_vc_common,
    smps_sa_reg: OMAP4_PRM_VC_SMPS_SA_OFFSET,
    smps_volra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_VOL_OFFSET,
    smps_cmdra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_CMD_OFFSET,
    cfg_channel_reg: OMAP4_PRM_VC_CFG_CHANNEL_OFFSET,
    cmdval_reg: OMAP4_PRM_VC_VAL_CMD_VDD_IVA_L_OFFSET,
    smps_sa_mask: OMAP4430_SA_VDD_IVA_L_PRM_VC_SMPS_SA_MASK,
    smps_volra_mask: OMAP4430_VOLRA_VDD_IVA_L_MASK,
    smps_cmdra_mask: OMAP4430_CMDRA_VDD_IVA_L_MASK,
    cfg_channel_sa_shift: OMAP4430_SA_VDD_IVA_L_SHIFT,
    ..unsafe { core::mem::zeroed() }
};

pub static mut omap4_vc_core: omap_vc_channel = omap_vc_channel {
    common: &omap4_vc_common,
    smps_sa_reg: OMAP4_PRM_VC_SMPS_SA_OFFSET,
    smps_volra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_VOL_OFFSET,
    smps_cmdra_reg: OMAP4_PRM_VC_VAL_SMPS_RA_CMD_OFFSET,
    cfg_channel_reg: OMAP4_PRM_VC_CFG_CHANNEL_OFFSET,
    cmdval_reg: OMAP4_PRM_VC_VAL_CMD_VDD_CORE_L_OFFSET,
    smps_sa_mask: OMAP4430_SA_VDD_CORE_L_0_6_MASK,
    smps_volra_mask: OMAP4430_VOLRA_VDD_CORE_L_MASK,
    smps_cmdra_mask: OMAP4430_CMDRA_VDD_CORE_L_MASK,
    cfg_channel_sa_shift: OMAP4430_SA_VDD_CORE_L_SHIFT,
    ..unsafe { core::mem::zeroed() }
};

// Voltage levels for different operating modes: on, sleep, retention and off
const OMAP4_ON_VOLTAGE_UV: i32 = 1375000;
const OMAP4_ONLP_VOLTAGE_UV: i32 = 1375000;
const OMAP4_RET_VOLTAGE_UV: i32 = 837500;
const OMAP4_OFF_VOLTAGE_UV: i32 = 0;

pub static mut omap4_mpu_vc_data: omap_vc_param = omap_vc_param {
    on: OMAP4_ON_VOLTAGE_UV,
    onlp: OMAP4_ONLP_VOLTAGE_UV,
    ret: OMAP4_RET_VOLTAGE_UV,
    off: OMAP4_OFF_VOLTAGE_UV,
    ..unsafe { core::mem::zeroed() }
};

pub static mut omap4_iva_vc_data: omap_vc_param = omap_vc_param {
    on: OMAP4_ON_VOLTAGE_UV,
    onlp: OMAP4_ONLP_VOLTAGE_UV,
    ret: OMAP4_RET_VOLTAGE_UV,
    off: OMAP4_OFF_VOLTAGE_UV,
    ..unsafe { core::mem::zeroed() }
};

pub static mut omap4_core_vc_data: omap_vc_param = omap_vc_param {
    on: OMAP4_ON_VOLTAGE_UV,
    onlp: OMAP4_ONLP_VOLTAGE_UV,
    ret: OMAP4_RET_VOLTAGE_UV,
    off: OMAP4_OFF_VOLTAGE_UV,
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
