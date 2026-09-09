// SPDX-License-Identifier: GPL-2.0
/*
 * opp2420_data.c - old-style "OPP" table for OMAP2420
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
 * XXX Missing voltage data.
 * XXX Missing 19.2MHz sys_clk rate sets (needed for N800/N810)
 *
 * THe format described in this file is deprecated.  Once a reasonable
 * OPP API exists, the data in this file should be converted to use it.
 *
 * This is technically part of the OMAP2xxx clock code.
 *
 * Considerable work is still needed to fully support dynamic frequency
 * changes on OMAP2xxx-series chips.  Readers interested in such a
 * project are encouraged to review the Maemo Diablo RX-34 and RX-44
 * kernel source at:
 *     http://repository.maemo.org/pool/diablo/free/k/kernel-source-diablo/
 */

// Dependencies supplied by the surrounding OMAP implementation:
// opp2xxx.h, sdrc.h, clock.h, and the Linux kernel definitions.

/*
 * Key dividers which make up a PRCM set. Ratios for a PRCM are mandated.
 * xtal_speed, dpll_speed, mpu_speed, CM_CLKSEL_MPU,
 * CM_CLKSEL_DSP, CM_CLKSEL_GFX, CM_CLKSEL1_CORE, CM_CLKSEL1_PLL,
 * CM_CLKSEL2_PLL, CM_CLKSEL_MDM
 *
 * Filling in table based on H4 boards available.  There are quite a
 * few more rate combinations which could be defined.
 *
 * When multiple values are defined the start up will try and choose
 * the fastest one. If a 'fast' value is defined, then automatically,
 * the /2 one should be included as it can be used.  Generally having
 * more than one fast set does not make sense, as static timings need
 * to be changed to change the set.  The exception is the bypass
 * setting which is available for low power bypass.
 *
 * Note: This table needs to be sorted, fastest to slowest.
 **/
pub static omap2420_rate_table: [prcm_config; 12] = [
    /* PRCM I - FAST */
    prcm_config { xtal_speed: S12M, dpll_speed: S660M, mpu_speed: S330M, cm_clksel_mpu: RI_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RI_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RI_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RI_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MI_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_165MHz,
        flags: RATE_IN_242X },

    /* PRCM II - FAST */
    prcm_config { xtal_speed: S12M, dpll_speed: S600M, mpu_speed: S300M, cm_clksel_mpu: RII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MII_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz,
        flags: RATE_IN_242X },
    prcm_config { xtal_speed: S13M, dpll_speed: S600M, mpu_speed: S300M, cm_clksel_mpu: RII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MII_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz,
        flags: RATE_IN_242X },

    /* PRCM III - FAST */
    prcm_config { xtal_speed: S12M, dpll_speed: S532M, mpu_speed: S266M, cm_clksel_mpu: RIII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RIII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RIII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RIII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MIII_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz,
        flags: RATE_IN_242X },
    prcm_config { xtal_speed: S13M, dpll_speed: S532M, mpu_speed: S266M, cm_clksel_mpu: RIII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RIII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RIII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RIII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MIII_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz,
        flags: RATE_IN_242X },

    /* PRCM II - SLOW */
    prcm_config { xtal_speed: S12M, dpll_speed: S300M, mpu_speed: S150M, cm_clksel_mpu: RII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MII_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz,
        flags: RATE_IN_242X },
    prcm_config { xtal_speed: S13M, dpll_speed: S300M, mpu_speed: S150M, cm_clksel_mpu: RII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MII_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_100MHz,
        flags: RATE_IN_242X },

    /* PRCM III - SLOW */
    prcm_config { xtal_speed: S12M, dpll_speed: S266M, mpu_speed: S133M, cm_clksel_mpu: RIII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RIII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RIII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RIII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MIII_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz,
        flags: RATE_IN_242X },
    prcm_config { xtal_speed: S13M, dpll_speed: S266M, mpu_speed: S133M, cm_clksel_mpu: RIII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RIII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RIII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RIII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MIII_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_133MHz,
        flags: RATE_IN_242X },

    /* PRCM-VII (boot-bypass) */
    prcm_config { xtal_speed: S12M, dpll_speed: S12M, mpu_speed: S12M, cm_clksel_mpu: RVII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RVII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RVII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RVII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MVII_CM_CLKSEL1_PLL_12_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_BYPASS,
        flags: RATE_IN_242X },
    prcm_config { xtal_speed: S13M, dpll_speed: S13M, mpu_speed: S13M, cm_clksel_mpu: RVII_CM_CLKSEL_MPU_VAL,
        cm_clksel_dsp: RVII_CM_CLKSEL_DSP_VAL, cm_clksel_gfx: RVII_CM_CLKSEL_GFX_VAL,
        cm_clksel1_core: RVII_CM_CLKSEL1_CORE_VAL, cm_clksel1_pll: MVII_CM_CLKSEL1_PLL_13_VAL,
        cm_clksel2_pll: MX_CLKSEL2_PLL_2x_VAL, cm_clksel_mdm: 0, sdrc_rfr_ctrl: SDRC_RFR_CTRL_BYPASS,
        flags: RATE_IN_242X },

    prcm_config { xtal_speed: 0, dpll_speed: 0, mpu_speed: 0, cm_clksel_mpu: 0,
        cm_clksel_dsp: 0, cm_clksel_gfx: 0, cm_clksel1_core: 0, cm_clksel1_pll: 0,
        cm_clksel2_pll: 0, cm_clksel_mdm: 0, sdrc_rfr_ctrl: 0, flags: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
