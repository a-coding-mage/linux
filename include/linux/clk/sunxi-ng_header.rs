/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2017 Chen-Yu Tsai. All rights reserved.
 */

// C header guard: _LINUX_CLK_SUNXI_NG_H_

use core::ffi::c_void;

// Opaque types supplied by dependent translation units.
pub enum clk {}
pub enum device {}

extern "C" {
    pub fn sunxi_ccu_set_mmc_timing_mode(clk: *mut clk, new_mode: bool) -> i32;
    pub fn sunxi_ccu_get_mmc_timing_mode(clk: *mut clk) -> i32;

    pub fn sun6i_rtc_ccu_probe(dev: *mut device, reg: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
