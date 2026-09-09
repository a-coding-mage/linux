/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Neil Armstrong <neil.armstrong@linaro.org>
 */

// Dependencies supplied by the surrounding translation unit:
// clk-regmap.h and parm.h

/**
 * struct meson_vclk_gate_data - vclk_gate regmap backed specific data
 *
 * @enable: vclk enable field
 * @reset:  vclk reset field
 * @flags:  hardware-specific flags
 *
 * Flags:
 * Same as clk_gate except CLK_GATE_HIWORD_MASK which is ignored
 */
#[repr(C)]
pub struct meson_vclk_gate_data {
    pub enable: parm,
    pub reset: parm,
    pub flags: u8,
}

pub unsafe extern "C" {
    pub static meson_vclk_gate_ops: clk_ops;
}

/**
 * struct meson_vclk_div_data - vclk_div regmap back specific data
 *
 * @div:     divider field
 * @enable:  vclk divider enable field
 * @reset:   vclk divider reset field
 * @table:   array of value/divider pairs, last entry should have div = 0
 *
 * Flags:
 * Same as clk_divider except CLK_DIVIDER_HIWORD_MASK which is ignored
 */
#[repr(C)]
pub struct meson_vclk_div_data {
    pub div: parm,
    pub enable: parm,
    pub reset: parm,
    pub table: *const clk_div_table,
    pub flags: u8,
}

pub unsafe extern "C" {
    pub static meson_vclk_div_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
