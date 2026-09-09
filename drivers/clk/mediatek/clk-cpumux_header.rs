/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Linaro Ltd.
 * Author: Pi-Cheng Chen <pi-cheng.chen@linaro.org>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct clk_hw_onecell_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_composite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn mtk_clk_register_cpumuxes(
        dev: *mut device,
        node: *mut device_node,
        clks: *const mtk_composite,
        num: c_int,
        clk_data: *mut clk_hw_onecell_data,
    ) -> c_int;

    pub fn mtk_clk_unregister_cpumuxes(
        clks: *const mtk_composite,
        num: c_int,
        clk_data: *mut clk_hw_onecell_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
