// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Types, constants, logging macros, and helper functions are supplied by the
// corresponding DML headers and translation units.

pub unsafe fn dml2_core_utils_div_rem(dividend: f64, divisor: u32, remainder: *mut u32) -> f64 {
    *remainder = ((dividend / divisor as f64) - (dividend / divisor as f64) as i32 as f64 > 0.0) as u32;
    dividend / divisor as f64
}

pub unsafe fn dml2_core_utils_internal_bw_type_str(bw_type: dml2_core_internal_bw_type) -> *const i8 {
    match bw_type {
        dml2_core_internal_bw_sdp => b"dml2_core_internal_bw_sdp\0".as_ptr() as *const i8,
        dml2_core_internal_bw_dram => b"dml2_core_internal_bw_dram\0".as_ptr() as *const i8,
        dml2_core_internal_bw_max => b"dml2_core_internal_bw_max\0".as_ptr() as *const i8,
        _ => b"dml2_core_internal_bw_unknown\0".as_ptr() as *const i8,
    }
}

pub unsafe fn dml2_core_utils_is_420(source_format: dml2_source_format_class) -> bool {
    match source_format { dml2_420_8 | dml2_420_10 | dml2_420_12 => true, _ => false }
}
pub unsafe fn dml2_core_utils_is_422_planar(source_format: dml2_source_format_class) -> bool {
    match source_format { dml2_422_planar_8 | dml2_422_planar_10 | dml2_422_planar_12 => true, _ => false }
}
pub unsafe fn dml2_core_utils_is_422_packed(source_format: dml2_source_format_class) -> bool {
    match source_format { dml2_422_packed_8 | dml2_422_packed_10 | dml2_422_packed_12 => true, _ => false }
}

