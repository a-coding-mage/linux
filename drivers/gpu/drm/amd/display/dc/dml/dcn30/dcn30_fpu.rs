/* Faithful low-level Rust translation of dcn30_fpu.c. Includes are supplied
 * by the surrounding kernel bindings and are intentionally not reproduced. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    static mut dcn3_0_ip: _vcs_dpi_ip_params_st;
    static mut dcn3_0_soc: _vcs_dpi_soc_bounding_box_st;
    fn dc_assert_fp_enabled();
    fn dml30_CalculateWriteBackDISPCLK(format: dm_output_format, pixel_rate: f64, hratio: f64, vratio: f64, htaps: i32, vtaps: i32, src_width: i32, dst_width: i32, htotal: i32, line_buffer: i32) -> f64;
    fn get_wm_writeback_urgent(dml: *mut display_mode_lib, pipes: *mut display_e2e_pipe_params_st, count: i32) -> f64;
    fn get_wm_writeback_dram_clock_change(dml: *mut display_mode_lib, pipes: *mut display_e2e_pipe_params_st, count: i32) -> f64;
    fn dml_init_instance(dml: *mut display_mode_lib, soc: *mut _vcs_dpi_soc_bounding_box_st, ip: *mut _vcs_dpi_ip_params_st, project: i32);
    fn dml1_frl_cap_chk_common(i: *mut frl_cap_chk_intermediates, p: *mut frl_cap_chk_params) -> frl_cap_chk_result;
    fn dml1_frl_cap_chk_uncompressed(p: *mut frl_cap_chk_params, i: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result;
    fn dml1_frl_cap_chk_compressed(p: *mut frl_cap_chk_params, i: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result;
}

#[no_mangle]
pub unsafe extern "C" fn dcn30_fpu_populate_dml_writeback_from_context(dc: *mut dc, res_ctx: *mut resource_context, pipes: *mut display_e2e_pipe_params_st) {
    dc_assert_fp_enabled();
    let mut pipe_cnt: i32 = 0;
    for i in 0..(*(*dc).res_pool).pipe_count {
        let stream = (*res_ctx).pipe_ctx[i as usize].stream;
        if stream.is_null() { continue; }
        let mut max_calc_writeback_dispclk = 0.0f64;
        (*pipes.add(pipe_cnt as usize)).dout.wb_enable = 0;
        (*pipes.add(pipe_cnt as usize)).dout.num_active_wb = 0;
        for j in 0..(*stream).num_wb_info {
            let wb = &(*stream).writeback_info[j as usize];
            if wb.wb_enabled && !wb.writeback_source_plane.is_null() && wb.writeback_source_plane == (*res_ctx).pipe_ctx[i as usize].plane_state {
                let mut out: writeback_st = core::mem::zeroed();
                (*pipes.add(pipe_cnt as usize)).dout.wb_enable = 1;
                (*pipes.add(pipe_cnt as usize)).dout.num_active_wb += 1;
                out.wb_src_height = if wb.dwb_params.cnv_params.crop_en { wb.dwb_params.cnv_params.crop_height } else { wb.dwb_params.cnv_params.src_height };
                out.wb_src_width = if wb.dwb_params.cnv_params.crop_en { wb.dwb_params.cnv_params.crop_width } else { wb.dwb_params.cnv_params.src_width };
                out.wb_dst_width = wb.dwb_params.dest_width; out.wb_dst_height = wb.dwb_params.dest_height;
                if (*dc).dml.ip.writeback_max_hscl_taps > 1 { out.wb_htaps_luma = wb.dwb_params.scaler_taps.h_taps; out.wb_vtaps_luma = wb.dwb_params.scaler_taps.v_taps; } else { out.wb_htaps_luma = 1; out.wb_vtaps_luma = 1; }
                out.wb_htaps_chroma = 0; out.wb_vtaps_chroma = 0;
                out.wb_hratio = (if wb.dwb_params.cnv_params.crop_en { wb.dwb_params.cnv_params.crop_width } else { wb.dwb_params.cnv_params.src_width }) as f64 / wb.dwb_params.dest_width as f64;
                out.wb_vratio = (if wb.dwb_params.cnv_params.crop_en { wb.dwb_params.cnv_params.crop_height } else { wb.dwb_params.cnv_params.src_height }) as f64 / wb.dwb_params.dest_height as f64;
                out.wb_pixel_format = if wb.dwb_params.cnv_params.fc_out_format == DWB_OUT_FORMAT_64BPP_ARGB || wb.dwb_params.cnv_params.fc_out_format == DWB_OUT_FORMAT_64BPP_RGBA { dm_444_64 } else { dm_444_32 };
                let clk = dml30_CalculateWriteBackDISPCLK(out.wb_pixel_format, (*pipes.add(pipe_cnt as usize)).pipe.dest.pixel_rate_mhz, out.wb_hratio, out.wb_vratio, out.wb_htaps_luma, out.wb_vtaps_luma, out.wb_src_width, out.wb_dst_width, (*pipes.add(pipe_cnt as usize)).pipe.dest.htotal, (*dc).current_state.bw_ctx.dml.ip.writeback_line_buffer_buffer_size);
                if clk > max_calc_writeback_dispclk { max_calc_writeback_dispclk = clk; (*pipes.add(pipe_cnt as usize)).dout.wb = out; }
            }
        }
        pipe_cnt += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn dcn30_fpu_set_mcif_arb_params(wb: *mut mcif_arb_params, dml: *mut display_mode_lib, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, cur_pipe: i32) {
    dc_assert_fp_enabled();
    for i in 0..(*wb).cli_watermark.len() { (*wb).cli_watermark[i] = (get_wm_writeback_urgent(dml, pipes, pipe_cnt) * 1000.0) as u32; (*wb).pstate_watermark[i] = (get_wm_writeback_dram_clock_change(dml, pipes, pipe_cnt) * 1000.0) as u32; }
    (*wb).dram_speed_change_duration = ((*dml).vba.WritebackAllowDRAMClockChangeEndPosition[cur_pipe as usize] * (*pipes).clks_cfg.refclk_mhz) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn dcn30_fpu_update_dram_channel_width_bytes(dc: *mut dc) { dc_assert_fp_enabled(); if (*(*(*dc).ctx).dc_bios).vram_info.dram_channel_width_bytes != 0 { dcn3_0_soc.dram_channel_width_bytes = (*(*(*dc).ctx).dc_bios).vram_info.dram_channel_width_bytes; } }

#[no_mangle]
pub unsafe extern "C" fn dcn30_fpu_update_max_clk(x: *mut dc_bounding_box_max_clk) { dc_assert_fp_enabled(); if (*x).max_dcfclk_mhz == 0 { (*x).max_dcfclk_mhz = dcn3_0_soc.clock_limits[0].dcfclk_mhz as i32; } if (*x).max_dispclk_mhz == 0 { (*x).max_dispclk_mhz = dcn3_0_soc.clock_limits[0].dispclk_mhz as i32; } if (*x).max_dppclk_mhz == 0 { (*x).max_dppclk_mhz = dcn3_0_soc.clock_limits[0].dppclk_mhz as i32; } if (*x).max_phyclk_mhz == 0 { (*x).max_phyclk_mhz = dcn3_0_soc.clock_limits[0].phyclk_mhz as i32; } }

#[no_mangle]
pub unsafe extern "C" fn dcn30_fpu_get_optimal_dcfclk_fclk_for_uclk(uclk_mts: u32, optimal_dcfclk: *mut u32, optimal_fclk: *mut u32) { dc_assert_fp_enabled(); let a = uclk_mts as f64 * dcn3_0_soc.num_chans as f64 * dcn3_0_soc.dram_channel_width_bytes as f64 * (dcn3_0_soc.max_avg_dram_bw_use_normal_percent / 100.0); let b = uclk_mts as f64 * dcn3_0_soc.num_chans as f64 * dcn3_0_soc.dram_channel_width_bytes as f64 * (dcn3_0_soc.max_avg_sdp_bw_use_normal_percent / 100.0); let bw = a.min(b); if !optimal_fclk.is_null() { *optimal_fclk = (bw / (dcn3_0_soc.fabric_datapath_to_dcn_data_return_bytes as f64 * dcn3_0_soc.max_avg_sdp_bw_use_normal_percent / 100.0)) as u32; } if !optimal_dcfclk.is_null() { *optimal_dcfclk = (bw / (dcn3_0_soc.return_bus_width_bytes as f64 * dcn3_0_soc.max_avg_sdp_bw_use_normal_percent / 100.0)) as u32; } }

#[no_mangle]
pub unsafe extern "C" fn frl_fpu_cap_chk_common(enc: *mut hpo_frl_stream_encoder, inter: *mut frl_cap_chk_intermediates, params: *mut frl_cap_chk_params) -> frl_cap_chk_result { let _ = enc; dml1_frl_cap_chk_common(inter, params) }
#[no_mangle]
pub unsafe extern "C" fn frl_fpu_cap_chk_uncompressed(enc: *mut hpo_frl_stream_encoder, params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result { let _ = enc; dml1_frl_cap_chk_uncompressed(params, inter) }
#[no_mangle]
pub unsafe extern "C" fn frl_fpu_cap_chk_compressed(enc: *mut hpo_frl_stream_encoder, params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result { let _ = enc; dml1_frl_cap_chk_compressed(params, inter) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
