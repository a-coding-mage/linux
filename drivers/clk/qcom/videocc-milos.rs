// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

// Kernel clock-provider, module, platform-device, regmap, Qualcomm clock and
// device-tree bindings are supplied by the surrounding kernel Rust bindings.

#[repr(C)]
pub enum DtClock { DT_BI_TCXO, DT_BI_TCXO_AO, DT_SLEEP_CLK, DT_IFACE }
#[repr(C)]
pub enum Parent { P_BI_TCXO, P_SLEEP_CLK, P_VIDEO_CC_PLL0_OUT_MAIN }

static LUCID_OLE_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];

static VIDEO_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x1f, alpha: 0x8000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, config: &VIDEO_CC_PLL0_CONFIG, vco_table: &LUCID_OLE_VCO,
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "video_cc_pll0", parent_data: &clk_parent_data { index: DtClock::DT_BI_TCXO as u32 },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static VIDEO_CC_PARENT_MAP_0: [parent_map; 1] = [parent_map { src: Parent::P_BI_TCXO as u32, cfg: 0 }];
static VIDEO_CC_PARENT_DATA_0: [clk_parent_data; 1] = [clk_parent_data { index: DtClock::DT_BI_TCXO as u32 }];
static VIDEO_CC_PARENT_DATA_0_AO: [clk_parent_data; 1] = [clk_parent_data { index: DtClock::DT_BI_TCXO_AO as u32 }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [
    parent_map { src: Parent::P_BI_TCXO as u32, cfg: 0 }, parent_map { src: Parent::P_VIDEO_CC_PLL0_OUT_MAIN as u32, cfg: 1 }
];
static mut VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [
    clk_parent_data { index: DtClock::DT_BI_TCXO as u32 }, clk_parent_data { hw: unsafe { &video_cc_pll0.clkr.hw } }
];
static VIDEO_CC_PARENT_MAP_2: [parent_map; 1] = [parent_map { src: Parent::P_SLEEP_CLK as u32, cfg: 0 }];
static VIDEO_CC_PARENT_DATA_2_AO: [clk_parent_data; 1] = [clk_parent_data { index: DtClock::DT_SLEEP_CLK as u32 }];

macro_rules! freq { ($f:expr, $p:expr) => { freq_tbl { freq: $f, src: $p as u32, pre_div: 1, m: 0, n: 0 } }; }
static FTBL_AHB: [freq_tbl; 2] = [freq!(19200000, Parent::P_BI_TCXO), freq_tbl::ZERO];
static FTBL_MVS0: [freq_tbl; 7] = [
    freq!(604800000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN), freq!(720000000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN),
    freq!(1014000000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN), freq!(1098000000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN),
    freq!(1332000000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN), freq!(1656000000, Parent::P_VIDEO_CC_PLL0_OUT_MAIN), freq_tbl::ZERO
];
static FTBL_SLEEP: [freq_tbl; 2] = [freq!(32000, Parent::P_SLEEP_CLK), freq_tbl::ZERO];

// The following declarations preserve the literal clock topology and register layout.
static mut video_cc_ahb_clk_src: clk_rcg2 = clk_rcg2::new(0x8030, 0, 5, &VIDEO_CC_PARENT_MAP_0, &FTBL_AHB, "video_cc_ahb_clk_src", &VIDEO_CC_PARENT_DATA_0_AO, &clk_rcg2_shared_ops);
static mut video_cc_mvs0_clk_src: clk_rcg2 = clk_rcg2::new(0x8000, 0, 5, &VIDEO_CC_PARENT_MAP_1, &FTBL_MVS0, "video_cc_mvs0_clk_src", unsafe { &VIDEO_CC_PARENT_DATA_1 }, &clk_rcg2_shared_ops);
static mut video_cc_sleep_clk_src: clk_rcg2 = clk_rcg2::new(0x8128, 0, 5, &VIDEO_CC_PARENT_MAP_2, &FTBL_SLEEP, "video_cc_sleep_clk_src", &VIDEO_CC_PARENT_DATA_2_AO, &clk_rcg2_ops);
static mut video_cc_xo_clk_src: clk_rcg2 = clk_rcg2::new(0x810c, 0, 5, &VIDEO_CC_PARENT_MAP_0, &FTBL_AHB, "video_cc_xo_clk_src", &VIDEO_CC_PARENT_DATA_0, &clk_rcg2_ops);

static mut video_cc_mvs0_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x80c4, 0, 4, "video_cc_mvs0_div_clk_src", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x8070, 0, 4, "video_cc_mvs0c_div2_div_clk_src", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_clk: clk_branch = clk_branch::new(0x80b8, BRANCH_HALT_VOTED, 0x80b8, 1, 0, "video_cc_mvs0_clk", unsafe { &video_cc_mvs0_div_clk_src.clkr.hw });
static mut video_cc_mvs0_shift_clk: clk_branch = clk_branch::new(0x8144, BRANCH_HALT_VOTED, 0x8144, 1, 0, "video_cc_mvs0_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw });
static mut video_cc_mvs0c_clk: clk_branch = clk_branch::new(0x8064, BRANCH_HALT, 0, 0, 0, "video_cc_mvs0c_clk", unsafe { &video_cc_mvs0c_div2_div_clk_src.clkr.hw });
static mut video_cc_mvs0c_shift_clk: clk_branch = clk_branch::new(0x8148, BRANCH_HALT_VOTED, 0x8148, 1, 0, "video_cc_mvs0c_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw });

static mut video_cc_mvs0c_gdsc: gdsc = gdsc::new(0x804c, 2, 2, 6, "video_cc_mvs0c_gdsc", PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE);
static mut video_cc_mvs0_gdsc: gdsc = gdsc::with_parent(0x80a4, 2, 2, 6, "video_cc_mvs0_gdsc", unsafe { &video_cc_mvs0c_gdsc.pd }, PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE | HW_CTRL_TRIGGER);

static mut video_cc_milos_clocks: [*mut clk_regmap; 11] = [
    &mut video_cc_ahb_clk_src.clkr, &mut video_cc_mvs0_clk.clkr, &mut video_cc_mvs0_clk_src.clkr,
    &mut video_cc_mvs0_div_clk_src.clkr, &mut video_cc_mvs0_shift_clk.clkr, &mut video_cc_mvs0c_clk.clkr,
    &mut video_cc_mvs0c_div2_div_clk_src.clkr, &mut video_cc_mvs0c_shift_clk.clkr, &mut video_cc_pll0.clkr,
    &mut video_cc_sleep_clk_src.clkr, &mut video_cc_xo_clk_src.clkr,
];
static mut video_cc_milos_gdscs: [*mut gdsc; 2] = [&mut video_cc_mvs0c_gdsc, &mut video_cc_mvs0_gdsc];
static VIDEO_CC_MILOS_RESETS: [qcom_reset_map; 4] = [qcom_reset_map { reg: 0x80f0, bit: 0 }, qcom_reset_map { reg: 0x80a0, bit: 0 }, qcom_reset_map { reg: 0x8064, bit: 2 }, qcom_reset_map { reg: 0x8048, bit: 0 }];
static mut VIDEO_CC_MILOS_PLLS: [*mut clk_alpha_pll; 1] = [&mut video_cc_pll0];
static VIDEO_CC_MILOS_CRITICAL_CBCRS: [u32; 3] = [0x80f4, 0x8140, 0x8124];

static VIDEO_CC_MILOS_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f50, fast_io: true };
static VIDEO_CC_MILOS_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: unsafe { &VIDEO_CC_MILOS_PLLS }, num_alpha_plls: 1, clk_cbcrs: &VIDEO_CC_MILOS_CRITICAL_CBCRS, num_clk_cbcrs: 3 };
static VIDEO_CC_MILOS_DESC: qcom_cc_desc = qcom_cc_desc { config: &VIDEO_CC_MILOS_REGMAP_CONFIG, clks: unsafe { &video_cc_milos_clocks }, num_clks: 11, resets: &VIDEO_CC_MILOS_RESETS, num_resets: 4, gdscs: unsafe { &video_cc_milos_gdscs }, num_gdscs: 2, use_rpm: true, driver_data: &VIDEO_CC_MILOS_DRIVER_DATA };

#[no_mangle]
pub unsafe extern "C" fn video_cc_milos_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &VIDEO_CC_MILOS_DESC) }

// Equivalent platform-driver/module registration and OF match table.
static VIDEO_CC_MILOS_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,milos-videocc" }, of_device_id::ZERO];
static mut video_cc_milos_driver: platform_driver = platform_driver::new("video_cc-milos", video_cc_milos_probe, &VIDEO_CC_MILOS_MATCH_TABLE);
module_platform_driver!(video_cc_milos_driver);
module_description!("QTI VIDEO_CC Milos Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
