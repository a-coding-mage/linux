/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding DC implementation are intentionally
// left external, matching the declarations provided by the C headers.

unsafe fn write_indirect_azalia_reg(audio: *mut audio, reg_index: u32, reg_data: u32) {
    let aud = DCE_AUD(audio);
    REG_SET(AZALIA_F0_CODEC_ENDPOINT_INDEX, 0, AZALIA_ENDPOINT_REG_INDEX, reg_index);
    REG_SET(AZALIA_F0_CODEC_ENDPOINT_DATA, 0, AZALIA_ENDPOINT_REG_DATA, reg_data);
}

unsafe fn read_indirect_azalia_reg(audio: *mut audio, reg_index: u32) -> u32 {
    let aud = DCE_AUD(audio);
    REG_SET(AZALIA_F0_CODEC_ENDPOINT_INDEX, 0, AZALIA_ENDPOINT_REG_INDEX, reg_index);
    REG_READ(AZALIA_F0_CODEC_ENDPOINT_DATA)
}

unsafe fn is_audio_format_supported(info: *const audio_info, code: audio_format_code,
                                    format_index: *mut u32) -> bool {
    if info.is_null() { return false; }
    let mut index = 0;
    let mut max_channel_index = 0;
    let mut found = false;
    while index < (*info).mode_count {
        if (*info).modes[index as usize].format_code == code {
            if found {
                if (*info).modes[index as usize].channel_count >
                   (*info).modes[max_channel_index as usize].channel_count {
                    max_channel_index = index;
                }
            } else { found = true; max_channel_index = index; }
        }
        index += 1;
    }
    if found && !format_index.is_null() { *format_index = max_channel_index; }
    found
}

unsafe fn check_audio_bandwidth_hdmi(info: *const audio_crtc_info, channels: u32,
                                     rates: *mut audio_sample_rates) {
    if info.is_null() { return; }
    let mut l48 = false; let mut l88 = false; let mut l96 = false; let mut l174 = false;
    if channels > 2 {
        if (*info).requested_pixel_clock_100Hz <= 270000 && (*info).v_active <= 576 &&
           !(*info).interlaced && (*info).pixel_repetition != 2 && (*info).pixel_repetition != 4 { l48 = true; }
        else if (*info).requested_pixel_clock_100Hz <= 270000 && (*info).v_active <= 576 &&
                (*info).interlaced && (*info).pixel_repetition == 2 { l88 = true; }
        else if (*info).requested_pixel_clock_100Hz <= 540000 && (*info).v_active <= 576 && !(*info).interlaced { l174 = true; }
    }
    let mut blank = (*info).h_total - (*info).h_active;
    if (*info).pixel_repetition != 0 { blank *= (*info).pixel_repetition; }
    blank -= 58; blank -= 16;
    let mut samples = blank * 10 / 32 * (*info).v_active * (*info).refresh_rate / 10;
    samples *= match (*info).color_depth { COLOR_DEPTH_888 => 4, COLOR_DEPTH_101010 => 5,
        COLOR_DEPTH_121212 => 6, _ => 4 }; samples /= 4;
    if samples < 88200 { l48 = true; } else if samples < 96000 { l88 = true; }
    else if samples < 176400 { l96 = true; } else if samples < 192000 { l174 = true; }
    if !rates.is_null() {
        if l174 { (*rates).rate.RATE_192 = 0; }
        if l96 { (*rates).rate.RATE_192 = 0; (*rates).rate.RATE_176_4 = 0; }
        if l88 { (*rates).rate.RATE_192 = 0; (*rates).rate.RATE_176_4 = 0; (*rates).rate.RATE_96 = 0; }
        if l48 { (*rates).rate.RATE_192 = 0; (*rates).rate.RATE_176_4 = 0; (*rates).rate.RATE_96 = 0; (*rates).rate.RATE_88_2 = 0; }
    }
}

unsafe fn get_link_symbol_clk_freq_mhz(rate: dc_link_rate) -> fixed31_32 {
    match rate { LINK_RATE_LOW => dc_fixpt_from_int(162), LINK_RATE_HIGH => dc_fixpt_from_int(270),
        LINK_RATE_HIGH2 => dc_fixpt_from_int(540), LINK_RATE_HIGH3 => dc_fixpt_from_int(810),
        LINK_RATE_UHBR10 => dc_fixpt_from_fraction(3125, 10), LINK_RATE_UHBR13_5 => dc_fixpt_from_fraction(421875, 1000),
        LINK_RATE_UHBR20 => dc_fixpt_from_int(625), _ => { ASSERT(0); dc_fixpt_from_int(0) } }
}

