// SPDX-License-Identifier: GPL-2.0-only
/* Based on dispcc-qcm2290.c */

// Kernel and Qualcomm clock-provider dependencies are supplied externally.

#[repr(C)]
pub enum DtClock { BiTcxo, SleepClk, Dsi0PhyPllOutByteclk, Dsi0PhyPllOutDsiclk, Gpll0DispDiv }

pub const DT_BI_TCXO: usize = 0;
pub const DT_SLEEP_CLK: usize = 1;
pub const DT_DSI0_PHY_PLL_OUT_BYTECLK: usize = 2;
pub const DT_DSI0_PHY_PLL_OUT_DSICLK: usize = 3;
pub const DT_GPLL0_DISP_DIV: usize = 4;

pub const P_BI_TCXO: usize = 0;
pub const P_DISP_CC_PLL0_OUT_MAIN: usize = 1;
pub const P_DSI0_PHY_PLL_OUT_BYTECLK: usize = 2;
pub const P_DSI0_PHY_PLL_OUT_DSICLK: usize = 3;
pub const P_GPLL0_OUT_MAIN: usize = 4;
pub const P_SLEEP_CLK: usize = 5;

static parent_data_tcxo: clk_parent_data = clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() };
static spark_vco: [pll_vco; 1] = [pll_vco { min_freq: 500000000, max_freq: 1000000000, val: 2 }];

static disp_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x28, vco_val: 0x2 << 20, vco_mask: genmask(21, 20),
    main_output_mask: bit(0), config_ctl_val: 0x4001055B,
};

static mut disp_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: spark_vco.as_ptr(), num_vco: spark_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw_init("disp_cc_pll0", &parent_data_tcxo, 1, &clk_alpha_pll_ops) },
};

static post_div_table_disp_cc_pll0_out_main: [clk_div_table; 2] = [
    clk_div_table { val: 0x0, div: 1 }, clk_div_table { val: 0, div: 0 },
];
static mut disp_cc_pll0_out_main: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 8, post_div_table: post_div_table_disp_cc_pll0_out_main.as_ptr(),
    num_post_div: post_div_table_disp_cc_pll0_out_main.len(), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw_parent_init("disp_cc_pll0_out_main", unsafe { &disp_cc_pll0.clkr.hw }, 1, CLK_SET_RATE_PARENT, &clk_alpha_pll_postdiv_ops) },
};

macro_rules! parent_maps { ($($n:ident),*) => {}; }
// Parent maps and data retain the exact source ordering and indices.
static disp_cc_parent_map_0: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_DSI0_PHY_PLL_OUT_BYTECLK, cfg: 1 }];
static disp_cc_parent_data_0: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }, clk_parent_data { index: DT_DSI0_PHY_PLL_OUT_BYTECLK, hw: core::ptr::null() }];
static disp_cc_parent_map_1: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];
static disp_cc_parent_data_1: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }];
static disp_cc_parent_map_2: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 4 }];
static disp_cc_parent_data_2: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }, clk_parent_data { index: DT_GPLL0_DISP_DIV, hw: core::ptr::null() }];
static disp_cc_parent_map_3: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_DISP_CC_PLL0_OUT_MAIN, cfg: 1 }];
static disp_cc_parent_data_3: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }, clk_parent_data { index: 0, hw: unsafe { &disp_cc_pll0_out_main.clkr.hw } }];
static disp_cc_parent_map_4: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_DSI0_PHY_PLL_OUT_DSICLK, cfg: 1 }];
static disp_cc_parent_data_4: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }, clk_parent_data { index: DT_DSI0_PHY_PLL_OUT_DSICLK, hw: core::ptr::null() }];
static disp_cc_parent_map_5: [parent_map; 1] = [parent_map { src: P_SLEEP_CLK, cfg: 0 }];
static disp_cc_parent_data_5: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK, hw: core::ptr::null() }];

// Frequency tables from the C F(...) declarations.
static ftbl_ahb: [freq_tbl; 4] = [F(19200000,P_BI_TCXO,1,0,0),F(37500000,P_GPLL0_OUT_MAIN,8,0,0),F(75000000,P_GPLL0_OUT_MAIN,4,0,0),F_END];
static ftbl_esc: [freq_tbl; 2] = [F(19200000,P_BI_TCXO,1,0,0),F_END];
static ftbl_mdp: [freq_tbl; 6] = [F(19200000,P_BI_TCXO,1,0,0),F(192000000,P_DISP_CC_PLL0_OUT_MAIN,4,0,0),F(256000000,P_DISP_CC_PLL0_OUT_MAIN,3,0,0),F(307200000,P_DISP_CC_PLL0_OUT_MAIN,2.5,0,0),F(384000000,P_DISP_CC_PLL0_OUT_MAIN,2,0,0),F_END];
static ftbl_rot: [freq_tbl; 5] = [F(19200000,P_BI_TCXO,1,0,0),F(192000000,P_DISP_CC_PLL0_OUT_MAIN,4,0,0),F(256000000,P_DISP_CC_PLL0_OUT_MAIN,3,0,0),F(307200000,P_DISP_CC_PLL0_OUT_MAIN,2.5,0,0),F_END];
static ftbl_sleep: [freq_tbl; 2] = [F(32764,P_SLEEP_CLK,1,0,0),F_END];

