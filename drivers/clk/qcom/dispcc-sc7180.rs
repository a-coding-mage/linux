// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019, 2022, The Linux Foundation. All rights reserved. */
// Kernel headers and local clock-controller dependencies are supplied externally.

#[repr(usize)]
enum Parent {
    BiTcxo,
    DispCcPll0OutEven,
    DispCcPll0OutMain,
    DpPhyPllLinkClk,
    DpPhyPllVcoDivClk,
    Dsi0PhyPllOutByteclk,
    Dsi0PhyPllOutDsiclk,
    Gpll0OutMain,
}

static FABIA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static mut DISP_CC_PLL0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: FABIA_VCO.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "disp_cc_pll0", parent_data: &clk_parent_data { fw_name: "bi_tcxo" }, num_parents: 1, ops: &clk_alpha_pll_fabia_ops, ..Default::default() }, ..Default::default() }, ..Default::default() },
};
static POST_DIV_TABLE: [clk_div_table; 2] = [clk_div_table { val: 0, div: 1 }, clk_div_table { val: 0, div: 0 }];
static mut DISP_CC_PLL0_OUT_EVEN: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0, post_div_shift: 8, post_div_table: POST_DIV_TABLE.as_ptr(), num_post_div: 2, width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "disp_cc_pll0_out_even", parent_hws: &(&mut DISP_CC_PLL0.clkr.hw as *mut _), num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_fabia_ops, ..Default::default() }, ..Default::default() }, ..Default::default() },
};

static PARENT_MAP_0: [parent_map; 1] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }];
static PARENT_DATA_0: [clk_parent_data; 1] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }];
static PARENT_MAP_1: [parent_map; 3] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }, parent_map { src: Parent::DpPhyPllLinkClk as u32, val: 1 }, parent_map { src: Parent::DpPhyPllVcoDivClk as u32, val: 2 }];
static PARENT_DATA_1: [clk_parent_data; 3] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }, clk_parent_data { fw_name: "dp_phy_pll_link_clk", ..Default::default() }, clk_parent_data { fw_name: "dp_phy_pll_vco_div_clk", ..Default::default() }];
static PARENT_MAP_2: [parent_map; 2] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }, parent_map { src: Parent::Dsi0PhyPllOutByteclk as u32, val: 1 }];
static PARENT_DATA_2: [clk_parent_data; 2] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }, clk_parent_data { fw_name: "dsi0_phy_pll_out_byteclk", ..Default::default() }];
static PARENT_MAP_3: [parent_map; 4] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }, parent_map { src: Parent::DispCcPll0OutMain as u32, val: 1 }, parent_map { src: Parent::Gpll0OutMain as u32, val: 4 }, parent_map { src: Parent::DispCcPll0OutEven as u32, val: 5 }];
static PARENT_DATA_3: [clk_parent_data; 4] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }, clk_parent_data { hw: unsafe { &DISP_CC_PLL0.clkr.hw }, ..Default::default() }, clk_parent_data { fw_name: "gcc_disp_gpll0_clk_src", ..Default::default() }, clk_parent_data { hw: unsafe { &DISP_CC_PLL0_OUT_EVEN.clkr.hw }, ..Default::default() }];
static PARENT_MAP_4: [parent_map; 2] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }, parent_map { src: Parent::Gpll0OutMain as u32, val: 4 }];
static PARENT_DATA_4: [clk_parent_data; 2] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }, clk_parent_data { fw_name: "gcc_disp_gpll0_clk_src", ..Default::default() }];
static PARENT_MAP_5: [parent_map; 2] = [parent_map { src: Parent::BiTcxo as u32, val: 0 }, parent_map { src: Parent::Dsi0PhyPllOutDsiclk as u32, val: 1 }];
static PARENT_DATA_5: [clk_parent_data; 2] = [clk_parent_data { fw_name: "bi_tcxo", ..Default::default() }, clk_parent_data { fw_name: "dsi0_phy_pll_out_dsiclk", ..Default::default() }];