#[repr(C)]
struct dp_audio_layout_config { layouts_per_sample_denom: u8, symbols_per_layout: u8, max_layouts_per_audio_sdp: u8 }

unsafe fn get_audio_layout_config(channels: u32, encoding: dp_link_encoding, out: *mut dp_audio_layout_config) {
    if encoding == DP_8b_10b_ENCODING || encoding == DP_128b_132b_ENCODING {
        let symbols = if encoding == DP_8b_10b_ENCODING { 40 } else { 10 };
        if channels == 2 { (*out).layouts_per_sample_denom = 4; (*out).symbols_per_layout = symbols; (*out).max_layouts_per_audio_sdp = 1; }
        else if channels == 8 || channels == 6 { (*out).layouts_per_sample_denom = 1; (*out).symbols_per_layout = symbols; (*out).max_layouts_per_audio_sdp = 1; }
    }
}

unsafe fn get_av_stream_map_lane_count(enc: dp_link_encoding, lanes: dc_lane_count, mst: bool) -> u32 {
    let n = if enc == DP_8b_10b_ENCODING { if mst { 4 } else { lanes } } else if enc == DP_128b_132b_ENCODING { 4 } else { 0 };
    ASSERT(n != 0); n
}

unsafe fn get_audio_sdp_overhead(enc: dp_link_encoding, lanes: dc_lane_count, mst: bool) -> u32 {
    let n = if enc == DP_8b_10b_ENCODING { if mst { 16 } else { lanes * 2 + 8 } } else if enc == DP_128b_132b_ENCODING { 10 } else { 0 };
    ASSERT(n != 0); n
}

unsafe fn calculate_required_audio_bw_in_symbols(c: *const audio_crtc_info, l: *const dp_audio_layout_config,
    _channels: u32, rate: u32, lane_count: u32, overhead: u32) -> u32 {
    let margin = dc_fixpt_from_fraction(110, 100);
    let hfreq = dc_fixpt_from_fraction((*c).requested_pixel_clock_100Hz, ((*c).h_total * 10) as i64);
    let samples = dc_fixpt_div(dc_fixpt_from_fraction(rate, 1000), hfreq);
    let layouts = dc_fixpt_div_int(samples, (*l).layouts_per_sample_denom);
    let n = dc_fixpt_floor(dc_fixpt_div_int(layouts, (*l).max_layouts_per_audio_sdp));
    let mut symbols = dc_fixpt_mul(dc_fixpt_add_int(dc_fixpt_from_int((*l).max_layouts_per_audio_sdp * (*l).symbols_per_layout), overhead), margin);
    let mut required = n * ((dc_fixpt_ceil(symbols) + lane_count) / lane_count) * lane_count;
    if n != dc_fixpt_ceil(dc_fixpt_div_int(layouts, (*l).max_layouts_per_audio_sdp)) {
        let mut rem = dc_fixpt_sub_int(layouts, n * (*l).max_layouts_per_audio_sdp);
        rem = dc_fixpt_mul_int(rem, (*l).symbols_per_layout); rem = dc_fixpt_mul(dc_fixpt_add_int(rem, overhead), margin);
        required += ((dc_fixpt_ceil(rem) + lane_count) / lane_count) * lane_count;
    }
    required
}

