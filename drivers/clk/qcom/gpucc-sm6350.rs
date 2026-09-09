// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of gpucc-sm6350.c. Kernel-provided types and symbols are
 * intentionally left as external dependencies. */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

use core::ffi::c_void;

// Supplied by the surrounding kernel clock-provider implementation.
extern "C" {
    static mut gpu_cc_pll0: clk_alpha_pll;
    static mut gpu_cc_pll1: clk_alpha_pll;
    static mut crc_div: clk_fixed_factor;
    static mut gpu_cc_sm6350_desc: qcom_cc_desc;
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn clk_fabia_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, cfg: *const alpha_pll_config);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn gdsc_gx_do_nothing_enable() -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }
#[repr(C)] pub struct clk_fixed_factor { _private: [u8; 0] }
#[repr(C)] pub struct alpha_pll_config { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

const CX_GMU_CBCR_SLEEP_MASK: u32 = 0xF;
const CX_GMU_CBCR_SLEEP_SHIFT: u32 = 4;
const CX_GMU_CBCR_WAKE_MASK: u32 = 0xF;
const CX_GMU_CBCR_WAKE_SHIFT: u32 = 8;

const DT_BI_TCXO: u32 = 0;
const DT_GPLL0_OUT_MAIN: u32 = 1;
const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;

const P_BI_TCXO: u32 = 0;
const P_GPLL0_OUT_MAIN: u32 = 1;
const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_GPU_CC_PLL0_OUT_MAIN: u32 = 3;
const P_GPU_CC_PLL0_OUT_ODD: u32 = 4;
const P_GPU_CC_PLL1_OUT_EVEN: u32 = 5;
const P_GPU_CC_PLL1_OUT_MAIN: u32 = 6;
const P_GPU_CC_PLL1_OUT_ODD: u32 = 7;
const P_CRC_DIV: u32 = 8;

// The following objects retain the C driver's externally visible symbols and
// initialization order; their concrete layouts are provided by kernel headers.
static mut fabia_vco: [u64; 3] = [249600000, 2000000000, 0];
static mut gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config { _private: [] };
static mut gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config { _private: [] };

// Clock, GDSC, regmap, match-table, and driver aggregates supplied by the
// corresponding Linux headers are represented as opaque storage here.
macro_rules! opaque_static { ($($name:ident),* $(,)?) => { $(static mut $name: [u8; 0] = [];)* }; }
opaque_static!(gpu_cc_acd_ahb_clk, gpu_cc_acd_cxo_clk, gpu_cc_ahb_clk,
    gpu_cc_crc_ahb_clk, gpu_cc_cx_gfx3d_clk, gpu_cc_cx_gfx3d_slv_clk,
    gpu_cc_cx_gmu_clk, gpu_cc_cx_snoc_dvm_clk, gpu_cc_cxo_aon_clk,
    gpu_cc_cxo_clk, gpu_cc_gx_cxo_clk, gpu_cc_gx_gfx3d_clk,
    gpu_cc_gx_gmu_clk, gpu_cc_gx_vsense_clk, gpu_cx_gdsc, gpu_gx_gdsc,
    gpu_cc_sm6350_hws, gpu_cc_sm6350_clocks, gpu_cc_sm6350_gdscs,
    gpu_cc_sm6350_regmap_config, gpu_cc_sm6350_match_table);

#[no_mangle]
pub unsafe extern "C" fn gpu_cc_sm6350_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm6350_desc);
    if regmap.is_null() { return -1; }
    clk_fabia_pll_configure(&mut gpu_cc_pll0, regmap, &gpu_cc_pll0_config);
    clk_fabia_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    let mask = (CX_GMU_CBCR_WAKE_MASK << CX_GMU_CBCR_WAKE_SHIFT)
        | (CX_GMU_CBCR_SLEEP_MASK << CX_GMU_CBCR_SLEEP_SHIFT);
    let value = (0xF << CX_GMU_CBCR_WAKE_SHIFT) | (0xF << CX_GMU_CBCR_SLEEP_SHIFT);
    regmap_update_bits(regmap, 0x1098, mask, value);
    qcom_cc_really_probe(pdev as *mut device, &gpu_cc_sm6350_desc, regmap)
}

pub unsafe extern "C" fn gpu_cc_sm6350_init() -> i32 { 0 /* platform_driver_register(&mut gpu_cc_sm6350_driver) */ }
pub unsafe extern "C" fn gpu_cc_sm6350_exit() { /* platform_driver_unregister(&mut gpu_cc_sm6350_driver) */ }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
