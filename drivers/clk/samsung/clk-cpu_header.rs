/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 *
 * Common Clock Framework support for all PLL's in Samsung platforms
 */

/* The CPU clock registers have DIV1 configuration register */
pub const CLK_CPU_HAS_DIV1: usize = 1usize << 0;
/* When ALT parent is active, debug clocks need safe divider values */
pub const CLK_CPU_NEEDS_DEBUG_ALT_DIV: usize = 1usize << 1;

/**
 * enum exynos_cpuclk_layout - CPU clock registers layout compatibility
 * @CPUCLK_LAYOUT_E4210: Exynos4210 compatible layout
 * @CPUCLK_LAYOUT_E5433: Exynos5433 compatible layout
 * @CPUCLK_LAYOUT_E850_CL0: Exynos850 cluster 0 compatible layout
 * @CPUCLK_LAYOUT_E850_CL1: Exynos850 cluster 1 compatible layout
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum exynos_cpuclk_layout {
    CPUCLK_LAYOUT_E4210,
    CPUCLK_LAYOUT_E5433,
    CPUCLK_LAYOUT_E850_CL0,
    CPUCLK_LAYOUT_E850_CL1,
}

/**
 * struct exynos_cpuclk_cfg_data - config data to setup cpu clocks
 * @prate: frequency of the primary parent clock (in KHz)
 * @div0: value to be programmed in the div_cpu0 register
 * @div1: value to be programmed in the div_cpu1 register
 *
 * This structure holds the divider configuration data for dividers in the CPU
 * clock domain. The parent frequency at which these divider values are valid is
 * specified in @prate. The @prate is the frequency of the primary parent clock.
 * For CPU clock domains that do not have a DIV1 register, the @div1 member
 * value is not used.
 */
#[repr(C)]
pub struct exynos_cpuclk_cfg_data {
    pub prate: usize,
    pub div0: usize,
    pub div1: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
