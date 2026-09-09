/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL common clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// C dependencies supplied by other translation units:
// <linux/clk-provider.h>
// <linux/regmap.h>

pub enum device_node {}
pub enum platform_device {}
pub enum device {}
pub enum regmap {}
pub enum clk_hw {}
pub enum owl_reset_map {}

#[repr(C)]
pub struct clk_hw_onecell_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct owl_clk_common {
    pub regmap: *mut regmap,
    pub hw: clk_hw,
}

#[repr(C)]
pub struct owl_clk_desc {
    pub clks: *mut *mut owl_clk_common,
    pub num_clks: libc::c_ulong,
    pub hw_clks: *mut clk_hw_onecell_data,
    pub resets: *const owl_reset_map,
    pub num_resets: libc::c_ulong,
    pub regmap: *mut regmap,
}

#[inline]
pub unsafe fn hw_to_owl_clk_common(hw: *mut clk_hw) -> *mut owl_clk_common {
    // Equivalent to the Linux container_of(hw, struct owl_clk_common, hw).
    (hw as *mut u8)
        .sub(core::mem::size_of::<*mut regmap>())
        as *mut owl_clk_common
}

extern "C" {
    pub fn owl_clk_regmap_init(
        pdev: *mut platform_device,
        desc: *mut owl_clk_desc,
    ) -> libc::c_int;
    pub fn owl_clk_probe(
        dev: *mut device,
        hw_clks: *mut clk_hw_onecell_data,
    ) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
