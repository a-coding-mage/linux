// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

static mut lpass_top_cc_lpi_q6_axim_hs_clk: clk_branch = clk_branch {
    halt_reg: 0x0,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_top_cc_lpi_q6_axim_hs_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut lpass_qdsp6ss_core_clk: clk_branch = clk_branch {
    halt_reg: 0x20,
    // CLK_OFF would not toggle until LPASS is out of reset
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x20,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_qdsp6ss_core_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut lpass_qdsp6ss_xo_clk: clk_branch = clk_branch {
    halt_reg: 0x38,
    // CLK_OFF would not toggle until LPASS is out of reset
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x38,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_qdsp6ss_xo_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut lpass_qdsp6ss_sleep_clk: clk_branch = clk_branch {
    halt_reg: 0x3c,
    // CLK_OFF would not toggle until LPASS is out of reset
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x3c,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_qdsp6ss_sleep_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut lpass_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    fast_io: true,
};

static mut lpass_cc_top_sc7280_clocks: [*mut clk_regmap; LPASS_TOP_CC_LPI_Q6_AXIM_HS_CLK + 1] = {
    let mut clocks = [core::ptr::null_mut(); LPASS_TOP_CC_LPI_Q6_AXIM_HS_CLK + 1];
    clocks[LPASS_TOP_CC_LPI_Q6_AXIM_HS_CLK] = unsafe { &mut lpass_top_cc_lpi_q6_axim_hs_clk.clkr };
    clocks
};

static lpass_cc_top_sc7280_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &lpass_regmap_config },
    clks: unsafe { &lpass_cc_top_sc7280_clocks },
    num_clks: lpass_cc_top_sc7280_clocks.len(),
};

static mut lpass_qdsp6ss_sc7280_clocks: [*mut clk_regmap; LPASS_QDSP6SS_CORE_CLK + 1] = {
    let mut clocks = [core::ptr::null_mut(); LPASS_QDSP6SS_CORE_CLK + 1];
    clocks[LPASS_QDSP6SS_XO_CLK] = unsafe { &mut lpass_qdsp6ss_xo_clk.clkr };
    clocks[LPASS_QDSP6SS_SLEEP_CLK] = unsafe { &mut lpass_qdsp6ss_sleep_clk.clkr };
    clocks[LPASS_QDSP6SS_CORE_CLK] = unsafe { &mut lpass_qdsp6ss_core_clk.clkr };
    clocks
};

static lpass_qdsp6ss_sc7280_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &lpass_regmap_config },
    clks: unsafe { &lpass_qdsp6ss_sc7280_clocks },
    num_clks: lpass_qdsp6ss_sc7280_clocks.len(),
};

unsafe fn lpass_cc_sc7280_probe(pdev: *mut platform_device) -> c_int {
    let mut desc: *const qcom_cc_desc;
    let mut ret: c_int;

    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }

    ret = pm_clk_create(&mut (*pdev).dev);
    if ret != 0 { return ret; }

    ret = pm_clk_add(&mut (*pdev).dev, "iface");
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "failed to acquire iface clock\n");
        goto_err_destroy_pm_clk!(ret);
    }

    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret != 0 { goto_err_destroy_pm_clk!(ret); }

    if !of_property_read_bool((*pdev).dev.of_node, "qcom,adsp-pil-mode") {
        lpass_regmap_config.name = "qdsp6ss";
        lpass_regmap_config.max_register = 0x3f;
        desc = &lpass_qdsp6ss_sc7280_desc;
        ret = qcom_cc_probe_by_index(pdev, 0, desc);
        if ret != 0 { goto_err_put_rpm!(ret); }
    }

    lpass_regmap_config.name = "top_cc";
    lpass_regmap_config.max_register = 0x4;
    desc = &lpass_cc_top_sc7280_desc;
    ret = qcom_cc_probe_by_index(pdev, 1, desc);
    if ret != 0 { goto_err_put_rpm!(ret); }

    pm_runtime_put(&mut (*pdev).dev);
    return 0;

    // The labels below represent the direct C cleanup branches.
    goto_err_put_rpm!(ret) => {
        pm_runtime_put_sync(&mut (*pdev).dev);
        pm_clk_destroy(&mut (*pdev).dev);
        return ret;
    }
    goto_err_destroy_pm_clk!(ret) => {
        pm_clk_destroy(&mut (*pdev).dev);
        return ret;
    }
}

static lpass_cc_sc7280_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sc7280-lpasscc" },
    of_device_id { compatible: "" },
];

static mut lpass_cc_sc7280_driver: platform_driver = platform_driver {
    probe: Some(lpass_cc_sc7280_probe),
    driver: driver {
        name: "sc7280-lpasscc",
        of_match_table: &lpass_cc_sc7280_match_table,
    },
};

unsafe fn lpass_cc_sc7280_init() -> c_int {
    platform_driver_register(&mut lpass_cc_sc7280_driver)
}

subsys_initcall!(lpass_cc_sc7280_init);

unsafe fn lpass_cc_sc7280_exit() {
    platform_driver_unregister(&mut lpass_cc_sc7280_driver);
}

module_exit!(lpass_cc_sc7280_exit);

MODULE_DEVICE_TABLE!(of, lpass_cc_sc7280_match_table);
MODULE_DESCRIPTION!("QTI LPASS_CC SC7280 Driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