macro_rules! freq { ($rate:expr, $parent:expr, $n:expr) => { freq_tbl { rate: $rate, src: $parent as u32, pre_div: $n, m: 0, n: 0 } }; }
static FTBL_AHB: [freq_tbl; 4] = [freq!(19200000, Parent::BiTcxo, 1), freq!(37500000, Parent::Gpll0OutMain, 16), freq!(75000000, Parent::Gpll0OutMain, 8), freq_tbl::default()];
static FTBL_AUX: [freq_tbl; 2] = [freq!(19200000, Parent::BiTcxo, 1), freq_tbl::default()];
static FTBL_MDP: [freq_tbl; 6] = [freq!(19200000, Parent::BiTcxo, 1), freq!(200000000, Parent::Gpll0OutMain, 3), freq!(300000000, Parent::Gpll0OutMain, 2), freq!(345000000, Parent::DispCcPll0OutMain, 4), freq!(460000000, Parent::DispCcPll0OutMain, 3), freq_tbl::default()];

// RCG and branch declarations retain the C driver's register layout and graph.
macro_rules! rcg { ($name:ident, $reg:expr, $mnd:expr, $map:ident, $data:ident, $ops:ident $(, $tbl:ident)?) => { static mut $name: clk_rcg2 = clk_rcg2 { cmd_rcgr: $reg, mnd_width: $mnd, hid_width: 5, parent_map: $map.as_ptr(), freq_tbl: rcg!(@tbl $($tbl)?), clkr: clk_regmap::with_init(clk_init_data { name: stringify!($name), parent_data: $data.as_ptr(), num_parents: $data.len(), ops: &$ops, ..Default::default() }) }; }; (@tbl $t:ident) => { $t.as_ptr() }; (@tbl) => { core::ptr::null() }; }
rcg!(DISP_CC_MDSS_AHB_CLK_SRC, 0x22bc, 0, PARENT_MAP_4, PARENT_DATA_4, clk_rcg2_shared_ops, FTBL_AHB);
rcg!(DISP_CC_MDSS_BYTE0_CLK_SRC, 0x2110, 0, PARENT_MAP_2, PARENT_DATA_2, clk_byte2_ops);
rcg!(DISP_CC_MDSS_DP_AUX_CLK_SRC, 0x21dc, 0, PARENT_MAP_0, PARENT_DATA_0, clk_rcg2_ops, FTBL_AUX);
rcg!(DISP_CC_MDSS_DP_CRYPTO_CLK_SRC, 0x2194, 0, PARENT_MAP_1, PARENT_DATA_1, clk_byte2_ops);
rcg!(DISP_CC_MDSS_DP_LINK_CLK_SRC, 0x2178, 0, PARENT_MAP_1, PARENT_DATA_1, clk_byte2_ops);
rcg!(DISP_CC_MDSS_DP_PIXEL_CLK_SRC, 0x21ac, 16, PARENT_MAP_1, PARENT_DATA_1, clk_dp_ops);
rcg!(DISP_CC_MDSS_ESC0_CLK_SRC, 0x2148, 0, PARENT_MAP_2, PARENT_DATA_2, clk_rcg2_ops, FTBL_AUX);
rcg!(DISP_CC_MDSS_MDP_CLK_SRC, 0x20c8, 0, PARENT_MAP_3, PARENT_DATA_3, clk_rcg2_shared_ops, FTBL_MDP);
rcg!(DISP_CC_MDSS_PCLK0_CLK_SRC, 0x2098, 8, PARENT_MAP_5, PARENT_DATA_5, clk_pixel_ops);
rcg!(DISP_CC_MDSS_ROT_CLK_SRC, 0x20e0, 0, PARENT_MAP_3, PARENT_DATA_3, clk_rcg2_shared_ops, FTBL_MDP);
rcg!(DISP_CC_MDSS_VSYNC_CLK_SRC, 0x20f8, 0, PARENT_MAP_0, PARENT_DATA_0, clk_rcg2_shared_ops, FTBL_AUX);

