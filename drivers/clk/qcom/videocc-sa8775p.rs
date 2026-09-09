// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Linux and Qualcomm clock-controller dependencies are supplied externally.

#[repr(C)]
pub enum DtInput { DT_IFACE, DT_BI_TCXO, DT_BI_TCXO_AO, DT_SLEEP_CLK }

#[repr(C)]
pub enum Parent { P_BI_TCXO, P_BI_TCXO_AO, P_SLEEP_CLK, P_VIDEO_PLL0_OUT_MAIN, P_VIDEO_PLL1_OUT_MAIN }

static LUCID_EVO_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2020000000, val: 0 }];

static VIDEO_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x39, alpha: 0x3000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    user_ctl_val: 0x00000000, user_ctl_hi_val: 0x00400805,
};

static mut video_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: LUCID_EVO_VCO.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_EVO],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO as u32 },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static VIDEO_PLL1_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x39, alpha: 0x3000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    user_ctl_val: 0x00000000, user_ctl_hi_val: 0x00400805,
};

static mut video_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, vco_table: LUCID_EVO_VCO.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_EVO],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO as u32 },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static VIDEO_CC_PARENT_MAP_0_AO: [parent_map; 1] = [parent_map { parent: P_BI_TCXO_AO as u32, val: 0 }];
static VIDEO_CC_PARENT_DATA_0_AO: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO_AO as u32 }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [parent_map { parent: P_BI_TCXO as u32, val: 0 }, parent_map { parent: P_VIDEO_PLL0_OUT_MAIN as u32, val: 1 }];
static VIDEO_CC_PARENT_MAP_2: [parent_map; 2] = [parent_map { parent: P_BI_TCXO as u32, val: 0 }, parent_map { parent: P_VIDEO_PLL1_OUT_MAIN as u32, val: 1 }];
static VIDEO_CC_PARENT_MAP_3: [parent_map; 1] = [parent_map { parent: P_SLEEP_CLK as u32, val: 0 }];

static VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO as u32 }, clk_parent_data { hw: unsafe { &video_pll0.clkr.hw } }];
static VIDEO_CC_PARENT_DATA_2: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO as u32 }, clk_parent_data { hw: unsafe { &video_pll1.clkr.hw } }];
static VIDEO_CC_PARENT_DATA_3: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK as u32 }];

