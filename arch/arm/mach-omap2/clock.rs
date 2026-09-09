// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-omap2/clock.c
 *
 *  Copyright (C) 2005-2008 Texas Instruments, Inc.
 *  Copyright (C) 2004-2010 Nokia Corporation
 *
 *  Contacts:
 *  Richard Woodruff <r-woodruff2@ti.com>
 *  Paul Walmsley
 */
// DEBUG is undefined in the original source.

// Dependencies supplied by the surrounding kernel translation.

/* DPLL valid Fint frequency band limits - from 34xx TRM Section 4.7.6.2 */
const OMAP3430_DPLL_FINT_BAND1_MIN: u32 = 750000;
const OMAP3430_DPLL_FINT_BAND1_MAX: u32 = 2100000;
const OMAP3430_DPLL_FINT_BAND2_MIN: u32 = 7500000;
const OMAP3430_DPLL_FINT_BAND2_MAX: u32 = 21000000;

/*
 * DPLL valid Fint frequency range for OMAP36xx and OMAP4xxx.
 * From device data manual section 4.3 "DPLL and DLL Specifications".
 */
const OMAP3PLUS_DPLL_FINT_MIN: u32 = 32000;
const OMAP3PLUS_DPLL_FINT_MAX: u32 = 52000000;

static mut omap_clk_ll_ops: ti_clk_ll_ops = ti_clk_ll_ops {
    clkdm_clk_enable: clkdm_clk_enable,
    clkdm_clk_disable: clkdm_clk_disable,
    clkdm_lookup: clkdm_lookup,
    cm_wait_module_ready: omap_cm_wait_module_ready,
    cm_split_idlest_reg: cm_split_idlest_reg,
};

/**
 * omap2_clk_setup_ll_ops - setup clock driver low-level ops
 *
 * Sets up clock driver low-level platform ops. These are needed
 * for register accesses and various other misc platform operations.
 * Returns 0 on success, -EBUSY if low level ops have been registered
 * already.
 */
pub unsafe fn omap2_clk_setup_ll_ops() -> i32 {
    ti_clk_setup_ll_ops(&raw const omap_clk_ll_ops)
}

/*
 * OMAP2+ specific clock functions
 */

/**
 * ti_clk_init_features - init clock features struct for the SoC
 *
 * Initializes the clock features struct based on the SoC type.
 */
pub unsafe fn ti_clk_init_features() {
    let mut features: ti_clk_features = core::mem::zeroed();

    /* Fint setup for DPLLs */
    if cpu_is_omap3430() {
        features.fint_min = OMAP3430_DPLL_FINT_BAND1_MIN;
        features.fint_max = OMAP3430_DPLL_FINT_BAND2_MAX;
        features.fint_band1_max = OMAP3430_DPLL_FINT_BAND1_MAX;
        features.fint_band2_min = OMAP3430_DPLL_FINT_BAND2_MIN;
    } else {
        features.fint_min = OMAP3PLUS_DPLL_FINT_MIN;
        features.fint_max = OMAP3PLUS_DPLL_FINT_MAX;
    }

    /* Bypass value setup for DPLLs */
    if cpu_is_omap24xx() {
        features.dpll_bypass_vals |=
            (1 << OMAP2XXX_EN_DPLL_LPBYPASS) |
            (1 << OMAP2XXX_EN_DPLL_FRBYPASS);
    } else if cpu_is_omap34xx() {
        features.dpll_bypass_vals |=
            (1 << OMAP3XXX_EN_DPLL_LPBYPASS) |
            (1 << OMAP3XXX_EN_DPLL_FRBYPASS);
    } else if soc_is_am33xx() || cpu_is_omap44xx() || soc_is_am43xx() ||
              soc_is_omap54xx() || soc_is_dra7xx() {
        features.dpll_bypass_vals |=
            (1 << OMAP4XXX_EN_DPLL_LPBYPASS) |
            (1 << OMAP4XXX_EN_DPLL_FRBYPASS) |
            (1 << OMAP4XXX_EN_DPLL_MNBYPASS);
    }

    /* Jitter correction only available on OMAP343X */
    if cpu_is_omap343x() {
        features.flags |= TI_CLK_DPLL_HAS_FREQSEL;
    }

    if omap_type() == OMAP2_DEVICE_TYPE_GP {
        features.flags |= TI_CLK_DEVICE_TYPE_GP;
    }

    /* Idlest value for interface clocks.
     * 24xx uses 0 to indicate not ready, and 1 to indicate ready.
     * 34xx reverses this, just to keep us on our toes
     * AM35xx uses both, depending on the module.
     */
    if cpu_is_omap24xx() {
        features.cm_idlest_val = OMAP24XX_CM_IDLEST_VAL;
    } else if cpu_is_omap34xx() {
        features.cm_idlest_val = OMAP34XX_CM_IDLEST_VAL;
    }

    /* On OMAP3430 ES1.0, DPLL4 can't be re-programmed */
    if omap_rev() == OMAP3430_REV_ES1_0 {
        features.flags |= TI_CLK_DPLL4_DENY_REPROGRAM;
    }

    /* Errata I810 for omap5 / dra7 */
    if soc_is_omap54xx() || soc_is_dra7xx() {
        features.flags |= TI_CLK_ERRATA_I810;
    }

    ti_clk_setup_features(&features);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
