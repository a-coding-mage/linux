/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Translated from clk-phase.h. The declarations below depend on the
// externally supplied `parm` and `clk_ops` types.

#[repr(C)]
pub struct meson_clk_phase_data {
    pub ph: parm,
}

#[repr(C)]
pub struct meson_clk_triphase_data {
    pub ph0: parm,
    pub ph1: parm,
    pub ph2: parm,
}

#[repr(C)]
pub struct meson_sclk_ws_inv_data {
    pub ph: parm,
    pub ws: parm,
}

extern "C" {
    pub static meson_clk_phase_ops: clk_ops;
    pub static meson_clk_triphase_ops: clk_ops;
    pub static meson_sclk_ws_inv_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