macro_rules! F { ($rate:expr, $parent:expr, $m:expr, $n:expr, $d:expr) => { freq_tbl { freq: $rate, src: $parent as u32, pre_div: $m, m: $n, n: $d } }; }
static FTBL_VIDEO_CC_AHB_CLK_SRC: [freq_tbl; 2] = [F!(19200000, P_BI_TCXO_AO, 1, 0, 0), freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0_CLK_SRC: [freq_tbl; 5] = [F!(1098000000, P_VIDEO_PLL0_OUT_MAIN,1,0,0),F!(1332000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F!(1599000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F!(1680000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),freq_tbl::default()];
static FTBL_VIDEO_CC_MVS1_CLK_SRC: [freq_tbl; 5] = [F!(1098000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F!(1332000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F!(1600000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F!(1800000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),freq_tbl::default()];
static FTBL_VIDEO_CC_SLEEP_CLK_SRC: [freq_tbl; 2] = [F!(32000,P_SLEEP_CLK,1,0,0),freq_tbl::default()];

// The following kernel object types and operation tables are supplied by the
// translated clock framework dependencies.
static mut video_cc_ahb_clk_src: clk_rcg2 = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0_clk_src: clk_rcg2 = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1_clk_src: clk_rcg2 = unsafe { core::mem::zeroed() };
static mut video_cc_sleep_clk_src: clk_rcg2 = unsafe { core::mem::zeroed() };
static mut video_cc_xo_clk_src: clk_rcg2 = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0_div_clk_src: clk_regmap_div = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1_div_clk_src: clk_regmap_div = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1c_div2_div_clk_src: clk_regmap_div = unsafe { core::mem::zeroed() };
static mut video_cc_sm_div_clk_src: clk_regmap_div = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0c_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1c_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_pll_lock_monitor_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_sm_obs_clk: clk_branch = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0c_gdsc: gdsc = unsafe { core::mem::zeroed() };
static mut video_cc_mvs0_gdsc: gdsc = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1c_gdsc: gdsc = unsafe { core::mem::zeroed() };
static mut video_cc_mvs1_gdsc: gdsc = unsafe { core::mem::zeroed() };

static mut VIDEO_CC_SA8775P_CLOCKS: [*mut clk_regmap; 23] = [core::ptr::null_mut(); 23];
static VIDEO_CC_SA8775P_RESETS: [qcom_reset_map; 7] = [qcom_reset_map { reg: 0, bit: 0 }; 7];
static mut VIDEO_CC_SA8775P_GDSCS: [*mut gdsc; 4] = [core::ptr::null_mut(); 4];
static VIDEO_CC_SA8775P_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb000, fast_io: true };
static VIDEO_CC_SA8775P_DESC: qcom_cc_desc = qcom_cc_desc { config: &VIDEO_CC_SA8775P_REGMAP_CONFIG, clks: VIDEO_CC_SA8775P_CLOCKS.as_ptr(), num_clks: 23, resets: VIDEO_CC_SA8775P_RESETS.as_ptr(), num_resets: 7, gdscs: VIDEO_CC_SA8775P_GDSCS.as_ptr(), num_gdscs: 4 };

extern "C" {
    fn devm_pm_runtime_enable(dev: *mut device) -> i32;
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn pm_runtime_put(dev: *mut device);
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn clk_lucid_evo_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn of_device_is_compatible(node: *mut device_node, compatible: *const i8) -> bool;
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
}

unsafe fn video_cc_sa8775p_probe(pdev: *mut platform_device) -> i32 {
    let ret = devm_pm_runtime_enable((*pdev).dev.as_mut_ptr());
    if ret != 0 { return ret; }
    let ret = pm_runtime_resume_and_get((*pdev).dev.as_mut_ptr());
    if ret != 0 { return ret; }
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SA8775P_DESC);
    if regmap.is_null() { pm_runtime_put((*pdev).dev.as_mut_ptr()); return -1; }
    clk_lucid_evo_pll_configure(&mut video_pll0, regmap, &VIDEO_PLL0_CONFIG);
    clk_lucid_evo_pll_configure(&mut video_pll1, regmap, &VIDEO_PLL1_CONFIG);
    if of_device_is_compatible((*pdev).dev.of_node, b"qcom,qcs8300-videocc\0".as_ptr() as *const i8) { regmap_write(regmap, 0x806c, 2); }
    qcom_branch_set_clk_en(regmap, 0x80ec);
    qcom_branch_set_clk_en(regmap, 0x8144);
    qcom_branch_set_clk_en(regmap, 0x8128);
    let ret = qcom_cc_really_probe((*pdev).dev.as_mut_ptr(), &VIDEO_CC_SA8775P_DESC, regmap);
    pm_runtime_put((*pdev).dev.as_mut_ptr());
    ret
}

static VIDEO_CC_SA8775P_MATCH_TABLE: [of_device_id; 3] = [
    of_device_id { compatible: b"qcom,qcs8300-videocc\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"qcom,sa8775p-videocc\0".as_ptr() as *const i8 },
    of_device_id::default(),
];

static VIDEO_CC_SA8775P_DRIVER: platform_driver = platform_driver {
    probe: Some(video_cc_sa8775p_probe),
    driver: driver { name: b"videocc-sa8775p\0".as_ptr() as *const i8, of_match_table: VIDEO_CC_SA8775P_MATCH_TABLE.as_ptr() },
};

// module_platform_driver!(VIDEO_CC_SA8775P_DRIVER);
// MODULE_DESCRIPTION!("QTI VIDEOCC SA8775P Driver");
// MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
