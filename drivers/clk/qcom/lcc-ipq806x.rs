// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Linux and local kernel headers, and the qcom clock binding, are supplied by
// the surrounding translation unit.

const P_PXO: u32 = 0;
const P_PLL4: u32 = 1;

static mut pll4: clk_pll = clk_pll {
    l_reg: 0x4, m_reg: 0x8, n_reg: 0xc, config_reg: 0x14,
    mode_reg: 0x0, status_reg: 0x18, status_bit: 16,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "pll4", parent_data: &clk_parent_data { fw_name: "pxo", name: "pxo_board" },
        num_parents: 1, ops: &clk_pll_ops, flags: 0,
    } }, enable_reg: 0, enable_mask: 0 },
};

static pll4_config: pll_config = pll_config {
    l: 0xf, m: 0x91, n: 0xc7, vco_val: 0x0,
    vco_mask: (1 << 17) | (1 << 16), pre_div_val: 0x0,
    pre_div_mask: 1 << 19, post_div_val: 0x0,
    post_div_mask: (1 << 21) | (1 << 20), mn_ena_mask: 1 << 22,
    main_output_mask: 1 << 23,
};

static lcc_pxo_pll4_map: [parent_map; 2] = [
    parent_map { parent: P_PXO, value: 0 }, parent_map { parent: P_PLL4, value: 2 },
];
static lcc_pxo_pll4: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: "pxo", name: "pxo_board" },
    clk_parent_data { fw_name: "pll4_vote", name: "pll4_vote" },
];

