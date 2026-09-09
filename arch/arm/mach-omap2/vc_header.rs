/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP3/4 Voltage Controller (VC) structure and macro definitions
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

// Dependency supplied by the surrounding translation unit:
// struct voltagedomain;

/**
 * struct omap_vc_common - per-VC register/bitfield data
 * @cmd_on_mask: ON bitmask in PRM_VC_CMD_VAL* register
 * @valid: VALID bitmask in PRM_VC_BYPASS_VAL register
 * @bypass_val_reg: Offset of PRM_VC_BYPASS_VAL reg from PRM start
 * @data_shift: DATA field shift in PRM_VC_BYPASS_VAL register
 * @slaveaddr_shift: SLAVEADDR field shift in PRM_VC_BYPASS_VAL register
 * @regaddr_shift: REGADDR field shift in PRM_VC_BYPASS_VAL register
 * @cmd_on_shift: ON field shift in PRM_VC_CMD_VAL_* register
 * @cmd_onlp_shift: ONLP field shift in PRM_VC_CMD_VAL_* register
 * @cmd_ret_shift: RET field shift in PRM_VC_CMD_VAL_* register
 * @cmd_off_shift: OFF field shift in PRM_VC_CMD_VAL_* register
 * @i2c_cfg_reg: I2C configuration register offset
 * @i2c_cfg_clear_mask: high-speed mode bit clear mask in I2C config register
 * @i2c_cfg_hsen_mask: high-speed mode bit field mask in I2C config register
 * @i2c_mcode_mask: MCODE field mask for I2C config register
 *
 * XXX One of cmd_on_mask and cmd_on_shift are not needed
 * XXX VALID should probably be a shift, not a mask
 */
#[repr(C)]
pub struct omap_vc_common {
    pub cmd_on_mask: u32,
    pub valid: u32,
    pub bypass_val_reg: u8,
    pub data_shift: u8,
    pub slaveaddr_shift: u8,
    pub regaddr_shift: u8,
    pub cmd_on_shift: u8,
    pub cmd_onlp_shift: u8,
    pub cmd_ret_shift: u8,
    pub cmd_off_shift: u8,
    pub i2c_cfg_reg: u8,
    pub i2c_cfg_clear_mask: u8,
    pub i2c_cfg_hsen_mask: u8,
    pub i2c_mcode_mask: u8,
}

/* omap_vc_channel.flags values */
pub const OMAP_VC_CHANNEL_DEFAULT: u32 = 1u32 << 0;
pub const OMAP_VC_CHANNEL_CFG_MUTANT: u32 = 1u32 << 1;

/**
 * struct omap_vc_channel - VC per-instance data
 * @i2c_slave_addr: I2C slave address of PMIC for this VC channel
 * @volt_reg_addr: voltage configuration register address
 * @cmd_reg_addr: command configuration register address
 * @setup_time: setup time (in sys_clk cycles) of regulator for this channel
 * @cfg_channel: current value of VC channel configuration register
 * @i2c_high_speed: whether or not to use I2C high-speed mode
 *
 * @common: pointer to VC common data for this platform
 * @smps_sa_mask: i2c slave address bitmask in the PRM_VC_SMPS_SA register
 * @smps_volra_mask: VOLRA* bitmask in the PRM_VC_VOL_RA register
 * @smps_cmdra_mask: CMDRA* bitmask in the PRM_VC_CMD_RA register
 * @cmdval_reg: register for on/ret/off voltage level values for this channel
 * @smps_sa_reg: Offset of PRM_VC_SMPS_SA reg from PRM start
 * @smps_volra_reg: Offset of PRM_VC_SMPS_VOL_RA reg from PRM start
 * @smps_cmdra_reg: Offset of PRM_VC_SMPS_CMD_RA reg from PRM start
 * @cfg_channel_reg: VC channel configuration register
 * @cfg_channel_sa_shift: bit shift for slave address cfg_channel register
 * @flags: VC channel-specific flags (optional)
 */
#[repr(C)]
pub struct omap_vc_channel {
    /* channel state */
    pub i2c_slave_addr: u16,
    pub volt_reg_addr: u16,
    pub cmd_reg_addr: u16,
    pub cfg_channel: u8,
    pub i2c_high_speed: bool,

    /* register access data */
    pub common: *const omap_vc_common,
    pub smps_sa_mask: u32,
    pub smps_volra_mask: u32,
    pub smps_cmdra_mask: u32,
    pub cmdval_reg: u8,
    pub smps_sa_reg: u8,
    pub smps_volra_reg: u8,
    pub smps_cmdra_reg: u8,
    pub cfg_channel_reg: u8,
    pub cfg_channel_sa_shift: u8,
    pub flags: u8,
}

// omap_vc_param is supplied by another translated dependency.
extern "C" {
    pub static mut omap3_vc_mpu: omap_vc_channel;
    pub static mut omap3_vc_core: omap_vc_channel;
    pub static mut omap4_vc_mpu: omap_vc_channel;
    pub static mut omap4_vc_iva: omap_vc_channel;
    pub static mut omap4_vc_core: omap_vc_channel;

    pub static mut omap3_mpu_vc_data: omap_vc_param;
    pub static mut omap3_core_vc_data: omap_vc_param;
    pub static mut omap4_mpu_vc_data: omap_vc_param;
    pub static mut omap4_iva_vc_data: omap_vc_param;
    pub static mut omap4_core_vc_data: omap_vc_param;

    pub fn omap3_vc_set_pmic_signaling(core_next_state: core::ffi::c_int);
    pub fn omap4_vc_set_pmic_signaling(core_next_state: core::ffi::c_int);
    pub fn omap_vc_init_channel(voltdm: *mut voltagedomain);
    pub fn omap_vc_pre_scale(
        voltdm: *mut voltagedomain,
        target_volt: core::ffi::c_ulong,
        target_vsel: *mut u8,
        current_vsel: *mut u8,
    ) -> core::ffi::c_int;
    pub fn omap_vc_post_scale(
        voltdm: *mut voltagedomain,
        target_volt: core::ffi::c_ulong,
        target_vsel: u8,
        current_vsel: u8,
    );
    pub fn omap_vc_bypass_scale(
        voltdm: *mut voltagedomain,
        target_volt: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
