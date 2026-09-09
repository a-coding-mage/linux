/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/mach-omap2/clock.h
 *
 *  Copyright (C) 2005-2009 Texas Instruments, Inc.
 *  Copyright (C) 2004-2011 Nokia Corporation
 *
 *  Contacts:
 *  Richard Woodruff <r-woodruff2@ti.com>
 *  Paul Walmsley
 */

// C dependencies: linux/kernel.h, linux/list.h, linux/clkdev.h,
// linux/clk-provider.h, and linux/clk/ti.h.

/* struct clksel_rate.flags possibilities */
pub const RATE_IN_242X: u32 = 1 << 0;
pub const RATE_IN_243X: u32 = 1 << 1;
pub const RATE_IN_3430ES1: u32 = 1 << 2; /* 3430ES1 rates only */
pub const RATE_IN_3430ES2PLUS: u32 = 1 << 3; /* 3430 ES >= 2 rates only */
pub const RATE_IN_36XX: u32 = 1 << 4;
pub const RATE_IN_4430: u32 = 1 << 5;
pub const RATE_IN_TI816X: u32 = 1 << 6;
pub const RATE_IN_4460: u32 = 1 << 7;
pub const RATE_IN_AM33XX: u32 = 1 << 8;
pub const RATE_IN_TI814X: u32 = 1 << 9;

pub const RATE_IN_24XX: u32 = RATE_IN_242X | RATE_IN_243X;
pub const RATE_IN_34XX: u32 = RATE_IN_3430ES1 | RATE_IN_3430ES2PLUS;
pub const RATE_IN_3XXX: u32 = RATE_IN_34XX | RATE_IN_36XX;
pub const RATE_IN_44XX: u32 = RATE_IN_4430 | RATE_IN_4460;

/* RATE_IN_3430ES2PLUS_36XX includes 34xx/35xx with ES >=2, and all 36xx/37xx */
pub const RATE_IN_3430ES2PLUS_36XX: u32 = RATE_IN_3430ES2PLUS | RATE_IN_36XX;

/* CM_CLKSEL2_PLL.CORE_CLK_SRC bits (2XXX) */
pub const CORE_CLK_SRC_32K: u32 = 0x0;
pub const CORE_CLK_SRC_DPLL: u32 = 0x1;
pub const CORE_CLK_SRC_DPLL_X2: u32 = 0x2;

/* OMAP2xxx CM_CLKEN_PLL.EN_DPLL bits - for omap2_get_dpll_rate() */
pub const OMAP2XXX_EN_DPLL_LPBYPASS: u32 = 0x1;
pub const OMAP2XXX_EN_DPLL_FRBYPASS: u32 = 0x2;
pub const OMAP2XXX_EN_DPLL_LOCKED: u32 = 0x3;

/* OMAP3xxx CM_CLKEN_PLL*.EN_*_DPLL bits - for omap2_get_dpll_rate() */
pub const OMAP3XXX_EN_DPLL_LPBYPASS: u32 = 0x5;
pub const OMAP3XXX_EN_DPLL_FRBYPASS: u32 = 0x6;
pub const OMAP3XXX_EN_DPLL_LOCKED: u32 = 0x7;

/* OMAP4xxx CM_CLKMODE_DPLL*.EN_*_DPLL bits - for omap2_get_dpll_rate() */
pub const OMAP4XXX_EN_DPLL_MNBYPASS: u32 = 0x4;
pub const OMAP4XXX_EN_DPLL_LPBYPASS: u32 = 0x5;
pub const OMAP4XXX_EN_DPLL_FRBYPASS: u32 = 0x6;
pub const OMAP4XXX_EN_DPLL_LOCKED: u32 = 0x7;

#[repr(C)]
pub struct ti_clk_ll_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut omap_clk_ll_ops: ti_clk_ll_ops;

    // C __init annotation has no direct Rust equivalent.
    pub fn omap2_clk_setup_ll_ops() -> i32;

    // C __init annotation has no direct Rust equivalent.
    pub fn ti_clk_init_features();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
