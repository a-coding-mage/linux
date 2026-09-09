// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux clock, platform-device, module, regmap,
// Qualcomm clock-binding, and local clock-provider interfaces.

static mut lpass_q6ss_ahbm_aon_clk: clk_branch = clk_branch {
    halt_reg: 0x12000,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x12000,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_q6ss_ahbm_aon_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut lpass_q6ss_ahbs_aon_clk: clk_branch = clk_branch {
    halt_reg: 0x1f000,
    halt_check: BRANCH_VOTED,
    clkr: clk_regmap {
        enable_reg: 0x1f000,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "lpass_q6ss_ahbs_aon_clk",
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

static mut lpass_cc_sdm845_clocks: [*mut clk_regmap; 2] = [
    [LPASS_Q6SS_AHBM_AON_CLK] = unsafe { &raw mut lpass_q6ss_ahbm_aon_clk.clkr },
    [LPASS_Q6SS_AHBS_AON_CLK] = unsafe { &raw mut lpass_q6ss_ahbs_aon_clk.clkr },
];

static lpass_cc_sdm845_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &raw mut lpass_regmap_config },
    clks: unsafe { &raw mut lpass_cc_sdm845_clocks },
    num_clks: ARRAY_SIZE(lpass_cc_sdm845_clocks),
};

static mut lpass_qdsp6ss_sdm845_clocks: [*mut clk_regmap; 3] = [
    [LPASS_QDSP6SS_XO_CLK] = unsafe { &raw mut lpass_qdsp6ss_xo_clk.clkr },
    [LPASS_QDSP6SS_SLEEP_CLK] = unsafe { &raw mut lpass_qdsp6ss_sleep_clk.clkr },
    [LPASS_QDSP6SS_CORE_CLK] = unsafe { &raw mut lpass_qdsp6ss_core_clk.clkr },
];

static lpass_qdsp6ss_sdm845_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &raw mut lpass_regmap_config },
    clks: unsafe { &raw mut lpass_qdsp6ss_sdm845_clocks },
    num_clks: ARRAY_SIZE(lpass_qdsp6ss_sdm845_clocks),
};

unsafe fn lpass_cc_sdm845_probe(pdev: *mut platform_device) -> c_int {
    let mut desc: *const qcom_cc_desc;
    let ret: c_int;

    lpass_regmap_config.name = "cc";
    desc = &lpass_cc_sdm845_desc;

    ret = qcom_cc_probe_by_index(pdev, 0, desc);
    if ret != 0 {
        return ret;
    }

    lpass_regmap_config.name = "qdsp6ss";
    desc = &lpass_qdsp6ss_sdm845_desc;

    qcom_cc_probe_by_index(pdev, 1, desc)
}

static lpass_cc_sdm845_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sdm845-lpasscc" },
    of_device_id { ..Default::default() },
];

static mut lpass_cc_sdm845_driver: platform_driver = platform_driver {
    probe: Some(lpass_cc_sdm845_probe),
    driver: device_driver {
        name: "sdm845-lpasscc",
        of_match_table: lpass_cc_sdm845_match_table,
    },
};

unsafe fn lpass_cc_sdm845_init() -> c_int {
    platform_driver_register(&raw mut lpass_cc_sdm845_driver)
}

subsys_initcall!(lpass_cc_sdm845_init);

unsafe fn lpass_cc_sdm845_exit() {
    platform_driver_unregister(&raw mut lpass_cc_sdm845_driver);
}

module_exit!(lpass_cc_sdm845_exit);

module_description!("QTI LPASS_CC SDM845 Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
