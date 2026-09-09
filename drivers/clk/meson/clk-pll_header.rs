/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/clk-provider.h, linux/regmap.h, and "parm.h".

#[repr(C)]
pub struct pll_params_table {
    pub m: ::core::ffi::c_uint,
    pub n: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pll_mult_range {
    pub min: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
}

#[macro_export]
macro_rules! PLL_PARAMS {
    ($m:expr, $n:expr) => {
        $crate::pll_params_table {
            m: $m,
            n: $n,
        }
    };
}

pub const CLK_MESON_PLL_ROUND_CLOSEST: ::core::ffi::c_uint = 1u32 << 0;
pub const CLK_MESON_PLL_NOINIT_ENABLED: ::core::ffi::c_uint = 1u32 << 1;

#[repr(C)]
pub struct meson_clk_pll_data {
    pub en: parm,
    pub m: parm,
    pub n: parm,
    pub frac: parm,
    pub l: parm,
    pub rst: parm,
    pub current_en: parm,
    pub l_detect: parm,
    pub init_regs: *const reg_sequence,
    pub init_count: ::core::ffi::c_uint,
    pub table: *const pll_params_table,
    pub range: *const pll_mult_range,
    pub frac_max: ::core::ffi::c_uint,
    pub flags: u8,
}

extern "C" {
    pub static meson_clk_pll_ro_ops: clk_ops;
    pub static meson_clk_pll_ops: clk_ops;
    pub static meson_clk_pcie_pll_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