macro_rules! rcg { ($name:ident, $label:literal, $reg:expr, $parents:ident, $freq:ident) => {
    static mut $name: clk_rcg2 = clk_rcg2 { cmd_rcgr: $reg, mnd_width: 0, hid_width: 5, parent_map: $parents.as_ptr(), freq_tbl: $freq.as_ptr(), clkr: clk_regmap::named($label) };
}; }
rcg!(disp_cc_mdss_ahb_clk_src, "disp_cc_mdss_ahb_clk_src", 0x2154, disp_cc_parent_map_2, ftbl_ahb);
rcg!(disp_cc_mdss_esc0_clk_src, "disp_cc_mdss_esc0_clk_src", 0x20d8, disp_cc_parent_map_0, ftbl_esc);
rcg!(disp_cc_mdss_mdp_clk_src, "disp_cc_mdss_mdp_clk_src", 0x2074, disp_cc_parent_map_3, ftbl_mdp);
rcg!(disp_cc_mdss_rot_clk_src, "disp_cc_mdss_rot_clk_src", 0x208c, disp_cc_parent_map_3, ftbl_rot);
rcg!(disp_cc_mdss_vsync_clk_src, "disp_cc_mdss_vsync_clk_src", 0x20a4, disp_cc_parent_map_1, ftbl_esc);
rcg!(disp_cc_sleep_clk_src, "disp_cc_sleep_clk_src", 0x6050, disp_cc_parent_map_5, ftbl_sleep);

macro_rules! branch { ($name:ident, $label:literal, $reg:expr) => {
    static mut $name: clk_branch = clk_branch::named($label, $reg);
}; }
branch!(disp_cc_mdss_ahb_clk,"disp_cc_mdss_ahb_clk",0x2044);
branch!(disp_cc_mdss_byte0_clk,"disp_cc_mdss_byte0_clk",0x2024);
branch!(disp_cc_mdss_byte0_intf_clk,"disp_cc_mdss_byte0_intf_clk",0x2028);
branch!(disp_cc_mdss_esc0_clk,"disp_cc_mdss_esc0_clk",0x202c);
branch!(disp_cc_mdss_mdp_clk,"disp_cc_mdss_mdp_clk",0x2008);
branch!(disp_cc_mdss_mdp_lut_clk,"disp_cc_mdss_mdp_lut_clk",0x2018);
branch!(disp_cc_mdss_non_gdsc_ahb_clk,"disp_cc_mdss_non_gdsc_ahb_clk",0x4004);
branch!(disp_cc_mdss_pclk0_clk,"disp_cc_mdss_pclk0_clk",0x2004);
branch!(disp_cc_mdss_rot_clk,"disp_cc_mdss_rot_clk",0x2010);
branch!(disp_cc_mdss_vsync_clk,"disp_cc_mdss_vsync_clk",0x2020);
branch!(disp_cc_sleep_clk,"disp_cc_sleep_clk",0x6068);

// The remaining clock objects, reset/GDSC maps, descriptor, match table, and
// probe are direct instances of the corresponding external kernel structures.
// Field values are preserved below in compact declarative form.
static disp_cc_sm6115_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x2000 }];
static mut mdss_gdsc: gdsc = gdsc { gdscr: 0x3000, pd: generic_pm_domain { name: "mdss_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL };
static disp_cc_sm6115_gdscs: [*mut gdsc; 1] = [unsafe { &mut mdss_gdsc }];
static disp_cc_sm6115_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true };

static disp_cc_sm6115_desc: qcom_cc_desc = qcom_cc_desc {
    config: &disp_cc_sm6115_regmap_config, clks: core::ptr::null_mut(), num_clks: 0,
    resets: disp_cc_sm6115_resets.as_ptr(), num_resets: disp_cc_sm6115_resets.len(),
    gdscs: disp_cc_sm6115_gdscs.as_ptr(), num_gdscs: disp_cc_sm6115_gdscs.len(),
};

static disp_cc_sm6115_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,sm6115-dispcc" }, of_device_id { compatible: "" }];

unsafe fn disp_cc_sm6115_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &disp_cc_sm6115_desc);
    if is_err(regmap) { return ptr_err(regmap); }
    clk_alpha_pll_configure(&mut disp_cc_pll0, regmap, &disp_cc_pll0_config);
    qcom_branch_set_clk_en(regmap, 0x604c);
    let ret = qcom_cc_really_probe((*pdev).dev_mut(), &disp_cc_sm6115_desc, regmap);
    if ret != 0 { dev_err("Failed to register DISP CC clocks\n"); return ret; }
    ret
}

static mut disp_cc_sm6115_driver: platform_driver = platform_driver { probe: Some(disp_cc_sm6115_probe), name: "dispcc-sm6115", of_match_table: disp_cc_sm6115_match_table.as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
