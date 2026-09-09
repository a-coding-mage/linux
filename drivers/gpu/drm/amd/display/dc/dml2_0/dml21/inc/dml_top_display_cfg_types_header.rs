// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
// Translated from dml_top_display_cfg_types.h. External dependencies are supplied elsewhere.

use core::ffi::{c_char, c_uint, c_ulong};

pub const DML2_MAX_PLANES: usize = 8;
pub const DML2_MAX_DCN_PIPES: usize = 8;
pub const DML2_MAX_MCACHES: usize = 8;
pub const DML2_MAX_WRITEBACK: usize = 3;

macro_rules! c_enum { ($name:ident { $($v:ident $(= $n:expr)?),* $(,)? }) => {
    #[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum $name { $($v $(= $n)?),* }
}; }

c_enum!(dml2_swizzle_mode { dml2_sw_linear, dml2_sw_256b_2d, dml2_sw_4kb_2d, dml2_sw_64kb_2d, dml2_sw_256kb_2d, dml2_gfx11_sw_linear, dml2_gfx11_sw_64kb_d, dml2_gfx11_sw_64kb_d_t, dml2_gfx11_sw_64kb_d_x, dml2_gfx11_sw_64kb_r_x, dml2_gfx11_sw_256kb_d_x, dml2_gfx11_sw_256kb_r_x });
c_enum!(dml2_source_format_class { dml2_444_8 = 0, dml2_444_16 = 1, dml2_444_32 = 2, dml2_444_64 = 3, dml2_420_8 = 4, dml2_420_10 = 5, dml2_420_12 = 6, dml2_rgbe_alpha = 9, dml2_rgbe = 10, dml2_mono_8 = 11, dml2_mono_16 = 12, dml2_422_planar_8 = 13, dml2_422_planar_10 = 14, dml2_422_planar_12 = 15, dml2_422_packed_8 = 16, dml2_422_packed_10 = 17, dml2_422_packed_12 = 18 });
c_enum!(dml2_sample_positioning { dml2_interstitial = 0, dml2_cosited = 1 });
c_enum!(dml2_rotation_angle { dml2_rotation_0 = 0, dml2_rotation_90 = 1, dml2_rotation_180 = 2, dml2_rotation_270 = 3 });
c_enum!(dml2_output_format_class { dml2_444 = 0, dml2_s422 = 1, dml2_n422 = 2, dml2_420 = 3 });
c_enum!(dml2_output_encoder_class { dml2_dp = 0, dml2_edp = 1, dml2_dp2p0 = 2, dml2_hdmi = 3, dml2_hdmifrl = 4, dml2_none = 5 });
c_enum!(dml2_output_link_dp_rate { dml2_dp_rate_na = 0, dml2_dp_rate_hbr = 1, dml2_dp_rate_hbr2 = 2, dml2_dp_rate_hbr3 = 3, dml2_dp_rate_uhbr10 = 4, dml2_dp_rate_uhbr13p5 = 5, dml2_dp_rate_uhbr20 = 6 });
c_enum!(dml2_pstate_type { dml2_pstate_type_uclk = 0, dml2_pstate_type_fclk = 1, dml2_pstate_type_ppt = 2, dml2_pstate_type_temp_read = 3, dml2_pstate_type_dummy_pstate = 4, dml2_pstate_type_count = 5 });
c_enum!(dml2_uclk_pstate_change_strategy { dml2_uclk_pstate_change_strategy_auto = 0, dml2_uclk_pstate_change_strategy_force_vactive = 1, dml2_uclk_pstate_change_strategy_force_vblank = 2, dml2_uclk_pstate_change_strategy_force_drr = 3, dml2_uclk_pstate_change_strategy_force_mall_svp = 4, dml2_uclk_pstate_change_strategy_force_mall_full_frame = 5, dml2_uclk_pstate_change_strategy_force_alternate = 6 });
c_enum!(dml2_svp_mode_override { dml2_svp_mode_override_auto = 0, dml2_svp_mode_override_main_pipe = 1, dml2_svp_mode_override_phantom_pipe = 2, dml2_svp_mode_override_phantom_pipe_no_data_return = 3, dml2_svp_mode_override_imall = 4 });
c_enum!(dml2_refresh_from_mall_mode_override { dml2_refresh_from_mall_mode_override_auto = 0, dml2_refresh_from_mall_mode_override_force_disable = 1, dml2_refresh_from_mall_mode_override_force_enable = 2 });
c_enum!(dml2_odm_mode { dml2_odm_mode_auto = 0, dml2_odm_mode_bypass, dml2_odm_mode_combine_2to1, dml2_odm_mode_combine_3to1, dml2_odm_mode_combine_4to1, dml2_odm_mode_split_1to2, dml2_odm_mode_mso_1to2, dml2_odm_mode_mso_1to4 });
c_enum!(dml2_scaling_transform { dml2_scaling_transform_explicit = 0, dml2_scaling_transform_fullscreen, dml2_scaling_transform_aspect_ratio, dml2_scaling_transform_centered });
c_enum!(dml2_dsc_enable_option { dml2_dsc_disable = 0, dml2_dsc_enable = 1, dml2_dsc_enable_if_necessary = 2 });
c_enum!(dml2_tdlut_addressing_mode { dml2_tdlut_sw_linear = 0, dml2_tdlut_simple_linear = 1 });
c_enum!(dml2_tdlut_width_mode { dml2_tdlut_width_17_cube = 0, dml2_tdlut_width_33_cube = 1 });
c_enum!(dml2_twait_budgeting_setting { dml2_twait_budgeting_setting_ignore = 0, dml2_twait_budgeting_setting_if_needed, dml2_twait_budgeting_setting_try });

