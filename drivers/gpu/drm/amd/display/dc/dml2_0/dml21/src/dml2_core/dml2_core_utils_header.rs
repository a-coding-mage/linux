// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml2_core_utils.h.
// Dependencies supplied by the surrounding translation unit:
// dml2_internal_shared_types.h, dml2_debug.h, and lib_float_math.h.

use std::os::raw::{c_char, c_double, c_int, c_uint, c_ulong};

extern "C" {
    pub fn dml2_core_utils_div_rem(
        dividend: c_double,
        divisor: c_uint,
        remainder: *mut c_uint,
    ) -> c_double;
    pub fn dml2_core_utils_internal_bw_type_str(
        bw_type: dml2_core_internal_bw_type,
    ) -> *const c_char;
    pub fn dml2_core_utils_is_420(source_format: dml2_source_format_class) -> bool;
    pub fn dml2_core_utils_is_422_planar(source_format: dml2_source_format_class) -> bool;
    pub fn dml2_core_utils_is_422_packed(source_format: dml2_source_format_class) -> bool;
    pub fn dml2_core_utils_print_mode_support_info(
        support: *const dml2_core_internal_mode_support_info,
        fail_only: bool,
    );
    pub fn dml2_core_utils_internal_soc_state_type_str(
        dml2_core_internal_soc_state_type: dml2_core_internal_soc_state_type,
    ) -> *const c_char;
    pub fn dml2_core_utils_get_stream_output_bpp(
        out_bpp: *mut c_double,
        display_cfg: *const dml2_display_cfg,
    );
    pub fn dml2_core_utils_round_to_multiple(
        num: c_uint,
        multiple: c_uint,
        up: bool,
    ) -> c_uint;
    pub fn dml2_core_util_get_num_active_pipes(
        num_planes: c_uint,
        cfg_support_info: *const core_display_cfg_support_info,
    ) -> c_uint;
    pub fn dml2_core_utils_pipe_plane_mapping(
        cfg_support_info: *const core_display_cfg_support_info,
        pipe_plane: *mut c_uint,
    );
    pub fn dml2_core_utils_is_phantom_pipe(plane_cfg: *const dml2_plane_parameters) -> bool;
    pub fn dml2_core_utils_get_tile_block_size_bytes(
        sw_mode: dml2_swizzle_mode,
        byte_per_pixel: c_uint,
    ) -> c_uint;
    pub fn dml2_core_utils_get_segment_horizontal_contiguous(
        sw_mode: dml2_swizzle_mode,
        byte_per_pixel: c_uint,
    ) -> bool;
    pub fn dml2_core_utils_is_vertical_rotation(Scan: dml2_rotation_angle) -> bool;
    pub fn dml2_core_utils_is_linear(sw_mode: dml2_swizzle_mode) -> bool;
    pub fn dml2_core_utils_get_gfx_version(sw_mode: dml2_swizzle_mode) -> c_uint;
    pub fn dml2_core_utils_get_qos_param_index(
        uclk_freq_khz: c_ulong,
        per_uclk_dpm_params: *const dml2_dcn4_uclk_dpm_dependent_qos_params,
    ) -> c_uint;
    pub fn dml2_core_utils_get_active_min_uclk_dpm_index(
        uclk_freq_khz: c_ulong,
        clk_table: *const dml2_soc_state_table,
    ) -> c_uint;
    pub fn dml2_core_utils_is_dual_plane(source_format: dml2_source_format_class) -> bool;
    pub fn dml2_core_utils_log_and_substract_if_non_zero(
        a: c_uint,
        subtrahend: c_uint,
    ) -> c_uint;
    pub fn dml2_core_utils_expand_implict_subvp(
        display_cfg: *const display_configuation_with_meta,
        svp_expanded_display_cfg: *mut dml2_display_cfg,
        scratch: *mut dml2_core_scratch,
    );
    pub fn dml2_core_utils_is_stream_encoder_required(
        stream_descriptor: *const dml2_stream_parameters,
    ) -> bool;
    pub fn dml2_core_utils_is_encoder_dsc_capable(
        stream_descriptor: *const dml2_stream_parameters,
    ) -> bool;
    pub fn dml2_core_utils_is_dp_encoder(
        stream_descriptor: *const dml2_stream_parameters,
    ) -> bool;
    pub fn dml2_core_utils_is_dio_dp_encoder(
        stream_descriptor: *const dml2_stream_parameters,
    ) -> bool;
    pub fn dml2_core_utils_is_hpo_dp_encoder(
        stream_descriptor: *const dml2_stream_parameters,
    ) -> bool;
    pub fn dml2_core_utils_is_dp_8b_10b_link_rate(rate: dml2_output_link_dp_rate) -> bool;
    pub fn dml2_core_utils_is_dp_128b_132b_link_rate(rate: dml2_output_link_dp_rate) -> bool;
    pub fn dml2_core_utils_is_odm_split(odm_mode: dml2_odm_mode) -> bool;
    pub fn dml2_core_utils_get_frame_time_us(stream: *const dml2_stream_parameters) -> c_double;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
