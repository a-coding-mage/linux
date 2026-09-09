// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// The CONFIG_DRM_AMD_DC_FP conditional includes are supplied by other
// translation units when that build configuration is enabled.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_hardmin_params {
    // inputs
    pub ppclk: u32,
    pub freq_mhz: u16,
    // outputs
    pub response: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_hardmin_optimized_params {
    // inputs
    pub ppclk: u32,
    pub freq_khz: i32,
    // outputs
    pub response: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_deep_sleep_dcfclk_params {
    // inputs
    pub freq_mhz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_indicate_pstate_status_params {
    // inputs
    pub allow_fclk: bool,
    pub allow_uclk: bool,
    pub wait_resp: bool,
    pub drr_enable: bool,
    pub alt_ch_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_dppclk_dto_params {
    // inputs
    pub context: *mut dc_state,
    pub ref_dppclk_khz: *mut i32,
    pub safe_to_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_dtbclk_dto_params {
    // inputs
    pub context: *mut dc_state,
    pub ref_dtbclk_khz: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_dentist_params {
    // inputs
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_psr_wait_loop_params {
    // inputs
    pub dmcu: *mut dmcu,
    pub wait: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_stutter_efficiency_params {
    // inputs
    pub base_efficiency: u8,
    pub low_power_efficiency: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_clk_mgr_block_sequence_params_update_utm_qos_request_params {
    pub utm_urgent_bandwidth_lb_KBps: u32,
    pub utm_nominal_bandwidth_lb_KBps: u32,
    pub utm_lsdma_bandwidth_lb_KBps: u32,
    pub utm_latency_ub_index: u32,
}

#[repr(C)]
pub union dcn60_clk_mgr_block_sequence_params {
    pub update_hardmin_params: dcn60_clk_mgr_block_sequence_params_update_hardmin_params,
    pub update_hardmin_optimized_params: dcn60_clk_mgr_block_sequence_params_update_hardmin_optimized_params,
    pub update_deep_sleep_dcfclk_params: dcn60_clk_mgr_block_sequence_params_update_deep_sleep_dcfclk_params,
    pub indicate_pstate_status_params: dcn60_clk_mgr_block_sequence_params_indicate_pstate_status_params,
    pub update_dppclk_dto_params: dcn60_clk_mgr_block_sequence_params_update_dppclk_dto_params,
    pub update_dtbclk_dto_params: dcn60_clk_mgr_block_sequence_params_update_dtbclk_dto_params,
    pub update_dentist_params: dcn60_clk_mgr_block_sequence_params_update_dentist_params,
    pub update_psr_wait_loop_params: dcn60_clk_mgr_block_sequence_params_update_psr_wait_loop_params,
    pub update_stutter_efficiency_params: dcn60_clk_mgr_block_sequence_params_update_stutter_efficiency_params,
    pub update_utm_qos_request_params: dcn60_clk_mgr_block_sequence_params_update_utm_qos_request_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcn60_clk_mgr_block_sequence_func {
    CLK_MGR60_READ_CLOCKS_FROM_DENTIST,
    CLK_MGR60_UPDATE_HARDMIN_PPCLK,
    CLK_MGR60_UPDATE_HARDMIN_PPCLK_OPTIMIZED,
    CLK_MGR60_UPDATE_DEEP_SLEEP_DCFCLK,
    CLK_MGR60_INDICATE_PSTATE_STATUS,
    CLK_MGR60_UPDATE_DPPCLK_DTO,
    CLK_MGR60_UPDATE_DTBCLK_DTO,
    CLK_MGR60_UPDATE_DENTIST,
    CLK_MGR60_UPDATE_PSR_WAIT_LOOP,
    CLK_MGR60_UPDATE_STUTTER_EFFICIENCY,
    CLK_MGR60_UPDATE_UTM_QOS_REQUEST,
}

#[repr(C)]
pub struct dcn60_clk_mgr_block_sequence {
    pub params: dcn60_clk_mgr_block_sequence_params,
    pub func: dcn60_clk_mgr_block_sequence_func,
}

#[repr(C)]
pub struct dcn60_update_action {
    /// clk_mgr state needs updating
    pub update: bool,
    /// SMU message required
    pub send_message: bool,
}

#[repr(C)]
pub struct dcn60_enablement_action {
    /// feature transitioning to enabled
    pub enable: bool,
    /// feature transitioning to disabled
    pub disable: bool,
    /// SMU message required for this transition
    pub send_message: bool,
}

/**
 * struct dcn60_bandwidth_clocks_update_action - captures what clock and
 * p-state changes are needed for a bandwidth update, separating the action
 * logic from block sequence construction and state mutation.
 */
#[repr(C)]
pub struct dcn60_bandwidth_clocks_update_action {
    pub dcfclk: dcn60_update_action,
    pub deep_sleep_dcfclk: dcn60_update_action,
    pub socclk: dcn60_update_action,
    pub stutter: dcn60_update_action,
    pub utm_qos: dcn60_update_action,
    pub uclk_pstate: dcn60_enablement_action,
    pub fclk_pstate: dcn60_enablement_action,
    pub fams: dcn60_enablement_action,
    pub alt_ch: dcn60_enablement_action,
}

#[repr(C)]
pub struct dcn60_clk_mgr {
    pub base: clk_mgr_internal,
    pub block_sequence: [dcn60_clk_mgr_block_sequence; DCN401_CLK_MGR_MAX_SEQUENCE_SIZE],
    #[cfg(CONFIG_DRM_AMD_DC_FP)]
    pub utm_qos_model: utm_qos_model,
    #[cfg(CONFIG_DRM_AMD_DC_FP)]
    pub dchub_v3: utm_qos_model_dchub_v3,
    pub num_block_sequence_steps: u32,
}

extern "C" {
    pub fn dcn60_init_clocks(clk_mgr_base: *mut clk_mgr);
}

extern "C" {
    pub fn dcn60_build_clock_update_for_bls(
        clk_mgr_base: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
        seq_state: *mut block_sequence_state,
    );
    pub fn dcn60_clk_mgr_construct(
        ctx: *mut dc_context,
        dccg: *mut dccg,
    ) -> *mut clk_mgr_internal;
    pub fn dcn60_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
