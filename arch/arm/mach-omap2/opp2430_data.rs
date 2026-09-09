// SPDX-License-Identifier: GPL-2.0
/*
 * opp2430_data.c - old-style "OPP" table for OMAP2430
 *
 * Copyright (C) 2005-2009 Texas Instruments, Inc.
 * Copyright (C) 2004-2009 Nokia Corporation
 *
 * Richard Woodruff <r-woodruff2@ti.com>
 *
 * The OMAP2 processor can be run at several discrete 'PRCM configurations'.
 * These configurations are characterized by voltage and speed for clocks.
 * The device is only validated for certain combinations. One way to express
 * these combinations is via the 'ratios' which the clocks operate with
 * respect to each other. These ratio sets are for a given voltage/DPLL
 * setting. All configurations can be described by a DPLL setting and a ratio.
 *
 * 2430 differs from 2420 in that there are no more phase synchronizers used.
 * They both have a slightly different clock domain setup. 2420(iva1,dsp) vs
 * 2430 (iva2.1, NOdsp, mdm)
 *
 * XXX Missing voltage data.
 * XXX Missing 19.2MHz sys_clk rate sets.
 *
 * THe format described in this file is deprecated.  Once a reasonable
 * OPP API exists, the data in this file should be converted to use it.
 *
 * This is technically part of the OMAP2xxx clock code.
 */

/* Dependencies supplied by the surrounding OMAP2xxx translation unit. */

/*
 * Key dividers which make up a PRCM set. Ratios for a PRCM are mandated.
 * xtal_speed, dpll_speed, mpu_speed, CM_CLKSEL_MPU,
 * CM_CLKSEL_DSP, CM_CLKSEL_GFX, CM_CLKSEL1_CORE, CM_CLKSEL1_PLL,
 * CM_CLKSEL2_PLL, CM_CLKSEL_MDM
 *
 * Filling in table based on 2430-SDPs variants available.  There are
 * quite a few more rate combinations which could be defined.
 *
 * When multiple values are defined the start up will try and choose
 * the fastest one. If a 'fast' value is defined, then automatically,
 * the /2 one should be included as it can be used.  Generally having
 * more than one fast set does not make sense, as static timings need
 * to be changed to change the set.  The exception is the bypass
 * setting which is available for low power bypass.
 *
 * Note: This table needs to be sorted, fastest to slowest.
 */
pub static omap2430_rate_table: [prcm_config; 11] = [
    /* PRCM #4 - ratio2 (ES2.1) - FAST */
    prcm_config { xtal_speed: S13M, dpll_speed: S798M, mpu_speed: S399M, cm_clksel_mpu: R2_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R2_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R2_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R2_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M4_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: R2_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz, flags: RATE_IN_243X },

    /* PRCM #2 - ratio1 (ES2) - FAST */
    prcm_config { xtal_speed: S13M, dpll_speed: S658M, mpu_speed: S329M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M2_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_165MHz, flags: RATE_IN_243X },

    /* PRCM #5a - ratio1 - FAST */
    prcm_config { xtal_speed: S13M, dpll_speed: S532M, mpu_speed: S266M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M5A_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz, flags: RATE_IN_243X },

    /* PRCM #5b - ratio1 - FAST */
    prcm_config { xtal_speed: S13M, dpll_speed: S400M, mpu_speed: S200M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M5B_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz, flags: RATE_IN_243X },

    /* PRCM #4 - ratio1 (ES2.1) - SLOW */
    prcm_config { xtal_speed: S13M, dpll_speed: S399M, mpu_speed: S199M, cm_clksel_mpu: R2_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R2_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R2_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R2_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M4_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_1x_VAL, cm_clksel_mdm: R2_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz, flags: RATE_IN_243X },

    /* PRCM #2 - ratio1 (ES2) - SLOW */
    prcm_config { xtal_speed: S13M, dpll_speed: S329M, mpu_speed: S164M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M2_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_1x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_165MHz, flags: RATE_IN_243X },

    /* PRCM #5a - ratio1 - SLOW */
    prcm_config { xtal_speed: S13M, dpll_speed: S266M, mpu_speed: S133M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M5A_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_1x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz, flags: RATE_IN_243X },

    /* PRCM #5b - ratio1 - SLOW*/
    prcm_config { xtal_speed: S13M, dpll_speed: S200M, mpu_speed: S100M, cm_clksel_mpu: R1_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: R1_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: R1_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: R1_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: M5B_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_1x_VAL, cm_clksel_mdm: R1_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz, flags: RATE_IN_243X },

    /* PRCM-boot/bypass */
    prcm_config { xtal_speed: S13M, dpll_speed: S13M, mpu_speed: S13M, cm_clksel_mpu: RB_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RB_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RB_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RB_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MB_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: RB_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_BYPASS, flags: RATE_IN_243X },

    /* PRCM-boot/bypass */
    prcm_config { xtal_speed: S12M, dpll_speed: S12M, mpu_speed: S12M, cm_clksel_mpu: RB_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RB_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RB_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RB_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MB_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: RB_CM_CLKSEL_MDM_VAL,
        sdrc_rfr_ctrl: SDRC_RFR_CTRL_BYPASS, flags: RATE_IN_243X },

    prcm_config { xtal_speed: 0, dpll_speed: 0, mpu_speed: 0, cm_clksel_mpu: 0,
        cm_clksel_dsp: 0, cm_clksel_gfx: 0, cm_clksel1_core: 0, cm_clksel1_pll: 0,
        cm_clksel2_pll: 0, cm_clksel_mdm: 0, sdrc_rfr_ctrl: 0, flags: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
