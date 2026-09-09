// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// header guard omitted
// #define __DML2_CORE_SHARED_TYPES_H__

// dependency: "dml2_external_lib_deps.h"
// dependency: "dml_top_display_cfg_types.h"
// dependency: "dml_top_types.h"
// dependency: "lib_frl_cap_check.h"

// #define __DML_VBA_DEBUG__
const __DML2_CALCS_MAX_VRATIO_PRE_OTO__: f64 = 4.0; //<brief max vratio for one-to-one prefetch bw scheduling
const __DML2_CALCS_MAX_VRATIO_PRE_EQU__: f64 = 6.0; //<brief max vratio for equalized prefetch bw scheduling
const __DML2_CALCS_MAX_VRATIO_PRE__: f64 = 8.0; //<brief max prefetch vratio register limit
const __DML2_CALCS_MAX_VSTARTUP__: f64 = 1023.0; // <brief max vstartup lines supported by hardware

const __DML2_CALCS_DPP_INVALID__: f64 = 0;
const __DML2_CALCS_DCFCLK_FACTOR__: f64 = 1.15; //<brief fudge factor for min dcfclk calclation
const __DML2_CALCS_PIPE_NO_PLANE__: f64 = 99;

#: [repr(C)]
pub struct dml2_core_ip_params {
    pub vblank_nom_default_us: u32;
    pub remote_iommu_outstanding_translations: u32;
    pub rob_buffer_size_kbytes: u32;
    pub config_return_buffer_size_in_kbytes: u32;
    pub config_return_buffer_segment_size_in_kbytes: u32;
    pub compressed_buffer_segment_size_in_kbytes: u32;
    pub meta_fifo_size_in_kentries: u32;
    pub dpte_buffer_size_in_pte_reqs_luma: u32;
    pub dpte_buffer_size_in_pte_reqs_chroma: u32;
    pub pixel_chunk_size_kbytes: u32;
    pub alpha_pixel_chunk_size_kbytes: u32;
    pub min_pixel_chunk_size_bytes: u32;
    pub writeback_chunk_size_kbytes: u32;
    pub line_buffer_size_bits: u32;
    pub max_line_buffer_lines: u32;
    pub writeback_interface_buffer_size_kbytes: u32;
    pub max_num_dpp: u32;
    pub max_num_opp: u32;
    pub max_num_otg: u32;
    pub TDLUT_33cube_count: u32;
    pub max_num_wb: u32;
    pub max_dchub_pscl_bw_pix_per_clk: u32;
    pub max_pscl_lb_bw_pix_per_clk: u32;
    pub max_lb_vscl_bw_pix_per_clk: u32;
    pub max_vscl_hscl_bw_pix_per_clk: u32;
    pub max_hscl_ratio: f64;
    pub max_vscl_ratio: f64;
    pub max_hscl_taps: u32;
    pub max_vscl_taps: u32;
    pub odm_combine_support_mask: u32;
    pub num_dsc: u32;
    pub maximum_dsc_slices_per_pipe: u32;
    pub maximum_dsc_bits_per_component: u32;
    pub maximum_pixels_per_line_per_dsc_unit: u32;
    pub dsc422_native_support: bool;
    pub cursor_64bpp_support: bool;
    pub dispclk_ramp_margin_percent: f64;
    pub dppclk_delay_subtotal: u32;
    pub dppclk_delay_scl: u32;
    pub dppclk_delay_scl_lb_only: u32;
    pub dppclk_delay_cnvc_formatter: u32;
    pub dppclk_delay_cnvc_cursor: u32;
    pub cursor_buffer_size: u32;
    pub cursor_chunk_size: u32;
    pub dispclk_delay_subtotal: u32;
    pub dynamic_metadata_vm_enabled: bool;
    pub max_inter_dcn_tile_repeaters: u32;
    pub max_num_hdmi_frl_outputs: u32;
    pub max_num_dp2p0_outputs: u32;
    pub max_num_dp2p0_streams: u32;
    pub dcc_supported: bool;
    pub ptoi_supported: bool;
    pub writeback_max_hscl_ratio: f64;
    pub writeback_max_vscl_ratio: f64;
    pub writeback_min_hscl_ratio: f64;
    pub writeback_min_vscl_ratio: f64;
    pub writeback_max_hscl_taps: u32;
    pub writeback_max_vscl_taps: u32;
    pub writeback_line_buffer_buffer_size: u32;

    pub words_per_channel: u32;
    pub imall_supported: bool;
    pub max_flip_time_us: u32;
    pub max_flip_time_lines: u32;
    pub subvp_swath_height_margin_lines: u32;
    pub subvp_fw_processing_delay_us: u32;
    pub subvp_pstate_allow_width_us: u32;
    pub alt_chan_fw_delay_us: u32; // FW delay value required in mode support calculation
	// MRQ
    pub dcn_mrq_present: bool;
    pub zero_size_buffer_entries: u32;
    pub compbuf_reserved_space_zs: u32;
    pub dcc_meta_buffer_size_bytes: u32;
    pub meta_chunk_size_kbytes: u32;
    pub min_meta_chunk_size_bytes: u32;

    pub dchub_arb_to_ret_delay: u32; // num of dcfclk
    pub hostvm_mode: u32;
    pub fams2_max_allow_delay_us: u32;
    pub fams2_min_allow_width_us: u32;
    pub ppt_max_allow_delay_us: u32;
    pub temp_read_max_allow_delay_us: u32;
    pub use_legacy_dsc_delay_formula: bool;
}

#: [repr(C)]
pub struct dml2_core_internal_DmlPipe {
    pub Dppclk: f64;
    pub Dispclk: f64;
    pub PixelClock: f64;
    pub DCFClkDeepSleep: f64;
    pub DPPPerSurface: u32;
    pub ScalerEnabled: bool;
    pub UPSPEnabled: bool;
    pub UPSPVTaps: u32;
    pub UPSPSamplePositioning: dml2_sample_positioning;
    pub RotationAngle: dml2_rotation_angle;
    pub mirrored: bool;
    pub ViewportHeight: u32;
    pub ViewportHeightC: u32;
    pub BlockWidth256BytesY: u32;
    pub BlockHeight256BytesY: u32;
    pub BlockWidth256BytesC: u32;
    pub BlockHeight256BytesC: u32;
    pub BlockWidthY: u32;
    pub BlockHeightY: u32;
    pub BlockWidthC: u32;
    pub BlockHeightC: u32;
    pub InterlaceEnable: u32;
    pub NumberOfCursors: u32;
    pub VBlank: u32;
    pub HTotal: u32;
    pub HActive: u32;
    pub DCCEnable: bool;
    pub ODMMode: dml2_odm_mode;
    pub SourcePixelFormat: dml2_source_format_class;
    pub SurfaceTiling: dml2_swizzle_mode;
    pub BytePerPixelY: u32;
    pub BytePerPixelC: u32;
    pub ProgressiveToInterlaceUnitInOPP: bool;
    pub VRatio: f64;
    pub VRatioChroma: f64;
    pub VTaps: u32;
    pub VTapsChroma: u32;
    pub PitchY: u32;
    pub PitchC: u32;
    pub ViewportStationary: bool;
    pub ViewportXStart: u32;
    pub ViewportYStart: u32;
    pub ViewportXStartC: u32;
    pub ViewportYStartC: u32;
    pub FORCE_ONE_ROW_FOR_FRAME: bool;
    pub SwathHeightY: u32;
    pub SwathHeightC: u32;

    pub DCCMetaPitchY: u32;
    pub DCCMetaPitchC: u32;
}

#: [repr(C)]
pub dml2_core_internal_request_type {
    dml2_core_internal_request_type_256_bytes = 0,
    dml2_core_internal_request_type_128_bytes_non_contiguous = 1,
    dml2_core_internal_request_type_128_bytes_contiguous = 2,
	dml2_core_internal_request_type_na = 3
}
#: [repr(C)]
pub dml2_core_internal_bw_type {
    dml2_core_internal_bw_sdp = 0,
    dml2_core_internal_bw_dram = 1,
	dml2_core_internal_bw_max
}

#: [repr(C)]
pub dml2_core_internal_soc_state_type {
    dml2_core_internal_soc_state_sys_active = 0,
    dml2_core_internal_soc_state_svp_prefetch = 1,
    dml2_core_internal_soc_state_sys_idle = 2,
	dml2_core_internal_soc_state_max
}

#: [repr(C)]
pub dml2_core_internal_output_type {
    dml2_core_internal_output_type_unknown = 0,
    dml2_core_internal_output_type_dp = 1,
    dml2_core_internal_output_type_edp = 2,
    dml2_core_internal_output_type_dp2p0 = 3,
    dml2_core_internal_output_type_hdmi = 4,
	dml2_core_internal_output_type_hdmifrl = 5
}

#: [repr(C)]
pub dml2_core_internal_output_type_rate {
    dml2_core_internal_output_rate_unknown = 0,
    dml2_core_internal_output_rate_dp_rate_hbr = 1,
    dml2_core_internal_output_rate_dp_rate_hbr2 = 2,
    dml2_core_internal_output_rate_dp_rate_hbr3 = 3,
    dml2_core_internal_output_rate_dp_rate_uhbr10 = 4,
    dml2_core_internal_output_rate_dp_rate_uhbr13p5 = 5,
    dml2_core_internal_output_rate_dp_rate_uhbr20 = 6,
    dml2_core_internal_output_rate_hdmi_rate_3x3 = 7,
    dml2_core_internal_output_rate_hdmi_rate_6x3 = 8,
    dml2_core_internal_output_rate_hdmi_rate_6x4 = 9,
    dml2_core_internal_output_rate_hdmi_rate_8x4 = 10,
    dml2_core_internal_output_rate_hdmi_rate_10x4 = 11,
    dml2_core_internal_output_rate_hdmi_rate_12x4 = 12,
    dml2_core_internal_output_rate_hdmi_rate_16x4 = 13,
	dml2_core_internal_output_rate_hdmi_rate_20x4 = 14
}

#: [repr(C)]
pub struct dml2_core_internal_watermarks {
    pub UrgentWatermark: f64;
    pub WritebackUrgentWatermark: f64;
    pub DRAMClockChangeWatermark: f64;
    pub FCLKChangeWatermark: f64;
    pub WritebackDRAMClockChangeWatermark: f64;
    pub WritebackFCLKChangeWatermark: f64;
    pub StutterExitWatermark: f64;
    pub StutterEnterPlusExitWatermark: f64;
    pub LowPowerStutterExitWatermark: f64;
    pub LowPowerStutterEnterPlusExitWatermark: f64;
    pub Z8StutterExitWatermark: f64;
    pub Z8StutterEnterPlusExitWatermark: f64;
    pub USRRetrainingWatermark: f64;
    pub temp_read_or_ppt_watermark_us: f64;
    pub writeback_temp_read_or_ppt_watermark_us: f64;
}

#: [repr(C)]
pub struct dml2_core_internal_mode_support_info {
	//-----------------
	// Mode Support Information
	//-----------------
    pub ImmediateFlipSupport: bool; //<brief Means mode support immediate flip at the max combine setting; determine in mode support and used in mode programming

	// Mode Support Reason/
    pub WritebackLatencySupport: bool;
    pub ScaleRatioAndTapsSupport: bool;
    pub SourceFormatPixelAndScanSupport: bool;
    pub P2IWith420: bool;
    pub DSCSlicesODMModeSupported: bool;
    pub DSCOnlyIfNecessaryWithBPP: bool;
    pub DSC422NativeNotSupported: bool;
    pub LinkRateDoesNotMatchDPVersion: bool;
    pub LinkRateForMultistreamNotIndicated: bool;
    pub BPPForMultistreamNotIndicated: bool;
    pub MultistreamWithHDMIOreDP: bool;
    pub MSOOrODMSplitWithNonDPLink: bool;
    pub NotEnoughLanesForMSO: bool;
    pub NumberOfOTGSupport: bool;
    pub NumberOfTDLUT33cubeSupport: bool;
    pub NumberOfHDMIFRLSupport: bool;
    pub NumberOfDP2p0Support: bool;
    pub WritebackScaleRatioAndTapsSupport: bool;
    pub CursorSupport: bool;
    pub PitchSupport: bool;
    pub ViewportExceedsSurface: bool;
	//bool ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified;
    pub ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe: bool;
    pub InvalidCombinationOfMALLUseForPStateAndStaticScreen: bool;
    pub InvalidCombinationOfMALLUseForPState: bool;
    pub ExceededMALLSize: bool;
    pub EnoughWritebackUnits: bool;

    pub ExceededMultistreamSlots: bool;
    pub NotEnoughDSCUnits: bool;
    pub NotEnoughDSCSlices: bool;
    pub PixelsPerLinePerDSCUnitSupport: bool;
    pub DSCCLKRequiredMoreThanSupported: bool;
    pub DTBCLKRequiredMoreThanSupported: bool;
    pub LinkCapacitySupport: bool;

    pub ROBSupport: bool;
    pub OutstandingRequestsSupport: bool;
    pub OutstandingRequestsUrgencyAvoidance: bool;

    pub PTEBufferSizeNotExceeded: bool;
    pub DCCMetaBufferSizeNotExceeded: bool;
	dml2_pstate_change_support DRAMClockChangeSupport: [DML2_MAX_PLANES];
	dml2_pstate_change_support FCLKChangeSupport: [DML2_MAX_PLANES];
	dml2_pstate_change_support temp_read_or_ppt_support: [DML2_MAX_PLANES];
    pub global_dram_clock_change_support_required: bool;
    pub global_dram_clock_change_supported: bool;
    pub global_fclk_change_supported: bool;
    pub global_temp_read_or_ppt_supported: bool;
    pub fclk_pstate_schedule_admissible: bool;
    pub temp_read_pstate_schedule_admissible: bool;
    pub ppt_pstate_schedule_admissible: bool;
    pub USRRetrainingSupport: bool;
    pub AvgBandwidthSupport: bool;
    pub UrgVactiveBandwidthSupport: bool;
    pub EnoughUrgentLatencyHidingSupport: bool;
    pub PrefetchScheduleSupported: bool;
    pub PrefetchSupported: bool;
    pub PrefetchBandwidthSupported: bool;
    pub DynamicMetadataSupported: bool;
    pub VRatioInPrefetchSupported: bool;
    pub DISPCLK_DPPCLK_Support: bool;
    pub TotalAvailablePipesSupport: bool;
    pub ODMSupport: bool;
    pub ModeSupport: bool;
    pub ViewportSizeSupport: bool;

    pub MPCCombineEnable: [bool; DML2_MAX_PLANES]; /// <brief Indicate if the MPC Combine enable in the given state and optimize mpc combine setting
	dml2_odm_mode ODMMode: [DML2_MAX_PLANES]; /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming stage
    pub DPPPerSurface: [u32; DML2_MAX_PLANES]; /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or 4.
    pub DSCEnabled: [bool; DML2_MAX_PLANES]; /// <brief Indicate if the DSC is actually required; used in mode_programming
    pub FECEnabled: [bool; DML2_MAX_PLANES]; /// <brief Indicate if the FEC is actually required
    pub NumberOfDSCSlices: [u32; DML2_MAX_PLANES]; /// <brief Indicate how many slices needed to support the given mode

    pub OutputBpp: [f64; DML2_MAX_PLANES];
	dml2_core_internal_output_type OutputType: [DML2_MAX_PLANES];
	dml2_core_internal_output_type_rate OutputRate: [DML2_MAX_PLANES];

    pub AlignedYPitch: [u32; DML2_MAX_PLANES];
    pub AlignedCPitch: [u32; DML2_MAX_PLANES];

    pub AlignedDCCMetaPitchY: [u32; DML2_MAX_PLANES];
    pub AlignedDCCMetaPitchC: [u32; DML2_MAX_PLANES];

    pub request_size_bytes_luma: [u32; DML2_MAX_PLANES];
    pub request_size_bytes_chroma: [u32; DML2_MAX_PLANES];
	dml2_core_internal_request_type RequestLuma: [DML2_MAX_PLANES];
	dml2_core_internal_request_type RequestChroma: [DML2_MAX_PLANES];

    pub DCCYMaxUncompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCYMaxCompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCYIndependentBlock: [u32; DML2_MAX_PLANES];
    pub DCCCMaxUncompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCCMaxCompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCCIndependentBlock: [u32; DML2_MAX_PLANES];

