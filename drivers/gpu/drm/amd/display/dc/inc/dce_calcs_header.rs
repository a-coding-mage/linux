/*
 * Copyright 2015-2017 Advanced Micro Devices, Inc.
 *
 * Bandwidth and Watermark calculations interface.
 * (Refer to "DCEx_mode_support.xlsm" from Perforce.)
 *
 * This is a direct Rust declaration translation of dce_calcs.h.  The included
 * bw_fixed and hardware types are supplied by the surrounding implementation.
 */

use core::ffi::c_int;

pub const maximum_number_of_surfaces: usize = 12;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bw_calcs_version {
    BW_CALCS_VERSION_INVALID,
    BW_CALCS_VERSION_CARRIZO,
    BW_CALCS_VERSION_POLARIS10,
    BW_CALCS_VERSION_POLARIS11,
    BW_CALCS_VERSION_POLARIS12,
    BW_CALCS_VERSION_VEGAM,
    BW_CALCS_VERSION_STONEY,
    BW_CALCS_VERSION_VEGA10,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bw_defines {
    bw_def_no = 0, bw_def_none = 0, bw_def_yes = 1, bw_def_ok = 1,
    bw_def_high = 2, bw_def_mid = 1, bw_def_low = 0,
    bw_defs_start = 255,
    bw_def_underlay422, bw_def_underlay420_luma, bw_def_underlay420_chroma,
    bw_def_underlay444, bw_def_graphics, bw_def_display_write_back420_luma,
    bw_def_display_write_back420_chroma, bw_def_portrait, bw_def_hsr_mtn_4,
    bw_def_hsr_mtn_h_taps, bw_def_ceiling__h_taps_div_4___meq_hsr,
    bw_def_invalid_linear_or_stereo_mode, bw_def_invalid_rotation_or_bpp_or_stereo,
    bw_def_vsr_mtn_v_taps, bw_def_vsr_mtn_4, bw_def_auto, bw_def_manual,
    bw_def_exceeded_allowed_maximum_sclk, bw_def_exceeded_allowed_page_close_open,
    bw_def_exceeded_allowed_outstanding_pte_req_queue_size,
    bw_def_exceeded_allowed_maximum_bw, bw_def_landscape, bw_def_any_lines,
    bw_def_underlay_only, bw_def_blended, bw_def_blend, bw_def_mono,
    bw_def_side_by_side, bw_def_top_bottom, bw_def_420, bw_def_422, bw_def_444,
    bw_def_linear, bw_def_tiled, bw_def_array_linear_general,
    bw_def_array_linear_aligned, bw_def_rotated_micro_tiling,
    bw_def_display_micro_tiling, bw_def_gddr5, bw_def_hbm,
    bw_def_high_no_nbp_state_change, bw_def_0_72, bw_def_0_8, bw_def_0_9,
    bw_def_notok = -1, bw_def_na = -1,
}

#[repr(C)]
pub struct bw_calcs_dceip {
    pub version: bw_calcs_version,
    pub percent_of_ideal_port_bw_received_after_urgent_latency: u32,
    pub max_average_percent_of_ideal_port_bw_display_can_use_in_normal_system_operation: u32,
    pub max_average_percent_of_ideal_drambw_display_can_use_in_normal_system_operation: u32,
    pub large_cursor: bool, pub cursor_max_outstanding_group_num: u32,
    pub dmif_pipe_en_fbc_chunk_tracker: bool, pub dmif_request_buffer_size: bw_fixed,
    pub lines_interleaved_into_lb: u32, pub low_power_tiling_mode: u32, pub chunk_width: u32,
    pub number_of_graphics_pipes: u32, pub number_of_underlay_pipes: u32,
    pub display_write_back_supported: bool, pub argb_compression_support: bool,
    pub underlay_vscaler_efficiency6_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency8_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency10_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency12_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency6_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency8_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency10_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency12_bit_per_component: bw_fixed,
    pub alpha_vscaler_efficiency: bw_fixed, pub max_dmif_buffer_allocated: u32,
    pub graphics_dmif_size: u32, pub underlay_luma_dmif_size: u32,
    pub underlay_chroma_dmif_size: u32, pub pre_downscaler_enabled: bool,
    pub underlay_downscale_prefetch_enabled: bool, pub lb_write_pixels_per_dispclk: bw_fixed,
    pub lb_size_per_component444: bw_fixed,
    pub graphics_lb_nodownscaling_multi_line_prefetching: bool,
    pub stutter_and_dram_clock_state_change_gated_before_cursor: bw_fixed,
    pub underlay420_luma_lb_size_per_component: bw_fixed,
    pub underlay420_chroma_lb_size_per_component: bw_fixed,
    pub underlay422_lb_size_per_component: bw_fixed, pub cursor_chunk_width: bw_fixed,
    pub cursor_dcp_buffer_lines: bw_fixed, pub underlay_maximum_width_efficient_for_tiling: bw_fixed,
    pub underlay_maximum_height_efficient_for_tiling: bw_fixed,
    pub peak_pte_request_to_eviction_ratio_limiting_multiple_displays_or_single_rotated_display: bw_fixed,
    pub peak_pte_request_to_eviction_ratio_limiting_single_display_no_rotation: bw_fixed,
    pub minimum_outstanding_pte_request_limit: bw_fixed,
    pub maximum_total_outstanding_pte_requests_allowed_by_saw: bw_fixed,
    pub limit_excessive_outstanding_dmif_requests: bool,
    pub linear_mode_line_request_alternation_slice: bw_fixed,
    pub scatter_gather_lines_of_pte_prefetching_in_linear_mode: u32,
    pub display_write_back420_luma_mcifwr_buffer_size: u32,
    pub display_write_back420_chroma_mcifwr_buffer_size: u32,
    pub request_efficiency: bw_fixed, pub dispclk_per_request: bw_fixed,
    pub dispclk_ramping_factor: bw_fixed, pub display_pipe_throughput_factor: bw_fixed,
    pub scatter_gather_pte_request_rows_in_tiling_mode: u32,
    pub mcifwr_all_surfaces_burst_time: bw_fixed,
}

#[repr(C)]
pub struct bw_calcs_vbios {
    pub memory_type: bw_defines, pub dram_channel_width_in_bits: u32,
    pub number_of_dram_channels: u32, pub number_of_dram_banks: u32,
    pub low_yclk: bw_fixed, pub mid_yclk: bw_fixed, pub high_yclk: bw_fixed,
    pub low_sclk: bw_fixed, pub mid1_sclk: bw_fixed, pub mid2_sclk: bw_fixed,
    pub mid3_sclk: bw_fixed, pub mid4_sclk: bw_fixed, pub mid5_sclk: bw_fixed,
    pub mid6_sclk: bw_fixed, pub high_sclk: bw_fixed,
    pub low_voltage_max_dispclk: bw_fixed, pub mid_voltage_max_dispclk: bw_fixed,
    pub high_voltage_max_dispclk: bw_fixed, pub low_voltage_max_phyclk: bw_fixed,
    pub mid_voltage_max_phyclk: bw_fixed, pub high_voltage_max_phyclk: bw_fixed,
    pub data_return_bus_width: bw_fixed, pub trc: bw_fixed, pub dmifmc_urgent_latency: bw_fixed,
    pub stutter_self_refresh_exit_latency: bw_fixed, pub stutter_self_refresh_entry_latency: bw_fixed,
    pub nbp_state_change_latency: bw_fixed, pub mcifwrmc_urgent_latency: bw_fixed,
    pub scatter_gather_enable: bool, pub down_spread_percentage: bw_fixed,
    pub cursor_width: u32, pub average_compression_rate: u32,
    pub number_of_request_slots_gmc_reserves_for_dmif_per_channel: u32,
    pub blackout_duration: bw_fixed, pub maximum_blackout_recovery_time: bw_fixed,
}

/* Temporary calculation data.  Array dimensions and field order mirror C. */
#[repr(C)]
pub struct bw_calcs_data {
    pub display_synchronization_enabled: bool, pub number_of_displays: u32,
    pub underlay_surface_type: bw_defines, pub panning_and_bezel_adjustment: bw_defines,
    pub graphics_tiling_mode: bw_defines, pub graphics_lb_bpc: u32, pub underlay_lb_bpc: u32,
    pub underlay_tiling_mode: bw_defines, pub d0_underlay_mode: bw_defines,
    pub d1_display_write_back_dwb_enable: bool, pub d1_underlay_mode: bw_defines,
    pub increase_voltage_to_support_mclk_switch: bool, pub cpup_state_change_enable: bool,
    pub cpuc_state_change_enable: bool, pub nbp_state_change_enable: bool,
    pub stutter_mode_enable: bool, pub y_clk_level: u32, pub sclk_level: u32,
    pub number_of_underlay_surfaces: u32, pub number_of_dram_wrchannels: u32,
    pub chunk_request_delay: u32, pub number_of_dram_channels: u32,
    pub underlay_micro_tile_mode: bw_defines, pub graphics_micro_tile_mode: bw_defines,
    pub max_phyclk: bw_fixed, pub dram_efficiency: bw_fixed,
    pub source_height_pixels: bw_fixed, pub dispclk: bw_fixed, pub dram_bandwidth: bw_fixed,
    pub required_sclk: bw_fixed, pub total_average_bandwidth: bw_fixed,
    pub total_average_bandwidth_no_compression: bw_fixed,
    pub fbc_en: [bool; maximum_number_of_surfaces], pub lpt_en: [bool; maximum_number_of_surfaces],
    pub enable: [bool; maximum_number_of_surfaces],
    pub bytes_per_pixel: [u32; maximum_number_of_surfaces],
    pub stereo_mode: [bw_defines; maximum_number_of_surfaces],
    pub pixel_rate: [bw_fixed; maximum_number_of_surfaces],
    pub src_width: [bw_fixed; maximum_number_of_surfaces],
    pub src_height: [bw_fixed; maximum_number_of_surfaces],
    pub scale_ratio: [bw_fixed; maximum_number_of_surfaces],
    pub h_taps: [bw_fixed; maximum_number_of_surfaces],
    pub v_taps: [bw_fixed; maximum_number_of_surfaces],
    pub display_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub request_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub data_buffer_size: [bw_fixed; maximum_number_of_surfaces],
    pub urgent_watermark: [bw_fixed; maximum_number_of_surfaces],
    pub average_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub dmif_burst_time: [[bw_fixed; 8]; 3], pub mcifwr_burst_time: [[bw_fixed; 8]; 3],
    pub line_source_transfer_time: [[[bw_fixed; 8]; 3]; maximum_number_of_surfaces],
    pub dmif_required_sclk_for_urgent_latency: [bw_fixed; 6],
}

extern "C" {
    pub fn bw_calcs_init(bw_dceip: *mut bw_calcs_dceip, bw_vbios: *mut bw_calcs_vbios,
                         asic_id: hw_asic_id);
    pub fn bw_calcs(ctx: *mut dc_context, dceip: *const bw_calcs_dceip,
                    vbios: *const bw_calcs_vbios, pipe: *const pipe_ctx, pipe_count: c_int,
                    calcs_output: *mut dce_bw_output) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
