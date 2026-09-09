/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP3/4 Voltage Processor (VP) structure and macro definitions
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

// Dependency supplied by the surrounding translation unit: linux/kernel.h.

// XXX document
pub const VP_IDLE_TIMEOUT: u32 = 200;
pub const VP_TRANXDONE_TIMEOUT: u32 = 300;

#[repr(C)]
pub struct voltagedomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_vp_param {
    _private: [u8; 0],
}

/**
 * struct omap_vp_ops - per-VP operations
 * @check_txdone: check for VP transaction done
 * @clear_txdone: clear VP transaction done status
 */
#[repr(C)]
pub struct omap_vp_ops {
    pub check_txdone: Option<unsafe extern "C" fn(vp_id: u8) -> u32>,
    pub clear_txdone: Option<unsafe extern "C" fn(vp_id: u8)>,
}

/**
 * struct omap_vp_common - register data common to all VDDs
 * @vpconfig_erroroffset_mask: ERROROFFSET bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_errorgain_mask: ERRORGAIN bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_initvoltage_mask: INITVOLTAGE bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_timeouten: TIMEOUT bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_initvdd: INITVDD bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_forceupdate: FORCEUPDATE bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_vpenable: VPENABLE bitmask in the PRM_VP*_CONFIG reg
 * @vpconfig_erroroffset_shift: ERROROFFSET field shift in PRM_VP*_CONFIG reg
 * @vpconfig_errorgain_shift: ERRORGAIN field shift in PRM_VP*_CONFIG reg
 * @vpconfig_initvoltage_shift: INITVOLTAGE field shift in PRM_VP*_CONFIG reg
 * @vstepmin_stepmin_shift: VSTEPMIN field shift in the PRM_VP*_VSTEPMIN reg
 * @vstepmin_smpswaittimemin_shift: SMPSWAITTIMEMIN field shift in the PRM_VP*_VSTEPMIN reg
 * @vstepmax_stepmax_shift: VSTEPMAX field shift in the PRM_VP*_VSTEPMAX reg
 * @vstepmax_smpswaittimemax_shift: SMPSWAITTIMEMAX field shift in PRM_VP*_VSTEPMAX reg
 * @vlimitto_vddmin_shift: VDDMIN field shift in PRM_VP*_VLIMITTO reg
 * @vlimitto_vddmax_shift: VDDMAX field shift in PRM_VP*_VLIMITTO reg
 * @vlimitto_timeout_shift: TIMEOUT field shift in PRM_VP*_VLIMITTO reg
 * @vpvoltage_mask: VPVOLTAGE field mask in PRM_VP*_VOLTAGE reg
 */
#[repr(C)]
pub struct omap_vp_common {
    pub vpconfig_erroroffset_mask: u32,
    pub vpconfig_errorgain_mask: u32,
    pub vpconfig_initvoltage_mask: u32,
    pub vpconfig_timeouten: u8,
    pub vpconfig_initvdd: u8,
    pub vpconfig_forceupdate: u8,
    pub vpconfig_vpenable: u8,
    pub vstepmin_stepmin_shift: u8,
    pub vstepmin_smpswaittimemin_shift: u8,
    pub vstepmax_stepmax_shift: u8,
    pub vstepmax_smpswaittimemax_shift: u8,
    pub vlimitto_vddmin_shift: u8,
    pub vlimitto_vddmax_shift: u8,
    pub vlimitto_timeout_shift: u8,
    pub vpvoltage_mask: u8,
    pub ops: *const omap_vp_ops,
}

/**
 * struct omap_vp_instance - VP register offsets (per-VDD)
 * @common: pointer to struct omap_vp_common * for this SoC
 * @vpconfig: PRM_VP*_CONFIG reg offset from PRM start
 * @vstepmin: PRM_VP*_VSTEPMIN reg offset from PRM start
 * @vlimitto: PRM_VP*_VLIMITTO reg offset from PRM start
 * @vstatus: PRM_VP*_VSTATUS reg offset from PRM start
 * @voltage: PRM_VP*_VOLTAGE reg offset from PRM start
 * @id: Unique identifier for VP instance.
 * @enabled: flag to keep track of whether vp is enabled or not
 *
 * XXX vp_common is probably not needed since it is per-SoC
 */
#[repr(C)]
pub struct omap_vp_instance {
    pub common: *const omap_vp_common,
    pub vpconfig: u8,
    pub vstepmin: u8,
    pub vstepmax: u8,
    pub vlimitto: u8,
    pub vstatus: u8,
    pub voltage: u8,
    pub id: u8,
    pub enabled: bool,
}

extern "C" {
    pub static mut omap3_vp_mpu: omap_vp_instance;
    pub static mut omap3_vp_core: omap_vp_instance;
    pub static mut omap4_vp_mpu: omap_vp_instance;
    pub static mut omap4_vp_iva: omap_vp_instance;
    pub static mut omap4_vp_core: omap_vp_instance;

    pub static mut omap3_mpu_vp_data: omap_vp_param;
    pub static mut omap3_core_vp_data: omap_vp_param;
    pub static mut omap4_mpu_vp_data: omap_vp_param;
    pub static mut omap4_iva_vp_data: omap_vp_param;
    pub static mut omap4_core_vp_data: omap_vp_param;

    pub fn omap_vp_init(voltdm: *mut voltagedomain);
    pub fn omap_vp_enable(voltdm: *mut voltagedomain);
    pub fn omap_vp_disable(voltdm: *mut voltagedomain);
    pub fn omap_vp_forceupdate_scale(
        voltdm: *mut voltagedomain,
        target_volt: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn omap_vp_update_errorgain(
        voltdm: *mut voltagedomain,
        target_volt: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
