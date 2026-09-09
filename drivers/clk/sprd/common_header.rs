/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum clock infrastructure
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// C dependencies:
// #include <linux/clk-provider.h>
// #include <linux/of_platform.h>
// #include <linux/regmap.h>

// `device_node`, `regmap`, `clk_hw`, `platform_device`, `device`, and
// `clk_hw_onecell_data` are supplied by the corresponding external bindings.

#[repr(C)]
pub struct sprd_clk_common {
    pub regmap: *mut regmap,
    pub reg: u32,
    pub hw: clk_hw,
}

#[repr(C)]
pub struct sprd_clk_desc {
    pub clk_clks: *mut *mut sprd_clk_common,
    pub num_clk_clks: usize,
    pub hw_clks: *mut clk_hw_onecell_data,
}

#[inline]
pub unsafe fn hw_to_sprd_clk_common(hw: *const clk_hw) -> *mut sprd_clk_common {
    (hw as *const u8)
        .sub(core::mem::offset_of!(sprd_clk_common, hw))
        as *mut sprd_clk_common
}

extern "C" {
    pub fn sprd_clk_regmap_init(
        pdev: *mut platform_device,
        desc: *const sprd_clk_desc,
    ) -> i32;

    pub fn sprd_clk_probe(dev: *mut device, clkhw: *mut clk_hw_onecell_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
