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
 */

pub unsafe fn set_hpo_dp_throttled_vcp_size(
    pipe_ctx: *mut pipe_ctx,
    throttled_vcp_size: fixed31_32,
) {
    let hpo_dp_stream_encoder = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    let hpo_dp_link_encoder = (*pipe_ctx).link_res.hpo_dp_link_enc;
    ((*(*hpo_dp_link_encoder).funcs).set_throttled_vcp_size)(
        hpo_dp_link_encoder,
        (*hpo_dp_stream_encoder).inst,
        throttled_vcp_size,
    );
}

pub unsafe fn set_hpo_dp_hblank_min_symbol_width(
    pipe_ctx: *mut pipe_ctx,
    link_settings: *const dc_link_settings,
    throttled_vcp_size: fixed31_32,
) {
    let hpo_dp_stream_encoder = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    let timing = &mut (*(*pipe_ctx).stream).timing;
    let mut h_blank_in_ms: fixed31_32;
    let mut time_slot_in_ms: fixed31_32;
    let mut mtp_cnt_per_h_blank: fixed31_32;
    let link_bw_in_kbps = ((*(*(*hpo_dp_stream_encoder).ctx).dc).link_srv).dp_link_bandwidth_kbps(
        (*pipe_ctx).stream.link,
        link_settings,
    );
    let mut hblank_min_symbol_width: u16 = 0;

    if link_bw_in_kbps > 0 {
        h_blank_in_ms = dc_fixpt_div(
            dc_fixpt_from_int(timing.h_total - timing.h_addressable),
            dc_fixpt_from_fraction(timing.pix_clk_100hz, 10),
        );
        time_slot_in_ms = dc_fixpt_from_fraction(32 * 4, link_bw_in_kbps);
        mtp_cnt_per_h_blank = dc_fixpt_div(
            h_blank_in_ms,
            dc_fixpt_mul_int(time_slot_in_ms, 64),
        );
        hblank_min_symbol_width = dc_fixpt_floor(
            dc_fixpt_mul(mtp_cnt_per_h_blank, throttled_vcp_size),
        ) as u16;
    }

    ((*(*hpo_dp_stream_encoder).funcs).set_hblank_min_symbol_width)(
        hpo_dp_stream_encoder,
        hblank_min_symbol_width,
    );
}

pub unsafe fn setup_hpo_dp_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let stream_enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    let link_enc = (*pipe_ctx).link_res.hpo_dp_link_enc;
    ((*(*stream_enc).funcs).enable_stream)(stream_enc);
    ((*(*stream_enc).funcs).map_stream_to_link)(stream_enc, (*stream_enc).inst, (*link_enc).inst);
}

pub unsafe fn reset_hpo_dp_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let stream_enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    ((*(*stream_enc).funcs).disable)(stream_enc);
}

pub unsafe fn setup_hpo_dp_stream_attribute(pipe_ctx: *mut pipe_ctx) {
    let stream_enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    let stream = (*pipe_ctx).stream;
    let link = (*stream).link;
    ((*(*stream_enc).funcs).set_stream_attribute)(
        stream_enc,
        &mut (*stream).timing,
        (*stream).output_color_space,
        (*stream).use_vsc_sdp_for_colorimetry,
        ((*stream).timing.flags.DSC != 0),
        false,
    );
    ((*(*(*link).dc).link_srv).dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_DP_STREAM_ATTR);
}

pub unsafe fn enable_hpo_dp_link_output(
    link: *mut dc_link,
    link_res: *const link_resource,
    _signal: signal_type,
    _clock_source: clock_source_id,
    link_settings: *const dc_link_settings,
) {
    if (*link_res).hpo_dp_link_enc.is_null() {
        DC_LOG_ERROR("%s: invalid hpo_dp_link_enc\n", "enable_hpo_dp_link_output");
        return;
    }
    let dccg = (*(*(*link).dc).res_pool).dccg;
    if !(*(*dccg).funcs).set_symclk32_le_root_clock_gating.is_none() {
        ((*(*dccg).funcs).set_symclk32_le_root_clock_gating.unwrap())(
            dccg, (*(*link_res).hpo_dp_link_enc).inst, true,
        );
    }
    ((*(*(*link_res).hpo_dp_link_enc).funcs).enable_link_phy)(
        (*link_res).hpo_dp_link_enc,
        link_settings,
        (*(*link).link_enc).transmitter,
        (*(*link).link_enc).hpd_source,
    );
}

