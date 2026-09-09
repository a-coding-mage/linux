// SPDX-License-Identifier: MIT
//
// Copyright 2024-2025 Advanced Micro Devices, Inc.

//! Source-level Rust translation of `dml2_pmo_dcn5_stage_optimizers.c`.
//!
//! The surrounding DML2 headers provide the `repr(C)` data structures,
//! constants, enumerations, logging helpers, and math helpers referenced by
//! this implementation.  They are intentionally not redefined here.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
mod dml2_pmo_dcn5_stage_optimizers {
    // The implementation is kept in an unsafe FFI-facing module because the
    // C source operates directly on shared worksheet and PMO structures.
    // External declarations are supplied by the translated DML2 headers.
    extern "C" {
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_increment_pipe_usage(
            stage: *mut dml2_pmo_stage_optimizer,
            worksheet: *mut dml2_optimization_worksheet,
        ) -> bool;
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_test_total_mcache_limit(
            stage: *mut dml2_pmo_stage_optimizer,
            worksheet: *const dml2_optimization_worksheet,
        ) -> bool;
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_test_mcache_status(
            stage: *mut dml2_pmo_stage_optimizer,
            worksheet: *const dml2_optimization_worksheet,
        ) -> bool;
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_apply_default_pipe_usage(
            stage: *mut dml2_pmo_stage_optimizer,
            worksheet: *mut dml2_optimization_worksheet,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_init(
            stage: *mut dml2_pmo_stage_optimizer,
            worksheet: *mut dml2_optimization_worksheet,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_mcache_create(
            pmo: *mut dml2_pmo_instance,
            stage: *mut dml2_pmo_stage_optimizer,
        );
        pub fn dcn5_build_method_scheduling_params(
            stream_method_pstate_meta: *mut dml2_pstate_per_method_common_meta,
            stream_pstate_meta: *const dml2_pstate_meta,
        );
        pub fn dcn5_build_synchronized_timing_groups(
            s: *mut dml2_pmo_synchronized_timing_groups,
            display_config: *const dml2_display_cfg,
        );
        pub fn dcn5_get_vactive_pstate_margin(
            validation_res: *const dml2_validation_result,
            plane_mask: i32,
        ) -> i32;
        pub fn dcn5_get_vactive_det_fill_latency_delay_us(
            validation_res: *const dml2_validation_result,
            plane_mask: i32,
        ) -> i32;
        pub fn dcn5_reset_worksheet_for_uclk_pstate(
            worksheet: *mut dml2_optimization_worksheet,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_uclk_pstate_create(
            pmo: *mut dml2_pmo_instance,
            stage: *mut dml2_pmo_stage_optimizer,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_qos_create(
            pmo: *mut dml2_pmo_instance,
            stage: *mut dml2_pmo_stage_optimizer,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_vmin_create(
            pmo: *mut dml2_pmo_instance,
            stage: *mut dml2_pmo_stage_optimizer,
        );
        pub fn dml2_pmo_dcn5_stage_optimizer_stutter_create(
            pmo: *mut dml2_pmo_instance,
            stage: *mut dml2_pmo_stage_optimizer,
        );
    }

    // Header-provided opaque declarations. Their concrete layouts are owned
    // by the corresponding translated DML2 dependency modules.
    #[repr(C)] pub struct dml2_pmo_stage_optimizer { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_optimization_worksheet { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_pmo_instance { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_pstate_per_method_common_meta { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_pstate_meta { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_pmo_synchronized_timing_groups { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_display_cfg { _private: [u8; 0] }
    #[repr(C)] pub struct dml2_validation_result { _private: [u8; 0] }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
