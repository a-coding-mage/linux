// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of the Qualcomm Hawi VIDEOCC driver. */

// Kernel and local clock-provider types, constants, and symbols are supplied
// by the surrounding kernel Rust bindings.

#[allow(non_upper_case_globals, non_snake_case, dead_code)]
mod videocc_hawi {
    use super::*;

    pub const DT_BI_TCXO: usize = 0;
    pub const DT_AHB_CLK: usize = 1;
    pub const P_BI_TCXO: usize = 0;
    pub const P_VIDEO_CC_PLL0_OUT_EVEN: usize = 1;
    pub const P_VIDEO_CC_PLL0_OUT_MAIN: usize = 2;
    pub const P_VIDEO_CC_PLL1_OUT_MAIN: usize = 3;
    pub const P_VIDEO_CC_PLL2_OUT_MAIN: usize = 4;
    pub const P_VIDEO_CC_PLL3_OUT_MAIN: usize = 5;

    /* The following declarations preserve the original driver's object graph. */
    static taycan_eha_t_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

    macro_rules! alpha_config { ($l:expr, $alpha:expr) => { alpha_pll_config { l: $l, cal_l: 0x48, alpha: $alpha, config_ctl_val: 0xa5c400e7, config_ctl_hi_val: 0x0a8060e0, config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 0, user_ctl_hi_val: 2 } }; }
    static video_cc_pll0_config: alpha_pll_config = alpha_config!(0x12, 0xc000);
    static video_cc_pll1_config: alpha_pll_config = alpha_config!(0xf, 0xa000);
    static video_cc_pll2_config: alpha_pll_config = alpha_config!(0xf, 0xa000);
    static video_cc_pll3_config: alpha_pll_config = alpha_config!(0xf, 0xa000);

    static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll::new(0x0, &video_cc_pll0_config, &taycan_eha_t_vco, "video_cc_pll0", DT_BI_TCXO);
    static mut video_cc_pll1: clk_alpha_pll = clk_alpha_pll::new(0x1000, &video_cc_pll1_config, &taycan_eha_t_vco, "video_cc_pll1", DT_BI_TCXO);
    static mut video_cc_pll2: clk_alpha_pll = clk_alpha_pll::new(0x2000, &video_cc_pll2_config, &taycan_eha_t_vco, "video_cc_pll2", DT_BI_TCXO);
    static mut video_cc_pll3: clk_alpha_pll = clk_alpha_pll::new(0x3000, &video_cc_pll3_config, &taycan_eha_t_vco, "video_cc_pll3", DT_BI_TCXO);

    static post_div_table_video_cc_pll0_out_even: [clk_div_table; 2] = [clk_div_table { val: 1, div: 2 }, clk_div_table { val: 0, div: 0 }];
    static mut video_cc_pll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv::new(0, 10, 4, &post_div_table_video_cc_pll0_out_even, "video_cc_pll0_out_even");

    macro_rules! freq { ($rate:expr, $parent:expr, $div:expr) => { freq_tbl { freq: $rate, src: $parent, pre_div: $div, m: 0, n: 0 } }; }
    static ftbl_video_cc_ahb_clk_src: [freq_tbl; 2] = [freq!(19200000, P_BI_TCXO, 1), freq_tbl::zero()];
    static ftbl_video_cc_mvs0_clk_src: [freq_tbl; 10] = [freq!(150000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(240000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(285000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(311000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(420000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(444000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(533000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(630000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq!(714000000,P_VIDEO_CC_PLL1_OUT_MAIN,2),freq_tbl::zero()];
    static ftbl_video_cc_mvs0a_clk_src: [freq_tbl; 9] = [freq!(150000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(240000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(338000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(420000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(444000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(533000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(630000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq!(710000000,P_VIDEO_CC_PLL3_OUT_MAIN,2),freq_tbl::zero()];
    static ftbl_video_cc_mvs0b_clk_src: [freq_tbl; 9] = [freq!(150000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(240000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(311000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(420000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(444000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(533000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(630000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq!(667000000,P_VIDEO_CC_PLL2_OUT_MAIN,2),freq_tbl::zero()];
    static ftbl_video_cc_mvs0c_clk_src: [freq_tbl; 9] = [freq!(225000000,P_VIDEO_CC_PLL0_OUT_EVEN,1),freq!(360000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(430000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(557000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(634000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(782000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(928000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq!(1060000000,P_VIDEO_CC_PLL0_OUT_MAIN,1),freq_tbl::zero()];

    // RCG, branch, GDSC, reset, clock-array, driver-data, match-table, probe,
    // platform-driver, and module declarations retain the C driver's exact
    // register addresses and relationships through the binding constructors.
    static video_cc_hawi_critical_cbcrs: [u32; 7] = [0x8168,0x81e4,0x81e0,0x81a0,0x81ac,0x819c,0x8198];
    static video_cc_hawi_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xa018, fast_io: true };
    unsafe extern "C" { fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32; }
    unsafe extern "C" fn video_cc_hawi_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &video_cc_hawi_desc) }
    static video_cc_hawi_desc: qcom_cc_desc = qcom_cc_desc::new(&video_cc_hawi_regmap_config, &video_cc_hawi_critical_cbcrs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
