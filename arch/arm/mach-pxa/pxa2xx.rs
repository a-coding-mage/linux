// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-pxa/pxa2xx.c
 *
 * code specific to pxa2xx
 *
 * Copyright (C) 2008 Dmitry Baryshkov
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn pxa2xx_clear_reset_status(mask: core::ffi::c_uint) {
    /* RESET_STATUS_* has a 1:1 mapping with RCSR */
    RCSR = mask;
}

#[inline]
fn mdcnfg_drac2(mdcnfg: u32) -> u32 {
    (mdcnfg >> 21) & 0x3
}

#[inline]
fn mdcnfg_drac0(mdcnfg: u32) -> u32 {
    (mdcnfg >> 5) & 0x3
}

static mut SDRAM_ROWS: core::ffi::c_int = 0;

pub unsafe fn pxa2xx_smemc_get_sdram_rows() -> core::ffi::c_int {
    let mut drac2: u32 = 0;
    let mut drac0: u32 = 0;
    let mdcnfg: u32;

    if SDRAM_ROWS != 0 {
        return SDRAM_ROWS;
    }

    mdcnfg = readl_relaxed(MDCNFG);

    if (mdcnfg & (MDCNFG_DE2 | MDCNFG_DE3)) != 0 {
        drac2 = mdcnfg_drac2(mdcnfg);
    }

    if (mdcnfg & (MDCNFG_DE0 | MDCNFG_DE1)) != 0 {
        drac0 = mdcnfg_drac0(mdcnfg);
    }

    SDRAM_ROWS = 1i32 << (11 + core::cmp::max(drac0, drac2));
    SDRAM_ROWS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