pub unsafe fn dml2_core_utils_print_mode_support_info(support: *const dml2_core_internal_mode_support_info, fail_only: bool) {
    DML_LOG_VERBOSE!("DML: ===================================== \n");
    DML_LOG_VERBOSE!("DML: DML_MODE_SUPPORT_INFO_ST\n");
    macro_rules! log_if { ($field:ident, $bad:expr, $name:expr) => { if !fail_only || (*support).$field == $bad { DML_LOG_VERBOSE!(concat!("DML: support: ", $name, " = %d\n"), (*support).$field); } }; }
    log_if!(ScaleRatioAndTapsSupport, 0, "ScaleRatioAndTapsSupport"); log_if!(SourceFormatPixelAndScanSupport, 0, "SourceFormatPixelAndScanSupport");
    log_if!(ViewportSizeSupport, 0, "ViewportSizeSupport"); log_if!(LinkRateDoesNotMatchDPVersion, 1, "LinkRateDoesNotMatchDPVersion");
    log_if!(LinkRateForMultistreamNotIndicated, 1, "LinkRateForMultistreamNotIndicated"); log_if!(BPPForMultistreamNotIndicated, 1, "BPPForMultistreamNotIndicated");
    log_if!(MultistreamWithHDMIOreDP, 1, "MultistreamWithHDMIOreDP"); log_if!(ExceededMultistreamSlots, 1, "ExceededMultistreamSlots");
    log_if!(MSOOrODMSplitWithNonDPLink, 1, "MSOOrODMSplitWithNonDPLink"); log_if!(NotEnoughLanesForMSO, 1, "NotEnoughLanesForMSO");
    log_if!(P2IWith420, 1, "P2IWith420"); log_if!(DSC422NativeNotSupported, 1, "DSC422NativeNotSupported"); log_if!(DSCSlicesODMModeSupported, 0, "DSCSlicesODMModeSupported");
    log_if!(NotEnoughDSCUnits, 1, "NotEnoughDSCUnits"); log_if!(NotEnoughDSCSlices, 1, "NotEnoughDSCSlices"); log_if!(ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe, 1, "ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe");
    log_if!(InvalidCombinationOfMALLUseForPStateAndStaticScreen, 1, "InvalidCombinationOfMALLUseForPStateAndStaticScreen"); log_if!(DSCCLKRequiredMoreThanSupported, 1, "DSCCLKRequiredMoreThanSupported");
    log_if!(PixelsPerLinePerDSCUnitSupport, 0, "PixelsPerLinePerDSCUnitSupport"); log_if!(DTBCLKRequiredMoreThanSupported, 1, "DTBCLKRequiredMoreThanSupported"); log_if!(InvalidCombinationOfMALLUseForPState, 1, "InvalidCombinationOfMALLUseForPState");
    log_if!(ROBSupport, 0, "ROBSupport"); log_if!(OutstandingRequestsSupport, 0, "OutstandingRequestsSupport"); log_if!(OutstandingRequestsUrgencyAvoidance, 0, "OutstandingRequestsUrgencyAvoidance"); log_if!(DISPCLK_DPPCLK_Support, 0, "DISPCLK_DPPCLK_Support");
    log_if!(TotalAvailablePipesSupport, 0, "TotalAvailablePipesSupport"); log_if!(NumberOfOTGSupport, 0, "NumberOfOTGSupport"); log_if!(NumberOfHDMIFRLSupport, 0, "NumberOfHDMIFRLSupport"); log_if!(NumberOfDP2p0Support, 0, "NumberOfDP2p0Support");
    log_if!(EnoughWritebackUnits, 0, "EnoughWritebackUnits"); log_if!(WritebackScaleRatioAndTapsSupport, 0, "WritebackScaleRatioAndTapsSupport"); log_if!(WritebackLatencySupport, 0, "WritebackLatencySupport"); log_if!(CursorSupport, 0, "CursorSupport"); log_if!(PitchSupport, 0, "PitchSupport");
    log_if!(ViewportExceedsSurface, 1, "ViewportExceedsSurface"); log_if!(PrefetchSupported, 0, "PrefetchSupported"); log_if!(EnoughUrgentLatencyHidingSupport, 0, "EnoughUrgentLatencyHidingSupport"); log_if!(AvgBandwidthSupport, 0, "AvgBandwidthSupport"); log_if!(DynamicMetadataSupported, 0, "DynamicMetadataSupported");
    log_if!(VRatioInPrefetchSupported, 0, "VRatioInPrefetchSupported"); log_if!(PTEBufferSizeNotExceeded, 0, "PTEBufferSizeNotExceeded"); log_if!(DCCMetaBufferSizeNotExceeded, 0, "DCCMetaBufferSizeNotExceeded"); log_if!(ExceededMALLSize, 1, "ExceededMALLSize"); log_if!(g6_temp_read_support, 0, "g6_temp_read_support");
    log_if!(ImmediateFlipSupport, 0, "ImmediateFlipSupport"); log_if!(LinkCapacitySupport, 0, "LinkCapacitySupport"); log_if!(ModeSupport, 0, "ModeSupport");
    DML_LOG_VERBOSE!("DML: ===================================== \n");
}

pub unsafe fn dml2_core_utils_internal_soc_state_type_str(v: dml2_core_internal_soc_state_type) -> *const i8 { match v { dml2_core_internal_soc_state_sys_idle => b"dml2_core_internal_soc_state_sys_idle\0".as_ptr() as _, dml2_core_internal_soc_state_sys_active => b"dml2_core_internal_soc_state_sys_active\0".as_ptr() as _, dml2_core_internal_soc_state_svp_prefetch => b"dml2_core_internal_soc_state_svp_prefetch\0".as_ptr() as _, _ => b"dml2_core_internal_soc_state_unknown\0".as_ptr() as _ } }

