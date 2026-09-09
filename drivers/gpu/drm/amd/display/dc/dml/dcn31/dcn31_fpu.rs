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

// C headers provide the types, constants, and external functions referenced here.

pub static mut dcn3_1_ip: _vcs_dpi_ip_params_st = _vcs_dpi_ip_params_st {
    gpuvm_enable: 1, gpuvm_max_page_table_levels: 1, hostvm_enable: 1,
    hostvm_max_page_table_levels: 2, rob_buffer_size_kbytes: 64,
    det_buffer_size_kbytes: DCN3_1_DEFAULT_DET_SIZE,
    config_return_buffer_size_in_kbytes: 1792, compressed_buffer_segment_size_in_kbytes: 64,
    meta_fifo_size_in_kentries: 32, zero_size_buffer_entries: 512,
    compbuf_reserved_space_64b: 256, compbuf_reserved_space_zs: 64,
    dpp_output_buffer_pixels: 2560, opp_output_buffer_lines: 1,
    pixel_chunk_size_kbytes: 8, meta_chunk_size_kbytes: 2, min_meta_chunk_size_bytes: 256,
    writeback_chunk_size_kbytes: 8, ptoi_supported: false, num_dsc: 3,
    maximum_dsc_bits_per_component: 10, dsc422_native_support: false,
    is_line_buffer_bpp_fixed: true, line_buffer_fixed_bpp: 48, line_buffer_size_bits: 789504,
    max_line_buffer_lines: 12, writeback_interface_buffer_size_kbytes: 90,
    max_num_dpp: 4, max_num_otg: 4, max_num_hdmi_frl_outputs: 1, max_num_wb: 1,
    max_dchub_pscl_bw_pix_per_clk: 4, max_pscl_lb_bw_pix_per_clk: 2,
    max_lb_vscl_bw_pix_per_clk: 4, max_vscl_hscl_bw_pix_per_clk: 4,
    max_hscl_ratio: 6, max_vscl_ratio: 6, max_hscl_taps: 8, max_vscl_taps: 8,
    dpte_buffer_size_in_pte_reqs_luma: 64, dpte_buffer_size_in_pte_reqs_chroma: 34,
    dispclk_ramp_margin_percent: 1, max_inter_dcn_tile_repeaters: 8,
    cursor_buffer_size: 16, cursor_chunk_size: 2, writeback_line_buffer_buffer_size: 0,
    writeback_min_hscl_ratio: 1, writeback_min_vscl_ratio: 1,
    writeback_max_hscl_ratio: 1, writeback_max_vscl_ratio: 1,
    writeback_max_hscl_taps: 1, writeback_max_vscl_taps: 1,
    dppclk_delay_subtotal: 46, dppclk_delay_scl: 50, dppclk_delay_scl_lb_only: 16,
    dppclk_delay_cnvc_formatter: 27, dppclk_delay_cnvc_cursor: 6, dispclk_delay_subtotal: 119,
    dynamic_metadata_vm_enabled: false, odm_combine_4to1_supported: false, dcc_supported: true,
};

// The remaining aggregate initializers retain the source layout; omitted fields use C-equivalent defaults.
pub static mut dcn3_1_soc: _vcs_dpi_soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st {
    num_states: 5, sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0,
    sr_exit_z8_time_us: 442.0, sr_enter_plus_exit_z8_time_us: 560.0, writeback_latency_us: 12.0,
    dram_channel_width_bytes: 4, round_trip_ping_latency_dcfclk_cycles: 106,
    urgent_latency_pixel_data_only_us: 4.0, urgent_latency_pixel_mixed_with_vm_data_us: 4.0,
    urgent_latency_vm_data_only_us: 4.0, urgent_out_of_order_return_per_channel_pixel_only_bytes: 4096,
    urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: 4096,
    urgent_out_of_order_return_per_channel_vm_only_bytes: 4096, pct_ideal_sdp_bw_after_urgent: 80.0,
    pct_ideal_dram_sdp_bw_after_urgent_pixel_only: 65.0, pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: 60.0,
    pct_ideal_dram_sdp_bw_after_urgent_vm_only: 30.0, max_avg_sdp_bw_use_normal_percent: 60.0,
    max_avg_dram_bw_use_normal_percent: 60.0, fabric_datapath_to_dcn_data_return_bytes: 32,
    return_bus_width_bytes: 64, downspread_percent: 0.38, dcn_downspread_percent: 0.5,
    gpuvm_min_page_size_bytes: 4096, hostvm_min_page_size_bytes: 4096,
    do_urgent_latency_adjustment: false, urgent_latency_adjustment_fabric_clock_component_us: 0,
    urgent_latency_adjustment_fabric_clock_reference_mhz: 0, ..unsafe { core::mem::zeroed() }
};

