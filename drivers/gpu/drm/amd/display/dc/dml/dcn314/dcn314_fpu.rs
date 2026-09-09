// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C dependencies supplied by the surrounding translation unit.

pub static mut dcn3_14_ip: _vcs_dpi_ip_params_st = _vcs_dpi_ip_params_st {
    VBlankNomDefaultUS: 668, gpuvm_enable: 1, gpuvm_max_page_table_levels: 1,
    hostvm_enable: 1, hostvm_max_page_table_levels: 2, rob_buffer_size_kbytes: 64,
    det_buffer_size_kbytes: DCN3_14_DEFAULT_DET_SIZE, config_return_buffer_size_in_kbytes: 1792,
    compressed_buffer_segment_size_in_kbytes: 64, meta_fifo_size_in_kentries: 32,
    zero_size_buffer_entries: 512, compbuf_reserved_space_64b: 256, compbuf_reserved_space_zs: 64,
    dpp_output_buffer_pixels: 2560, opp_output_buffer_lines: 1, pixel_chunk_size_kbytes: 8,
    meta_chunk_size_kbytes: 2, min_meta_chunk_size_bytes: 256, writeback_chunk_size_kbytes: 8,
    ptoi_supported: false, num_dsc: 4, maximum_dsc_bits_per_component: 10, dsc422_native_support: false,
    is_line_buffer_bpp_fixed: true, line_buffer_fixed_bpp: 48, line_buffer_size_bits: 789504,
    max_line_buffer_lines: 12, writeback_interface_buffer_size_kbytes: 90, max_num_dpp: 4,
    max_num_otg: 4, max_num_hdmi_frl_outputs: 1, max_num_wb: 1, max_dchub_pscl_bw_pix_per_clk: 4,
    max_pscl_lb_bw_pix_per_clk: 2, max_lb_vscl_bw_pix_per_clk: 4, max_vscl_hscl_bw_pix_per_clk: 4,
    max_hscl_ratio: 6, max_vscl_ratio: 6, max_hscl_taps: 8, max_vscl_taps: 8,
    dpte_buffer_size_in_pte_reqs_luma: 64, dpte_buffer_size_in_pte_reqs_chroma: 34,
    dispclk_ramp_margin_percent: 1, max_inter_dcn_tile_repeaters: 8, cursor_buffer_size: 16,
    cursor_chunk_size: 2, writeback_line_buffer_buffer_size: 0, writeback_min_hscl_ratio: 1,
    writeback_min_vscl_ratio: 1, writeback_max_hscl_ratio: 1, writeback_max_vscl_ratio: 1,
    writeback_max_hscl_taps: 1, writeback_max_vscl_taps: 1, dppclk_delay_subtotal: 46,
    dppclk_delay_scl: 50, dppclk_delay_scl_lb_only: 16, dppclk_delay_cnvc_formatter: 27,
    dppclk_delay_cnvc_cursor: 6, dispclk_delay_subtotal: 119, dynamic_metadata_vm_enabled: false,
    odm_combine_4to1_supported: false, dcc_supported: true,
};

static mut dcn3_14_soc: _vcs_dpi_soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st {
    // TODO: correct dispclk/dppclk voltage level determination
    clock_limits: [
        _vcs_dpi_voltage_scaling_st { state: 0, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 600.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 186.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 1, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 209.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 2, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 209.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 3, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 371.0, dtbclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 4, dispclk_mhz: 1200.0, dppclk_mhz: 1200.0, phyclk_mhz: 810.0, phyclk_d18_mhz: 667.0, dscclk_mhz: 417.0, dtbclk_mhz: 600.0 },
    ], num_states: 5, sr_exit_time_us: 16.5, sr_enter_plus_exit_time_us: 18.5,
    sr_exit_z8_time_us: 268.0, sr_enter_plus_exit_z8_time_us: 393.0, writeback_latency_us: 12.0,
    dram_channel_width_bytes: 4, round_trip_ping_latency_dcfclk_cycles: 106,
    urgent_latency_pixel_data_only_us: 4.0, urgent_latency_pixel_mixed_with_vm_data_us: 4.0,
    urgent_latency_vm_data_only_us: 4.0, urgent_out_of_order_return_per_channel_pixel_only_bytes: 4096,
    urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: 4096,
    urgent_out_of_order_return_per_channel_vm_only_bytes: 4096, pct_ideal_sdp_bw_after_urgent: 80.0,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_only: 65.0, pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: 60.0,
    pct_ideal_dram_sdp_bw_after_urgent_vm_only: 30.0, max_avg_sdp_bw_use_normal_percent: 60.0,
    max_avg_dram_bw_use_normal_percent: 60.0, fabric_datapath_to_dcn_data_return_bytes: 32,
    return_bus_width_bytes: 64, downspread_percent: 0.38, dcn_downspread_percent: 0.5,
    gpuvm_min_page_size_bytes: 4096, hostvm_min_page_size_bytes: 4096, do_urgent_latency_adjustment: false,
    urgent_latency_adjustment_fabric_clock_component_us: 0, urgent_latency_adjustment_fabric_clock_reference_mhz: 0,
};

