// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct low-level Rust translation of the Qualcomm Kaanapali VIDEOCC
 * implementation.  Kernel-provided types, constants, macros, and operations
 * are intentionally referenced as external dependencies.
 */

const ACCU_CFG_MASK: u32 = genmask(25, 21);

#[repr(u32)]
enum DtClock {
    DT_BI_TCXO,
    DT_AHB_CLK,
}

#[repr(u32)]
enum Parent {
    P_BI_TCXO,
    P_VIDEO_CC_PLL0_OUT_MAIN,
    P_VIDEO_CC_PLL1_OUT_MAIN,
    P_VIDEO_CC_PLL2_OUT_MAIN,
    P_VIDEO_CC_PLL3_OUT_MAIN,
}

static TAYCAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

macro_rules! F { ($r:expr, $p:expr, $n:expr, $m:expr, $d:expr) => { freq_tbl { freq: $r, src: $p, pre_div: $n, m: $m, n: $d } }; }
macro_rules! ARRAY_SIZE { ($x:expr) => { $x.len() }; }
macro_rules! BIT { ($x:expr) => { 1u32 << $x }; }

static VIDEO_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x12, cal_l: 0x48, alpha: 0xc000, config_ctl_val: 0x25c400e7, config_ctl_hi_val: 0x0a8062e0, config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 8, user_ctl_hi_val: 2 };
static VIDEO_CC_PLL1_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x19, cal_l: 0x48, alpha: 0, config_ctl_val: 0x25c400e7, config_ctl_hi_val: 0x0a8062e0, config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 8, user_ctl_hi_val: 2 };
static VIDEO_CC_PLL2_CONFIG: alpha_pll_config = VIDEO_CC_PLL1_CONFIG;
static VIDEO_CC_PLL3_CONFIG: alpha_pll_config = VIDEO_CC_PLL1_CONFIG;

// PLL objects retain the C layout and external kernel operation tables.
static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll { offset: 0x0, config: &VIDEO_CC_PLL0_CONFIG, vco_table: &TAYCAN_EKO_T_VCO, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init("video_cc_pll0", DT_BI_TCXO as u32, &clk_alpha_pll_taycan_eko_t_ops) } };
static mut video_cc_pll1: clk_alpha_pll = clk_alpha_pll { offset: 0x1000, config: &VIDEO_CC_PLL1_CONFIG, vco_table: &TAYCAN_EKO_T_VCO, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init("video_cc_pll1", DT_BI_TCXO as u32, &clk_alpha_pll_taycan_eko_t_ops) } };
static mut video_cc_pll2: clk_alpha_pll = clk_alpha_pll { offset: 0x2000, config: &VIDEO_CC_PLL2_CONFIG, vco_table: &TAYCAN_EKO_T_VCO, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init("video_cc_pll2", DT_BI_TCXO as u32, &clk_alpha_pll_taycan_eko_t_ops) } };
static mut video_cc_pll3: clk_alpha_pll = clk_alpha_pll { offset: 0x3000, config: &VIDEO_CC_PLL3_CONFIG, vco_table: &TAYCAN_EKO_T_VCO, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init("video_cc_pll3", DT_BI_TCXO as u32, &clk_alpha_pll_taycan_eko_t_ops) } };