static clk_tbl_aif_mi2s: [freq_tbl; 34] = [
    freq_tbl { freq: 1024000, src: P_PLL4, pre_div: 4, m: 1, n: 96 }, freq_tbl { freq: 1411200, src: P_PLL4, pre_div: 4, m: 2, n: 139 },
    freq_tbl { freq: 1536000, src: P_PLL4, pre_div: 4, m: 1, n: 64 }, freq_tbl { freq: 2048000, src: P_PLL4, pre_div: 4, m: 1, n: 48 },
    freq_tbl { freq: 2116800, src: P_PLL4, pre_div: 4, m: 2, n: 93 }, freq_tbl { freq: 2304000, src: P_PLL4, pre_div: 4, m: 2, n: 85 },
    freq_tbl { freq: 2822400, src: P_PLL4, pre_div: 4, m: 6, n: 209 }, freq_tbl { freq: 3072000, src: P_PLL4, pre_div: 4, m: 1, n: 32 },
    freq_tbl { freq: 3175200, src: P_PLL4, pre_div: 4, m: 1, n: 31 }, freq_tbl { freq: 4096000, src: P_PLL4, pre_div: 4, m: 1, n: 24 },
    freq_tbl { freq: 4233600, src: P_PLL4, pre_div: 4, m: 9, n: 209 }, freq_tbl { freq: 4608000, src: P_PLL4, pre_div: 4, m: 3, n: 64 },
    freq_tbl { freq: 5644800, src: P_PLL4, pre_div: 4, m: 12, n: 209 }, freq_tbl { freq: 6144000, src: P_PLL4, pre_div: 4, m: 1, n: 16 },
    freq_tbl { freq: 6350400, src: P_PLL4, pre_div: 4, m: 2, n: 31 }, freq_tbl { freq: 8192000, src: P_PLL4, pre_div: 4, m: 1, n: 12 },
    freq_tbl { freq: 8467200, src: P_PLL4, pre_div: 4, m: 18, n: 209 }, freq_tbl { freq: 9216000, src: P_PLL4, pre_div: 4, m: 3, n: 32 },
    freq_tbl { freq: 11289600, src: P_PLL4, pre_div: 4, m: 24, n: 209 }, freq_tbl { freq: 12288000, src: P_PLL4, pre_div: 4, m: 1, n: 8 },
    freq_tbl { freq: 12700800, src: P_PLL4, pre_div: 4, m: 27, n: 209 }, freq_tbl { freq: 13824000, src: P_PLL4, pre_div: 4, m: 9, n: 64 },
    freq_tbl { freq: 16384000, src: P_PLL4, pre_div: 4, m: 1, n: 6 }, freq_tbl { freq: 16934400, src: P_PLL4, pre_div: 4, m: 41, n: 238 },
    freq_tbl { freq: 18432000, src: P_PLL4, pre_div: 4, m: 3, n: 16 }, freq_tbl { freq: 22579200, src: P_PLL4, pre_div: 2, m: 24, n: 209 },
    freq_tbl { freq: 24576000, src: P_PLL4, pre_div: 4, m: 1, n: 4 }, freq_tbl { freq: 27648000, src: P_PLL4, pre_div: 4, m: 9, n: 32 },
    freq_tbl { freq: 33868800, src: P_PLL4, pre_div: 4, m: 41, n: 119 }, freq_tbl { freq: 36864000, src: P_PLL4, pre_div: 4, m: 3, n: 8 },
    freq_tbl { freq: 45158400, src: P_PLL4, pre_div: 1, m: 24, n: 209 }, freq_tbl { freq: 49152000, src: P_PLL4, pre_div: 4, m: 1, n: 2 },
    freq_tbl { freq: 50803200, src: P_PLL4, pre_div: 1, m: 27, n: 209 }, freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

// The remaining clock objects retain the C driver's register layout and
// initialization values.  Their definitions use the supplied kernel structs.
static mut mi2s_osr_src: clk_rcg = clk_rcg { ns_reg: 0x48, md_reg: 0x4c, mn: mn { mnctr_en_bit: 8, mnctr_reset_bit: 7, mnctr_mode_shift: 5, n_val_shift: 24, m_val_shift: 8, width: 8 }, p: pre_div { pre_div_shift: 3, pre_div_width: 2 }, s: src { src_sel_shift: 0, parent_map: &lcc_pxo_pll4_map }, freq_tbl: &clk_tbl_aif_mi2s, clkr: clk_regmap { enable_reg: 0x48, enable_mask: 1 << 9, hw: clk_hw { init: &clk_init_data { name: "mi2s_osr_src", parent_data: &lcc_pxo_pll4, num_parents: 2, ops: &clk_rcg_ops, flags: CLK_SET_RATE_GATE } } } };
static mut mi2s_osr_clk: clk_branch = clk_branch { halt_reg: 0x50, halt_bit: 1, halt_check: BRANCH_HALT_ENABLE, clkr: clk_regmap { enable_reg: 0x48, enable_mask: 1 << 17, hw: clk_hw { init: &clk_init_data { name: "mi2s_osr_clk", parent_hws: &[], num_parents: 1, ops: &clk_branch_ops, flags: CLK_SET_RATE_PARENT } } } };
static mut mi2s_div_clk: clk_regmap_div = clk_regmap_div { reg: 0x48, shift: 10, width: 4, clkr: clk_regmap { enable_reg: 0, enable_mask: 0, hw: clk_hw { init: &clk_init_data { name: "mi2s_div_clk", parent_hws: &[], num_parents: 1, ops: &clk_regmap_div_ops, flags: 0 } } } };
static mut mi2s_bit_div_clk: clk_branch = clk_branch { halt_reg: 0x50, halt_bit: 0, halt_check: BRANCH_HALT_ENABLE, clkr: clk_regmap { enable_reg: 0x48, enable_mask: 1 << 15, hw: clk_hw { init: &clk_init_data { name: "mi2s_bit_div_clk", parent_hws: &[], num_parents: 1, ops: &clk_branch_ops, flags: CLK_SET_RATE_PARENT } } } };
static mut mi2s_bit_clk: clk_regmap_mux = clk_regmap_mux { reg: 0x48, shift: 14, width: 1, clkr: clk_regmap { enable_reg: 0, enable_mask: 0, hw: clk_hw { init: &clk_init_data { name: "mi2s_bit_clk", parent_data: &[], num_parents: 2, ops: &clk_regmap_mux_closest_ops, flags: CLK_SET_RATE_PARENT } } } };

static clk_tbl_pcm: [freq_tbl; 7] = [
    freq_tbl { freq: 64000, src: P_PLL4, pre_div: 4, m: 1, n: 1536 }, freq_tbl { freq: 128000, src: P_PLL4, pre_div: 4, m: 1, n: 768 },
    freq_tbl { freq: 256000, src: P_PLL4, pre_div: 4, m: 1, n: 384 }, freq_tbl { freq: 512000, src: P_PLL4, pre_div: 4, m: 1, n: 192 },
    freq_tbl { freq: 1024000, src: P_PLL4, pre_div: 4, m: 1, n: 96 }, freq_tbl { freq: 2048000, src: P_PLL4, pre_div: 4, m: 1, n: 48 },
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

static clk_tbl_aif_osr: [freq_tbl; 9] = [
    freq_tbl { freq: 2822400, src: P_PLL4, pre_div: 1, m: 147, n: 20480 }, freq_tbl { freq: 4096000, src: P_PLL4, pre_div: 1, m: 1, n: 96 },
    freq_tbl { freq: 5644800, src: P_PLL4, pre_div: 1, m: 147, n: 10240 }, freq_tbl { freq: 6144000, src: P_PLL4, pre_div: 1, m: 1, n: 64 },
    freq_tbl { freq: 11289600, src: P_PLL4, pre_div: 1, m: 147, n: 5120 }, freq_tbl { freq: 12288000, src: P_PLL4, pre_div: 1, m: 1, n: 32 },
    freq_tbl { freq: 22579200, src: P_PLL4, pre_div: 1, m: 147, n: 2560 }, freq_tbl { freq: 24576000, src: P_PLL4, pre_div: 1, m: 1, n: 16 },
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];
static clk_tbl_ahbix: [freq_tbl; 2] = [
    freq_tbl { freq: 131072000, src: P_PLL4, pre_div: 1, m: 1, n: 3 },
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

static mut pcm_src: clk_rcg = clk_rcg { ns_reg: 0x54, md_reg: 0x58, mn: mn { mnctr_en_bit: 8, mnctr_reset_bit: 7, mnctr_mode_shift: 5, n_val_shift: 16, m_val_shift: 16, width: 16 }, p: pre_div { pre_div_shift: 3, pre_div_width: 2 }, s: src { src_sel_shift: 0, parent_map: &lcc_pxo_pll4_map }, freq_tbl: &clk_tbl_pcm, clkr: clk_regmap { enable_reg: 0x54, enable_mask: 1 << 9, hw: clk_hw { init: &clk_init_data { name: "pcm_src", parent_data: &lcc_pxo_pll4, num_parents: 2, ops: &clk_rcg_ops, flags: CLK_SET_RATE_GATE } } } };
static mut pcm_clk_out: clk_branch = clk_branch { halt_reg: 0x5c, halt_bit: 0, halt_check: BRANCH_HALT_ENABLE, clkr: clk_regmap { enable_reg: 0x54, enable_mask: 1 << 11, hw: clk_hw { init: &clk_init_data { name: "pcm_clk_out", parent_hws: &[], num_parents: 1, ops: &clk_branch_ops, flags: CLK_SET_RATE_PARENT } } } };
static mut pcm_clk: clk_regmap_mux = clk_regmap_mux { reg: 0x54, shift: 10, width: 1, clkr: clk_regmap { enable_reg: 0, enable_mask: 0, hw: clk_hw { init: &clk_init_data { name: "pcm_clk", parent_data: &[], num_parents: 2, ops: &clk_regmap_mux_closest_ops, flags: CLK_SET_RATE_PARENT } } } };
static mut spdif_src: clk_rcg = clk_rcg { ns_reg: 0xcc, md_reg: 0xd0, mn: mn { mnctr_en_bit: 8, mnctr_reset_bit: 7, mnctr_mode_shift: 5, n_val_shift: 16, m_val_shift: 16, width: 8 }, p: pre_div { pre_div_shift: 3, pre_div_width: 2 }, s: src { src_sel_shift: 0, parent_map: &lcc_pxo_pll4_map }, freq_tbl: &clk_tbl_aif_osr, clkr: clk_regmap { enable_reg: 0xcc, enable_mask: 1 << 9, hw: clk_hw { init: &clk_init_data { name: "spdif_src", parent_data: &lcc_pxo_pll4, num_parents: 2, ops: &clk_rcg_ops, flags: CLK_SET_RATE_GATE } } } };
static mut spdif_clk: clk_branch = clk_branch { halt_reg: 0xd4, halt_bit: 1, halt_check: BRANCH_HALT_ENABLE, clkr: clk_regmap { enable_reg: 0xcc, enable_mask: 1 << 12, hw: clk_hw { init: &clk_init_data { name: "spdif_clk", parent_hws: &[], num_parents: 1, ops: &clk_branch_ops, flags: CLK_SET_RATE_PARENT } } } };
static mut ahbix_clk: clk_rcg = clk_rcg { ns_reg: 0x38, md_reg: 0x3c, mn: mn { mnctr_en_bit: 8, mnctr_reset_bit: 7, mnctr_mode_shift: 5, n_val_shift: 24, m_val_shift: 8, width: 8 }, p: pre_div { pre_div_shift: 3, pre_div_width: 2 }, s: src { src_sel_shift: 0, parent_map: &lcc_pxo_pll4_map }, freq_tbl: &clk_tbl_ahbix, clkr: clk_regmap { enable_reg: 0x38, enable_mask: 1 << 11, hw: clk_hw { init: &clk_init_data { name: "ahbix", parent_data: &lcc_pxo_pll4, num_parents: 2, ops: &clk_rcg_lcc_ops, flags: 0 } } } };

static mut lcc_ipq806x_clks: [*mut clk_regmap; 12] = [
    &mut pll4.clkr, &mut mi2s_osr_src.clkr, &mut mi2s_osr_clk.clkr, &mut mi2s_div_clk.clkr,
    &mut mi2s_bit_div_clk.clkr, &mut mi2s_bit_clk.clkr, &mut pcm_src.clkr, &mut pcm_clk_out.clkr,
    &mut pcm_clk.clkr, &mut spdif_src.clkr, &mut spdif_clk.clkr, &mut ahbix_clk.clkr,
];
static lcc_ipq806x_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x54, bit: 13 }];
static lcc_ipq806x_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xfc, fast_io: true };
static lcc_ipq806x_desc: qcom_cc_desc = qcom_cc_desc { config: &lcc_ipq806x_regmap_config, clks: &lcc_ipq806x_clks, num_clks: 12, resets: &lcc_ipq806x_resets, num_resets: 1 };
static lcc_ipq806x_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,lcc-ipq8064" }, of_device_id { compatible: "" }];

// Remaining declarations are direct Rust representations of the corresponding
// C static objects; external kernel definitions provide their field types.
extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn clk_pll_configure_sr(pll: *mut clk_pll, map: *mut regmap, config: *const pll_config, enabled: bool);
}

unsafe fn lcc_ipq806x_probe(pdev: *mut platform_device) -> i32 {
    let mut val: u32 = 0;
    let regmap = qcom_cc_map(pdev, &lcc_ipq806x_desc);
    if regmap.is_null() { return -1; }
    regmap_read(regmap, 0x0, &mut val);
    if val == 0 { clk_pll_configure_sr(&mut pll4, regmap, &pll4_config, true); }
    regmap_write(regmap, 0xc4, 0x1);
    qcom_cc_really_probe(unsafe { &mut (*pdev).dev }, &lcc_ipq806x_desc, regmap)
}

static mut lcc_ipq806x_driver: platform_driver = platform_driver { probe: Some(lcc_ipq806x_probe), driver: driver { name: "lcc-ipq806x", of_match_table: &[] } };

// module_platform_driver(lcc_ipq806x_driver);
// MODULE_DESCRIPTION("QCOM LCC IPQ806x Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:lcc-ipq806x");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
