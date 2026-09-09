// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency: dml2_external_lib_deps.h

pub const DML_MAX_CLK_TABLE_SIZE: usize = 20;

#[repr(C)]
pub struct dml2_soc_derate_values {
    pub dram_derate_percent_pixel: ::core::ffi::c_uint,
    pub dram_derate_percent_vm: ::core::ffi::c_uint,
    pub dram_derate_percent_pixel_and_vm: ::core::ffi::c_uint,
    pub fclk_derate_percent: ::core::ffi::c_uint,
    pub dcfclk_derate_percent: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_soc_derates {
    pub system_active_urgent: dml2_soc_derate_values,
    pub system_active_average: dml2_soc_derate_values,
    pub dcn_mall_prefetch_urgent: dml2_soc_derate_values,
    pub dcn_mall_prefetch_average: dml2_soc_derate_values,
    pub system_idle_average: dml2_soc_derate_values,
}

#[repr(C)]
pub struct dml2_dcn32x_soc_qos_params_urgent_latency_us {
    pub base_latency_us: ::core::ffi::c_uint,
    pub base_latency_pixel_vm_us: ::core::ffi::c_uint,
    pub base_latency_vm_us: ::core::ffi::c_uint,
    pub scaling_factor_fclk_us: ::core::ffi::c_uint,
    pub scaling_factor_mhz: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_dcn32x_soc_qos_params {
    pub urgent_latency_us: dml2_dcn32x_soc_qos_params_urgent_latency_us,
    pub loaded_round_trip_latency_fclk_cycles: ::core::ffi::c_uint,
    pub urgent_out_of_order_return_per_channel_pixel_only_bytes: ::core::ffi::c_uint,
    pub urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: ::core::ffi::c_uint,
    pub urgent_out_of_order_return_per_channel_vm_only_bytes: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_dcn4_uclk_dpm_dependent_qos_params {
    pub minimum_uclk_khz: ::core::ffi::c_ulong,
    pub urgent_ramp_uclk_cycles: ::core::ffi::c_uint,
    pub trip_to_memory_uclk_cycles: ::core::ffi::c_uint,
    pub meta_trip_to_memory_uclk_cycles: ::core::ffi::c_uint,
    pub maximum_latency_when_urgent_uclk_cycles: ::core::ffi::c_uint,
    pub average_latency_when_urgent_uclk_cycles: ::core::ffi::c_uint,
    pub maximum_latency_when_non_urgent_uclk_cycles: ::core::ffi::c_uint,
    pub average_latency_when_non_urgent_uclk_cycles: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_dcn4x_soc_qos_params {
    pub df_qos_response_time_fclk_cycles: ::core::ffi::c_uint,
    pub max_round_trip_to_furthest_cs_fclk_cycles: ::core::ffi::c_uint,
    pub mall_overhead_fclk_cycles: ::core::ffi::c_uint,
    pub meta_trip_adder_fclk_cycles: ::core::ffi::c_uint,
    pub average_transport_distance_fclk_cycles: ::core::ffi::c_uint,
    pub umc_urgent_ramp_latency_margin: f64,
    pub umc_max_latency_margin: f64,
    pub umc_average_latency_margin: f64,
    pub fabric_max_transport_latency_margin: f64,
    pub fabric_average_transport_latency_margin: f64,
    pub per_uclk_dpm_params: [dml2_dcn4_uclk_dpm_dependent_qos_params; DML_MAX_CLK_TABLE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_qos_param_type {
    dml2_qos_param_type_dcn3,
    dml2_qos_param_type_dcn4x,
}

// Indicies mapped to DPM level
// Unpopulated indicies should fallback to the global derate value.
#[repr(C)]
pub struct dml2_soc_derate_values_per_dpm {
    pub dram_derate_percent_pixel: [::core::ffi::c_uint; DML_MAX_CLK_TABLE_SIZE],
    pub fclk_derate_percent: [::core::ffi::c_uint; DML_MAX_CLK_TABLE_SIZE],
    pub dcfclk_derate_percent: [::core::ffi::c_uint; DML_MAX_CLK_TABLE_SIZE],
}

#[repr(C)]
pub struct dml2_soc_derates_per_dpm {
    pub system_active_derates_per_dpm: dml2_soc_derate_values_per_dpm,
}

#[repr(C)]
pub struct dml2_soc_qos_parameters_writeback {
    pub base_latency_us: ::core::ffi::c_uint,
    pub scaling_factor_us: ::core::ffi::c_uint,
    pub scaling_factor_mhz: ::core::ffi::c_uint,
}

#[repr(C)]
pub union dml2_soc_qos_parameters_qos_params {
    pub dcn32x: dml2_dcn32x_soc_qos_params,
    pub dcn4x: dml2_dcn4x_soc_qos_params,
}

#[repr(C)]
pub struct dml2_soc_qos_parameters {
    pub derate_table: dml2_soc_derates,
    pub derate_table_per_dpm: dml2_soc_derates_per_dpm,
    pub writeback: dml2_soc_qos_parameters_writeback,
    pub qos_params: dml2_soc_qos_parameters_qos_params,
    pub qos_type: dml2_qos_param_type,
}

#[repr(C)]
pub struct dml2_soc_power_management_parameters {
    pub dram_clk_change_blackout_us: f64,
    pub dram_clk_change_read_only_us: f64, // deprecated
    pub dram_clk_change_write_only_us: f64, // deprecated
    pub fclk_change_blackout_us: f64,
    pub g7_ppt_blackout_us: f64,
    pub g7_temperature_read_blackout_us: f64,
    pub stutter_enter_plus_exit_latency_us: f64,
    pub stutter_exit_latency_us: f64,
    pub low_power_stutter_enter_plus_exit_latency_us: f64,
    pub low_power_stutter_exit_latency_us: f64,
    pub z8_stutter_enter_plus_exit_latency_us: f64,
    pub z8_stutter_exit_latency_us: f64,
    pub z8_min_idle_time: f64,
    pub g6_temp_read_blackout_us: [f64; DML_MAX_CLK_TABLE_SIZE],
    pub type_b_dram_clk_change_blackout_us: f64,
    pub type_b_ppt_blackout_us: f64,
    pub alternate_dram_carveout_size_mb: ::core::ffi::c_uint, // size per aperture - assumed same for both apertures for now
}

#[repr(C)]
pub struct dml2_clk_table {
    pub clk_values_khz: [::core::ffi::c_ulong; DML_MAX_CLK_TABLE_SIZE],
    pub num_clk_values: ::core::ffi::c_uchar,
}

#[repr(C)]
pub struct dml2_dram_params {
    pub channel_width_bytes: ::core::ffi::c_uint,
    pub channel_count: ::core::ffi::c_uint,
    pub transactions_per_clock: ::core::ffi::c_uint,
    pub alt_clock_bw_conversion: bool,
}

// ENABLE_WCK
#[repr(C)]
pub struct dml2_soc_state_table {
    pub wck_ratio: dml2_clk_table,
    pub uclk: dml2_clk_table,
    pub fclk: dml2_clk_table,
    pub dcfclk: dml2_clk_table,
    pub dispclk: dml2_clk_table,
    pub dppclk: dml2_clk_table,
    pub dtbclk: dml2_clk_table,
    pub phyclk: dml2_clk_table,
    pub socclk: dml2_clk_table,
    pub dscclk: dml2_clk_table,
    pub phyclk_d18: dml2_clk_table,
    pub phyclk_d32: dml2_clk_table,
    pub dram_config: dml2_dram_params,
}

#[repr(C)]
pub struct dml2_soc_vmin_clock_limits {
    pub dispclk_khz: ::core::ffi::c_ulong,
    pub dcfclk_khz: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct dml2_soc_bb {
    pub clk_table: dml2_soc_state_table,
    pub qos_parameters: dml2_soc_qos_parameters,
    pub power_management_parameters: dml2_soc_power_management_parameters,
    pub vmin_limit: dml2_soc_vmin_clock_limits,
    pub lower_bound_bandwidth_dchub: f64,
    pub fraction_of_urgent_bandwidth_nominal_target: f64,
    pub fraction_of_urgent_bandwidth_flip_target: f64,
    pub dprefclk_mhz: ::core::ffi::c_uint,
    pub xtalclk_mhz: ::core::ffi::c_uint,
    pub pcie_refclk_mhz: ::core::ffi::c_uint,
    pub dchub_refclk_mhz: ::core::ffi::c_uint,
    pub mall_allocated_for_dcn_mbytes: ::core::ffi::c_uint,
    pub max_outstanding_reqs: ::core::ffi::c_uint,
    pub fabric_datapath_to_dcn_data_return_bytes: ::core::ffi::c_ulong,
    pub return_bus_width_bytes: ::core::ffi::c_ulong,
    pub hostvm_min_page_size_kbytes: ::core::ffi::c_ulong,
    pub gpuvm_min_page_size_kbytes: ::core::ffi::c_ulong,
    pub hostvm_max_non_cached_page_table_levels: ::core::ffi::c_uint,
    pub gpuvm_max_page_table_levels: ::core::ffi::c_uint,
    pub phy_downspread_percent: f64,
    pub dcn_downspread_percent: f64,
    pub dispclk_dppclk_vco_speed_mhz: f64,
    pub no_dfs: bool,
    pub do_urgent_latency_adjustment: bool,
    pub mem_word_bytes: ::core::ffi::c_uint,
    pub num_dcc_mcaches: ::core::ffi::c_uint,
    pub mcache_size_bytes: ::core::ffi::c_uint,
    pub mcache_line_size_bytes: ::core::ffi::c_uint,
    pub max_fclk_for_uclk_dpm_khz: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct dml2_ip_capabilities_fams2 {
    pub max_allow_delay_us: ::core::ffi::c_uint,
    pub scheduling_delay_us: ::core::ffi::c_uint,
    pub vertical_interrupt_ack_delay_us: ::core::ffi::c_uint, // delay to acknowledge vline int
    pub allow_programming_delay_us: ::core::ffi::c_uint, // time requires to program allow
    pub min_allow_width_us: ::core::ffi::c_uint,
    pub subvp_df_throttle_delay_us: ::core::ffi::c_uint,
    pub subvp_programming_delay_us: ::core::ffi::c_uint,
    pub subvp_prefetch_to_mall_delay_us: ::core::ffi::c_uint,
    pub drr_programming_delay_us: ::core::ffi::c_uint,
    pub lock_timeout_us: ::core::ffi::c_uint,
    pub recovery_timeout_us: ::core::ffi::c_uint,
    pub flip_programming_delay_us: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_ip_capabilities {
    pub pipe_count: ::core::ffi::c_uint,
    pub otg_count: ::core::ffi::c_uint,
    pub TDLUT_33cube_count: ::core::ffi::c_uint,
    pub num_dsc: ::core::ffi::c_uint,
    pub max_num_dp2p0_streams: ::core::ffi::c_uint,
    pub max_num_hdmi_frl_outputs: ::core::ffi::c_uint,
    pub max_num_dp2p0_outputs: ::core::ffi::c_uint,
    pub max_num_wb: ::core::ffi::c_uint,
    pub rob_buffer_size_kbytes: ::core::ffi::c_uint,
    pub config_return_buffer_size_in_kbytes: ::core::ffi::c_uint,
    pub config_return_buffer_segment_size_in_kbytes: ::core::ffi::c_uint,
    pub meta_fifo_size_in_kentries: ::core::ffi::c_uint,
    pub compressed_buffer_segment_size_in_kbytes: ::core::ffi::c_uint,
    pub cursor_buffer_size: ::core::ffi::c_uint,
    pub max_flip_time_us: ::core::ffi::c_uint,
    pub max_flip_time_lines: ::core::ffi::c_uint,
    pub hostvm_mode: ::core::ffi::c_uint,
    pub dcn_mrq_present: bool,
    pub subvp_drr_scheduling_margin_us: ::core::ffi::c_uint,
    pub subvp_prefetch_end_to_mall_start_us: ::core::ffi::c_uint,
    pub subvp_fw_processing_delay: ::core::ffi::c_uint,
    pub max_vactive_det_fill_delay_us: ::core::ffi::c_uint,
    pub ppt_max_allow_delay_us: ::core::ffi::c_uint,
    pub temp_read_max_allow_delay_us: ::core::ffi::c_uint,
    pub dummy_pstate_max_allow_delay_us: ::core::ffi::c_uint,
    pub vblank_nom_default_us: ::core::ffi::c_uint,
    /* FAMS2 delays */
    pub fams2: dml2_ip_capabilities_fams2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
