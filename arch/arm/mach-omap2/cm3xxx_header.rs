/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 Clock Management (CM) register definitions
 *
 * Copyright (C) 2007-2009 Texas Instruments, Inc.
 * Copyright (C) 2007-2010 Nokia Corporation
 * Paul Walmsley
 *
 * The CM hardware modules on the OMAP2/3 are quite similar to each
 * other.  The CM modules/instances on OMAP4 are quite different, so
 * they are handled in a separate file.
 */

// Dependencies supplied by prcm-common.h and cm2xxx_3xxx.h are external.

macro_rules! OMAP34XX_CM_REGADDR {
    ($module:expr, $reg:expr) => {
        OMAP2_L4_IO_ADDRESS(OMAP3430_CM_BASE + ($module) + ($reg))
    };
}

/*
 * OMAP3-specific global CM registers
 * Use cm_{read,write}_reg() with these registers.
 * These registers appear once per CM module.
 */

pub const OMAP3430_CM_SYSCONFIG: u32 = 0x0010;
pub const OMAP3430_CM_POLCTRL: u32 = 0x009c;

pub const OMAP3_CM_CLKOUT_CTRL_OFFSET: u32 = 0x0070;
pub const OMAP3430_CM_CLKOUT_CTRL: u32 = OMAP_CM_REGADDR(OMAP3430_CCR_MOD, 0x0070);

/*
 * Module specific CM register offsets from CM_BASE + domain offset
 * Use cm_{read,write}_mod_reg() with these registers.
 * These register offsets generally appear in more than one PRCM submodule.
 */

/* OMAP3-specific register offsets */

pub const OMAP3430_CM_CLKEN_PLL: u32 = 0x0004;
pub const OMAP3430ES2_CM_CLKEN2: u32 = 0x0004;
pub const OMAP3430ES2_CM_FCLKEN3: u32 = 0x0008;
pub const OMAP3430_CM_IDLEST_PLL: u32 = CM_IDLEST2;
pub const OMAP3430_CM_AUTOIDLE_PLL: u32 = CM_AUTOIDLE2;
pub const OMAP3430ES2_CM_AUTOIDLE2_PLL: u32 = CM_AUTOIDLE2;
pub const OMAP3430_CM_CLKSEL1: u32 = CM_CLKSEL;
pub const OMAP3430_CM_CLKSEL1_PLL: u32 = CM_CLKSEL;
pub const OMAP3430_CM_CLKSEL2_PLL: u32 = CM_CLKSEL2;
pub const OMAP3430_CM_SLEEPDEP: u32 = CM_CLKSEL2;
pub const OMAP3430_CM_CLKSEL3: u32 = OMAP2_CM_CLKSTCTRL;
pub const OMAP3430_CM_CLKSTST: u32 = 0x004c;
pub const OMAP3430ES2_CM_CLKSEL4: u32 = 0x004c;
pub const OMAP3430ES2_CM_CLKSEL5: u32 = 0x0050;
pub const OMAP3430_CM_CLKSEL2_EMU: u32 = 0x0050;
pub const OMAP3430_CM_CLKSEL3_EMU: u32 = 0x0054;

/* CM_IDLEST bit field values to indicate deasserted IdleReq */

pub const OMAP34XX_CM_IDLEST_VAL: u32 = 1;

unsafe extern "C" {
    pub fn omap3_cm_save_context();
    pub fn omap3_cm_restore_context();
    pub fn omap3_cm_save_scratchpad_contents(ptr: *mut u32);

    // C declaration: int __init omap3xxx_cm_init(const struct omap_prcm_init_data *data);
    pub fn omap3xxx_cm_init(data: *const omap_prcm_init_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
