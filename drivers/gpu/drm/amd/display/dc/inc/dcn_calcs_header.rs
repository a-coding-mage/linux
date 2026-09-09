// Translated from dcn_calcs.h.
pub const NUMBER_OF_PLANES: usize = 6;
pub const NUMBER_OF_PLANES_MINUS_ONE: usize = 5;
pub const NUMBER_OF_STATES: usize = 4;
pub const NUMBER_OF_STATES_PLUS_ONE: usize = 5;
pub const DDR4_DRAM_WIDTH: i32 = 64;
pub const DDR4_DRAM_FACTOR_SINGLE_CHANNEL: i32 = 16;
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dcn_bw_defs {
    dcn_bw_v_min0p65,
    dcn_bw_v_mid0p72,
    dcn_bw_v_nom0p8,
    dcn_bw_v_max0p9,
    dcn_bw_v_max0p91,
    dcn_bw_no_support = 5,
    dcn_bw_yes,
    dcn_bw_hor,
    dcn_bw_vert,
    dcn_bw_override,
    dcn_bw_rgb_sub_64,
    dcn_bw_rgb_sub_32,
    dcn_bw_rgb_sub_16,
    dcn_bw_no,
    dcn_bw_sw_linear,
    dcn_bw_sw_4_kb_d,
    dcn_bw_sw_4_kb_d_x,
    dcn_bw_sw_64_kb_d,
    dcn_bw_sw_64_kb_d_t,
    dcn_bw_sw_64_kb_d_x,
    dcn_bw_sw_var_d,
    dcn_bw_sw_var_d_x,
    dcn_bw_yuv420_sub_8,
    dcn_bw_sw_4_kb_s,
    dcn_bw_sw_4_kb_s_x,
    dcn_bw_sw_64_kb_s,
    dcn_bw_sw_64_kb_s_t,
    dcn_bw_sw_64_kb_s_x,
    dcn_bw_writeback,
    dcn_bw_444,
    dcn_bw_dp,
    dcn_bw_420,
    dcn_bw_hdmi,
    dcn_bw_sw_var_s,
    dcn_bw_sw_var_s_x,
    dcn_bw_yuv420_sub_10,
    dcn_bw_supported_in_v_active,
    dcn_bw_supported_in_v_blank,
    dcn_bw_not_supported,
    dcn_bw_na,
    dcn_bw_encoder_8bpc,
    dcn_bw_encoder_10bpc,
    dcn_bw_encoder_12bpc,
    dcn_bw_encoder_16bpc,
}
#[repr(C)]
pub struct dcn_bw_internal_vars {
    pub voltage: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub max_dispclk: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub max_dppclk: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub dcfclk_per_state: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub phyclk_per_state: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub fabric_and_dram_bandwidth_per_state: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub sr_exit_time: f32;
    pub sr_enter_plus_exit_time: f32;
    pub dram_clock_change_latency: f32;
    pub urgent_latency: f32;
    pub write_back_latency: f32;
    pub percent_of_ideal_drambw_received_after_urg_latency: f32;
    pub dcfclkv_max0p9: f32;
    pub dcfclkv_nom0p8: f32;
    pub dcfclkv_mid0p72: f32;
    pub dcfclkv_min0p65: f32;
    pub max_dispclk_vmax0p9: f32;
    pub max_dppclk_vmax0p9: f32;
    pub max_dispclk_vnom0p8: f32;
    pub max_dppclk_vnom0p8: f32;
    pub max_dispclk_vmid0p72: f32;
    pub max_dppclk_vmid0p72: f32;
    pub max_dispclk_vmin0p65: f32;
    pub max_dppclk_vmin0p65: f32;
    pub socclk: f32;
    pub fabric_and_dram_bandwidth_vmax0p9: f32;
    pub fabric_and_dram_bandwidth_vnom0p8: f32;
    pub fabric_and_dram_bandwidth_vmid0p72: f32;
    pub fabric_and_dram_bandwidth_vmin0p65: f32;
    pub round_trip_ping_latency_cycles: f32;
    pub urgent_out_of_order_return_per_channel: f32;
    pub number_of_channels: f32;
    pub vmm_page_size: f32;
    pub return_bus_width: f32;
    pub rob_buffer_size_in_kbyte: f32;
    pub det_buffer_size_in_kbyte: f32;
    pub dpp_output_buffer_pixels: f32;
    pub opp_output_buffer_lines: f32;
    pub pixel_chunk_size_in_kbyte: f32;
    pub pte_chunk_size: f32;
    pub meta_chunk_size: f32;
    pub writeback_chunk_size: f32;
    pub odm_capability: dcn_bw_defs;
    pub dsc_capability: dcn_bw_defs;
    pub line_buffer_size: f32;
    pub is_line_buffer_bpp_fixed: dcn_bw_defs;
    pub line_buffer_fixed_bpp: f32;
    pub max_line_buffer_lines: f32;
    pub writeback_luma_buffer_size: f32;
    pub writeback_chroma_buffer_size: f32;
    pub max_num_dpp: f32;
    pub max_num_writeback: f32;
    pub max_dchub_topscl_throughput: f32;
    pub max_pscl_tolb_throughput: f32;
    pub max_lb_tovscl_throughput: f32;
    pub max_vscl_tohscl_throughput: f32;
    pub max_hscl_ratio: f32;
    pub max_vscl_ratio: f32;
    pub max_hscl_taps: f32;
    pub max_vscl_taps: f32;
    pub under_scan_factor: f32;
    pub phyclkv_max0p9: f32;
    pub phyclkv_nom0p8: f32;
    pub phyclkv_mid0p72: f32;
    pub phyclkv_min0p65: f32;
    pub pte_buffer_size_in_requests: f32;
    pub dispclk_ramping_margin: f32;
    pub downspreading: f32;
    pub max_inter_dcn_tile_repeaters: f32;
    pub can_vstartup_lines_exceed_vsync_plus_back_porch_lines_minus_one: dcn_bw_defs;
    pub bug_forcing_luma_and_chroma_request_to_same_size_fixed: dcn_bw_defs;
    pub mode: i32;
    pub viewport_width: [f32; NUMBER_OF_PLANES];
    pub htotal: [f32; NUMBER_OF_PLANES];
    pub vtotal: [f32; NUMBER_OF_PLANES];
    pub v_sync_plus_back_porch: [f32; NUMBER_OF_PLANES];
    pub vactive: [f32; NUMBER_OF_PLANES];
    pub pixel_clock: [f32; NUMBER_OF_PLANES];
    pub viewport_height: [f32; NUMBER_OF_PLANES];
    pub dcc_enable: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub dcc_rate: [f32; NUMBER_OF_PLANES];
    pub source_scan: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub lb_bit_per_pixel: [f32; NUMBER_OF_PLANES];
    pub source_pixel_format: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub source_surface_mode: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub output_format: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub output_deep_color: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub output: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub scaler_rec_out_width: [f32; NUMBER_OF_PLANES];
    pub scaler_recout_height: [f32; NUMBER_OF_PLANES];
    pub underscan_output: [f32; NUMBER_OF_PLANES];
    pub interlace_output: [f32; NUMBER_OF_PLANES];
    pub override_hta_ps: [f32; NUMBER_OF_PLANES];
    pub override_vta_ps: [f32; NUMBER_OF_PLANES];
    pub override_hta_pschroma: [f32; NUMBER_OF_PLANES];
    pub override_vta_pschroma: [f32; NUMBER_OF_PLANES];
    pub urgent_latency_support_us: [f32; NUMBER_OF_PLANES];
    pub h_ratio: [f32; NUMBER_OF_PLANES];
    pub v_ratio: [f32; NUMBER_OF_PLANES];
    pub htaps: [f32; NUMBER_OF_PLANES];
    pub vtaps: [f32; NUMBER_OF_PLANES];
    pub hta_pschroma: [f32; NUMBER_OF_PLANES];
    pub vta_pschroma: [f32; NUMBER_OF_PLANES];
    pub pte_enable: dcn_bw_defs;
    pub synchronized_vblank: dcn_bw_defs;
    pub ta_pscalculation: dcn_bw_defs;
    pub voltage_override_level: i32;
    pub number_of_active_planes: i32;
    pub voltage_level: i32;
    pub immediate_flip_supported: dcn_bw_defs;
    pub dcfclk: f32;
    pub max_phyclk: f32;
    pub fabric_and_dram_bandwidth: f32;
    pub dpp_per_plane_per_ratio: [f32; NUMBER_OF_PLANES][f32; 2];
    pub dispclk_dppclk_support_per_ratio: [dcn_bw_defs; 2];
    pub required_dispclk_per_ratio: [f32; 2];
    pub error_message: [dcn_bw_defs; 2];
    pub dispclk_dppclk_ratio: i32;
    pub dpp_per_plane: [f32; NUMBER_OF_PLANES];
    pub det_buffer_size_y: [f32; NUMBER_OF_PLANES];
    pub det_buffer_size_c: [f32; NUMBER_OF_PLANES];
    pub swath_height_y: [f32; NUMBER_OF_PLANES];
    pub swath_height_c: [f32; NUMBER_OF_PLANES];
    pub final_error_message: dcn_bw_defs;
    pub frequency: f32;
    pub header_line: f32;
    pub header: f32;
    pub voltage_override: dcn_bw_defs;
    pub allow_different_hratio_vratio: dcn_bw_defs;
    pub acceptable_quality_hta_ps: f32;
    pub acceptable_quality_vta_ps: f32;
    pub no_of_dpp: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub swath_width_yper_state: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub swath_height_yper_state: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub swath_height_cper_state: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub urgent_latency_support_us_per_state: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_pre_ywith_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_pre_cwith_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub required_prefetch_pixel_data_bw_with_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_pre_ywithout_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_pre_cwithout_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub required_prefetch_pixel_data_bw_without_immediate_flip: [f32; NUMBER_OF_PLANES][f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub prefetch_supported_with_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub prefetch_supported_without_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_in_prefetch_supported_with_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub v_ratio_in_prefetch_supported_without_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub required_dispclk: [f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub dispclk_dppclk_support: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub total_available_pipes_support: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub total_number_of_active_dpp: [f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub total_number_of_dcc_active_dpp: [f32; 2][f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub urgent_latency_support: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub mode_support_with_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub mode_support_without_immediate_flip: [dcn_bw_defs; 2][dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub return_bw_per_state: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub dio_support: [dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub urgent_round_trip_and_out_of_order_latency_per_state: [f32; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub rob_support: [dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub bandwidth_support: [dcn_bw_defs; NUMBER_OF_STATES_PLUS_ONE + 1];
    pub prefetch_bw: [f32; NUMBER_OF_PLANES];
    pub meta_pte_bytes_per_frame: [f32; NUMBER_OF_PLANES];
    pub meta_row_bytes: [f32; NUMBER_OF_PLANES];
    pub dpte_bytes_per_row: [f32; NUMBER_OF_PLANES];
    pub prefetch_lines_y: [f32; NUMBER_OF_PLANES];
    pub prefetch_lines_c: [f32; NUMBER_OF_PLANES];
    pub max_num_sw_y: [f32; NUMBER_OF_PLANES];
    pub max_num_sw_c: [f32; NUMBER_OF_PLANES];
    pub line_times_for_prefetch: [f32; NUMBER_OF_PLANES];
    pub lines_for_meta_pte_with_immediate_flip: [f32; NUMBER_OF_PLANES];
    pub lines_for_meta_pte_without_immediate_flip: [f32; NUMBER_OF_PLANES];
    pub lines_for_meta_and_dpte_row_with_immediate_flip: [f32; NUMBER_OF_PLANES];
    pub lines_for_meta_and_dpte_row_without_immediate_flip: [f32; NUMBER_OF_PLANES];
    pub min_dppclk_using_single_dpp: [f32; NUMBER_OF_PLANES];
    pub swath_width_ysingle_dpp: [f32; NUMBER_OF_PLANES];
    pub byte_per_pixel_in_dety: [f32; NUMBER_OF_PLANES];
    pub byte_per_pixel_in_detc: [f32; NUMBER_OF_PLANES];
    pub number_of_dpp_required_for_det_and_lb_size: [f32; NUMBER_OF_PLANES];
    pub required_phyclk: [f32; NUMBER_OF_PLANES];
    pub read256_block_height_y: [f32; NUMBER_OF_PLANES];
    pub read256_block_width_y: [f32; NUMBER_OF_PLANES];
    pub read256_block_height_c: [f32; NUMBER_OF_PLANES];
    pub read256_block_width_c: [f32; NUMBER_OF_PLANES];
    pub max_swath_height_y: [f32; NUMBER_OF_PLANES];
    pub max_swath_height_c: [f32; NUMBER_OF_PLANES];
    pub min_swath_height_y: [f32; NUMBER_OF_PLANES];
    pub min_swath_height_c: [f32; NUMBER_OF_PLANES];
    pub read_bandwidth: [f32; NUMBER_OF_PLANES];
    pub write_bandwidth: [f32; NUMBER_OF_PLANES];
    pub pscl_factor: [f32; NUMBER_OF_PLANES];
    pub pscl_factor_chroma: [f32; NUMBER_OF_PLANES];
    pub scale_ratio_support: dcn_bw_defs;
    pub source_format_pixel_and_scan_support: dcn_bw_defs;
    pub total_read_bandwidth_consumed_gbyte_per_second: f32;
    pub total_write_bandwidth_consumed_gbyte_per_second: f32;
    pub total_bandwidth_consumed_gbyte_per_second: f32;
    pub dcc_enabled_in_any_plane: dcn_bw_defs;
    pub return_bw_todcn_per_state: f32;
    pub critical_point: f32;
    pub writeback_latency_support: dcn_bw_defs;
    pub required_output_bw: f32;
    pub total_number_of_active_writeback: f32;
    pub total_available_writeback_support: dcn_bw_defs;
    pub maximum_swath_width: f32;
    pub number_of_dpp_required_for_det_size: f32;
    pub number_of_dpp_required_for_lb_size: f32;
    pub min_dispclk_using_single_dpp: f32;
    pub min_dispclk_using_dual_dpp: f32;
    pub viewport_size_support: dcn_bw_defs;
    pub swath_width_granularity_y: f32;
    pub rounded_up_max_swath_size_bytes_y: f32;
    pub swath_width_granularity_c: f32;
    pub rounded_up_max_swath_size_bytes_c: f32;
    pub lines_in_det_luma: f32;
    pub lines_in_det_chroma: f32;
    pub effective_lb_latency_hiding_source_lines_luma: f32;
    pub effective_lb_latency_hiding_source_lines_chroma: f32;
    pub effective_detlb_lines_luma: f32;
    pub effective_detlb_lines_chroma: f32;
    pub projected_dcfclk_deep_sleep: f32;
    pub meta_req_height_y: f32;
    pub meta_req_width_y: f32;
    pub meta_surface_width_y: f32;
    pub meta_surface_height_y: f32;
    pub meta_pte_bytes_per_frame_y: f32;
    pub meta_row_bytes_y: f32;
    pub macro_tile_block_size_bytes_y: f32;
    pub macro_tile_block_height_y: f32;
    pub data_pte_req_height_y: f32;
    pub data_pte_req_width_y: f32;
    pub dpte_bytes_per_row_y: f32;
    pub meta_req_height_c: f32;
    pub meta_req_width_c: f32;
    pub meta_surface_width_c: f32;
    pub meta_surface_height_c: f32;
    pub meta_pte_bytes_per_frame_c: f32;
    pub meta_row_bytes_c: f32;
    pub macro_tile_block_size_bytes_c: f32;
    pub macro_tile_block_height_c: f32;
    pub macro_tile_block_width_c: f32;
    pub data_pte_req_height_c: f32;
    pub data_pte_req_width_c: f32;
    pub dpte_bytes_per_row_c: f32;
    pub v_init_y: f32;
    pub max_partial_sw_y: f32;
    pub v_init_c: f32;
    pub max_partial_sw_c: f32;
    pub dst_x_after_scaler: f32;
    pub dst_y_after_scaler: f32;
    pub time_calc: f32;
    pub v_update_offset: [f32; 2][f32; NUMBER_OF_PLANES];
    pub total_repeater_delay: f32;
    pub v_update_width: [f32; 2][f32; NUMBER_OF_PLANES];
    pub v_ready_offset: [f32; 2][f32; NUMBER_OF_PLANES];
    pub time_setup: f32;
    pub extra_latency: f32;
    pub maximum_vstartup: f32;
    pub bw_available_for_immediate_flip: f32;
    pub total_immediate_flip_bytes: [f32; NUMBER_OF_PLANES];
    pub time_for_meta_pte_with_immediate_flip: f32;
    pub time_for_meta_pte_without_immediate_flip: f32;
    pub time_for_meta_and_dpte_row_with_immediate_flip: f32;
    pub time_for_meta_and_dpte_row_without_immediate_flip: f32;
    pub line_times_to_request_prefetch_pixel_data_with_immediate_flip: f32;
    pub line_times_to_request_prefetch_pixel_data_without_immediate_flip: f32;
    pub maximum_read_bandwidth_with_prefetch_with_immediate_flip: f32;
    pub maximum_read_bandwidth_with_prefetch_without_immediate_flip: f32;
    pub voltage_level_with_immediate_flip: f32;
    pub voltage_level_without_immediate_flip: f32;
    pub total_number_of_active_dpp_per_ratio: [f32; 2];
    pub byte_per_pix_dety: f32;
    pub byte_per_pix_detc: f32;
    pub read256_bytes_block_height_y: f32;
    pub read256_bytes_block_width_y: f32;
    pub read256_bytes_block_height_c: f32;
    pub read256_bytes_block_width_c: f32;
    pub maximum_swath_height_y: f32;
    pub maximum_swath_height_c: f32;
    pub minimum_swath_height_y: f32;
    pub minimum_swath_height_c: f32;
    pub swath_width: f32;
    pub prefetch_bandwidth: [f32; NUMBER_OF_PLANES];
    pub v_init_pre_fill_y: [f32; NUMBER_OF_PLANES];
    pub v_init_pre_fill_c: [f32; NUMBER_OF_PLANES];
    pub max_num_swath_y: [f32; NUMBER_OF_PLANES];
    pub max_num_swath_c: [f32; NUMBER_OF_PLANES];
    pub prefill_y: [f32; NUMBER_OF_PLANES];
    pub prefill_c: [f32; NUMBER_OF_PLANES];
    pub v_startup: [f32; NUMBER_OF_PLANES];
    pub allow_dram_clock_change_during_vblank: [dcn_bw_defs; NUMBER_OF_PLANES];
    pub allow_dram_self_refresh_during_vblank: [f32; NUMBER_OF_PLANES];
    pub v_ratio_prefetch_y: [f32; NUMBER_OF_PLANES];
    pub v_ratio_prefetch_c: [f32; NUMBER_OF_PLANES];
    pub destination_lines_for_prefetch: [f32; NUMBER_OF_PLANES];
    pub destination_lines_to_request_vm_inv_blank: [f32; NUMBER_OF_PLANES];
    pub destination_lines_to_request_row_in_vblank: [f32; NUMBER_OF_PLANES];
    pub min_ttuv_blank: [f32; NUMBER_OF_PLANES];
    pub byte_per_pixel_dety: [f32; NUMBER_OF_PLANES];
    pub byte_per_pixel_detc: [f32; NUMBER_OF_PLANES];
    pub swath_width_y: [f32; NUMBER_OF_PLANES];
    pub lines_in_dety: [f32; NUMBER_OF_PLANES];
    pub lines_in_dety_rounded_down_to_swath: [f32; NUMBER_OF_PLANES];
    pub lines_in_detc: [f32; NUMBER_OF_PLANES];
    pub lines_in_detc_rounded_down_to_swath: [f32; NUMBER_OF_PLANES];
    pub full_det_buffering_time_y: [f32; NUMBER_OF_PLANES];
    pub full_det_buffering_time_c: [f32; NUMBER_OF_PLANES];
    pub active_dram_clock_change_latency_margin: [f32; NUMBER_OF_PLANES];
    pub v_blank_dram_clock_change_latency_margin: [f32; NUMBER_OF_PLANES];
    pub dcfclk_deep_sleep_per_plane: [f32; NUMBER_OF_PLANES];
    pub read_bandwidth_plane_luma: [f32; NUMBER_OF_PLANES];
    pub read_bandwidth_plane_chroma: [f32; NUMBER_OF_PLANES];
    pub display_pipe_line_delivery_time_luma: [f32; NUMBER_OF_PLANES];
    pub display_pipe_line_delivery_time_chroma: [f32; NUMBER_OF_PLANES];
    pub display_pipe_line_delivery_time_luma_prefetch: [f32; NUMBER_OF_PLANES];
    pub display_pipe_line_delivery_time_chroma_prefetch: [f32; NUMBER_OF_PLANES];
    pub pixel_pte_bytes_per_row: [f32; NUMBER_OF_PLANES];
    pub meta_pte_bytes_frame: [f32; NUMBER_OF_PLANES];
    pub meta_row_byte: [f32; NUMBER_OF_PLANES];
    pub prefetch_source_lines_y: [f32; NUMBER_OF_PLANES];
    pub prefetch_source_lines_c: [f32; NUMBER_OF_PLANES];
    pub pscl_throughput: [f32; NUMBER_OF_PLANES];
    pub pscl_throughput_chroma: [f32; NUMBER_OF_PLANES];
    pub output_bpphdmi: [f32; NUMBER_OF_PLANES];
    pub output_bppdp4_lane_hbr: [f32; NUMBER_OF_PLANES];
    pub output_bppdp4_lane_hbr2: [f32; NUMBER_OF_PLANES];
    pub output_bppdp4_lane_hbr3: [f32; NUMBER_OF_PLANES];
    pub max_vstartup_lines: [f32; NUMBER_OF_PLANES];
    pub dispclk_with_ramping: f32;
    pub dispclk_without_ramping: f32;
    pub dppclk_using_single_dpp_luma: f32;
    pub dppclk_using_single_dpp: f32;
    pub dppclk_using_single_dpp_chroma: f32;
    pub odm_capable: dcn_bw_defs;
    pub dispclk: f32;
    pub dppclk: f32;
    pub return_bandwidth_to_dcn: f32;
    pub dcc_enabled_any_plane: dcn_bw_defs;
    pub return_bw: f32;
    pub critical_compression: f32;
    pub total_data_read_bandwidth: f32;
    pub total_active_dpp: f32;
    pub total_dcc_active_dpp: f32;
    pub urgent_round_trip_and_out_of_order_latency: f32;
    pub last_pixel_of_line_extra_watermark: f32;
    pub data_fabric_line_delivery_time_luma: f32;
    pub data_fabric_line_delivery_time_chroma: f32;
    pub urgent_extra_latency: f32;
    pub urgent_watermark: f32;
    pub ptemeta_urgent_watermark: f32;
    pub dram_clock_change_watermark: f32;
    pub total_active_writeback: f32;
    pub writeback_dram_clock_change_watermark: f32;
    pub min_full_det_buffering_time: f32;
    pub frame_time_for_min_full_det_buffering_time: f32;
    pub average_read_bandwidth_gbyte_per_second: f32;
    pub part_of_burst_that_fits_in_rob: f32;
    pub stutter_burst_time: f32;
    pub stutter_efficiency_not_including_vblank: f32;
    pub smallest_vblank: f32;
    pub v_blank_time: f32;
    pub stutter_efficiency: f32;
    pub dcf_clk_deep_sleep: f32;
    pub stutter_exit_watermark: f32;
    pub stutter_enter_plus_exit_watermark: f32;
    pub effective_det_plus_lb_lines_luma: f32;
    pub urgent_latency_support_us_luma: f32;
    pub effective_det_plus_lb_lines_chroma: f32;
    pub urgent_latency_support_us_chroma: f32;
    pub min_urgent_latency_support_us: f32;
    pub non_urgent_latency_tolerance: f32;
    pub block_height256_bytes_y: f32;
    pub block_height256_bytes_c: f32;
    pub meta_request_width_y: f32;
    pub meta_surf_width_y: f32;
    pub meta_surf_height_y: f32;
    pub meta_pte_bytes_frame_y: f32;
    pub meta_row_byte_y: f32;
    pub macro_tile_size_byte_y: f32;
    pub macro_tile_height_y: f32;
    pub pixel_pte_req_height_y: f32;
    pub pixel_pte_req_width_y: f32;
    pub pixel_pte_bytes_per_row_y: f32;
    pub meta_request_width_c: f32;
    pub meta_surf_width_c: f32;
    pub meta_surf_height_c: f32;
    pub meta_pte_bytes_frame_c: f32;
    pub meta_row_byte_c: f32;
    pub macro_tile_size_bytes_c: f32;
    pub macro_tile_height_c: f32;
    pub pixel_pte_req_height_c: f32;
    pub pixel_pte_req_width_c: f32;
    pub pixel_pte_bytes_per_row_c: f32;
    pub max_partial_swath_y: f32;
    pub max_partial_swath_c: f32;
    pub t_calc: f32;
    pub next_prefetch_mode: f32;
    pub v_startup_lines: f32;
    pub planes_with_room_to_increase_vstartup_prefetch_bw_less_than_active_bw: dcn_bw_defs;
    pub planes_with_room_to_increase_vstartup_vratio_prefetch_more_than4: dcn_bw_defs;
    pub planes_with_room_to_increase_vstartup_destination_line_times_for_prefetch_less_than2: dcn_bw_defs;
    pub v_ratio_prefetch_more_than4: dcn_bw_defs;
    pub destination_line_times_for_prefetch_less_than2: dcn_bw_defs;
    pub prefetch_mode: f32;
    pub dstx_after_scaler: f32;
    pub dsty_after_scaler: f32;
    pub v_update_offset_pix: [f32; NUMBER_OF_PLANES];
    pub total_repeater_delay_time: f32;
    pub v_update_width_pix: [f32; NUMBER_OF_PLANES];
    pub v_ready_offset_pix: [f32; NUMBER_OF_PLANES];
    pub t_setup: f32;
    pub t_wait: f32;
    pub bandwidth_available_for_immediate_flip: f32;
    pub tot_immediate_flip_bytes: f32;
    pub max_rd_bandwidth: f32;
    pub time_for_fetching_meta_pte: f32;
    pub time_for_fetching_row_in_vblank: f32;
    pub lines_to_request_prefetch_pixel_data: f32;
    pub required_prefetch_pix_data_bw: f32;
    pub prefetch_mode_supported: dcn_bw_defs;
    pub active_dp_ps: f32;
    pub lb_latency_hiding_source_lines_y: f32;
    pub lb_latency_hiding_source_lines_c: f32;
    pub effective_lb_latency_hiding_y: f32;
    pub effective_lb_latency_hiding_c: f32;
    pub dpp_output_buffer_lines_y: f32;
    pub dpp_output_buffer_lines_c: f32;
    pub dppopp_buffering_y: f32;
    pub max_det_buffering_time_y: f32;
    pub active_dram_clock_change_latency_margin_y: f32;
    pub dppopp_buffering_c: f32;
    pub max_det_buffering_time_c: f32;
    pub active_dram_clock_change_latency_margin_c: f32;
    pub writeback_dram_clock_change_latency_margin: f32;
    pub min_active_dram_clock_change_margin: f32;
    pub v_blank_of_min_active_dram_clock_change_margin: f32;
    pub second_min_active_dram_clock_change_margin: f32;
    pub min_vblank_dram_clock_change_margin: f32;
    pub dram_clock_change_margin: f32;
    pub dram_clock_change_support: f32;
    pub wr_bandwidth: f32;
    pub max_used_bw: f32;
}
#[repr(C)]
pub struct dcn_soc_bounding_box {
    pub sr_exit_time;: f32;
    pub sr_enter_plus_exit_time;: f32;
    pub urgent_latency;: f32;
    pub write_back_latency;: f32;
    pub percent_of_ideal_drambw_received_after_urg_latency;: f32;
    pub max_request_size;: i32;
    pub dcfclkv_max0p9;: f32;
    pub dcfclkv_nom0p8;: f32;
    pub dcfclkv_mid0p72;: f32;
    pub dcfclkv_min0p65;: f32;
    pub max_dispclk_vmax0p9;: f32;
    pub max_dispclk_vmid0p72;: f32;
    pub max_dispclk_vnom0p8;: f32;
    pub max_dispclk_vmin0p65;: f32;
    pub max_dppclk_vmax0p9;: f32;
    pub max_dppclk_vnom0p8;: f32;
    pub max_dppclk_vmid0p72;: f32;
    pub max_dppclk_vmin0p65;: f32;
    pub socclk;: f32;
    pub fabric_and_dram_bandwidth_vmax0p9;: f32;
    pub fabric_and_dram_bandwidth_vnom0p8;: f32;
    pub fabric_and_dram_bandwidth_vmid0p72;: f32;
    pub fabric_and_dram_bandwidth_vmin0p65;: f32;
    pub phyclkv_max0p9;: f32;
    pub phyclkv_nom0p8;: f32;
    pub phyclkv_mid0p72;: f32;
    pub phyclkv_min0p65;: f32;
    pub downspreading;: f32;
    pub round_trip_ping_latency_cycles;: i32;
    pub urgent_out_of_order_return_per_channel;: i32;
    pub number_of_channels: i32;
    pub vmm_page_size;: i32;
    pub dram_clock_change_latency;: f32;
    pub return_bus_width;: i32;
    pub percent_disp_bw_limit;: f32;
}
#[repr(C)]
pub struct dcn_ip_params {
    pub rob_buffer_size_in_kbyte: f32;
    pub det_buffer_size_in_kbyte: f32;
    pub dpp_output_buffer_pixels: f32;
    pub opp_output_buffer_lines: f32;
    pub pixel_chunk_size_in_kbyte: f32;
    pub pte_enable: dcn_bw_defs;
    pub pte_chunk_size;: i32;
    pub meta_chunk_size;: i32;
    pub writeback_chunk_size;: i32;
    pub odm_capability: dcn_bw_defs;
    pub dsc_capability: dcn_bw_defs;
    pub line_buffer_size;: i32;
    pub max_line_buffer_lines: i32;
    pub is_line_buffer_bpp_fixed: dcn_bw_defs;
    pub line_buffer_fixed_bpp: i32;
    pub writeback_luma_buffer_size;: i32;
    pub writeback_chroma_buffer_size;: i32;
    pub max_num_dpp: i32;
    pub max_num_writeback: i32;
    pub max_dchub_topscl_throughput;: i32;
    pub max_pscl_tolb_throughput;: i32;
    pub max_lb_tovscl_throughput;: i32;
    pub max_vscl_tohscl_throughput;: i32;
    pub max_hscl_ratio: f32;
    pub max_vscl_ratio: f32;
    pub max_hscl_taps: i32;
    pub max_vscl_taps: i32;
    pub pte_buffer_size_in_requests: i32;
    pub dispclk_ramping_margin;: f32;
    pub under_scan_factor: f32;
    pub max_inter_dcn_tile_repeaters: i32;
    pub can_vstartup_lines_exceed_vsync_plus_back_porch_lines_minus_one: dcn_bw_defs;
    pub bug_forcing_luma_and_chroma_request_to_same_size_fixed: dcn_bw_defs;
    pub dcfclk_cstate_latency: i32;
}
pub type dc = core::ffi::c_void;
pub type dc_state = core::ffi::c_void;
pub type dm_pp_clock_levels_with_voltage = core::ffi::c_void;
pub type dc_validate_mode = i32;
pub type swizzle_mode_values = i32;
pub type source_macro_tile_size = i32;
extern "C" {
    pub static dcn10_soc_defaults: dcn_soc_bounding_box;
    pub static dcn10_ip_defaults: dcn_ip_params;
    pub fn dcn_validate_bandwidth(dc: *mut dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> bool;
    pub fn dcn_get_soc_clks(dc: *mut dc, min_fclk_khz: *mut i32, min_dcfclk_khz: *mut i32, socclk_khz: *mut i32);
    pub fn dcn_bw_update_from_pplib_fclks(dc: *mut dc, fclks: *mut dm_pp_clock_levels_with_voltage);
    pub fn dcn_bw_update_from_pplib_dcfclks(dc: *mut dc, dcfclks: *mut dm_pp_clock_levels_with_voltage);
    pub fn dcn_bw_notify_pplib_of_wm_ranges(dc: *mut dc, min_fclk_khz: i32, min_dcfclk_khz: i32, socclk_khz: i32);
    pub fn dcn_bw_sync_calcs_and_dml(dc: *mut dc);
    pub fn swizzle_mode_to_macro_tile_size(sw_mode: swizzle_mode_values) -> source_macro_tile_size;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
