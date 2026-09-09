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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit.

pub static mut dcn3_03_ip: _vcs_dpi_ip_params_st = _vcs_dpi_ip_params_st {
    use_min_dcfclk: 0, clamp_min_dcfclk: 0, odm_capable: 1, gpuvm_enable: 1,
    hostvm_enable: 0, gpuvm_max_page_table_levels: 4, hostvm_max_page_table_levels: 4,
    hostvm_cached_page_table_levels: 0, pte_group_size_bytes: 2048, num_dsc: 2,
    rob_buffer_size_kbytes: 184, det_buffer_size_kbytes: 184,
    dpte_buffer_size_in_pte_reqs_luma: 64, dpte_buffer_size_in_pte_reqs_chroma: 34,
    pde_proc_buffer_size_64k_reqs: 48, dpp_output_buffer_pixels: 2560,
    opp_output_buffer_lines: 1, pixel_chunk_size_kbytes: 8, pte_enable: 1,
    max_page_table_levels: 2, pte_chunk_size_kbytes: 2, meta_chunk_size_kbytes: 2,
    writeback_chunk_size_kbytes: 8, line_buffer_size_bits: 789504,
    is_line_buffer_bpp_fixed: 0, line_buffer_fixed_bpp: 0, dcc_supported: true,
    writeback_interface_buffer_size_kbytes: 90, writeback_line_buffer_buffer_size: 0,
    max_line_buffer_lines: 12, writeback_luma_buffer_size_kbytes: 12,
    writeback_chroma_buffer_size_kbytes: 8, writeback_chroma_line_buffer_width_pixels: 4,
    writeback_max_hscl_ratio: 1, writeback_max_vscl_ratio: 1,
    writeback_min_hscl_ratio: 1, writeback_min_vscl_ratio: 1,
    writeback_max_hscl_taps: 1, writeback_max_vscl_taps: 1,
    writeback_line_buffer_luma_buffer_size: 0, writeback_line_buffer_chroma_buffer_size: 14643,
    cursor_buffer_size: 8, cursor_chunk_size: 2, max_num_otg: 2, max_num_dpp: 2,
    max_num_wb: 1, max_dchub_pscl_bw_pix_per_clk: 4, max_pscl_lb_bw_pix_per_clk: 2,
    max_lb_vscl_bw_pix_per_clk: 4, max_vscl_hscl_bw_pix_per_clk: 4,
    max_hscl_ratio: 6, max_vscl_ratio: 6, hscl_mults: 4, vscl_mults: 4,
    max_hscl_taps: 8, max_vscl_taps: 8, dispclk_ramp_margin_percent: 1,
    underscan_factor: 1.11, min_vblank_lines: 32, dppclk_delay_subtotal: 46,
    dynamic_metadata_vm_enabled: true, dppclk_delay_scl_lb_only: 16,
    dppclk_delay_scl: 50, dppclk_delay_cnvc_formatter: 27, dppclk_delay_cnvc_cursor: 6,
    dispclk_delay_subtotal: 119, dcfclk_cstate_latency: 5.2,
    max_inter_dcn_tile_repeaters: 8, max_num_hdmi_frl_outputs: 1,
    odm_combine_4to1_supported: false, xfc_supported: false,
    xfc_fill_bw_overhead_percent: 10.0, xfc_fill_constant_bytes: 0,
    gfx7_compat_tiling_supported: 0, number_of_cursors: 1,
};

pub static mut dcn3_03_soc: _vcs_dpi_soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st {
    clock_limits: [_vcs_dpi_soc_clock_state_st { state: 0, dispclk_mhz: 562.0,
        dppclk_mhz: 300.0, phyclk_mhz: 300.0, phyclk_d18_mhz: 667.0,
        dscclk_mhz: 405.6, dtbclk_mhz: 1217.0, ..unsafe { core::mem::zeroed() } },
        ..unsafe { core::mem::zeroed() }],
    min_dcfclk: 500.0, num_states: 1, sr_exit_time_us: 35.5,
    sr_enter_plus_exit_time_us: 40.0, urgent_latency_us: 4.0,
    urgent_latency_pixel_data_only_us: 4.0, urgent_latency_pixel_mixed_with_vm_data_us: 4.0,
    urgent_latency_vm_data_only_us: 4.0, urgent_out_of_order_return_per_channel_pixel_only_bytes: 4096,
    urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: 4096,
    urgent_out_of_order_return_per_channel_vm_only_bytes: 4096,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_only: 80.0,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: 60.0,
    pct_ideal_dram_sdp_bw_after_urgent_vm_only: 40.0, max_avg_sdp_bw_use_normal_percent: 60.0,
    max_avg_dram_bw_use_normal_percent: 40.0, writeback_latency_us: 12.0,
    max_request_size_bytes: 256, fabric_datapath_to_dcn_data_return_bytes: 64,
    dcn_downspread_percent: 0.5, downspread_percent: 0.38, dram_page_open_time_ns: 50.0,
    dram_rw_turnaround_time_ns: 17.5, dram_return_buffer_per_channel_bytes: 8192,
    round_trip_ping_latency_dcfclk_cycles: 156, urgent_out_of_order_return_per_channel_bytes: 4096,
    channel_interleave_bytes: 256, num_banks: 8, gpuvm_min_page_size_bytes: 4096,
    hostvm_min_page_size_bytes: 4096, dram_clock_change_latency_us: 404,
    dummy_pstate_latency_us: 5, writeback_dram_clock_change_latency_us: 23.0,
    return_bus_width_bytes: 64, dispclk_dppclk_vco_speed_mhz: 3650,
    xfc_bus_transport_time_us: 20, xfc_xbuf_latency_tolerance_us: 4, use_urgent_burst_bw: 1,
    do_urgent_latency_adjustment: true, urgent_latency_adjustment_fabric_clock_component_us: 1.0,
    urgent_latency_adjustment_fabric_clock_reference_mhz: 1000, ..unsafe { core::mem::zeroed() }
};