unsafe fn calculate_available_hblank_bw_in_symbols(c: *const audio_crtc_info, d: *const audio_dp_link_info) -> u32 {
    let blank = (*c).h_total - (*c).h_active;
    let htime = dc_fixpt_from_fraction(blank * 10, (*c).requested_pixel_clock_100Hz);
    let clk = get_link_symbol_clk_freq_mhz((*d).link_rate);
    let mut bpp = if (*c).dsc_bits_per_pixel != 0 { dc_fixpt_from_fraction((*c).dsc_bits_per_pixel, 16) } else {
        let x = match (*c).color_depth { COLOR_DEPTH_666 => 6, COLOR_DEPTH_888 => 8, COLOR_DEPTH_101010 => 10, COLOR_DEPTH_121212 => 12, _ => 8 };
        let mut v = dc_fixpt_mul_int(dc_fixpt_from_int(x), 3);
        if (*c).pixel_encoding == PIXEL_ENCODING_YCBCR422 { v = dc_fixpt_mul_int(dc_fixpt_div_int(v, 3), 2); }
        else if (*c).pixel_encoding == PIXEL_ENCODING_YCBCR420 { v = dc_fixpt_div_int(v, 2); } v
    };
    let peak = dc_fixpt_mul(dc_fixpt_from_fraction((*c).requested_pixel_clock_100Hz, 10), bpp);
    let frac = dc_fixpt_div(peak, dc_fixpt_from_int((*d).link_bandwidth_kbps));
    let mut available = dc_fixpt_floor(dc_fixpt_mul(dc_fixpt_mul(dc_fixpt_mul_int(htime, 1000), clk), frac));
    available *= (*d).lane_count; available -= (*c).dsc_num_slices * 4;
    if available < (*d).hblank_min_symbol_width { available = 4 * (*d).hblank_min_symbol_width; }
    if available < 12 { 0 } else { available - 12 }
}

unsafe fn check_audio_bandwidth_dp(c: *const audio_crtc_info, d: *const audio_dp_link_info, channels: u32, r: *mut audio_sample_rates) {
    if !(*d).is_mst && (*d).encoding == DP_8b_10b_ENCODING { return; }
    let mut l = dp_audio_layout_config { layouts_per_sample_denom: 0, symbols_per_layout: 0, max_layouts_per_audio_sdp: 0 };
    let available = calculate_available_hblank_bw_in_symbols(c, d);
    let lanes = get_av_stream_map_lane_count((*d).encoding, (*d).lane_count, (*d).is_mst);
    let overhead = get_audio_sdp_overhead((*d).encoding, (*d).lane_count, (*d).is_mst);
    get_audio_layout_config(channels, (*d).encoding, &mut l);
    if l.max_layouts_per_audio_sdp == 0 || l.symbols_per_layout == 0 || l.layouts_per_sample_denom == 0 { return; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 192000, lanes, overhead) { (*r).rate.RATE_192 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 176400, lanes, overhead) { (*r).rate.RATE_176_4 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 96000, lanes, overhead) { (*r).rate.RATE_96 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 88200, lanes, overhead) { (*r).rate.RATE_88_2 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 48000, lanes, overhead) { (*r).rate.RATE_48 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 44100, lanes, overhead) { (*r).rate.RATE_44_1 = 0; }
    if available < calculate_required_audio_bw_in_symbols(c, &l, channels, 32000, lanes, overhead) { (*r).rate.RATE_32 = 0; }
}

unsafe fn check_audio_bandwidth(c: *const audio_crtc_info, d: *const audio_dp_link_info, n: u32, s: signal_type, r: *mut audio_sample_rates) {
    match s { SIGNAL_TYPE_HDMI_TYPE_A | SIGNAL_TYPE_HDMI_FRL => check_audio_bandwidth_hdmi(c, n, r),
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST => check_audio_bandwidth_dp(c, d, n, r), _ => {} }
}

unsafe fn set_high_bit_rate_capable(audio: *mut audio, capable: bool) {
    let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR);
    set_reg_field_value(v, capable, AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR, HBR_CAPABLE);
    AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR, v);
}
unsafe fn set_video_latency(audio: *mut audio, latency: i32) { if latency < 0 || latency > 255 { return; } let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC); set_reg_field_value(v, latency, AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC, VIDEO_LIPSYNC); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC, v); }
unsafe fn set_audio_latency(audio: *mut audio, mut latency: i32) { if latency < 0 { latency = 0; } if latency > 255 { latency = 255; } let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC); set_reg_field_value(v, latency, AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC, AUDIO_LIPSYNC); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC, v); }

pub unsafe fn dce_aud_az_enable(audio: *mut audio) { let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, AUDIO_ENABLED); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); DC_LOG_HW_AUDIO("\n\t========= AUDIO:dce_aud_az_enable: index: %u  data: 0x%x\n", (*audio).inst, v); }
pub unsafe fn dce_aud_az_disable_hbr_audio(audio: *mut audio) { set_high_bit_rate_capable(audio, false); }
pub unsafe fn dce_aud_az_disable(audio: *mut audio) { let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, AUDIO_ENABLED); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); DC_LOG_HW_AUDIO("\n\t========= AUDIO:dce_aud_az_disable: index: %u  data: 0x%x\n", (*audio).inst, v); }

