/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2xxx Clock Management (CM) register definitions
 *
 * Copyright (C) 2007-2009, 2012 Texas Instruments, Inc.
 * Copyright (C) 2007-2010 Nokia Corporation
 * Paul Walmsley
 *
 * The CM hardware modules on the OMAP2/3 are quite similar to each
 * other.  The CM modules/instances on OMAP4 are quite different, so
 * they are handled in a separate file.
 */

// Dependencies supplied by the corresponding PRCM headers:
// "prcm-common.h" and "cm2xxx_3xxx.h".

/// OMAP2420 CM register address.
#[macro_export]
macro_rules! OMAP2420_CM_REGADDR {
    ($module:expr, $reg:expr) => {
        OMAP2_L4_IO_ADDRESS(OMAP2420_CM_BASE + ($module) + ($reg))
    };
}

/// OMAP2430 CM register address.
#[macro_export]
macro_rules! OMAP2430_CM_REGADDR {
    ($module:expr, $reg:expr) => {
        OMAP2_L4_IO_ADDRESS(OMAP2430_CM_BASE + ($module) + ($reg))
    };
}

/*
 * Module specific CM register offsets from CM_BASE + domain offset
 * Use cm_{read,write}_mod_reg() with these registers.
 * These register offsets generally appear in more than one PRCM submodule.
 */

/* OMAP2-specific register offsets */

pub const OMAP24XX_CM_FCLKEN2: u32 = 0x0004;
pub const OMAP24XX_CM_ICLKEN4: u32 = 0x001c;
pub const OMAP24XX_CM_AUTOIDLE4: u32 = 0x003c;
pub const OMAP24XX_CM_IDLEST4: u32 = 0x002c;

/* CM_IDLEST bit field values to indicate deasserted IdleReq */

pub const OMAP24XX_CM_IDLEST_VAL: u32 = 0;

/* Clock management domain register get/set */

unsafe extern "C" {
    pub fn omap2xxx_cm_set_dpll_disable_autoidle();
    pub fn omap2xxx_cm_set_dpll_auto_low_power_stop();

    pub fn omap2xxx_cm_fclks_active() -> i32;
    pub fn omap2xxx_cm_mpu_retention_allowed() -> i32;
    pub fn omap2xxx_cm_get_core_clk_src() -> u32;
    pub fn omap2xxx_cm_get_core_pll_config() -> u32;
    pub fn omap2xxx_cm_set_mod_dividers(
        mpu: u32,
        dsp: u32,
        gfx: u32,
        core: u32,
        mdm: u32,
    );

    pub fn omap2xxx_cm_init(data: *const omap_prcm_init_data) -> i32;
}

#[repr(C)]
pub struct omap_prcm_init_data {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