fn is_dual_plane(format: enum_surface_pixel_format) -> bool {
    format >= SURFACE_PIXEL_FORMAT_VIDEO_BEGIN || format == SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA
}

/* Converts microseconds to vertical lines, rounding up to guarantee the interval. */
unsafe fn micro_sec_to_vert_lines(num_us: u32, timing: *mut dc_crtc_timing) -> u32 {
    let lines_time_in_ns = 1000.0 * ((*timing).h_total as f64 * 1000.0 / ((*timing).pix_clk_100hz as f64 / 10.0));
    dml_ceil(1000.0 * num_us as f64 / lines_time_in_ns, 1.0) as u32
}

unsafe fn get_vertical_back_porch(timing: *mut dc_crtc_timing) -> u32 {
    let v_active = (*timing).v_border_top + (*timing).v_addressable + (*timing).v_border_bottom;
    let v_blank = (*timing).v_total - v_active;
    v_blank - (*timing).v_front_porch - (*timing).v_sync_width
}

pub unsafe fn dcn314_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let clk_table = &mut (*bw_params).clk_table;
    let clock_limits = dcn3_14_soc.clock_limits.as_mut_ptr();
    let mut max_dispclk_mhz = 0;
    let mut max_dppclk_mhz = 0;
    dc_assert_fp_enabled();
    if !(*dc).config.use_default_clock_table {
        dcn3_14_ip.max_num_otg = (*(*dc).res_pool).res_cap.num_timing_generator;
        dcn3_14_ip.max_num_dpp = (*dc).res_pool.pipe_count;
        if (*bw_params).dram_channel_width_bytes > 0 { dcn3_14_soc.dram_channel_width_bytes = (*bw_params).dram_channel_width_bytes; }
        if (*bw_params).num_channels > 0 { dcn3_14_soc.num_chans = (*bw_params).num_channels; }
        ASSERT!(dcn3_14_soc.num_chans); ASSERT!(clk_table.num_entries);
        for i in 0..clk_table.num_entries { max_dispclk_mhz = max_dispclk_mhz.max(clk_table.entries[i].dispclk_mhz); max_dppclk_mhz = max_dppclk_mhz.max(clk_table.entries[i].dppclk_mhz); }
        for i in 0..clk_table.num_entries {
            let mut closest_clk_lvl = 0; let mut j = dcn3_14_soc.num_states as i32 - 1;
            while j >= 0 { if dcn3_14_soc.clock_limits[j as usize].dcfclk_mhz as u32 <= clk_table.entries[i].dcfclk_mhz { closest_clk_lvl = j as usize; break; } j -= 1; }
            if clk_table.num_entries == 1 { closest_clk_lvl = dcn3_14_soc.num_states - 1; }
            let e = &clk_table.entries[i]; let l = &mut *clock_limits.add(i); l.state = i; l.dcfclk_mhz = e.dcfclk_mhz;
            if clk_table.num_entries == 1 && l.dcfclk_mhz < dcn3_14_soc.clock_limits[closest_clk_lvl].dcfclk_mhz { l.dcfclk_mhz = dcn3_14_soc.clock_limits[closest_clk_lvl].dcfclk_mhz; }
            l.fabricclk_mhz = e.fclk_mhz; l.socclk_mhz = e.socclk_mhz;
            if e.memclk_mhz != 0 && e.wck_ratio != 0 { l.dram_speed_mts = e.memclk_mhz * 2 * e.wck_ratio; }
            l.dispclk_mhz = if max_dispclk_mhz != 0 { max_dispclk_mhz } else { dcn3_14_soc.clock_limits[closest_clk_lvl].dispclk_mhz };
            l.dppclk_mhz = if max_dppclk_mhz != 0 { max_dppclk_mhz } else { dcn3_14_soc.clock_limits[closest_clk_lvl].dppclk_mhz };
            l.dram_bw_per_chan_gbps = dcn3_14_soc.clock_limits[closest_clk_lvl].dram_bw_per_chan_gbps; l.dscclk_mhz = dcn3_14_soc.clock_limits[closest_clk_lvl].dscclk_mhz; l.dtbclk_mhz = dcn3_14_soc.clock_limits[closest_clk_lvl].dtbclk_mhz; l.phyclk_d18_mhz = dcn3_14_soc.clock_limits[closest_clk_lvl].phyclk_d18_mhz; l.phyclk_mhz = dcn3_14_soc.clock_limits[closest_clk_lvl].phyclk_mhz;
        }
        for i in 0..clk_table.num_entries { dcn3_14_soc.clock_limits[i] = *clock_limits.add(i); }
        if clk_table.num_entries != 0 { dcn3_14_soc.num_states = clk_table.num_entries; }
    }
    if max_dispclk_mhz != 0 { dcn3_14_soc.dispclk_dppclk_vco_speed_mhz = max_dispclk_mhz * 2; (*dc).dml.soc.dispclk_dppclk_vco_speed_mhz = max_dispclk_mhz * 2; }
    dcn20_patch_bounding_box(dc, &mut dcn3_14_soc); dml_init_instance(&mut (*dc).dml, &mut dcn3_14_soc, &mut dcn3_14_ip, DML_PROJECT_DCN314);
}