// The descriptor programming sequence below mirrors the C implementation.
// Register-field and structure definitions are supplied by the translated
// neighboring DC units and are therefore referenced, not redefined here.
pub unsafe fn dce_aud_az_configure(audio: *mut audio, signal: signal_type, c: *const audio_crtc_info, info: *const audio_info, d: *const audio_dp_link_info) {
    if signal == SIGNAL_TYPE_VIRTUAL { return; }
    let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v);
    v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER); set_reg_field_value(v, (*info).flags.info.ALLSPEAKERS, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, SPEAKER_ALLOCATION); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, LFE_PLAYBACK_LEVEL); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, HDMI_CONNECTION); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, DP_CONNECTION);
    let mut extra = get_reg_field_value(v, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, EXTRA_CONNECTION_INFO); extra &= !1; set_reg_field_value(v, extra, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, EXTRA_CONNECTION_INFO);
    match signal { SIGNAL_TYPE_HDMI_TYPE_A | SIGNAL_TYPE_HDMI_FRL => set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, HDMI_CONNECTION), SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST => set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, DP_CONNECTION), _ => BREAK_TO_DEBUGGER() }
    AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER, v);
    v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_ACP_DATA); set_reg_field_value(v, (*info).flags.info.SUPPORT_AI, AZALIA_F0_CODEC_PIN_CONTROL_ACP_DATA, SUPPORTS_AI); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_ACP_DATA, v);
    let mut ac3 = false; let mut fi = 0;
    for i in 0..AUDIO_FORMAT_CODE_COUNT { let code = AUDIO_FORMAT_CODE_FIRST + i; if code == AUDIO_FORMAT_CODE_1BITAUDIO || code == AUDIO_FORMAT_CODE_DST { continue; } v = 0; if is_audio_format_supported(info, code, &mut fi) { let mode = &(*info).modes[fi as usize]; let mut rates = mode.sample_rates; let mut chans = mode.channel_count; let mut byte2 = mode.max_bit_rate; match code { AUDIO_FORMAT_CODE_LINEARPCM => { if signal == SIGNAL_TYPE_HDMI_FRL && chans > 2 && !c.is_null() && (*c).v_active <= 576 { chans = 2; } check_audio_bandwidth(c, d, chans, signal, &mut rates); byte2 = mode.sample_size; set_reg_field_value(v, rates.all, AZALIA_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0, SUPPORTED_FREQUENCIES_STEREO); }, AUDIO_FORMAT_CODE_AC3 => ac3 = true, AUDIO_FORMAT_CODE_DOLBYDIGITALPLUS | AUDIO_FORMAT_CODE_DTS_HD | AUDIO_FORMAT_CODE_MAT_MLP | AUDIO_FORMAT_CODE_WMAPRO => byte2 = mode.vendor_specific, _ => {} } set_reg_field_value(v, chans - 1, AZALIA_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0, MAX_CHANNELS); set_reg_field_value(v, rates.all, AZALIA_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0, SUPPORTED_FREQUENCIES); set_reg_field_value(v, byte2, AZALIA_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0, DESCRIPTOR_BYTE_2); } AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0 + i, v); }
    if ac3 { REG_WRITE(AZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS, 0x05); }
    let mut rates = audio_sample_rates { all: 0 }; rates.rate.RATE_192 = 1; check_audio_bandwidth(c, d, 8, signal, &mut rates); set_high_bit_rate_capable(audio, rates.rate.RATE_192); set_video_latency(audio, (*info).video_latency); set_audio_latency(audio, (*info).audio_latency);
    v = 0; set_reg_field_value(v, (*info).manufacture_id, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO0, MANUFACTURER_ID); set_reg_field_value(v, (*info).product_id, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO0, PRODUCT_ID); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO0, v);
    let mut len = 0; while (*info).display_name[len as usize] != 0 { len += 1; if len >= MAX_HW_AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS { break; } } v = 0; set_reg_field_value(v, len, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO1, SINK_DESCRIPTION_LEN); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO1, v);
    v = 0; set_reg_field_value(v, (*info).port_id[0], AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO2, PORT_ID0); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO2, v); v = 0; set_reg_field_value(v, (*info).port_id[1], AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO3, PORT_ID1); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO3, v);
    let regs = [AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO4, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO5, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO6, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO7, AZALIA_F0_CODEC_PIN_CONTROL_SINK_INFO8]; for j in 0..5 { v = 0; for k in 0..4 { let ix = j * 4 + k; if ix < 18 { set_reg_field_value(v, (*info).display_name[ix], regs[j], DESCRIPTION0 + k); } } AZ_REG_WRITE(regs[j], v); }
    v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v);
}