    pub avg_bandwidth_available_min: [f64; dml2_core_internal_soc_state_max];
	double avg_bandwidth_available: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
    pub urg_bandwidth_available_min_latency: [f64; dml2_core_internal_soc_state_max]; // min between SDP and DRAM, for latency evaluation
    pub urg_bandwidth_available_min: [f64; dml2_core_internal_soc_state_max]; // min between SDP and DRAM
	double urg_bandwidth_available: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
    pub urg_bandwidth_available_vm_only: [f64; dml2_core_internal_soc_state_max]; // the min of sdp bw and dram_vm_only bw, sdp has no different derate for vm/non-vm etc.
    pub urg_bandwidth_available_pixel_and_vm: [f64; dml2_core_internal_soc_state_max]; // the min of sdp bw and dram_pixel_and_vm bw, sdp has no different derate for vm/non-vm etc.

	double avg_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
	double urg_vactive_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // active bandwidth, scaled by urg burst factor
	double urg_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth, scaled by urg burst factor
	double urg_bandwidth_required_qual: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth, scaled by urg burst factor, use qual_row_bw
	double urg_bandwidth_required_flip: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth + flip

	double non_urg_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // same as urg_bandwidth, except not scaled by urg burst factor
	double non_urg_bandwidth_required_flip: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
	bool avg_bandwidth_support_ok: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
    pub bandwidth_upper_bound: dml2_memory_path_bandwidth;
    pub max_urgent_latency_us: f64;
    pub max_non_urgent_latency_us: f64;
    pub avg_non_urgent_latency_us: f64;
    pub avg_urgent_latency_us: f64;
    pub df_response_time_us: f64;

    pub incorrect_imall_usage: bool;

    pub g6_temp_read_support: bool;

    pub watermarks: dml2_core_internal_watermarks;
    pub dcfclk_support: bool;
    pub qos_bandwidth_support: bool;
    pub alternate_channel_size_support: bool;
}

#: [repr(C)]
pub struct dml2_core_internal_mode_support {
	// Physical info; only using for programming
    pub state_idx: u32; // <brief min clk state table index for mode support call
    pub qos_param_index: u32; // to access the uclk dependent qos_parameters table
    pub active_min_uclk_dpm_index: u32; // to access the min_clk table
    pub num_active_planes: u32; // <brief As determined by either e2e_pipe_param or display_cfg

	// Calculated Clocks
    pub RequiredDISPCLK: f64; /// <brief Required DISPCLK; depends on pixel rate; odm mode etc.
    pub RequiredDPPCLK: [f64; DML2_MAX_PLANES];
    pub RequiredDISPCLKPerSurface: [f64; DML2_MAX_PLANES];
    pub RequiredDTBCLK: [f64; DML2_MAX_PLANES];

    pub required_dscclk_freq_mhz: [f64; DML2_MAX_PLANES];

    pub FabricClock: f64; /// <brief Basically just the clock freq at the min (or given) state
    pub SOCCLK: f64; /// <brief Basically just the clock freq at the min (or given) state
    pub DCFCLK: f64; /// <brief Basically just the clock freq at the min (or given) state and max combine setting
    pub GlobalDPPCLK: f64; /// <brief the Max DPPCLK freq out of all pipes
    pub GlobalDTBCLK: f64; /// <brief the Max DTBCLK freq out of all pipes
    pub uclk_freq_mhz: f64;
    pub dram_bw_mbps: f64;
    pub max_dram_bw_mbps: f64;
    pub min_available_urgent_bandwidth_MBps: f64; /// <brief Minimum guaranteed available urgent return bandwidth in MBps

    pub MaxFabricClock: f64; /// <brief Basically just the clock freq at the min (or given) state
    pub MaxDCFCLK: f64; /// <brief Basically just the clock freq at the min (or given) state and max combine setting
    pub max_dispclk_freq_mhz: f64;
    pub max_dppclk_freq_mhz: f64;
    pub max_dscclk_freq_mhz: f64;

    pub NoTimeForPrefetch: [bool; DML2_MAX_PLANES];
    pub NoTimeForDynamicMetadata: [bool; DML2_MAX_PLANES];

	// ----------------------------------
	// Mode Support Info and fail reason
	// ----------------------------------
    pub support: dml2_core_internal_mode_support_info;

	// These are calculated before the ModeSupport and ModeProgram step
	// They represent the bound for the return buffer sizing
    pub MaxTotalDETInKByte: u32;
    pub NomDETInKByte: u32;
    pub MinCompressedBufferSizeInKByte: u32;

	// Info obtained at the end of mode support calculations
	// The reported info is at the "optimal" state and combine setting
    pub DETBufferSizeInKByte: [u32; DML2_MAX_PLANES]; // <brief Recommended DET size configuration for this plane. All pipes under this plane should program the DET buffer size to the calculated value.
    pub DETBufferSizeY: [u32; DML2_MAX_PLANES];
    pub DETBufferSizeC: [u32; DML2_MAX_PLANES];
    pub SwathHeightY: [u32; DML2_MAX_PLANES];
    pub SwathHeightC: [u32; DML2_MAX_PLANES];
    pub SwathWidthY: [u32; DML2_MAX_PLANES]; // per-pipe
    pub SwathWidthC: [u32; DML2_MAX_PLANES]; // per-pipe

	// ----------------------------------
	// Intermediates/Informational
	// ----------------------------------
    pub TotImmediateFlipBytes: u32;
    pub DCCEnabledInAnySurface: bool;
    pub WritebackRequiredDISPCLK: f64;
    pub TimeCalc: f64;
    pub TWait: [f64; DML2_MAX_PLANES];

    pub UnboundedRequestEnabled: bool;
    pub compbuf_reserved_space_64b: u32;
    pub hw_debug5: bool;
    pub CompressedBufferSizeInkByte: u32;
    pub VRatioPreY: [f64; DML2_MAX_PLANES];
    pub VRatioPreC: [f64; DML2_MAX_PLANES];
    pub req_per_swath_ub_l: [u32; DML2_MAX_PLANES];
    pub req_per_swath_ub_c: [u32; DML2_MAX_PLANES];
    pub swath_width_luma_ub: [u32; DML2_MAX_PLANES];
    pub swath_width_chroma_ub: [u32; DML2_MAX_PLANES];
    pub RequiredSlots: [u32; DML2_MAX_PLANES];
    pub vm_bytes: [u32; DML2_MAX_PLANES];
    pub DPTEBytesPerRow: [u32; DML2_MAX_PLANES];
    pub PrefetchLinesY: [u32; DML2_MAX_PLANES];
    pub PrefetchLinesC: [u32; DML2_MAX_PLANES];
    pub MaxNumSwathY: [u32; DML2_MAX_PLANES]; /// <brief Max number of swath for prefetch
    pub MaxNumSwathC: [u32; DML2_MAX_PLANES]; /// <brief Max number of swath for prefetch
    pub PrefillY: [u32; DML2_MAX_PLANES];
    pub PrefillC: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_l: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_c: [u32; DML2_MAX_PLANES];

    pub use_one_row_for_frame: [bool; DML2_MAX_PLANES];
    pub use_one_row_for_frame_flip: [bool; DML2_MAX_PLANES];

    pub dst_y_prefetch: [f64; DML2_MAX_PLANES];
    pub LinesForVM: [f64; DML2_MAX_PLANES];
    pub LinesForDPTERow: [f64; DML2_MAX_PLANES];
    pub SwathWidthYSingleDPP: [u32; DML2_MAX_PLANES];
    pub SwathWidthCSingleDPP: [u32; DML2_MAX_PLANES];
    pub BytePerPixelY: [u32; DML2_MAX_PLANES];
    pub BytePerPixelC: [u32; DML2_MAX_PLANES];
    pub BytePerPixelInDETY: [f64; DML2_MAX_PLANES];
    pub BytePerPixelInDETC: [f64; DML2_MAX_PLANES];

    pub Read256BlockHeightY: [u32; DML2_MAX_PLANES];
    pub Read256BlockWidthY: [u32; DML2_MAX_PLANES];
    pub Read256BlockHeightC: [u32; DML2_MAX_PLANES];
    pub Read256BlockWidthC: [u32; DML2_MAX_PLANES];
    pub MacroTileHeightY: [u32; DML2_MAX_PLANES];
    pub MacroTileHeightC: [u32; DML2_MAX_PLANES];
    pub MacroTileWidthY: [u32; DML2_MAX_PLANES];
    pub MacroTileWidthC: [u32; DML2_MAX_PLANES];

    pub surf_linear128_l: [bool; DML2_MAX_PLANES];
    pub surf_linear128_c: [bool; DML2_MAX_PLANES];

    pub PSCL_FACTOR: [f64; DML2_MAX_PLANES];
    pub PSCL_FACTOR_CHROMA: [f64; DML2_MAX_PLANES];
    pub MaximumSwathWidthLuma: [f64; DML2_MAX_PLANES];
    pub MaximumSwathWidthChroma: [f64; DML2_MAX_PLANES];
    pub Tno_bw: [f64; DML2_MAX_PLANES];
    pub Tno_bw_flip: [f64; DML2_MAX_PLANES];
    pub dst_y_per_vm_flip: [f64; DML2_MAX_PLANES];
    pub dst_y_per_row_flip: [f64; DML2_MAX_PLANES];
    pub WritebackDelayTime: [f64; DML2_MAX_PLANES];
    pub dpte_group_bytes: [u32; DML2_MAX_PLANES];
    pub dpte_row_height: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_chroma: [u32; DML2_MAX_PLANES];
    pub UrgLatency: f64;
    pub TripToMemory: f64;
    pub UrgentBurstFactorCursor: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorCursorPre: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorLuma: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorLumaPre: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorChroma: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorChromaPre: [f64; DML2_MAX_PLANES];
    pub MaximumSwathWidthInLineBufferLuma: f64;
    pub MaximumSwathWidthInLineBufferChroma: f64;
    pub ExtraLatency: f64;
    pub ExtraLatency_sr: f64;
    pub ExtraLatencyPrefetch: f64;

    pub dcc_dram_bw_nom_overhead_factor_p0: [f64; DML2_MAX_PLANES]; // overhead to request meta
    pub dcc_dram_bw_nom_overhead_factor_p1: [f64; DML2_MAX_PLANES];
    pub dcc_dram_bw_pref_overhead_factor_p0: [f64; DML2_MAX_PLANES]; // overhead to request meta
    pub dcc_dram_bw_pref_overhead_factor_p1: [f64; DML2_MAX_PLANES];
    pub mall_prefetch_sdp_overhead_factor: [f64; DML2_MAX_PLANES]; // overhead to the imall or phantom pipe
    pub mall_prefetch_dram_overhead_factor: [f64; DML2_MAX_PLANES];

    pub is_using_mall_for_ss: [bool; DML2_MAX_PLANES];
    pub meta_row_width_chroma: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqHeightC: [u32; DML2_MAX_PLANES];
    pub PTE_BUFFER_MODE: [bool; DML2_MAX_PLANES];
    pub meta_req_height_chroma: [u32; DML2_MAX_PLANES];
    pub meta_pte_bytes_per_frame_ub_c: [u32; DML2_MAX_PLANES];
    pub dpde0_bytes_per_frame_ub_c: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_luma_ub: [u32; DML2_MAX_PLANES];
    pub meta_req_width: [u32; DML2_MAX_PLANES];
    pub meta_row_width: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqWidthY: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_linear: [u32; DML2_MAX_PLANES];
    pub PTERequestSizeY: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_chroma_ub: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqWidthC: [u32; DML2_MAX_PLANES];
    pub meta_pte_bytes_per_frame_ub_l: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_linear_chroma: [u32; DML2_MAX_PLANES];
    pub PTERequestSizeC: [u32; DML2_MAX_PLANES];
    pub meta_req_height: [u32; DML2_MAX_PLANES];
    pub dpde0_bytes_per_frame_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_req_width_chroma: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqHeightY: [u32; DML2_MAX_PLANES];
    pub BIGK_FRAGMENT_SIZE: [u32; DML2_MAX_PLANES];
    pub vm_group_bytes: [u32; DML2_MAX_PLANES];
    pub VReadyOffsetPix: [u32; DML2_MAX_PLANES];
    pub VUpdateOffsetPix: [u32; DML2_MAX_PLANES];
    pub VUpdateWidthPix: [u32; DML2_MAX_PLANES];
    pub TSetup: [f64; DML2_MAX_PLANES];
    pub Tdmdl_vm_raw: [f64; DML2_MAX_PLANES];
    pub Tdmdl_raw: [f64; DML2_MAX_PLANES];
    pub VStartupMin: [u32; DML2_MAX_PLANES]; /// <brief Minimum vstartup to meet the prefetch schedule (i.e. the prefetch solution can be found at this vstartup time); not the actual global sync vstartup pos.
    pub MaxActiveDRAMClockChangeLatencySupported: [f64; DML2_MAX_PLANES];
    pub MaxActiveFCLKChangeLatencySupported: f64;

	// Backend
    pub RequiresDSC: [bool; DML2_MAX_PLANES];
    pub RequiresFEC: [bool; DML2_MAX_PLANES];
    pub OutputBpp: [f64; DML2_MAX_PLANES];
    pub DesiredOutputBpp: [f64; DML2_MAX_PLANES];
    pub PixelClockBackEnd: [f64; DML2_MAX_PLANES];
    pub DSCDelay: [u32; DML2_MAX_PLANES];
    pub use_legacy_dsc_delay_formula: bool;
	dml2_core_internal_output_type OutputType: [DML2_MAX_PLANES];
	dml2_core_internal_output_type_rate OutputRate: [DML2_MAX_PLANES];
    pub TotalAvailablePipesSupportNoDSC: bool;
    pub TotalAvailablePipesSupportDSC: bool;
    pub NumberOfDPPNoDSC: u32;
    pub NumberOfDPPDSC: u32;
    pub ODMModeNoDSC: dml2_odm_mode;
    pub ODMModeDSC: dml2_odm_mode;
    pub RequiredDISPCLKPerSurfaceNoDSC: f64;
    pub RequiredDISPCLKPerSurfaceDSC: f64;
    pub EstimatedNumberOfDSCSlices: [u32; DML2_MAX_PLANES];

