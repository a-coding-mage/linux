// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Kernel and local clock-controller dependencies are supplied by other files.

enum {
    DT_AHB_CLK,
    DT_BI_TCXO,
    DT_SLEEP_CLK,
}

enum {
    P_BI_TCXO,
    P_EVA_CC_PLL0_OUT_MAIN,
    P_SLEEP_CLK,
}

static taycan_eko_t_vco: [pll_vco; 1] = [
    pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 },
];

/* 840.0 MHz Configuration */
static eva_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x2b,
    alpha: 0xc000,
    config_ctl_val: 0x25c400e7,
    config_ctl_hi_val: 0x0a8060e0,
    config_ctl_hi1_val: 0xf51dea20,
    user_ctl_val: 0x00000008,
    user_ctl_hi_val: 0x00000002,
};

static mut eva_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    config: &eva_cc_pll0_config,
    vco_table: &taycan_eko_t_vco,
    num_vco: ARRAY_SIZE(&taycan_eko_t_vco),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "eva_cc_pll0",
                parent_data: &clk_parent_data { index: DT_BI_TCXO },
                num_parents: 1,
                ops: &clk_alpha_pll_taycan_eko_t_ops,
            },
        },
    },
};

static eva_cc_parent_map_0: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];
static eva_cc_parent_data_0: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO }];

static eva_cc_parent_map_1: [parent_map; 2] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_EVA_CC_PLL0_OUT_MAIN, cfg: 1 },
];
static eva_cc_parent_data_1: [clk_parent_data; 2] = [
    clk_parent_data { index: DT_BI_TCXO },
    clk_parent_data { hw: unsafe { &eva_cc_pll0.clkr.hw } },
];

static eva_cc_parent_map_2: [parent_map; 1] = [parent_map { src: P_SLEEP_CLK, cfg: 0 }];
static eva_cc_parent_data_2: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK }];

static ftbl_eva_cc_ahb_clk_src: [freq_tbl; 2] = [
    F(19200000, P_BI_TCXO, 1, 0, 0),
    freq_tbl::default(),
];
static mut eva_cc_ahb_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x8018, mnd_width: 0, hid_width: 5,
    parent_map: &eva_cc_parent_map_0, freq_tbl: &ftbl_eva_cc_ahb_clk_src, hw_clk_ctrl: true,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_ahb_clk_src", parent_data: &eva_cc_parent_data_0,
        num_parents: ARRAY_SIZE(&eva_cc_parent_data_0), flags: CLK_SET_RATE_PARENT,
        ops: &clk_rcg2_shared_ops,
    } } },
};

static ftbl_eva_cc_mvs0_clk_src: [freq_tbl; 6] = [
    F(840000000, P_EVA_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1050000000, P_EVA_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1350000000, P_EVA_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1500000000, P_EVA_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1650000000, P_EVA_CC_PLL0_OUT_MAIN, 1, 0, 0),
    freq_tbl::default(),
];
static mut eva_cc_mvs0_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x8000, mnd_width: 0, hid_width: 5,
    parent_map: &eva_cc_parent_map_1, freq_tbl: &ftbl_eva_cc_mvs0_clk_src, hw_clk_ctrl: true,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_mvs0_clk_src", parent_data: &eva_cc_parent_data_1,
        num_parents: ARRAY_SIZE(&eva_cc_parent_data_1), flags: CLK_SET_RATE_PARENT,
        ops: &clk_rcg2_shared_ops,
    } } },
};

static ftbl_eva_cc_sleep_clk_src: [freq_tbl; 2] = [
    F(32000, P_SLEEP_CLK, 1, 0, 0), freq_tbl::default(),
];
static mut eva_cc_sleep_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x80e0, mnd_width: 0, hid_width: 5,
    parent_map: &eva_cc_parent_map_2, freq_tbl: &ftbl_eva_cc_sleep_clk_src, hw_clk_ctrl: true,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_sleep_clk_src", parent_data: &eva_cc_parent_data_2,
        num_parents: ARRAY_SIZE(&eva_cc_parent_data_2), flags: CLK_SET_RATE_PARENT,
        ops: &clk_rcg2_shared_ops,
    } } },
};

static mut eva_cc_xo_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x80bc, mnd_width: 0, hid_width: 5,
    parent_map: &eva_cc_parent_map_0, freq_tbl: &ftbl_eva_cc_ahb_clk_src, hw_clk_ctrl: true,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_xo_clk_src", parent_data: &eva_cc_parent_data_0,
        num_parents: ARRAY_SIZE(&eva_cc_parent_data_0), flags: CLK_SET_RATE_PARENT,
        ops: &clk_rcg2_shared_ops,
    } } },
};

static mut eva_cc_mvs0_div_clk_src: clk_regmap_div = clk_regmap_div {
    reg: 0x809c, shift: 0, width: 4,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_mvs0_div_clk_src", parent_hws: &[unsafe { &eva_cc_mvs0_clk_src.clkr.hw }],
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_regmap_div_ro_ops,
    } } },
};
static mut eva_cc_mvs0c_div2_div_clk_src: clk_regmap_div = clk_regmap_div {
    reg: 0x8060, shift: 0, width: 4,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "eva_cc_mvs0c_div2_div_clk_src", parent_hws: &[unsafe { &eva_cc_mvs0_clk_src.clkr.hw }],
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_regmap_div_ro_ops,
    } } },
};

macro_rules! branch { ($name:expr, $halt:expr, $check:expr, $parent:expr) => {
    clk_branch { halt_reg: $halt, halt_check: $check,
        clkr: clk_regmap { enable_reg: $halt, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
            name: $name, parent_hws: &[$parent], num_parents: 1,
            flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops,
        } } }, }
}; }

