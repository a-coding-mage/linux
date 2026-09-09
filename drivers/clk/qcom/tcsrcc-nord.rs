// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// External kernel and Qualcomm clock-controller dependencies are supplied by
// the surrounding translation environment.

#[repr(u32)]
enum {
    DT_BI_TCXO_PAD = 0,
}

static mut tcsr_dp_rx_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xa008,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0xa008,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "tcsr_dp_rx_0_clkref_en",
            parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
            num_parents: 1,
            ops: &clk_branch2_ops,
        } },
    },
};

static mut tcsr_dp_rx_1_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xb008,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0xb008,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "tcsr_dp_rx_1_clkref_en",
            parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
            num_parents: 1,
            ops: &clk_branch2_ops,
        } },
    },
};

static mut tcsr_dp_tx_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xc008, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0xc008, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "tcsr_dp_tx_0_clkref_en", parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD }, num_parents: 1, ops: &clk_branch2_ops,
    } } },
};
static mut tcsr_dp_tx_1_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xd008, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0xd008, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "tcsr_dp_tx_1_clkref_en", parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD }, num_parents: 1, ops: &clk_branch2_ops,
    } } },
};
static mut tcsr_dp_tx_2_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xe008, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0xe008, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "tcsr_dp_tx_2_clkref_en", parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD }, num_parents: 1, ops: &clk_branch2_ops,
    } } },
};
static mut tcsr_dp_tx_3_clkref_en: clk_branch = clk_branch {
    halt_reg: 0xf008, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0xf008, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "tcsr_dp_tx_3_clkref_en", parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD }, num_parents: 1, ops: &clk_branch2_ops,
    } } },
};

macro_rules! branch { ($reg:expr, $name:expr) => { clk_branch { halt_reg: $reg, halt_check: BRANCH_HALT_DELAY, clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: $name, parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD }, num_parents: 1, ops: &clk_branch2_ops } } } } }; }
static mut tcsr_pcie_clkref_en: clk_branch = branch!(0x8, "tcsr_pcie_clkref_en");
static mut tcsr_ufs_clkref_en: clk_branch = branch!(0x3008, "tcsr_ufs_clkref_en");
static mut tcsr_usb2_0_clkref_en: clk_branch = branch!(0x4008, "tcsr_usb2_0_clkref_en");
static mut tcsr_usb2_1_clkref_en: clk_branch = branch!(0x5008, "tcsr_usb2_1_clkref_en");
static mut tcsr_usb2_2_clkref_en: clk_branch = branch!(0x6008, "tcsr_usb2_2_clkref_en");
static mut tcsr_usb3_0_clkref_en: clk_branch = branch!(0x8008, "tcsr_usb3_0_clkref_en");
static mut tcsr_usb3_1_clkref_en: clk_branch = branch!(0x7008, "tcsr_usb3_1_clkref_en");
static mut tcsr_ux_sgmii_0_clkref_en: clk_branch = branch!(0x1008, "tcsr_ux_sgmii_0_clkref_en");
static mut tcsr_ux_sgmii_1_clkref_en: clk_branch = branch!(0x2008, "tcsr_ux_sgmii_1_clkref_en");

static mut tcsr_cc_nord_clocks: [*mut clk_regmap; 15] = [
    unsafe { &mut tcsr_dp_rx_0_clkref_en.clkr }, unsafe { &mut tcsr_dp_rx_1_clkref_en.clkr },
    unsafe { &mut tcsr_dp_tx_0_clkref_en.clkr }, unsafe { &mut tcsr_dp_tx_1_clkref_en.clkr },
    unsafe { &mut tcsr_dp_tx_2_clkref_en.clkr }, unsafe { &mut tcsr_dp_tx_3_clkref_en.clkr },
    unsafe { &mut tcsr_pcie_clkref_en.clkr }, unsafe { &mut tcsr_ufs_clkref_en.clkr },
    unsafe { &mut tcsr_usb2_0_clkref_en.clkr }, unsafe { &mut tcsr_usb2_1_clkref_en.clkr },
    unsafe { &mut tcsr_usb2_2_clkref_en.clkr }, unsafe { &mut tcsr_usb3_0_clkref_en.clkr },
    unsafe { &mut tcsr_usb3_1_clkref_en.clkr }, unsafe { &mut tcsr_ux_sgmii_0_clkref_en.clkr },
    unsafe { &mut tcsr_ux_sgmii_1_clkref_en.clkr },
];

static tcsr_cc_nord_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xf008, fast_io: true,
};
static tcsr_cc_nord_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_nord_regmap_config, clks: tcsr_cc_nord_clocks.as_ptr(), num_clks: 15,
};
static tcsr_cc_nord_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,nord-tcsrcc" }, of_device_id { compatible: "" },
];

unsafe fn tcsr_cc_nord_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &tcsr_cc_nord_desc)
}

static mut tcsr_cc_nord_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_nord_probe),
    driver: driver { name: "tcsrcc-nord", of_match_table: tcsr_cc_nord_match_table.as_ptr() },
};

module_platform_driver!(tcsr_cc_nord_driver);
module_description!("QTI TCSRCC NORD Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
