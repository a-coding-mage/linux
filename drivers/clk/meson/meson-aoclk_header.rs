/* SPDX-License-Identifier: (GPL-2.0+ OR MIT) */
/*
 * Copyright (c) 2017 BayLibre, SAS
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 *
 * Copyright (c) 2018 Amlogic, inc.
 * Author: Qiufang Dai <qiufang.dai@amlogic.com>
 * Author: Yixun Lan <yixun.lan@amlogic.com>
 */

// Dependencies supplied by the corresponding kernel and clock-controller modules:
// linux/clk-provider.h, linux/platform_device.h, linux/regmap.h,
// linux/reset-controller.h, clk-regmap.h, and meson-clkc-utils.h.

#[repr(C)]
pub struct MesonAoclkData {
    pub clkc_data: crate::meson_clkc_data,
    pub reset_reg: ::core::ffi::c_uint,
    pub num_reset: ::core::ffi::c_int,
    pub reset: *const ::core::ffi::c_uint,
}

#[repr(C)]
pub struct MesonAoclkResetController {
    pub reset: crate::reset_controller_dev,
    pub data: *const MesonAoclkData,
    pub regmap: *mut crate::regmap,
}

unsafe extern "C" {
    pub fn meson_aoclkc_probe(pdev: *mut crate::platform_device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
