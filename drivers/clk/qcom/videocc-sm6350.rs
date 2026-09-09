// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2021, Konrad Dybcio <konrad.dybcio@somainline.org>
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

// External Linux kernel and Qualcomm clock-controller declarations are supplied
// by the surrounding translation environment.

#[repr(C)]
pub enum DtClock {
    DT_IFACE,
    DT_BI_TCXO,
    DT_SLEEP_CLK,
}

#[repr(C)]
pub enum Parent {
    P_BI_TCXO,
    P_CHIP_SLEEP_CLK,
    P_VIDEO_PLL0_OUT_EVEN,
}

static FABIA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 125000000, max_freq: 1000000000, val: 1 }];

/* 600 MHz */
static VIDEO_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x1f,
    alpha: 0x4000,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002067,
    test_ctl_val: 0x40000000,
    test_ctl_hi_val: 0x00000002,
    user_ctl_val: 0x00000101,
    user_ctl_hi_val: 0x00004005,
};

static mut VIDEO_PLL0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    vco_table: FABIA_VCO.as_ptr(),
    num_vco: ARRAY_SIZE(&FABIA_VCO),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_VIDEO_PLL0 } },
};

static POST_DIV_TABLE_VIDEO_PLL0_OUT_EVEN: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 },
    clk_div_table { val: 0, div: 0 },
];

static mut VIDEO_PLL0_OUT_EVEN: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0,
    post_div_shift: 8,
    post_div_table: POST_DIV_TABLE_VIDEO_PLL0_OUT_EVEN.as_ptr(),
    num_post_div: ARRAY_SIZE(&POST_DIV_TABLE_VIDEO_PLL0_OUT_EVEN),
    width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_VIDEO_PLL0_OUT_EVEN } },
};

static VIDEO_CC_PARENT_MAP_0: [parent_map; 2] = [
    parent_map { parent: P_BI_TCXO as u32, val: 0 },
    parent_map { parent: P_VIDEO_PLL0_OUT_EVEN as u32, val: 3 },
];

static mut VIDEO_CC_PARENT_DATA_0: [clk_parent_data; 2] = [
    clk_parent_data { index: DT_BI_TCXO as u8 },
    clk_parent_data { hw: unsafe { &mut VIDEO_PLL0_OUT_EVEN.clkr.hw } },
];

static VIDEO_CC_PARENT_MAP_1: [parent_map; 1] = [
    parent_map { parent: P_CHIP_SLEEP_CLK as u32, val: 0 },
];

static VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 1] = [
    clk_parent_data { index: DT_SLEEP_CLK as u8 },
];

static FTBL_VIDEO_CC_IRIS_CLK_SRC: [freq_tbl; 6] = [
    F(133250000, P_VIDEO_PLL0_OUT_EVEN, 2, 0, 0),
    F(240000000, P_VIDEO_PLL0_OUT_EVEN, 1.5, 0, 0),
    F(300000000, P_VIDEO_PLL0_OUT_EVEN, 1, 0, 0),
    F(380000000, P_VIDEO_PLL0_OUT_EVEN, 1, 0, 0),
    F(460000000, P_VIDEO_PLL0_OUT_EVEN, 1, 0, 0),
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

static mut VIDEO_CC_IRIS_CLK_SRC: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x1000,
    mnd_width: 0,
    hid_width: 5,
    parent_map: VIDEO_CC_PARENT_MAP_0.as_ptr(),
    freq_tbl: FTBL_VIDEO_CC_IRIS_CLK_SRC.as_ptr(),
    clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_VIDEO_CC_IRIS_CLK_SRC } },
};

static FTBL_VIDEO_CC_SLEEP_CLK_SRC: [freq_tbl; 2] = [
    F(32764, P_CHIP_SLEEP_CLK, 1, 0, 0),
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

static mut VIDEO_CC_SLEEP_CLK_SRC: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x701c,
    mnd_width: 0,
    hid_width: 5,
    parent_map: VIDEO_CC_PARENT_MAP_1.as_ptr(),
    freq_tbl: FTBL_VIDEO_CC_SLEEP_CLK_SRC.as_ptr(),
    clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_VIDEO_CC_SLEEP_CLK_SRC } },
};

