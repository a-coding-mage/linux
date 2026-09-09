/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependency: <linux/clk-provider.h>
// Dependency: "parm.h"

#[repr(C)]
pub struct meson_vid_pll_div_data {
    pub val: parm,
    pub sel: parm,
}

extern "C" {
    pub static meson_vid_pll_div_ro_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
