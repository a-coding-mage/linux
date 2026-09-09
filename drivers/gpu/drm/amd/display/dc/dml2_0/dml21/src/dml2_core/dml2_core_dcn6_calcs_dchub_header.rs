// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency supplied by the surrounding translation unit:
// dml2_internal_shared_types.h

extern "C" {
    pub fn dcn6_calculate_max_vstartup(
        ptoi_supported: bool,
        vblank_nom_default_us: u32,
        timing: *const dml2_timing_cfg,
        pstate_strategy: dml2_uclk_pstate_change_strategy,
        write_back_delay_us: f64,
        svp_lines: u32,
    ) -> u32;

    pub fn dcn6_calculate_alternate_params(
        p: *mut dml2_core_calcs_calculate_alternate_params,
    );
    pub fn dcn6_calculate_alternate_svp_lines(
        p: *mut dml2_core_calcs_calculate_alternate_svp_lines,
    );
    pub fn dcn6_calculate_alternate_lead_lines(
        p: *mut dml2_core_calcs_calculate_alternate_lead_lines,
    );

    pub fn dcn6_calculate_flip_schedule(
        s: *mut dml2_core_internal_scratch,
        iflip_enable: bool,
        ihostvm_enable: bool,
        iffbm_enable: bool,
        HostVMInefficiencyFactor: f64,
        Tvm_trips_flip: f64,
        Tr0_trips_flip: f64,
        Tvm_trips_flip_rounded: f64,
        Tr0_trips_flip_rounded: f64,
        GPUVMEnable: bool,
        vm_bytes: f64, // vm_bytes
        DPTEBytesPerRow: f64, // dpte_row_bytes
        SourcePixelFormat: dml2_source_format_class,
        LineTime: f64,
        VRatio: f64,
        VRatioChroma: f64,
        Tno_bw_flip: f64,
        dpte_row_height: u32,
        dpte_row_height_chroma: u32,
        max_flip_time_us: u32,
        max_flip_time_lines: u32,
        meta_row_height: u32,
        meta_row_height_chroma: u32,

        // Output
        dst_y_per_vm_flip: *mut f64,
        dst_y_per_row_flip: *mut f64,
        final_flip_bw: *mut f64,
        ImmediateFlipSupportedForPipe: *mut bool,
    );

    pub fn dcn6_get_pipe_regs(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_dchub_per_pipe_register_set,
        pipe_index: i32,
        utm_soc_bb: *const dml2_utm_soc_bb,
        s: *mut dml2_core_internal_scratch,
    );

    pub fn dcn6_calculate_watermarks_and_dram_speed_change_support(
        scratch: *mut dml2_core_internal_scratch,
        p: *mut dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params,
    );

    pub fn dcn6_calculate_stutter_efficiency(
        scratch: *mut dml2_core_internal_scratch,
        p: *mut dml2_core_calcs_CalculateStutterEfficiency_params,
    );

    pub fn dcn6_get_watermarks(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        utm_soc_bb: *const dml2_utm_soc_bb,
        out: *mut dml2_dchub_watermark_regs,
    );

    pub fn dcn6_calculate_excess_vactive_bandwidth_required(
        display_cfg: *const dml2_display_cfg,
        bytes_required_l: *mut [[u32; DML2_MAX_PLANES]; dml2_pstate_type_count],
        bytes_required_c: *mut [[u32; DML2_MAX_PLANES]; dml2_pstate_type_count],
        // outputs
        excess_vactive_fill_bw_l: *mut f64,
        excess_vactive_fill_bw_c: *mut f64,
    );

    pub fn dcn6_calculate_pstate_schedule_windows(
        num_active_planes: i32,
        v_blank_start: *const u32,
        v_blank_end: *const u32,
        otg_vline_time_us: *const f64,
        det_fill_delay_us: *const f64,
        reserved_vblank_us: *const f64,
        blackout_us: f64,
        // Outputs
        allow_start_us: *mut f64,
        allow_end_us: *mut f64,
    );

    pub fn dcn6_calculate_pstate_schedule_admissibility(
        num_active_planes: u32,
        max_allow_delay_us: f64,
        min_allow_width_us: f64,
        timing_group_id: *const u32,
        timing_group_count: u32,
        frame_time_us: *const f64,
        allow_start_us: *const f64,
        allow_end_us: *const f64,
        pstate_method: *const dml2_pstate_method,
        drr_enabled: *const bool,
        // Output
        allow_window_us: *mut f64,
        disallow_window_us: *mut f64,
        pstate_admissible: *mut bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