pub unsafe fn dml2_core_utils_get_stream_output_bpp(out_bpp: *mut f64, display_cfg: *const dml2_display_cfg) { for k in 0..(*display_cfg).num_planes { let s = (*display_cfg).plane_descriptors[k as usize].stream_index; let bpc = (*display_cfg).stream_descriptors[s as usize].timing.bpc as f64; let dsc = (*display_cfg).stream_descriptors[s as usize].timing.dsc; if dsc.enable == dml2_dsc_disable { (*out_bpp.add(k as usize)) = match (*display_cfg).stream_descriptors[s as usize].output.output_format { dml2_444 => bpc * 3.0, dml2_s422 | dml2_n422 => bpc * 2.0, _ => bpc * 1.5 }; } else if dsc.enable == dml2_dsc_enable { (*out_bpp.add(k as usize)) = dsc.dsc_compressed_bpp_x16 as f64 / 16.0; } else { (*out_bpp.add(k as usize)) = 0.0; } } }

pub unsafe fn dml2_core_utils_round_to_multiple(num: u32, multiple: u32, up: bool) -> u32 { if multiple == 0 { return num; } let r = num % multiple; if r == 0 { num } else if up { num + multiple - r } else { num - r } }
pub unsafe fn dml2_core_util_get_num_active_pipes(num_planes: u32, info: *const core_display_cfg_support_info) -> u32 { let mut n=0; for k in 0..num_planes { n += (*info).plane_support_info[k as usize].dpps_used as u32; } n }
pub unsafe fn dml2_core_utils_pipe_plane_mapping(info: *const core_display_cfg_support_info, pipe_plane: *mut u32) { for k in 0..DML2_MAX_PLANES { *pipe_plane.add(k as usize)=__DML2_CALCS_PIPE_NO_PLANE__; } let mut p=0; for plane in 0..DML2_MAX_PLANES { for _ in 0..(*info).plane_support_info[plane as usize].dpps_used { *pipe_plane.add(p as usize)=plane; p+=1; } } }
pub unsafe fn dml2_core_utils_is_phantom_pipe(p: *const dml2_plane_parameters) -> bool { (*p).overrides.legacy_svp_config == dml2_svp_mode_override_phantom_pipe || (*p).overrides.legacy_svp_config == dml2_svp_mode_override_phantom_pipe_no_data_return }
pub unsafe fn dml2_core_utils_get_tile_block_size_bytes(m: dml2_swizzle_mode, _bpp: u32) -> u32 { match m { dml2_sw_linear|dml2_sw_256b_2d|dml2_gfx11_sw_linear=>256, dml2_sw_4kb_2d=>4096, dml2_sw_64kb_2d|dml2_gfx11_sw_64kb_d|dml2_gfx11_sw_64kb_d_t|dml2_gfx11_sw_64kb_d_x|dml2_gfx11_sw_64kb_r_x=>65536, dml2_sw_256kb_2d|dml2_gfx11_sw_256kb_d_x|dml2_gfx11_sw_256kb_r_x=>262144, _=>256 } }
pub unsafe fn dml2_core_utils_get_segment_horizontal_contiguous(_m: dml2_swizzle_mode, bpp: u32) -> bool { bpp != 2 }
pub unsafe fn dml2_core_utils_is_linear(m: dml2_swizzle_mode) -> bool { m == dml2_sw_linear }
pub unsafe fn dml2_core_utils_is_vertical_rotation(s: dml2_rotation_angle) -> bool { s == dml2_rotation_90 || s == dml2_rotation_270 }
pub unsafe fn dml2_core_utils_get_gfx_version(m: dml2_swizzle_mode) -> u32 { match m { dml2_sw_linear|dml2_sw_256b_2d|dml2_sw_4kb_2d|dml2_sw_64kb_2d|dml2_sw_256kb_2d=>12, dml2_gfx11_sw_linear|dml2_gfx11_sw_64kb_d|dml2_gfx11_sw_64kb_d_t|dml2_gfx11_sw_64kb_d_x|dml2_gfx11_sw_64kb_r_x|dml2_gfx11_sw_256kb_d_x|dml2_gfx11_sw_256kb_r_x=>11, _=>0 } }
pub unsafe fn dml2_core_utils_is_dual_plane(f: dml2_source_format_class) -> bool { dml2_core_utils_is_420(f)||dml2_core_utils_is_422_planar(f)||f==dml2_rgbe_alpha }
pub unsafe fn dml2_core_utils_log_and_substract_if_non_zero(a:u32, s:u32)->u32 { if a==0 {0} else {math_log2_approx(a)-s} }