unsafe fn dcn303_get_optimal_dcfclk_fclk_for_uclk(uclk_mts: u32, optimal_dcfclk: *mut u32, optimal_fclk: *mut u32) {
    let bw1 = uclk_mts as f64 * dcn3_03_soc.num_chans as f64 * dcn3_03_soc.dram_channel_width_bytes as f64 * (dcn3_03_soc.max_avg_dram_bw_use_normal_percent / 100.0);
    let bw2 = uclk_mts as f64 * dcn3_03_soc.num_chans as f64 * dcn3_03_soc.dram_channel_width_bytes as f64 * (dcn3_03_soc.max_avg_sdp_bw_use_normal_percent / 100.0);
    let bw = if bw1 < bw2 { bw1 } else { bw2 };
    if !optimal_fclk.is_null() { *optimal_fclk = (bw / (dcn3_03_soc.fabric_datapath_to_dcn_data_return_bytes as f64 * (dcn3_03_soc.max_avg_sdp_bw_use_normal_percent / 100.0))) as u32; }
    if !optimal_dcfclk.is_null() { *optimal_dcfclk = (bw / (dcn3_03_soc.return_bus_width_bytes as f64 * (dcn3_03_soc.max_avg_sdp_bw_use_normal_percent / 100.0))) as u32; }
}