#[repr(C)] pub struct dml2_get_cursor_dlg_reg { pub cursor_x_position: c_uint, pub cursor_hotspot_x: c_uint, pub cursor_primary_offset: c_uint, pub cursor_secondary_offset: c_uint, pub cursor_stereo_en: bool, pub cursor_2x_magnify: bool, pub hratio: f64, pub pixel_rate_mhz: f64, pub dlg_refclk_mhz: f64 }
#[repr(C)] pub struct dml2_surface_cfg { pub tiling: dml2_swizzle_mode, pub plane0: dml2_surface_plane, pub plane1: dml2_surface_plane, pub dcc: dml2_dcc_cfg }
#[repr(C)] pub struct dml2_surface_plane { pub pitch: c_ulong, pub width: c_ulong, pub height: c_ulong }
#[repr(C)] pub struct dml2_dcc_cfg { pub enable: bool, pub plane0: dml2_dcc_plane, pub plane1: dml2_dcc_plane, pub informative: dml2_dcc_informative }
#[repr(C)] pub struct dml2_dcc_plane { pub pitch: c_ulong }
#[repr(C)] pub struct dml2_dcc_informative { pub dcc_rate_plane0: f64, pub dcc_rate_plane1: f64, pub fraction_of_zero_size_request_plane0: f64, pub fraction_of_zero_size_request_plane1: f64 }

#[repr(C)] pub struct dml2_composition_cfg { pub rotation_angle: dml2_rotation_angle, pub mirrored: bool, pub scaling_transform: dml2_scaling_transform, pub rect_out_height_spans_vactive: bool, pub viewport: dml2_viewport, pub scaler_info: dml2_scaler_info }
#[repr(C)] pub struct dml2_viewport { pub stationary: bool, pub plane0: dml2_viewport_plane, pub plane1: dml2_viewport_plane }
#[repr(C)] pub struct dml2_viewport_plane { pub width: c_ulong, pub height: c_ulong, pub x_start: c_ulong, pub y_start: c_ulong }
#[repr(C)] pub struct dml2_scaler_info { pub enabled: bool, pub easf_enabled: bool, pub isharp_enabled: bool, pub upsp_enabled: bool, pub upsp_sample_positioning: dml2_sample_positioning, pub upsp_vtaps: c_uint, pub plane0: dml2_scaler_plane, pub plane1: dml2_scaler_plane, pub rect_out_width: c_ulong }
#[repr(C)] pub struct dml2_scaler_plane { pub h_ratio: f64, pub v_ratio: f64, pub h_taps: c_uint, pub v_taps: c_uint }

