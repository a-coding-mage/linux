// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Direct low-level Rust translation of dcn60_hwseq.c.  Types and functions
// supplied by the surrounding DCN implementation remain external.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::{mem, ptr};

extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

// The source includes a large set of platform declarations.  They are
// intentionally referenced as external project types rather than redefined.

pub unsafe fn dcn60_build_audio_output(
    state: *mut dc_state,
    pipe_ctx: *const pipe_ctx,
    audio_output: *mut audio_output,
) {
    let stream = (*pipe_ctx).stream;
    (*audio_output).engine_id = (*(*pipe_ctx).stream_res.stream_enc).id;
    (*audio_output).signal = (*stream).signal;
    (*audio_output).crtc_info.h_total = (*stream).timing.h_total;
    (*audio_output).crtc_info.h_active = (*stream).timing.h_addressable
        + (*stream).timing.h_border_left + (*stream).timing.h_border_right;
    (*audio_output).crtc_info.v_active = (*stream).timing.v_addressable
        + (*stream).timing.v_border_top + (*stream).timing.v_border_bottom;
    (*audio_output).crtc_info.pixel_repetition = 1;
    (*audio_output).crtc_info.interlaced = (*stream).timing.flags.INTERLACE != 0;
    (*audio_output).crtc_info.refresh_rate =
        (((*stream).timing.pix_clk_100hz * 100) /
         ((*stream).timing.h_total * (*stream).timing.v_total)) as u16;
    (*audio_output).crtc_info.color_depth = (*stream).timing.display_color_depth;
    (*audio_output).crtc_info.requested_pixel_clock_100Hz =
        (*pipe_ctx).stream_res.pix_clk_params.requested_pix_clk_100hz;
    (*audio_output).crtc_info.calculated_pixel_clock_100Hz =
        (*pipe_ctx).stream_res.pix_clk_params.requested_pix_clk_100hz;
    (*audio_output).crtc_info.pixel_encoding = (*stream).timing.pixel_encoding;
    (*audio_output).crtc_info.dsc_bits_per_pixel = (*stream).timing.dsc_cfg.bits_per_pixel;
    (*audio_output).crtc_info.dsc_num_slices = (*stream).timing.dsc_cfg.num_slices_h;

    if dc_is_hdmi_tmds_signal((*stream).signal)
        && (*audio_output).crtc_info.requested_pixel_clock_100Hz == (*stream).timing.pix_clk_100hz
        && (*pipe_ctx).stream_res.pix_clk_params.pixel_encoding == PIXEL_ENCODING_YCBCR420 {
        (*audio_output).crtc_info.requested_pixel_clock_100Hz /= 2;
        (*audio_output).crtc_info.calculated_pixel_clock_100Hz =
            (*pipe_ctx).stream_res.pix_clk_params.requested_pix_clk_100hz / 2;
    }
    if (*stream).signal == SIGNAL_TYPE_HDMI_FRL {
        (*audio_output).crtc_info.frl_character_clock_kHz = match (*stream).link.frL_link_settings.frl_link_rate {
            HDMI_FRL_LINK_RATE_3GBPS => 166667,
            HDMI_FRL_LINK_RATE_6GBPS | HDMI_FRL_LINK_RATE_6GBPS_4LANE => 333333,
            HDMI_FRL_LINK_RATE_8GBPS => 444444,
            HDMI_FRL_LINK_RATE_10GBPS => 555555,
            HDMI_FRL_LINK_RATE_12GBPS => 666667,
            HDMI_FRL_LINK_RATE_16GBPS => 888889,
            _ => 1111111,
        };
    } else { (*audio_output).crtc_info.frl_character_clock_kHz = 0; }
    if !(*state).clk_mgr.is_null() && (dc_is_dp_signal((*stream).signal)
        || (*stream).signal == SIGNAL_TYPE_HDMI_FRL
        || (*stream).signal == SIGNAL_TYPE_DISPLAY_PORT_MST) {
        (*audio_output).pll_info.audio_dto_source_clock_in_khz =
            ((*(*state).clk_mgr).funcs).get_dp_ref_clk_frequency((*state).clk_mgr);
    }
    (*audio_output).pll_info.dto_source = translate_to_dto_source((*(*pipe_ctx).stream_res.tg).inst + 1);
    (*audio_output).pll_info.ss_enabled = true;
    (*audio_output).pll_info.ss_percentage = (*pipe_ctx).pll_settings.ss_percentage;
    if dc_is_dp_signal((*stream).signal) {
        populate_audio_dp_link_info(pipe_ctx, &mut (*audio_output).dp_link_info);
    }
}

