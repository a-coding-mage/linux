// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 * Copyright (c) 2025, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// External Linux/QCOM clock, platform, regmap, and interconnect definitions
// are supplied by the surrounding translation unit.

#[repr(C)]
pub enum DtInput { DT_XO, DT_CLK_REF }

#[repr(C)]
pub enum Parent { P_XO, P_GPLL0, P_APSS_PLL_EARLY, P_L3_PLL }

static APSS_PLL_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x3b, config_ctl_val: 0x08200920, config_ctl_hi_val: 0x05008001,
    config_ctl_hi1_val: 0x04000000, user_ctl_val: 0xf,
};

static mut IPQ5424_APSS_PLL: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &APSS_PLL_CONFIG,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_2290], flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: clk_regmap { enable_reg: 0x0, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_APSS_PLL } },
};

static CLK_INIT_APSS_PLL: clk_init_data = clk_init_data {
    name: "apss_pll", parent_data: &CLK_PARENT_APSS_PLL, num_parents: 1,
    ops: &clk_alpha_pll_huayra_ops,
};
static CLK_PARENT_APSS_PLL: clk_parent_data = clk_parent_data { index: DT_XO };

static PARENTS_APSS_SILVER_CLK_SRC: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_XO }, clk_parent_data { index: DT_CLK_REF },
    clk_parent_data { hw: unsafe { &IPQ5424_APSS_PLL.clkr.hw } },
];
static PARENTS_APSS_SILVER_CLK_SRC_MAP: [parent_map; 3] = [
    parent_map { parent: P_XO, index: 0 }, parent_map { parent: P_GPLL0, index: 4 },
    parent_map { parent: P_APSS_PLL_EARLY, index: 5 },
];
static FTBL_APSS_CLK_SRC: [freq_tbl; 4] = [
    F(816000000, P_APSS_PLL_EARLY, 1, 0, 0),
    F(1416000000, P_APSS_PLL_EARLY, 1, 0, 0),
    F(1800000000, P_APSS_PLL_EARLY, 1, 0, 0), freq_tbl {},
];
static mut APSS_SILVER_CLK_SRC: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x0080, freq_tbl: &FTBL_APSS_CLK_SRC, hid_width: 5,
    parent_map: &PARENTS_APSS_SILVER_CLK_SRC_MAP,
    clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_APSS_SILVER_CLK_SRC } },
};
static CLK_INIT_APSS_SILVER_CLK_SRC: clk_init_data = clk_init_data {
    name: "apss_silver_clk_src", parent_data: &PARENTS_APSS_SILVER_CLK_SRC,
    num_parents: ARRAY_SIZE(&PARENTS_APSS_SILVER_CLK_SRC), ops: &clk_rcg2_ops,
    flags: CLK_SET_RATE_PARENT,
};
static mut APSS_SILVER_CORE_CLK: clk_branch = clk_branch {
    halt_reg: 0x008c, clkr: clk_regmap { enable_reg: 0x008c, enable_mask: BIT(0),
        hw: clk_hw { init: &CLK_INIT_APSS_SILVER_CORE_CLK } },
};
static CLK_INIT_APSS_SILVER_CORE_CLK: clk_init_data = clk_init_data {
    name: "apss_silver_core_clk", parent_hws: &APSS_SILVER_CORE_PARENT,
    num_parents: 1, flags: CLK_SET_RATE_PARENT | CLK_IS_CRITICAL, ops: &clk_branch2_ops,
};
static APSS_SILVER_CORE_PARENT: [*const clk_hw; 1] = [unsafe { &APSS_SILVER_CLK_SRC.clkr.hw }];

