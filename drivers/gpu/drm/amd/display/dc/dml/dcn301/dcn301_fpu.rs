/*
 * Copyright 2019-2021 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External declarations and types are supplied by the surrounding kernel translation.

#[allow(non_upper_case_globals)]
pub static mut dcn3_01_ip: _vcs_dpi_ip_params_st = _vcs_dpi_ip_params_st {
    odm_capable: 1, gpuvm_enable: 1, hostvm_enable: 1,
    gpuvm_max_page_table_levels: 1, hostvm_max_page_table_levels: 2,
    hostvm_cached_page_table_levels: 0, pte_group_size_bytes: 2048,
    num_dsc: 3, rob_buffer_size_kbytes: 184, det_buffer_size_kbytes: 184,
    dpte_buffer_size_in_pte_reqs_luma: 64, dpte_buffer_size_in_pte_reqs_chroma: 32,
    pde_proc_buffer_size_64k_reqs: 48, dpp_output_buffer_pixels: 2560,
    opp_output_buffer_lines: 1, pixel_chunk_size_kbytes: 8, meta_chunk_size_kbytes: 2,
    writeback_chunk_size_kbytes: 8, line_buffer_size_bits: 789504,
    is_line_buffer_bpp_fixed: 0, line_buffer_fixed_bpp: 48, dcc_supported: true,
    writeback_interface_buffer_size_kbytes: 90, writeback_line_buffer_buffer_size: 656640,
    max_line_buffer_lines: 12, writeback_luma_buffer_size_kbytes: 12,
    writeback_chroma_buffer_size_kbytes: 8, writeback_chroma_line_buffer_width_pixels: 4,
    writeback_max_hscl_ratio: 1, writeback_max_vscl_ratio: 1, writeback_min_hscl_ratio: 1,
    writeback_min_vscl_ratio: 1, writeback_max_hscl_taps: 1, writeback_max_vscl_taps: 1,
    writeback_line_buffer_luma_buffer_size: 0, writeback_line_buffer_chroma_buffer_size: 14643,
    cursor_buffer_size: 8, cursor_chunk_size: 2, max_num_otg: 4, max_num_dpp: 4,
    max_num_wb: 1, max_dchub_pscl_bw_pix_per_clk: 4, max_pscl_lb_bw_pix_per_clk: 2,
    max_lb_vscl_bw_pix_per_clk: 4, max_vscl_hscl_bw_pix_per_clk: 4, max_hscl_ratio: 6,
    max_vscl_ratio: 6, hscl_mults: 4, vscl_mults: 4, max_hscl_taps: 8, max_vscl_taps: 8,
    dispclk_ramp_margin_percent: 1, underscan_factor: 1.11, min_vblank_lines: 32,
    dppclk_delay_subtotal: 46, dynamic_metadata_vm_enabled: true, dppclk_delay_scl_lb_only: 16,
    dppclk_delay_scl: 50, dppclk_delay_cnvc_formatter: 27, dppclk_delay_cnvc_cursor: 6,
    dispclk_delay_subtotal: 119, dcfclk_cstate_latency: 5.2, max_inter_dcn_tile_repeaters: 8,
    max_num_hdmi_frl_outputs: 0, odm_combine_4to1_supported: true, xfc_supported: false,
    xfc_fill_bw_overhead_percent: 10.0, xfc_fill_constant_bytes: 0,
    gfx7_compat_tiling_supported: 0, number_of_cursors: 1,
};

#[allow(non_upper_case_globals)]
pub static mut dcn3_01_soc: _vcs_dpi_soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st {
    clock_limits: [
        _vcs_dpi_voltage_scaling_st { state: 0, dram_speed_mts: 2400.0, fabricclk_mhz: 600, socclk_mhz: 278.0, dcfclk_mhz: 400.0, dscclk_mhz: 206.0, dppclk_mhz: 1015.0, dispclk_mhz: 1015.0, phyclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 1, dram_speed_mts: 2400.0, fabricclk_mhz: 688, socclk_mhz: 278.0, dcfclk_mhz: 400.0, dscclk_mhz: 206.0, dppclk_mhz: 1015.0, dispclk_mhz: 1015.0, phyclk_mhz: 600.0 },
        _vcs_dpi_voltage_scaling_st { state: 2, dram_speed_mts: 4267.0, fabricclk_mhz: 1067, socclk_mhz: 278.0, dcfclk_mhz: 608.0, dscclk_mhz: 296.0, dppclk_mhz: 1015.0, dispclk_mhz: 1015.0, phyclk_mhz: 810.0 },
        _vcs_dpi_voltage_scaling_st { state: 3, dram_speed_mts: 4267.0, fabricclk_mhz: 1067, socclk_mhz: 715.0, dcfclk_mhz: 676.0, dscclk_mhz: 338.0, dppclk_mhz: 1015.0, dispclk_mhz: 1015.0, phyclk_mhz: 810.0 },
        _vcs_dpi_voltage_scaling_st { state: 4, dram_speed_mts: 4267.0, fabricclk_mhz: 1067, socclk_mhz: 953.0, dcfclk_mhz: 810.0, dscclk_mhz: 338.0, dppclk_mhz: 1015.0, dispclk_mhz: 1015.0, phyclk_mhz: 810.0 },
    ],
    sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0, urgent_latency_us: 4.0,
    urgent_latency_pixel_data_only_us: 4.0, urgent_latency_pixel_mixed_with_vm_data_us: 4.0,
    urgent_latency_vm_data_only_us: 4.0, urgent_out_of_order_return_per_channel_pixel_only_bytes: 4096,
    urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: 4096,
    urgent_out_of_order_return_per_channel_vm_only_bytes: 4096,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_only: 80.0, pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: 75.0,
    pct_ideal_dram_sdp_bw_after_urgent_vm_only: 40.0, max_avg_sdp_bw_use_normal_percent: 60.0,
    max_avg_dram_bw_use_normal_percent: 60.0, writeback_latency_us: 12.0, max_request_size_bytes: 256,
    dram_channel_width_bytes: 4, fabric_datapath_to_dcn_data_return_bytes: 32, dcn_downspread_percent: 0.5,
    downspread_percent: 0.38, dram_page_open_time_ns: 50.0, dram_rw_turnaround_time_ns: 17.5,
    dram_return_buffer_per_channel_bytes: 8192, round_trip_ping_latency_dcfclk_cycles: 191,
    urgent_out_of_order_return_per_channel_bytes: 4096, channel_interleave_bytes: 256, num_banks: 8,
    num_chans: 4, gpuvm_min_page_size_bytes: 4096, hostvm_min_page_size_bytes: 4096,
    dram_clock_change_latency_us: 23.84, writeback_dram_clock_change_latency_us: 23.0,
    return_bus_width_bytes: 64, dispclk_dppclk_vco_speed_mhz: 3550, xfc_bus_transport_time_us: 20,
    xfc_xbuf_latency_tolerance_us: 4, use_urgent_burst_bw: 1, num_states: 5,
    do_urgent_latency_adjustment: false, urgent_latency_adjustment_fabric_clock_component_us: 0,
    urgent_latency_adjustment_fabric_clock_reference_mhz: 0,
};

pub static mut ddr4_wm_table: wm_table = wm_table { entries: [
    wm_range_table_entry { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 6.09, sr_enter_plus_exit_time_us: 7.14, valid: true },
    wm_range_table_entry { wm_inst: WM_B, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 10.12, sr_enter_plus_exit_time_us: 11.48, valid: true },
    wm_range_table_entry { wm_inst: WM_C, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 10.12, sr_enter_plus_exit_time_us: 11.48, valid: true },
    wm_range_table_entry { wm_inst: WM_D, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 10.12, sr_enter_plus_exit_time_us: 11.48, valid: true },
]};

pub static mut lpddr5_wm_table: wm_table = wm_table { entries: [
    wm_range_table_entry { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 13.5, sr_enter_plus_exit_time_us: 16.5, valid: true },
    wm_range_table_entry { wm_inst: WM_B, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 13.5, sr_enter_plus_exit_time_us: 16.5, valid: true },
    wm_range_table_entry { wm_inst: WM_C, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 13.5, sr_enter_plus_exit_time_us: 16.5, valid: true },
    wm_range_table_entry { wm_inst: WM_D, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 13.5, sr_enter_plus_exit_time_us: 16.5, valid: true },
]};

unsafe fn calculate_wm_set_for_vlevel(vlevel: i32, table_entry: *mut wm_range_table_entry, wm_set: *mut dcn_watermarks, dml: *mut display_mode_lib, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32) {
    let dram_clock_change_latency_cached = (*dml).soc.dram_clock_change_latency_us;
    ASSERT(vlevel < (*dml).soc.num_states as i32);
    (*pipes).clks_cfg.voltage = vlevel;
    (*pipes).clks_cfg.dcfclk_mhz = (*dml).soc.clock_limits[vlevel as usize].dcfclk_mhz;
    (*pipes).clks_cfg.socclk_mhz = (*dml).soc.clock_limits[vlevel as usize].socclk_mhz;
    (*dml).soc.dram_clock_change_latency_us = (*table_entry).pstate_latency_us;
    (*dml).soc.sr_exit_time_us = (*table_entry).sr_exit_time_us;
    (*dml).soc.sr_enter_plus_exit_time_us = (*table_entry).sr_enter_plus_exit_time_us;
    (*wm_set).urgent_ns = (get_wm_urgent(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).cstate_pstate.cstate_enter_plus_exit_ns = (get_wm_stutter_enter_exit(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).cstate_pstate.cstate_exit_ns = (get_wm_stutter_exit(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).cstate_pstate.pstate_change_ns = (get_wm_dram_clock_change(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).pte_meta_urgent_ns = (get_wm_memory_trip(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).frac_urg_bw_nom = (get_fraction_of_urgent_bandwidth(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).frac_urg_bw_flip = (get_fraction_of_urgent_bandwidth_imm_flip(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*wm_set).urgent_latency_ns = (get_urgent_latency(dml, pipes, pipe_cnt) * 1000.0) as u32;
    (*dml).soc.dram_clock_change_latency_us = dram_clock_change_latency_cached;
}

pub unsafe fn dcn301_fpu_update_bw_bounding_box(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let s = (*dc).scratch.update_bw_bounding_box.clock_limits;
    let pool = TO_DCN301_RES_POOL((*dc).res_pool);
    let clk_table = &mut (*bw_params).clk_table;
    let mut closest_clk_lvl: usize;
    let mut max_dispclk_mhz = 0;
    let mut max_dppclk_mhz = 0;
    dc_assert_fp_enabled();
    memcpy(s, dcn3_01_soc.clock_limits.as_ptr(), core::mem::size_of_val(&dcn3_01_soc.clock_limits));
    dcn3_01_ip.max_num_otg = (*pool).base.res_cap.num_timing_generator;
    dcn3_01_ip.max_num_dpp = (*pool).base.pipe_count;
    dcn3_01_soc.num_chans = (*bw_params).num_channels;
    ASSERT(clk_table.num_entries != 0);
    for i in 0..clk_table.num_entries as usize { max_dispclk_mhz = max_dispclk_mhz.max(clk_table.entries[i].dispclk_mhz); max_dppclk_mhz = max_dppclk_mhz.max(clk_table.entries[i].dppclk_mhz); }
    for i in 0..clk_table.num_entries as usize {
        closest_clk_lvl = 0;
        let mut j = dcn3_01_soc.num_states as i32 - 1;
        while j >= 0 { if dcn3_01_soc.clock_limits[j as usize].dcfclk_mhz as u32 <= clk_table.entries[i].dcfclk_mhz { closest_clk_lvl = j as usize; break; } j -= 1; }
        dcn3_01_soc.clock_limits[i].state = i;
        dcn3_01_soc.clock_limits[i].dcfclk_mhz = clk_table.entries[i].dcfclk_mhz;
        dcn3_01_soc.clock_limits[i].fabricclk_mhz = clk_table.entries[i].fclk_mhz;
        dcn3_01_soc.clock_limits[i].socclk_mhz = clk_table.entries[i].socclk_mhz;
        dcn3_01_soc.clock_limits[i].dram_speed_mts = clk_table.entries[i].memclk_mhz * 2;
        dcn3_01_soc.clock_limits[i].dispclk_mhz = if max_dispclk_mhz != 0 { max_dispclk_mhz as f64 } else { dcn3_01_soc.clock_limits[closest_clk_lvl].dispclk_mhz };
        dcn3_01_soc.clock_limits[i].dppclk_mhz = if max_dppclk_mhz != 0 { max_dppclk_mhz as f64 } else { dcn3_01_soc.clock_limits[closest_clk_lvl].dppclk_mhz };
        dcn3_01_soc.clock_limits[i].dram_bw_per_chan_gbps = dcn3_01_soc.clock_limits[closest_clk_lvl].dram_bw_per_chan_gbps;
        dcn3_01_soc.clock_limits[i].dscclk_mhz = dcn3_01_soc.clock_limits[closest_clk_lvl].dscclk_mhz;
        dcn3_01_soc.clock_limits[i].dtbclk_mhz = dcn3_01_soc.clock_limits[closest_clk_lvl].dtbclk_mhz;
        dcn3_01_soc.clock_limits[i].phyclk_d18_mhz = dcn3_01_soc.clock_limits[closest_clk_lvl].phyclk_d18_mhz;
        dcn3_01_soc.clock_limits[i].phyclk_mhz = dcn3_01_soc.clock_limits[closest_clk_lvl].phyclk_mhz;
    }
    if clk_table.num_entries != 0 { dcn3_01_soc.num_states = clk_table.num_entries; dcn3_01_soc.clock_limits[dcn3_01_soc.num_states] = dcn3_01_soc.clock_limits[dcn3_01_soc.num_states - 1]; dcn3_01_soc.clock_limits[dcn3_01_soc.num_states].state = dcn3_01_soc.num_states; }
    memcpy(dcn3_01_soc.clock_limits.as_mut_ptr(), s, core::mem::size_of_val(&dcn3_01_soc.clock_limits));
    dcn3_01_soc.dispclk_dppclk_vco_speed_mhz = (*dc).clk_mgr.dentist_vco_freq_khz as f64 / 1000.0;
    (*dc).dml.soc.dispclk_dppclk_vco_speed_mhz = (*dc).clk_mgr.dentist_vco_freq_khz as f64 / 1000.0;
    if (dcn3_01_soc.dram_clock_change_latency_us * 1000.0) as i32 != (*dc).debug.dram_clock_change_latency_ns && (*dc).debug.dram_clock_change_latency_ns != 0 { dcn3_01_soc.dram_clock_change_latency_us = (*dc).debug.dram_clock_change_latency_ns as f64 / 1000.0; }
    dml_init_instance(&mut (*dc).dml, &mut dcn3_01_soc, &mut dcn3_01_ip, DML_PROJECT_DCN30);
}

pub unsafe fn dcn301_fpu_set_wm_ranges(i: i32, ranges: *mut pp_smu_wm_range_sets, loaded_bb: *mut _vcs_dpi_soc_bounding_box_st) { dc_assert_fp_enabled(); (*ranges).reader_wm_sets[i as usize].min_fill_clk_mhz = if i > 0 { ((*loaded_bb).clock_limits[(i - 1) as usize].dram_speed_mts / 16.0 + 1.0) as u16 } else { 0 }; (*ranges).reader_wm_sets[i as usize].max_fill_clk_mhz = ((*loaded_bb).clock_limits[i as usize].dram_speed_mts / 16.0) as u16; }

pub unsafe fn dcn301_fpu_init_soc_bounding_box(bb_info: bp_soc_bb_info) { dc_assert_fp_enabled(); if bb_info.dram_clock_change_latency_100ns > 0.0 { dcn3_01_soc.dram_clock_change_latency_us = bb_info.dram_clock_change_latency_100ns * 10.0; } if bb_info.dram_sr_enter_exit_latency_100ns > 0.0 { dcn3_01_soc.sr_enter_plus_exit_time_us = bb_info.dram_sr_enter_exit_latency_100ns * 10.0; } if bb_info.dram_sr_exit_latency_100ns > 0.0 { dcn3_01_soc.sr_exit_time_us = bb_info.dram_sr_exit_latency_100ns * 10.0; } }

pub unsafe fn dcn301_fpu_calculate_wm_and_dlg(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, vlevel_req: i32) {
    let bw_params = (*dc).clk_mgr.bw_params;
    ASSERT(!bw_params.is_null()); dc_assert_fp_enabled();
    let vlevel_max = (*bw_params).clk_table.num_entries as i32 - 1;
    let mut table_entry = &mut (*bw_params).wm_table.entries[WM_D as usize];
    let vlevel = if table_entry.wm_type == WM_TYPE_RETRAINING { 0 } else { vlevel_max };
    calculate_wm_set_for_vlevel(vlevel, table_entry, &mut (*context).bw_ctx.bw.dcn.watermarks.d, &mut (*context).bw_ctx.dml, pipes, pipe_cnt);
    table_entry = &mut (*bw_params).wm_table.entries[WM_C as usize]; calculate_wm_set_for_vlevel(vlevel_req.clamp(2, vlevel_max), table_entry, &mut (*context).bw_ctx.bw.dcn.watermarks.c, &mut (*context).bw_ctx.dml, pipes, pipe_cnt);
    table_entry = &mut (*bw_params).wm_table.entries[WM_B as usize]; calculate_wm_set_for_vlevel(vlevel_req.clamp(1, vlevel_max), table_entry, &mut (*context).bw_ctx.bw.dcn.watermarks.b, &mut (*context).bw_ctx.dml, pipes, pipe_cnt);
    table_entry = &mut (*bw_params).wm_table.entries[WM_A as usize]; calculate_wm_set_for_vlevel(vlevel_req.min(vlevel_max), table_entry, &mut (*context).bw_ctx.bw.dcn.watermarks.a, &mut (*context).bw_ctx.dml, pipes, pipe_cnt);
    let mut pipe_idx = 0usize;
    for i in 0..(*dc).res_pool.pipe_count as usize { if (*context).res_ctx.pipe_ctx[i].stream.is_null() { continue; } (*pipes.add(pipe_idx)).clks_cfg.dispclk_mhz = get_dispclk_calculated(&mut (*context).bw_ctx.dml, pipes, pipe_cnt); (*pipes.add(pipe_idx)).clks_cfg.dppclk_mhz = get_dppclk_calculated(&mut (*context).bw_ctx.dml, pipes, pipe_cnt, pipe_idx as i32); if (*dc).config.forced_clocks { (*pipes.add(pipe_idx)).clks_cfg.dispclk_mhz = (*context).bw_ctx.dml.soc.clock_limits[0].dispclk_mhz; (*pipes.add(pipe_idx)).clks_cfg.dppclk_mhz = (*context).bw_ctx.dml.soc.clock_limits[0].dppclk_mhz; } if (*dc).debug.min_disp_clk_khz as f64 > (*pipes.add(pipe_idx)).clks_cfg.dispclk_mhz * 1000.0 { (*pipes.add(pipe_idx)).clks_cfg.dispclk_mhz = (*dc).debug.min_disp_clk_khz as f64 / 1000.0; } if (*dc).debug.min_dpp_clk_khz as f64 > (*pipes.add(pipe_idx)).clks_cfg.dppclk_mhz * 1000.0 { (*pipes.add(pipe_idx)).clks_cfg.dppclk_mhz = (*dc).debug.min_dpp_clk_khz as f64 / 1000.0; } pipe_idx += 1; }
    dcn20_calculate_dlg_params(dc, context, pipes, pipe_cnt, vlevel);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
