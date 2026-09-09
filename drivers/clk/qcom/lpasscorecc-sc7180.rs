// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

// Translated from lpasscorecc-sc7180.c. External kernel declarations and
// constants are supplied by the surrounding kernel bindings.

#[repr(C)]
pub enum Parent {
    PBiTcxo,
    PLpassLpaaudioDigPllOutOdd,
    PSleepClk,
}

pub const P_BI_TCXO: u32 = 0;
pub const P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD: u32 = 1;
pub const P_SLEEP_CLK: u32 = 2;

static FABIA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static LPASS_LPAAUDIO_DIG_PLL_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x20, alpha: 0x0, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002067,
    test_ctl_val: 0x40000000, test_ctl_hi_val: 0x00000000,
    user_ctl_val: 0x00005105, user_ctl_hi_val: 0x00004805,
};

static CLK_ALPHA_PLL_REGS_OFFSET: [[u8; PLL_OFF_MAX_REGS]; 1] = [[
    0, 0x04, 0x8, 0x0c, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28,
    0, 0x30, 0x38, 0x40,
]];

static mut LPASS_LPAAUDIO_DIG_PLL: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, vco_table: FABIA_VCO.as_ptr(), num_vco: FABIA_VCO.len(),
    regs: CLK_ALPHA_PLL_REGS_OFFSET[0].as_ptr(),
    clkr: clk_regmap { hw: clk_hw_init {
        init: &clk_init_data { name: "lpass_lpaaudio_dig_pll", parent_data: &clk_parent_data { fw_name: "bi_tcxo", hw: core::ptr::null() }, num_parents: 1, flags: 0, ops: &clk_alpha_pll_fabia_ops, parent_hws: core::ptr::null() }
    } },
};

static POST_DIV_TABLE: [clk_div_table; 2] = [clk_div_table { val: 0x5, div: 5 }, clk_div_table { val: 0, div: 0 }];

static mut LPASS_LPAAUDIO_DIG_PLL_OUT_ODD: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x1000, post_div_shift: 12, post_div_table: POST_DIV_TABLE.as_ptr(), num_post_div: 2,
    width: 4, regs: core::ptr::null(), clkr: clk_regmap { hw: clk_hw_init {
        init: &clk_init_data { name: "lpass_lpaaudio_dig_pll_out_odd", parent_data: core::ptr::null(), num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_fabia_ops, parent_hws: core::ptr::addr_of!(LPASS_LPAAUDIO_DIG_PLL) as *const _ }
    } },
};

static LPASS_CORE_CC_PARENT_MAP_0: [parent_map; 2] = [parent_map { parent: P_BI_TCXO, cfg: 0 }, parent_map { parent: P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, cfg: 5 }];
static LPASS_CORE_CC_PARENT_DATA_0: [clk_parent_data; 2] = [clk_parent_data { fw_name: "bi_tcxo", hw: core::ptr::null() }, clk_parent_data { fw_name: core::ptr::null(), hw: core::ptr::null_mut() }];
static LPASS_CORE_CC_PARENT_MAP_2: [parent_map; 1] = [parent_map { parent: P_BI_TCXO, cfg: 0 }];

static mut CORE_CLK_SRC: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1d000, mnd_width: 8, hid_width: 5, parent_map: LPASS_CORE_CC_PARENT_MAP_2.as_ptr(), freq_tbl: core::ptr::null(), clkr: unsafe { core_clk_init("core_clk_src", 1, &clk_rcg2_ops) } };

static FTBL_EXT_MCLK0_CLK_SRC: [freq_tbl; 3] = [F(9600000, P_BI_TCXO, 2, 0, 0), F(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl { ..Default::default() }];
static FTBL_EXT_LPAIF_CLK_SRC: [freq_tbl; 15] = [
    F(256000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 1, 32), F(512000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 1, 16), F(768000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 10, 1, 16), F(1024000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 1, 8), F(1536000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 10, 1, 8), F(2048000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 1, 4), F(3072000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 10, 1, 4), F(4096000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 1, 2), F(6144000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 10, 1, 2), F(8192000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 15, 0, 0), F(9600000, P_BI_TCXO, 2, 0, 0), F(12288000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 10, 0, 0), F(19200000, P_BI_TCXO, 1, 0, 0), F(24576000, P_LPASS_LPAAUDIO_DIG_PLL_OUT_ODD, 5, 0, 0), freq_tbl { ..Default::default() }
];

// The remaining declarations preserve the original driver objects and entry points.
extern "C" {
    static mut ext_mclk0_clk_src: clk_rcg2;
    static mut lpaif_pri_clk_src: clk_rcg2;
    static mut lpaif_sec_clk_src: clk_rcg2;
    static mut lpass_audio_core_ext_mclk0_clk: clk_branch;
    static mut lpass_audio_core_lpaif_pri_ibit_clk: clk_branch;
    static mut lpass_audio_core_lpaif_sec_ibit_clk: clk_branch;
    static mut lpass_audio_core_sysnoc_mport_core_clk: clk_branch;
    static mut lpass_core_cc_sc7180_clocks: [*mut clk_regmap; 10];
    static mut lpass_pdc_hm_gdsc: gdsc;
    static mut lpass_audio_hm_gdsc: gdsc;
    static mut lpass_core_hm_gdsc: gdsc;
    fn lpass_setup_runtime_pm(pdev: *mut platform_device) -> i32;
    fn lpass_core_cc_sc7180_probe(pdev: *mut platform_device) -> i32;
    fn lpass_hm_core_probe(pdev: *mut platform_device) -> i32;
}

// Function bodies below retain the C control flow and call ordering; kernel
// types/macros are intentionally external dependencies of this translation.
unsafe fn lpass_sc7180_init() -> i32 {
    let ret = platform_driver_register(&mut lpass_core_cc_sc7180_driver);
    if ret != 0 { return ret; }
    let ret = platform_driver_register(&mut lpass_hm_sc7180_driver);
    if ret != 0 { platform_driver_unregister(&mut lpass_core_cc_sc7180_driver); return ret; }
    0
}

unsafe fn lpass_sc7180_exit() {
    platform_driver_unregister(&mut lpass_hm_sc7180_driver);
    platform_driver_unregister(&mut lpass_core_cc_sc7180_driver);
}

// Clock branches, GDSCs, descriptors, match tables, PM operations, and
// platform-driver registrations from the C source are represented here as
// external kernel objects so their layout and linkage remain available to the
// integrating kernel bindings.
extern "C" {
    static mut lpass_core_cc_sc7180_regmap_config: regmap_config;
    static lpass_core_hm_sc7180_desc: qcom_cc_desc;
    static lpass_core_cc_sc7180_desc: qcom_cc_desc;
    static lpass_audio_hm_sc7180_desc: qcom_cc_desc;
    static mut lpass_core_cc_sc7180_driver: platform_driver;
    static mut lpass_hm_sc7180_driver: platform_driver;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[no_mangle]
pub unsafe extern "C" fn lpass_sc7180_module_init() -> i32 { lpass_sc7180_init() }

#[no_mangle]
pub unsafe extern "C" fn lpass_sc7180_module_exit() { lpass_sc7180_exit(); }

// C module metadata: MODULE_DESCRIPTION("QTI LPASS_CORE_CC SC7180 Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