unsafe fn get_azalia_clock_info_hdmi(_requested: u32, actual: u32, out: *mut azalia_clock_info) { (*out).audio_dto_phase = 24 * 10000; (*out).audio_dto_module = actual; }
unsafe fn get_azalia_clock_info_dp(_requested: u32, pll: *const audio_pll_info, out: *mut azalia_clock_info) { (*out).audio_dto_phase = 24 * 10000; (*out).audio_dto_module = (*pll).audio_dto_source_clock_in_khz * 10; }

pub unsafe fn dce_aud_wall_dto_setup(audio: *mut audio, signal: signal_type, c: *const audio_crtc_info, pll: *const audio_pll_info) {
    let aud = DCE_AUD(audio); let mut ci = azalia_clock_info { audio_dto_phase: 0, audio_dto_module: 0 };
    if dc_is_hdmi_tmds_signal(signal) { get_azalia_clock_info_hdmi((*c).requested_pixel_clock_100Hz, (*c).calculated_pixel_clock_100Hz, &mut ci); let src = (*pll).dto_source - DTO_SOURCE_ID0; REG_UPDATE_2(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO0_SOURCE_SEL, src, DCCG_AUDIO_DTO_SEL, 0); REG_UPDATE(DCCG_AUDIO_DTO0_MODULE, DCCG_AUDIO_DTO0_MODULE, ci.audio_dto_module); REG_UPDATE(DCCG_AUDIO_DTO0_PHASE, DCCG_AUDIO_DTO0_PHASE, ci.audio_dto_phase); } else { get_azalia_clock_info_dp((*c).requested_pixel_clock_100Hz, pll, &mut ci); REG_UPDATE(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO_SEL, 1); REG_UPDATE(DCCG_AUDIO_DTO1_MODULE, DCCG_AUDIO_DTO1_MODULE, ci.audio_dto_module); REG_UPDATE(DCCG_AUDIO_DTO1_PHASE, DCCG_AUDIO_DTO1_PHASE, ci.audio_dto_phase); if (*aud).masks.DCCG_AUDIO_DTO2_USE_512FBR_DTO != 0 { REG_UPDATE(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO2_USE_512FBR_DTO, 1); } }
}

unsafe fn dce_aud_endpoint_valid(audio: *mut audio) -> bool { let v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_CONFIGURATION_DEFAULT); get_reg_field_value(v, AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_CONFIGURATION_DEFAULT, PORT_CONNECTIVITY) != 1 }
pub unsafe fn dce_aud_hw_init(audio: *mut audio) { if (*audio).inst != 0 { return; } let mut v = AZ_REG_READ(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL); set_reg_field_value(v, 1, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); REG_UPDATE(AZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES, AUDIO_RATE_CAPABILITIES, 0x70); REG_UPDATE_2(AZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES, CLKSTOP, 1, EPSS, 1); set_reg_field_value(v, 0, AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, CLOCK_GATING_DISABLE); AZ_REG_WRITE(AZALIA_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL, v); }

pub unsafe fn dce_aud_destroy(audio: *mut *mut audio) { let aud = DCE_AUD(*audio); kfree(aud); *audio = core::ptr::null_mut(); }

pub unsafe fn dce_audio_create(ctx: *mut dc_context, inst: u32, reg: *const dce_audio_registers, shifts: *const dce_audio_shift, masks: *const dce_audio_mask) -> *mut audio {
    let a = kzalloc_obj::<dce_audio>(); if a.is_null() { ASSERT_CRITICAL(a); return core::ptr::null_mut(); }
    (*a).base.ctx = ctx; (*a).base.inst = inst; (*a).base.funcs = &funcs; (*a).regs = reg; (*a).shifts = shifts; (*a).masks = masks; &mut (*a).base
}

// Function table definition is supplied by the corresponding audio interface.
extern "C" { static funcs: audio_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