static mut eva_cc_mvs0_clk: clk_branch = branch!("eva_cc_mvs0_clk", 0x807c, BRANCH_HALT_VOTED, unsafe { &eva_cc_mvs0_div_clk_src.clkr.hw });
static mut eva_cc_mvs0_freerun_clk: clk_branch = branch!("eva_cc_mvs0_freerun_clk", 0x808c, BRANCH_HALT, unsafe { &eva_cc_mvs0_div_clk_src.clkr.hw });
static mut eva_cc_mvs0_shift_clk: clk_branch = branch!("eva_cc_mvs0_shift_clk", 0x80d8, BRANCH_HALT_VOTED, unsafe { &eva_cc_xo_clk_src.clkr.hw });
static mut eva_cc_mvs0c_clk: clk_branch = branch!("eva_cc_mvs0c_clk", 0x804c, BRANCH_HALT, unsafe { &eva_cc_mvs0c_div2_div_clk_src.clkr.hw });
static mut eva_cc_mvs0c_freerun_clk: clk_branch = branch!("eva_cc_mvs0c_freerun_clk", 0x805c, BRANCH_HALT, unsafe { &eva_cc_mvs0c_div2_div_clk_src.clkr.hw });
static mut eva_cc_mvs0c_shift_clk: clk_branch = branch!("eva_cc_mvs0c_shift_clk", 0x80dc, BRANCH_HALT_VOTED, unsafe { &eva_cc_xo_clk_src.clkr.hw });

static mut eva_cc_mvs0c_gdsc: gdsc = gdsc { gdscr: 0x8034, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: power_domain { name: "eva_cc_mvs0c_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE };
static mut eva_cc_mvs0_gdsc: gdsc = gdsc { gdscr: 0x8068, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: power_domain { name: "eva_cc_mvs0_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, parent: unsafe { &eva_cc_mvs0c_gdsc.pd } };

static mut eva_cc_glymur_clocks: [*mut clk_regmap; 13] = [
    &mut eva_cc_ahb_clk_src.clkr, &mut eva_cc_mvs0_clk.clkr, &mut eva_cc_mvs0_clk_src.clkr,
    &mut eva_cc_mvs0_div_clk_src.clkr, &mut eva_cc_mvs0_freerun_clk.clkr, &mut eva_cc_mvs0_shift_clk.clkr,
    &mut eva_cc_mvs0c_clk.clkr, &mut eva_cc_mvs0c_div2_div_clk_src.clkr, &mut eva_cc_mvs0c_freerun_clk.clkr,
    &mut eva_cc_mvs0c_shift_clk.clkr, &mut eva_cc_pll0.clkr, &mut eva_cc_sleep_clk_src.clkr,
    &mut eva_cc_xo_clk_src.clkr,
];
static mut eva_cc_glymur_gdscs: [*mut gdsc; 2] = [&mut eva_cc_mvs0_gdsc, &mut eva_cc_mvs0c_gdsc];
static eva_cc_glymur_resets: [qcom_reset_map; 5] = [
    qcom_reset_map { reg: 0x80a0, bit: 0 }, qcom_reset_map { reg: 0x8064, bit: 0 },
    qcom_reset_map { reg: 0x804c, bit: 2 }, qcom_reset_map { reg: 0x8030, bit: 0 },
    qcom_reset_map { reg: 0x805c, bit: 2 },
];
static mut eva_cc_glymur_plls: [*mut clk_alpha_pll; 1] = [&mut eva_cc_pll0];
static eva_cc_glymur_critical_cbcrs: [u32; 3] = [0x80a4, 0x80f8, 0x80d4];

static eva_cc_glymur_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f50, fast_io: true };

unsafe fn clk_glymur_regs_configure(dev: *mut device, regmap: *mut regmap) {
    /* Update CTRL_IN register as per HW recommendation to ensure clocks stay cycle-aligned when the EVA core is ON. */
    regmap_set_bits(regmap, 0x9f24, BIT(0));
}

static eva_cc_glymur_driver_data: qcom_cc_driver_data = qcom_cc_driver_data {
    alpha_plls: &eva_cc_glymur_plls, num_alpha_plls: ARRAY_SIZE(&eva_cc_glymur_plls),
    clk_cbcrs: &eva_cc_glymur_critical_cbcrs, num_clk_cbcrs: ARRAY_SIZE(&eva_cc_glymur_critical_cbcrs),
    clk_regs_configure: Some(clk_glymur_regs_configure),
};
static eva_cc_glymur_desc: qcom_cc_desc = qcom_cc_desc {
    config: &eva_cc_glymur_regmap_config, clks: &eva_cc_glymur_clocks, num_clks: ARRAY_SIZE(&eva_cc_glymur_clocks),
    resets: &eva_cc_glymur_resets, num_resets: ARRAY_SIZE(&eva_cc_glymur_resets),
    gdscs: &eva_cc_glymur_gdscs, num_gdscs: ARRAY_SIZE(&eva_cc_glymur_gdscs), use_rpm: true,
    driver_data: &eva_cc_glymur_driver_data,
};

static eva_cc_glymur_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,glymur-evacc" }, of_device_id::default(),
];

unsafe fn eva_cc_glymur_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &eva_cc_glymur_desc)
}

static mut eva_cc_glymur_driver: platform_driver = platform_driver {
    probe: Some(eva_cc_glymur_probe),
    driver: device_driver { name: "evacc-glymur", of_match_table: &eva_cc_glymur_match_table },
};

module_platform_driver!(eva_cc_glymur_driver);
MODULE_DEVICE_TABLE!(of, eva_cc_glymur_match_table);
MODULE_DESCRIPTION!("QTI EVACC Glymur Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
