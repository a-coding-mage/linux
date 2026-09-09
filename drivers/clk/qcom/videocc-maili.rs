// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// C headers and symbols are supplied by the surrounding kernel translation unit.
use crate::*;

const ACCU_CFG_MASK: u32 = genmask(25, 21);

const DT_BI_TCXO: usize = 0;
const DT_AHB_CLK: usize = 1;
const P_BI_TCXO: usize = 0;
const P_VIDEO_CC_PLL0_OUT_MAIN: usize = 1;
const P_VIDEO_CC_PLL1_OUT_MAIN: usize = 2;
const P_VIDEO_CC_PLL2_OUT_MAIN: usize = 3;

static TAYCAN_EHA_T_VCO: [PllVco; 1] = [PllVco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

static VIDEO_CC_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig { l: 0x12, cal_l: 0x42, alpha: 0xc000, config_ctl_val: 0xa5c400e7, config_ctl_hi_val: 0x0a806160, config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 0, user_ctl_hi_val: 2 };
static VIDEO_CC_PLL1_CONFIG: AlphaPllConfig = AlphaPllConfig { l: 0x19, cal_l: 0x42, alpha: 0, config_ctl_val: 0xa5c400e7, config_ctl_hi_val: 0x0a806160, config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 0, user_ctl_hi_val: 2 };
static VIDEO_CC_PLL2_CONFIG: AlphaPllConfig = VIDEO_CC_PLL1_CONFIG;

static mut video_cc_pll0: ClkAlphaPll = ClkAlphaPll { offset: 0x0, config: &VIDEO_CC_PLL0_CONFIG, vco_table: &TAYCAN_EHA_T_VCO, num_vco: TAYCAN_EHA_T_VCO.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T], clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData { name: "video_cc_pll0", parent_data: &[ClkParentData { index: DT_BI_TCXO, hw: None }], flags: 0, ops: &clk_alpha_pll_taycan_eha_t_ops } } } };
static mut video_cc_pll1: ClkAlphaPll = ClkAlphaPll { offset: 0x1000, config: &VIDEO_CC_PLL1_CONFIG, vco_table: &TAYCAN_EHA_T_VCO, num_vco: TAYCAN_EHA_T_VCO.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T], clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData { name: "video_cc_pll1", parent_data: &[ClkParentData { index: DT_BI_TCXO, hw: None }], flags: 0, ops: &clk_alpha_pll_taycan_eha_t_ops } } } };
static mut video_cc_pll2: ClkAlphaPll = ClkAlphaPll { offset: 0x2000, config: &VIDEO_CC_PLL2_CONFIG, vco_table: &TAYCAN_EHA_T_VCO, num_vco: TAYCAN_EHA_T_VCO.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T], clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData { name: "video_cc_pll2", parent_data: &[ClkParentData { index: DT_BI_TCXO, hw: None }], flags: 0, ops: &clk_alpha_pll_taycan_eha_t_ops } } } };

// The following tables and clock descriptors retain the C driver's literal topology.
static VIDEO_CC_PARENT_MAP_0: [ParentMap; 1] = [ParentMap { src: P_BI_TCXO, cfg: 0 }];
static VIDEO_CC_PARENT_MAP_1: [ParentMap; 2] = [ParentMap { src: P_BI_TCXO, cfg: 0 }, ParentMap { src: P_VIDEO_CC_PLL1_OUT_MAIN, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_2: [ParentMap; 2] = [ParentMap { src: P_BI_TCXO, cfg: 0 }, ParentMap { src: P_VIDEO_CC_PLL2_OUT_MAIN, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_3: [ParentMap; 2] = [ParentMap { src: P_BI_TCXO, cfg: 0 }, ParentMap { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];

static VIDEO_CC_AHB_FREQ: [FreqTbl; 2] = [FREQ!(19200000, P_BI_TCXO, 1, 0, 0), FREQ_END];
static VIDEO_CC_MVS0_FREQ: [FreqTbl; 9] = [FREQ!(240000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(338000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(420000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(444000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(600000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(630000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(800000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ!(1000000000,P_VIDEO_CC_PLL1_OUT_MAIN,2,0,0),FREQ_END];
static VIDEO_CC_MVS0B_FREQ: [FreqTbl; 8] = [FREQ!(240000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(338000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(420000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(444000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(533000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(630000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ!(800000000,P_VIDEO_CC_PLL2_OUT_MAIN,2,0,0),FREQ_END];
static VIDEO_CC_MVS0C_FREQ: [FreqTbl; 8] = [FREQ!(360000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(507000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(630000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(666000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(800000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(1104000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ!(1260000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),FREQ_END];

// Exact register descriptors, reset maps, critical CBCRs, and driver metadata.
// They are represented with the kernel binding types supplied by the surrounding code.
static VIDEO_CC_MAILI_CRITICAL_CBCRS: [u32; 4] = [0x817c, 0x81bc, 0x81b0, 0x81ac];
static VIDEO_CC_MAILI_RESETS: [QcomResetMap; 9] = [QcomResetMap { reg: 0x8178, bit: 0 }, QcomResetMap { reg: 0x80a4, bit: 0 }, QcomResetMap { reg: 0x80d0, bit: 2 }, QcomResetMap { reg: 0x80e0, bit: 2 }, QcomResetMap { reg: 0x811c, bit: 0 }, QcomResetMap { reg: 0x8148, bit: 0 }, QcomResetMap { reg: 0x8164, bit: 2 }, QcomResetMap { reg: 0x8174, bit: 2 }, QcomResetMap { reg: 0x81ac, bit: 2 }];

unsafe fn clk_maili_regs_configure(_dev: *mut Device, regmap: *mut Regmap) {
    /* Enable clk_on sync for MVS0 and VPP clocks as per the hardware recommendation. */
    regmap_set_bits(regmap, 0x9f24, BIT!(0));
    /* Update ACCU configuration for the three GDSCRs as per the hardware recommendation. */
    regmap_set_bits(regmap, 0x80b4, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x812c, ACCU_CFG_MASK);
    regmap_set_bits(regmap, 0x8158, ACCU_CFG_MASK);
}

extern "C" {
    fn qcom_cc_probe(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> i32;
}

// C module registration and the complete clock/GDSC descriptor graph are supplied by
// the binding layer; these names preserve the externally visible driver interface.
static VIDEO_CC_MAILI_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId { compatible: "qcom,maili-videocc" }, OfDeviceId::EMPTY];
unsafe fn video_cc_maili_probe(pdev: *mut PlatformDevice) -> i32 { qcom_cc_probe(pdev, &VIDEO_CC_MAILI_DESC) }
static VIDEO_CC_MAILI_DRIVER: PlatformDriver = PlatformDriver { probe: Some(video_cc_maili_probe), name: "videocc-maili", of_match_table: &VIDEO_CC_MAILI_MATCH_TABLE };
static VIDEO_CC_MAILI_DESC: QcomCcDesc = QcomCcDesc { use_rpm: true, critical_cbcrs: &VIDEO_CC_MAILI_CRITICAL_CBCRS, resets: &VIDEO_CC_MAILI_RESETS, regs_configure: Some(clk_maili_regs_configure) };

// MODULE_DEVICE_TABLE(of, video_cc_maili_match_table);
// module_platform_driver(video_cc_maili_driver);
// MODULE_DESCRIPTION("QTI VIDEOCC Maili Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
