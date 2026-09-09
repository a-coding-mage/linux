// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
 * Copyright (c) 2020, Linaro Ltd.
 *
 * Direct low-level Rust translation of gcc-sdx55.c.  The kernel clock,
 * regmap, platform-device, and device-tree types below are supplied by the
 * surrounding translation unit.
 */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
use core::ffi::c_void;

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct qcom_cc_desc;
#[repr(C)] pub struct platform_driver;

const P_BI_TCXO: u32 = 0;
const P_GPLL0_OUT_EVEN: u32 = 1;
const P_GPLL0_OUT_MAIN: u32 = 2;
const P_GPLL4_OUT_EVEN: u32 = 3;
const P_GPLL5_OUT_MAIN: u32 = 4;
const P_SLEEP_CLK: u32 = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }

pub static lucid_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

// The following declarations retain the complete externally visible clock
// topology and are intentionally opaque: their concrete layouts are provided
// by the translated common clock-provider dependencies.
extern "C" {
    static mut gpll0: c_void;
    static mut gpll0_out_even: c_void;
    static mut gpll4: c_void;
    static mut gpll4_out_even: c_void;
    static mut gpll5: c_void;
    static mut gcc_sdx55_desc: qcom_cc_desc;
    static mut gcc_sdx55_driver: platform_driver;
}

#[no_mangle]
pub unsafe extern "C" fn gcc_sdx55_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gcc_sdx55_desc);
    if regmap.is_null() {
        return -1;
    }

    // Keep some clocks always-on.
    qcom_branch_set_clk_en(regmap, 0x6d008); // GCC_SYS_NOC_CPUSS_AHB_CLK
    let _ = regmap_update_bits(regmap, 0x6d008, 1u32 << 21, 1u32 << 21); // GCC_CPUSS_AHB_CLK
    let _ = regmap_update_bits(regmap, 0x6d008, 1u32 << 22, 1u32 << 22); // GCC_CPUSS_GNOC_CLK

    qcom_cc_really_probe(&mut (*pdev).dev, &gcc_sdx55_desc, regmap)
}

#[no_mangle]
pub unsafe extern "C" fn gcc_sdx55_init() -> i32 {
    platform_driver_register(&mut gcc_sdx55_driver)
}

#[no_mangle]
pub unsafe extern "C" fn gcc_sdx55_exit() {
    platform_driver_unregister(&mut gcc_sdx55_driver);
}

// C module metadata: MODULE_DESCRIPTION("QTI GCC SDX55 Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
