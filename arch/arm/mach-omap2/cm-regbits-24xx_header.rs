/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * OMAP24XX Clock Management register bits
 *
 * Copyright (C) 2007 Texas Instruments, Inc.
 * Copyright (C) 2007 Nokia Corporation
 *
 * Written by Paul Walmsley
 */

pub const OMAP24XX_AUTOSTATE_MPU_MASK: u32 = 1u32 << 0;
pub const OMAP24XX_EN_DSS1_MASK: u32 = 1u32 << 0;
pub const OMAP24XX_ST_MAILBOXES_SHIFT: u32 = 30;
pub const OMAP24XX_ST_HDQ_SHIFT: u32 = 23;
pub const OMAP2420_ST_I2C2_SHIFT: u32 = 20;
pub const OMAP2430_ST_I2CHS1_SHIFT: u32 = 19;
pub const OMAP2420_ST_I2C1_SHIFT: u32 = 19;
pub const OMAP2430_ST_I2CHS2_SHIFT: u32 = 20;
pub const OMAP24XX_ST_MCBSP2_SHIFT: u32 = 16;
pub const OMAP24XX_ST_MCBSP1_SHIFT: u32 = 15;
pub const OMAP2430_ST_MCBSP5_SHIFT: u32 = 5;
pub const OMAP2430_ST_MCBSP4_SHIFT: u32 = 4;
pub const OMAP2430_ST_MCBSP3_SHIFT: u32 = 3;
pub const OMAP24XX_ST_AES_SHIFT: u32 = 3;
pub const OMAP24XX_ST_RNG_SHIFT: u32 = 2;
pub const OMAP24XX_ST_SHA_SHIFT: u32 = 1;
pub const OMAP24XX_CLKSEL_DSS2_MASK: u32 = 0x1u32 << 13;
pub const OMAP24XX_AUTOSTATE_DSS_MASK: u32 = 1u32 << 2;
pub const OMAP24XX_AUTOSTATE_L4_MASK: u32 = 1u32 << 1;
pub const OMAP24XX_AUTOSTATE_L3_MASK: u32 = 1u32 << 0;
pub const OMAP24XX_AUTOSTATE_GFX_MASK: u32 = 1u32 << 0;
pub const OMAP24XX_ST_MPU_WDT_SHIFT: u32 = 3;
pub const OMAP24XX_ST_32KSYNC_SHIFT: u32 = 1;
pub const OMAP24XX_EN_54M_PLL_SHIFT: u32 = 6;
pub const OMAP24XX_EN_96M_PLL_SHIFT: u32 = 2;
pub const OMAP24XX_ST_54M_APLL_SHIFT: u32 = 9;
pub const OMAP24XX_ST_96M_APLL_SHIFT: u32 = 8;
pub const OMAP24XX_AUTO_54M_MASK: u32 = 0x3u32 << 6;
pub const OMAP24XX_AUTO_96M_MASK: u32 = 0x3u32 << 2;
pub const OMAP24XX_AUTO_DPLL_SHIFT: u32 = 0;
pub const OMAP24XX_AUTO_DPLL_MASK: u32 = 0x3u32 << 0;
pub const OMAP24XX_CORE_CLK_SRC_MASK: u32 = 0x3u32 << 0;
pub const OMAP2420_AUTOSTATE_IVA_MASK: u32 = 1u32 << 8;
pub const OMAP24XX_AUTOSTATE_DSP_MASK: u32 = 1u32 << 0;
pub const OMAP2430_AUTOSTATE_MDM_MASK: u32 = 1u32 << 0;
pub const OMAP24XX_CLKSTCTRL_DISABLE_AUTO: u32 = 0x0;
pub const OMAP24XX_CLKSTCTRL_ENABLE_AUTO: u32 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