pub unsafe fn dml2_core_utils_get_qos_param_index(freq:u64, params:*const dml2_dcn4_uclk_dpm_dependent_qos_params)->u32 { let mut index=0; for i in 0..DML_MAX_CLK_TABLE_SIZE { if i==0 {index=0} else {index=i-1;} if freq < (*params.add(i as usize)).minimum_uclk_khz || (*params.add(i as usize)).minimum_uclk_khz==0 {break;} } index }
pub unsafe fn dml2_core_utils_get_active_min_uclk_dpm_index(freq:u64, table:*const dml2_soc_state_table)->u32 { let mut i=0; let mut found=false; while i<(*table).uclk.num_clk_values { if freq==(*table).uclk.clk_values_khz[i as usize] {found=true;break;} i+=1; } if !found {DML_ASSERT!(found);} i }

unsafe fn create_phantom_stream_from_main_stream(p:*mut dml2_stream_parameters, m:*const dml2_stream_parameters, meta:*const dml2_implicit_svp_meta) { core::ptr::copy_nonoverlapping(m,p,1); (*p).timing.v_total=(*meta).v_total; (*p).timing.v_active=(*meta).v_active; (*p).timing.v_front_porch=(*meta).v_front_porch; (*p).timing.v_blank_end=(*p).timing.v_total-(*p).timing.v_front_porch-(*p).timing.v_active; (*p).timing.vblank_nom=(*p).timing.v_total-(*p).timing.v_active; (*p).timing.drr_config.enabled=false; }
unsafe fn create_phantom_plane_from_main_plane(p:*mut dml2_plane_parameters,m:*const dml2_plane_parameters, ps:*const dml2_stream_parameters, idx:i32, _ms:*const dml2_stream_parameters) { core::ptr::copy_nonoverlapping(m,p,1); (*p).stream_index=idx; (*p).overrides.refresh_from_mall=dml2_refresh_from_mall_mode_override_force_disable; (*p).overrides.legacy_svp_config=dml2_svp_mode_override_phantom_pipe_no_data_return; (*p).composition.viewport.plane0.height=math_min2(math_ceil2((*m).composition.scaler_info.plane0.v_ratio as f64*(*ps).timing.v_active as f64,16.0),(*p).composition.viewport.plane0.height as f64) as u64; (*p).composition.viewport.plane1.height=math_min2(math_ceil2((*m).composition.scaler_info.plane1.v_ratio as f64*(*ps).timing.v_active as f64,16.0),(*p).composition.viewport.plane1.height as f64) as u64; (*p).immediate_flip=false; (*p).dynamic_meta_data.enable=false; (*p).cursor.num_cursors=0; (*p).cursor.cursor_width=0; (*p).tdlut.setup_for_tdlut=false; }

