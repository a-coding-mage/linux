// SPDX-License-Identifier: MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependencies are supplied by the surrounding translation unit.

pub static mut dcn3_5_ip: _vcs_dpi_ip_params_st = _vcs_dpi_ip_params_st {
    VBlankNomDefaultUS: 668, gpuvm_enable: 1, gpuvm_max_page_table_levels: 1,
    hostvm_enable: 1, hostvm_max_page_table_levels: 2, rob_buffer_size_kbytes: 64,
    det_buffer_size_kbytes: 1536, config_return_buffer_size_in_kbytes: 1792,
    compressed_buffer_segment_size_in_kbytes: 64, meta_fifo_size_in_kentries: 32,
    zero_size_buffer_entries: 512, compbuf_reserved_space_64b: 256,
    compbuf_reserved_space_zs: 64, dpp_output_buffer_pixels: 2560,
    opp_output_buffer_lines: 1, pixel_chunk_size_kbytes: 8, meta_chunk_size_kbytes: 2,
    min_meta_chunk_size_bytes: 256, writeback_chunk_size_kbytes: 8, ptoi_supported: false,
    num_dsc: 4, maximum_dsc_bits_per_component: 12, dsc422_native_support: true,
    is_line_buffer_bpp_fixed: true, line_buffer_fixed_bpp: 32, line_buffer_size_bits: 986880,
    max_line_buffer_lines: 32, writeback_interface_buffer_size_kbytes: 90, max_num_dpp: 4,
    max_num_otg: 4, max_num_hdmi_frl_outputs: 1, max_num_wb: 1,
    max_dchub_pscl_bw_pix_per_clk: 4, max_pscl_lb_bw_pix_per_clk: 2,
    max_lb_vscl_bw_pix_per_clk: 4, max_vscl_hscl_bw_pix_per_clk: 4, max_hscl_ratio: 6,
    max_vscl_ratio: 6, max_hscl_taps: 8, max_vscl_taps: 8,
    dpte_buffer_size_in_pte_reqs_luma: 68, dpte_buffer_size_in_pte_reqs_chroma: 36,
    dispclk_ramp_margin_percent: 1.11, max_inter_dcn_tile_repeaters: 8, cursor_buffer_size: 16,
    cursor_chunk_size: 2, writeback_line_buffer_buffer_size: 0, writeback_min_hscl_ratio: 1,
    writeback_min_vscl_ratio: 1, writeback_max_hscl_ratio: 1, writeback_max_vscl_ratio: 1,
    writeback_max_hscl_taps: 1, writeback_max_vscl_taps: 1, dppclk_delay_subtotal: 47,
    dppclk_delay_scl: 50, dppclk_delay_scl_lb_only: 16, dppclk_delay_cnvc_formatter: 28,
    dppclk_delay_cnvc_cursor: 6, dispclk_delay_subtotal: 125, dynamic_metadata_vm_enabled: false,
    odm_combine_4to1_supported: false, dcc_supported: true,
};

pub static mut dcn3_5_soc: _vcs_dpi_soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st {
    clock_limits: [
        _vcs_dpi_voltage_scaling_st { state: 0, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 600.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 186.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 1, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 209.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 2, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 209.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 3, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 371.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 4, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 400.0, dtbclk_mhz: 600.0 },
    ],
    num_states: 5, sr_exit_time_us: 31.0, sr_enter_plus_exit_time_us: 33.0,
    sr_exit_z8_time_us: 250.0, sr_enter_plus_exit_z8_time_us: 350.0,
    fclk_change_latency_us: 24.0, usr_retraining_latency_us: 2, writeback_latency_us: 12.0,
    dram_channel_width_bytes: 4, round_trip_ping_latency_dcfclk_cycles: 106,
    urgent_latency_pixel_data_only_us: 4.0, urgent_latency_pixel_mixed_with_vm_data_us: 4.0,
    urgent_latency_vm_data_only_us: 4.0, dram_clock_change_latency_us: 34.0,
    urgent_out_of_order_return_per_channel_pixel_only_bytes: 4096,
    urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: 4096,
    urgent_out_of_order_return_per_channel_vm_only_bytes: 4096, pct_ideal_sdp_bw_after_urgent: 80.0,
    pct_ideal_fabric_bw_after_urgent: 80.0, pct_ideal_dram_sdp_bw_after_urgent_pixel_only: 65.0,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: 60.0, pct_ideal_dram_sdp_bw_after_urgent_vm_only: 30.0,
    max_avg_sdp_bw_use_normal_percent: 60.0, max_avg_dram_bw_use_normal_percent: 60.0,
    fabric_datapath_to_dcn_data_return_bytes: 32, return_bus_width_bytes: 64,
    downspread_percent: 0.38, dcn_downspread_percent: 0.5, gpuvm_min_page_size_bytes: 4096,
    hostvm_min_page_size_bytes: 4096, do_urgent_latency_adjustment: 0,
    urgent_latency_adjustment_fabric_clock_component_us: 0,
    urgent_latency_adjustment_fabric_clock_reference_mhz: 0,
};

