// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Dependencies supplied by the surrounding kernel clock framework and device-tree bindings.

enum {
    DT_BI_TCXO_PAD,
}

static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31100,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31100,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_pcie_0_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_pcie_1_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31114,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31114,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_pcie_1_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_ufs_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31110,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31110,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_ufs_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_ufs_pad_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31104,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31104,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_ufs_pad_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_usb2_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31118,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31118,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_usb2_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_usb3_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x31108,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x31108,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &mut clk_init_data {
                name: "tcsr_usb3_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut tcsr_cc_sm8650_clocks: [*mut clk_regmap; 6] = [
    [TCSR_PCIE_0_CLKREF_EN] = &mut tcsr_pcie_0_clkref_en.clkr,
    [TCSR_PCIE_1_CLKREF_EN] = &mut tcsr_pcie_1_clkref_en.clkr,
    [TCSR_UFS_CLKREF_EN] = &mut tcsr_ufs_clkref_en.clkr,
    [TCSR_UFS_PAD_CLKREF_EN] = &mut tcsr_ufs_pad_clkref_en.clkr,
    [TCSR_USB2_CLKREF_EN] = &mut tcsr_usb2_clkref_en.clkr,
    [TCSR_USB3_CLKREF_EN] = &mut tcsr_usb3_clkref_en.clkr,
];

static tcsr_cc_sm8650_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x3b000,
    fast_io: true,
};

static tcsr_cc_sm8650_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_sm8650_regmap_config,
    clks: tcsr_cc_sm8650_clocks.as_ptr(),
    num_clks: tcsr_cc_sm8650_clocks.len(),
};

static tcsr_cc_sm8650_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,milos-tcsr" },
    of_device_id { compatible: "qcom,sm8650-tcsr" },
    of_device_id { },
];

static mut tcsr_cc_sm8650_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_sm8650_probe),
    driver: driver {
        name: "tcsr_cc-sm8650",
        of_match_table: tcsr_cc_sm8650_match_table.as_ptr(),
    },
};

unsafe fn tcsr_cc_sm8650_probe(pdev: *mut platform_device) -> i32 {
    if of_device_is_compatible((*(*pdev).dev).of_node, "qcom,milos-tcsr") {
        tcsr_ufs_clkref_en.halt_reg = 0x31118;
        tcsr_ufs_clkref_en.clkr.enable_reg = 0x31118;
        tcsr_cc_sm8650_clocks[TCSR_USB2_CLKREF_EN] = core::ptr::null_mut();
        tcsr_cc_sm8650_clocks[TCSR_USB3_CLKREF_EN] = core::ptr::null_mut();
    }

    qcom_cc_probe(pdev, &tcsr_cc_sm8650_desc)
}

unsafe fn tcsr_cc_sm8650_init() -> i32 {
    platform_driver_register(&mut tcsr_cc_sm8650_driver)
}

unsafe fn tcsr_cc_sm8650_exit() {
    platform_driver_unregister(&mut tcsr_cc_sm8650_driver);
}

// MODULE_DEVICE_TABLE(of, tcsr_cc_sm8650_match_table);
// subsys_initcall(tcsr_cc_sm8650_init);
// module_exit(tcsr_cc_sm8650_exit);
// MODULE_DESCRIPTION("QTI TCSRCC SM8650 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