pub unsafe fn dcn314_populate_dml_pipes_from_context_fpu(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, validate_mode: enum_dc_validate_mode) -> i32 {
    let res_ctx = &mut (*context).res_ctx; let mut pipe: *mut pipe_ctx = core::ptr::null_mut(); let mut upscaled = false; let max_allowed_vblank_nom = 1023; let mut pipe_cnt = 0;
    dc_assert_fp_enabled(); dcn31x_populate_dml_pipes_from_context(dc, context, pipes, validate_mode);
    for i in 0..(*dc).res_pool.pipe_count { let cur = &mut res_ctx.pipe_ctx[i]; if cur.stream.is_null() { continue; } pipe = cur; let timing = &mut (*cur.stream).timing; let num_lines = micro_sec_to_vert_lines(dcn3_14_ip.VBlankNomDefaultUS, timing);
        let p = &mut *pipes.add(pipe_cnt as usize); p.pipe.dest.vtotal = if (*cur.stream).adjust.v_total_min != 0 { (*cur.stream).adjust.v_total_min } else { timing.v_total }; let v_back_porch = get_vertical_back_porch(timing);
        p.pipe.dest.vblank_nom = min(timing.v_total - p.pipe.dest.vactive, num_lines); p.pipe.dest.vblank_nom = max(p.pipe.dest.vblank_nom, timing.v_sync_width + v_back_porch + 2); p.pipe.dest.vblank_nom = min(p.pipe.dest.vblank_nom, max_allowed_vblank_nom);
        if !cur.plane_state.is_null() && ((*cur.plane_state).src_rect.height < (*cur.plane_state).dst_rect.height || (*cur.plane_state).src_rect.width < (*cur.plane_state).dst_rect.width) { upscaled = true; }
        if (*dc).debug.dml_hostvm_override == DML_HOSTVM_NO_OVERRIDE { p.pipe.src.hostvm = (*dc).vm_pa_config.is_hvm_enabled || (*(*dc).res_pool).hubbub.riommu_active; }
        p.pipe.src.immediate_flip = true; p.pipe.src.unbounded_req_mode = false; p.pipe.src.dcc_fraction_of_zs_req_luma = 0; p.pipe.src.dcc_fraction_of_zs_req_chroma = 0; p.pipe.dest.vfront_porch = timing.v_front_porch; p.pipe.src.dcc_rate = 3; p.dout.dsc_input_bpc = 0;
        if p.dout.dsc_enable { p.dout.dsc_input_bpc = match timing.display_color_depth { COLOR_DEPTH_888 => 8, COLOR_DEPTH_101010 => 10, COLOR_DEPTH_121212 => 12, _ => { ASSERT!(false); 0 } }; }
        pipe_cnt += 1;
    }
    (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = DCN3_14_DEFAULT_DET_SIZE;
    if pipe_cnt == 1 && !pipe.is_null() && !(*pipe).plane_state.is_null() && (*pipe).plane_state.rotation == ROTATION_ANGLE_0 && !(*dc).debug.disable_z9_mpc { if !is_dual_plane((*pipe).plane_state.format) && (*pipe).plane_state.src_rect.width <= 5120 { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = 192; (*pipes).pipe.src.unbounded_req_mode = true; } }
    else if (*context).stream_count >= (*dc).debug.crb_alloc_policy_min_disp_count && (*dc).debug.crb_alloc_policy > DET_SIZE_DEFAULT { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = (*dc).debug.crb_alloc_policy * 64; }
    else if (*context).stream_count >= 3 && upscaled { (*context).bw_ctx.dml.ip.det_buffer_size_kbytes = 192; }
    if (*dc).debug.force_odm_combine_4to1 { (*context).bw_ctx.dml.ip.odm_combine_4to1_supported = true; }
    for i in 0..(*dc).res_pool.pipe_count { let cur = &mut res_ctx.pipe_ctx[i]; if cur.stream.is_null() { continue; } if (*cur.stream).signal == SIGNAL_TYPE_EDP && (*dc).debug.seamless_boot_odm_combine && (*cur.stream).apply_seamless_boot_optimization && (*cur.stream).apply_boot_odm_mode == dm_odm_combine_policy_2to1 { (*context).bw_ctx.dml.vba.ODMCombinePolicy = dm_odm_combine_policy_2to1; break; } }
    pipe_cnt
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