#[repr(C)] pub struct dml2_timing_cfg { pub h_total: c_ulong, pub v_total: c_ulong, pub h_blank_end: c_ulong, pub v_blank_end: c_ulong, pub h_front_porch: c_ulong, pub v_front_porch: c_ulong, pub h_sync_width: c_ulong, pub pixel_clock_khz: c_ulong, pub h_active: c_ulong, pub v_active: c_ulong, pub bpc: c_uint, pub dsc: dml2_dsc_cfg, pub interlaced: bool, pub drr_config: dml2_drr_config, pub vblank_nom: c_ulong }
#[repr(C)] pub struct dml2_dsc_cfg { pub enable: dml2_dsc_enable_option, pub dsc_compressed_bpp_x16: c_uint, pub overrides: dml2_dsc_overrides }
#[repr(C)] pub struct dml2_dsc_overrides { pub num_slices: c_uint }
#[repr(C)] pub struct dml2_drr_config { pub enabled: bool, pub min_refresh_uhz: c_ulong, pub max_instant_vtotal_delta: c_uint, pub disallowed: bool, pub drr_active_variable: bool, pub drr_active_fixed: bool }
#[repr(C)] pub struct dml2_link_output_cfg { pub output_format: dml2_output_format_class, pub output_encoder: dml2_output_encoder_class, pub output_dp_lane_count: c_uint, pub output_dp_link_rate: dml2_output_link_dp_rate, pub audio_sample_rate: c_ulong, pub audio_sample_layout: c_ulong, pub output_disabled: bool, pub validate_output: bool }
#[repr(C)] pub struct dml2_writeback_info { pub pixel_format: dml2_source_format_class, pub input_width: c_ulong, pub input_height: c_ulong, pub output_width: c_ulong, pub output_height: c_ulong, pub v_taps: c_ulong, pub h_taps: c_ulong, pub v_taps_chroma: c_ulong, pub h_taps_chroma: c_ulong, pub h_ratio: f64, pub v_ratio: f64 }
#[repr(C)] pub struct dml2_writeback_cfg { pub active_writebacks_per_stream: c_uint, pub writeback_stream: [dml2_writeback_info; DML2_MAX_WRITEBACK] }

#[repr(C)] pub struct dml2_plane_parameters {
    pub stream_index: c_uint, pub pixel_format: dml2_source_format_class,
    pub surface: dml2_surface_cfg, pub composition: dml2_composition_cfg,
    pub dynamic_meta_data: dml2_dynamic_meta_data, pub cursor: dml2_cursor_cfg,
    pub tdlut: dml2_tdlut_cfg, pub immediate_flip: bool, pub overrides: dml2_plane_overrides,
}
#[repr(C)] pub struct dml2_dynamic_meta_data { pub enable: bool, pub lines_before_active_required: c_ulong, pub transmitted_bytes: c_ulong }
#[repr(C)] pub struct dml2_cursor_cfg { pub num_cursors: c_uint, pub cursor_width: c_ulong, pub cursor_bpp: c_ulong }
#[repr(C)] pub struct dml2_tdlut_cfg { pub setup_for_tdlut: bool, pub tdlut_addressing_mode: dml2_tdlut_addressing_mode, pub tdlut_width_mode: dml2_tdlut_width_mode, pub tdlut_mpc_width_flag: bool }
#[repr(C)] pub struct dml2_plane_overrides { pub uclk_pstate_change_strategy: dml2_uclk_pstate_change_strategy, pub refresh_from_mall: dml2_refresh_from_mall_mode_override, pub det_size_override_kb: c_uint, pub mpcc_combine_factor: c_uint, pub reserved_vblank_time_ns: isize, pub max_vactive_det_fill_delay_us: [c_uint; 5], pub gpuvm_min_page_size_kbytes: c_uint, pub hostvm_min_page_size_kbytes: c_uint, pub legacy_svp_config: dml2_svp_mode_override, pub use_max_lsw: bool, pub hw: dml2_plane_hw_overrides }
#[repr(C)] pub struct dml2_plane_hw_overrides { pub force_one_row_for_frame: bool, pub force_pte_buffer_mode: dml2_bool_override, pub dppclk_mhz: f64 }
#[repr(C)] pub struct dml2_bool_override { pub enable: bool, pub value: bool }

