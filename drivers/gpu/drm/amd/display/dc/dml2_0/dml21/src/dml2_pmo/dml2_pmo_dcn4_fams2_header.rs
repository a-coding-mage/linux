// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

#[repr(C)]
pub struct display_configuation_with_meta {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pmo_initialize_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_optimize_dcc_mcache_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_init_for_vmin_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_test_for_vmin_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_optimize_for_vmin_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_init_for_pstate_support_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_test_for_pstate_support_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_optimize_for_pstate_support_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_init_for_stutter_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_test_for_stutter_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_optimize_for_stutter_in_out {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_pstate_strategy {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_init_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_pmo_scratch {
    _private: [u8; 0],
}

// External enum types supplied by dml2_internal_shared_types.h.
pub type dml2_pstate_method = i32;
pub type dml2_uclk_pstate_change_strategy = i32;

extern "C" {
    pub fn pmo_dcn4_fams2_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool;
    pub fn pmo_dcn4_fams2_optimize_dcc_mcache(
        in_out: *mut dml2_pmo_optimize_dcc_mcache_in_out,
    ) -> bool;

    pub fn pmo_dcn4_fams2_init_for_vmin(in_out: *mut dml2_pmo_init_for_vmin_in_out) -> bool;
    pub fn pmo_dcn4_fams2_test_for_vmin(in_out: *mut dml2_pmo_test_for_vmin_in_out) -> bool;
    pub fn pmo_dcn4_fams2_optimize_for_vmin(
        in_out: *mut dml2_pmo_optimize_for_vmin_in_out,
    ) -> bool;

    pub fn pmo_dcn4_fams2_init_for_pstate_support(
        in_out: *mut dml2_pmo_init_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn4_fams2_test_for_pstate_support(
        in_out: *mut dml2_pmo_test_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn4_fams2_optimize_for_pstate_support(
        in_out: *mut dml2_pmo_optimize_for_pstate_support_in_out,
    ) -> bool;

    pub fn pmo_dcn4_fams2_init_for_stutter(
        in_out: *mut dml2_pmo_init_for_stutter_in_out,
    ) -> bool;
    pub fn pmo_dcn4_fams2_test_for_stutter(
        in_out: *mut dml2_pmo_test_for_stutter_in_out,
    ) -> bool;
    pub fn pmo_dcn4_fams2_optimize_for_stutter(
        in_out: *mut dml2_pmo_optimize_for_stutter_in_out,
    ) -> bool;

    pub fn pmo_dcn4_fams2_expand_base_pstate_strategies(
        base_strategies_list: *const dml2_pmo_pstate_strategy,
        num_base_strategies: u32,
        stream_count: u32,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );

    // Helpers shared with derived PMO implementations (e.g. DCN42).
    pub fn dcn4_get_vactive_pstate_margin(
        display_cfg: *const display_configuation_with_meta,
        plane_mask: i32,
    ) -> i32;
    pub fn dcn4_get_minimum_reserved_time_us_for_planes(
        display_config: *const display_configuation_with_meta,
        plane_mask: i32,
    ) -> i32;
    pub fn dcn4_uclk_pstate_strategy_override_to_pstate_method(
        override_strategy: dml2_uclk_pstate_change_strategy,
    ) -> dml2_pstate_method;
    pub fn dcn4_get_expanded_strategy_list(
        init_data: *mut dml2_pmo_init_data,
        stream_count: i32,
    ) -> *mut dml2_pmo_pstate_strategy;
    pub fn dcn4_get_num_expanded_strategies(
        init_data: *mut dml2_pmo_init_data,
        stream_count: i32,
    ) -> u32;
    pub fn dcn4_insert_strategy_into_expanded_list(
        per_stream_pstate_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: i32,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );
    pub fn dcn4_expand_variant_strategy(
        base_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: u32,
        should_permute: bool,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );
    pub fn dcn4_insert_into_candidate_list(
        pstate_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: i32,
        scratch: *mut dml2_pmo_scratch,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
