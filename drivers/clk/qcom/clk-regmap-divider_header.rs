/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Dependency declarations supplied by the surrounding translation:
// linux/clk-provider.h and clk-regmap.h

#[repr(C)]
pub struct clk_regmap_div {
    pub reg: u32,
    pub shift: u32,
    pub width: u32,
    pub clkr: clk_regmap,
}

extern "C" {
    pub static clk_regmap_div_ops: clk_ops;
    pub static clk_regmap_div_ro_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