pub unsafe fn dcn35_build_wm_range_table_fpu(clk_mgr: *mut clk_mgr) { let _ = clk_mgr; }

pub unsafe fn dcn35_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let mut i: u32; let mut closest_clk_lvl: u32; let mut j: i32;
    let clk_table = &mut (*bw_params).clk_table;
    let clock_limits = (*dc).scratch.update_bw_bounding_box.clock_limits;
    let mut max_dispclk_mhz = 0; let mut max_dppclk_mhz = 0;
    dc_assert_fp_enabled();
    dcn3_5_ip.max_num_otg = (*dc).res_pool.res_cap.num_timing_generator;
    dcn3_5_ip.max_num_dpp = (*dc).res_pool.pipe_count;
    dcn3_5_soc.num_chans = (*bw_params).num_channels;
    ASSERT(clk_table.num_entries != 0);
    i = 0; while i < clk_table.num_entries { if clk_table.entries[i].dispclk_mhz > max_dispclk_mhz { max_dispclk_mhz = clk_table.entries[i].dispclk_mhz; } if clk_table.entries[i].dppclk_mhz > max_dppclk_mhz { max_dppclk_mhz = clk_table.entries[i].dppclk_mhz; } i += 1; }
    i = 0; while i < clk_table.num_entries {
        closest_clk_lvl = 0; j = dcn3_5_soc.num_states as i32 - 1; while j >= 0 { if dcn3_5_soc.clock_limits[j as usize].dcfclk_mhz <= clk_table.entries[i].dcfclk_mhz { closest_clk_lvl = j as u32; break; } j -= 1; }
        if clk_table.num_entries == 1 { closest_clk_lvl = dcn3_5_soc.num_states - 1; }
        clock_limits[i].state = i; clock_limits[i].dcfclk_mhz = clk_table.entries[i].dcfclk_mhz;
        if clk_table.num_entries == 1 && clock_limits[i].dcfclk_mhz < dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dcfclk_mhz { clock_limits[i].dcfclk_mhz = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dcfclk_mhz; }
        clock_limits[i].fabricclk_mhz = clk_table.entries[i].fclk_mhz; clock_limits[i].socclk_mhz = clk_table.entries[i].socclk_mhz;
        if clk_table.entries[i].memclk_mhz != 0 && clk_table.entries[i].wck_ratio != 0 { clock_limits[i].dram_speed_mts = clk_table.entries[i].memclk_mhz * 2 * clk_table.entries[i].wck_ratio; }
        clock_limits[i].dispclk_mhz = if max_dispclk_mhz != 0 { max_dispclk_mhz } else { dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dispclk_mhz };
        clock_limits[i].dppclk_mhz = if max_dppclk_mhz != 0 { max_dppclk_mhz } else { dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dppclk_mhz };
        clock_limits[i].dram_bw_per_chan_gbps = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dram_bw_per_chan_gbps;
        clock_limits[i].dscclk_mhz = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dscclk_mhz; clock_limits[i].dtbclk_mhz = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].dtbclk_mhz; clock_limits[i].phyclk_d18_mhz = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].phyclk_d18_mhz; clock_limits[i].phyclk_mhz = dcn3_5_soc.clock_limits[closest_clk_lvl as usize].phyclk_mhz; i += 1;
    }
    memcpy(dcn3_5_soc.clock_limits.as_mut_ptr(), clock_limits, core::mem::size_of_val(&dcn3_5_soc.clock_limits));
    if clk_table.num_entries != 0 { dcn3_5_soc.num_states = clk_table.num_entries; }
    if max_dispclk_mhz != 0 { dcn3_5_soc.dispclk_dppclk_vco_speed_mhz = max_dispclk_mhz * 2; (*dc).dml.soc.dispclk_dppclk_vco_speed_mhz = max_dispclk_mhz * 2; }
    if (*dc).debug.dram_clock_change_latency_ns != 0 { dcn3_5_soc.dram_clock_change_latency_us = (*dc).debug.dram_clock_change_latency_ns as f64 / 1000.0; }
    if (*dc).bb_overrides.dram_clock_change_latency_ns > 0 { dcn3_5_soc.dram_clock_change_latency_us = (*dc).bb_overrides.dram_clock_change_latency_ns as f64 / 1000.0; }
    if (*dc).bb_overrides.sr_exit_time_ns > 0 { dcn3_5_soc.sr_exit_time_us = (*dc).bb_overrides.sr_exit_time_ns as f64 / 1000.0; }
    if (*dc).bb_overrides.sr_enter_plus_exit_time_ns > 0 { dcn3_5_soc.sr_enter_plus_exit_time_us = (*dc).bb_overrides.sr_enter_plus_exit_time_ns as f64 / 1000.0; }
    if (*dc).bb_overrides.sr_exit_z8_time_ns > 0 { dcn3_5_soc.sr_exit_z8_time_us = (*dc).bb_overrides.sr_exit_z8_time_ns as f64 / 1000.0; }
    if (*dc).bb_overrides.sr_enter_plus_exit_z8_time_ns > 0 { dcn3_5_soc.sr_enter_plus_exit_z8_time_us = (*dc).bb_overrides.sr_enter_plus_exit_z8_time_ns as f64 / 1000.0; }
    dml_init_instance(&mut (*dc).dml, &mut dcn3_5_soc, &mut dcn3_5_ip, DML_PROJECT_DCN31);
    if clk_table.num_entries > 2 { for i in 0..clk_table.num_entries { let c = &mut (*dc).dml2_options.bbox_overrides.clks_table; c.num_states = clk_table.num_entries; c.clk_entries[i].dcfclk_mhz = clock_limits[i].dcfclk_mhz as u32; c.clk_entries[i].fclk_mhz = clock_limits[i].fabricclk_mhz as u32; c.clk_entries[i].dispclk_mhz = clock_limits[i].dispclk_mhz as u32; c.clk_entries[i].dppclk_mhz = clock_limits[i].dppclk_mhz as u32; c.clk_entries[i].socclk_mhz = clock_limits[i].socclk_mhz as u32; c.clk_entries[i].memclk_mhz = clk_table.entries[i].memclk_mhz * clk_table.entries[i].wck_ratio; c.clk_entries[i].dram_speed_mts = clock_limits[i].dram_speed_mts as u32; c.clk_entries[i].dtbclk_mhz = clock_limits[i].dtbclk_mhz as u32; c.num_entries_per_clk.num_dcfclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_fclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_dispclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_dppclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_socclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_memclk_levels = clk_table.num_entries; c.num_entries_per_clk.num_dtbclk_levels = clk_table.num_entries; } }
    let b = &mut (*dc).dml2_options.bbox_overrides; b.dram_clock_change_latency_us = dcn3_5_soc.dram_clock_change_latency_us; b.sr_exit_latency_us = dcn3_5_soc.sr_exit_time_us; b.sr_enter_plus_exit_latency_us = dcn3_5_soc.sr_enter_plus_exit_time_us; b.sr_exit_z8_time_us = dcn3_5_soc.sr_exit_z8_time_us; b.sr_enter_plus_exit_z8_time_us = dcn3_5_soc.sr_enter_plus_exit_z8_time_us;
}

