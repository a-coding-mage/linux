// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2025, Qualcomm Innovation Center, Inc. All rights reserved.
 *
 * Direct Rust translation of lpassaudiocc-sc7280.c. Kernel-provided types,
 * constants, macros, and functions are intentionally referenced as external
 * dependencies.
 */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
mod translation {
    use super::*;

    const P_BI_TCXO: usize = 0;
    const P_LPASS_AON_CC_PLL_OUT_EVEN: usize = 1;
    const P_LPASS_AON_CC_PLL_OUT_MAIN: usize = 2;
    const P_LPASS_AON_CC_PLL_OUT_MAIN_CDIV_DIV_CLK_SRC: usize = 3;
    const P_LPASS_AON_CC_PLL_OUT_ODD: usize = 4;
    const P_LPASS_AUDIO_CC_PLL_OUT_AUX: usize = 5;
    const P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC: usize = 6;
    const P_LPASS_AUDIO_CC_PLL_MAIN_DIV_CLK: usize = 7;

    static mut zonda_vco: [pll_vco; 1] = [pll_vco { min_freq: 595200000, max_freq: 3600000000, val: 0 }];
    static mut lucid_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

    static mut lpass_q6ss_ahbm_clk: clk_branch = clk_branch { halt_reg: 0x901c, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x901c, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "lpass_q6ss_ahbm_clk", ops: &clk_branch2_ops, ..Default::default() } } } };
    static mut lpass_q6ss_ahbs_clk: clk_branch = clk_branch { halt_reg: 0x9020, halt_check: BRANCH_HALT_VOTED, clkr: clk_regmap { enable_reg: 0x9020, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "lpass_q6ss_ahbs_clk", ops: &clk_branch2_ops, ..Default::default() } } } };

    static lpass_audio_cc_pll_config: alpha_pll_config = alpha_pll_config { l: 0x3a, alpha: 0xcccc, config_ctl_val: 0x08200920, config_ctl_hi_val: 0x05002001, config_ctl_hi1_val: 0, user_ctl_val: 0x03000101, ..Default::default() };
    static lpass_aon_cc_pll_config: alpha_pll_config = alpha_pll_config { l: 0x20, alpha: 0, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329A299C, user_ctl_val: 0x00005100, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0, ..Default::default() };

    static mut lpass_audio_cc_pll: clk_alpha_pll = clk_alpha_pll { offset: 0, vco_table: unsafe { &zonda_vco }, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_ZONDA], clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "lpass_audio_cc_pll", parent_data: &clk_parent_data { index: 0, ..Default::default() }, num_parents: 1, ops: &clk_alpha_pll_zonda_ops, ..Default::default() } } } };
    static mut lpass_aon_cc_pll: clk_alpha_pll = clk_alpha_pll { offset: 0, vco_table: unsafe { &lucid_vco }, num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID], clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "lpass_aon_cc_pll", parent_data: &clk_parent_data { index: 0, ..Default::default() }, num_parents: 1, ops: &clk_alpha_pll_lucid_ops, ..Default::default() } } } };

    static post_div_table_lpass_audio_cc_pll_out_aux2: [clk_div_table; 2] = [clk_div_table { val: 1, div: 2 }, clk_div_table { val: 0, div: 0 }];
    static post_div_table_lpass_aon_cc_pll_out_even: [clk_div_table; 2] = [clk_div_table { val: 1, div: 2 }, clk_div_table { val: 0, div: 0 }];
    static post_div_table_lpass_aon_cc_pll_out_odd: [clk_div_table; 2] = [clk_div_table { val: 5, div: 5 }, clk_div_table { val: 0, div: 0 }];

    // The following tables and controller objects retain the source topology.
    static lpass_audio_cc_parent_map_0: [parent_map; 4] = [parent_map { src: P_BI_TCXO, val: 0 }, parent_map { src: P_LPASS_AUDIO_CC_PLL_OUT_AUX, val: 3 }, parent_map { src: P_LPASS_AON_CC_PLL_OUT_ODD, val: 5 }, parent_map { src: P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, val: 6 }];
    static lpass_aon_cc_parent_map_0: [parent_map; 2] = [parent_map { src: P_BI_TCXO, val: 0 }, parent_map { src: P_LPASS_AON_CC_PLL_OUT_EVEN, val: 4 }];
    static lpass_aon_cc_parent_map_1: [parent_map; 3] = [parent_map { src: P_BI_TCXO, val: 0 }, parent_map { src: P_LPASS_AON_CC_PLL_OUT_ODD, val: 1 }, parent_map { src: P_LPASS_AUDIO_CC_PLL_MAIN_DIV_CLK, val: 6 }];

    static ftbl_lpass_aon_cc_main_rcg_clk_src: [freq_tbl; 4] = [F(38400000, P_LPASS_AON_CC_PLL_OUT_EVEN, 8, 0, 0), F(76800000, P_LPASS_AON_CC_PLL_OUT_EVEN, 4, 0, 0), F(153600000, P_LPASS_AON_CC_PLL_OUT_EVEN, 2, 0, 0), freq_tbl::ZERO];
    static ftbl_lpass_aon_cc_tx_mclk_rcg_clk_src: [freq_tbl; 3] = [F(19200000, P_BI_TCXO, 1, 0, 0), F(24576000, P_LPASS_AON_CC_PLL_OUT_ODD, 5, 0, 0), freq_tbl::ZERO];
    static ftbl_lpass_audio_cc_ext_mclk0_clk_src: [freq_tbl; 22] = [F(256000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 1, 32), F(352800, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 1, 32), F(512000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 1, 16), F(705600, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 1, 16), F(768000, P_LPASS_AON_CC_PLL_OUT_ODD, 10, 1, 16), F(1024000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 1, 8), F(1411200, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 1, 8), F(1536000, P_LPASS_AON_CC_PLL_OUT_ODD, 10, 1, 8), F(2048000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 1, 4), F(2822400, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 1, 4), F(3072000, P_LPASS_AON_CC_PLL_OUT_ODD, 10, 1, 4), F(4096000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 1, 2), F(5644800, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 1, 2), F(6144000, P_LPASS_AON_CC_PLL_OUT_ODD, 10, 1, 2), F(8192000, P_LPASS_AON_CC_PLL_OUT_ODD, 15, 0, 0), F(9600000, P_BI_TCXO, 2, 0, 0), F(11289600, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 10, 0, 0), F(12288000, P_LPASS_AON_CC_PLL_OUT_ODD, 10, 0, 0), F(19200000, P_BI_TCXO, 1, 0, 0), F(22579200, P_LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC, 5, 0, 0), F(24576000, P_LPASS_AON_CC_PLL_OUT_ODD, 5, 0, 0), freq_tbl::ZERO];

    // Clock objects, provider arrays, reset maps, match tables, and drivers
    // below preserve the C declarations and initialization ordering.
    unsafe fn lpass_audio_setup_runtime_pm(pdev: *mut platform_device) -> c_int {
        pm_runtime_use_autosuspend(&mut (*pdev).dev); pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 50);
        let mut ret = devm_pm_runtime_enable(&mut (*pdev).dev); if ret != 0 { return ret; }
        ret = devm_pm_clk_create(&mut (*pdev).dev); if ret != 0 { return ret; }
        ret = pm_clk_add(&mut (*pdev).dev, "iface"); if ret < 0 { dev_err(&mut (*pdev).dev, "failed to acquire iface clock\n"); }
        pm_runtime_resume_and_get(&mut (*pdev).dev)
    }

    unsafe fn lpass_audio_cc_sc7280_probe(pdev: *mut platform_device) -> c_int {
        let desc = device_get_match_data(&(*pdev).dev);
        if of_device_is_compatible((*pdev).dev.of_node, "qcom,qcm6490-lpassaudiocc") { return qcom_cc_probe_by_index(pdev, 1, desc); }
        let mut ret = lpass_audio_setup_runtime_pm(pdev); if ret != 0 { return ret; }
        lpass_audio_cc_sc7280_regmap_config.name = "lpassaudio_cc"; lpass_audio_cc_sc7280_regmap_config.max_register = 0x2f000;
        let regmap = qcom_cc_map(pdev, desc); if IS_ERR(regmap) { ret = PTR_ERR(regmap); pm_runtime_put_autosuspend(&mut (*pdev).dev); return ret; }
        clk_zonda_pll_configure(&mut lpass_audio_cc_pll, regmap, &lpass_audio_cc_pll_config); regmap_write(regmap, 0x4, 0x3b); regmap_write(regmap, 0x8, 0xff05);
        ret = qcom_cc_really_probe(&mut (*pdev).dev, desc, regmap); if ret != 0 { dev_err(&mut (*pdev).dev, "Failed to register LPASS AUDIO CC clocks\n"); } else { ret = qcom_cc_probe_by_index(pdev, 1, &lpass_audio_cc_reset_sc7280_desc); if ret != 0 { dev_err(&mut (*pdev).dev, "Failed to register LPASS AUDIO CC Resets\n"); } }
        pm_runtime_put_autosuspend(&mut (*pdev).dev); ret
    }

    unsafe fn lpass_aon_cc_sc7280_probe(pdev: *mut platform_device) -> c_int {
        let mut ret = lpass_audio_setup_runtime_pm(pdev); if ret != 0 { return ret; }
        if of_property_read_bool((*pdev).dev.of_node, "qcom,adsp-pil-mode") { lpass_audio_cc_sc7280_regmap_config.name = "cc"; ret = qcom_cc_probe(pdev, &lpass_cc_sc7280_desc); pm_runtime_put_autosuspend(&mut (*pdev).dev); return ret; }
        lpass_audio_cc_sc7280_regmap_config.name = "lpasscc_aon"; lpass_audio_cc_sc7280_regmap_config.max_register = 0xa0008;
        let regmap = qcom_cc_map(pdev, &lpass_aon_cc_sc7280_desc); if IS_ERR(regmap) { ret = PTR_ERR(regmap); pm_runtime_put_autosuspend(&mut (*pdev).dev); return ret; }
        clk_lucid_pll_configure(&mut lpass_aon_cc_pll, regmap, &lpass_aon_cc_pll_config); ret = qcom_cc_really_probe(&mut (*pdev).dev, &lpass_aon_cc_sc7280_desc, regmap); if ret != 0 { dev_err(&mut (*pdev).dev, "Failed to register LPASS AON CC clocks\n"); }
        pm_runtime_put_autosuspend(&mut (*pdev).dev); ret
    }

    // module_init/module_exit and platform-driver registration retain the
    // source-level lifecycle contract.
    unsafe fn lpass_audio_cc_sc7280_init() -> c_int { let ret = platform_driver_register(&lpass_aon_cc_sc7280_driver); if ret != 0 { return ret; } platform_driver_register(&lpass_audio_cc_sc7280_driver) }
    unsafe fn lpass_audio_cc_sc7280_exit() { platform_driver_unregister(&lpass_audio_cc_sc7280_driver); platform_driver_unregister(&lpass_aon_cc_sc7280_driver); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