static L3_PLL_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x29, config_ctl_val: 0x08200920, config_ctl_hi_val: 0x05008001,
    config_ctl_hi1_val: 0x04000000, user_ctl_val: 0xf,
};
static mut IPQ5424_L3_PLL: clk_alpha_pll = clk_alpha_pll {
    offset: 0x10000, config: &L3_PLL_CONFIG,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_2290], flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: clk_regmap { enable_reg: 0x0, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_L3_PLL } },
};
static CLK_INIT_L3_PLL: clk_init_data = clk_init_data {
    name: "l3_pll", parent_data: &CLK_PARENT_L3_PLL, num_parents: 1, ops: &clk_alpha_pll_huayra_ops,
};
static CLK_PARENT_L3_PLL: clk_parent_data = clk_parent_data { index: DT_XO };
static PARENTS_L3_CLK_SRC: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_XO }, clk_parent_data { index: DT_CLK_REF },
    clk_parent_data { hw: unsafe { &IPQ5424_L3_PLL.clkr.hw } },
];
static PARENTS_L3_CLK_SRC_MAP: [parent_map; 3] = [
    parent_map { parent: P_XO, index: 0 }, parent_map { parent: P_GPLL0, index: 4 }, parent_map { parent: P_L3_PLL, index: 5 },
];
static FTBL_L3_CLK_SRC: [freq_tbl; 4] = [
    F(816000000, P_L3_PLL, 1, 0, 0), F(984000000, P_L3_PLL, 1, 0, 0), F(1272000000, P_L3_PLL, 1, 0, 0), freq_tbl {},
];
static mut L3_CLK_SRC: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x10080, freq_tbl: &FTBL_L3_CLK_SRC, hid_width: 5, parent_map: &PARENTS_L3_CLK_SRC_MAP, clkr: clk_regmap { hw: clk_hw { init: &CLK_INIT_L3_CLK_SRC } } };
static CLK_INIT_L3_CLK_SRC: clk_init_data = clk_init_data { name: "l3_clk_src", parent_data: &PARENTS_L3_CLK_SRC, num_parents: ARRAY_SIZE(&PARENTS_L3_CLK_SRC), ops: &clk_rcg2_ops, flags: CLK_SET_RATE_PARENT };
static mut L3_CORE_CLK: clk_branch = clk_branch { halt_reg: 0x1008c, clkr: clk_regmap { enable_reg: 0x1008c, enable_mask: BIT(0), hw: clk_hw { init: &CLK_INIT_L3_CORE_CLK } } };
static CLK_INIT_L3_CORE_CLK: clk_init_data = clk_init_data { name: "l3_clk", parent_hws: &L3_CORE_PARENT, num_parents: 1, flags: CLK_SET_RATE_PARENT | CLK_IS_CRITICAL, ops: &clk_branch2_ops };
static L3_CORE_PARENT: [*const clk_hw; 1] = [unsafe { &L3_CLK_SRC.clkr.hw }];

static APSS_IPQ5424_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x20000, fast_io: true };
static mut APSS_IPQ5424_CLKS: [*mut clk_regmap; 6] = [unsafe { &mut IPQ5424_APSS_PLL.clkr }, unsafe { &mut APSS_SILVER_CLK_SRC.clkr }, unsafe { &mut APSS_SILVER_CORE_CLK.clkr }, unsafe { &mut IPQ5424_L3_PLL.clkr }, unsafe { &mut L3_CLK_SRC.clkr }, unsafe { &mut L3_CORE_CLK.clkr }];
static mut IPA5424_APSS_PLLS: [*mut clk_alpha_pll; 2] = [unsafe { &mut IPQ5424_L3_PLL }, unsafe { &mut IPQ5424_APSS_PLL }];
static IPA5424_APSS_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &IPA5424_APSS_PLLS, num_alpha_plls: ARRAY_SIZE(&IPA5424_APSS_PLLS) };
const IPQ_APPS_PLL_ID: u32 = 5424 * 3;
static ICC_IPQ5424_CPU_L3: [qcom_icc_hws_data; 1] = [qcom_icc_hws_data { master: MASTER_CPU, slave: SLAVE_L3, clk: L3_CORE_CLK }];
static APSS_IPQ5424_DESC: qcom_cc_desc = qcom_cc_desc { config: &APSS_IPQ5424_REGMAP_CONFIG, clks: &APSS_IPQ5424_CLKS, num_clks: ARRAY_SIZE(&APSS_IPQ5424_CLKS), icc_hws: &ICC_IPQ5424_CPU_L3, num_icc_hws: ARRAY_SIZE(&ICC_IPQ5424_CPU_L3), icc_first_node_id: IPQ_APPS_PLL_ID, driver_data: &IPA5424_APSS_DRIVER_DATA };

unsafe fn apss_ipq5424_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &APSS_IPQ5424_DESC) }
static APSS_IPQ5424_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,ipq5424-apss-clk" }, of_device_id {}];
static mut APSS_IPQ5424_DRIVER: platform_driver = platform_driver { probe: Some(apss_ipq5424_probe), driver: driver { name: "apss-ipq5424-clk", of_match_table: &APSS_IPQ5424_MATCH_TABLE, sync_state: Some(icc_sync_state) } };
module_platform_driver!(APSS_IPQ5424_DRIVER);
module_description!("QCOM APSS IPQ5424 CLK Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
