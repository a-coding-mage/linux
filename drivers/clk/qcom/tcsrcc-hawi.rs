// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies:
// linux/clk-provider.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/regmap.h, dt-bindings/clock/qcom,hawi-tcsrcc.h
// and the local clock, common, and reset headers provide the referenced types,
// constants, and functions.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
const DT_BI_TCXO_PAD: usize = 0;

static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x4c,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x4c,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_pcie_0_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_pcie_1_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x0,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_pcie_1_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_ufs_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x10,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x10,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_ufs_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_usb2_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x18,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x18,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_usb2_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_usb3_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x8,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x8,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_usb3_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_cc_hawi_clocks: [*mut clk_regmap; 5] = [
    &raw mut tcsr_pcie_0_clkref_en.clkr,
    &raw mut tcsr_pcie_1_clkref_en.clkr,
    &raw mut tcsr_ufs_clkref_en.clkr,
    &raw mut tcsr_usb2_clkref_en.clkr,
    &raw mut tcsr_usb3_clkref_en.clkr,
];

static tcsr_cc_hawi_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x4c,
    fast_io: true,
};

static tcsr_cc_hawi_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_hawi_regmap_config,
    clks: &raw mut tcsr_cc_hawi_clocks,
    num_clks: tcsr_cc_hawi_clocks.len(),
};

static tcsr_cc_hawi_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,hawi-tcsrcc" },
    of_device_id { compatible: "" },
];

unsafe fn tcsr_cc_hawi_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &tcsr_cc_hawi_desc)
}

static mut tcsr_cc_hawi_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_hawi_probe),
    driver: device_driver {
        name: "tcsrcc-hawi",
        of_match_table: tcsr_cc_hawi_match_table.as_ptr(),
    },
};

// Equivalent of module_platform_driver(tcsr_cc_hawi_driver).
// MODULE_DEVICE_TABLE(of, tcsr_cc_hawi_match_table);
// MODULE_DESCRIPTION("QTI TCSRCC HAWI Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
