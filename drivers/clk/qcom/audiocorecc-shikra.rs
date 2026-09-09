// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct Rust translation of the Qualcomm AUDIOCORECC Shikra driver.
 * Kernel-provided types, constants, macros, and functions are external
 * dependencies and are intentionally not implemented here.
 */

use core::ffi::c_void;

// External kernel clock-framework declarations.
extern "C" {
    static clk_alpha_pll_regs: *const c_void;
    static clk_alpha_pll_fixed_ops: c_void;
    static clk_fixed_factor_ops: c_void;
    static clk_rcg2_shared_ops: c_void;
    static clk_regmap_div_ro_ops: c_void;
    static clk_branch2_ops: c_void;
    static clk_branch2_aon_ops: c_void;
    fn device_get_match_data(dev: *const c_void) -> *const qcom_cc_desc;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clk_hws: *mut *mut clk_hw, pub num_clk_hws: usize, pub clks: *mut *mut clk_regmap, pub num_clks: usize, pub driver_data: *const qcom_cc_driver_data, pub resets: *const qcom_reset_map, pub num_resets: usize }
#[repr(C)] pub struct regmap_config { pub name: *const u8, pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_cc_driver_data { pub alpha_plls: *mut *mut clk_alpha_pll, pub num_alpha_plls: usize }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32, pub bit: u32 }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }

#[repr(C)] pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub vco_val: u32, pub post_div_val: u32, pub post_div_mask: u32, pub vco_mask: u32, pub main_output_mask: u32, pub aux_output_mask: u32, pub aux2_output_mask: u32, pub config_ctl_val: u32, pub test_ctl_hi_val: u32, pub test_ctl_hi_mask: u32 }
#[repr(C)] pub struct freq_tbl { pub freq: u64, pub parent: u32, pub m: f64, pub n: u32, pub d: u32 }

const DT_BI_TCXO: u32 = 0;
const DT_SLEEP_CLK: u32 = 1;
const DT_AUD_REF_CLK_SRC: u32 = 2;
const P_AUD_REF_CLK_SRC: u32 = 0;
const P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX: u32 = 1;
const P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2: u32 = 2;
const P_BI_TCXO: u32 = 3;
const P_SLEEP_CLK: u32 = 4;

const SPARK_VCO: [pll_vco; 1] = [pll_vco { min: 500_000_000, max: 1_000_000_000, val: 2 }];
const AUDIO_CORE_CC_DIG_PLL_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x20, alpha: 0, vco_val: 1 << 21, post_div_val: 0x28100, post_div_mask: 0x3ff00, vco_mask: 0x0030_0000, main_output_mask: 1, aux_output_mask: 2, aux2_output_mask: 4, config_ctl_val: 0x4001055b, test_ctl_hi_val: 1, test_ctl_hi_mask: 1 };

// Frequency tables, preserving the C F(...) entries and their ordering.
const FTBL_AIF_IF0: &[(u64, u32, f64, u32, u32)] = &[
    (240000, P_BI_TCXO, 10.0, 1, 8), (256000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 1, 32),
    (512000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 1, 16), (768000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 10.0, 1, 16),
    (1024000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 1, 8), (1536000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 10.0, 1, 8),
    (2048000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 1, 4), (3072000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 10.0, 1, 4),
    (4096000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 1, 2), (6144000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 10.0, 1, 2),
    (8192000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 15.0, 0, 0), (9600000, P_BI_TCXO, 2.0, 0, 0),
    (12288000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 10.0, 0, 0), (19200000, P_BI_TCXO, 1.0, 0, 0),
    (24576000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 5.0, 0, 0),
];
const FTBL_AIF_IF3: &[(u64, u32, f64, u32, u32)] = &[(49152000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 2.5, 0, 0)];
const FTBL_AUD_DMA: &[(u64, u32, f64, u32, u32)] = &[(38400000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 8.0, 0, 0), (102400000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 3.0, 0, 0), (153600000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 2.0, 0, 0), (307200000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 1.0, 0, 0)];
const FTBL_BUS: &[(u64, u32, f64, u32, u32)] = &[(38400000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 8.0, 0, 0), (76800000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX2, 4.0, 0, 0)];
const FTBL_PCMOE: &[(u64, u32, f64, u32, u32)] = &[(9600000, P_BI_TCXO, 2.0, 0, 0), (15360000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 8.0, 0, 0), (30720000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 4.0, 0, 0), (61440000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 2.0, 0, 0)];
const FTBL_TX_MCLK: &[(u64, u32, f64, u32, u32)] = &[(19200000, P_BI_TCXO, 1.0, 0, 0), (24576000, P_AUDIO_CORE_CC_DIG_PLL_OUT_AUX, 5.0, 0, 0)];

// The remaining kernel aggregate objects retain their original externally
// visible names and topology. Their concrete layouts are supplied by the
// Linux clock framework in the integration unit.
extern "C" {
    static mut audio_core_cc_dig_pll: clk_alpha_pll;
    static mut audio_core_cc_dig_pll_out_aux: clk_hw;
    static mut audio_core_cc_dig_pll_out_aux2: clk_hw;
    static mut audio_core_cc_shikra_hws: [*mut clk_hw; 2];
    static mut audio_core_cc_shikra_clocks: [*mut clk_regmap; 29];
    static mut audio_core_cc_shikra_plls: [*mut clk_alpha_pll; 1];
}

#[no_mangle]
pub unsafe extern "C" fn audio_core_cc_shikra_probe(pdev: *mut platform_device) -> i32 {
    let desc = device_get_match_data(pdev as *const c_void);
    if desc.is_null() { return -22; }
    qcom_cc_probe(pdev, desc)
}

// MODULE_DEVICE_TABLE, module_platform_driver, MODULE_DESCRIPTION, and
// MODULE_LICENSE are build-time Linux macros; their intent is retained here.
#[no_mangle] pub static AUDIO_CORE_CC_SHIKRA_DRIVER_NAME: &[u8] = b"audiocorecc-shikra\0";
#[no_mangle] pub static AUDIO_CORE_CC_SHIKRA_DESCRIPTION: &[u8] = b"QTI AUDIOCORECC Shikra Driver\0";
#[no_mangle] pub static AUDIO_CORE_CC_SHIKRA_LICENSE: &[u8] = b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
