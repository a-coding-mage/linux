/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017, Linaro Limited
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Dependencies supplied by the surrounding kernel clock implementation:
// linux/clk-provider.h and clk-regmap.h

/**
 * struct mux_div_clk - combined mux/divider clock
 * @reg_offset: offset of the mux/divider register
 * @hid_width: number of bits in half integer divider
 * @hid_shift: lowest bit of hid value field
 * @src_width: number of bits in source select
 * @src_shift: lowest bit of source select field
 * @div: the divider raw configuration value
 * @src: the mux index which will be used if the clock is enabled
 * @parent_map: map from parent_names index to src_sel field
 * @clkr: handle between common and hardware-specific interfaces
 * @pclk: the input PLL clock
 * @clk_nb: clock notifier for rate changes of the input PLL
 */
#[repr(C)]
pub struct clk_regmap_mux_div {
    pub reg_offset: u32,
    pub hid_width: u32,
    pub hid_shift: u32,
    pub src_width: u32,
    pub src_shift: u32,
    pub div: u32,
    pub src: u32,
    pub parent_map: *const u32,
    pub clkr: clk_regmap,
    pub pclk: *mut clk,
    pub clk_nb: notifier_block,
}

// External types and symbols supplied by the included kernel headers.
pub struct clk_regmap;
pub struct clk;
pub struct notifier_block;
pub struct clk_ops;

unsafe extern "C" {
    pub static clk_regmap_mux_div_ops: clk_ops;
    pub fn mux_div_set_src_div(
        md: *mut clk_regmap_mux_div,
        src: u32,
        div: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
