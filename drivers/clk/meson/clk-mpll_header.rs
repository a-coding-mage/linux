/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/clk-provider.h, linux/spinlock.h, and "parm.h".

#[repr(C)]
pub struct meson_clk_mpll_data {
    pub sdm: crate::parm,
    pub sdm_en: crate::parm,
    pub n2: crate::parm,
    pub ssen: crate::parm,
    pub misc: crate::parm,
    pub init_regs: *const crate::reg_sequence,
    pub init_count: ::core::ffi::c_uint,
    pub flags: u8,
}

pub const CLK_MESON_MPLL_ROUND_CLOSEST: u32 = 1u32 << 0;
pub const CLK_MESON_MPLL_SPREAD_SPECTRUM: u32 = 1u32 << 1;

extern "C" {
    pub static meson_clk_mpll_ro_ops: crate::clk_ops;
    pub static meson_clk_mpll_ops: crate::clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
