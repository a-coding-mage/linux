// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Rust translation of dml2_internal_shared_types.h.  Types supplied by the
// included headers are intentionally left as external dependencies.

pub const DML_MCG_MAX_CLK_TABLE_SIZE: usize = 20;
pub const DML2_PMO_LEGACY_PREFETCH_MAX_TWAIT_OPTIONS: usize = 8;
pub const DML2_PMO_PSTATE_CANDIDATE_LIST_SIZE: usize = 10;
pub const DML2_PMO_STUTTER_CANDIDATE_LIST_SIZE: usize = 3;
pub const PMO_DCN4_MAX_DISPLAYS: usize = 4;
pub const PMO_DCN4_MAX_NUM_VARIANTS: usize = 2;
pub const PMO_DCN4_MAX_BASE_STRATEGIES: usize = 10;

extern "C" {
    pub fn dml2_status_str(status: dml2_status) -> *const core::ffi::c_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dram_bw_to_min_clk_table_entry { pub pre_derate_dram_bw_kbps: u64, pub min_uclk_khz: c_ulong, pub min_fclk_khz: c_ulong, pub min_dcfclk_khz: c_ulong }
#[repr(C)] pub struct dml2_mcg_dram_bw_to_min_clk_table { pub entries: [dram_bw_to_min_clk_table_entry; DML_MCG_MAX_CLK_TABLE_SIZE], pub num_entries: c_uint }
#[repr(C)] pub struct dml2_mcg_min_clock_table {
    pub max_clocks_khz: dml2_mcg_max_clocks, pub max_ss_clocks_khz: dml2_mcg_max_ss_clocks,
    pub fixed_clocks_khz: dml2_mcg_fixed_clocks, pub dram_bw_table: dml2_mcg_dram_bw_to_min_clk_table,
}
#[repr(C)] pub struct dml2_mcg_max_clocks { pub dispclk:c_uint,pub dppclk:c_uint,pub dscclk:c_uint,pub dtbclk:c_uint,pub phyclk:c_uint,pub fclk:c_uint,pub dcfclk:c_uint }
#[repr(C)] pub struct dml2_mcg_max_ss_clocks { pub dispclk:c_uint,pub dppclk:c_uint,pub dtbclk:c_uint }
#[repr(C)] pub struct dml2_mcg_fixed_clocks { pub dprefclk:c_uint,pub xtalclk:c_uint,pub pcierefclk:c_uint,pub dchubrefclk:c_uint,pub amclk:c_uint }
#[repr(C)] pub struct dml2_mcg_build_min_clock_table_params_in_out { pub soc_bb:*mut dml2_soc_bb, pub perform_pseudo_build: bool, pub min_clk_table:*mut dml2_mcg_min_clock_table }

