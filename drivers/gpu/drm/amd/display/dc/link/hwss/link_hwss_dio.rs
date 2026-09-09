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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

pub unsafe fn set_dio_throttled_vcp_size(pipe_ctx: *mut pipe_ctx, throttled_vcp_size: fixed31_32) {
    let stream_encoder = (*pipe_ctx).stream_res.stream_enc;
    ((*stream_encoder).funcs.set_throttled_vcp_size)(stream_encoder, throttled_vcp_size);
}

pub unsafe fn setup_dio_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let mut link_enc = (*pipe_ctx).link_res.dio_link_enc;
    let stream_enc = (*pipe_ctx).stream_res.stream_enc;
    if !(*(*(*pipe_ctx).stream).ctx).dc.config.unify_link_enc_assignment {
        link_enc = link_enc_cfg_get_link_enc((*pipe_ctx).stream.link);
    }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    if !dc_is_rgb_signal((*pipe_ctx).stream.signal) {
        ((*link_enc).funcs.connect_dig_be_to_fe)(link_enc, (*stream_enc).id, true);
    }
    if dc_is_dp_signal((*pipe_ctx).stream.signal) {
        ((*(*(*(*pipe_ctx).stream).ctx).dc).link_srv.dp_trace_source_sequence)(
            (*pipe_ctx).stream.link, DPCD_SOURCE_SEQ_AFTER_CONNECT_DIG_FE_BE);
    }
    if let Some(f) = (*stream_enc).funcs.enable_stream {
        f(stream_enc, (*pipe_ctx).stream.signal, true);
    }
    if let Some(f) = (*stream_enc).funcs.map_stream_to_link {
        f(stream_enc, (*stream_enc).stream_enc_inst,
          (*link_enc).transmitter - TRANSMITTER_UNIPHY_A);
    }
    if let Some(f) = (*stream_enc).funcs.set_input_mode {
        f(stream_enc, (*pipe_ctx).stream_res.pix_clk_params.dio_se_pix_per_cycle);
    }
    if let Some(f) = (*stream_enc).funcs.enable_fifo { f(stream_enc); }
}

pub unsafe fn reset_dio_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let mut link_enc = (*pipe_ctx).link_res.dio_link_enc;
    let stream_enc = (*pipe_ctx).stream_res.stream_enc;
    if !(*(*(*pipe_ctx).stream).ctx).dc.config.unify_link_enc_assignment {
        link_enc = link_enc_cfg_get_link_enc((*pipe_ctx).stream.link);
    }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    if stream_enc.is_null() { return; }
    if let Some(f) = (*stream_enc).funcs.disable_fifo { f(stream_enc); }
    if let Some(f) = (*stream_enc).funcs.set_input_mode { f(stream_enc, 0); }
    if let Some(f) = (*stream_enc).funcs.enable_stream {
        f(stream_enc, (*pipe_ctx).stream.signal, false);
    }
    if !dc_is_rgb_signal((*pipe_ctx).stream.signal) {
        ((*link_enc).funcs.connect_dig_be_to_fe)(link_enc, (*stream_enc).id, false);
    }
    if dc_is_dp_signal((*pipe_ctx).stream.signal) {
        ((*(*(*(*pipe_ctx).stream).ctx).dc).link_srv.dp_trace_source_sequence)(
            (*pipe_ctx).stream.link, DPCD_SOURCE_SEQ_AFTER_DISCONNECT_DIG_FE_BE);
    }
}

pub unsafe fn setup_dio_stream_attribute(pipe_ctx: *mut pipe_ctx) {
    let stream_encoder = (*pipe_ctx).stream_res.stream_enc;
    let stream = (*pipe_ctx).stream;
    let link = (*stream).link;
    if !dc_is_virtual_signal((*stream).signal) && !dc_is_rgb_signal((*stream).signal) {
        ((*stream_encoder).funcs.setup_stereo_sync)(stream_encoder, (*pipe_ctx).stream_res.tg.inst,
            (*stream).timing.timing_3d_format != TIMING_3D_FORMAT_NONE);
    }
    if dc_is_dp_signal((*stream).signal) {
        ((*stream_encoder).funcs.dp_set_stream_attribute)(stream_encoder, &(*stream).timing,
            (*stream).output_color_space, (*stream).use_vsc_sdp_for_colorimetry,
            (*link).dpcd_caps.dprx_feature.bits.SST_SPLIT_SDP_CAP);
    } else if dc_is_hdmi_tmds_signal((*stream).signal) {
        ((*stream_encoder).funcs.hdmi_set_stream_attribute)(stream_encoder, &(*stream).timing,
            (*stream).phy_pix_clk, !(*pipe_ctx).stream_res.audio.is_null());
    } else if dc_is_dvi_signal((*stream).signal) {
        ((*stream_encoder).funcs.dvi_set_stream_attribute)(stream_encoder, &(*stream).timing,
            (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK);
    } else if dc_is_lvds_signal((*stream).signal) {
        ((*stream_encoder).funcs.lvds_set_stream_attribute)(stream_encoder, &(*stream).timing);
    }
    if dc_is_dp_signal((*stream).signal) {
        ((*(*link).dc).link_srv.dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_DP_STREAM_ATTR);
    }
}

pub unsafe fn enable_dio_dp_link_output(link: *mut dc_link, link_res: *const link_resource,
    signal: signal_type, clock_source: clock_source_id, link_settings: *const dc_link_settings) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    if dc_is_dp_sst_signal(signal) { ((*link_enc).funcs.enable_dp_output)(link_enc, link_settings, clock_source); }
    else { ((*link_enc).funcs.enable_dp_mst_output)(link_enc, link_settings, clock_source); }
    ((*(*link).dc).link_srv.dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_ENABLE_LINK_PHY);
}

