// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translation unit.

unsafe fn dml21_populate_pmo_options(pmo_options: *mut dml2_pmo_options, in_dc: *const dc, config: *const dml2_configuration_options) {
    let disable_fams2 = !(*in_dc).debug.fams2_config.bits.enable;
    (*pmo_options).disable_dyn_odm = !(*config).minimize_dispclk_using_odm;
    (*pmo_options).disable_dyn_odm_for_multi_stream = true;
    (*pmo_options).disable_dyn_odm_for_stream_with_svp = true;
    (*pmo_options).disable_vblank = ((*in_dc).debug.dml21_disable_pstate_method_mask >> 1) & 1;
    (*pmo_options).disable_svp = (((*in_dc).debug.dml21_disable_pstate_method_mask >> 2) & 1) != 0 || (*in_dc).debug.force_disable_subvp || disable_fams2;
    (*pmo_options).disable_drr_clamped = (((*in_dc).debug.dml21_disable_pstate_method_mask >> 3) & 1) != 0 || disable_fams2;
    (*pmo_options).disable_drr_var = (((*in_dc).debug.dml21_disable_pstate_method_mask >> 4) & 1) != 0 || disable_fams2;
    (*pmo_options).disable_fams2 = disable_fams2;
    (*pmo_options).disable_drr_var_when_var_active = (*in_dc).debug.disable_fams_gaming == INGAME_FAMS_DISABLE || (*in_dc).debug.disable_fams_gaming == INGAME_FAMS_MULTI_DISP_CLAMPED_ONLY;
    (*pmo_options).disable_drr_clamped_when_var_active = (*in_dc).debug.disable_fams_gaming == INGAME_FAMS_DISABLE;
    (*pmo_options).force_mandatory_uclk_pstate_support = (*config).pmo.force_mandatory_uclk_pstate_support;
}

unsafe fn dml21_dcn_revision_to_dml2_project_id(in_dc: *const dc) -> dml2_project_id {
    match (*in_dc).ctx.dce_version {
        DCN_VERSION_4_01 => dml2_project_dcn4x_stage2_auto_drr_svp,
        DCN_VERSION_4_2 | DCN_VERSION_4_2B => dml2_project_dcn42,
        DCN_VERSION_6_0 => dml2_project_dcn6x_soc_var_a,
        _ => { DC_ERR!("unsupported dcn version for DML21!"); dml2_project_invalid }
    }
}

pub unsafe fn dml21_populate_dml_init_params(dml_init: *mut dml2_initialize_instance_in_out, config: *const dml2_configuration_options, in_dc: *const dc) {
    (*dml_init).options.project_id = dml21_dcn_revision_to_dml2_project_id(in_dc);
    if (*config).use_native_soc_bb_construction {
        ((*in_dc).soc_and_ip_translator).translator_funcs.get_soc_bb(&mut (*dml_init).soc_bb, in_dc, config);
        ((*in_dc).soc_and_ip_translator).translator_funcs.get_ip_caps(&mut (*dml_init).ip_caps);
    } else {
        (*dml_init).soc_bb = (*config).external_socbb_ip_params.soc_bb;
        (*dml_init).ip_caps = (*config).external_socbb_ip_params.ip_params;
    }
    dml21_populate_pmo_options(&mut (*dml_init).options.pmo_options, in_dc, config);
    if !(*in_dc).clk_mgr.is_null() && !(*(*in_dc).clk_mgr).bw_params.is_null() {
        (*dml_init).overrides.explicit_qos_model = (*(*(*in_dc).clk_mgr).bw_params).utm_qos_model;
    }
}

unsafe fn calc_max_hardware_v_total(stream: *const dc_stream_state) -> u32 {
    let mut v = (*(*stream).ctx).dc.caps.max_v_total;
    if (*(*stream).ctx).dc.caps.vtotal_limited_by_fp2 { v -= (*stream).timing.v_front_porch + 1; }
    v
}