static VIDEO_CC_PARENT_MAP_0: [parent_map; 1] = [parent_map { src: P_BI_TCXO as u32, cfg: 0 }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO as u32, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL1_OUT_MAIN as u32, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_2: [parent_map; 2] = [parent_map { src: P_BI_TCXO as u32, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL3_OUT_MAIN as u32, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_3: [parent_map; 2] = [parent_map { src: P_BI_TCXO as u32, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL2_OUT_MAIN as u32, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_4: [parent_map; 2] = [parent_map { src: P_BI_TCXO as u32, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN as u32, cfg: 1 }];

static FTBL_VIDEO_CC_AHB_CLK_SRC: [freq_tbl; 2] = [F!(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0_CLK_SRC: [freq_tbl; 9] = [F!(240000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(338000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(420000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(444000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(533000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(630000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(800000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),F!(1000000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0A_CLK_SRC: [freq_tbl; 7] = [F!(240000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),F!(338000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),F!(420000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),F!(444000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),F!(533000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),F!(630000000,P_VIDEO_CC_PLL3_OUT_MAIN,2,0,0),freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0B_CLK_SRC: [freq_tbl; 8] = [F!(240000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(338000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(420000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(444000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(533000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(630000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),F!(850000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0C_CLK_SRC: [freq_tbl; 8] = [F!(360000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(507000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(630000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(666000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(800000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(1104000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),F!(1260000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),freq_tbl::default()];

// RCG, branch, memory-branch, and GDSC declarations preserve the exact
// register offsets, parent relationships, flags, and operation tables.
static mut video_cc_ahb_clk_src: clk_rcg2 = rcg2(0x8060, &VIDEO_CC_PARENT_MAP_0, &FTBL_VIDEO_CC_AHB_CLK_SRC, "video_cc_ahb_clk_src", false);
static mut video_cc_mvs0_clk_src: clk_rcg2 = rcg2(0x8030, &VIDEO_CC_PARENT_MAP_1, &FTBL_VIDEO_CC_MVS0_CLK_SRC, "video_cc_mvs0_clk_src", true);
static mut video_cc_mvs0a_clk_src: clk_rcg2 = rcg2(0x8000, &VIDEO_CC_PARENT_MAP_2, &FTBL_VIDEO_CC_MVS0A_CLK_SRC, "video_cc_mvs0a_clk_src", true);
static mut video_cc_mvs0b_clk_src: clk_rcg2 = rcg2(0x8018, &VIDEO_CC_PARENT_MAP_3, &FTBL_VIDEO_CC_MVS0B_CLK_SRC, "video_cc_mvs0b_clk_src", true);
static mut video_cc_mvs0c_clk_src: clk_rcg2 = rcg2(0x8048, &VIDEO_CC_PARENT_MAP_4, &FTBL_VIDEO_CC_MVS0C_CLK_SRC, "video_cc_mvs0c_clk_src", true);
static mut video_cc_xo_clk_src: clk_rcg2 = rcg2(0x8194, &VIDEO_CC_PARENT_MAP_0, &FTBL_VIDEO_CC_AHB_CLK_SRC, "video_cc_xo_clk_src", false);

static mut video_cc_mvs0_clk: clk_branch = branch(0x80d0, BRANCH_HALT_VOTED, 1, "video_cc_mvs0_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_freerun_clk: clk_mem_branch = mem_branch(0x80e0, 0x80e4, 0x80e4, BIT!(3), genmask(11,10), true, "video_cc_mvs0_freerun_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_shift_clk: clk_branch = branch(0x81b4, BRANCH_HALT_VOTED, 1, "video_cc_mvs0_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw });
static mut video_cc_mvs0_vpp0_clk: clk_branch = branch(0x8134, BRANCH_HALT_VOTED, 1, "video_cc_mvs0_vpp0_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_vpp0_freerun_clk: clk_branch = branch(0x8144, BRANCH_HALT, 0, "video_cc_mvs0_vpp0_freerun_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_vpp1_clk: clk_branch = branch(0x8108, BRANCH_HALT_VOTED, 1, "video_cc_mvs0_vpp1_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0_vpp1_freerun_clk: clk_branch = branch(0x8118, BRANCH_HALT, 0, "video_cc_mvs0_vpp1_freerun_clk", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0a_clk: clk_branch = branch(0x8090, BRANCH_HALT_VOTED, 1, "video_cc_mvs0a_clk", unsafe { &video_cc_mvs0a_clk_src.clkr.hw });
static mut video_cc_mvs0a_freerun_clk: clk_branch = branch(0x80a0, BRANCH_HALT, 0, "video_cc_mvs0a_freerun_clk", unsafe { &video_cc_mvs0a_clk_src.clkr.hw });
static mut video_cc_mvs0b_clk: clk_branch = branch(0x80bc, BRANCH_HALT_VOTED, 1, "video_cc_mvs0b_clk", unsafe { &video_cc_mvs0b_clk_src.clkr.hw });
static mut video_cc_mvs0b_freerun_clk: clk_branch = branch(0x80cc, BRANCH_HALT, 0, "video_cc_mvs0b_freerun_clk", unsafe { &video_cc_mvs0b_clk_src.clkr.hw });
static mut video_cc_mvs0c_clk: clk_branch = branch(0x8164, BRANCH_HALT_VOTED, 1, unsafe { "video_cc_mvs0c_clk" }, unsafe { &video_cc_mvs0c_clk_src.clkr.hw });
static mut video_cc_mvs0c_freerun_clk: clk_branch = branch(0x8174, BRANCH_HALT, 0, "video_cc_mvs0c_freerun_clk", unsafe { &video_cc_mvs0c_clk_src.clkr.hw });
static mut video_cc_mvs0c_shift_clk: clk_branch = branch(0x81b8, BRANCH_HALT_VOTED, 1, "video_cc_mvs0c_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw });

static mut video_cc_mvs0_vpp0_gdsc: gdsc = gdsc_init(0x8120, 2, 2, 0xf, "video_cc_mvs0_vpp0_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);
static mut video_cc_mvs0_vpp1_gdsc: gdsc = gdsc_init(0x80f4, 2, 2, 0xf, "video_cc_mvs0_vpp1_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);
static mut video_cc_mvs0a_gdsc: gdsc = gdsc_init(0x807c, 2, 2, 0xf, "video_cc_mvs0a_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);
static mut video_cc_mvs0c_gdsc: gdsc = gdsc_init(0x814c, 2, 2, 6, "video_cc_mvs0c_gdsc", PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);
static mut video_cc_mvs0_gdsc: gdsc = gdsc_init(0x80a8, 2, 2, 6, "video_cc_mvs0_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, Some(unsafe { &video_cc_mvs0c_gdsc.pd }));

static mut video_cc_kaanapali_clocks: [*mut clk_regmap; 25] = [
    unsafe { &mut video_cc_ahb_clk_src.clkr }, unsafe { &mut video_cc_mvs0_clk.clkr }, unsafe { &mut video_cc_mvs0_clk_src.clkr }, unsafe { &mut video_cc_mvs0_freerun_clk.branch.clkr }, unsafe { &mut video_cc_mvs0_shift_clk.clkr }, unsafe { &mut video_cc_mvs0_vpp0_clk.clkr }, unsafe { &mut video_cc_mvs0_vpp0_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0_vpp1_clk.clkr }, unsafe { &mut video_cc_mvs0_vpp1_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0a_clk.clkr }, unsafe { &mut video_cc_mvs0a_clk_src.clkr }, unsafe { &mut video_cc_mvs0a_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0b_clk.clkr }, unsafe { &mut video_cc_mvs0b_clk_src.clkr }, unsafe { &mut video_cc_mvs0b_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0c_clk.clkr }, unsafe { &mut video_cc_mvs0c_clk_src.clkr }, unsafe { &mut video_cc_mvs0c_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0c_shift_clk.clkr }, unsafe { &mut video_cc_pll0.clkr }, unsafe { &mut video_cc_pll1.clkr }, unsafe { &mut video_cc_pll2.clkr }, unsafe { &mut video_cc_pll3.clkr }, unsafe { &mut video_cc_xo_clk_src.clkr }, core::ptr::null_mut(),
];
static mut video_cc_kaanapali_gdscs: [*mut gdsc; 5] = [unsafe { &mut video_cc_mvs0a_gdsc }, unsafe { &mut video_cc_mvs0_gdsc }, unsafe { &mut video_cc_mvs0_vpp1_gdsc }, unsafe { &mut video_cc_mvs0_vpp0_gdsc }, unsafe { &mut video_cc_mvs0c_gdsc }];
static video_cc_kaanapali_resets: [qcom_reset_map; 10] = [qcom_reset_map { reg: 0x8178, bit: 0 }, qcom_reset_map { reg: 0x80a4, bit: 0 }, qcom_reset_map { reg: 0x811c, bit: 0 }, qcom_reset_map { reg: 0x80f0, bit: 0 }, qcom_reset_map { reg: 0x8078, bit: 0 }, qcom_reset_map { reg: 0x8164, bit: 2 }, qcom_reset_map { reg: 0x8148, bit: 0 }, qcom_reset_map { reg: 0x80e0, bit: 2 }, qcom_reset_map { reg: 0x8174, bit: 2 }, qcom_reset_map { reg: 0x81ac, bit: 2 }];
static mut video_cc_kaanapali_plls: [*mut clk_alpha_pll; 4] = [unsafe { &mut video_cc_pll0 }, unsafe { &mut video_cc_pll1 }, unsafe { &mut video_cc_pll2 }, unsafe { &mut video_cc_pll3 }];
static VIDEO_CC_KAANAPALI_CRITICAL_CBCRS: [u32; 4] = [0x817c, 0x81bc, 0x81b0, 0x81ac];

static VIDEO_CC_KAANAPALI_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xa010, fast_io: true };

unsafe fn clk_kaanapali_regs_configure(dev: *mut device, regmap: *mut regmap) {
    // Enable clk_on sync for MVS0 and VPP clocks via VIDEO_CC_SPARE1 during core reset by default.
    regmap_set_bits(regmap, 0x9f24, BIT!(0));
    // As per HW design recommendation, update DLY_ACCU_RED_SHIFTER_DONE to 0xF for each GDSC CFG3.
    regmap_set_bits(regmap, 0x8088, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x80b4, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x8100, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x812c, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x8158, ACCU_CFG_MASK);
}

static VIDEO_CC_KAANAPALI_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &video_cc_kaanapali_plls, num_alpha_plls: 4, clk_cbcrs: &VIDEO_CC_KAANAPALI_CRITICAL_CBCRS, num_clk_cbcrs: 4, clk_regs_configure: Some(clk_kaanapali_regs_configure) };
static VIDEO_CC_KAANAPALI_DESC: qcom_cc_desc = qcom_cc_desc { config: &VIDEO_CC_KAANAPALI_REGMAP_CONFIG, clks: &video_cc_kaanapali_clocks, num_clks: 25, resets: &video_cc_kaanapali_resets, num_resets: 10, gdscs: &video_cc_kaanapali_gdscs, num_gdscs: 5, use_rpm: true, driver_data: &VIDEO_CC_KAANAPALI_DRIVER_DATA };

static VIDEO_CC_KAANAPALI_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,kaanapali-videocc" }, of_device_id::default()];

unsafe fn video_cc_kaanapali_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &VIDEO_CC_KAANAPALI_DESC) }

static VIDEO_CC_KAANAPALI_DRIVER: platform_driver = platform_driver { probe: Some(video_cc_kaanapali_probe), name: "videocc-kaanapali", of_match_table: &VIDEO_CC_KAANAPALI_MATCH_TABLE };

// Equivalent of module_platform_driver(video_cc_kaanapali_driver).
module_platform_driver!(VIDEO_CC_KAANAPALI_DRIVER);
module_description!("QTI VIDEOCC Kaanapali Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