#[repr(C)] pub struct dml2_stream_parameters { pub timing: dml2_timing_cfg, pub output: dml2_link_output_cfg, pub writeback: dml2_writeback_cfg, pub overrides: dml2_stream_overrides }
#[repr(C)] pub struct dml2_stream_overrides { pub odm_mode: dml2_odm_mode, pub disable_dynamic_odm: bool, pub disable_subvp: bool, pub minimum_vblank_idle_requirement_us: i32, pub hw: dml2_stream_hw_overrides }
#[repr(C)] pub struct dml2_stream_hw_overrides { pub twait_budgeting: dml2_twait_budgeting }
#[repr(C)] pub struct dml2_twait_budgeting { pub uclk_pstate: dml2_twait_budgeting_setting, pub fclk_pstate: dml2_twait_budgeting_setting, pub stutter_enter_exit: dml2_twait_budgeting_setting }

#[repr(C)] pub struct dml2_display_cfg { pub gpuvm_enable: bool, pub ffbm_enable: bool, pub hostvm_enable: bool, pub minimize_det_reallocation: bool, pub gpuvm_max_page_table_levels: c_uint, pub hostvm_max_non_cached_page_table_levels: c_uint, pub plane_descriptors: [dml2_plane_parameters; DML2_MAX_PLANES], pub stream_descriptors: [dml2_stream_parameters; DML2_MAX_PLANES], pub num_planes: c_uint, pub num_streams: c_uint, pub overrides: dml2_display_overrides }
#[repr(C)] pub struct dml2_display_overrides { pub hw: dml2_display_hw_overrides, pub power_management: dml2_power_management, pub enhanced_prefetch_schedule_acceleration: bool, pub dcc_programming_assumes_scan_direction_unknown: bool, pub synchronize_timings: bool, pub synchronize_ddr_displays_for_uclk_pstate_change: bool, pub max_outstanding_when_urgent_expected_disable: bool, pub enable_subvp_implicit_pmo: bool, pub all_streams_blanked: bool }
#[repr(C)] pub struct dml2_display_hw_overrides { pub force_unbounded_requesting: dml2_bool_override, pub force_nom_det_size_kbytes: dml2_bool_override, pub force_alt_chan_copy_time: dml2_copy_time_override, pub force_alt_chan_fw_delay: dml2_fw_delay_override, pub mode_support_check_disable: bool, pub mcache_admissibility_check_disable: bool, pub surface_viewport_size_check_disable: bool, pub dlg_ref_clk_mhz: f64, pub dispclk_mhz: f64, pub dcfclk_mhz: f64, pub optimize_tdlut_scheduling: bool }
#[repr(C)] pub struct dml2_copy_time_override { pub enable: bool, pub copy_time_us: c_uint }
#[repr(C)] pub struct dml2_fw_delay_override { pub enable: bool, pub fw_delay_us: c_uint }
#[repr(C)] pub struct dml2_power_management { pub uclk_pstate_change_disable: bool, pub fclk_pstate_change_disable: bool, pub g6_temp_read_pstate_disable: bool, pub g7_ppt_pstate_disable: bool }

#[repr(C)] pub struct dml2_pipe_configuration_descriptor { pub plane0: dml2_pipe_plane, pub plane1: dml2_pipe_plane, pub plane1_enabled: bool, pub imall_enabled: bool }
#[repr(C)] pub struct dml2_pipe_plane { pub viewport_x_start: c_uint, pub viewport_width: c_uint }
#[repr(C)] pub struct dml2_plane_mcache_configuration_descriptor { pub plane_descriptor: *const dml2_plane_parameters, pub mcache_allocation: *const dml2_mcache_surface_allocation, pub pipe_configurations: [dml2_pipe_configuration_descriptor; DML2_MAX_DCN_PIPES], pub num_pipes: c_char }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