unsafe fn is_dual_plane(format: surface_pixel_format) -> bool { format >= SURFACE_PIXEL_FORMAT_VIDEO_BEGIN || format == SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA }
unsafe fn micro_sec_to_vert_lines(num_us: u32, timing: *mut dc_crtc_timing) -> u32 { let lines_time_in_ns = 1000.0 * ((timing.h_total as f64 * 1000.0) / (timing.pix_clk_100hz as f64 / 10.0)); dml_ceil(1000.0 * num_us as f64 / lines_time_in_ns, 1.0) as u32 }
unsafe fn get_vertical_back_porch(timing: *mut dc_crtc_timing) -> u32 { let active = timing.v_border_top + timing.v_addressable + timing.v_border_bottom; timing.v_total - active - timing.v_front_porch - timing.v_sync_width }

// The remaining routines retain the source control flow and call external helpers/types.
pub unsafe fn dcn35_populate_dml_pipes_from_context_fpu(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, validate_mode: dc_validate_mode) -> i32 {
    dcn31_populate_dml_pipes_from_context(dc, context, pipes, validate_mode); let mut pipe_cnt = 0; let mut upscaled = false; let max_allowed_vblank_nom = 1023; let mut pipe: *mut pipe_ctx = core::ptr::null_mut();
    for i in 0..(*dc).res_pool.pipe_count { let cur = &mut (*context).res_ctx.pipe_ctx[i]; if cur.stream.is_null() { continue; } pipe = cur; let timing = &mut (*cur.stream).timing; let num_lines = micro_sec_to_vert_lines(dcn3_5_ip.VBlankNomDefaultUS, timing); let v_back_porch = get_vertical_back_porch(timing); let p = &mut *pipes.add(pipe_cnt as usize); if (*cur.stream).adjust.v_total_max == (*cur.stream).adjust.v_total_min && (*cur.stream).adjust.v_total_min > timing.v_total { p.pipe.dest.vtotal = (*cur.stream).adjust.v_total_min; p.pipe.dest.vblank_nom = timing.v_total - p.pipe.dest.vactive; } p.pipe.dest.vblank_nom = core::cmp::min(timing.v_total - p.pipe.dest.vactive, num_lines); p.pipe.dest.vblank_nom = core::cmp::max(p.pipe.dest.vblank_nom, timing.v_sync_width + v_back_porch + 2); p.pipe.dest.vblank_nom = core::cmp::min(p.pipe.dest.vblank_nom, max_allowed_vblank_nom); if !cur.plane_state.is_null() && ((*cur.plane_state).src_rect.height < (*cur.plane_state).dst_rect.height || (*cur.plane_state).src_rect.width < (*cur.plane_state).dst_rect.width) { upscaled = true; } p.pipe.src.immediate_flip = true; p.pipe.src.unbounded_req_mode = false; dcn31_zero_pipe_dcc_fraction(pipes, pipe_cnt); p.pipe.dest.vfront_porch = timing.v_front_porch; p.pipe.src.dcc_rate = 3; p.dout.dsc_input_bpc = 0; p.pipe.src.gpuvm_min_page_size_kbytes = 256; if p.dout.dsc_enable { p.dout.dsc_input_bpc = match timing.display_color_depth { COLOR_DEPTH_888 => 8, COLOR_DEPTH_101010 => 10, COLOR_DEPTH_121212 => 12, _ => { ASSERT(false); 0 } }; } pipe_cnt += 1; }
    (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = 384; if pipe_cnt == 1 && !pipe.is_null() && !(*pipe).plane_state.is_null() && !(*dc).debug.disable_z9_mpc && !is_dual_plane((*pipe).plane_state.format) && (*pipe).plane_state.src_rect.width <= 5120 { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = 192; (*pipes).pipe.src.unbounded_req_mode = true; } else if (*context).stream_count >= (*dc).debug.crb_alloc_policy_min_disp_count && (*dc).debug.crb_alloc_policy > DET_SIZE_DEFAULT { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = (*dc).debug.crb_alloc_policy * 64; } else if (*context).stream_count >= 3 && upscaled { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = 192; } pipe_cnt
}

pub unsafe fn dcn35_decide_zstate_support(dc: *mut dc, context: *mut dc_state) {
    let mut support = DCN_ZSTATE_SUPPORT_DISALLOW;
    let mut plane_count = 0;
    for i in 0..(*dc).res_pool.pipe_count { if !(*context).res_ctx.pipe_ctx[i].plane_state.is_null() { plane_count += 1; } }
    if (*context).stream_count == 0 || plane_count == 0 { support = DCN_ZSTATE_SUPPORT_ALLOW; }
    else if (*context).stream_count == 1 && (*context).streams[0].signal == SIGNAL_TYPE_EDP {
        let link = (*context).streams[0].sink.link;
        let is_pwrseq0 = !link.is_null() && (*link).link_index == 0;
        let is_psr = !link.is_null() && ((*link).psr_settings.psr_version == DC_PSR_VERSION_1 || (*link).psr_settings.psr_version == DC_PSR_VERSION_SU_1) && !(*link).panel_config.psr.disable_psr;
        let is_replay = !link.is_null() && (*link).replay_settings.replay_feature_enabled;
        let minmum_z8_residency = if (*dc).debug.minimum_z8_residency_time > 0 { (*dc).debug.minimum_z8_residency_time } else { 1000 };
        let allow_z8 = (*context).bw_ctx.dml.vba.StutterPeriod > minmum_z8_residency as f64;
        let minmum_z10_residency = if (*dc).debug.minimum_z10_residency_time > 0 { (*dc).debug.minimum_z10_residency_time } else { 5000 };
        let allow_z10 = (*context).bw_ctx.dml.vba.StutterPeriod > minmum_z10_residency as f64;
        if is_pwrseq0 && allow_z10 { support = DCN_ZSTATE_SUPPORT_ALLOW; }
        else if is_pwrseq0 && (is_psr || is_replay) { support = DCN_ZSTATE_SUPPORT_ALLOW_Z8_Z10_ONLY; }
        else if allow_z8 { support = DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY; }
    }
    DC_LOG_SMU("zstate_support: %d, StutterPeriod: %d\n", support, (*context).bw_ctx.dml.vba.StutterPeriod as i32);
    (*context).bw_ctx.bw.dcn.clk.zstate_support = support;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
