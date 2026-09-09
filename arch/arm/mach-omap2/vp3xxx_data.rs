// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 Voltage Processor (VP) data
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

// C dependencies: linux/io.h, linux/err.h, linux/init.h, common.h,
// prm-regbits-34xx.h, voltage.h, vp.h, and prm2xxx_3xxx.h.

static omap3_vp_ops: omap_vp_ops = omap_vp_ops {
    check_txdone: omap_prm_vp_check_txdone,
    clear_txdone: omap_prm_vp_clear_txdone,
};

/*
 * VP data common to 34xx/36xx chips
 * XXX This stuff presumably belongs in the vp3xxx.c or vp.c file.
 */
static omap3_vp_common: omap_vp_common = omap_vp_common {
    vpconfig_erroroffset_mask: OMAP3430_ERROROFFSET_MASK,
    vpconfig_errorgain_mask: OMAP3430_ERRORGAIN_MASK,
    vpconfig_initvoltage_mask: OMAP3430_INITVOLTAGE_MASK,
    vpconfig_timeouten: OMAP3430_TIMEOUTEN_MASK,
    vpconfig_initvdd: OMAP3430_INITVDD_MASK,
    vpconfig_forceupdate: OMAP3430_FORCEUPDATE_MASK,
    vpconfig_vpenable: OMAP3430_VPENABLE_MASK,
    vstepmin_smpswaittimemin_shift: OMAP3430_SMPSWAITTIMEMIN_SHIFT,
    vstepmax_smpswaittimemax_shift: OMAP3430_SMPSWAITTIMEMAX_SHIFT,
    vstepmin_stepmin_shift: OMAP3430_VSTEPMIN_SHIFT,
    vstepmax_stepmax_shift: OMAP3430_VSTEPMAX_SHIFT,
    vlimitto_vddmin_shift: OMAP3430_VDDMIN_SHIFT,
    vlimitto_vddmax_shift: OMAP3430_VDDMAX_SHIFT,
    vlimitto_timeout_shift: OMAP3430_TIMEOUT_SHIFT,
    vpvoltage_mask: OMAP3430_VPVOLTAGE_MASK,

    ops: &omap3_vp_ops,
};

pub static mut omap3_vp_mpu: omap_vp_instance = omap_vp_instance {
    id: OMAP3_VP_VDD_MPU_ID,
    common: &omap3_vp_common,
    vpconfig: OMAP3_PRM_VP1_CONFIG_OFFSET,
    vstepmin: OMAP3_PRM_VP1_VSTEPMIN_OFFSET,
    vstepmax: OMAP3_PRM_VP1_VSTEPMAX_OFFSET,
    vlimitto: OMAP3_PRM_VP1_VLIMITTO_OFFSET,
    vstatus: OMAP3_PRM_VP1_STATUS_OFFSET,
    voltage: OMAP3_PRM_VP1_VOLTAGE_OFFSET,
};

pub static mut omap3_vp_core: omap_vp_instance = omap_vp_instance {
    id: OMAP3_VP_VDD_CORE_ID,
    common: &omap3_vp_common,
    vpconfig: OMAP3_PRM_VP2_CONFIG_OFFSET,
    vstepmin: OMAP3_PRM_VP2_VSTEPMIN_OFFSET,
    vstepmax: OMAP3_PRM_VP2_VSTEPMAX_OFFSET,
    vlimitto: OMAP3_PRM_VP2_VLIMITTO_OFFSET,
    vstatus: OMAP3_PRM_VP2_STATUS_OFFSET,
    voltage: OMAP3_PRM_VP2_VOLTAGE_OFFSET,
};

pub static mut omap3_mpu_vp_data: omap_vp_param = omap_vp_param {
    vddmin: OMAP3430_VP1_VLIMITTO_VDDMIN,
    vddmax: OMAP3430_VP1_VLIMITTO_VDDMAX,
};

pub static mut omap3_core_vp_data: omap_vp_param = omap_vp_param {
    vddmin: OMAP3430_VP2_VLIMITTO_VDDMIN,
    vddmax: OMAP3430_VP2_VLIMITTO_VDDMAX,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
