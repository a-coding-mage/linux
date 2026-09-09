// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel clock-controller code.

static mut lcc_ahbfabric_cbc_clk: clk_branch = clk_branch {
    halt_reg: 0x1b004,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap {
        enable_reg: 0x1b004,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_ahbfabric_cbc_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut lcc_q6ss_ahbs_cbc_clk: clk_branch = clk_branch {
    halt_reg: 0x22000,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x22000,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_q6ss_ahbs_cbc_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut lcc_q6ss_tcm_slave_cbc_clk: clk_branch = clk_branch {
    halt_reg: 0x1c000,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x1c000,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_q6ss_tcm_slave_cbc_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut lcc_q6ss_ahbm_cbc_clk: clk_branch = clk_branch {
    halt_reg: 0x22004,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x22004,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_q6ss_ahbm_cbc_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut lcc_q6ss_axim_cbc_clk: clk_branch = clk_branch {
    halt_reg: 0x1c004,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x1c004,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_q6ss_axim_cbc_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut lcc_q6ss_bcr_sleep_clk: clk_branch = clk_branch {
    halt_reg: 0x6004,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x6004,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "lcc_q6ss_bcr_sleep_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

/* TCSR clock */
static mut tcsr_lcc_csr_cbcr_clk: clk_branch = clk_branch {
    halt_reg: 0x8008,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x8008,
        enable_mask: BIT(0),
        hw: clk_hw { init: &clk_init_data {
            name: "tcsr_lcc_csr_cbcr_clk",
            ops: &clk_branch2_ops,
        } },
    },
};

static mut q6sstop_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    fast_io: true,
};

static mut q6sstop_qcs404_clocks: [*mut clk_regmap; 6] = [
    &mut lcc_ahbfabric_cbc_clk.clkr,
    &mut lcc_q6ss_ahbs_cbc_clk.clkr,
    &mut lcc_q6ss_tcm_slave_cbc_clk.clkr,
    &mut lcc_q6ss_ahbm_cbc_clk.clkr,
    &mut lcc_q6ss_axim_cbc_clk.clkr,
    &mut lcc_q6ss_bcr_sleep_clk.clkr,
];

static q6sstop_qcs404_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x6000 }];

static q6sstop_qcs404_desc: qcom_cc_desc = qcom_cc_desc {
    config: &q6sstop_regmap_config,
    clks: &q6sstop_qcs404_clocks,
    num_clks: q6sstop_qcs404_clocks.len(),
    resets: &q6sstop_qcs404_resets,
    num_resets: q6sstop_qcs404_resets.len(),
};

static mut tcsr_qcs404_clocks: [*mut clk_regmap; 1] = [&mut tcsr_lcc_csr_cbcr_clk.clkr];

static tcsr_qcs404_desc: qcom_cc_desc = qcom_cc_desc {
    config: &q6sstop_regmap_config,
    clks: &tcsr_qcs404_clocks,
    num_clks: tcsr_qcs404_clocks.len(),
};

static q6sstopcc_qcs404_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,qcs404-q6sstopcc" },
    of_device_id { compatible: "" },
];

static q6sstopcc_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(pm_clk_suspend),
    resume: Some(pm_clk_resume),
    idle: None,
};

static mut q6sstopcc_qcs404_driver: platform_driver = platform_driver {
    probe: Some(q6sstopcc_qcs404_probe),
    driver: device_driver {
        name: "qcs404-q6sstopcc",
        of_match_table: &q6sstopcc_qcs404_match_table,
        pm: &q6sstopcc_pm_ops,
    },
};

unsafe fn q6sstopcc_qcs404_probe(pdev: *mut platform_device) -> i32 {
    let mut desc: *const qcom_cc_desc;
    let mut ret: i32;

    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }

    ret = devm_pm_clk_create(&mut (*pdev).dev);
    if ret != 0 { return ret; }

    ret = pm_clk_add(&mut (*pdev).dev, core::ptr::null());
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "failed to acquire iface clock\n");
        return ret;
    }

    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret != 0 { return ret; }

    q6sstop_regmap_config.name = "q6sstop_tcsr";
    desc = &tcsr_qcs404_desc;
    ret = qcom_cc_probe_by_index(pdev, 1, desc);
    if ret != 0 { pm_runtime_put_sync(&mut (*pdev).dev); return ret; }

    q6sstop_regmap_config.name = "q6sstop_cc";
    desc = &q6sstop_qcs404_desc;
    ret = qcom_cc_probe_by_index(pdev, 0, desc);
    if ret != 0 { pm_runtime_put_sync(&mut (*pdev).dev); return ret; }

    pm_runtime_put(&mut (*pdev).dev);
    0
}

module_platform_driver!(q6sstopcc_qcs404_driver);

// MODULE_DESCRIPTION("QTI QCS404 Q6SSTOP Clock Controller Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