unsafe fn populate_dml21_timing_config_from_stream_state(timing: *mut dml2_timing_cfg, stream: *mut dc_stream_state, pipe: *mut pipe_ctx, dml_ctx: *mut dml2_context) {
    let min_v_front_porch = if (*stream).timing.flags.INTERLACE != 0 { 2 } else { 1 };
    (*timing).h_active = (*stream).timing.h_addressable + (*stream).timing.h_border_left + (*stream).timing.h_border_right + (*pipe).dsc_padding_params.dsc_hactive_padding;
    (*timing).v_active = (*stream).timing.v_addressable + (*stream).timing.v_border_bottom + (*stream).timing.v_border_top;
    (*timing).h_front_porch = (*stream).timing.h_front_porch;
    (*timing).v_front_porch = if (*stream).timing.v_front_porch > min_v_front_porch { (*stream).timing.v_front_porch } else { min_v_front_porch };
    (*timing).pixel_clock_khz = (*stream).timing.pix_clk_100hz / 10;
    if (*pipe).dsc_padding_params.dsc_hactive_padding != 0 { (*timing).pixel_clock_khz = (*pipe).dsc_padding_params.dsc_pix_clk_100hz / 10; }
    if (*stream).timing.timing_3d_format == TIMING_3D_FORMAT_HW_FRAME_PACKING { (*timing).pixel_clock_khz *= 2; }
    (*timing).h_total = (*stream).timing.h_total + (*pipe).dsc_padding_params.dsc_htotal_padding;
    (*timing).v_total = (*stream).timing.v_total;
    (*timing).h_sync_width = (*stream).timing.h_sync_width;
    (*timing).interlaced = (*stream).timing.flags.INTERLACE != 0;
    let hblank_start = (*stream).timing.h_total - (*stream).timing.h_front_porch;
    (*timing).h_blank_end = if hblank_start < (*stream).timing.h_addressable { 0 } else { hblank_start - (*stream).timing.h_addressable - (*pipe).dsc_padding_params.dsc_hactive_padding - (*stream).timing.h_border_left - (*stream).timing.h_border_right };
    let vblank_start = (*timing).v_total - (*timing).v_front_porch;
    (*timing).v_blank_end = vblank_start - (*stream).timing.v_addressable - (*stream).timing.v_border_top - (*stream).timing.v_border_bottom;
    (*timing).drr_config.enabled = (*stream).ignore_msa_timing_param;
    (*timing).drr_config.drr_active_variable = (*stream).vrr_active_variable;
    (*timing).drr_config.drr_active_fixed = (*stream).vrr_active_fixed;
    (*timing).drr_config.disallowed = !(*stream).allow_freesync;
    let mut min_refresh = (*stream).timing.min_refresh_in_uhz;
    if (*(*stream).ctx).dc.caps.max_v_total != 0 {
        let pix = if (*pipe).dsc_padding_params.dsc_hactive_padding != 0 { (*pipe).dsc_padding_params.dsc_pix_clk_100hz } else { (*stream).timing.pix_clk_100hz };
        min_refresh = div64_u64(pix as u64 * 100000000u64, (*timing).h_total as u64 * calc_max_hardware_v_total(stream) as u64);
    }
    (*timing).drr_config.min_refresh_uhz = core::cmp::max((*stream).timing.min_refresh_in_uhz, min_refresh) as usize;
    if !(*dml_ctx).config.callbacks.get_max_flickerless_instant_vtotal_increase.is_none() && (*(*stream).ctx).dc.config.enable_fpo_flicker_detection == 1 { (*timing).drr_config.max_instant_vtotal_delta = ((*dml_ctx).config.callbacks.get_max_flickerless_instant_vtotal_increase.unwrap())(stream, false); } else { (*timing).drr_config.max_instant_vtotal_delta = 0; }
    if (*stream).timing.flags.DSC { (*timing).dsc.enable = dml2_dsc_enable; (*timing).dsc.overrides.num_slices = (*stream).timing.dsc_cfg.num_slices_h; (*timing).dsc.dsc_compressed_bpp_x16 = (*stream).timing.dsc_cfg.bits_per_pixel; } else { (*timing).dsc.enable = dml2_dsc_disable; }
    (*timing).bpc = match (*stream).timing.display_color_depth { COLOR_DEPTH_666=>6, COLOR_DEPTH_888=>8, COLOR_DEPTH_101010=>10, COLOR_DEPTH_121212=>12, COLOR_DEPTH_141414=>14, COLOR_DEPTH_161616=>16, COLOR_DEPTH_999=>9, COLOR_DEPTH_111111=>11, _=>8 };
    (*timing).vblank_nom = (*timing).v_total - (*timing).v_active;
}