pub unsafe fn disable_hpo_dp_link_output(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
) {
    if (*link_res).hpo_dp_link_enc.is_null() {
        DC_LOG_ERROR("%s: invalid hpo_dp_link_enc\n", "disable_hpo_dp_link_output");
        return;
    }
    let enc = (*link_res).hpo_dp_link_enc;
    ((*(*enc).funcs).link_disable)(enc);
    ((*(*enc).funcs).disable_link_phy)(enc, signal);
    let dccg = (*(*(*link).dc).res_pool).dccg;
    if !(*(*dccg).funcs).set_symclk32_le_root_clock_gating.is_none() {
        ((*(*dccg).funcs).set_symclk32_le_root_clock_gating.unwrap())(dccg, (*enc).inst, false);
    }
}

unsafe fn set_hpo_dp_link_test_pattern(
    link: *mut dc_link,
    link_res: *const link_resource,
    tp_params: *mut encoder_set_dp_phy_pattern_param,
) {
    let enc = (*link_res).hpo_dp_link_enc;
    ((*(*enc).funcs).set_link_test_pattern)(enc, tp_params);
    ((*(*(*link).dc).link_srv).dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN);
}

unsafe fn set_hpo_dp_lane_settings(
    _link: *mut dc_link,
    link_res: *const link_resource,
    link_settings: *const dc_link_settings,
    lane_settings: *const dc_lane_settings,
) {
    let enc = (*link_res).hpo_dp_link_enc;
    ((*(*enc).funcs).set_ffe)(enc, link_settings, (*lane_settings).FFE_PRESET.raw);
}

pub unsafe fn update_hpo_dp_stream_allocation_table(
    _link: *mut dc_link,
    link_res: *const link_resource,
    table: *const link_mst_stream_allocation_table,
) {
    let enc = (*link_res).hpo_dp_link_enc;
    ((*(*enc).funcs).update_stream_allocation_table)(enc, table);
}

pub unsafe fn setup_hpo_dp_audio_output(
    pipe_ctx: *mut pipe_ctx,
    _audio_output: *mut audio_output,
    audio_inst: u32,
) {
    let enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    ((*(*enc).funcs).dp_audio_setup)(enc, audio_inst, &(*(*pipe_ctx).stream).audio_info);
}

pub unsafe fn enable_hpo_dp_audio_packet(pipe_ctx: *mut pipe_ctx) {
    let enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
    ((*(*enc).funcs).dp_audio_enable)(enc);
}

pub unsafe fn disable_hpo_dp_audio_packet(pipe_ctx: *mut pipe_ctx) {
    if !(*pipe_ctx).stream_res.audio.is_null() {
        let enc = (*pipe_ctx).stream_res.hpo_dp_stream_enc;
        ((*(*enc).funcs).dp_audio_disable)(enc);
    }
}

static HPO_DP_LINK_HWSS: link_hwss = link_hwss {
    setup_stream_encoder: Some(setup_hpo_dp_stream_encoder),
    reset_stream_encoder: Some(reset_hpo_dp_stream_encoder),
    setup_stream_attribute: Some(setup_hpo_dp_stream_attribute),
    disable_link_output: Some(disable_hpo_dp_link_output),
    setup_audio_output: Some(setup_hpo_dp_audio_output),
    enable_audio_packet: Some(enable_hpo_dp_audio_packet),
    disable_audio_packet: Some(disable_hpo_dp_audio_packet),
    ext: link_hwss_ext {
        set_throttled_vcp_size: Some(set_hpo_dp_throttled_vcp_size),
        set_hblank_min_symbol_width: Some(set_hpo_dp_hblank_min_symbol_width),
        enable_dp_link_output: Some(enable_hpo_dp_link_output),
        set_dp_link_test_pattern: Some(set_hpo_dp_link_test_pattern),
        set_dp_lane_settings: Some(set_hpo_dp_lane_settings),
        update_stream_allocation_table: Some(update_hpo_dp_stream_allocation_table),
    },
};

pub unsafe fn can_use_hpo_dp_link_hwss(
    _link: *const dc_link,
    link_res: *const link_resource,
) -> bool {
    !(*link_res).hpo_dp_link_enc.is_null()
}

pub unsafe fn get_hpo_dp_link_hwss() -> *const link_hwss {
    &HPO_DP_LINK_HWSS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
