// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml2_core_dcn5_calcs_dchub.h.
// Dependency declarations are supplied by dml2_internal_shared_types.

extern "C" {
    pub fn dcn5_calculate_max_det_and_min_compressed_buffer_size(
        ConfigReturnBufferSizeInKByte: ::core::ffi::c_uint,
        ConfigReturnBufferSegmentSizeInKByte: ::core::ffi::c_uint,
        ROBBufferSizeInKByte: ::core::ffi::c_uint,
        MaxNumDPP: ::core::ffi::c_uint,
        nomDETInKByteOverrideEnable: ::core::ffi::c_uint,
        nomDETInKByteOverrideValue: ::core::ffi::c_uint,
        is_mrq_present: bool,
        MaxTotalDETInKByte: *mut ::core::ffi::c_uint,
        nomDETInKByte: *mut ::core::ffi::c_uint,
        MinCompressedBufferSizeInKByte: *mut ::core::ffi::c_uint,
    );

    // ???
    pub fn dcn5_adjust_pixel_clock_for_progressive_to_interlace_unit(
        display_cfg: *const dml2_display_cfg,
        ptoi_supported: bool,
        PixelClockBackEnd: *mut f64,
    );

    pub fn dcn5_calculate_swath_width(
        display_cfg: *const dml2_display_cfg,
        ForceSingleDPP: bool,
        NumberOfActiveSurfaces: ::core::ffi::c_uint,
        ODMMode: *mut dml2_odm_mode,
        BytePerPixY: *mut ::core::ffi::c_uint,
        BytePerPixC: *mut ::core::ffi::c_uint,
        Read256BytesBlockHeightY: *mut ::core::ffi::c_uint,
        Read256BytesBlockHeightC: *mut ::core::ffi::c_uint,
        Read256BytesBlockWidthY: *mut ::core::ffi::c_uint,
        Read256BytesBlockWidthC: *mut ::core::ffi::c_uint,
        surf_linear128_l: *mut bool,
        surf_linear128_c: *mut bool,
        DPPPerSurface: *mut ::core::ffi::c_uint,
        req_per_swath_ub_l: *mut ::core::ffi::c_uint,
        req_per_swath_ub_c: *mut ::core::ffi::c_uint,
        SwathWidthSingleDPPY: *mut ::core::ffi::c_uint,
        SwathWidthSingleDPPC: *mut ::core::ffi::c_uint,
        SwathWidthY: *mut ::core::ffi::c_uint,
        SwathWidthC: *mut ::core::ffi::c_uint,
        MaximumSwathHeightY: *mut ::core::ffi::c_uint,
        MaximumSwathHeightC: *mut ::core::ffi::c_uint,
        swath_width_luma_ub: *mut ::core::ffi::c_uint,
        swath_width_chroma_ub: *mut ::core::ffi::c_uint,
        swath_width_luma_ub_single_dpp: *mut ::core::ffi::c_uint,
        swath_width_chroma_ub_single_dpp: *mut ::core::ffi::c_uint,
    );

    pub fn dcn5_calculate_swath_and_det_configuration(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_CalculateSwathAndDETConfiguration_params);
    pub fn dcn5_calculate_vm_row_and_swath(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_CalculateVMRowAndSwath_params);
    pub fn dcn5_calculate_bytes_to_fetch_required_to_hide_latency(p: *mut dml2_core_calcs_calculate_bytes_to_fetch_required_to_hide_latency_params);

    pub fn dcn5_calculate_excess_vactive_bandwidth_required(
        display_cfg: *const dml2_display_cfg,
        num_active_planes: ::core::ffi::c_uint,
        bytes_required_l: *mut ::core::ffi::c_uint,
        bytes_required_c: *mut ::core::ffi::c_uint,
        excess_vactive_fill_bw_l: *mut f64,
        excess_vactive_fill_bw_c: *mut f64,
    );

    pub fn dcn5_calculate_cursor_req_attributes(cursor_width: ::core::ffi::c_uint, cursor_bpp: ::core::ffi::c_uint, cursor_lines_per_chunk: *mut ::core::ffi::c_uint, cursor_bytes_per_line: *mut ::core::ffi::c_uint, cursor_bytes_per_chunk: *mut ::core::ffi::c_uint, cursor_bytes: *mut ::core::ffi::c_uint);
    pub fn dcn5_calculate_cursor_urgent_burst_factor(CursorBufferSize: ::core::ffi::c_uint, CursorWidth: ::core::ffi::c_uint, cursor_bytes_per_chunk: ::core::ffi::c_uint, cursor_lines_per_chunk: ::core::ffi::c_uint, LineTime: f64, UrgentLatency: f64, UrgentBurstFactorCursor: *mut f64, NotEnoughUrgentLatencyHiding: *mut bool);

    pub fn dcn5_calculate_urgent_burst_factor(
        plane_cfg: *const dml2_plane_parameters, swath_width_luma_ub: ::core::ffi::c_uint, swath_width_chroma_ub: ::core::ffi::c_uint,
        SwathHeightY: ::core::ffi::c_uint, SwathHeightC: ::core::ffi::c_uint, LineTime: f64, UrgentLatency: f64, VRatio: f64, VRatioC: f64,
        BytePerPixelInDETY: f64, BytePerPixelInDETC: f64, DETBufferSizeY: ::core::ffi::c_uint, DETBufferSizeC: ::core::ffi::c_uint,
        UrgentBurstFactorLuma: *mut f64, UrgentBurstFactorChroma: *mut f64, NotEnoughUrgentLatencyHiding: *mut bool,
    );

    pub fn dcn5_calculate_dcfclk_deep_sleep(display_cfg: *const dml2_display_cfg, NumberOfActiveSurfaces: ::core::ffi::c_uint, BytePerPixelY: *mut ::core::ffi::c_uint, BytePerPixelC: *mut ::core::ffi::c_uint, SwathWidthY: *mut ::core::ffi::c_uint, SwathWidthC: *mut ::core::ffi::c_uint, DPPPerSurface: *mut ::core::ffi::c_uint, PSCL_THROUGHPUT: *mut f64, PSCL_THROUGHPUT_CHROMA: *mut f64, Dppclk: *mut f64, ReadBandwidthLuma: *mut f64, ReadBandwidthChroma: *mut f64, ReturnBusWidth: ::core::ffi::c_uint, DCFClkDeepSleep: *mut f64);
    pub fn dcn5_calculate_max_vstartup(ptoi_supported: bool, vblank_nom_default_us: ::core::ffi::c_uint, timing: *const dml2_timing_cfg, write_back_delay_us: f64) -> ::core::ffi::c_uint;
    pub fn dcn5_calculate_mcache_setting(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_calculate_mcache_setting_params);
    pub fn dcn5_calculate_avg_bandwidth_required(avg_bandwidth_required: *mut f64, num_active_planes: ::core::ffi::c_uint, ReadBandwidthLuma: *mut f64, ReadBandwidthChroma: *mut f64, cursor_bw: *mut f64, dcc_dram_bw_nom_overhead_factor_p0: *mut f64, dcc_dram_bw_nom_overhead_factor_p1: *mut f64);
    pub fn dcn5_calculate_hostvm_inefficiency_factor(HostVMInefficiencyFactor: *mut f64, HostVMInefficiencyFactorPrefetch: *mut f64, gpuvm_enable: bool, hostvm_enable: bool, remote_iommu_outstanding_translations: ::core::ffi::c_uint, max_outstanding_reqs: ::core::ffi::c_uint, urg_bandwidth_avail_active_pixel_and_vm: f64, urg_bandwidth_avail_active_vm_only: f64);
    pub fn dcn5_calculate_tdlut_setting(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_calculate_tdlut_setting_params);
    pub fn dcn5_calculate_extra_latency(display_cfg: *const dml2_display_cfg, ROBBufferSizeInKByte: ::core::ffi::c_uint, RoundTripPingLatencyCycles: ::core::ffi::c_uint, ReorderingBytes: ::core::ffi::c_uint, DCFCLK: f64, FabricClock: f64, PixelChunkSizeInKByte: ::core::ffi::c_uint, ReturnBW: f64, NumberOfActiveSurfaces: ::core::ffi::c_uint, NumberOfDPP: *mut ::core::ffi::c_uint, dpte_group_bytes: *mut ::core::ffi::c_uint, tdlut_bytes_per_group: *mut ::core::ffi::c_uint, HostVMInefficiencyFactor: f64, HostVMInefficiencyFactorPrefetch: f64, qos_type: dml2_qos_param_type, max_outstanding_when_urgent_expected: bool, max_outstanding_requests: ::core::ffi::c_uint, request_size_bytes_luma: *mut ::core::ffi::c_uint, request_size_bytes_chroma: *mut ::core::ffi::c_uint, MetaChunkSize: ::core::ffi::c_uint, dchub_arb_to_ret_delay: ::core::ffi::c_uint, Ttrip: f64, hostvm_mode: ::core::ffi::c_uint, ExtraLatency: *mut f64, ExtraLatency_sr: *mut f64, ExtraLatencyPrefetch: *mut f64);
    pub fn dcn5_calculate_t_wait(reserved_vblank_time_ns: isize, UrgentLatency: f64, Ttrip: f64, temp_read_or_ppt_blackout_us: f64, drr_enabled: bool) -> f64;
    pub fn dcn5_calculate_prefetch_schedule(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_CalculatePrefetchSchedule_params) -> bool;
    pub fn dcn5_calculate_peak_bandwidth_required(s: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_calculate_peak_bandwidth_required_params);
    pub fn dcn5_check_urgent_bandwidth_support(frac_urg_bandwidth_nom: *mut f64, bandwidth_support_ok: *mut bool, non_urg_bandwidth_required: f64, urg_bandwidth_required: f64, urg_bandwidth_available: f64);
    pub fn dcn5_get_bandwidth_available_for_immediate_flip(urg_bandwidth_required: f64, urg_bandwidth_available: f64) -> f64;
    pub fn dcn5_get_pipe_flip_bytes(hostvm_inefficiency_factor: f64, vm_bytes: ::core::ffi::c_uint, dpte_row_bytes: ::core::ffi::c_uint, meta_row_bytes: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn dcn5_check_immediate_flip_bandwidth_support(frac_urg_bandwidth_flip: *mut f64, flip_bandwidth_support_ok: *mut bool, urg_bandwidth_required_flip: f64, non_urg_bandwidth_required_flip: f64, urg_bandwidth_available: f64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
