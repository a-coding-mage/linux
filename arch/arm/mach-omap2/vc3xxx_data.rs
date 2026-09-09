// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 Voltage Controller (VC) data
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

// Dependencies supplied by the surrounding translation unit:
// common.h, prm-regbits-34xx.h, voltage.h, and vc.h

/*
 * VC data common to 34xx/36xx chips
 * XXX This stuff presumably belongs in the vc3xxx.c or vc.c file.
 */
static mut omap3_vc_common: omap_vc_common = omap_vc_common {
    bypass_val_reg: OMAP3_PRM_VC_BYPASS_VAL_OFFSET,
    data_shift: OMAP3430_DATA_SHIFT,
    slaveaddr_shift: OMAP3430_SLAVEADDR_SHIFT,
    regaddr_shift: OMAP3430_REGADDR_SHIFT,
    valid: OMAP3430_VALID_MASK,
    cmd_on_shift: OMAP3430_VC_CMD_ON_SHIFT,
    cmd_on_mask: OMAP3430_VC_CMD_ON_MASK,
    cmd_onlp_shift: OMAP3430_VC_CMD_ONLP_SHIFT,
    cmd_ret_shift: OMAP3430_VC_CMD_RET_SHIFT,
    cmd_off_shift: OMAP3430_VC_CMD_OFF_SHIFT,
    i2c_cfg_clear_mask: OMAP3430_SREN_MASK | OMAP3430_HSEN_MASK,
    i2c_cfg_hsen_mask: OMAP3430_HSEN_MASK,
    i2c_cfg_reg: OMAP3_PRM_VC_I2C_CFG_OFFSET,
    i2c_mcode_mask: OMAP3430_MCODE_MASK,
};

pub static mut omap3_vc_mpu: omap_vc_channel = omap_vc_channel {
    flags: OMAP_VC_CHANNEL_DEFAULT,
    common: unsafe { &mut omap3_vc_common },
    smps_sa_reg: OMAP3_PRM_VC_SMPS_SA_OFFSET,
    smps_volra_reg: OMAP3_PRM_VC_SMPS_VOL_RA_OFFSET,
    smps_cmdra_reg: OMAP3_PRM_VC_SMPS_CMD_RA_OFFSET,
    cfg_channel_reg: OMAP3_PRM_VC_CH_CONF_OFFSET,
    cmdval_reg: OMAP3_PRM_VC_CMD_VAL_0_OFFSET,
    smps_sa_mask: OMAP3430_PRM_VC_SMPS_SA_SA0_MASK,
    smps_volra_mask: OMAP3430_VOLRA0_MASK,
    smps_cmdra_mask: OMAP3430_CMDRA0_MASK,
    cfg_channel_sa_shift: OMAP3430_PRM_VC_SMPS_SA_SA0_SHIFT,
};

pub static mut omap3_vc_core: omap_vc_channel = omap_vc_channel {
    common: unsafe { &mut omap3_vc_common },
    smps_sa_reg: OMAP3_PRM_VC_SMPS_SA_OFFSET,
    smps_volra_reg: OMAP3_PRM_VC_SMPS_VOL_RA_OFFSET,
    smps_cmdra_reg: OMAP3_PRM_VC_SMPS_CMD_RA_OFFSET,
    cfg_channel_reg: OMAP3_PRM_VC_CH_CONF_OFFSET,
    cmdval_reg: OMAP3_PRM_VC_CMD_VAL_1_OFFSET,
    smps_sa_mask: OMAP3430_PRM_VC_SMPS_SA_SA1_MASK,
    smps_volra_mask: OMAP3430_VOLRA1_MASK,
    smps_cmdra_mask: OMAP3430_CMDRA1_MASK,
    cfg_channel_sa_shift: OMAP3430_PRM_VC_SMPS_SA_SA1_SHIFT,
};

/* Voltage levels for different operating modes: on, sleep, retention and off */
const OMAP3_ON_VOLTAGE_UV: u32 = 1200000;
const OMAP3_ONLP_VOLTAGE_UV: u32 = 1000000;
const OMAP3_RET_VOLTAGE_UV: u32 = 975000;
const OMAP3_OFF_VOLTAGE_UV: u32 = 600000;

pub static mut omap3_mpu_vc_data: omap_vc_param = omap_vc_param {
    on: OMAP3_ON_VOLTAGE_UV,
    onlp: OMAP3_ONLP_VOLTAGE_UV,
    ret: OMAP3_RET_VOLTAGE_UV,
    off: OMAP3_OFF_VOLTAGE_UV,
};

pub static mut omap3_core_vc_data: omap_vc_param = omap_vc_param {
    on: OMAP3_ON_VOLTAGE_UV,
    onlp: OMAP3_ONLP_VOLTAGE_UV,
    ret: OMAP3_RET_VOLTAGE_UV,
    off: OMAP3_OFF_VOLTAGE_UV,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