	// Bandwidth Related Info
    pub BandwidthAvailableForImmediateFlip: f64;
    pub vactive_sw_bw_l: [f64; DML2_MAX_PLANES]; // no dcc overhead, for the plane
    pub vactive_sw_bw_c: [f64; DML2_MAX_PLANES];
	double WriteBandwidth: [DML2_MAX_PLANES]: [DML2_MAX_WRITEBACK];
    pub RequiredPrefetchPixelDataBWLuma: [f64; DML2_MAX_PLANES];
    pub RequiredPrefetchPixelDataBWChroma: [f64; DML2_MAX_PLANES];
	/* Max bandwidth calculated from prefetch schedule should be considered in addition to the pixel data bw to avoid ms/mp mismatches.
	 * 1. oto bw should also be considered when calculating peak urgent bw to avoid situations oto/equ mismatches between ms and mp
	 *
	 * 2. equ bandwidth needs to be considered for calculating peak urgent bw when equ schedule is used in mode support.
	 *    Some slight difference in variables may cause the pixel data bandwidth to be higher
	 *    even though overall equ prefetch bandwidths can be lower going from ms to mp
	 */
    pub RequiredPrefetchBWMax: [f64; DML2_MAX_PLANES];
    pub cursor_bw: [f64; DML2_MAX_PLANES];
    pub prefetch_cursor_bw: [f64; DML2_MAX_PLANES];
    pub prefetch_vmrow_bw: [f64; DML2_MAX_PLANES];
    pub final_flip_bw: [f64; DML2_MAX_PLANES];
    pub meta_row_bw: [f64; DML2_MAX_PLANES];
    pub meta_row_bytes: [u32; DML2_MAX_PLANES];
    pub dpte_row_bw: [f64; DML2_MAX_PLANES];
    pub excess_vactive_fill_bw_l: [f64; DML2_MAX_PLANES];
    pub excess_vactive_fill_bw_c: [f64; DML2_MAX_PLANES];
	double surface_avg_vactive_required_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];
	double surface_peak_required_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];

	// Something that should be feedback to caller
	dml2_odm_mode ODMMode: [DML2_MAX_PLANES];
    pub SurfaceSizeInMALL: [u32; DML2_MAX_PLANES];
    pub NoOfDPP: [u32; DML2_MAX_PLANES];
    pub NoOfOPP: [u32; DML2_MAX_PLANES];
    pub MPCCombine: [bool; DML2_MAX_PLANES];
    pub dcfclk_deepsleep: f64;
    pub MinDPPCLKUsingSingleDPP: [f64; DML2_MAX_PLANES];
    pub SingleDPPViewportSizeSupportPerSurface: [bool; DML2_MAX_PLANES];
    pub ImmediateFlipSupportedForPipe: [bool; DML2_MAX_PLANES];
    pub NotEnoughUrgentLatencyHiding: [bool; DML2_MAX_PLANES];
    pub NotEnoughUrgentLatencyHidingPre: [bool; DML2_MAX_PLANES];
    pub PTEBufferSizeNotExceeded: [bool; DML2_MAX_PLANES];
    pub DCCMetaBufferSizeNotExceeded: [bool; DML2_MAX_PLANES];
    pub TotalNumberOfActiveDPP: u32;
    pub TotalNumberOfActiveOPP: u32;
    pub TotalNumberOfSingleDPPSurfaces: u32;
    pub TotalNumberOfDCCActiveDPP: u32;
    pub Total3dlutActive: u32;

    pub SubViewportLinesNeededInMALL: [u32; DML2_MAX_PLANES];
    pub VActiveLatencyHidingMargin: [f64; DML2_MAX_PLANES];
    pub VActiveLatencyHidingUs: [f64; DML2_MAX_PLANES];
    pub MaxVStartupLines: [u32; DML2_MAX_PLANES];
	double pstate_vactive_det_fill_delay_us: [dml2_pstate_type_count]: [DML2_MAX_PLANES];

    pub num_mcaches_l: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_l: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_per_channel_l: [u32; DML2_MAX_PLANES];
	unsigned int mcache_offsets_l: [DML2_MAX_PLANES]: [DML2_MAX_MCACHES + 1];
    pub mcache_shift_granularity_l: [u32; DML2_MAX_PLANES];

    pub num_mcaches_c: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_c: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_per_channel_c: [u32; DML2_MAX_PLANES];
	unsigned int mcache_offsets_c: [DML2_MAX_PLANES]: [DML2_MAX_MCACHES + 1];
    pub mcache_shift_granularity_c: [u32; DML2_MAX_PLANES];

    pub mall_comb_mcache_l: [bool; DML2_MAX_PLANES];
    pub mall_comb_mcache_c: [bool; DML2_MAX_PLANES];
    pub lc_comb_mcache: [bool; DML2_MAX_PLANES];

    pub vmpg_width_y: [u32; DML2_MAX_PLANES];
    pub vmpg_height_y: [u32; DML2_MAX_PLANES];
    pub vmpg_width_c: [u32; DML2_MAX_PLANES];
    pub vmpg_height_c: [u32; DML2_MAX_PLANES];

    pub meta_row_height_luma: [u32; DML2_MAX_PLANES];
    pub meta_row_height_chroma: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_c: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_l: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_c: [u32; DML2_MAX_PLANES];

	unsigned int pstate_bytes_required_l: [dml2_pstate_type_count]: [DML2_MAX_PLANES];
	unsigned int pstate_bytes_required_c: [dml2_pstate_type_count]: [DML2_MAX_PLANES];
    pub cursor_bytes_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes_per_line: [u32; DML2_MAX_PLANES];

    pub MaximumVStartup: [u32; DML2_MAX_PLANES];

    pub HostVMInefficiencyFactor: f64;
    pub HostVMInefficiencyFactorPrefetch: f64;

    pub tdlut_pte_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_groups_per_2row_ub: [u32; DML2_MAX_PLANES];
    pub tdlut_opt_time: [f64; DML2_MAX_PLANES];
    pub tdlut_drain_time: [f64; DML2_MAX_PLANES];
    pub tdlut_bytes_per_group: [u32; DML2_MAX_PLANES];

    pub Tvm_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip_rounded: [f64; DML2_MAX_PLANES];

    pub DSTYAfterScaler: [u32; DML2_MAX_PLANES];
    pub DSTXAfterScaler: [u32; DML2_MAX_PLANES];

	dml2_pstate_method uclk_pstate_switch_modes: [DML2_MAX_PLANES];
    pub svp0_max_bytes: u32;
    pub svp1_max_bytes: u32;
    pub svp0_max_bytes_per_dpp: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp0_max_bytes_per_dpp_c: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp1_max_bytes_per_dpp: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp1_max_bytes_per_dpp_c: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp0_dst_lines: [u32; DML2_MAX_PLANES]; // per stream
    pub svp1_dst_lines: [u32; DML2_MAX_PLANES]; // per stream
    pub svp_req_limit: [u32; DML2_MAX_PLANES]; // per stream, should be the same value in time between all streams max(2 swaths, dst_y_pre) over all planes
    pub nom_req_limit_alt: [u32; DML2_MAX_PLANES];
    pub min_lead_dst_lines: [u32; DML2_MAX_PLANES];
    pub total_swaths: [u32; DML2_MAX_PLANES];
    pub total_swaths_c: [u32; DML2_MAX_PLANES];
    pub prefetch_swaths: [u32; DML2_MAX_PLANES];
    pub prefetch_swaths_c: [u32; DML2_MAX_PLANES];
    pub prefetch_hdl_delta: [f64; DML2_MAX_PLANES]; // in dst
    pub recout_hdl_delta: [f64; DML2_MAX_PLANES]; // in dst
    pub prefetch_hdl_delta_c: [f64; DML2_MAX_PLANES]; // in dst
    pub recout_hdl_delta_c: [f64; DML2_MAX_PLANES]; // in dst
    pub max_prefetch_in_lines: [u32; DML2_MAX_PLANES]; // max prefetch time over all planes converted to lines for the given stream
    pub lsdma_bw_req_for_alt_kbps: f64;

	// Synchronized timing group assignement
    pub timing_group_id: [u32; DML2_MAX_PLANES]; // timing group id for the given plane
    pub timing_group_count: u32;

    pub fclk_pstate_required: bool;
    pub ppt_pstate_required: bool;
    pub temp_read_pstate_required: bool;
	// P-state schedule windows
    pub fclk_pstate_allow_start_us: [f64; DML2_MAX_PLANES];
    pub fclk_pstate_allow_end_us: [f64; DML2_MAX_PLANES];
    pub ppt_pstate_allow_start_us: [f64; DML2_MAX_PLANES];
    pub ppt_pstate_allow_end_us: [f64; DML2_MAX_PLANES];
    pub temp_read_pstate_allow_start_us: [f64; DML2_MAX_PLANES];
    pub temp_read_pstate_allow_end_us: [f64; DML2_MAX_PLANES];
}

/// @brief A mega structure that houses various info for model programming step.
#: [repr(C)]
pub struct dml2_core_internal_mode_program {
    pub qos_param_index: u32; // to access the uclk dependent dpm table
    pub active_min_uclk_dpm_index: u32; // to access the min_clk table
    pub FabricClock: f64; /// <brief Basically just the clock freq at the min (or given) state
	//double DCFCLK; /// <brief Basically just the clock freq at the min (or given) state and max combine setting
    pub dram_bw_mbps: f64;
    pub min_available_urgent_bandwidth_MBps: f64; /// <brief Minimum guaranteed available urgent return bandwidth in MBps
    pub uclk_freq_mhz: f64;
    pub NoOfDPP: [u32; DML2_MAX_PLANES];
	dml2_odm_mode ODMMode: [DML2_MAX_PLANES];

	//-------------
	// Intermediate/Informational
	//-------------
    pub UrgentLatency: f64;
    pub TripToMemory: f64;
    pub MetaTripToMemory: f64;
    pub VInitPreFillY: [u32; DML2_MAX_PLANES];
    pub VInitPreFillC: [u32; DML2_MAX_PLANES];
    pub MaxNumSwathY: [u32; DML2_MAX_PLANES];
    pub MaxNumSwathC: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_l: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_c: [u32; DML2_MAX_PLANES];

    pub BytePerPixelInDETY: [f64; DML2_MAX_PLANES];
    pub BytePerPixelInDETC: [f64; DML2_MAX_PLANES];
    pub BytePerPixelY: [u32; DML2_MAX_PLANES];
    pub BytePerPixelC: [u32; DML2_MAX_PLANES];
    pub SwathWidthY: [u32; DML2_MAX_PLANES]; // per-pipe
    pub SwathWidthC: [u32; DML2_MAX_PLANES]; // per-pipe
    pub req_per_swath_ub_l: [u32; DML2_MAX_PLANES];
    pub req_per_swath_ub_c: [u32; DML2_MAX_PLANES];
    pub SwathWidthSingleDPPY: [u32; DML2_MAX_PLANES];
    pub SwathWidthSingleDPPC: [u32; DML2_MAX_PLANES];
    pub vactive_sw_bw_l: [f64; DML2_MAX_PLANES];
    pub vactive_sw_bw_c: [f64; DML2_MAX_PLANES];
    pub excess_vactive_fill_bw_l: [f64; DML2_MAX_PLANES];
    pub excess_vactive_fill_bw_c: [f64; DML2_MAX_PLANES];

    pub PixelPTEBytesPerRow: [u32; DML2_MAX_PLANES];
    pub vm_bytes: [u32; DML2_MAX_PLANES];
    pub PrefetchSourceLinesY: [u32; DML2_MAX_PLANES];
    pub RequiredPrefetchPixelDataBWLuma: [f64; DML2_MAX_PLANES];
    pub RequiredPrefetchPixelDataBWChroma: [f64; DML2_MAX_PLANES];
    pub PrefetchSourceLinesC: [u32; DML2_MAX_PLANES];
    pub PSCL_THROUGHPUT: [f64; DML2_MAX_PLANES];
    pub PSCL_THROUGHPUT_CHROMA: [f64; DML2_MAX_PLANES];
    pub DSCDelay: [u32; DML2_MAX_PLANES];
    pub use_legacy_dsc_delay_formula: bool;
    pub DPPCLKUsingSingleDPP: [f64; DML2_MAX_PLANES];

    pub Read256BlockHeightY: [u32; DML2_MAX_PLANES];
    pub Read256BlockWidthY: [u32; DML2_MAX_PLANES];
    pub Read256BlockHeightC: [u32; DML2_MAX_PLANES];
    pub Read256BlockWidthC: [u32; DML2_MAX_PLANES];
    pub MacroTileHeightY: [u32; DML2_MAX_PLANES];
    pub MacroTileHeightC: [u32; DML2_MAX_PLANES];
    pub MacroTileWidthY: [u32; DML2_MAX_PLANES];
    pub MacroTileWidthC: [u32; DML2_MAX_PLANES];
    pub MaximumSwathWidthLuma: [f64; DML2_MAX_PLANES];
    pub MaximumSwathWidthChroma: [f64; DML2_MAX_PLANES];

    pub surf_linear128_l: [bool; DML2_MAX_PLANES];
    pub surf_linear128_c: [bool; DML2_MAX_PLANES];

    pub SurfaceSizeInTheMALL: [u32; DML2_MAX_PLANES];
    pub VRatioPrefetchY: [f64; DML2_MAX_PLANES];
    pub VRatioPrefetchC: [f64; DML2_MAX_PLANES];
    pub Tno_bw: [f64; DML2_MAX_PLANES];
    pub Tno_bw_flip: [f64; DML2_MAX_PLANES];
    pub final_flip_bw: [f64; DML2_MAX_PLANES];
    pub prefetch_vmrow_bw: [f64; DML2_MAX_PLANES];
    pub cursor_bw: [f64; DML2_MAX_PLANES];
    pub prefetch_cursor_bw: [f64; DML2_MAX_PLANES];
    pub WritebackDelay: [f64; DML2_MAX_PLANES];
    pub dpte_row_height: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_linear: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_luma_ub: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_chroma_ub: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_chroma: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_linear_chroma: [u32; DML2_MAX_PLANES];
    pub vm_group_bytes: [u32; DML2_MAX_PLANES];
    pub dpte_group_bytes: [u32; DML2_MAX_PLANES];

    pub dpte_row_bw: [f64; DML2_MAX_PLANES];
    pub time_per_tdlut_group: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorCursor: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorCursorPre: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorLuma: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorLumaPre: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorChroma: [f64; DML2_MAX_PLANES];
    pub UrgentBurstFactorChromaPre: [f64; DML2_MAX_PLANES];

    pub MaximumSwathWidthInLineBufferLuma: f64;
    pub MaximumSwathWidthInLineBufferChroma: f64;

    pub vmpg_width_y: [u32; DML2_MAX_PLANES];
    pub vmpg_height_y: [u32; DML2_MAX_PLANES];
    pub vmpg_width_c: [u32; DML2_MAX_PLANES];
    pub vmpg_height_c: [u32; DML2_MAX_PLANES];

    pub meta_row_bw: [f64; DML2_MAX_PLANES];
    pub meta_row_bytes: [u32; DML2_MAX_PLANES];
    pub meta_req_width: [u32; DML2_MAX_PLANES];
    pub meta_req_height: [u32; DML2_MAX_PLANES];
    pub meta_row_width: [u32; DML2_MAX_PLANES];
    pub meta_row_height: [u32; DML2_MAX_PLANES];
    pub meta_req_width_chroma: [u32; DML2_MAX_PLANES];
    pub meta_row_height_chroma: [u32; DML2_MAX_PLANES];
    pub meta_row_width_chroma: [u32; DML2_MAX_PLANES];
    pub meta_req_height_chroma: [u32; DML2_MAX_PLANES];

    pub swath_width_luma_ub: [u32; DML2_MAX_PLANES];
    pub swath_width_chroma_ub: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqWidthY: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqHeightY: [u32; DML2_MAX_PLANES];
    pub PTERequestSizeY: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqWidthC: [u32; DML2_MAX_PLANES];
    pub PixelPTEReqHeightC: [u32; DML2_MAX_PLANES];
    pub PTERequestSizeC: [u32; DML2_MAX_PLANES];

    pub TWait: [f64; DML2_MAX_PLANES];
    pub Tdmdl_vm_raw: [f64; DML2_MAX_PLANES];
    pub Tdmdl_vm: [f64; DML2_MAX_PLANES];
    pub Tdmdl_raw: [f64; DML2_MAX_PLANES];
    pub Tdmdl: [f64; DML2_MAX_PLANES];
    pub TSetup: [f64; DML2_MAX_PLANES];
    pub dpde0_bytes_per_frame_ub_l: [u32; DML2_MAX_PLANES];
    pub dpde0_bytes_per_frame_ub_c: [u32; DML2_MAX_PLANES];

    pub meta_pte_bytes_per_frame_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_pte_bytes_per_frame_ub_c: [u32; DML2_MAX_PLANES];

    pub UnboundedRequestEnabled: bool;
    pub CompressedBufferSizeInkByte: u32;
    pub compbuf_reserved_space_64b: u32;
    pub hw_debug5: bool;
    pub dcfclk_deep_sleep_hysteresis: u32;
    pub min_return_latency_in_dcfclk: u32;

    pub NotEnoughUrgentLatencyHiding: [bool; DML2_MAX_PLANES];
    pub NotEnoughUrgentLatencyHidingPre: [bool; DML2_MAX_PLANES];
    pub ExtraLatency: f64;
    pub ExtraLatency_sr: f64;
    pub ExtraLatencyPrefetch: f64;
    pub PrefetchAndImmediateFlipSupported: bool;
    pub TotalDataReadBandwidth: f64;
    pub BandwidthAvailableForImmediateFlip: f64;
    pub NotEnoughTimeForDynamicMetadata: [bool; DML2_MAX_PLANES];

    pub use_one_row_for_frame: [bool; DML2_MAX_PLANES];
    pub use_one_row_for_frame_flip: [bool; DML2_MAX_PLANES];

    pub TCalc: f64;
    pub TotImmediateFlipBytes: u32;

