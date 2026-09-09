// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies supplied by the surrounding kernel clock framework:
// linux/clk-provider.h, linux/module.h, linux/platform_device.h,
// linux/regmap.h, dt-bindings/clock/qcom,eliza-tcsr.h, clk-branch.h,
// clk-regmap.h, and common.h.

const DT_BI_TCXO_PAD: usize = 0;

static mut tcsr_hdmi_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x14,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x14,
        enable_mask: 1 << 0,
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_hdmi_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x0,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: 1 << 0,
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
    halt_reg: 0x1c,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x1c,
        enable_mask: 1 << 0,
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
    halt_reg: 0x8,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x8,
        enable_mask: 1 << 0,
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
    halt_reg: 0x4,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x4,
        enable_mask: 1 << 0,
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
    halt_reg: 0x10,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x10,
        enable_mask: 1 << 0,
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

static mut tcsr_cc_eliza_clocks: [*mut clk_regmap; 6] = [
    unsafe { &mut tcsr_hdmi_clkref_en.clkr },
    unsafe { &mut tcsr_pcie_0_clkref_en.clkr },
    unsafe { &mut tcsr_pcie_1_clkref_en.clkr },
    unsafe { &mut tcsr_ufs_clkref_en.clkr },
    unsafe { &mut tcsr_usb2_clkref_en.clkr },
    unsafe { &mut tcsr_usb3_clkref_en.clkr },
];

static tcsr_cc_eliza_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x1c,
    fast_io: true,
};

static tcsr_cc_eliza_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_eliza_regmap_config,
    clks: &mut tcsr_cc_eliza_clocks,
    num_clks: 6,
};

static tcsr_cc_eliza_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,eliza-tcsr" },
    of_device_id { compatible: "" },
];

unsafe fn tcsr_cc_eliza_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &tcsr_cc_eliza_desc)
}

static mut tcsr_cc_eliza_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_eliza_probe),
    driver: device_driver {
        name: "tcsr_cc-eliza",
        of_match_table: &tcsr_cc_eliza_match_table,
    },
};

unsafe fn tcsr_cc_eliza_init() -> i32 {
    platform_driver_register(&mut tcsr_cc_eliza_driver)
}

// subsys_initcall(tcsr_cc_eliza_init);

unsafe fn tcsr_cc_eliza_exit() {
    platform_driver_unregister(&mut tcsr_cc_eliza_driver);
}

// module_exit(tcsr_cc_eliza_exit);
// MODULE_DESCRIPTION("QTI TCSR_CC Eliza Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
