// SPDX-License-Identifier: MIT
//
// Copyright 2024-2025 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

unsafe extern "C" {
    pub fn set_bit_in_bitfield(bit_field: *mut u32, bit_offset: u32);
    pub fn is_bit_set_in_bitfield(bit_field: u32, bit_offset: u32) -> bool;
    pub fn dcn5_get_vactive_pstate_margin(
        validation_res: *const dml2_validation_result,
        plane_mask: i32,
    ) -> i32;
    pub fn dcn5_build_method_scheduling_params(
        stream_method_pstate_meta: *mut dml2_pstate_per_method_common_meta,
        stream_pstate_meta: *const dml2_pstate_meta,
    );
    pub fn dcn5_build_synchronized_timing_groups(
        // Output
        s: *mut dml2_pmo_synchronized_timing_groups,
        // Input
        display_config: *const dml2_display_cfg,
    );
    pub fn dcn5_insert_strategy_into_expanded_list(
        per_stream_pstate_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: i32,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );
    pub fn dcn5_is_variant_method_valid(
        base_strategy: *const dml2_pmo_pstate_strategy,
        variant_strategy: *const dml2_pmo_pstate_strategy,
        num_streams_per_base_method: *const u32,
        num_streams_per_variant_method: *const u32,
        stream_count: u32,
    ) -> bool;
    pub fn dcn5_expand_base_strategy(
        base_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: u32,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );
    pub fn dcn5_expand_variant_strategy(
        base_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: u32,
        should_permute: bool,
        expanded_strategy_list: *mut dml2_pmo_pstate_strategy,
        num_expanded_strategies: *mut u32,
    );
    pub fn dcn5_get_expanded_strategy_list(
        stage: *mut dml2_pmo_stage_optimizer,
        stream_count: i32,
    ) -> *const dml2_pmo_pstate_strategy;
    pub fn dcn5_get_num_expanded_strategies(
        stage: *mut dml2_pmo_stage_optimizer,
        stream_count: i32,
    ) -> u32;
    pub fn dcn5_stream_matches_drr_policy(
        stage: *mut dml2_pmo_stage_optimizer,
        display_cfg: *const dml2_display_cfg,
        stream_pstate_method: dml2_pstate_method,
        stream_index: u32,
    ) -> bool;
    pub fn dcn5_all_timings_support_vactive(
        stage: *mut dml2_pmo_stage_optimizer,
        display_config: *const dml2_display_cfg,
        mask: u32,
    ) -> bool;
    pub fn dcn5_all_timings_support_vblank(
        stage: *mut dml2_pmo_stage_optimizer,
        display_config: *const dml2_display_cfg,
        mask: u32,
    ) -> bool;
    pub fn dcn5_all_timings_support_drr(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *const dml2_optimization_worksheet,
        display_config: *const dml2_display_cfg,
        mask: u32,
    ) -> bool;
    pub fn dcn5_insert_into_candidate_list(
        pstate_strategy: *const dml2_pmo_pstate_strategy,
        stream_count: i32,
        worksheet: *mut dml2_optimization_worksheet,
    );
    pub fn dcn5_reset_worksheet_for_uclk_pstate(worksheet: *mut dml2_optimization_worksheet);
    pub fn dcn5_setup_planes_for_vactive_by_mask(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
        plane_mask: i32,
    );
    pub fn dcn5_setup_planes_for_vblank_by_mask(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
        plane_mask: i32,
    );
    pub fn dcn5_setup_planes_for_vactive_drr_by_mask(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
        plane_mask: i32,
    );
    pub fn dcn5_setup_planes_for_vblank_drr_by_mask(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
        plane_mask: i32,
    );
    pub fn dcn5_setup_planes_for_drr_by_mask(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
        plane_mask: i32,
    );
    pub fn dcn5_get_vactive_det_fill_latency_delay_us(
        validation_res: *const dml2_validation_result,
        plane_mask: i32,
    ) -> i32;
    pub fn dcn5_get_minimum_reserved_time_us_for_planes(
        worksheet: *const dml2_optimization_worksheet,
        plane_mask: i32,
    ) -> i32;

    /* Public DCN5 PMO optimizers */
    pub fn dml2_pmo_dcn5_stage_optimizer_qos_create(
        pmo_inst: *mut dml2_pmo_instance,
        optimizer: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_create(
        pmo_inst: *mut dml2_pmo_instance,
        optimizer: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_uclk_pstate_create(
        pmo_inst: *mut dml2_pmo_instance,
        optimizer: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_vmin_create(
        pmo_inst: *mut dml2_pmo_instance,
        optimizer: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_stutter_create(
        pmo_inst: *mut dml2_pmo_instance,
        optimizer: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_init(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
    );
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_test_total_mcache_limit(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *const dml2_optimization_worksheet,
    ) -> bool;
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_test_mcache_status(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *const dml2_optimization_worksheet,
    ) -> bool;
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_increment_pipe_usage(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
    ) -> bool;
    pub fn dml2_pmo_dcn5_stage_optimizer_mcache_apply_default_pipe_usage(
        stage: *mut dml2_pmo_stage_optimizer,
        worksheet: *mut dml2_optimization_worksheet,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