pub unsafe fn disable_dio_link_output(link: *mut dc_link, link_res: *const link_resource, signal: signal_type) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    ((*link_enc).funcs.disable_output)(link_enc, signal);
    ((*(*link).dc).link_srv.dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_DISABLE_LINK_PHY);
}

pub unsafe fn set_dio_dp_link_test_pattern(link: *mut dc_link, link_res: *const link_resource,
    tp_params: *mut encoder_set_dp_phy_pattern_param) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    ((*link_enc).funcs.dp_set_phy_pattern)(link_enc, tp_params);
    ((*(*link).dc).link_srv.dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN);
}

pub unsafe fn set_dio_dp_lane_settings(link: *mut dc_link, link_res: *const link_resource,
    link_settings: *const dc_link_settings, lane_settings: *const dc_lane_settings) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    ((*link_enc).funcs.dp_set_lane_settings)(link_enc, link_settings, lane_settings);
}

pub unsafe fn update_dio_stream_allocation_table(link: *mut dc_link, link_res: *const link_resource,
    table: *const link_mst_stream_allocation_table) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { ASSERT(!link_enc.is_null()); return; }
    ((*link_enc).funcs.update_mst_stream_allocation_table)(link_enc, table);
}

pub unsafe fn setup_dio_audio_output(pipe_ctx: *mut pipe_ctx, audio_output: *mut audio_output, audio_inst: u32) {
    let enc = (*pipe_ctx).stream_res.stream_enc;
    if dc_is_dp_signal((*pipe_ctx).stream.signal) {
        ((*enc).funcs.dp_audio_setup)(enc, audio_inst, &(*(*pipe_ctx).stream).audio_info);
    } else {
        ((*enc).funcs.hdmi_audio_setup)(enc, audio_inst, &(*(*pipe_ctx).stream).audio_info, &(*audio_output).crtc_info);
    }
}

pub unsafe fn enable_dio_audio_packet(pipe_ctx: *mut pipe_ctx) {
    let enc = (*pipe_ctx).stream_res.stream_enc;
    if dc_is_dp_signal((*pipe_ctx).stream.signal) { ((*enc).funcs.dp_audio_enable)(enc); }
    ((*enc).funcs.audio_mute_control)(enc, false);
    if dc_is_dp_signal((*pipe_ctx).stream.signal) {
        ((*(*(*(*pipe_ctx).stream).ctx).dc).link_srv.dp_trace_source_sequence)(
            (*pipe_ctx).stream.link, DPCD_SOURCE_SEQ_AFTER_ENABLE_AUDIO_STREAM);
    }
}

pub unsafe fn disable_dio_audio_packet(pipe_ctx: *mut pipe_ctx) {
    let enc = (*pipe_ctx).stream_res.stream_enc;
    ((*enc).funcs.audio_mute_control)(enc, true);
    if !(*pipe_ctx).stream_res.audio.is_null() {
        if dc_is_dp_signal((*pipe_ctx).stream.signal) { ((*enc).funcs.dp_audio_disable)(enc); }
        else { ((*enc).funcs.hdmi_audio_disable)(enc); }
    }
    if dc_is_dp_signal((*pipe_ctx).stream.signal) {
        ((*(*(*(*pipe_ctx).stream).ctx).dc).link_srv.dp_trace_source_sequence)(
            (*pipe_ctx).stream.link, DPCD_SOURCE_SEQ_AFTER_DISABLE_AUDIO_STREAM);
    }
}

static dio_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: Some(setup_dio_stream_encoder),
    reset_stream_encoder: Some(reset_dio_stream_encoder),
    setup_stream_attribute: Some(setup_dio_stream_attribute),
    disable_link_output: Some(disable_dio_link_output),
    setup_audio_output: Some(setup_dio_audio_output),
    enable_audio_packet: Some(enable_dio_audio_packet),
    disable_audio_packet: Some(disable_dio_audio_packet),
    ext: link_hwss_ext {
        set_throttled_vcp_size: Some(set_dio_throttled_vcp_size),
        enable_dp_link_output: Some(enable_dio_dp_link_output),
        set_dp_link_test_pattern: Some(set_dio_dp_link_test_pattern),
        set_dp_lane_settings: Some(set_dio_dp_lane_settings),
        update_stream_allocation_table: Some(update_dio_stream_allocation_table),
    },
};

pub unsafe fn can_use_dio_link_hwss(link: *const dc_link, link_res: *const link_resource) -> bool {
    if !(*(*link).dc).config.unify_link_enc_assignment { !(*link).link_enc.is_null() }
    else { !(*link_res).dio_link_enc.is_null() }
}

pub unsafe fn get_dio_link_hwss() -> *const link_hwss { &dio_link_hwss }

extern "C" {
    fn link_enc_cfg_get_link_enc(link: *mut dc_link) -> *mut link_encoder;
    fn dc_is_rgb_signal(signal: signal_type) -> bool;
    fn dc_is_dp_signal(signal: signal_type) -> bool;
    fn dc_is_dp_sst_signal(signal: signal_type) -> bool;
    fn dc_is_virtual_signal(signal: signal_type) -> bool;
    fn dc_is_hdmi_tmds_signal(signal: signal_type) -> bool;
    fn dc_is_dvi_signal(signal: signal_type) -> bool;
    fn dc_is_lvds_signal(signal: signal_type) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