#[repr(C)] pub struct dml2_soc_operating_point { pub uclk_khz:c_uint,pub fclk_khz:c_uint,pub dcfclk_khz:c_uint,pub socclk_khz:c_uint }
#[repr(C)] pub struct dml2_sop_constraint { pub min_sop_index:c_uint,pub latency:dml2_memory_path_latency,pub clocks:dml2_soc_operating_point,pub min_available_urgent_bandwidth_KBps:f64 }
#[repr(C)] pub struct dml2_sop_table { pub is_initialized:bool,pub model:*const utm_qos_model,pub sop_min_available_urgent_bandwidths_KBps:[u32;MAX_UTM_SOP_COUNT],pub sop_optimal_dcfclks_khz:[u32;MAX_UTM_SOP_COUNT],pub get_highest_sop_index:Option<unsafe extern "C" fn(*const dml2_sop_table)->c_uint>,pub get_sop_constraint_at_index:Option<unsafe extern "C" fn(*const dml2_sop_table,c_uint,*mut dml2_sop_constraint)>,pub is_bw_supported_at_index:Option<unsafe extern "C" fn(*const dml2_sop_table,*const dml2_memory_path_bandwidth,c_uint)->bool>,pub get_max_sop:Option<unsafe extern "C" fn(*const dml2_sop_table,*mut dml2_soc_operating_point)>,pub get_min_sop:Option<unsafe extern "C" fn(*const dml2_sop_table,*mut dml2_soc_operating_point)> }
#[repr(C)] pub struct dml2_utm_soc_bb { pub sop_table:dml2_sop_table,pub power_management_parameters:dml2_soc_power_management_parameters,pub vmin_limit:dml2_soc_vmin_clock_limits,pub dram_config:dml2_dram_params,pub qos_model:utm_qos_model,pub qos_model_dchub_v1:utm_qos_model_dchub_v1,pub lower_bound_bandwidth_dchub:f64,pub fraction_of_urgent_bandwidth_nominal_target:f64,pub fraction_of_urgent_bandwidth_flip_target:f64,pub dchub_refclk_mhz:c_uint,pub max_outstanding_reqs:c_uint,pub return_bus_width_bytes:c_ulong,pub phy_downspread_percent:f64,pub dcn_downspread_percent:f64,pub nominal_sdp_derate_percent:f64,pub urgent_sdp_derate_percent:f64,pub dispclk_dppclk_vco_speed_mhz:f64,pub no_dfs:bool,pub mem_word_bytes:c_uint,pub num_dcc_mcaches:c_uint,pub mcache_size_bytes:c_uint,pub mcache_line_size_bytes:c_uint,pub writeback_base_latency_us:c_uint,pub max_dispclk_khz:c_uint,pub max_dppclk_khz:c_uint,pub max_dscclk_khz:c_uint,pub max_dtbclk_khz:c_uint,pub max_phyclk_khz:c_uint,pub max_phyclk_d18_khz:c_uint,pub max_phyclk_d32_khz:c_uint,pub min_socclk_khz:c_uint,pub max_dcfclk_khz:c_uint,pub min_dcfclk_khz:c_uint,pub max_uclk_khz:c_uint,pub max_fclk_khz:c_uint }

#[repr(C)] pub struct dml2_pstate_per_method_common_meta { pub allow_start_otg_vline:c_int,pub allow_end_otg_vline:c_int,pub allow_time_us:f64,pub disallow_time_us:f64,pub period_us:f64 }
#[repr(C)] pub struct dml2_implicit_svp_meta { pub valid:bool,pub v_active:c_ulong,pub v_total:c_ulong,pub v_front_porch:c_ulong }
#[repr(C)] pub struct dml2_pstate_strategy { pub per_stream_pstate_method:[dml2_pstate_method;DML2_MAX_PLANES],pub allow_state_increase:bool }
#[repr(C)] pub struct dml2_optimization_stage1_state { pub performed:bool,pub success:bool,pub min_clk_index_for_latency:c_int }
#[repr(C)] pub struct dml2_optimization_stage2_state { pub performed:bool,pub success:bool,pub per_plane_mcache_support:[bool;DML2_MAX_PLANES],pub mcache_allocations:[dml2_mcache_surface_allocation;DML2_MAX_PLANES] }
#[repr(C)] pub struct dml2_optimization_stage3_state { pub performed:bool,pub success:bool,pub pstate_switch_modes:[dml2_pstate_method;DML2_MAX_PLANES],pub stream_svp_meta:[dml2_implicit_svp_meta;DML2_MAX_PLANES],pub fams2_required:bool,pub stream_pstate_meta:[dml2_pstate_meta;DML2_MAX_PLANES],pub min_clk_index_for_latency:c_int }
#[repr(C)] pub struct dml2_optimization_stage4_state { pub performed:bool,pub success:bool,pub unoptimizable_streams:[bool;DML2_MAX_DCN_PIPES] }
#[repr(C)] pub struct dml2_optimization_stage5_state { pub performed:bool,pub success:bool,pub optimal_reserved_time_in_vblank_us:bool,pub vblank_includes_z8_optimization:bool }