// The remaining implementation is a direct low-level translation of the C routine;
// external structures and helpers are intentionally left as supplied dependencies.
pub unsafe fn dcn303_fpu_update_bw_bounding_box(dc: *mut dc, bw_params: *mut clk_bw_params) {
    dc_assert_fp_enabled();
    let mut dcf = [0u32; DC__VOLTAGE_STATES]; let mut dram = [0u32; DC__VOLTAGE_STATES];
    let mut ou = [0u32; DC__VOLTAGE_STATES]; let mut od = [0u32; DC__VOLTAGE_STATES];
    let mut targets = [694u32, 875, 1000, 1200]; let mut nt = 4usize; let mut ns = 0usize;
    if (*dc).ctx.dc_bios.vram_info.num_chans != 0 { dcn3_03_soc.num_chans = (*dc).ctx.dc_bios.vram_info.num_chans; }
    if (*dc).ctx.dc_bios.vram_info.dram_channel_width_bytes != 0 { dcn3_03_soc.dram_channel_width_bytes = (*dc).ctx.dc_bios.vram_info.dram_channel_width_bytes; }
    dcn3_03_soc.dispclk_dppclk_vco_speed_mhz = (*dc).clk_mgr.dentist_vco_freq_khz as f64 / 1000.0;
    (*dc).dml.soc.dispclk_dppclk_vco_speed_mhz = dcn3_03_soc.dispclk_dppclk_vco_speed_mhz;
    if (*bw_params).clk_table.entries[0].memclk_mhz != 0 {
        let mut max_dcf=0u32; let mut max_disp=0u32; let mut max_dpp=0u32; let mut max_phy=0u32;
        for i in 0..MAX_NUM_DPM_LVL { let e=&(*bw_params).clk_table.entries[i]; max_dcf=max_dcf.max(e.dcfclk_mhz); max_disp=max_disp.max(e.dispclk_mhz); max_dpp=max_dpp.max(e.dppclk_mhz); max_phy=max_phy.max(e.phyclk_mhz); }
        if max_dcf==0 { max_dcf=dcn3_03_soc.clock_limits[0].dcfclk_mhz as u32; } if max_disp==0 { max_disp=dcn3_03_soc.clock_limits[0].dispclk_mhz as u32; } if max_dpp==0 { max_dpp=dcn3_03_soc.clock_limits[0].dppclk_mhz as u32; } if max_phy==0 { max_phy=dcn3_03_soc.clock_limits[0].phyclk_mhz as u32; }
        if max_dcf > targets[3] { targets[nt]=max_dcf; nt+=1; } else if max_dcf < targets[3] { for i in 0..nt { if targets[i]>max_dcf { targets[i]=max_dcf; nt=i+1; break; } } }
        let nu=(*bw_params).clk_table.num_entries as usize;
        for i in 0..nu { dcn303_get_optimal_dcfclk_fclk_for_uclk((*bw_params).clk_table.entries[i].memclk_mhz*16, &mut od[i], core::ptr::null_mut()); if od[i]<(*bw_params).clk_table.entries[0].dcfclk_mhz { od[i]=(*bw_params).clk_table.entries[0].dcfclk_mhz; } }
        for i in 0..nt { for j in 0..nu { if targets[i]<od[j] { ou[i]=(*bw_params).clk_table.entries[j].memclk_mhz*16; break; } else if j==nu-1 { ou[i]=(*bw_params).clk_table.entries[j].memclk_mhz*16; } } }
        let mut i=0; let mut j=0; while i<nt && j<nu && ns<DC__VOLTAGE_STATES { if targets[i]<od[j] { dcf[ns]=targets[i]; dram[ns]=ou[i]; ns+=1; i+=1; } else if od[j]<=max_dcf { dcf[ns]=od[j]; dram[ns]=(*bw_params).clk_table.entries[j].memclk_mhz*16; ns+=1; j+=1; } else { j=nu; } }
        while i<nt && ns<DC__VOLTAGE_STATES { dcf[ns]=targets[i]; dram[ns]=ou[i]; ns+=1; i+=1; }
        while j<nu && ns<DC__VOLTAGE_STATES && od[j]<=max_dcf { dcf[ns]=od[j]; dram[ns]=(*bw_params).clk_table.entries[j].memclk_mhz*16; ns+=1; j+=1; }
        if ns>MAX_NUM_DPM_LVL { ASSERT(0); return; }
        dcn3_03_soc.num_states=ns; for i in 0..ns { let s=&mut dcn3_03_soc.clock_limits[i]; s.state=i; s.dcfclk_mhz=dcf[i]; s.fabricclk_mhz=dcf[i]; s.dram_speed_mts=dram[i]; s.dispclk_mhz=max_disp; s.dppclk_mhz=max_dpp; s.phyclk_mhz=max_phy; s.dtbclk_mhz=if (*bw_params).clk_table.entries[i].dtbclk_mhz==0 && i>0 { dcn3_03_soc.clock_limits[i-1].dtbclk_mhz } else { (*bw_params).clk_table.entries[i].dtbclk_mhz }; s.socclk_mhz=if (*bw_params).clk_table.entries[i].socclk_mhz==0 && i>0 { dcn3_03_soc.clock_limits[i-1].socclk_mhz } else { (*bw_params).clk_table.entries[i].socclk_mhz }; s.phyclk_d18_mhz=dcn3_03_soc.clock_limits[0].phyclk_d18_mhz; s.dscclk_mhz=dcn3_03_soc.clock_limits[0].dscclk_mhz; }
        if dcn3_03_soc.num_chans<=4 { for i in 0..ns { if dcn3_03_soc.clock_limits[i].dram_speed_mts>1700 { break; } if dcn3_03_soc.clock_limits[i].dram_speed_mts>=1500 { dcn3_03_soc.clock_limits[i].dcfclk_mhz=100; dcn3_03_soc.clock_limits[i].fabricclk_mhz=100; } } }
        dml_init_instance(&mut (*dc).dml, &dcn3_03_soc, &dcn3_03_ip, DML_PROJECT_DCN30); if !(*dc).current_state.is_null() { dml_init_instance(&mut (*(*dc).current_state).bw_ctx.dml, &dcn3_03_soc, &dcn3_03_ip, DML_PROJECT_DCN30); }
    }
}

pub unsafe fn dcn303_fpu_init_soc_bounding_box(bb_info: bp_soc_bb_info) {
    dc_assert_fp_enabled();
    if bb_info.dram_clock_change_latency_100ns > 0 { dcn3_03_soc.dram_clock_change_latency_us = bb_info.dram_clock_change_latency_100ns * 10; }
    if bb_info.dram_sr_enter_exit_latency_100ns > 0 { dcn3_03_soc.sr_enter_plus_exit_time_us = bb_info.dram_sr_enter_exit_latency_100ns * 10; }
    if bb_info.dram_sr_exit_latency_100ns > 0 { dcn3_03_soc.sr_exit_time_us = bb_info.dram_sr_exit_latency_100ns * 10; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
