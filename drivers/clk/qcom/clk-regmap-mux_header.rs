/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the corresponding Linux clock-provider and
// clk-regmap/common headers.

#[repr(C)]
pub struct clk_regmap_mux {
    pub reg: u32,
    pub shift: u32,
    pub width: u32,
    pub parent_map: *const parent_map,
    pub clkr: clk_regmap,
}

extern "C" {
    pub static clk_regmap_mux_closest_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
