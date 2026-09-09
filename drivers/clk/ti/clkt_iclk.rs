// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2/3 interface clock control
 *
 * Copyright (C) 2011 Nokia Corporation
 * Paul Walmsley
 */

// C dependencies:
// linux/kernel.h, linux/clk-provider.h, linux/io.h, linux/clk/ti.h, and
// clock.h provide the types and symbols referenced by this translation.

/* Register offsets */
const OMAP24XX_CM_FCLKEN2: u32 = 0x04;
const CM_AUTOIDLE: u32 = 0x30;
const CM_ICLKEN: u32 = 0x10;
const CM_IDLEST: u32 = 0x20;

const OMAP24XX_CM_IDLEST_VAL: u8 = 0;

/* Private functions */

/* XXX */
pub unsafe extern "C" fn omap2_clkt_iclk_allow_idle(clk: *mut clk_hw_omap) {
    let mut v: u32;
    let mut r: clk_omap_reg;

    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*clk).enable_reg),
        core::ptr::addr_of_mut!(r),
        1,
    );
    r.offset ^= CM_AUTOIDLE ^ CM_ICLKEN;

    v = ((*ti_clk_ll_ops).clk_readl)(&r);
    v |= 1u32 << (*clk).enable_bit;
    ((*ti_clk_ll_ops).clk_writel)(v, &r);
}

/* XXX */
pub unsafe extern "C" fn omap2_clkt_iclk_deny_idle(clk: *mut clk_hw_omap) {
    let mut v: u32;
    let mut r: clk_omap_reg;

    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*clk).enable_reg),
        core::ptr::addr_of_mut!(r),
        1,
    );

    r.offset ^= CM_AUTOIDLE ^ CM_ICLKEN;

    v = ((*ti_clk_ll_ops).clk_readl)(&r);
    v &= !(1u32 << (*clk).enable_bit);
    ((*ti_clk_ll_ops).clk_writel)(v, &r);
}

/**
 * omap2430_clk_i2chs_find_idlest - return CM_IDLEST info for 2430 I2CHS
 * @clk: struct clk * being enabled
 * @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
 * @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
 * @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
 *
 * OMAP2430 I2CHS CM_IDLEST bits are in CM_IDLEST1_CORE, but the
 * CM_*CLKEN bits are in CM_{I,F}CLKEN2_CORE.  This custom function
 * passes back the correct CM_IDLEST register address for I2CHS
 * modules.  No return value.
 */
unsafe extern "C" fn omap2430_clk_i2chs_find_idlest(
    clk: *mut clk_hw_omap,
    idlest_reg: *mut clk_omap_reg,
    idlest_bit: *mut u8,
    idlest_val: *mut u8,
) {
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*clk).enable_reg),
        idlest_reg,
        1,
    );
    (*idlest_reg).offset ^= OMAP24XX_CM_FCLKEN2 ^ CM_IDLEST;
    *idlest_bit = (*clk).enable_bit;
    *idlest_val = OMAP24XX_CM_IDLEST_VAL;
}

/* Public data */

pub static clkhwops_iclk: clk_hw_omap_ops = clk_hw_omap_ops {
    .allow_idle: Some(omap2_clkt_iclk_allow_idle),
    .deny_idle: Some(omap2_clkt_iclk_deny_idle),
    ..unsafe { core::mem::zeroed() }
};

pub static clkhwops_iclk_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    .allow_idle: Some(omap2_clkt_iclk_allow_idle),
    .deny_idle: Some(omap2_clkt_iclk_deny_idle),
    .find_idlest: Some(omap2_clk_dflt_find_idlest),
    .find_companion: Some(omap2_clk_dflt_find_companion),
    ..unsafe { core::mem::zeroed() }
};

/* 2430 I2CHS has non-standard IDLEST register */
pub static clkhwops_omap2430_i2chs_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    .find_idlest: Some(omap2430_clk_i2chs_find_idlest),
    .find_companion: Some(omap2_clk_dflt_find_companion),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