#[repr(C)] pub struct dml2_status_list { pub value:c_int }
#[repr(C)] pub struct dml2_mcg_instance { pub build_min_clock_table:Option<unsafe extern "C" fn(*mut dml2_mcg_build_min_clock_table_params_in_out)->bool> }
#[repr(C)] pub struct dml2_clock_granularity_adjuster { pub dcn_downspread_percent:f64,pub dispclk_dppclk_vco_speed_mhz:f64,pub dispclk_ramp_margin_percent:f64,pub max_dispclk_mhz:f64 }
#[repr(C)] pub struct dml2_core_internal_state_inputs { pub dummy:c_uint }
#[repr(C)] pub struct dml2_core_internal_state_intermediates { pub dummy:c_uint }

pub type c_uint = core::ffi::c_uint;
pub type c_int = core::ffi::c_int;
pub type c_ulong = core::ffi::c_ulong;
pub const MAX_UTM_SOP_COUNT: usize = 16;

#[repr(C)] pub enum dml2_status { DML2_STATUS_OK, DML2_STATUS_UNKNOWN, DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT, DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_PREFETCH, DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_PREFETCH_URGENT, DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_QOS_BANDWIDTH, DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_DCFCLK, DML2_STATUS_VALIDATE_FAIL_PREFETCH, DML2_STATUS_VALIDATE_FAIL_MCACHE, DML2_STATUS_VALIDATE_FAIL_PMO_SANITY_TOTAL_PIPE_USAGE, DML2_STATUS_VALIDATE_FAIL_PMO_SANITY_ODM_DIVISIBILITY, DML2_STATUS_VALIDATE_FAIL_PSTATE_SCHEDULE, DML2_STATUS_PSTATE_UNEXPECTED_PSTATE, DML2_STATUS_PSTATE_NOT_ADMISSIBLE, DML2_STATUS_OPTIMIZE_FAIL_MCACHE, DML2_STATUS_OPTIMIZE_FAIL_UCLK_PSTATE, DML2_STATUS_OPTIMIZE_FAIL_QOS, DML2_STATUS_OPTIMIZE_FAIL_VMIN, DML2_STATUS_OPTIMIZE_FAIL_STUTTER, DML2_STATUS_OPTIMIZE_FAIL_FCLK_PSTATE_UNSYNCHRONIZABLE_TIMINGS, DML2_STATUS_OPTIMIZE_FAIL_FCLK_PSTATE_INSUFFICENT_HIDING, DML2_STATUS_OPTIMIZE_FAIL_VMIN_DCFCLK, DML2_STATUS_OPTIMIZE_FAIL_EXCEED_MAX_ITERATION, DML2_STATUS_POPULATE_FAIL_MIN_CLOCK_STATE, DML2_STATUS_POPULATE_FAIL_PROGRAMMING, DML2_STATUS_POPULATE_FAIL_PROGRAMMING_PREFETCH, DML2_STATUS_POPULATE_FAIL_PROGRAMMING_PREFETCH_URGENT, DML2_STATUS_POPULATE_FAIL_PROGRAMMING_FLIP_BANDWIDTH, DML2_STATUS_POPULATE_FAIL_PROGRAMMING_DCFCLK }

// External declarations from the included DML headers.
pub enum dml2_soc_bb {} pub enum dml2_memory_path_latency {} pub enum dml2_memory_path_bandwidth {}
pub enum utm_qos_model {} pub enum dml2_soc_power_management_parameters {} pub enum dml2_soc_vmin_clock_limits {} pub enum dml2_dram_params {}
pub enum utm_qos_model_dchub_v1 {} pub enum dml2_mcache_surface_allocation {} pub enum dml2_pstate_meta {} pub enum dml2_pstate_method {}
pub const DML2_MAX_PLANES: usize = 16; pub const DML2_MAX_DCN_PIPES: usize = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
