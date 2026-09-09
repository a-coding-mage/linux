// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C header guard: __DCN401_CLK_MGR_H_

pub const DCN401_CLK_MGR_MAX_SEQUENCE_SIZE: usize = 30;

#[repr(C)]
pub union dcn401_clk_mgr_block_sequence_params {
    pub update_num_displays_params: update_num_displays_params,
    pub update_hardmin_params: update_hardmin_params,
    pub update_hardmin_optimized_params: update_hardmin_optimized_params,
    pub update_idle_hardmin_params: update_idle_hardmin_params,
    pub update_deep_sleep_dcfclk_params: update_deep_sleep_dcfclk_params,
    pub update_pstate_support_params: update_pstate_support_params,
    pub update_cab_for_uclk_params: update_cab_for_uclk_params,
    pub update_wait_for_dmub_ack_params: update_wait_for_dmub_ack_params,
    pub indicate_drr_status_params: indicate_drr_status_params,
    pub update_dppclk_dto_params: update_dppclk_dto_params,
    pub update_dtbclk_dto_params: update_dtbclk_dto_params,
    pub update_dentist_params: update_dentist_params,
    pub update_psr_wait_loop_params: update_psr_wait_loop_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_num_displays_params {
    // inputs
    pub num_displays: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_hardmin_params {
    // inputs
    pub ppclk: u32,
    pub freq_mhz: u16,
    // outputs
    pub response: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_hardmin_optimized_params {
    // inputs
    pub ppclk: u32,
    pub freq_khz: i32,
    // outputs
    pub response: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_idle_hardmin_params {
    // inputs
    pub uclk_mhz: u16,
    pub fclk_mhz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_deep_sleep_dcfclk_params {
    // inputs
    pub freq_mhz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_pstate_support_params {
    // inputs
    pub support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_cab_for_uclk_params {
    // inputs
    pub num_ways: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_wait_for_dmub_ack_params {
    // inputs
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct indicate_drr_status_params {
    // inputs
    pub mod_drr_for_pstate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_dppclk_dto_params {
    // inputs
    pub context: *mut dc_state,
    pub ref_dppclk_khz: *mut i32,
    pub safe_to_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_dtbclk_dto_params {
    // inputs
    pub context: *mut dc_state,
    pub ref_dtbclk_khz: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_dentist_params {
    // inputs
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_psr_wait_loop_params {
    // inputs
    pub dmcu: *mut dmcu,
    pub wait: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn401_clk_mgr_block_sequence_func {
    CLK_MGR401_READ_CLOCKS_FROM_DENTIST,
    CLK_MGR401_UPDATE_NUM_DISPLAYS,
    CLK_MGR401_UPDATE_HARDMIN_PPCLK,
    CLK_MGR401_UPDATE_HARDMIN_PPCLK_OPTIMIZED,
    CLK_MGR401_UPDATE_ACTIVE_HARDMINS,
    CLK_MGR401_UPDATE_IDLE_HARDMINS,
    CLK_MGR401_UPDATE_DEEP_SLEEP_DCFCLK,
    CLK_MGR401_UPDATE_FCLK_PSTATE_SUPPORT,
    CLK_MGR401_UPDATE_UCLK_PSTATE_SUPPORT,
    CLK_MGR401_UPDATE_CAB_FOR_UCLK,
    CLK_MGR401_UPDATE_WAIT_FOR_DMUB_ACK,
    CLK_MGR401_INDICATE_DRR_STATUS,
    CLK_MGR401_UPDATE_DPPCLK_DTO,
    CLK_MGR401_UPDATE_DTBCLK_DTO,
    CLK_MGR401_UPDATE_DENTIST,
    CLK_MGR401_UPDATE_PSR_WAIT_LOOP,
    CLK_MGR401_UPDATE_SUBVP_HARDMINS,
}

#[repr(C)]
pub struct dcn401_clk_mgr_block_sequence {
    pub params: dcn401_clk_mgr_block_sequence_params,
    pub func: dcn401_clk_mgr_block_sequence_func,
}

#[repr(C)]
pub struct dcn401_clk_mgr {
    pub base: clk_mgr_internal,
    pub block_sequence: [dcn401_clk_mgr_block_sequence; DCN401_CLK_MGR_MAX_SEQUENCE_SIZE],
    pub num_block_sequence_steps: libc::c_uint,
}

extern "C" {
    pub fn dcn401_init_clocks(clk_mgr_base: *mut clk_mgr);
    pub fn dcn401_is_dc_mode_present(clk_mgr_base: *mut clk_mgr) -> bool;
    pub fn dcn401_clk_mgr_construct(ctx: *mut dc_context, dccg: *mut dccg) -> *mut clk_mgr_internal;
    pub fn dcn401_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn401_get_max_clock_khz(clk_mgr_base: *mut clk_mgr, clk_type: clk_type) -> libc::c_uint;
}

pub struct block_sequence_state;

extern "C" {
    pub fn dcn401_build_clock_update_for_bls(
        clk_mgr_base: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
        seq_state: *mut block_sequence_state,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
