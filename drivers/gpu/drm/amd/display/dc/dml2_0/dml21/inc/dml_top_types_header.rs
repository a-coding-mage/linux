// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
// Translated from dml_top_types.h. External types/constants are supplied by dependencies.

#[repr(C)]
pub struct dml2_instance { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_project_id { dml2_project_invalid=0, dml2_project_dcn4x_stage1, dml2_project_dcn4x_stage2, dml2_project_dcn4x_stage2_auto_drr_svp, dml2_project_dcn42, dml2_project_dcn4x_utm, dml2_project_dcn5x, dml2_project_dcn5x_utm, dml2_project_dcn6x_soc_var_a, dml2_project_dcn6x_soc_var_b, dml2_project_dcn6x=dml2_project_dcn6x_soc_var_b }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_pstate_change_support { dml2_pstate_change_vactive=0, dml2_pstate_change_vblank=1, dml2_pstate_change_vblank_and_vactive=2, dml2_pstate_change_drr=3, dml2_pstate_change_mall_svp=4, dml2_pstate_change_mall_full_frame=6, dml2_pstate_change_unsupported=7 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_output_type_and_rate__type { dml2_output_type_unknown=0, dml2_output_type_dp, dml2_output_type_edp, dml2_output_type_dp2p0, dml2_output_type_hdmi, dml2_output_type_hdmifrl }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_output_type_and_rate__rate { dml2_output_rate_unknown=0, dml2_output_rate_dp_rate_hbr, dml2_output_rate_dp_rate_hbr2, dml2_output_rate_dp_rate_hbr3, dml2_output_rate_dp_rate_uhbr10, dml2_output_rate_dp_rate_uhbr13p5, dml2_output_rate_dp_rate_uhbr20, dml2_output_rate_hdmi_rate_3x3, dml2_output_rate_hdmi_rate_6x3, dml2_output_rate_hdmi_rate_6x4, dml2_output_rate_hdmi_rate_8x4, dml2_output_rate_hdmi_rate_10x4, dml2_output_rate_hdmi_rate_12x4, dml2_output_rate_hdmi_rate_16x4, dml2_output_rate_hdmi_rate_20x4 }

#[repr(C)]
pub struct dml2_pmo_options {
 pub disable_vblank: bool, pub disable_svp: bool, pub disable_drr_var: bool, pub disable_drr_clamped: bool, pub disable_drr_var_when_var_active: bool, pub disable_drr_clamped_when_var_active: bool, pub disable_fams2: bool, pub disable_vactive_det_fill_bw_pad: bool, pub disable_dyn_odm: bool, pub disable_dyn_odm_for_multi_stream: bool, pub disable_dyn_odm_for_stream_with_svp: bool, pub force_mandatory_uclk_pstate_support: bool, pub disable_alternate_memory_training: bool, pub force_optional_uclk_pstate_support: bool, pub force_optional_mcache_support: bool, pub force_optional_ppt_temp_read_admissibility: bool,
 pub override_strategy_lists: [*mut dml2_pmo_pstate_strategy; DML2_MAX_PLANES], pub num_override_strategies_per_list: [u32; DML2_MAX_PLANES],
}
#[repr(C)] pub struct dml2_options { pub project_id: dml2_project_id, pub pmo_options: dml2_pmo_options }
pub struct utm_qos_mode;
#[repr(C)] pub struct dml2_initialize_instance_in_out { pub dml2_instance:*mut dml2_instance, pub options:dml2_options, pub soc_bb:dml2_soc_bb, pub ip_caps:dml2_ip_capabilities, pub overrides:dml2_initialize_overrides }
#[repr(C)] pub struct dml2_initialize_overrides { pub explicit_ip_bb:*mut core::ffi::c_void, pub explicit_ip_bb_size:u32, pub explicit_qos_model:*const utm_qos_model }
#[repr(C)] pub struct dml2_reset_instance_in_out { pub dml2_instance:*mut dml2_instance }
#[repr(C)] pub struct dml2_check_mode_supported_in_out { pub dml2_instance:*mut dml2_instance, pub display_config:*const dml2_display_cfg, pub is_supported:bool }

#[repr(C)] pub struct dml2_mcache_surface_allocation {
 pub valid:bool, pub requires_dedicated_mall_mcache:bool, pub num_mcaches_plane0:u32, pub num_mcaches_plane1:u32,
 pub mcache_x_offsets_plane0:[i32; DML2_MAX_MCACHES+1], pub mcache_x_offsets_plane1:[i32; DML2_MAX_MCACHES+1], pub shift_granularity:dml2_shift_granularity,
 pub global_mcache_ids_plane0:[i32; DML2_MAX_MCACHES+1], pub global_mcache_ids_plane1:[i32; DML2_MAX_MCACHES+1], pub global_mcache_ids_mall_plane0:[i32; DML2_MAX_MCACHES+1], pub global_mcache_ids_mall_plane1:[i32; DML2_MAX_MCACHES+1], pub last_slice_sharing:dml2_last_slice_sharing, pub informative:dml2_mcache_informative }
#[repr(C)] pub struct dml2_shift_granularity { pub p0:i32, pub p1:i32 }
#[repr(C)] pub struct dml2_last_slice_sharing { pub mall_comb_mcache_p0:bool, pub mall_comb_mcache_p1:bool, pub plane0_plane1:bool }
#[repr(C)] pub struct dml2_mcache_informative { pub meta_row_bytes_plane0:i32, pub meta_row_bytes_plane1:i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dml2_pstate_method { dml2_pstate_method_na=0, dml2_pstate_method_vactive=1, dml2_pstate_method_vblank=2, dml2_pstate_method_reserved_hw=5, dml2_pstate_method_fw_svp=6, dml2_pstate_method_reserved_fw=10, dml2_pstate_method_fw_vactive_drr=11, dml2_pstate_method_fw_vblank_drr, dml2_pstate_method_fw_svp_drr, dml2_pstate_method_reserved_fw_drr_clamped=20, dml2_pstate_method_fw_drr=21, dml2_pstate_method_reserved_fw_drr_var=22, dml2_pstate_method_alternate, dml2_pstate_method_count }
#[repr(C)] pub union dml2_per_plane_min_clocks { pub dcn4x:dml2_per_plane_dcn4x_clocks }
#[repr(C)] pub struct dml2_per_plane_dcn4x_clocks { pub dppclk_khz: u64 }
#[repr(C)] pub struct dml2_per_plane_programming { pub plane_descriptor:*const dml2_plane_parameters, pub min_clocks:dml2_per_plane_min_clocks, pub mcache_allocation:dml2_mcache_surface_allocation, pub num_dpps_required:u32, pub uclk_pstate_support_method:dml2_pstate_method, pub surface_size_mall_bytes:u32, pub svp_size_mall_bytes:u32, pub pipe_regs:[*mut dml2_dchub_per_pipe_register_set; DML2_MAX_PLANES], pub phantom_plane:dml2_phantom_plane }
#[repr(C)] pub struct dml2_phantom_plane { pub valid:bool, pub descriptor:dml2_plane_parameters, pub mcache_allocation:dml2_mcache_surface_allocation, pub pipe_regs:[*mut dml2_dchub_per_pipe_register_set; DML2_MAX_PLANES] }

// The remaining declarations retain the C layout and externally supplied types.
#[repr(C)] pub union dml2_global_sync_programming { pub dcn4x:dml2_global_sync_dcn4x }
#[repr(C)] pub struct dml2_global_sync_dcn4x { pub vstartup_lines:u32, pub vupdate_offset_pixels:u32, pub vupdate_vupdate_width_pixels:u32, pub vready_offset_pixels:u32, pub pstate_keepout_start_lines:u32 }
#[repr(C)] pub struct dml2_per_stream_programming { pub stream_descriptor:*const dml2_stream_parameters, pub global_sync:dml2_global_sync_programming, pub num_odms_required:u32, pub uclk_pstate_method:dml2_pstate_method, pub mcif_regs:[*mut dml2_mcif_per_pipe_register_set; DML2_MAX_WRITEBACK], pub phantom_stream:dml2_phantom_stream, pub fams2_base_params:dmub_cmd_fams2_config, pub fams2_sub_params:dml2_fams2_sub_params }
#[repr(C)] pub struct dml2_phantom_stream { pub enabled:bool, pub descriptor:dml2_stream_parameters, pub global_sync:dml2_global_sync_programming }
#[repr(C)] pub union dml2_fams2_sub_params { pub fams2_sub_params:dmub_cmd_fams2_config, pub fams2_sub_params_v2:dmub_fams2_stream_static_sub_state_v2 }

// Mode support and display programming are represented field-for-field below; dependent
// declarations are intentionally left to the included translation units.
#[repr(C)] pub struct dml2_mode_support_info { pub ModeIsSupported:bool, pub ImmediateFlipSupport:bool, pub WritebackLatencySupport:bool, pub ScaleRatioAndTapsSupport:bool, pub SourceFormatPixelAndScanSupport:bool, pub P2IWith420:bool, pub DSCOnlyIfNecessaryWithBPP:bool, pub DSC422NativeNotSupported:bool, pub LinkRateDoesNotMatchDPVersion:bool, pub LinkRateForMultistreamNotIndicated:bool, pub BPPForMultistreamNotIndicated:bool, pub MultistreamWithHDMIOreDP:bool, pub MSOOrODMSplitWithNonDPLink:bool, pub NotEnoughLanesForMSO:bool, pub NumberOfOTGSupport:bool, pub NumberOfHDMIFRLSupport:bool, pub NumberOfDP2p0Support:bool, pub NumberOfTDLUT33cubeSupport:bool, pub WritebackScaleRatioAndTapsSupport:bool, pub CursorSupport:bool, pub PitchSupport:bool, pub ViewportExceedsSurface:bool, pub ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified:bool, pub ExceededMALLSize:bool, pub EnoughWritebackUnits:bool, pub ExceededMultistreamSlots:bool, pub NotEnoughDSCUnits:bool, pub NotEnoughDSCSlices:bool, pub LinkCapacitySupport:bool, pub ROBSupport:bool, pub OutstandingRequestsSupport:bool, pub PTEBufferSizeNotExceeded:bool, pub DCCMetaBufferSizeNotExceeded:bool, pub TotalVerticalActiveBandwidthSupport:bool, pub VActiveBandwidthSupport:bool, pub FCLKChangeSupport:[dml2_pstate_change_support; DML2_MAX_PLANES], pub USRRetrainingSupport:bool, pub PrefetchSupported:bool, pub DynamicMetadataSupported:bool, pub VRatioInPrefetchSupported:bool, pub DISPCLK_DPPCLK_Support:bool, pub TotalAvailablePipesSupport:bool, pub ViewportSizeSupport:bool, pub ImmediateFlipSupportedForState:bool, pub MaxTotalVerticalActiveAvailableBandwidth:f64, pub MPCCombineEnable:[bool;DML2_MAX_PLANES], pub ODMMode:[dml2_odm_mode;DML2_MAX_PLANES], pub DPPPerSurface:[u32;DML2_MAX_PLANES], pub DSCEnabled:[bool;DML2_MAX_PLANES], pub FECEnabled:[bool;DML2_MAX_PLANES], pub NumberOfDSCSlices:[u32;DML2_MAX_PLANES], pub OutputBpp:[f64;DML2_MAX_PLANES], pub OutputType:[dml2_output_type_and_rate__type;DML2_MAX_PLANES], pub OutputRate:[dml2_output_type_and_rate__rate;DML2_MAX_PLANES], pub AlignedYPitch:[u32;DML2_MAX_PLANES], pub AlignedCPitch:[u32;DML2_MAX_PLANES], pub g6_temp_read_support:bool, pub temp_read_or_ppt_support:bool, pub qos_bandwidth_support:bool, pub dcfclk_support:bool }

// External declarations used by this header.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
