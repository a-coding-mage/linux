/* SPDX-License-Identifier: MIT */
/* Copyright 2018-2026 Advanced Micro Devices, Inc. */
/* Translated from clk_mgr_internal.h; dependencies are supplied by the surrounding tree. */

// C preprocessor register-list and token-pasting macros are retained as declarative
// Rust macro placeholders because their register symbols are generated externally.
macro_rules! to_clk_mgr_internal { ($clk_mgr:expr) => { $clk_mgr }; }
macro_rules! ctx { ($clk_mgr:expr) => { $clk_mgr.base.ctx }; }
macro_rules! dc_logger { ($dc:expr) => { $dc.ctx.logger }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dentist_base_divider_id {
    DENTIST_BASE_DID_1 = 0x08,
    DENTIST_BASE_DID_2 = 0x40,
    DENTIST_BASE_DID_3 = 0x60,
    DENTIST_BASE_DID_4 = 0x7e,
    DENTIST_MAX_DID = 0x7f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dentist_divider_range {
    DENTIST_DIVIDER_RANGE_1_START = 8,
    DENTIST_DIVIDER_RANGE_1_STEP = 1,
    DENTIST_DIVIDER_RANGE_2_START = 64,
    DENTIST_DIVIDER_RANGE_2_STEP = 2,
    DENTIST_DIVIDER_RANGE_3_START = 128,
    DENTIST_DIVIDER_RANGE_3_STEP = 4,
    DENTIST_DIVIDER_RANGE_4_START = 248,
    DENTIST_DIVIDER_RANGE_4_STEP = 264,
    DENTIST_DIVIDER_RANGE_SCALE_FACTOR = 4,
}

#[repr(C)]
pub struct clk_mgr_registers {
    pub DPREFCLK_CNTL: u32, pub DENTIST_DISPCLK_CNTL: u32,
    pub CLK4_CLK2_CURRENT_CNT: u32, pub CLK4_CLK_PLL_REQ: u32, pub CLK4_CLK0_CURRENT_CNT: u32,
    pub CLK3_CLK2_DFS_CNTL: u32, pub CLK3_CLK_PLL_REQ: u32,
    pub CLK0_CLK2_DFS_CNTL: u32, pub CLK0_CLK_PLL_REQ: u32,
    pub CLK1_CLK_PLL_REQ: u32, pub CLK1_CLK0_DFS_CNTL: u32, pub CLK1_CLK1_DFS_CNTL: u32,
    pub CLK1_CLK2_DFS_CNTL: u32, pub CLK1_CLK3_DFS_CNTL: u32, pub CLK1_CLK4_DFS_CNTL: u32,
    pub CLK1_CLK5_DFS_CNTL: u32, pub CLK2_CLK2_DFS_CNTL: u32,
    pub CLK1_CLK0_CURRENT_CNT: u32, pub CLK1_CLK1_CURRENT_CNT: u32, pub CLK1_CLK2_CURRENT_CNT: u32,
    pub CLK1_CLK3_CURRENT_CNT: u32, pub CLK1_CLK4_CURRENT_CNT: u32, pub CLK1_CLK5_CURRENT_CNT: u32,
    pub CLK0_CLK0_DFS_CNTL: u32, pub CLK0_CLK1_DFS_CNTL: u32, pub CLK0_CLK3_DFS_CNTL: u32, pub CLK0_CLK4_DFS_CNTL: u32,
    pub CLK1_CLK0_BYPASS_CNTL: u32, pub CLK1_CLK1_BYPASS_CNTL: u32, pub CLK1_CLK2_BYPASS_CNTL: u32,
    pub CLK1_CLK3_BYPASS_CNTL: u32, pub CLK1_CLK4_BYPASS_CNTL: u32, pub CLK1_CLK5_BYPASS_CNTL: u32,
    pub CLK1_CLK0_DS_CNTL: u32, pub CLK1_CLK1_DS_CNTL: u32, pub CLK1_CLK2_DS_CNTL: u32,
    pub CLK1_CLK3_DS_CNTL: u32, pub CLK1_CLK4_DS_CNTL: u32, pub CLK1_CLK5_DS_CNTL: u32,
    pub CLK1_CLK0_ALLOW_DS: u32, pub CLK1_CLK1_ALLOW_DS: u32, pub CLK1_CLK2_ALLOW_DS: u32,
    pub CLK1_CLK3_ALLOW_DS: u32, pub CLK1_CLK4_ALLOW_DS: u32, pub CLK1_CLK5_ALLOW_DS: u32,
    pub CLK5_spll_field_8: u32, pub CLK6_spll_field_8: u32,
    pub CLK8_CLK_TICK_CNT_CONFIG_REG: u32, pub CLK8_CLK0_CURRENT_CNT: u32, pub CLK8_CLK1_CURRENT_CNT: u32,
    pub CLK8_CLK2_CURRENT_CNT: u32, pub CLK8_CLK3_CURRENT_CNT: u32, pub CLK8_CLK4_CURRENT_CNT: u32,
    pub CLK8_CLK0_BYPASS_CNTL: u32, pub CLK8_CLK1_BYPASS_CNTL: u32, pub CLK8_CLK2_BYPASS_CNTL: u32,
    pub CLK8_CLK3_BYPASS_CNTL: u32, pub CLK8_CLK4_BYPASS_CNTL: u32,
    pub CLK8_CLK0_DS_CNTL: u32, pub CLK8_CLK1_DS_CNTL: u32, pub CLK8_CLK2_DS_CNTL: u32,
    pub CLK8_CLK3_DS_CNTL: u32, pub CLK8_CLK4_DS_CNTL: u32,
    pub CLK5_CLK_TICK_CNT_CONFIG_REG: u32, pub CLK5_CLK0_CURRENT_CNT: u32, pub CLK5_CLK1_CURRENT_CNT: u32,
    pub CLK5_CLK2_CURRENT_CNT: u32, pub CLK5_CLK3_CURRENT_CNT: u32, pub CLK5_CLK0_DS_CNTL: u32,
    pub CLK5_CLK1_DS_CNTL: u32, pub CLK5_CLK2_DS_CNTL: u32, pub CLK5_CLK3_DS_CNTL: u32,
    pub CLK5_CLK0_BYPASS_CNTL: u32, pub CLK5_CLK1_BYPASS_CNTL: u32, pub CLK5_CLK2_BYPASS_CNTL: u32,
    pub CLK5_CLK3_BYPASS_CNTL: u32, pub CLK8_CLK0_ALLOW_DS: u32,
}

// CLK_REG_FIELD_LIST, CLK20_REG_FIELD_LIST, CLK42_REG_LIST and CLK42_REG_FIELD_LIST
// are C type/identifier-generation macros; their expanded fields are represented here.
#[repr(C)] pub struct clk_mgr_shift { pub DPREFCLK_SRC_SEL:u8, pub DENTIST_DPREFCLK_WDIVIDER:u8, pub DENTIST_DISPCLK_WDIVIDER:u8, pub DENTIST_DISPCLK_CHG_DONE:u8, pub DENTIST_DPPCLK_WDIVIDER:u8, pub DENTIST_DPPCLK_CHG_DONE:u8, pub FbMult_int:u8, pub FbMult_frac:u8, pub TIMER_THRESHOLD:u8, pub CLK0_BYPASS_SEL:u8, pub CLK1_BYPASS_SEL:u8, pub CLK2_BYPASS_SEL:u8, pub CLK3_BYPASS_SEL:u8, pub CLK4_BYPASS_SEL:u8, pub CLK0_DS_DIV_ID:u8, pub CLK1_DS_DIV_ID:u8, pub CLK2_DS_DIV_ID:u8, pub CLK3_DS_DIV_ID:u8, pub CLK4_DS_DIV_ID:u8, pub CLK0_ALLOW_DS:u8, pub CLK1_ALLOW_DS:u8, pub CLK2_ALLOW_DS:u8, pub CLK3_ALLOW_DS:u8, pub CLK4_ALLOW_DS:u8 }
#[repr(C)] pub struct clk_mgr_mask { pub DPREFCLK_SRC_SEL:u32, pub DENTIST_DPREFCLK_WDIVIDER:u32, pub DENTIST_DISPCLK_WDIVIDER:u32, pub DENTIST_DISPCLK_CHG_DONE:u32, pub DENTIST_DPPCLK_WDIVIDER:u32, pub DENTIST_DPPCLK_CHG_DONE:u32, pub FbMult_int:u32, pub FbMult_frac:u32, pub TIMER_THRESHOLD:u32, pub CLK0_BYPASS_SEL:u32, pub CLK1_BYPASS_SEL:u32, pub CLK2_BYPASS_SEL:u32, pub CLK3_BYPASS_SEL:u32, pub CLK4_BYPASS_SEL:u32, pub CLK0_DS_DIV_ID:u32, pub CLK1_DS_DIV_ID:u32, pub CLK2_DS_DIV_ID:u32, pub CLK3_DS_DIV_ID:u32, pub CLK4_DS_DIV_ID:u32, pub CLK0_ALLOW_DS:u32, pub CLK1_ALLOW_DS:u32, pub CLK2_ALLOW_DS:u32, pub CLK3_ALLOW_DS:u32, pub CLK4_ALLOW_DS:u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum clock_type { clock_type_dispclk=1, clock_type_dcfclk, clock_type_socclk, clock_type_pixelclk, clock_type_phyclk, clock_type_dppclk, clock_type_fclk, clock_type_dcfdsclk, clock_type_dscclk, clock_type_uclk, clock_type_dramclk, clock_type_dprefclk, clock_type_dtbclk }

#[repr(C)]
pub struct clk_mgr_internal {
    pub base: clk_mgr, pub smu_ver: i32, pub pp_smu: *mut pp_smu_funcs, pub funcs: *mut clk_mgr_internal_funcs, pub dccg: *mut dccg,
    pub regs: *const clk_mgr_registers, pub clk_mgr_shift: *const clk_mgr_shift, pub clk_mgr_mask: *const clk_mgr_mask,
    pub dfs_bypass_enabled: bool, pub dfs_bypass_active: bool, pub dfs_ref_freq_khz: u32, pub dfs_bypass_disp_clk: i32,
    pub ss_on_dprefclk: bool, pub xgmi_enabled: bool, pub dprefclk_ss_percentage: i32, pub dprefclk_ss_divider: i32,
    pub periodic_retraining_disabled: bool, pub cur_phyclk_req_table: [u32; MAX_LINKS], pub smu_present: bool,
    pub wm_range_table: *mut core::ffi::c_void, pub wm_range_table_addr: i64, pub dal_init_table: *const core::ffi::c_void,
    pub dal_init_table_addr: i64, pub dpm_present: bool, pub pme_trigger_pending: bool,
}

#[repr(C)] pub struct clk_mgr_internal_funcs { pub set_dispclk: Option<unsafe extern "C" fn(*mut clk_mgr_internal, i32) -> i32>, pub set_dprefclk: Option<unsafe extern "C" fn(*mut clk_mgr_internal)> }

#[inline] pub fn should_set_clock(safe_to_lower: bool, calc_clk: i32, cur_clk: i32) -> bool { (safe_to_lower && calc_clk < cur_clk) || calc_clk > cur_clk }
#[inline] pub fn should_update_pstate_support(safe_to_lower: bool, calc_support: bool, cur_support: bool) -> bool { if cur_support != calc_support { (calc_support && safe_to_lower) || (!calc_support && !safe_to_lower) } else { false } }
#[inline] pub fn khz_to_mhz_ceil(khz: i32) -> i32 { (khz + 999) / 1000 }
#[inline] pub fn khz_to_mhz_floor(khz: i32) -> i32 { khz / 1000 }

extern "C" { pub fn clk_mgr_helper_get_active_display_cnt(dc: *mut dc, context: *mut dc_state) -> i32; pub fn clk_mgr_helper_get_active_plane_cnt(dc: *mut dc, context: *mut dc_state) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
