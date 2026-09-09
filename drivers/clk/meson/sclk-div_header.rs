/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies supplied by the Linux clock provider and local parm definitions.

#[repr(C)]
pub struct meson_sclk_div_data {
    pub div: parm,
    pub hi: parm,
    pub cached_div: u32,
    pub cached_duty: clk_duty,
}

extern "C" {
    pub static meson_sclk_div_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