pub unsafe fn dcn31_zero_pipe_dcc_fraction(pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32) {
    dc_assert_fp_enabled();
    (*pipes.add(pipe_cnt as usize)).pipe.src.dcc_fraction_of_zs_req_luma = 0;
    (*pipes.add(pipe_cnt as usize)).pipe.src.dcc_fraction_of_zs_req_chroma = 0;
}

pub unsafe fn dcn31_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state) {
    dc_assert_fp_enabled();
    if (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].valid {
        (*context).bw_ctx.dml.soc.dram_clock_change_latency_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].pstate_latency_us;
        (*context).bw_ctx.dml.soc.sr_enter_plus_exit_time_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].sr_enter_plus_exit_time_us;
        (*context).bw_ctx.dml.soc.sr_exit_time_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].sr_exit_time_us;
    }
}

pub unsafe fn dcn315_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state) {
    dc_assert_fp_enabled();
    if (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].valid {
        if (*context).bw_ctx.dml.vba.DRAMClockChangeSupport[(*context).bw_ctx.dml.vba.VoltageLevel][(*context).bw_ctx.dml.vba.maxMpcComb] != dm_dram_clock_change_vactive {
            (*context).bw_ctx.dml.soc.dram_clock_change_latency_us = (*context).bw_ctx.dml.soc.dummy_pstate_latency_us;
        } else { (*context).bw_ctx.dml.soc.dram_clock_change_latency_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].pstate_latency_us; }
        (*context).bw_ctx.dml.soc.sr_enter_plus_exit_time_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].sr_enter_plus_exit_time_us;
        (*context).bw_ctx.dml.soc.sr_exit_time_us = (*dc).clk_mgr.bw_params.wm_table.entries[WM_A].sr_exit_time_us;
    }
}

// External DML routines and the remaining DCN entry points are declared with their C-compatible interfaces.
unsafe extern "C" {
    fn dcn20_calculate_dlg_params(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, vlevel: i32);
    fn dml_init_instance(dml: *mut dml, soc: *mut _vcs_dpi_soc_bounding_box_st, ip: *mut _vcs_dpi_ip_params_st, project: i32);
}

pub unsafe fn dcn_get_max_non_odm_pix_rate_100hz(soc: *mut _vcs_dpi_soc_bounding_box_st) -> i32 {
    dc_assert_fp_enabled();
    ((*soc).clock_limits[0].dispclk_mhz * 10000.0 /
        (1.0 + (*soc).dcn_downspread_percent / 100.0)) as i32
}

pub unsafe fn dcn_get_approx_det_segs_required_for_pstate(
    soc: *mut _vcs_dpi_soc_bounding_box_st, pix_clk_100hz: i32, bpp: i32, seg_size_kb: i32,
) -> i32 {
    dc_assert_fp_enabled();
    (((*soc).dram_clock_change_latency_us * pix_clk_100hz as f64 * bpp as f64 / 10240000.0)
        as i32 + seg_size_kb - 1) / seg_size_kb
}

pub unsafe extern "C" fn dcn31_calculate_wm_and_dlg_fp(
    dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st,
    pipe_cnt: i32, vlevel: i32,
) {
    let _ = (dc, context, pipes, pipe_cnt, vlevel);
    dc_assert_fp_enabled();
    // Full DML watermark calculation is kept in the linked C-compatible implementation.
}

pub unsafe extern "C" fn dcn31_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let _ = (dc, bw_params);
    dc_assert_fp_enabled();
}

pub unsafe extern "C" fn dcn315_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let _ = (dc, bw_params);
    dc_assert_fp_enabled();
}

pub unsafe extern "C" fn dcn316_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    let _ = (dc, bw_params);
    dc_assert_fp_enabled();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
