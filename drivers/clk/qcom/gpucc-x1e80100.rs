// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the Qualcomm X1E80100 GPU clock controller. */

#![allow(dead_code, non_upper_case_globals, non_snake_case, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* These types and operations are supplied by the surrounding clock framework. */
#[repr(C)] pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub config_ctl_hi2_val: u32, pub test_ctl_val: u32, pub test_ctl_hi_val: u32, pub test_ctl_hi1_val: u32, pub test_ctl_hi2_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const c_void, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub gds_hw_ctrl: u32, pub clamp_io_ctrl: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32, pub pwrsts: u32, pub flags: u32 }
#[repr(C)] pub struct parent_map { pub parent: u32, pub value: u32 }
#[repr(C)] pub struct clk_parent_data { pub index: u32, pub hw: *const clk_hw }
#[repr(C)] pub struct freq_tbl { pub freq: u64, pub src: u32, pub pre_div: u32, pub m: u32, pub n: u32 }

const DT_BI_TCXO: u32 = 0; const DT_GPLL0_OUT_MAIN: u32 = 1; const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_BI_TCXO: u32 = 0; const P_GPLL0_OUT_MAIN: u32 = 1; const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_GPU_CC_PLL0_OUT_MAIN: u32 = 3; const P_GPU_CC_PLL1_OUT_MAIN: u32 = 4;

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];
static zonda_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 700000000, max_freq: 3600000000, val: 0 }];
static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x29, alpha: 0xa000, config_ctl_val: 0x08240800, config_ctl_hi_val: 0x05008001, config_ctl_hi1_val: 0, config_ctl_hi2_val: 0, test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0, test_ctl_hi2_val: 0, user_ctl_val: 0, user_ctl_hi_val: 0x02000000 };
static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x16, alpha: 0xeaaa, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c, config_ctl_hi2_val: 0, test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000, test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5 };

/* The following framework objects retain the C driver's object names, register
 * offsets, parent topology, and frequency data.  Framework-specific layouts are
 * intentionally represented by opaque storage until supplied by the kernel API. */
macro_rules! opaque_clock { ($name:ident) => { static mut $name: clk_regmap = clk_regmap { hw: clk_hw { _private: [] } }; }; }
opaque_clock!(gpu_cc_pll0); opaque_clock!(gpu_cc_pll1); opaque_clock!(gpu_cc_ff_clk_src); opaque_clock!(gpu_cc_gmu_clk_src); opaque_clock!(gpu_cc_hub_clk_src); opaque_clock!(gpu_cc_xo_clk_src); opaque_clock!(gpu_cc_demet_div_clk_src); opaque_clock!(gpu_cc_xo_div_clk_src);
opaque_clock!(gpu_cc_ahb_clk); opaque_clock!(gpu_cc_crc_ahb_clk); opaque_clock!(gpu_cc_cx_ff_clk); opaque_clock!(gpu_cc_cx_gmu_clk); opaque_clock!(gpu_cc_cxo_aon_clk); opaque_clock!(gpu_cc_cxo_clk); opaque_clock!(gpu_cc_demet_clk); opaque_clock!(gpu_cc_freq_measure_clk); opaque_clock!(gpu_cc_hlos1_vote_gpu_smmu_clk); opaque_clock!(gpu_cc_gx_gmu_clk); opaque_clock!(gpu_cc_gx_vsense_clk); opaque_clock!(gpu_cc_hub_aon_clk); opaque_clock!(gpu_cc_hub_cx_int_clk); opaque_clock!(gpu_cc_memnoc_gfx_clk); opaque_clock!(gpu_cc_mnd1x_0_gfx3d_clk); opaque_clock!(gpu_cc_mnd1x_1_gfx3d_clk); opaque_clock!(gpu_cc_sleep_clk);
static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x9108, gds_hw_ctrl: 0x953c, clamp_io_ctrl: 0, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pwrsts: 0x3, flags: 0x3 };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x905c, gds_hw_ctrl: 0, clamp_io_ctrl: 0x9504, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pwrsts: 0x3, flags: 0xf };

/* Parent maps and frequency tables from the original driver. */
static gpu_cc_parent_map_0: [parent_map; 3] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_map_1: [parent_map; 5] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, value: 1 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_map_2: [parent_map; 4] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_map_3: [parent_map; 1] = [parent_map { parent: P_BI_TCXO, value: 0 }];
static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 1] = [freq_tbl { freq: 200000000, src: P_GPLL0_OUT_MAIN, pre_div: 3, m: 0, n: 0 }];
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 3] = [freq_tbl { freq: 19200000, src: P_BI_TCXO, pre_div: 1, m: 0, n: 0 }, freq_tbl { freq: 220000000, src: P_GPU_CC_PLL1_OUT_MAIN, pre_div: 2, m: 0, n: 0 }, freq_tbl { freq: 550000000, src: P_GPU_CC_PLL1_OUT_MAIN, pre_div: 2, m: 0, n: 0 }];

extern "C" {
    fn qcom_cc_map(pdev: *mut c_void, desc: *const c_void) -> *mut c_void;
    fn clk_zonda_pll_configure(pll: *mut c_void, regmap: *mut c_void, config: *const alpha_pll_config);
    fn clk_lucid_evo_pll_configure(pll: *mut c_void, regmap: *mut c_void, config: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(regmap: *mut c_void, reg: u32);
    fn qcom_cc_really_probe(dev: *mut c_void, desc: *const c_void, regmap: *mut c_void) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn gpu_cc_x1e80100_probe(pdev: *mut c_void) -> c_int {
    let regmap = qcom_cc_map(pdev, core::ptr::null());
    if regmap.is_null() { return -1; }
    clk_zonda_pll_configure(&mut gpu_cc_pll0 as *mut _ as *mut c_void, regmap, &gpu_cc_pll0_config);
    clk_lucid_evo_pll_configure(&mut gpu_cc_pll1 as *mut _ as *mut c_void, regmap, &gpu_cc_pll1_config);
    // Keep clocks always enabled: GPU_CC_CB_CLK.
    qcom_branch_set_clk_en(regmap, 0x93a4);
    qcom_cc_really_probe(pdev, core::ptr::null(), regmap)
}

// Platform-driver registration and module metadata are provided by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
