// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common powerdomain framework functions
 *
 * Copyright (C) 2010-2011 Texas Instruments, Inc.
 * Copyright (C) 2010 Nokia Corporation
 *
 * Derived from mach-omap2/powerdomain.c written by Paul Walmsley
 */

// Dependencies supplied by other translation units:
// linux/errno.h, linux/kernel.h, linux/bug.h, pm.h, cm.h,
// cm-regbits-34xx.h, prm-regbits-34xx.h, prm-regbits-44xx.h

/*
 * OMAP3 and OMAP4 specific register bit initialisations
 * Notice that the names here are not according to each power
 * domain but the bit mapping used applies to all of them
 */
/* OMAP3 and OMAP4 Memory Onstate Masks (common across all power domains) */
const OMAP_MEM0_ONSTATE_MASK: u32 = OMAP3430_SHAREDL1CACHEFLATONSTATE_MASK;
const OMAP_MEM1_ONSTATE_MASK: u32 = OMAP3430_L1FLATMEMONSTATE_MASK;
const OMAP_MEM2_ONSTATE_MASK: u32 = OMAP3430_SHAREDL2CACHEFLATONSTATE_MASK;
const OMAP_MEM3_ONSTATE_MASK: u32 = OMAP3430_L2FLATMEMONSTATE_MASK;
const OMAP_MEM4_ONSTATE_MASK: u32 = OMAP4430_OCP_NRET_BANK_ONSTATE_MASK;

/* OMAP3 and OMAP4 Memory Retstate Masks (common across all power domains) */
const OMAP_MEM0_RETSTATE_MASK: u32 = OMAP3430_SHAREDL1CACHEFLATRETSTATE_MASK;
const OMAP_MEM1_RETSTATE_MASK: u32 = OMAP3430_L1FLATMEMRETSTATE_MASK;
const OMAP_MEM2_RETSTATE_MASK: u32 = OMAP3430_SHAREDL2CACHEFLATRETSTATE_MASK;
const OMAP_MEM3_RETSTATE_MASK: u32 = OMAP3430_L2FLATMEMRETSTATE_MASK;
const OMAP_MEM4_RETSTATE_MASK: u32 = OMAP4430_OCP_NRET_BANK_RETSTATE_MASK;

/* OMAP3 and OMAP4 Memory Status bits */
const OMAP_MEM0_STATEST_MASK: u32 = OMAP3430_SHAREDL1CACHEFLATSTATEST_MASK;
const OMAP_MEM1_STATEST_MASK: u32 = OMAP3430_L1FLATMEMSTATEST_MASK;
const OMAP_MEM2_STATEST_MASK: u32 = OMAP3430_SHAREDL2CACHEFLATSTATEST_MASK;
const OMAP_MEM3_STATEST_MASK: u32 = OMAP3430_L2FLATMEMSTATEST_MASK;
const OMAP_MEM4_STATEST_MASK: u32 = OMAP4430_OCP_NRET_BANK_STATEST_MASK;

/* Common Internal functions used across OMAP rev's*/
pub fn omap2_pwrdm_get_mem_bank_onstate_mask(bank: u8) -> u32 {
    match bank {
        0 => OMAP_MEM0_ONSTATE_MASK,
        1 => OMAP_MEM1_ONSTATE_MASK,
        2 => OMAP_MEM2_ONSTATE_MASK,
        3 => OMAP_MEM3_ONSTATE_MASK,
        4 => OMAP_MEM4_ONSTATE_MASK,
        _ => {
            warn_on(1); /* should never happen */
            -(EEXIST as i32) as u32
        }
    }
}

pub fn omap2_pwrdm_get_mem_bank_retst_mask(bank: u8) -> u32 {
    match bank {
        0 => OMAP_MEM0_RETSTATE_MASK,
        1 => OMAP_MEM1_RETSTATE_MASK,
        2 => OMAP_MEM2_RETSTATE_MASK,
        3 => OMAP_MEM3_RETSTATE_MASK,
        4 => OMAP_MEM4_RETSTATE_MASK,
        _ => {
            warn_on(1); /* should never happen */
            -(EEXIST as i32) as u32
        }
    }
}

pub fn omap2_pwrdm_get_mem_bank_stst_mask(bank: u8) -> u32 {
    match bank {
        0 => OMAP_MEM0_STATEST_MASK,
        1 => OMAP_MEM1_STATEST_MASK,
        2 => OMAP_MEM2_STATEST_MASK,
        3 => OMAP_MEM3_STATEST_MASK,
        4 => OMAP_MEM4_STATEST_MASK,
        _ => {
            warn_on(1); /* should never happen */
            -(EEXIST as i32) as u32
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