static mut VIDEO_CC_IRIS_AHB_CLK: clk_branch = clk_branch { halt_reg: 0x5004, halt_check: BRANCH_VOTED, clkr: clk_regmap { enable_reg: 0x5004, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_IRIS_AHB_CLK } } };
static mut VIDEO_CC_MVS0_AXI_CLK: clk_branch = clk_branch { halt_reg: 0x800c, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x800c, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_MVS0_AXI_CLK } } };
static mut VIDEO_CC_MVS0_CORE_CLK: clk_branch = clk_branch { halt_reg: 0x3010, halt_check: BRANCH_VOTED, hwcg_reg: 0x3010, hwcg_bit: 1, clkr: clk_regmap { enable_reg: 0x3010, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_MVS0_CORE_CLK } } };
static mut VIDEO_CC_MVSC_CORE_CLK: clk_branch = clk_branch { halt_reg: 0x2014, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x2014, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_MVSC_CORE_CLK } } };
static mut VIDEO_CC_MVSC_CTL_AXI_CLK: clk_branch = clk_branch { halt_reg: 0x8004, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x8004, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_MVSC_CTL_AXI_CLK } } };
static mut VIDEO_CC_SLEEP_CLK: clk_branch = clk_branch { halt_reg: 0x7034, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x7034, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_SLEEP_CLK } } };
static mut VIDEO_CC_VENUS_AHB_CLK: clk_branch = clk_branch { halt_reg: 0x801c, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x801c, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_VIDEO_CC_VENUS_AHB_CLK } } };

static mut MVSC_GDSC: gdsc = gdsc { gdscr: 0x2004, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: generic_pm_domain { name: "mvsc_gdsc" }, pwrsts: PWRSTS_OFF_ON };
static mut MVS0_GDSC: gdsc = gdsc { gdscr: 0x3004, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: generic_pm_domain { name: "mvs0_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER };

static mut VIDEO_CC_SM6350_GDSCS: [*mut gdsc; 2] = [&mut MVSC_GDSC, &mut MVS0_GDSC];
static mut VIDEO_CC_SM6350_CLOCKS: [*mut clk_regmap; 11] = [
    &mut VIDEO_CC_IRIS_AHB_CLK.clkr, &mut VIDEO_CC_IRIS_CLK_SRC.clkr,
    &mut VIDEO_CC_MVS0_AXI_CLK.clkr, &mut VIDEO_CC_MVS0_CORE_CLK.clkr,
    &mut VIDEO_CC_MVSC_CORE_CLK.clkr, &mut VIDEO_CC_MVSC_CTL_AXI_CLK.clkr,
    &mut VIDEO_CC_SLEEP_CLK.clkr, &mut VIDEO_CC_SLEEP_CLK_SRC.clkr,
    &mut VIDEO_CC_VENUS_AHB_CLK.clkr, &mut VIDEO_PLL0.clkr,
    &mut VIDEO_PLL0_OUT_EVEN.clkr,
];

static VIDEO_CC_SM6350_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb000, fast_io: true };
static VIDEO_CC_SM6350_DESC: qcom_cc_desc = qcom_cc_desc { config: &VIDEO_CC_SM6350_REGMAP_CONFIG, clks: VIDEO_CC_SM6350_CLOCKS.as_ptr(), num_clks: 11, gdscs: VIDEO_CC_SM6350_GDSCS.as_ptr(), num_gdscs: 2 };

static VIDEO_CC_SM6350_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sm6350-videocc" },
    of_device_id { compatible: core::ptr::null() },
];

static mut VIDEO_CC_SM6350_DRIVER: platform_driver = platform_driver {
    probe: Some(video_cc_sm6350_probe),
    driver: device_driver { name: "video_cc-sm6350", of_match_table: VIDEO_CC_SM6350_MATCH_TABLE.as_ptr() },
};

unsafe extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_fabia_pll_configure(pll: *mut clk_alpha_pll, regmap: *mut regmap, config: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(regmap: *mut regmap, reg: u32);
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, regmap: *mut regmap) -> i32;
    fn PTR_ERR(ptr: *mut regmap) -> i32;
}

#[allow(non_snake_case)]
unsafe fn video_cc_sm6350_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SM6350_DESC);
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }
    clk_fabia_pll_configure(&mut VIDEO_PLL0, regmap, &VIDEO_PLL0_CONFIG);
    /* Keep some clocks always-on */
    qcom_branch_set_clk_en(regmap, 0x7018); /* VIDEO_CC_XO_CLK */
    qcom_cc_really_probe(unsafe { &mut (*pdev).dev }, &VIDEO_CC_SM6350_DESC, regmap)
}

// MODULE_DEVICE_TABLE(of, video_cc_sm6350_match_table);
// module_platform_driver(video_cc_sm6350_driver);
// MODULE_DESCRIPTION("QTI VIDEO_CC SM6350 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