pub unsafe fn dml2_core_utils_expand_implict_subvp(cfg:*const display_configuation_with_meta, out:*mut dml2_display_cfg, scratch:*mut dml2_core_scratch) { core::ptr::copy_nonoverlapping(&(*cfg).display_config,out,1); for i in 0..DML2_MAX_PLANES { (*scratch).main_stream_index_from_svp_stream_index[i as usize]=0; (*scratch).svp_stream_index_from_main_stream_index[i as usize]=0; (*scratch).main_plane_index_to_phantom_plane_index[i as usize]=0; } if !(*cfg).display_config.overrides.enable_subvp_implicit_pmo{return;} if !(*cfg).stage3.performed {(*out).overrides.hw.force_unbounded_requesting.enable=true;(*out).overrides.hw.force_unbounded_requesting.value=false;} for i in 0..(*cfg).display_config.num_streams { (*scratch).main_stream_index_from_svp_stream_index[i as usize]=i;(*scratch).svp_stream_index_from_main_stream_index[i as usize]=i; if (*cfg).stage3.stream_svp_meta[i as usize].valid {create_phantom_stream_from_main_stream(&mut (*out).stream_descriptors[(*out).num_streams as usize],&(*cfg).display_config.stream_descriptors[i as usize],&(*cfg).stage3.stream_svp_meta[i as usize]);(*scratch).main_stream_index_from_svp_stream_index[(*out).num_streams as usize]=i;(*scratch).svp_stream_index_from_main_stream_index[i as usize]=(*out).num_streams;(*out).num_streams+=1;} } for i in 0..(*cfg).display_config.num_planes {let mp=&(*cfg).display_config.plane_descriptors[i as usize];if (*cfg).stage3.stream_svp_meta[mp.stream_index as usize].valid {let ps=&(*out).stream_descriptors[(*scratch).svp_stream_index_from_main_stream_index[mp.stream_index as usize] as usize];create_phantom_plane_from_main_plane(&mut (*out).plane_descriptors[(*out).num_planes as usize],mp,ps,(*scratch).svp_stream_index_from_main_stream_index[mp.stream_index as usize] as i32,&(*cfg).display_config.stream_descriptors[mp.stream_index as usize]);(*scratch).phantom_plane_index_to_main_plane_index[(*out).num_planes as usize]=i;(*scratch).main_plane_index_to_phantom_plane_index[i as usize]=(*out).num_planes;(*out).num_planes+=1;(*out).plane_descriptors[i as usize].overrides.legacy_svp_config=dml2_svp_mode_override_main_pipe;}} }

pub unsafe fn dml2_core_utils_is_stream_encoder_required(s:*const dml2_stream_parameters)->bool { matches!((*s).output.output_encoder,dml2_dp|dml2_dp2p0|dml2_edp|dml2_hdmi|dml2_hdmifrl) }
pub unsafe fn dml2_core_utils_is_encoder_dsc_capable(s:*const dml2_stream_parameters)->bool { matches!((*s).output.output_encoder,dml2_dp|dml2_dp2p0|dml2_edp|dml2_hdmifrl) }
pub unsafe fn dml2_core_utils_is_dio_dp_encoder(s:*const dml2_stream_parameters)->bool { matches!((*s).output.output_encoder,dml2_dp|dml2_edp) }
pub unsafe fn dml2_core_utils_is_hpo_dp_encoder(s:*const dml2_stream_parameters)->bool { (*s).output.output_encoder==dml2_dp2p0 }
pub unsafe fn dml2_core_utils_is_dp_encoder(s:*const dml2_stream_parameters)->bool { dml2_core_utils_is_dio_dp_encoder(s)||dml2_core_utils_is_hpo_dp_encoder(s) }
pub unsafe fn dml2_core_utils_is_dp_8b_10b_link_rate(r:dml2_output_link_dp_rate)->bool { matches!(r,dml2_dp_rate_hbr|dml2_dp_rate_hbr2|dml2_dp_rate_hbr3) }
pub unsafe fn dml2_core_utils_is_dp_128b_132b_link_rate(r:dml2_output_link_dp_rate)->bool { matches!(r,dml2_dp_rate_uhbr10|dml2_dp_rate_uhbr13p5|dml2_dp_rate_uhbr20) }
pub unsafe fn dml2_core_utils_is_odm_split(m:dml2_odm_mode)->bool { matches!(m,dml2_odm_mode_split_1to2|dml2_odm_mode_mso_1to2|dml2_odm_mode_mso_1to4) }
pub unsafe fn dml2_core_utils_get_frame_time_us(s:*const dml2_stream_parameters)->f64 { let line=(*s).timing.h_total as f64/(*s).timing.pixel_clock_khz as f64*1000.0; ((*s).timing.vblank_nom+(*s).timing.v_active) as f64*line }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
