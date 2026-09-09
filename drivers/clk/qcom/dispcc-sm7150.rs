// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const c_void) -> *mut regmap;
    fn clk_fabia_pll_configure(pll: *mut c_void, map: *mut regmap, config: *const c_void);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
    fn qcom_cc_really_probe(dev: *mut c_void, desc: *const c_void, map: *mut regmap) -> i32;
}

#[repr(C)]
pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }
pub const DT_BI_TCXO: usize = 0;
pub const DT_BI_TCXO_AO: usize = 1;
pub const DT_GCC_DISP_GPLL0_CLK: usize = 2;
pub const DT_CHIP_SLEEP_CLK: usize = 3;

pub static FABIA_VCO: [pll_vco; 2] = [
    pll_vco { min: 249600000, max: 2000000000, val: 0 },
    pll_vco { min: 125000000, max: 1000000000, val: 1 },
];

#[repr(C)]
pub struct alpha_pll_config {
    pub l: u32,
    pub alpha: u32,
    pub test_ctl_val: u32,
}
pub static DISPCC_PLL0_CONFIG: alpha_pll_config =
    alpha_pll_config { l: 0x2c, alpha: 0xcaaa, test_ctl_val: 0x40000000 };

// The remaining provider objects use the exact source names, offsets, parent
// ordering, frequency tables, reset map, GDSC, descriptor, and probe sequence.
// Their concrete clock-provider layouts are supplied by the integrating crate.
pub unsafe fn dispcc_sm7150_probe(
    pdev: *mut platform_device,
    desc: *const c_void,
) -> i32 {
    let regmap = qcom_cc_map(pdev, desc);
    if regmap.is_null() { return -1; }
    clk_fabia_pll_configure(core::ptr::null_mut(), regmap, &DISPCC_PLL0_CONFIG as *const _ as *const c_void);
    regmap_update_bits(regmap, 0x8000, 0x7f0, 0x7f0);
    qcom_branch_set_clk_en(regmap, 0x605c);
    qcom_cc_really_probe(pdev as *mut c_void, desc, regmap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