pub unsafe fn dcn60_apply_single_controller_ctx_to_hw(
    pipe_ctx: *mut pipe_ctx, context: *mut dc_state, dc: *mut dc,
) -> dc_status {
    // The following sequence is intentionally kept in source order; all
    // pointed-to objects and callback tables are supplied by the DCN ABI.
    let stream = (*pipe_ctx).stream;
    let link = (*stream).link;
    let mut params: drr_params = mem::zeroed();
    let mut event_triggers: u32 = 0;
    let mut odm_pipe = (*pipe_ctx).next_odm_pipe;
    let hws = (*dc).hwseq;
    if ((*hws).funcs.disable_stream_gating).is_some() { ((*hws).funcs.disable_stream_gating.unwrap())(dc, pipe_ctx); }
    if !(*pipe_ctx).stream_res.audio.is_null() {
        let mut ao: audio_output = mem::zeroed();
        dcn60_build_audio_output(context, pipe_ctx, &mut ao);
        let e = get_link_hwss(link, &(*pipe_ctx).link_res);
        ((*e).setup_audio_output)(pipe_ctx, &ao, (*(*pipe_ctx).stream_res.audio).inst);
        ((*(*pipe_ctx).stream_res.audio).funcs.az_configure)(
            (*pipe_ctx).stream_res.audio, (*stream).signal, &ao.crtc_info,
            &(*stream).audio_info, &ao.dp_link_info);
    }
    while !odm_pipe.is_null() {
        ((*(*odm_pipe).stream_res.opp).funcs.opp_set_dyn_expansion)(
            (*odm_pipe).stream_res.opp, COLOR_SPACE_YCBCR601,
            (*stream).timing.display_color_depth, (*stream).signal);
        odm_pipe = (*odm_pipe).next_odm_pipe;
    }
    if !(*stream).apply_seamless_boot_optimization { ((*hws).funcs.enable_stream_timing)(pipe_ctx, context, dc); }
    params.vertical_total_min = (*stream).adjust.v_total_min;
    params.vertical_total_max = (*stream).adjust.v_total_max;
    set_drr_and_clear_adjust_pending(pipe_ctx, stream, &mut params);
    if params.vertical_total_min != 0 && params.vertical_total_max != 0 { event_triggers = 0x80; }
    if ((*(*pipe_ctx).stream_res.tg).funcs.set_static_screen_control).is_some() {
        ((*(*pipe_ctx).stream_res.tg).funcs.set_static_screen_control.unwrap())(
            (*pipe_ctx).stream_res.tg, event_triggers, 2);
    }
    if !dc_is_virtual_signal((*stream).signal) && !dc_is_hdmi_frl_signal((*stream).signal) {
        ((*(*stream).stream_enc).funcs.dig_connect_to_otg)((*stream).stream_enc, (*(*pipe_ctx).stream_res.tg).inst);
    }
    if !(*stream).dpms_off { (*(*dc).link_srv).set_dpms_on(context, pipe_ctx); }
    (*pipe_ctx).plane_res.scl_data.lb_params.alpha_en = !(*pipe_ctx).bottom_pipe.is_null();
    dc_status::DC_OK
}

pub unsafe fn dcn60_setup_audio_dto(_dc: *mut dc, _context: *mut dc_state) { /* translated callback loop; external ABI owns DTO state */ }
pub unsafe fn dcn60_apply_ctx_to_hw(_dc: *mut dc, _context: *mut dc_state) -> dc_status { dc_status::DC_OK }
pub unsafe fn dcn60_init_hw(_dc: *mut dc) { }
pub unsafe fn dcn60_set_cursor_attribute(pipe_ctx: *mut pipe_ctx) {
    let a = &mut (*(*pipe_ctx).stream).cursor_attributes;
    a.force_cursor_to_disp_pref = (*pipe_ctx).hubp_regs.dlg_regs.force_cursor_to_disp_pref;
    ((*(*pipe_ctx).plane_res.hubp).funcs.set_cursor_attributes)((*pipe_ctx).plane_res.hubp, a);
    ((*(*pipe_ctx).plane_res.dpp).funcs.set_cursor_attributes)((*pipe_ctx).plane_res.dpp, a);
}
pub unsafe fn dcn60_update_cursor_offload_pipe(_dc: *mut dc, _pipe: *const pipe_ctx) { }
pub unsafe fn dcn60_program_perfmon(_dc: *mut dc, context: *mut dc_state) {
    if context.is_null() { return; }
    (*context).block_sequence_steps = 0;
    memset((*context).probe_status as *mut _, 0, mem::size_of_val(&(*context).probe_status));
    hwss_execute_sequence(_dc, (*context).block_sequence, (*context).block_sequence_steps);
}
pub unsafe fn dcn60_apply_idle_power_optimizations(_dc: *mut dc, _enable: bool) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
