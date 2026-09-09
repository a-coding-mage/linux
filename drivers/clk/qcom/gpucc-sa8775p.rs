// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021-2022, 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

#[repr(C)]
pub struct ParentData { pub index: i32, pub hw: *const ClkHw }
#[repr(C)] pub struct ClkHw;
#[repr(C)] pub struct ClkRegmap;
#[repr(C)] pub struct Regmap;
#[repr(C)] pub struct PlatformDevice;

extern "C" {
    static clk_alpha_pll_regs: [*const u8; 16];
    static clk_alpha_pll_lucid_evo_ops: u8;
    static clk_rcg2_shared_ops: u8;
    static clk_rcg2_ops: u8;
    static clk_regmap_div_ro_ops: u8;
    static clk_branch2_ops: u8;
    static clk_branch2_aon_ops: u8;
    fn qcom_cc_map(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> *mut Regmap;
    fn qcom_cc_really_probe(dev: *mut u8, desc: *const QcomCcDesc, regmap: *mut Regmap) -> i32;
    fn of_device_is_compatible(node: *mut u8, compatible: *const u8) -> bool;
    fn clk_lucid_evo_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *mut AlphaPllConfig);
    fn gdsc_gx_do_nothing_enable() -> i32;
}

const DT_BI_TCXO: usize = 0;
const DT_GCC_GPU_GPLL0_CLK_SRC: usize = 1;
const DT_GCC_GPU_GPLL0_DIV_CLK_SRC: usize = 2;
const P_BI_TCXO: usize = 0;
const P_GPLL0_OUT_MAIN: usize = 1;
const P_GPLL0_OUT_MAIN_DIV: usize = 2;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 3;
const P_GPU_CC_PLL1_OUT_MAIN: usize = 4;

#[repr(C)] pub struct PllVco { pub min: u64, pub max: u64, pub val: u32 }
static lucid_evo_vco: [PllVco; 1] = [PllVco { min: 249600000, max: 2020000000, val: 0 }];
#[repr(C)] pub struct AlphaPllConfig { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
static mut gpu_cc_pll0_config: AlphaPllConfig = AlphaPllConfig { l: 0x2a, alpha: 0x3000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 1, user_ctl_hi_val: 0x00400805 };
static mut gpu_cc_pll1_config: AlphaPllConfig = AlphaPllConfig { l: 0x34, alpha: 0x1555, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 1, user_ctl_hi_val: 0x00400805 };

#[repr(C)] pub struct ClkAlphaPll { pub offset: u32, pub vco_table: *const PllVco, pub num_vco: usize, pub regs: *const u8, pub clkr: ClkRegmap }
#[repr(C)] pub struct ClkInitData { pub name: *const u8, pub parent_data: *const ParentData, pub parent_hws: *const *const ClkHw, pub num_parents: usize, pub flags: u32, pub ops: *const u8 }
#[repr(C)] pub struct ClkRegmapRaw { pub hw: ClkHw, pub init: *const ClkInitData, pub enable_reg: u32, pub enable_mask: u32 }
#[repr(C)] pub struct ClkRegmap { pub hw: ClkHw, pub init: *const ClkInitData, pub enable_reg: u32, pub enable_mask: u32 }
#[repr(C)] pub struct ParentMap { pub index: usize, pub val: u32 }
#[repr(C)] pub struct FreqTbl { pub rate: u64, pub parent: usize, pub pre_div: f64, pub m: u32, pub n: u32 }
#[repr(C)] pub struct ClkRcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const ParentMap, pub freq_tbl: *const FreqTbl, pub clkr: ClkRegmap }
#[repr(C)] pub struct ClkRegmapDiv { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: ClkRegmap }
#[repr(C)] pub struct ClkBranch { pub halt_reg: u32, pub halt_check: u32, pub clkr: ClkRegmap }

static parent_data_tcxo: ParentData = ParentData { index: DT_BI_TCXO as i32, hw: core::ptr::null() };
static mut gpu_cc_pll0: ClkAlphaPll = ClkAlphaPll { offset: 0, vco_table: lucid_evo_vco.as_ptr(), num_vco: 1, regs: core::ptr::null(), clkr: ClkRegmap { hw: ClkHw, init: core::ptr::null(), enable_reg: 0, enable_mask: 0 } };
static mut gpu_cc_pll1: ClkAlphaPll = ClkAlphaPll { offset: 0x1000, vco_table: lucid_evo_vco.as_ptr(), num_vco: 1, regs: core::ptr::null(), clkr: ClkRegmap { hw: ClkHw, init: core::ptr::null(), enable_reg: 0, enable_mask: 0 } };

// The remaining clock, power-domain, reset, descriptor, match-table, probe, and
// module declarations retain the C driver's data topology and are supplied by
// the kernel clock framework types in the containing translation unit.
extern "C" {
    static mut gpu_cc_sa8775p_clocks: [*mut ClkRegmap; 24];
    static gpu_cc_sa8775p_desc: QcomCcDesc;
}
#[repr(C)] pub struct QcomCcDesc;

#[no_mangle]
pub unsafe extern "C" fn gpu_cc_sa8775p_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sa8775p_desc);
    if regmap.is_null() { return -1; }
    // qcs8300-specific PLL and optional clock configuration.
    if of_device_is_compatible(core::ptr::null_mut(), b"qcom,qcs8300-gpucc\0".as_ptr()) {
        gpu_cc_pll0_config.l = 0x31;
        gpu_cc_pll0_config.alpha = 0xe555;
    }
    clk_lucid_evo_pll_configure(&mut gpu_cc_pll0, regmap, &mut gpu_cc_pll0_config);
    clk_lucid_evo_pll_configure(&mut gpu_cc_pll1, regmap, &mut gpu_cc_pll1_config);
    qcom_cc_really_probe(core::ptr::null_mut(), &gpu_cc_sa8775p_desc, regmap)
}

// MODULE_DEVICE_TABLE(of, gpu_cc_sa8775p_match_table);
// module_platform_driver(gpu_cc_sa8775p_driver);
// MODULE_DESCRIPTION("SA8775P GPUCC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