    pub MaxTotalDETInKByte: u32;
    pub NomDETInKByte: u32;
    pub MinCompressedBufferSizeInKByte: u32;
    pub PixelClockBackEnd: [f64; DML2_MAX_PLANES];
    pub OutputBpp: [f64; DML2_MAX_PLANES];
    pub dsc_enable: [bool; DML2_MAX_PLANES];
    pub num_dsc_slices: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_c: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_l: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_c: [u32; DML2_MAX_PLANES];
    pub cursor_bytes_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes_per_line: [u32; DML2_MAX_PLANES];
    pub MaxVStartupLines: [u32; DML2_MAX_PLANES]; /// <brief more like vblank for the plane's OTG
    pub HostVMInefficiencyFactor: f64;
    pub HostVMInefficiencyFactorPrefetch: f64;
    pub tdlut_pte_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_groups_per_2row_ub: [u32; DML2_MAX_PLANES];
    pub tdlut_opt_time: [f64; DML2_MAX_PLANES];
    pub tdlut_drain_time: [f64; DML2_MAX_PLANES];
    pub tdlut_bytes_per_group: [u32; DML2_MAX_PLANES];
    pub Tvm_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub immediate_flip_required: bool; // any pipes need immediate flip
    pub SOCCLK: f64; /// <brief Basically just the clock freq at the min (or given) state
    pub TotalWRBandwidth: f64;
    pub max_urgent_latency_us: f64;
    pub df_response_time_us: f64;

	dml2_pstate_method uclk_pstate_switch_modes: [DML2_MAX_PLANES];
    pub svp0_max_bytes: u32;
    pub svp1_max_bytes: u32;
    pub svp0_max_bytes_per_dpp: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp0_max_bytes_per_dpp_c: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp1_max_bytes_per_dpp: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp1_max_bytes_per_dpp_c: [u32; DML2_MAX_PLANES]; // max bytes for any DPP under a given plane
    pub svp0_dst_lines: [u32; DML2_MAX_PLANES]; // per stream
    pub svp1_dst_lines: [u32; DML2_MAX_PLANES]; // per stream
    pub min_lead_dst_lines: [u32; DML2_MAX_PLANES]; // per stream, should be max(nominal_req_limit, vstartup_to_vactive). Does not have to be maxed over all planes
    pub svp_req_limit: [u32; DML2_MAX_PLANES]; // per stream, should be the same value in time between all streams max(2 swaths, dst_y_pre) over all planes
    pub nom_req_limit_alt: [u32; DML2_MAX_PLANES]; // per stream
    pub total_swaths: [u32; DML2_MAX_PLANES];
    pub total_swaths_c: [u32; DML2_MAX_PLANES];
    pub prefetch_swaths: [u32; DML2_MAX_PLANES];
    pub prefetch_swaths_c: [u32; DML2_MAX_PLANES];
    pub prefetch_hdl_delta: [f64; DML2_MAX_PLANES]; // in dst
    pub recout_hdl_delta: [f64; DML2_MAX_PLANES]; // in dst
    pub prefetch_hdl_delta_c: [f64; DML2_MAX_PLANES]; // in dst
    pub recout_hdl_delta_c: [f64; DML2_MAX_PLANES]; // in dst
    pub max_prefetch_in_lines: [u32; DML2_MAX_PLANES]; // max prefetch time over all planes converted to lines for the given stream
	// -------------------
	// Output
	// -------------------
    pub pipe_plane: [u32; DML2_MAX_PLANES]; // <brief used mainly by dv to map the pipe inst to plane index within DML core; the plane idx of a pipe
    pub num_active_pipes: u32;

    pub NoTimeToPrefetch: [bool; DML2_MAX_PLANES]; // <brief Prefetch schedule calculation result

	// Support
    pub UrgVactiveBandwidthSupport: bool;
    pub PrefetchScheduleSupported: bool;
    pub UrgentBandwidthSupport: bool;
    pub PrefetchModeSupported: bool; // <brief Is the prefetch mode (bandwidth and latency) supported
    pub ImmediateFlipSupported: bool;
    pub ImmediateFlipSupportedForPipe: [bool; DML2_MAX_PLANES];
    pub dcfclk_support: bool;

	// Clock
    pub Dcfclk: f64;
    pub Dispclk: f64; // <brief dispclk being used in mode programming
    pub Dppclk: [f64; DML2_MAX_PLANES]; // <brief dppclk being used in mode programming
    pub GlobalDPPCLK: f64;

    pub DSCCLK: [f64; DML2_MAX_PLANES]; //< brief Required DSCCLK freq. Backend; not used in any subsequent calculations for now
    pub DCFCLKDeepSleep: f64;

	// ARB reg
    pub DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE: bool;
    pub Watermark: dml2_core_internal_watermarks;

	// DCC compression control
    pub request_size_bytes_luma: [u32; DML2_MAX_PLANES];
    pub request_size_bytes_chroma: [u32; DML2_MAX_PLANES];
	dml2_core_internal_request_type RequestLuma: [DML2_MAX_PLANES];
	dml2_core_internal_request_type RequestChroma: [DML2_MAX_PLANES];
    pub DCCYMaxUncompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCYMaxCompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCYIndependentBlock: [u32; DML2_MAX_PLANES];
    pub DCCCMaxUncompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCCMaxCompressedBlock: [u32; DML2_MAX_PLANES];
    pub DCCCIndependentBlock: [u32; DML2_MAX_PLANES];

	// Stutter Efficiency
    pub StutterEfficiency: f64;
    pub StutterEfficiencyNotIncludingVBlank: f64;
    pub NumberOfStutterBurstsPerFrame: u32;
    pub Z8StutterEfficiency: f64;
    pub Z8NumberOfStutterBurstsPerFrame: u32;
    pub Z8StutterEfficiencyNotIncludingVBlank: f64;
    pub LowPowerStutterEfficiency: f64;
    pub LowPowerStutterEfficiencyNotIncludingVBlank: f64;
    pub LowPowerNumberOfStutterBurstsPerFrame: u32;
    pub StutterPeriod: f64;
    pub Z8StutterEfficiencyBestCase: f64;
    pub Z8NumberOfStutterBurstsPerFrameBestCase: u32;
    pub Z8StutterEfficiencyNotIncludingVBlankBestCase: f64;
    pub StutterPeriodBestCase: f64;

	// DLG TTU reg
    pub MIN_DST_Y_NEXT_START: [f64; DML2_MAX_PLANES];
    pub VREADY_AT_OR_AFTER_VSYNC: [bool; DML2_MAX_PLANES];
    pub DSTYAfterScaler: [u32; DML2_MAX_PLANES];
    pub DSTXAfterScaler: [u32; DML2_MAX_PLANES];
    pub dst_y_prefetch: [f64; DML2_MAX_PLANES];
    pub dst_y_per_vm_vblank: [f64; DML2_MAX_PLANES];
    pub dst_y_per_row_vblank: [f64; DML2_MAX_PLANES];
    pub dst_y_per_vm_flip: [f64; DML2_MAX_PLANES];
    pub dst_y_per_row_flip: [f64; DML2_MAX_PLANES];
    pub MinTTUVBlank: [f64; DML2_MAX_PLANES];
    pub DisplayPipeLineDeliveryTimeLuma: [f64; DML2_MAX_PLANES];
    pub DisplayPipeLineDeliveryTimeChroma: [f64; DML2_MAX_PLANES];
    pub DisplayPipeLineDeliveryTimeLumaPrefetch: [f64; DML2_MAX_PLANES];
    pub DisplayPipeLineDeliveryTimeChromaPrefetch: [f64; DML2_MAX_PLANES];
    pub DisplayPipeRequestDeliveryTimeLuma: [f64; DML2_MAX_PLANES];
    pub DisplayPipeRequestDeliveryTimeChroma: [f64; DML2_MAX_PLANES];
    pub DisplayPipeRequestDeliveryTimeLumaPrefetch: [f64; DML2_MAX_PLANES];
    pub DisplayPipeRequestDeliveryTimeChromaPrefetch: [f64; DML2_MAX_PLANES];
    pub CursorDstXOffset: [u32; DML2_MAX_PLANES];
    pub CursorDstYOffset: [u32; DML2_MAX_PLANES];
    pub CursorChunkHDLAdjust: [u32; DML2_MAX_PLANES];

    pub DST_Y_PER_PTE_ROW_NOM_L: [f64; DML2_MAX_PLANES];
    pub DST_Y_PER_PTE_ROW_NOM_C: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_nom_luma: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_nom_chroma: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_vblank_luma: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_vblank_chroma: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_flip_luma: [f64; DML2_MAX_PLANES];
    pub time_per_pte_group_flip_chroma: [f64; DML2_MAX_PLANES];
    pub TimePerVMGroupVBlank: [f64; DML2_MAX_PLANES];
    pub TimePerVMGroupFlip: [f64; DML2_MAX_PLANES];
    pub TimePerVMRequestVBlank: [f64; DML2_MAX_PLANES];
    pub TimePerVMRequestFlip: [f64; DML2_MAX_PLANES];

    pub DST_Y_PER_META_ROW_NOM_L: [f64; DML2_MAX_PLANES];
    pub DST_Y_PER_META_ROW_NOM_C: [f64; DML2_MAX_PLANES];
    pub TimePerMetaChunkNominal: [f64; DML2_MAX_PLANES];
    pub TimePerChromaMetaChunkNominal: [f64; DML2_MAX_PLANES];
    pub TimePerMetaChunkVBlank: [f64; DML2_MAX_PLANES];
    pub TimePerChromaMetaChunkVBlank: [f64; DML2_MAX_PLANES];
    pub TimePerMetaChunkFlip: [f64; DML2_MAX_PLANES];
    pub TimePerChromaMetaChunkFlip: [f64; DML2_MAX_PLANES];

    pub FractionOfUrgentBandwidth: f64;
    pub FractionOfUrgentBandwidthImmediateFlip: f64;
    pub FractionOfUrgentBandwidthMALL: f64;

	// RQ registers
    pub PTE_BUFFER_MODE: [bool; DML2_MAX_PLANES];
    pub BIGK_FRAGMENT_SIZE: [u32; DML2_MAX_PLANES];
    pub VActiveLatencyHidingUs: [f64; DML2_MAX_PLANES];
    pub SubViewportLinesNeededInMALL: [u32; DML2_MAX_PLANES];
    pub is_using_mall_for_ss: [bool; DML2_MAX_PLANES];

	// OTG
    pub VStartupMin: [u32; DML2_MAX_PLANES]; /// <brief Minimum vstartup to meet the prefetch schedule (i.e. the prefetch solution can be found at this vstartup time); not the actual global sync vstartup pos.
    pub VStartup: [u32; DML2_MAX_PLANES]; /// <brief The vstartup value for OTG programming (will set to max vstartup; but now bounded by min(vblank_nom. actual vblank))
    pub VUpdateOffsetPix: [u32; DML2_MAX_PLANES];
    pub VUpdateWidthPix: [u32; DML2_MAX_PLANES];
    pub VReadyOffsetPix: [u32; DML2_MAX_PLANES];
    pub pstate_keepout_dst_lines: [u32; DML2_MAX_PLANES];

	// Latency and Support
    pub MaxActiveFCLKChangeLatencySupported: f64;
    pub USRRetrainingSupport: bool;
    pub g6_temp_read_support: bool;
	dml2_pstate_change_support FCLKChangeSupport: [DML2_MAX_PLANES];
	dml2_pstate_change_support DRAMClockChangeSupport: [DML2_MAX_PLANES];
	dml2_pstate_change_support temp_read_or_ppt_support: [DML2_MAX_PLANES];
    pub global_dram_clock_change_supported: bool;
    pub global_fclk_change_supported: bool;
    pub global_temp_read_or_ppt_supported: bool;
    pub MaxActiveDRAMClockChangeLatencySupported: [f64; DML2_MAX_PLANES];
    pub WritebackAllowFCLKChangeEndPosition: [f64; DML2_MAX_PLANES];
    pub WritebackAllowDRAMClockChangeEndPosition: [f64; DML2_MAX_PLANES];

	// buffer sizing
    pub DETBufferSizeInKByte: [u32; DML2_MAX_PLANES]; // <brief Recommended DET size configuration for this plane. All pipes under this plane should program the DET buffer size to the calculated value.
    pub DETBufferSizeY: [u32; DML2_MAX_PLANES];
    pub DETBufferSizeC: [u32; DML2_MAX_PLANES];
    pub SwathHeightY: [u32; DML2_MAX_PLANES];
    pub SwathHeightC: [u32; DML2_MAX_PLANES];