unsafe fn populate_dml21_output_config_from_stream_state(output: *mut dml2_link_output_cfg, stream: *mut dc_stream_state, pipe: *const pipe_ctx) {
    (*output).output_dp_lane_count = 4;
    (*output).output_encoder = match (*stream).signal {
        SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_DISPLAY_PORT => if check_dp2p0_output_encoder(pipe) { dml2_dp2p0 } else { dml2_dp },
        SIGNAL_TYPE_EDP => dml2_edp,
        SIGNAL_TYPE_HDMI_TYPE_A | SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => dml2_hdmi,
        SIGNAL_TYPE_HDMI_FRL => dml2_hdmifrl, _ => dml2_dp,
    };
    (*output).output_format = match (*stream).timing.pixel_encoding {
        PIXEL_ENCODING_RGB | PIXEL_ENCODING_YCBCR444 => dml2_444,
        PIXEL_ENCODING_YCBCR420 => dml2_420,
        PIXEL_ENCODING_YCBCR422 => if (*stream).timing.flags.DSC && !(*stream).timing.dsc_cfg.ycbcr422_simple { dml2_n422 } else { dml2_s422 }, _ => dml2_444,
    };
    (*output).output_dp_link_rate = dml2_dp_rate_na;
    (*output).audio_sample_layout = (*stream).audio_info.modes.sample_size;
    (*output).audio_sample_rate = (*stream).audio_info.modes.max_bit_rate;
    (*output).output_disabled = true;
    // TODO: New to DML2.1: validate_output is not populated by the source.
}

unsafe fn populate_dml21_writeback_config_from_stream_state(writeback: *mut dml2_writeback_cfg, stream: *const dc_stream_state) {
    if (*stream).num_wb_info > 0 {
        (*writeback).active_writebacks_per_stream = core::cmp::min((*stream).num_wb_info, DML2_MAX_WRITEBACK);
        for i in 0..(*stream).num_wb_info {
            let src = &(*stream).writeback_info[i]; let dst = &mut (*writeback).writeback_stream[i];
            dst.pixel_format = match src.dwb_params.cnv_params.fc_out_format { DWB_OUT_FORMAT_64BPP_ARGB | DWB_OUT_FORMAT_64BPP_RGBA => dml2_444_64, _ => dml2_444_32 };
            dst.input_width = if src.dwb_params.cnv_params.crop_en { src.dwb_params.cnv_params.crop_width } else { src.dwb_params.cnv_params.src_width };
            dst.input_height = if src.dwb_params.cnv_params.crop_en { src.dwb_params.cnv_params.crop_height } else { src.dwb_params.cnv_params.src_height };
            dst.output_width = src.dwb_params.dest_width; dst.output_height = src.dwb_params.dest_height;
            dst.v_taps = core::cmp::max(src.dwb_params.scaler_taps.v_taps, 1); dst.h_taps = core::cmp::max(src.dwb_params.scaler_taps.h_taps, 1);
            dst.v_taps_chroma = core::cmp::max(src.dwb_params.scaler_taps.v_taps_c, 1); dst.h_taps_chroma = core::cmp::max(src.dwb_params.scaler_taps.h_taps_c, 1);
            dst.h_ratio = dst.input_width as f64 / dst.output_width as f64; dst.v_ratio = dst.input_height as f64 / dst.output_height as f64;
        }
    }
}

unsafe fn populate_dml21_stream_overrides_from_stream_state(desc: *mut dml2_stream_parameters, stream: *mut dc_stream_state, status: *mut dc_stream_status) {
    (*desc).overrides.odm_mode = match (*stream).debug.force_odm_combine_segments { 0=>dml2_odm_mode_auto, 1=>dml2_odm_mode_bypass, 2=>dml2_odm_mode_combine_2to1, 3=>dml2_odm_mode_combine_3to1, 4=>dml2_odm_mode_combine_4to1, _=>dml2_odm_mode_auto };
    if !(*(*stream).ctx).dc.debug.enable_single_display_2to1_odm_policy || (*stream).debug.force_odm_combine_segments > 0 { (*desc).overrides.disable_dynamic_odm = true; }
    (*desc).overrides.disable_subvp = (*(*stream).ctx).dc.debug.force_disable_subvp || (*stream).hw_cursor_req || (*status).mall_stream_config.cursor_size_limit_subvp;
}

