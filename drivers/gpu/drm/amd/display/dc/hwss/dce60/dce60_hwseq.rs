/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
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
 *
 * Authors: AMD
 */

// C dependencies: dm_services.h, dc.h, core_types.h, dce60_hwseq.h,
// dce/dce_hwseq.h, dce110/dce110_hwseq.h, dce100/dce100_hwseq.h,
// dce/dce_6_0_d.h, and dce/dce_6_0_sh_mask.h.

/* Private definitions */

unsafe fn dce60_should_enable_fbc(
    dc: *mut dc,
    context: *mut dc_state,
    pipe_idx: *mut u32,
) -> bool {
    let mut i: u32;
    let mut pipe_ctx: *mut pipe_ctx = core::ptr::null_mut();
    let res_ctx: *mut resource_context = &mut (*context).res_ctx;
    let underlay_idx: u32 = (*(*dc).res_pool).underlay_pipe_index;

    ASSERT(!(*dc).fbc_compressor.is_null());
    if (*(*dc).ctx).fbc_gpu_addr == 0 { return false; }
    if (*context).stream_count != 1 { return false; }

    i = 0;
    while i < (*(*dc).res_pool).pipe_count {
        if !(*res_ctx).pipe_ctx[i as usize].stream.is_null() {
            pipe_ctx = &mut (*res_ctx).pipe_ctx[i as usize];
            if pipe_ctx.is_null() { continue; }
            if (*pipe_ctx).pipe_idx != underlay_idx {
                *pipe_idx = i;
                break;
            }
        }
        i += 1;
    }
    if i == (*(*dc).res_pool).pipe_count { return false; }
    if (*pipe_ctx).stream.is_null() || (*(*pipe_ctx).stream).link.is_null() { return false; }
    if (*(*pipe_ctx).stream).link.connector_signal != SIGNAL_TYPE_EDP { return false; }
    if (*(*pipe_ctx).stream).link.psr_settings.psr_feature_enabled { return false; }
    if (*pipe_ctx).plane_state.is_null() { return false; }
    if (*pipe_ctx).plane_state.tiling_info.gfx8.array_mode == DC_ARRAY_LINEAR_GENERAL { return false; }
    true
}

unsafe fn dce60_enable_fbc(dc: *mut dc, context: *mut dc_state) {
    let mut pipe_idx: u32 = 0;
    if dce60_should_enable_fbc(dc, context, &mut pipe_idx) {
        let mut params = compr_addr_and_pitch_params { source_view_width: 0, source_view_height: 0, inst: 0 };
        let compr = (*dc).fbc_compressor;
        let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[pipe_idx as usize];
        params.source_view_width = (*pipe_ctx).stream.timing.h_addressable;
        params.source_view_height = (*pipe_ctx).stream.timing.v_addressable;
        params.inst = (*pipe_ctx).stream_res.tg.inst;
        (*compr).compr_surface_address.quad_part = (*(*dc).ctx).fbc_gpu_addr;
        ((*compr).funcs.surface_address_and_pitch)(compr, &mut params);
        ((*compr).funcs.set_fbc_invalidation_triggers)(compr, 1);
        ((*compr).funcs.enable_fbc)(compr, &mut params);
    }
}

unsafe fn dce60_set_default_colors(pipe_ctx: *mut pipe_ctx) {
    let mut default_adjust = default_adjustment::default();
    default_adjust.force_hw_default = false;
    default_adjust.in_color_space = (*(*pipe_ctx).plane_state).color_space;
    default_adjust.out_color_space = (*(*pipe_ctx).stream).output_color_space;
    default_adjust.csc_adjust_type = GRAPHICS_CSC_ADJUST_TYPE_SW;
    default_adjust.surface_pixel_format = (*pipe_ctx).plane_res.scl_data.format;
    default_adjust.color_depth = (*(*pipe_ctx).stream).timing.display_color_depth;
    default_adjust.lb_color_depth = (*pipe_ctx).plane_res.scl_data.lb_params.depth;
    ((*(*pipe_ctx).plane_res.xfm).funcs.opp_set_csc_default)(
        (*pipe_ctx).plane_res.xfm, &mut default_adjust);
}

unsafe fn dce60_program_surface_visibility(_dc: *const dc, pipe_ctx: *mut pipe_ctx) {
    let blank_target = !(*(*pipe_ctx).plane_state).visible;
    ((*(*pipe_ctx).stream_res.tg).funcs.set_blank)((*pipe_ctx).stream_res.tg, blank_target);
}

unsafe fn dce60_get_surface_visual_confirm_color(pipe_ctx: *const pipe_ctx, color: *mut tg_color) {
    let color_value = MAX_TG_COLOR_VALUE * (4 - (*pipe_ctx).stream_res.tg.inst) / 4;
    match (*pipe_ctx).plane_res.scl_data.format {
        PIXEL_FORMAT_ARGB8888 => (*color).color_r_cr = color_value,
        PIXEL_FORMAT_ARGB2101010 => (*color).color_b_cb = color_value,
        PIXEL_FORMAT_420BPP8 => (*color).color_g_y = color_value,
        PIXEL_FORMAT_420BPP10 => { (*color).color_g_y = color_value; (*color).color_r_cr = color_value; }
        PIXEL_FORMAT_FP16 => { (*color).color_r_cr = color_value; (*color).color_b_cb = color_value; (*color).color_g_y = color_value; }
        _ => {}
    }
}