macro_rules! branch { ($name:ident, $reg:expr, $halt:ident, $parent:ident, $flags:expr) => { static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt, clkr: clk_regmap::branch($reg, stringify!($name), unsafe { &$parent.clkr.hw }, $flags, &clk_branch2_ops) }; }; }
branch!(DISP_CC_MDSS_AHB_CLK, 0x2080, BRANCH_HALT, DISP_CC_MDSS_AHB_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_BYTE0_CLK, 0x2028, BRANCH_HALT, DISP_CC_MDSS_BYTE0_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_BYTE0_INTF_CLK, 0x202c, BRANCH_HALT, DISP_CC_MDSS_BYTE0_DIV_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_DP_AUX_CLK, 0x2054, BRANCH_HALT, DISP_CC_MDSS_DP_AUX_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_DP_CRYPTO_CLK, 0x2048, BRANCH_HALT, DISP_CC_MDSS_DP_CRYPTO_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_DP_LINK_CLK, 0x2040, BRANCH_HALT, DISP_CC_MDSS_DP_LINK_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_DP_LINK_INTF_CLK, 0x2044, BRANCH_HALT, DISP_CC_MDSS_DP_LINK_DIV_CLK_SRC, 0);
branch!(DISP_CC_MDSS_DP_PIXEL_CLK, 0x204c, BRANCH_HALT, DISP_CC_MDSS_DP_PIXEL_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_ESC0_CLK, 0x2038, BRANCH_HALT, DISP_CC_MDSS_ESC0_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_MDP_CLK, 0x200c, BRANCH_HALT, DISP_CC_MDSS_MDP_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_MDP_LUT_CLK, 0x201c, BRANCH_VOTED, DISP_CC_MDSS_MDP_CLK_SRC, 0);
branch!(DISP_CC_MDSS_NON_GDSC_AHB_CLK, 0x4004, BRANCH_VOTED, DISP_CC_MDSS_AHB_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_PCLK0_CLK, 0x2004, BRANCH_HALT, DISP_CC_MDSS_PCLK0_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_ROT_CLK, 0x2014, BRANCH_HALT, DISP_CC_MDSS_ROT_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_RSCC_VSYNC_CLK, 0x4008, BRANCH_HALT, DISP_CC_MDSS_VSYNC_CLK_SRC, CLK_SET_RATE_PARENT);
branch!(DISP_CC_MDSS_VSYNC_CLK, 0x2024, BRANCH_HALT, DISP_CC_MDSS_VSYNC_CLK_SRC, CLK_SET_RATE_PARENT);

static mut MDSS_GDSC: gdsc = gdsc { gdscr: 0x3000, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd: genpd { name: "mdss_gdsc", ..Default::default() }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL };
static RESETS: [qcom_reset_map; 2] = [qcom_reset_map { reg: 0x2000 }, qcom_reset_map { reg: 0x4000 }];
static mut GDSCS: [*mut gdsc; 1] = [unsafe { &mut MDSS_GDSC }];
static mut CLOCKS: [*mut clk_regmap; 31] = [
    unsafe { &mut DISP_CC_MDSS_AHB_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_AHB_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_BYTE0_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_BYTE0_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_DP_AUX_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_DP_AUX_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_DP_CRYPTO_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_DP_CRYPTO_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_DP_LINK_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_DP_LINK_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_DP_PIXEL_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_DP_PIXEL_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_ESC0_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_ESC0_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_MDP_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_MDP_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_MDP_LUT_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_NON_GDSC_AHB_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_PCLK0_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_PCLK0_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_ROT_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_ROT_CLK_SRC.clkr }, unsafe { &mut DISP_CC_MDSS_RSCC_VSYNC_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_VSYNC_CLK.clkr }, unsafe { &mut DISP_CC_MDSS_VSYNC_CLK_SRC.clkr }, unsafe { &mut DISP_CC_PLL0.clkr }, unsafe { &mut DISP_CC_PLL0_OUT_EVEN.clkr }, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()
];
static REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true, ..Default::default() };
static DESC: qcom_cc_desc = qcom_cc_desc { config: &REGMAP_CONFIG, clks: CLOCKS.as_ptr(), num_clks: CLOCKS.len(), resets: RESETS.as_ptr(), num_resets: RESETS.len(), gdscs: GDSCS.as_ptr(), num_gdscs: GDSCS.len() };

unsafe fn disp_cc_sc7180_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &DESC);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    let mut config = alpha_pll_config::default();
    config.l = 0x47; config.alpha = 0xe000; config.user_ctl_val = 0x00000001; config.user_ctl_hi_val = 0x00004805;
    clk_fabia_pll_configure(&mut DISP_CC_PLL0, regmap, &config);
    qcom_cc_really_probe(&mut (*pdev).dev, &DESC, regmap)
}

static mut DISP_CC_SC7180_DRIVER: platform_driver = platform_driver { probe: Some(disp_cc_sc7180_probe), driver: driver { name: "sc7180-dispcc", of_match_table: &[of_device_id { compatible: "qcom,sc7180-dispcc" }, of_device_id::default()], ..Default::default() } };
// Equivalent of module_platform_driver(disp_cc_sc7180_driver).
// MODULE_DESCRIPTION("QTI DISP_CC SC7180 Driver"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