unsafe fn gfx_addr3_to_dml2_swizzle_mode(v: swizzle_mode_addr3_values) -> dml2_swizzle_mode { match v { DC_ADDR3_SW_LINEAR=>dml2_sw_linear, DC_ADDR3_SW_256B_2D=>dml2_sw_256b_2d, DC_ADDR3_SW_4KB_2D=>dml2_sw_4kb_2d, DC_ADDR3_SW_64KB_2D=>dml2_sw_64kb_2d, DC_ADDR3_SW_256KB_2D=>dml2_sw_256kb_2d, _=>{ ASSERT!(false); dml2_sw_linear } } }
unsafe fn gfx9_to_dml2_swizzle_mode(v: swizzle_mode_values) -> dml2_swizzle_mode { match v { DC_SW_LINEAR=>dml2_sw_linear, DC_SW_256_D|DC_SW_256_R=>dml2_sw_256b_2d, DC_SW_4KB_D|DC_SW_4KB_R|DC_SW_4KB_R_X=>dml2_sw_4kb_2d, DC_SW_64KB_D|DC_SW_64KB_D_X|DC_SW_64KB_R|DC_SW_64KB_R_X=>dml2_sw_64kb_2d, _=>dml2_sw_64kb_2d } }

unsafe fn populate_dml21_dummy_surface_cfg(surface: *mut dml2_surface_cfg, stream: *const dc_stream_state) { (*surface).plane0.width=(*stream).timing.h_addressable; (*surface).plane0.height=(*stream).timing.v_addressable; (*surface).plane1.width=(*surface).plane0.width; (*surface).plane1.height=(*surface).plane0.height; (*surface).plane0.pitch=(((*surface).plane0.width+127)/128)*128; (*surface).plane1.pitch=0; (*surface).dcc.enable=false; (*surface).dcc.informative.dcc_rate_plane0=2.0; (*surface).dcc.informative.dcc_rate_plane1=2.0; (*surface).dcc.informative.fraction_of_zero_size_request_plane0=0; (*surface).dcc.informative.fraction_of_zero_size_request_plane1=0; (*surface).tiling=dml2_sw_64kb_2d; }

pub unsafe fn dml21_map_dc_state_into_dml_display_cfg(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) -> bool { let cfg=&mut (*dml_ctx).v21.display_config; cfg.gpuvm_enable=(*dml_ctx).config.gpuvm_enable; cfg.hostvm_enable=(*dml_ctx).config.hostvm_enable; cfg.minimize_det_reallocation=true; cfg.overrides.enable_subvp_implicit_pmo=true; if (*in_dc).debug.disable_unbounded_requesting { cfg.overrides.hw.force_unbounded_requesting.enable=true; cfg.overrides.hw.force_unbounded_requesting.value=false; } true }

pub unsafe fn dml21_copy_clocks_to_dc_state(ctx: *mut dml2_context, context: *mut dc_state) { let p=(*ctx).v21.mode_programming.programming; (*context).bw_ctx.bw.dcn.clk.dispclk_khz=p.min_clocks.dcn4x.dispclk_khz; (*context).bw_ctx.bw.dcn.clk.dcfclk_khz=p.min_clocks.dcn4x.active.dcfclk_khz; (*context).bw_ctx.bw.dcn.clk.dramclk_khz=p.min_clocks.dcn4x.active.uclk_khz; (*context).bw_ctx.bw.dcn.clk.fclk_khz=p.min_clocks.dcn4x.active.fclk_khz; (*context).bw_ctx.bw.dcn.clk.idle_dramclk_khz=p.min_clocks.dcn4x.idle.uclk_khz; (*context).bw_ctx.bw.dcn.clk.idle_fclk_khz=p.min_clocks.dcn4x.idle.fclk_khz; }