unsafe fn dce60_program_scaler(dc: *const dc, pipe_ctx: *const pipe_ctx) {
    let mut color = tg_color::default();
    if (*dc).debug.visual_confirm == VISUAL_CONFIRM_SURFACE {
        dce60_get_surface_visual_confirm_color(pipe_ctx, &mut color);
    } else {
        color_space_to_black_color(dc, (*pipe_ctx).stream.output_color_space, &mut color);
    }
    ((*(*pipe_ctx).plane_res.xfm).funcs.transform_set_pixel_storage_depth)(
        (*pipe_ctx).plane_res.xfm, (*pipe_ctx).plane_res.scl_data.lb_params.depth,
        &mut (*pipe_ctx).stream.bit_depth_params);
    if let Some(set_color) = (*(*pipe_ctx).stream_res.tg).funcs.set_overscan_blank_color {
        if (*pipe_ctx).stream.timing.pixel_encoding == PIXEL_ENCODING_YCBCR420 { color.color_r_cr = color.color_g_y; }
        set_color((*pipe_ctx).stream_res.tg, &mut color);
    }
    ((*(*pipe_ctx).plane_res.xfm).funcs.transform_set_scaler)(
        (*pipe_ctx).plane_res.xfm, &mut (*pipe_ctx).plane_res.scl_data);
}

unsafe fn dce60_program_front_end_for_pipe(dc: *mut dc, pipe_ctx: *mut pipe_ctx) {
    let mi = (*pipe_ctx).plane_res.mi;
    let plane_state = (*pipe_ctx).plane_state;
    let mut adjust = xfm_grph_csc_adjustment::default();
    let mut tbl_entry = out_csc_color_matrix::default();
    let hws = (*dc).hwseq;
    adjust.gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS;
    dce_enable_fe_clock((*dc).hwseq, (*mi).inst, true);
    dce60_set_default_colors(pipe_ctx);
    if (*pipe_ctx).stream.csc_color_matrix.enable_adjustment == true {
        tbl_entry.color_space = (*pipe_ctx).stream.output_color_space;
        for i in 0..12 { tbl_entry.regval[i] = (*pipe_ctx).stream.csc_color_matrix.matrix[i]; }
        ((*(*pipe_ctx).plane_res.xfm).funcs.opp_set_csc_adjustment)((*pipe_ctx).plane_res.xfm, &mut tbl_entry);
    }
    if (*pipe_ctx).stream.gamut_remap_matrix.enable_remap == true {
        adjust.gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_SW;
        for i in 0..CSC_TEMPERATURE_MATRIX_SIZE { adjust.temperature_matrix[i] = (*pipe_ctx).stream.gamut_remap_matrix.matrix[i]; }
    }
    ((*(*pipe_ctx).plane_res.xfm).funcs.transform_set_gamut_remap)((*pipe_ctx).plane_res.xfm, &mut adjust);
    (*pipe_ctx).plane_res.scl_data.lb_params.alpha_en = (*pipe_ctx).bottom_pipe != 0;
    dce60_program_scaler(dc, pipe_ctx);
    ((*mi).funcs.mem_input_program_surface_config)(mi, (*plane_state).format, &mut (*plane_state).tiling_info, &mut (*plane_state).plane_size, (*plane_state).rotation, core::ptr::null_mut(), false);
    if let Some(set_blank) = (*mi).funcs.set_blank { set_blank(mi, (*plane_state).visible); }
    if (*dc).config.gpu_vm_support { ((*mi).funcs.mem_input_program_pte_vm)(mi, (*plane_state).format, &mut (*plane_state).tiling_info, (*plane_state).rotation); }
    if (*plane_state).update_bits.full_update || (*plane_state).update_bits.in_transfer_func_change || (*plane_state).update_bits.gamma_change { ((*hws).funcs.set_input_transfer_func)(dc, pipe_ctx, plane_state); }
    if (*plane_state).update_bits.full_update { hwss_set_output_transfer_func(dc, pipe_ctx); }
}

unsafe fn dce60_apply_ctx_for_surface(dc: *mut dc, stream: *const dc_stream_state, num_planes: i32, context: *mut dc_state) {
    if num_planes == 0 { return; }
    if !(*dc).fbc_compressor.is_null() { ((*(*dc).fbc_compressor).funcs.disable_fbc)((*dc).fbc_compressor); }
    for i in 0..(*(*dc).res_pool).pipe_count {
        let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.stream != stream { continue; }
        ((*(*pipe_ctx).plane_res.mi).funcs.allocate_mem_input)((*pipe_ctx).plane_res.mi, (*pipe_ctx).stream.timing.h_total, (*pipe_ctx).stream.timing.v_total, (*pipe_ctx).stream.timing.pix_clk_100hz / 10, (*context).stream_count);
        dce60_program_front_end_for_pipe(dc, pipe_ctx);
        ((*dc).hwss.update_plane_addr)(dc, pipe_ctx);
        dce60_program_surface_visibility(dc, pipe_ctx);
    }
    if !(*dc).fbc_compressor.is_null() { dce60_enable_fbc(dc, context); }
}

pub unsafe fn dce60_hw_sequencer_construct(dc: *mut dc) {
    dce110_hw_sequencer_construct(dc);
    (*dc).hwseq.funcs.enable_display_power_gating = Some(dce100_enable_display_power_gating);
    (*dc).hwss.apply_ctx_for_surface = Some(dce60_apply_ctx_for_surface);
    (*dc).hwss.cursor_lock = Some(dce60_pipe_control_lock);
    (*dc).hwss.pipe_control_lock = Some(dce60_pipe_control_lock);
    (*dc).hwss.prepare_bandwidth = Some(dce100_prepare_bandwidth);
    (*dc).hwss.optimize_bandwidth = Some(dce100_optimize_bandwidth);
    (*dc).hwss.clear_surface_dcc_and_tiling = Some(dce100_reset_surface_dcc_and_tiling);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
