/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2014, The Linux Foundation. All rights reserved. */

// Forward declarations supplied by other translation units.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct qcom_reset_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct freq_tbl {
    _private: [u8; 0],
}
#[repr(C)]
pub struct freq_multi_tbl {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}
#[repr(C)]
pub struct alpha_pll {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_rcg_dfs_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gdsc {
    _private: [u8; 0],
}

pub const PLL_LOCK_COUNT_SHIFT: u32 = 8;
pub const PLL_LOCK_COUNT_MASK: u32 = 0x3f;
pub const PLL_BIAS_COUNT_SHIFT: u32 = 14;
pub const PLL_BIAS_COUNT_MASK: u32 = 0x3f;
pub const PLL_VOTE_FSM_ENA: u32 = 1u32 << 20;
pub const PLL_VOTE_FSM_RESET: u32 = 1u32 << 21;

#[repr(C)]
pub struct qcom_icc_hws_data {
    pub master_id: i32,
    pub slave_id: i32,
    pub clk_id: i32,
}

#[repr(C)]
pub struct qcom_cc_driver_data {
    pub alpha_plls: *mut *mut alpha_pll,
    pub num_alpha_plls: usize,
    pub clk_cbcrs: *const u32,
    pub num_clk_cbcrs: usize,
    pub dfs_rcgs: *const clk_rcg_dfs_data,
    pub num_dfs_rcgs: usize,
    pub clk_regs_configure: Option<unsafe extern "C" fn(*mut device, *mut regmap)>,
}

#[repr(C)]
pub struct qcom_cc_desc {
    pub config: *const regmap_config,
    pub clks: *mut *mut clk_regmap,
    pub num_clks: usize,
    pub resets: *const qcom_reset_map,
    pub num_resets: usize,
    pub gdscs: *mut *mut gdsc,
    pub num_gdscs: usize,
    pub clk_hws: *mut *mut clk_hw,
    pub num_clk_hws: usize,
    pub icc_hws: *const qcom_icc_hws_data,
    pub num_icc_hws: usize,
    pub icc_first_node_id: u32,
    pub use_rpm: bool,
    pub driver_data: *const qcom_cc_driver_data,
}

/**
 * struct parent_map - map table for source select configuration values
 * @src: source
 * @cfg: configuration value
 */
#[repr(C)]
pub struct parent_map {
    pub src: u8,
    pub cfg: u8,
}

unsafe extern "C" {
    pub fn qcom_find_freq(f: *const freq_tbl, rate: u64) -> *const freq_tbl;
    pub fn qcom_find_freq_floor(f: *const freq_tbl, rate: u64) -> *const freq_tbl;
    pub fn qcom_find_freq_multi(f: *const freq_multi_tbl, rate: u64) -> *const freq_multi_tbl;
    pub fn qcom_pll_set_fsm_mode(m: *mut regmap, reg: u32, bias_count: u8, lock_count: u8);
    pub fn qcom_find_src_index(hw: *mut clk_hw, map: *const parent_map, src: u8) -> i32;
    pub fn qcom_find_cfg_index(hw: *mut clk_hw, map: *const parent_map, cfg: u8) -> i32;

    pub fn qcom_cc_register_board_clk(
        dev: *mut device,
        path: *const core::ffi::c_char,
        name: *const core::ffi::c_char,
        rate: u64,
    ) -> i32;
    pub fn qcom_cc_register_sleep_clk(dev: *mut device) -> i32;

    pub fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    pub fn qcom_cc_really_probe(
        dev: *mut device,
        desc: *const qcom_cc_desc,
        regmap: *mut regmap,
    ) -> i32;
    pub fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
    pub fn qcom_cc_probe_by_index(
        pdev: *mut platform_device,
        index: i32,
        desc: *const qcom_cc_desc,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