pub unsafe fn dml21_extract_watermark_sets(_in_dc: *const dc, watermarks: *mut dcn_watermark_set, ctx: *mut dml2_context) { let p=(*ctx).v21.mode_programming.programming; for i in 0..p.global_regs.num_watermark_sets { if i < 4 { core::ptr::copy_nonoverlapping(&p.global_regs.wm_regs[i], &mut (*watermarks).dcn4x[i], 1); } } }
pub unsafe fn dml21_map_hw_resources(ctx: *mut dml2_context) { for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ { (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id[i]=(*ctx).v21.dml_to_dc_pipe_mapping.disp_cfg_to_stream_id[i]; (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id_valid[i]=true; (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id[i]=(*ctx).v21.dml_to_dc_pipe_mapping.disp_cfg_to_plane_id[i]; (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id_valid[i]=true; } }

pub unsafe fn dml21_get_pipe_mcache_config(_context: *mut dc_state, pipe: *mut pipe_ctx, pln: *mut dml2_per_plane_programming, out: *mut dml2_pipe_configuration_descriptor) { (*out).plane0.viewport_x_start=(*pipe).plane_res.scl_data.viewport.x; (*out).plane0.viewport_width=(*pipe).plane_res.scl_data.viewport.width; (*out).plane1.viewport_x_start=(*pipe).plane_res.scl_data.viewport_c.x; (*out).plane1.viewport_width=(*pipe).plane_res.scl_data.viewport_c.width; (*out).plane1_enabled=dml21_is_plane1_enabled((*(*pln).plane_descriptor).pixel_format); }
pub unsafe fn dml21_set_dc_p_state_type(pipe: *mut pipe_ctx, prog: *mut dml2_per_stream_programming, subvp: bool) { (*pipe).p_state_type=match (*prog).uclk_pstate_method { dml2_pstate_method_vactive|dml2_pstate_method_fw_vactive_drr=>P_STATE_V_ACTIVE, dml2_pstate_method_vblank|dml2_pstate_method_fw_vblank_drr=>if subvp {P_STATE_V_BLANK_SUB_VP}else{P_STATE_V_BLANK}, dml2_pstate_method_fw_svp|dml2_pstate_method_fw_svp_drr=>P_STATE_SUB_VP, dml2_pstate_method_fw_drr=>if subvp {P_STATE_DRR_SUB_VP}else{P_STATE_FPO}, dml2_pstate_method_alternate=>P_STATE_ALT, _=>P_STATE_UNKNOWN }; }
pub unsafe fn dml21_init_min_clocks_for_dc_state(ctx: *mut dml2_context, context: *mut dc_state) { let c=&mut (*context).bw_ctx.bw.dcn.clk; let t=&(*ctx).v21.dml_init.soc_bb.clk_table; c.dispclk_khz=t.dispclk.clk_values_khz[0]; c.dppclk_khz=t.dppclk.clk_values_khz[0]; c.dcfclk_khz=t.dcfclk.clk_values_khz[0]; c.dramclk_khz=t.uclk.clk_values_khz[0]; c.fclk_khz=t.fclk.clk_values_khz[0]; c.idle_dramclk_khz=0; c.idle_fclk_khz=0; c.dcfclk_deep_sleep_khz=0; c.fclk_p_state_change_support=true; c.p_state_change_support=true; c.dtbclk_en=false; c.ref_dtbclk_khz=0; c.socclk_khz=t.socclk.clk_values_khz[0]; c.subvp_prefetch_dramclk_khz=0; c.subvp_prefetch_fclk_khz=0; c.phyclk_khz=t.phyclk.clk_values_khz[0]; c.utm_latency_ub_index=0; c.utm_nominal_bandwidth_lb_Kbps=0; c.utm_urgent_bandwidth_lb_Kbps=0; c.stutter_efficiency.base_efficiency=1; c.stutter_efficiency.low_power_efficiency=1; c.stutter_efficiency.z8_stutter_efficiency=1; c.stutter_efficiency.z8_stutter_period=100000; c.zstate_support=DCN_ZSTATE_SUPPORT_ALLOW; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