	double urg_vactive_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // active bandwidth, scaled by urg burst factor
	double urg_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth, scaled by urg burst factor
	double urg_bandwidth_required_qual: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth, scaled by urg burst factor, use qual_row_bw
	double urg_bandwidth_required_flip: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // include vm, prefetch, active bandwidth + flip
	double non_urg_bandwidth_required: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]; // same as urg_bandwidth, except not scaled by urg burst factor
	double non_urg_bandwidth_required_flip: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];

    pub avg_bandwidth_available_min: [f64; dml2_core_internal_soc_state_max];
	double avg_bandwidth_available: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
    pub urg_bandwidth_available_min: [f64; dml2_core_internal_soc_state_max]; // min between SDP and DRAM
	double urg_bandwidth_available: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
    pub urg_bandwidth_available_vm_only: [f64; dml2_core_internal_soc_state_max]; // the min of sdp bw and dram_vm_only bw, sdp has no different derate for vm/non-vm traffic etc.
    pub urg_bandwidth_available_pixel_and_vm: [f64; dml2_core_internal_soc_state_max]; // the min of sdp bw and dram_pixel_and_vm bw, sdp has no different derate for vm/non-vm etc.

    pub dcc_dram_bw_nom_overhead_factor_p0: [f64; DML2_MAX_PLANES];
    pub dcc_dram_bw_nom_overhead_factor_p1: [f64; DML2_MAX_PLANES];
    pub dcc_dram_bw_pref_overhead_factor_p0: [f64; DML2_MAX_PLANES];
    pub dcc_dram_bw_pref_overhead_factor_p1: [f64; DML2_MAX_PLANES];
    pub mall_prefetch_sdp_overhead_factor: [f64; DML2_MAX_PLANES];
    pub mall_prefetch_dram_overhead_factor: [f64; DML2_MAX_PLANES];

    pub num_mcaches_l: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_l: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_per_channel_l: [u32; DML2_MAX_PLANES];
	unsigned int mcache_offsets_l: [DML2_MAX_PLANES]: [DML2_MAX_MCACHES + 1];
    pub mcache_shift_granularity_l: [u32; DML2_MAX_PLANES];

    pub num_mcaches_c: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_c: [u32; DML2_MAX_PLANES];
    pub mcache_row_bytes_per_channel_c: [u32; DML2_MAX_PLANES];
	unsigned int mcache_offsets_c: [DML2_MAX_PLANES]: [DML2_MAX_MCACHES + 1];
    pub mcache_shift_granularity_c: [u32; DML2_MAX_PLANES];

    pub mall_comb_mcache_l: [bool; DML2_MAX_PLANES];
    pub mall_comb_mcache_c: [bool; DML2_MAX_PLANES];
    pub lc_comb_mcache: [bool; DML2_MAX_PLANES];

    pub impacted_prefetch_margin_us: [f64; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_internal_SOCParametersList {
    pub UrgentLatency: f64;
    pub ExtraLatency_sr: f64;
    pub ExtraLatency: f64;
    pub WritebackLatency: f64;
    pub DRAMClockChangeLatency: f64;
    pub FCLKChangeLatency: f64;
    pub SRExitTime: f64;
    pub SREnterPlusExitTime: f64;
    pub SRExitTimeLowPower: f64;
    pub SREnterPlusExitTimeLowPower: f64;
    pub SRExitZ8Time: f64;
    pub SREnterPlusExitZ8Time: f64;
    pub USRRetrainingLatency: f64;
    pub SMNLatency: f64;
    pub g6_temp_read_blackout_us: f64;
    pub temp_read_or_ppt_blackout_us: f64;
    pub max_urgent_latency_us: f64;
    pub df_response_time_us: f64;
    pub qos_type: dml2_qos_param_type;
}

#: [repr(C)]
pub struct dml2_core_calcs_mode_support_locals {
    pub PixelClockBackEnd: [f64; DML2_MAX_PLANES];
    pub OutputBpp: [f64; DML2_MAX_PLANES];

    pub meta_row_height_luma: [u32; DML2_MAX_PLANES];
    pub meta_row_height_chroma: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_c: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_l: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_c: [u32; DML2_MAX_PLANES];

    pub dummy_boolean: [bool; 3];
    pub dummy_integer: [u32; 3];
	unsigned int dummy_integer_array: [36]: [DML2_MAX_PLANES];
	dml2_odm_mode dummy_odm_mode: [DML2_MAX_PLANES];
	bool dummy_boolean_array: [2]: [DML2_MAX_PLANES];
    pub dummy_single: [f64; 3];
    pub dummy_single_array: [f64; DML2_MAX_PLANES];
	double dummy_double_array: [3]: [DML2_MAX_PLANES];
	dml2_pstate_method dummy_pstate_method_array: [DML2_MAX_PLANES];
    pub dummy_watermark: dml2_core_internal_watermarks;
	double dummy_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
	double surface_dummy_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];

    pub MaximumVStartup: [u32; DML2_MAX_PLANES];
    pub DSTYAfterScaler: [u32; DML2_MAX_PLANES];
    pub DSTXAfterScaler: [u32; DML2_MAX_PLANES];
    pub mSOCParameters: dml2_core_internal_SOCParametersList;
    pub myPipe: dml2_core_internal_DmlPipe;
	struct dml2_core_internal_DmlPipe SurfParameters: [DML2_MAX_PLANES];
    pub TotalNumberOfActiveWriteback: u32;
    pub MaximumSwathWidthSupportLuma: u32;
    pub MaximumSwathWidthSupportChroma: u32;
    pub MPCCombineMethodAsNeededForPStateChangeAndVoltage: bool;
    pub MPCCombineMethodAsPossible: bool;
    pub TotalAvailablePipesSupportNoDSC: bool;
    pub NumberOfDPPNoDSC: u32;
    pub ODMModeNoDSC: dml2_odm_mode;
    pub RequiredDISPCLKPerSurfaceNoDSC: f64;
    pub TotalAvailablePipesSupportDSC: bool;
    pub NumberOfDPPDSC: u32;
    pub ODMModeDSC: dml2_odm_mode;
    pub RequiredDISPCLKPerSurfaceDSC: f64;
    pub BWOfNonCombinedSurfaceOfMaximumBandwidth: f64;
    pub NumberOfNonCombinedSurfaceOfMaximumBandwidth: u32;
    pub TotalNumberOfActiveOTG: u32;
    pub TotalNumberOfActiveHDMIFRL: u32;
    pub TotalNumberOfActiveDP2p0: u32;
    pub TotalNumberOfActiveDP2p0Outputs: u32;
    pub TotalSlots: u32;
    pub DSCFormatFactor: u32;
    pub TotalDSCUnitsRequired: u32;
    pub ReorderingBytes: u32;
    pub ImmediateFlipRequired: bool;
    pub FullFrameMALLPStateMethod: bool;
    pub SubViewportMALLPStateMethod: bool;
    pub PhantomPipeMALLPStateMethod: bool;
    pub SubViewportMALLRefreshGreaterThan120Hz: bool;

    pub HostVMInefficiencyFactor: f64;
    pub HostVMInefficiencyFactorPrefetch: f64;
    pub MaxVStartup: u32;
    pub PixelClockBackEndFactor: f64;
    pub NumDSCUnitRequired: u32;

    pub Tvm_trips: [f64; DML2_MAX_PLANES];
    pub Tr0_trips: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub per_pipe_flip_bytes: [u32; DML2_MAX_PLANES];

    pub vmpg_width_y: [u32; DML2_MAX_PLANES];
    pub vmpg_height_y: [u32; DML2_MAX_PLANES];
    pub vmpg_width_c: [u32; DML2_MAX_PLANES];
    pub vmpg_height_c: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_l: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_c: [u32; DML2_MAX_PLANES];

    pub tdlut_pte_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_row_bytes: [u32; DML2_MAX_PLANES];
    pub tdlut_groups_per_2row_ub: [u32; DML2_MAX_PLANES];
    pub tdlut_opt_time: [f64; DML2_MAX_PLANES];
    pub tdlut_drain_time: [f64; DML2_MAX_PLANES];
    pub tdlut_bytes_to_deliver: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_group: [u32; DML2_MAX_PLANES];

    pub cursor_bytes_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes_per_line: [u32; DML2_MAX_PLANES];
    pub cursor_lines_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes: [u32; DML2_MAX_PLANES];
    pub stream_visited: [bool; DML2_MAX_PLANES];

	unsigned int pstate_bytes_required_l: [dml2_pstate_type_count]: [DML2_MAX_PLANES];
	unsigned int pstate_bytes_required_c: [dml2_pstate_type_count]: [DML2_MAX_PLANES];

    pub prefetch_sw_bytes: [f64; DML2_MAX_PLANES];
    pub Tpre_rounded: [f64; DML2_MAX_PLANES];
    pub Tpre_oto: [f64; DML2_MAX_PLANES];
    pub recalc_prefetch_schedule: bool;
    pub recalc_prefetch_done: bool;
    pub impacted_dst_y_pre: [f64; DML2_MAX_PLANES];
    pub line_times: [f64; DML2_MAX_PLANES];
	dml2_source_format_class pixel_format: [DML2_MAX_PLANES];
    pub lb_source_lines_l: [u32; DML2_MAX_PLANES];
    pub lb_source_lines_c: [u32; DML2_MAX_PLANES];
    pub prefetch_swath_time_us: [f64; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_calcs_mode_programming_locals {
    pub PixelClockBackEnd: [f64; DML2_MAX_PLANES];
    pub OutputBpp: [f64; DML2_MAX_PLANES];
    pub num_active_planes: u32; // <brief As determined by either e2e_pipe_param or display_cfg
    pub MaxTotalDETInKByte: u32;
    pub NomDETInKByte: u32;
    pub MinCompressedBufferSizeInKByte: u32;
    pub SOCCLK: f64; /// <brief Basically just the clock freq at the min (or given) state

	double dummy_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max];
	double surface_dummy_bw: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];
	double surface_dummy_bw0: [dml2_core_internal_soc_state_max]: [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];
	unsigned int dummy_integer_array: [4]: [DML2_MAX_PLANES];
	dml2_output_encoder_class dummy_output_encoder_array: [DML2_MAX_PLANES];
	double dummy_single_array: [2]: [DML2_MAX_PLANES];
	unsigned int dummy_long_array: [8]: [DML2_MAX_PLANES];
	bool dummy_boolean_array: [2]: [DML2_MAX_PLANES];
    pub dummy_boolean: [bool; 2];
    pub dummy_single: [f64; 2];
    pub dummy_watermark: dml2_core_internal_watermarks;

    pub DSCFormatFactor: u32;
	struct dml2_core_internal_DmlPipe SurfaceParameters: [DML2_MAX_PLANES];
    pub ReorderingBytes: u32;
    pub HostVMInefficiencyFactor: f64;
    pub HostVMInefficiencyFactorPrefetch: f64;
    pub TotalDCCActiveDPP: u32;
    pub TotalActiveDPP: u32;
    pub Total3dlutActive: u32;
    pub MaxVStartupLines: [u32; DML2_MAX_PLANES]; /// <brief more like vblank for the plane's OTG
    pub immediate_flip_required: bool; // any pipes need immediate flip
    pub DestinationLineTimesForPrefetchLessThan2: bool;
    pub VRatioPrefetchMoreThanMax: bool;
    pub MaxTotalRDBandwidthNotIncludingMALLPrefetch: f64;
    pub mmSOCParameters: dml2_core_internal_SOCParametersList;
    pub Tvstartup_margin: f64;
    pub dlg_vblank_start: f64;
    pub LSetup: f64;
    pub blank_lines_remaining: f64;
    pub WRBandwidth: f64;
    pub myPipe: dml2_core_internal_DmlPipe;
    pub PixelClockBackEndFactor: f64;
    pub vmpg_width_y: [u32; DML2_MAX_PLANES];
    pub vmpg_height_y: [u32; DML2_MAX_PLANES];
    pub vmpg_width_c: [u32; DML2_MAX_PLANES];
    pub vmpg_height_c: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_l: [u32; DML2_MAX_PLANES];
    pub full_swath_bytes_c: [u32; DML2_MAX_PLANES];

    pub meta_row_bytes_per_row_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_c: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_l: [u32; DML2_MAX_PLANES];
    pub dpte_row_bytes_per_row_c: [u32; DML2_MAX_PLANES];

    pub tdlut_pte_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_frame: [u32; DML2_MAX_PLANES];
    pub tdlut_row_bytes: [u32; DML2_MAX_PLANES];
    pub tdlut_groups_per_2row_ub: [u32; DML2_MAX_PLANES];
    pub tdlut_opt_time: [f64; DML2_MAX_PLANES];
    pub tdlut_drain_time: [f64; DML2_MAX_PLANES];
    pub tdlut_bytes_to_deliver: [u32; DML2_MAX_PLANES];
    pub tdlut_bytes_per_group: [u32; DML2_MAX_PLANES];

    pub cursor_bytes_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes_per_line: [u32; DML2_MAX_PLANES];
    pub cursor_lines_per_chunk: [u32; DML2_MAX_PLANES];
    pub cursor_bytes: [u32; DML2_MAX_PLANES];

    pub Tvm_trips: [f64; DML2_MAX_PLANES];
    pub Tr0_trips: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip: [f64; DML2_MAX_PLANES];
    pub Tvm_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub Tr0_trips_flip_rounded: [f64; DML2_MAX_PLANES];
    pub per_pipe_flip_bytes: [u32; DML2_MAX_PLANES];

	unsigned int pstate_bytes_required_l: [dml2_pstate_type_count]: [DML2_MAX_PLANES];
	unsigned int pstate_bytes_required_c: [dml2_pstate_type_count]: [DML2_MAX_PLANES];

    pub prefetch_sw_bytes: [f64; DML2_MAX_PLANES];
    pub Tpre_rounded: [f64; DML2_MAX_PLANES];
    pub Tpre_oto: [f64; DML2_MAX_PLANES];
    pub recalc_prefetch_schedule: bool;
    pub impacted_dst_y_pre: [f64; DML2_MAX_PLANES];
    pub line_times: [f64; DML2_MAX_PLANES];
	dml2_source_format_class pixel_format: [DML2_MAX_PLANES];
    pub lb_source_lines_l: [u32; DML2_MAX_PLANES];
    pub lb_source_lines_c: [u32; DML2_MAX_PLANES];
    pub num_dsc_slices: [u32; DML2_MAX_PLANES];
    pub dsc_enable: [bool; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals {
    pub ActiveDRAMClockChangeLatencyMargin: [f64; DML2_MAX_PLANES];
    pub ActiveFCLKChangeLatencyMargin: [f64; DML2_MAX_PLANES];
    pub USRRetrainingLatencyMargin: [f64; DML2_MAX_PLANES];
    pub g6_temp_read_latency_margin: [f64; DML2_MAX_PLANES];
    pub temp_read_or_ppt_latency_margin: [f64; DML2_MAX_PLANES];

    pub EffectiveLBLatencyHidingY: f64;
    pub EffectiveLBLatencyHidingC: f64;
    pub LinesInDETY: [f64; DML2_MAX_PLANES];
    pub LinesInDETC: [f64; DML2_MAX_PLANES];
    pub LinesInDETYRoundedDownToSwath: [u32; DML2_MAX_PLANES];
    pub LinesInDETCRoundedDownToSwath: [u32; DML2_MAX_PLANES];
    pub FullDETBufferingTimeY: f64;
    pub FullDETBufferingTimeC: f64;
    pub WritebackDRAMClockChangeLatencyMargin: f64;
    pub WritebackFCLKChangeLatencyMargin: f64;
    pub WritebackTempReadOrPptLatencyMargin: f64;
    pub WritebackLatencyHiding: f64;

    pub TotalActiveWriteback: u32;
    pub LBLatencyHidingSourceLinesY: [u32; DML2_MAX_PLANES];
    pub LBLatencyHidingSourceLinesC: [u32; DML2_MAX_PLANES];
    pub TotalPixelBW: f64;
    pub EffectiveDETBufferSizeY: f64;
    pub ActiveClockChangeLatencyHidingY: f64;
    pub ActiveClockChangeLatencyHidingC: f64;
    pub ActiveClockChangeLatencyHiding: f64;
    pub peak_vactive_p_vblank_latency_hiding_us: f64;
    pub dst_y_pstate: u32;
    pub src_y_pstate_l: u32;
    pub src_y_pstate_c: u32;
    pub src_y_ahead_l: u32;
    pub src_y_ahead_c: u32;
    pub sub_vp_lines_l: u32;
    pub sub_vp_lines_c: u32;

}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateVMRowAndSwath_locals {
    pub PTEBufferSizeInRequestsForLuma: [u32; DML2_MAX_PLANES];
    pub PTEBufferSizeInRequestsForChroma: [u32; DML2_MAX_PLANES];
    pub vm_bytes_l: u32;
    pub vm_bytes_c: u32;
    pub PixelPTEBytesPerRowY: [u32; DML2_MAX_PLANES];
    pub PixelPTEBytesPerRowC: [u32; DML2_MAX_PLANES];
    pub PixelPTEBytesPerRowStorageY: [u32; DML2_MAX_PLANES];
    pub PixelPTEBytesPerRowStorageC: [u32; DML2_MAX_PLANES];
    pub PixelPTEBytesPerRowY_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub PixelPTEBytesPerRowC_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_luma_ub_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_luma_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub dpte_row_width_chroma_ub_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub dpte_row_height_chroma_one_row_per_frame: [u32; DML2_MAX_PLANES];
    pub one_row_per_frame_fits_in_buffer: [bool; DML2_MAX_PLANES];
    pub HostVMDynamicLevels: u32;
    pub meta_row_bytes_per_row_ub_l: [u32; DML2_MAX_PLANES];
    pub meta_row_bytes_per_row_ub_c: [u32; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateVMRowAndSwath_params {
    pub display_cfg: *const dml2_display_cfg;
    pub NumberOfActiveSurfaces: u32;
    pub myPipe: *mut dml2_core_internal_DmlPipe;
	unsigned int *SurfaceSizeInMALL;
    pub PTEBufferSizeInRequestsLuma: u32;
    pub PTEBufferSizeInRequestsChroma: u32;
    pub MALLAllocatedForDCN: u32;
	unsigned int *SwathWidthY;
	unsigned int *SwathWidthC;
    pub HostVMMinPageSize: u32;
    pub DCCMetaBufferSizeBytes: u32;
    pub mrq_present: bool;
	dml2_pstate_method *uclk_pstate_switch_modes;

	// Output
    pub PTEBufferSizeNotExceeded: *mut bool;
    pub DCCMetaBufferSizeNotExceeded: *mut bool;

	unsigned int *dpte_row_width_luma_ub;
	unsigned int *dpte_row_width_chroma_ub;
	unsigned int *dpte_row_height_luma;
	unsigned int *dpte_row_height_chroma;
	unsigned int *dpte_row_height_linear_luma; // VBA_DELTA
	unsigned int *dpte_row_height_linear_chroma; // VBA_DELTA

	unsigned int *vm_group_bytes;
	unsigned int *dpte_group_bytes;
	unsigned int *PixelPTEReqWidthY;
	unsigned int *PixelPTEReqHeightY;
	unsigned int *PTERequestSizeY;
	unsigned int *vmpg_width_y;
	unsigned int *vmpg_height_y;

	unsigned int *PixelPTEReqWidthC;
	unsigned int *PixelPTEReqHeightC;
	unsigned int *PTERequestSizeC;
	unsigned int *vmpg_width_c;
	unsigned int *vmpg_height_c;

	unsigned int *dpde0_bytes_per_frame_ub_l;
	unsigned int *dpde0_bytes_per_frame_ub_c;

	unsigned int *PrefetchSourceLinesY;
	unsigned int *PrefetchSourceLinesC;
	unsigned int *VInitPreFillY;
	unsigned int *VInitPreFillC;
	unsigned int *MaxNumSwathY;
	unsigned int *MaxNumSwathC;
    pub dpte_row_bw: *mut double;
	unsigned int *PixelPTEBytesPerRow;
	unsigned int *dpte_row_bytes_per_row_l;
	unsigned int *dpte_row_bytes_per_row_c;
	unsigned int *vm_bytes;
    pub use_one_row_for_frame: *mut bool;
    pub use_one_row_for_frame_flip: *mut bool;
    pub is_using_mall_for_ss: *mut bool;
    pub PTE_BUFFER_MODE: *mut bool;
	unsigned int *BIGK_FRAGMENT_SIZE;

	// MRQ
	unsigned int *meta_req_width_luma;
	unsigned int *meta_req_height_luma;
	unsigned int *meta_row_width_luma;
	unsigned int *meta_row_height_luma;
	unsigned int *meta_pte_bytes_per_frame_ub_l;

	unsigned int *meta_req_width_chroma;
	unsigned int *meta_req_height_chroma;
	unsigned int *meta_row_width_chroma;
	unsigned int *meta_row_height_chroma;
	unsigned int *meta_pte_bytes_per_frame_ub_c;
    pub meta_row_bw: *mut double;
	unsigned int *meta_row_bytes;
	unsigned int *meta_row_bytes_per_row_ub_l;
	unsigned int *meta_row_bytes_per_row_ub_c;
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculatePrefetchSchedule_locals {
    pub NoTimeToPrefetch: bool;
    pub DPPCycles: u32;
    pub DISPCLKCycles: u32;
    pub DSTTotalPixelsAfterScaler: f64;
    pub LineTime: f64;
    pub dst_y_prefetch_equ: f64;
    pub prefetch_bw_oto: f64;
    pub per_pipe_vactive_sw_bw: f64;
    pub Tvm_oto: f64;
    pub Tr0_oto: f64;
    pub Tvm_oto_lines: f64;
    pub Tr0_oto_lines: f64;
    pub dst_y_prefetch_oto: f64;
    pub TimeForFetchingVM: f64;
    pub TimeForFetchingRowInVBlank: f64;
    pub LinesToRequestPrefetchPixelData: f64;
    pub HostVMDynamicLevelsTrips: u32;
    pub trip_to_mem: f64;
    pub Tvm_trips_rounded: f64;
    pub Tr0_trips_rounded: f64;
    pub max_Tsw: f64;
    pub Lsw_oto: f64;
    pub prefetch_bw_equ: f64;
    pub Tvm_equ: f64;
    pub Tr0_equ: f64;
    pub Tdmbf: f64;
    pub Tdmec: f64;
    pub Tdmsks: f64;
    pub total_row_bytes: f64;
    pub prefetch_bw_pr: f64;
    pub bytes_pp: f64;
    pub dep_bytes: f64;
    pub min_Lsw_oto: f64;
    pub min_Lsw_equ: f64;
    pub Tsw_est1: f64;
    pub Tsw_est2: f64;
    pub Tsw_est3: f64;
    pub prefetch_bw1: f64;
    pub prefetch_bw2: f64;
    pub prefetch_bw3: f64;
    pub prefetch_bw4: f64;
    pub dst_y_prefetch_equ_impacted: f64;

    pub TWait_p: f64;
    pub cursor_prefetch_bytes: u32;
}

#: [repr(C)]
pub struct dml2_core_shared_calculate_det_buffer_size_params {
    pub display_cfg: *const dml2_display_cfg;
    pub ForceSingleDPP: bool;
    pub NumberOfActiveSurfaces: u32;
    pub UnboundedRequestEnabled: bool;
    pub nomDETInKByte: u32;
    pub MaxTotalDETInKByte: u32;
    pub ConfigReturnBufferSizeInKByte: u32;
    pub MinCompressedBufferSizeInKByte: u32;
    pub ConfigReturnBufferSegmentSizeInkByte: u32;
    pub CompressedBufferSegmentSizeInkByte: u32;
    pub ReadBandwidthLuma: *mut double;
    pub ReadBandwidthChroma: *mut double;
	unsigned int *full_swath_bytes_l;
	unsigned int *full_swath_bytes_c;
	unsigned int *swath_time_value_us;
	unsigned int *DPPPerSurface;
    pub TryToAllocateForWriteLatency: bool;
    pub bestEffortMinActiveLatencyHidingUs: u32;

	// Output
	unsigned int *DETBufferSizeInKByte;
	unsigned int *CompressedBufferSizeInkByte;
}

#: [repr(C)]
pub struct dml2_core_shared_calculate_vm_and_row_bytes_params {
    pub ViewportStationary: bool;
    pub DCCEnable: bool;
    pub NumberOfDPPs: u32;
    pub BlockHeight256Bytes: u32;
    pub BlockWidth256Bytes: u32;
    pub SourcePixelFormat: dml2_source_format_class;
    pub SurfaceTiling: u32;
    pub BytePerPixel: u32;
    pub RotationAngle: dml2_rotation_angle;
    pub SwathWidth: u32; // per pipe
    pub ViewportHeight: u32;
    pub ViewportXStart: u32;
    pub ViewportYStart: u32;
    pub GPUVMEnable: bool;
    pub GPUVMMaxPageTableLevels: u32;
    pub GPUVMMinPageSizeKBytes: u32;
    pub PTEBufferSizeInRequests: u32;
    pub Pitch: u32;
    pub MacroTileWidth: u32;
    pub MacroTileHeight: u32;
    pub is_phantom: bool;
    pub DCCMetaPitch: u32;
    pub mrq_present: bool;

	// Output
	unsigned int *PixelPTEBytesPerRow; // for bandwidth calculation
	unsigned int *PixelPTEBytesPerRowStorage; // for PTE buffer size check
	unsigned int *dpte_row_width_ub;
	unsigned int *dpte_row_height;
	unsigned int *dpte_row_height_linear;
	unsigned int *PixelPTEBytesPerRow_one_row_per_frame;
	unsigned int *dpte_row_width_ub_one_row_per_frame;
	unsigned int *dpte_row_height_one_row_per_frame;
	unsigned int *vmpg_width;
	unsigned int *vmpg_height;
	unsigned int *PixelPTEReqWidth;
	unsigned int *PixelPTEReqHeight;
	unsigned int *PTERequestSize;
	unsigned int *dpde0_bytes_per_frame_ub;

	unsigned int *meta_row_bytes;
	unsigned int *MetaRequestWidth;
	unsigned int *MetaRequestHeight;
	unsigned int *meta_row_width;
	unsigned int *meta_row_height;
	unsigned int *meta_pte_bytes_per_frame_ub;
}

#: [repr(C)]
pub struct dml2_core_shared_CalculateSwathAndDETConfiguration_locals {
    pub MaximumSwathHeightY: [u32; DML2_MAX_PLANES];
    pub MaximumSwathHeightC: [u32; DML2_MAX_PLANES];
    pub RoundedUpSwathSizeBytesY: [u32; DML2_MAX_PLANES];
    pub RoundedUpSwathSizeBytesC: [u32; DML2_MAX_PLANES];
    pub SwathWidthSingleDPP: [u32; DML2_MAX_PLANES];
    pub SwathWidthSingleDPPChroma: [u32; DML2_MAX_PLANES];
    pub SwathTimeValueUs: [u32; DML2_MAX_PLANES];

    pub calculate_det_buffer_size_params: dml2_core_shared_calculate_det_buffer_size_params;
}

#: [repr(C)]
pub struct dml2_core_shared_TruncToValidBPP_locals {
    pub hdmifrlparams: lib_frl_cap_check_params;
    pub hdmifrlinter: lib_frl_cap_check_intermediates;
}

#: [repr(C)]
pub struct dml2_core_shared_CalculateDETBufferSize_locals {
    pub DETBufferSizePoolInKByte: u32;
    pub NextDETBufferPieceInKByte: u32;
    pub NextSurfaceToAssignDETPiece: u32;
    pub TotalBandwidth: f64;
    pub BandwidthOfSurfacesNotAssignedDETPiece: f64;
    pub max_minDET: u32;
    pub minDET: u32;
    pub minDET_pipe: u32;
    pub TotalBandwidthPerStream: [u32; DML2_MAX_PLANES];
    pub TotalPixelRate: u32;
    pub DETBudgetPerStream: [u32; DML2_MAX_PLANES];
    pub RemainingDETBudgetPerStream: [u32; DML2_MAX_PLANES];
	unsigned int IdealDETBudget, DeltaDETBudget;
    pub ResidualDETAfterRounding: u32;
}

#: [repr(C)]
pub struct dml2_core_shared_get_urgent_bandwidth_required_locals {
    pub required_bandwidth_mbps: f64;
    pub required_bandwidth_mbps_this_surface: f64;
    pub adj_factor_p0: f64;
    pub adj_factor_p1: f64;
    pub adj_factor_cur: f64;
    pub adj_factor_p0_pre: f64;
    pub adj_factor_p1_pre: f64;
    pub adj_factor_cur_pre: f64;
    pub per_plane_flip_bw: [f64; DML2_MAX_PLANES];
    pub mall_svp_prefetch_factor: f64;
    pub tmp_nom_adj_factor_p0: f64;
    pub tmp_nom_adj_factor_p1: f64;
    pub tmp_pref_adj_factor_p0: f64;
    pub tmp_pref_adj_factor_p1: f64;
    pub vm_row_bw: f64;
    pub flip_and_active_bw: f64;
    pub flip_and_prefetch_bw: f64;
    pub flip_and_prefetch_bw_max: f64;
    pub active_and_excess_bw: f64;
}

#: [repr(C)]
pub struct dml2_core_shared_calculate_peak_bandwidth_required_locals {
    pub unity_array: [f64; DML2_MAX_PLANES];
    pub zero_array: [f64; DML2_MAX_PLANES];
    pub surface_dummy_bw: [f64; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_shared_CalculateFlipSchedule_locals {
    pub min_row_time: f64;
    pub Tvm_flip: f64;
    pub Tr0_flip: f64;
    pub ImmediateFlipBW: f64;
    pub dpte_row_bytes: f64;
    pub min_row_height: f64;
    pub min_row_height_chroma: f64;
    pub max_flip_time: f64;
    pub lb_flip_bw: f64;
    pub hvm_scaled_vm_bytes: f64;
    pub num_rows: f64;
    pub hvm_scaled_row_bytes: f64;
    pub hvm_scaled_vm_row_bytes: f64;
    pub dual_plane: bool;
}

#: [repr(C)]
pub struct dml2_core_shared_rq_dlg_get_dlg_reg_locals {
    pub plane_idx: u32;
    pub stream_idx: u32;
    pub source_format: dml2_source_format_class;
    pub timing: *const dml2_timing_cfg;
    pub dual_plane: bool;
    pub odm_mode: dml2_odm_mode;

    pub htotal: u32;
    pub hactive: u32;
    pub hblank_end: u32;
    pub vblank_end: u32;
    pub interlaced: bool;
    pub pclk_freq_in_mhz: f64;
    pub refclk_freq_in_mhz: f64;
    pub ref_freq_to_pix_freq: f64;

    pub num_active_pipes: u32;
    pub first_pipe_idx_in_plane: u32;
    pub pipe_idx_in_combine: u32;
    pub odm_combine_factor: u32;

    pub min_ttu_vblank: f64;
    pub min_dst_y_next_start: u32;

    pub vready_after_vcount0: u32;

    pub dst_x_after_scaler: u32;
    pub dst_y_after_scaler: u32;

    pub dst_y_prefetch: f64;
    pub dst_y_per_vm_vblank: f64;
    pub dst_y_per_row_vblank: f64;
    pub dst_y_per_vm_flip: f64;
    pub dst_y_per_row_flip: f64;

    pub max_dst_y_per_vm_vblank: f64;
    pub max_dst_y_per_row_vblank: f64;

    pub vratio_pre_l: f64;
    pub vratio_pre_c: f64;

    pub refcyc_per_line_delivery_pre_l: f64;
    pub refcyc_per_line_delivery_l: f64;

    pub refcyc_per_line_delivery_pre_c: f64;
    pub refcyc_per_line_delivery_c: f64;

    pub refcyc_per_req_delivery_pre_l: f64;
    pub refcyc_per_req_delivery_l: f64;

    pub refcyc_per_req_delivery_pre_c: f64;
    pub refcyc_per_req_delivery_c: f64;

    pub dst_y_per_pte_row_nom_l: f64;
    pub dst_y_per_pte_row_nom_c: f64;
    pub refcyc_per_pte_group_nom_l: f64;
    pub refcyc_per_pte_group_nom_c: f64;
    pub refcyc_per_pte_group_vblank_l: f64;
    pub refcyc_per_pte_group_vblank_c: f64;
    pub refcyc_per_pte_group_flip_l: f64;
    pub refcyc_per_pte_group_flip_c: f64;
    pub refcyc_per_tdlut_group: f64;

    pub dst_y_per_meta_row_nom_l: f64;
    pub dst_y_per_meta_row_nom_c: f64;
    pub refcyc_per_meta_chunk_nom_l: f64;
    pub refcyc_per_meta_chunk_nom_c: f64;
    pub refcyc_per_meta_chunk_vblank_l: f64;
    pub refcyc_per_meta_chunk_vblank_c: f64;
    pub refcyc_per_meta_chunk_flip_l: f64;
    pub refcyc_per_meta_chunk_flip_c: f64;
}

#: [repr(C)]
pub struct dml2_core_shared_CalculateMetaAndPTETimes_params {
    pub scratch: *mut dml2_core_internal_scratch;
    pub display_cfg: *const dml2_display_cfg;
    pub NumberOfActiveSurfaces: u32;
    pub use_one_row_for_frame: *mut bool;
    pub dst_y_per_row_vblank: *mut double;
    pub dst_y_per_row_flip: *mut double;
	unsigned int *BytePerPixelY;
	unsigned int *BytePerPixelC;
	unsigned int *dpte_row_height;
	unsigned int *dpte_row_height_chroma;
	unsigned int *dpte_group_bytes;
	unsigned int *PTERequestSizeY;
	unsigned int *PTERequestSizeC;
	unsigned int *PixelPTEReqWidthY;
	unsigned int *PixelPTEReqHeightY;
	unsigned int *PixelPTEReqWidthC;
	unsigned int *PixelPTEReqHeightC;
	unsigned int *dpte_row_width_luma_ub;
	unsigned int *dpte_row_width_chroma_ub;
	unsigned int *tdlut_groups_per_2row_ub;
    pub mrq_present: bool;
    pub MetaChunkSize: u32;
    pub MinMetaChunkSizeBytes: u32;
	unsigned int *meta_row_width;
	unsigned int *meta_row_width_chroma;
	unsigned int *meta_row_height;
	unsigned int *meta_row_height_chroma;
	unsigned int *meta_req_width;
	unsigned int *meta_req_width_chroma;
	unsigned int *meta_req_height;
	unsigned int *meta_req_height_chroma;

	// Output
    pub time_per_tdlut_group: *mut double;
    pub DST_Y_PER_PTE_ROW_NOM_L: *mut double;
    pub DST_Y_PER_PTE_ROW_NOM_C: *mut double;
    pub time_per_pte_group_nom_luma: *mut double;
    pub time_per_pte_group_vblank_luma: *mut double;
    pub time_per_pte_group_flip_luma: *mut double;
    pub time_per_pte_group_nom_chroma: *mut double;
    pub time_per_pte_group_vblank_chroma: *mut double;
    pub time_per_pte_group_flip_chroma: *mut double;

    pub DST_Y_PER_META_ROW_NOM_L: *mut double;
    pub DST_Y_PER_META_ROW_NOM_C: *mut double;

    pub TimePerMetaChunkNominal: *mut double;
    pub TimePerChromaMetaChunkNominal: *mut double;
    pub TimePerMetaChunkVBlank: *mut double;
    pub TimePerChromaMetaChunkVBlank: *mut double;
    pub TimePerMetaChunkFlip: *mut double;
    pub TimePerChromaMetaChunkFlip: *mut double;
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params {
    pub display_cfg: *const dml2_display_cfg;
    pub USRRetrainingRequired: bool;
    pub NumberOfActiveSurfaces: u32;
    pub MaxLineBufferLines: u32;
    pub LineBufferSize: u32;
    pub WritebackInterfaceBufferSize: u32;
    pub DCFCLK: f64;
    pub ReturnBW: f64;
    pub SynchronizeTimings: bool;
    pub SynchronizeDRRDisplaysForUCLKPStateChange: bool;
    pub dpte_group_bytes: *const u32;
    pub mmSOCParameters: dml2_core_internal_SOCParametersList;
    pub WritebackChunkSize: u32;
    pub SOCCLK: f64;
    pub DCFClkDeepSleep: f64;
    pub DETBufferSizeY: *const u32;
    pub DETBufferSizeC: *const u32;
    pub SwathHeightY: *const u32;
    pub SwathHeightC: *const u32;
    pub SwathWidthY: *const u32;
    pub SwathWidthC: *const u32;
    pub DPPPerSurface: *const u32;
	const double *BytePerPixelDETY;
	const double *BytePerPixelDETC;
    pub DSTXAfterScaler: *const u32;
    pub DSTYAfterScaler: *const u32;
    pub UnboundedRequestEnabled: bool;
    pub CompressedBufferSizeInkByte: u32;
    pub max_outstanding_when_urgent_expected: bool;
	const unsigned int max_outstanding_requests;
	const unsigned int max_request_size_bytes;
    pub meta_row_height_l: *const u32;
    pub meta_row_height_c: *const u32;
    pub uclk_pstate_switch_modes: *const dml2_pstate_method;

	// Output
    pub Watermark: *mut dml2_core_internal_watermarks;
	dml2_pstate_change_support *DRAMClockChangeSupport;
    pub global_dram_clock_change_support_required: *mut bool;
    pub global_dram_clock_change_supported: *mut bool;
    pub MaxActiveDRAMClockChangeLatencySupported: *mut double;
	unsigned int *SubViewportLinesNeededInMALL;
	dml2_pstate_change_support *FCLKChangeSupport;
    pub global_fclk_change_supported: *mut bool;
    pub MaxActiveFCLKChangeLatencySupported: *mut double;
    pub USRRetrainingSupport: *mut bool;
    pub VActiveLatencyHidingMargin: *mut double;
    pub VActiveLatencyHidingUs: *mut double;
    pub g6_temp_read_support: *mut bool;
	dml2_pstate_change_support *temp_read_or_ppt_support;
    pub global_temp_read_or_ppt_supported: *mut bool;
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateSwathAndDETConfiguration_params {
    pub display_cfg: *const dml2_display_cfg;
    pub ConfigReturnBufferSizeInKByte: u32;
    pub MaxTotalDETInKByte: u32;
    pub MinCompressedBufferSizeInKByte: u32;
    pub rob_buffer_size_kbytes: u32;
    pub pixel_chunk_size_kbytes: u32;
    pub ForceSingleDPP: bool;
    pub NumberOfActiveSurfaces: u32;
    pub nomDETInKByte: u32;
    pub ConfigReturnBufferSegmentSizeInkByte: u32;
    pub CompressedBufferSegmentSizeInkByte: u32;
    pub ReadBandwidthLuma: *mut double;
    pub ReadBandwidthChroma: *mut double;
    pub MaximumSwathWidthLuma: *mut double;
    pub MaximumSwathWidthChroma: *mut double;
	unsigned int *Read256BytesBlockHeightY;
	unsigned int *Read256BytesBlockHeightC;
	unsigned int *Read256BytesBlockWidthY;
	unsigned int *Read256BytesBlockWidthC;
    pub surf_linear128_l: *mut bool;
    pub surf_linear128_c: *mut bool;
	dml2_odm_mode *ODMMode;
	unsigned int *BytePerPixY;
	unsigned int *BytePerPixC;
    pub BytePerPixDETY: *mut double;
    pub BytePerPixDETC: *mut double;
	unsigned int *DPPPerSurface;
    pub mrq_present: bool;
	unsigned int dummy: [2]: [DML2_MAX_PLANES];
    pub swath_width_luma_ub_single_dpp: [u32; DML2_MAX_PLANES];
    pub swath_width_chroma_ub_single_dpp: [u32; DML2_MAX_PLANES];

	// output
	unsigned int *req_per_swath_ub_l;
	unsigned int *req_per_swath_ub_c;
	unsigned int *swath_width_luma_ub;
	unsigned int *swath_width_chroma_ub;
	unsigned int *SwathWidth;
	unsigned int *SwathWidthChroma;
	unsigned int *SwathHeightY;
	unsigned int *SwathHeightC;
	unsigned int *request_size_bytes_luma;
	unsigned int *request_size_bytes_chroma;
	unsigned int *DETBufferSizeInKByte;
	unsigned int *DETBufferSizeY;
	unsigned int *DETBufferSizeC;
	unsigned int *full_swath_bytes_l;
	unsigned int *full_swath_bytes_c;
	unsigned int *full_swath_bytes_single_dpp_l;
	unsigned int *full_swath_bytes_single_dpp_c;
    pub UnboundedRequestEnabled: *mut bool;
	unsigned int *compbuf_reserved_space_64b;
	unsigned int *CompressedBufferSizeInkByte;
    pub ViewportSizeSupportPerSurface: *mut bool;
    pub ViewportSizeSupport: *mut bool;
    pub hw_debug5: *mut bool;

    pub funcs: *mut dml2_core_shared_calculation_funcs;
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateStutterEfficiency_locals {
    pub DETBufferingTimeY: f64;
    pub SwathWidthYCriticalSurface: f64;
    pub SwathHeightYCriticalSurface: f64;
    pub VActiveTimeCriticalSurface: f64;
    pub FrameTimeCriticalSurface: f64;
    pub BytePerPixelYCriticalSurface: u32;
    pub DETBufferSizeYCriticalSurface: u32;
    pub MinTTUVBlankCriticalSurface: f64;
    pub BlockWidth256BytesYCriticalSurface: u32;
    pub SinglePlaneCriticalSurface: bool;
    pub SinglePipeCriticalSurface: bool;
    pub TotalCompressedReadBandwidth: f64;
    pub TotalRowReadBandwidth: f64;
    pub AverageDCCCompressionRate: f64;
    pub EffectiveCompressedBufferSize: f64;
    pub PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer: f64;
    pub StutterBurstTime: f64;
    pub TotalActiveWriteback: u32;
    pub LinesInDETY: f64;
    pub LinesInDETYRoundedDownToSwath: f64;
    pub MaximumEffectiveCompressionLuma: f64;
    pub MaximumEffectiveCompressionChroma: f64;
    pub TotalZeroSizeRequestReadBandwidth: f64;
    pub TotalZeroSizeCompressedReadBandwidth: f64;
    pub AverageDCCZeroSizeFraction: f64;
    pub AverageZeroSizeCompressionRate: f64;
    pub stream_visited: [bool; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculateStutterEfficiency_params {
    pub display_cfg: *const dml2_display_cfg;
    pub CompressedBufferSizeInkByte: u32;
    pub UnboundedRequestEnabled: bool;
    pub MetaFIFOSizeInKEntries: u32;
    pub ZeroSizeBufferEntries: u32;
    pub PixelChunkSizeInKByte: u32;
    pub NumberOfActiveSurfaces: u32;
    pub ROBBufferSizeInKByte: u32;
    pub TotalDataReadBandwidth: f64;
    pub DCFCLK: f64;
    pub ReturnBW: f64;
    pub CompbufReservedSpace64B: u32;
    pub CompbufReservedSpaceZs: u32;
    pub hw_debug5: bool;
    pub SRExitTime: f64;
    pub SRExitTimeLowPower: f64;
    pub SRExitZ8Time: f64;
    pub SynchronizeTimings: bool;
    pub StutterEnterPlusExitWatermark: f64;
    pub LowPowerStutterEnterPlusExitWatermark: f64;
    pub Z8StutterEnterPlusExitWatermark: f64;
    pub ProgressiveToInterlaceUnitInOPP: bool;
    pub MinTTUVBlank: *mut double;
	unsigned int *DPPPerSurface;
	unsigned int *DETBufferSizeY;
	unsigned int *BytePerPixelY;
    pub BytePerPixelDETY: *mut double;
	unsigned int *SwathWidthY;
	unsigned int *SwathHeightY;
	unsigned int *SwathHeightC;
	unsigned int *BlockHeight256BytesY;
	unsigned int *BlockWidth256BytesY;
	unsigned int *BlockHeight256BytesC;
	unsigned int *BlockWidth256BytesC;
	unsigned int *DCCYMaxUncompressedBlock;
	unsigned int *DCCCMaxUncompressedBlock;
    pub ReadBandwidthSurfaceLuma: *mut double;
    pub ReadBandwidthSurfaceChroma: *mut double;
    pub meta_row_bw: *mut double;
    pub dpte_row_bw: *mut double;
    pub rob_alloc_compressed: bool;

	// output
    pub StutterEfficiencyNotIncludingVBlank: *mut double;
    pub StutterEfficiency: *mut double;
    pub LowPowerStutterEfficiencyNotIncludingVBlank: *mut double;
    pub LowPowerStutterEfficiency: *mut double;
	unsigned int *NumberOfStutterBurstsPerFrame;
	unsigned int *LowPowerNumberOfStutterBurstsPerFrame;
    pub Z8StutterEfficiencyNotIncludingVBlank: *mut double;
    pub Z8StutterEfficiency: *mut double;
	unsigned int *Z8NumberOfStutterBurstsPerFrame;
    pub StutterPeriod: *mut double;
    pub DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE: *mut bool;
}

#: [repr(C)]
pub struct dml2_core_calcs_CalculatePrefetchSchedule_params {
    pub display_cfg: *const dml2_display_cfg;
    pub HostVMInefficiencyFactor: f64;
    pub myPipe: *mut dml2_core_internal_DmlPipe;
    pub DSCDelay: u32;
    pub DPPCLKDelaySubtotalPlusCNVCFormater: f64;
    pub DPPCLKDelaySCL: f64;
    pub DPPCLKDelaySCLLBOnly: f64;
    pub DPPCLKDelayCNVCCursor: f64;
    pub DISPCLKDelaySubtotal: f64;
    pub DPP_RECOUT_WIDTH: u32;
    pub OutputFormat: dml2_output_format_class;
    pub MaxInterDCNTileRepeaters: u32;
    pub VStartup: u32;
    pub HostVMMinPageSize: u32;
    pub DynamicMetadataEnable: bool;
    pub DynamicMetadataVMEnabled: bool;
    pub DynamicMetadataLinesBeforeActiveRequired: u32;
    pub DynamicMetadataTransmittedBytes: u32;
    pub ExtraLatencyPrefetch: f64;
    pub TCalc: f64;
    pub vm_bytes: u32;
    pub PixelPTEBytesPerRow: u32;
    pub PrefetchSourceLinesY: f64;
    pub VInitPreFillY: u32;
    pub MaxNumSwathY: u32;
    pub PrefetchSourceLinesC: f64;
    pub VInitPreFillC: u32;
    pub MaxNumSwathC: u32;
    pub swath_width_luma_ub: u32;  // per-pipe
    pub swath_width_chroma_ub: u32; // per-pipe
    pub SwathHeightY: u32;
    pub SwathHeightC: u32;
    pub TWait: f64;
    pub Ttrip: f64;
    pub Turg: f64;
    pub setup_for_tdlut: bool;
    pub use_max_lsw: bool;
    pub tdlut_pte_bytes_per_frame: u32;
    pub tdlut_bytes_per_frame: u32;
    pub tdlut_opt_time: f64;
    pub tdlut_drain_time: f64;

    pub num_cursors: u32;
    pub cursor_bytes_per_chunk: u32;
    pub cursor_bytes_per_line: u32;

	// MRQ
    pub dcc_enable: bool;
    pub mrq_present: bool;
    pub meta_row_bytes: u32;
    pub mall_prefetch_sdp_overhead_factor: f64;

    pub impacted_dst_y_pre: f64;
    pub vactive_sw_bw_l: f64; // per surface bw
    pub vactive_sw_bw_c: f64; // per surface bw

	// output
	unsigned int *DSTXAfterScaler;
	unsigned int *DSTYAfterScaler;
    pub dst_y_prefetch: *mut double;
    pub dst_y_per_vm_vblank: *mut double;
    pub dst_y_per_row_vblank: *mut double;
    pub VRatioPrefetchY: *mut double;
    pub VRatioPrefetchC: *mut double;
    pub RequiredPrefetchPixelDataBWLuma: *mut double;
    pub RequiredPrefetchPixelDataBWChroma: *mut double;
    pub RequiredPrefetchBWMax: *mut double;
    pub NotEnoughTimeForDynamicMetadata: *mut bool;
    pub Tno_bw: *mut double;
    pub Tno_bw_flip: *mut double;
    pub prefetch_vmrow_bw: *mut double;
    pub Tdmdl_vm: *mut double;
    pub Tdmdl: *mut double;
    pub TSetup: *mut double;
    pub Tpre_rounded: *mut double;
    pub Tpre_oto: *mut double;
    pub Tvm_trips: *mut double;
    pub Tr0_trips: *mut double;
    pub Tvm_trips_flip: *mut double;
    pub Tr0_trips_flip: *mut double;
    pub Tvm_trips_flip_rounded: *mut double;
    pub Tr0_trips_flip_rounded: *mut double;
	unsigned int *VUpdateOffsetPix;
	unsigned int *VUpdateWidthPix;
	unsigned int *VReadyOffsetPix;
    pub prefetch_cursor_bw: *mut double;
    pub prefetch_sw_bytes: *mut double;
    pub prefetch_swath_time_us: *mut double;
}

#: [repr(C)]
pub struct dml2_core_calcs_CheckGlobalPrefetchAdmissibility_params {
    pub num_active_planes: u32;
	dml2_source_format_class *pixel_format;
    pub rob_buffer_size_kbytes: u32;
    pub compressed_buffer_size_kbytes: u32;
    pub chunk_bytes_l: u32; // same for all planes
    pub chunk_bytes_c: u32;
	unsigned int *detile_buffer_size_bytes_l;
	unsigned int *detile_buffer_size_bytes_c;
	unsigned int *full_swath_bytes_l;
	unsigned int *full_swath_bytes_c;
	unsigned int *lb_source_lines_l;
	unsigned int *lb_source_lines_c;
	unsigned int *swath_height_l;
	unsigned int *swath_height_c;
    pub prefetch_sw_bytes: *mut double;
    pub Tpre_rounded: *mut double;
    pub Tpre_oto: *mut double;
    pub estimated_dcfclk_mhz: f64;
    pub estimated_urg_bandwidth_required_mbps: f64;
    pub line_time: *mut double;
    pub dst_y_prefetch: *mut double;

	// output
    pub recalc_prefetch_schedule: *mut bool;
    pub impacted_dst_y_pre: *mut double;
}

#: [repr(C)]
pub struct dml2_core_calcs_CheckGlobalPrefetchAdmissibility_locals {
    pub max_Trpd_dcfclk_cycles: u32;
    pub burst_bytes_to_fill_det: u32;
    pub time_to_fill_det_us: f64;
    pub accumulated_return_path_dcfclk_cycles: [u32; DML2_MAX_PLANES];
    pub prefetch_global_check_passed: bool;
    pub src_swath_bytes_l: [u32; DML2_MAX_PLANES];
    pub src_swath_bytes_c: [u32; DML2_MAX_PLANES];
    pub src_detile_buf_size_bytes_l: [u32; DML2_MAX_PLANES];
    pub src_detile_buf_size_bytes_c: [u32; DML2_MAX_PLANES];
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_mcache_row_bytes_params {
    pub num_chans: u32;
    pub mem_word_bytes: u32;
    pub mcache_size_bytes: u32;
    pub mcache_line_size_bytes: u32;
    pub gpuvm_enable: u32;
    pub gpuvm_page_size_kbytes: u32;

	//dml_rotation_angle rotation_angle;
    pub surf_vert: bool;
    pub vp_stationary: u32;
    pub tiling_mode: u32;
    pub imall_enable: bool;

    pub vp_start_x: u32;
    pub vp_start_y: u32;
    pub full_vp_width: u32;
    pub full_vp_height: u32;
    pub blk_width: u32;
    pub blk_height: u32;
    pub vmpg_width: u32;
    pub vmpg_height: u32;
    pub full_swath_bytes: u32;
    pub bytes_per_pixel: u32;

	// output
	unsigned int *num_mcaches;
	unsigned int *mcache_row_bytes;
	unsigned int *mcache_row_bytes_per_channel;
	unsigned int *meta_row_width_ub;
    pub dcc_dram_bw_nom_overhead_factor: *mut double;
    pub dcc_dram_bw_pref_overhead_factor: *mut double;
	unsigned int *mvmpg_width;
	unsigned int *mvmpg_height;
	unsigned int *full_vp_access_width_mvmpg_aligned;
	unsigned int *mvmpg_per_mcache_lb;
}

#: [repr(C)]
pub struct dml2_core_shared_calculate_mcache_setting_locals {
    pub l_p: dml2_core_calcs_calculate_mcache_row_bytes_params;
    pub c_p: dml2_core_calcs_calculate_mcache_row_bytes_params;

    pub is_dual_plane: bool;
    pub mvmpg_width_l: u32;
    pub mvmpg_height_l: u32;
    pub full_vp_access_width_mvmpg_aligned_l: u32;
    pub mvmpg_per_mcache_lb_l: u32;
    pub meta_row_width_l: u32;

    pub mvmpg_width_c: u32;
    pub mvmpg_height_c: u32;
    pub full_vp_access_width_mvmpg_aligned_c: u32;
    pub mvmpg_per_mcache_lb_c: u32;
    pub meta_row_width_c: u32;

    pub lc_comb_last_mcache_size: u32;
    pub luma_time_factor: f64;
    pub mcache_remainder_l: f64;
    pub mcache_remainder_c: f64;
    pub mvmpg_access_width_l: u32;
    pub mvmpg_access_width_c: u32;
    pub avg_mcache_element_size_l: u32;
    pub avg_mcache_element_size_c: u32;

    pub full_vp_access_width_l: u32;
    pub full_vp_access_width_c: u32;
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_mcache_setting_params {
    pub dcc_enable: bool;
    pub num_chans: u32;
    pub mem_word_bytes: u32;
    pub mcache_size_bytes: u32;
    pub mcache_line_size_bytes: u32;
    pub gpuvm_enable: u32;
    pub gpuvm_page_size_kbytes: u32;

    pub source_format: dml2_source_format_class;
    pub surf_vert: bool;
    pub vp_stationary: u32;
    pub tiling_mode: u32;
    pub imall_enable: bool;

    pub vp_start_x_l: u32;
    pub vp_start_y_l: u32;
    pub full_vp_width_l: u32;
    pub full_vp_height_l: u32;
    pub blk_width_l: u32;
    pub blk_height_l: u32;
    pub vmpg_width_l: u32;
    pub vmpg_height_l: u32;
    pub full_swath_bytes_l: u32;
    pub bytes_per_pixel_l: u32;

    pub vp_start_x_c: u32;
    pub vp_start_y_c: u32;
    pub full_vp_width_c: u32;
    pub full_vp_height_c: u32;
    pub blk_width_c: u32;
    pub blk_height_c: u32;
    pub vmpg_width_c: u32;
    pub vmpg_height_c: u32;
    pub full_swath_bytes_c: u32;
    pub bytes_per_pixel_c: u32;

	// output
	unsigned int *num_mcaches_l;
	unsigned int *mcache_row_bytes_l;
	unsigned int *mcache_row_bytes_per_channel_l;
	unsigned int *mcache_offsets_l;
	unsigned int *mcache_shift_granularity_l;
    pub dcc_dram_bw_nom_overhead_factor_l: *mut double;
    pub dcc_dram_bw_pref_overhead_factor_l: *mut double;

	unsigned int *num_mcaches_c;
	unsigned int *mcache_row_bytes_c;
	unsigned int *mcache_row_bytes_per_channel_c;
	unsigned int *mcache_offsets_c;
	unsigned int *mcache_shift_granularity_c;
    pub dcc_dram_bw_nom_overhead_factor_c: *mut double;
    pub dcc_dram_bw_pref_overhead_factor_c: *mut double;

    pub mall_comb_mcache_l: *mut bool;
    pub mall_comb_mcache_c: *mut bool;
    pub lc_comb_mcache: *mut bool;
}


#: [repr(C)]
pub struct dml2_core_calcs_calculate_alternate_lead_lines {
	/* input params */
    pub display_cfg: *const dml2_display_cfg;
	unsigned int *VStartup;
    pub VActiveLatencyHidingUs: *mut double;

	/* output params */
	unsigned int *min_lead_dst_lines;
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_alternate_svp_lines {
	/* input params */
    pub display_cfg: *const dml2_display_cfg;
	unsigned int *SwathHeightY;
	unsigned int *SwathHeightC;
	unsigned int *DETBufferSizeY;
    pub BytePerPixelInDETC: *mut double;
    pub dram_blackout_us: f64;

	/* output params */
	unsigned int *svp0_dst_lines;
	unsigned int *svp1_dst_lines;
	unsigned int *svp_req_limit;
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_alternate_params {
	/* input params */
    pub display_cfg: *const dml2_display_cfg;
    pub dst_y_prefetch: *mut double;
	unsigned int *SwathHeightY;
	unsigned int *SwathHeightC;
	unsigned int *SwathWidthY;
	unsigned int *SwathWidthC;
	unsigned int *DETBufferSizeY;
	unsigned int *DETBufferSizeC;
	unsigned int *BytePerPixelY;
	unsigned int *BytePerPixelC;
    pub BytePerPixelInDETY: *mut double;
    pub BytePerPixelInDETC: *mut double;
	unsigned int *Read256BlockWidthY;
	unsigned int *Read256BlockHeightY;
	unsigned int *Read256BlockWidthC;
	unsigned int *Read256BlockHeightC;
	unsigned int *MacroTileWidthY;
	unsigned int *MacroTileWidthC;
	unsigned int *VInitPrefillY;
	unsigned int *VInitPrefillC;
    pub VRatioPrefetchY: *mut double;
    pub VRatioPrefetchC: *mut double;
	unsigned int *NoOfDPP;
    pub max_num_dpp: u32;
    pub dram_blackout_us: f64;
    pub VActiveLatencyHidingUs: *mut double;
	unsigned int *svp0_dst_lines;
	unsigned int *svp1_dst_lines;
	unsigned int *svp_req_limit;
    pub dcn_non_urgent_bandwidth_kbps: f64;
    pub alt_chan_fw_delay_us: u32;
    pub dst_y_per_vm_vblank: *mut double;
    pub dst_y_per_row_vblank: *mut double;
	unsigned int *DSTYAfterScaler;
	dml2_odm_mode *ODMMode;

	/* output params */
	unsigned int *svp0_max_bytes;
	unsigned int *svp1_max_bytes;
	unsigned int *svp0_max_bytes_per_dpp;
	unsigned int *svp0_max_bytes_per_dpp_c;
	unsigned int *svp1_max_bytes_per_dpp;
	unsigned int *svp1_max_bytes_per_dpp_c;
	unsigned int *nom_req_limit_alt;
	unsigned int *min_lead_dst_lines;
	unsigned int *total_swaths;
	unsigned int *total_swaths_c;
	unsigned int *prefetch_swaths;
	unsigned int *prefetch_swaths_c;
    pub prefetch_hdl_delta: *mut double;
    pub recout_hdl_delta: *mut double;
    pub prefetch_hdl_delta_c: *mut double;
    pub recout_hdl_delta_c: *mut double;
	unsigned int *max_prefetch_in_lines;
    pub lsdma_bw_req_for_alt_kbps: *mut double;
}
#: [repr(C)]
pub struct dml2_core_calcs_calculate_tdlut_setting_params {
	// input params
    pub dispclk_mhz: f64;
    pub setup_for_tdlut: bool;
    pub tdlut_width_mode: dml2_tdlut_width_mode;
    pub tdlut_addressing_mode: dml2_tdlut_addressing_mode;
    pub cursor_buffer_size: u32;
    pub gpuvm_enable: bool;
    pub gpuvm_page_size_kbytes: u32;
    pub is_gfx11: bool;
    pub tdlut_mpc_width_flag: bool;

	// output param
	unsigned int *tdlut_pte_bytes_per_frame;
	unsigned int *tdlut_bytes_per_frame;
	unsigned int *tdlut_groups_per_2row_ub;
    pub tdlut_opt_time: *mut double;
    pub tdlut_drain_time: *mut double;
	unsigned int *tdlut_bytes_to_deliver;
	unsigned int *tdlut_bytes_per_group;
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_peak_bandwidth_required_params {
	// output
	double (*urg_vactive_bandwidth_required): [dml2_core_internal_bw_max];
	double (*urg_bandwidth_required): [dml2_core_internal_bw_max];
	double (*urg_bandwidth_required_qual): [dml2_core_internal_bw_max];
	double (*non_urg_bandwidth_required): [dml2_core_internal_bw_max];
	double (*surface_avg_vactive_required_bw): [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];
	double (*surface_peak_required_bw): [dml2_core_internal_bw_max]: [DML2_MAX_PLANES];

	// input
    pub display_cfg: *const dml2_display_cfg;
    pub inc_flip_bw: bool;
    pub num_active_planes: u32;
	unsigned int *num_of_dpp;
    pub dcc_dram_bw_nom_overhead_factor_p0: *mut double;
    pub dcc_dram_bw_nom_overhead_factor_p1: *mut double;
    pub dcc_dram_bw_pref_overhead_factor_p0: *mut double;
    pub dcc_dram_bw_pref_overhead_factor_p1: *mut double;
    pub mall_prefetch_sdp_overhead_factor: *mut double;
    pub mall_prefetch_dram_overhead_factor: *mut double;
    pub surface_read_bandwidth_l: *mut double;
    pub surface_read_bandwidth_c: *mut double;
    pub prefetch_bandwidth_l: *mut double;
    pub prefetch_bandwidth_c: *mut double;
    pub prefetch_bandwidth_max: *mut double;
    pub excess_vactive_fill_bw_l: *mut double;
    pub excess_vactive_fill_bw_c: *mut double;
    pub cursor_bw: *mut double;
    pub dpte_row_bw: *mut double;
    pub meta_row_bw: *mut double;
    pub prefetch_cursor_bw: *mut double;
    pub prefetch_vmrow_bw: *mut double;
    pub flip_bw: *mut double;
    pub urgent_burst_factor_l: *mut double;
    pub urgent_burst_factor_c: *mut double;
    pub urgent_burst_factor_cursor: *mut double;
    pub urgent_burst_factor_prefetch_l: *mut double;
    pub urgent_burst_factor_prefetch_c: *mut double;
    pub urgent_burst_factor_prefetch_cursor: *mut double;
}

#: [repr(C)]
pub struct dml2_core_calcs_calculate_bytes_to_fetch_required_to_hide_latency_params {
	/* inputs */
    pub display_cfg: *const dml2_display_cfg;
    pub mrq_present: bool;
    pub num_active_planes: u32;
	unsigned int *num_of_dpp;
	unsigned int *meta_row_height_l;
	unsigned int *meta_row_height_c;
	unsigned int *meta_row_bytes_per_row_ub_l;
	unsigned int *meta_row_bytes_per_row_ub_c;
	unsigned int *dpte_row_height_l;
	unsigned int *dpte_row_height_c;
	unsigned int *dpte_bytes_per_row_l;
	unsigned int *dpte_bytes_per_row_c;
	unsigned int *byte_per_pix_l;
	unsigned int *byte_per_pix_c;
	unsigned int *swath_width_l;
	unsigned int *swath_width_c;
	unsigned int *swath_height_l;
	unsigned int *swath_height_c;
    pub latency_to_hide_us: [f64; DML2_MAX_PLANES];

	/* outputs */
	unsigned int *bytes_required_l;
	unsigned int *bytes_required_c;
}

// A list of overridable function pointers in the core
// shared calculation library.
#: [repr(C)]
pub struct dml2_core_shared_calculation_funcs {
    pub calculate_det_buffer_size: Option<unsafe extern "C" fn(dml2_core_shared_calculate_det_buffer_size_params *p)>;
}

#: [repr(C)]
pub struct dml2_core_internal_scratch {
	// Scratch space for function locals
    pub dml_core_mode_support_locals: dml2_core_calcs_mode_support_locals;
    pub dml_core_mode_programming_locals: dml2_core_calcs_mode_programming_locals;
    pub CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals: dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals;
    pub CalculateVMRowAndSwath_locals: dml2_core_calcs_CalculateVMRowAndSwath_locals;
    pub CalculatePrefetchSchedule_locals: dml2_core_calcs_CalculatePrefetchSchedule_locals;
    pub CheckGlobalPrefetchAdmissibility_locals: dml2_core_calcs_CheckGlobalPrefetchAdmissibility_locals;
    pub CalculateSwathAndDETConfiguration_locals: dml2_core_shared_CalculateSwathAndDETConfiguration_locals;
    pub TruncToValidBPP_locals: dml2_core_shared_TruncToValidBPP_locals;
    pub CalculateDETBufferSize_locals: dml2_core_shared_CalculateDETBufferSize_locals;
    pub get_urgent_bandwidth_required_locals: dml2_core_shared_get_urgent_bandwidth_required_locals;
    pub calculate_peak_bandwidth_required_locals: dml2_core_shared_calculate_peak_bandwidth_required_locals;
    pub CalculateFlipSchedule_locals: dml2_core_shared_CalculateFlipSchedule_locals;
    pub rq_dlg_get_dlg_reg_locals: dml2_core_shared_rq_dlg_get_dlg_reg_locals;
    pub CalculateStutterEfficiency_locals: dml2_core_calcs_CalculateStutterEfficiency_locals;

	// Scratch space for function params
    pub CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params: dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params;
    pub CalculateVMRowAndSwath_params: dml2_core_calcs_CalculateVMRowAndSwath_params;
    pub CalculateSwathAndDETConfiguration_params: dml2_core_calcs_CalculateSwathAndDETConfiguration_params;
    pub CalculateStutterEfficiency_params: dml2_core_calcs_CalculateStutterEfficiency_params;
    pub CalculatePrefetchSchedule_params: dml2_core_calcs_CalculatePrefetchSchedule_params;
    pub CheckGlobalPrefetchAdmissibility_params: dml2_core_calcs_CheckGlobalPrefetchAdmissibility_params;
    pub calculate_mcache_setting_params: dml2_core_calcs_calculate_mcache_setting_params;
    pub calculate_tdlut_setting_params: dml2_core_calcs_calculate_tdlut_setting_params;
    pub calculate_vm_and_row_bytes_params: dml2_core_shared_calculate_vm_and_row_bytes_params;
    pub calculate_mcache_setting_locals: dml2_core_shared_calculate_mcache_setting_locals;
    pub CalculateMetaAndPTETimes_params: dml2_core_shared_CalculateMetaAndPTETimes_params;
    pub calculate_peak_bandwidth_params: dml2_core_calcs_calculate_peak_bandwidth_required_params;
    pub calculate_bytes_to_fetch_required_to_hide_latency_params: dml2_core_calcs_calculate_bytes_to_fetch_required_to_hide_latency_params;
    pub calculate_alternate_params: dml2_core_calcs_calculate_alternate_params;
    pub calculate_alternate_svp_lines: dml2_core_calcs_calculate_alternate_svp_lines;
    pub calculate_alternate_lead_lines: dml2_core_calcs_calculate_alternate_lead_lines;
}

//struct dml2_svp_mode_override;
#: [repr(C)]
pub struct dml2_core_internal_display_mode_lib {
    pub ip: dml2_core_ip_params;
    pub soc: dml2_soc_bb;
    pub ip_caps: dml2_ip_capabilities;

	//@brief Mode Support and Mode programming struct
	// Used to hold input; intermediate and output of the calculations
    pub ms: dml2_core_internal_mode_support; // struct for mode support
    pub mp: dml2_core_internal_mode_program; // struct for mode programming
	// Available overridable calculators for core_shared.
	// if null, core_shared will use default calculators.
    pub funcs: dml2_core_shared_calculation_funcs;

    pub scratch: dml2_core_internal_scratch;
}

#: [repr(C)]
pub struct dml2_core_calcs_mode_support_ex {
    pub mode_lib: *mut dml2_core_internal_display_mode_lib;
    pub in_display_cfg: *const dml2_display_cfg;
    pub min_clk_table: *const dml2_mcg_min_clock_table;
	int min_clk_index;
    pub utm_soc_bb: *const dml2_utm_soc_bb;
	//unsigned int in_state_index;
    pub out_evaluation_info: *mut dml2_core_internal_mode_support_info;
    pub uclk_pstate_switch_modes: *const dml2_pstate_method;
}

struct core_display_cfg_support_info;

#: [repr(C)]
pub struct dml2_core_calcs_mode_programming_ex {
    pub mode_lib: *mut dml2_core_internal_display_mode_lib;
    pub in_display_cfg: *const dml2_display_cfg;
    pub min_clk_table: *const dml2_mcg_min_clock_table;
    pub cfg_support_info: *const core_display_cfg_support_info;
	int min_clk_index;
    pub uclk_params: *const dml2_uclk_pstate_params;
    pub solution: *const dml2_display_solution;
    pub utm_soc_bb: *const dml2_utm_soc_bb;
    pub programming: *mut dml2_display_cfg_programming;
}

// header guard end

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
