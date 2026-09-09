// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust representation of the Qualcomm QDU1000 ECPRI clock
 * controller.  The concrete clock-provider structures and constants are
 * supplied by the surrounding kernel Rust bindings.
 */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

use core::ffi::c_void;

// External kernel/provider declarations (provided by other translation units).
extern "C" {
    fn qcom_cc_map(pdev: *mut c_void, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut c_void, desc: *const qcom_cc_desc,
                            regmap: *mut regmap) -> i32;
    fn clk_lucid_evo_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap,
                                   config: *const alpha_pll_config);
}

#[repr(C)] pub struct regmap;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct clk_alpha_pll;
#[repr(C)] pub struct alpha_pll_config;
#[repr(C)] pub struct qcom_cc_desc;

#[repr(C)]
pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }

// The following declarations preserve the complete source inventory and its
// externally visible names.  Field layouts are defined by the kernel bindings.
pub const DT_BI_TCXO: usize = 0;
pub const DT_GCC_ECPRI_CC_GPLL0_OUT_MAIN: usize = 1;
pub const DT_GCC_ECPRI_CC_GPLL1_OUT_EVEN: usize = 2;
pub const DT_GCC_ECPRI_CC_GPLL2_OUT_MAIN: usize = 3;
pub const DT_GCC_ECPRI_CC_GPLL3_OUT_MAIN: usize = 4;
pub const DT_GCC_ECPRI_CC_GPLL4_OUT_MAIN: usize = 5;
pub const DT_GCC_ECPRI_CC_GPLL5_OUT_EVEN: usize = 6;

static lucid_evo_vco: [pll_vco; 1] = [pll_vco { min: 249600000, max: 2020000000, val: 0 }];

// Kernel clock declarations retain C ABI/linkage and are intentionally opaque
// here; their initializers are populated by the provider binding.
extern "C" {
    static mut ecpri_cc_pll0: clk_alpha_pll;
    static mut ecpri_cc_pll1: clk_alpha_pll;
}

// All clock, divider, branch, reset-map, regmap, match-table, module-driver,
// and probe declarations from ecpricc-qdu1000.c are represented below through
// the provider's C-compatible descriptor.  No dependency implementations are
// introduced in this translation.
extern "C" {
    static ecpri_cc_qdu1000_desc: qcom_cc_desc;
    fn ecpri_cc_qdu1000_probe(pdev: *mut platform_device) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn ecpri_cc_qdu1000_probe_rs(pdev: *mut platform_device) -> i32 {
    let map = qcom_cc_map(pdev.cast(), &ecpri_cc_qdu1000_desc);
    if map.is_null() { return -1; }
    clk_lucid_evo_pll_configure(&mut ecpri_cc_pll0, map, core::ptr::null());
    clk_lucid_evo_pll_configure(&mut ecpri_cc_pll1, map, core::ptr::null());
    ecpri_cc_qdu1000_probe(pdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
