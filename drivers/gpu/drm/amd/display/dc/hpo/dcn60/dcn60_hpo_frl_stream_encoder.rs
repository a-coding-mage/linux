// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from dcn60_hpo_frl_stream_encoder.c.
// External types, constants, helpers, and register macros are supplied by dependencies.

const DEBUG_FRL_CAP_CHK: i32 = 1;
const VBI_LINE_0: u32 = 0;

unsafe fn hpo_enc60_set_hdmi_stream_attribute(
    enc: *mut hpo_frl_stream_encoder,
    crtc_timing: *mut dc_crtc_timing,
    borrow_params: *mut frl_borrow_params,
    odm_combine_num_segments: i32,
) {
    let _ = odm_combine_num_segments;
    let mut h_active: u32;
    let mut h_blank: u32;
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);

    DC_LOG_DEBUG!("Entering [{}]\n", "hpo_enc60_set_hdmi_stream_attribute");

    match (*crtc_timing).pixel_encoding {
        PIXEL_ENCODING_YCBCR422 => {
            REG_UPDATE!(enc401, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 1);
            REG_UPDATE_2!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
                FIFO_PIXEL_ENCODING_TYPE, 0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 0);
        }
        PIXEL_ENCODING_YCBCR420 => {
            REG_UPDATE!(enc401, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 2);
            REG_UPDATE_2!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
                FIFO_PIXEL_ENCODING_TYPE, 0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 1);
        }
        _ => {
            REG_UPDATE!(enc401, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 0);
            REG_UPDATE_2!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
                FIFO_PIXEL_ENCODING_TYPE, 0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 0);
        }
    }

    match (*crtc_timing).display_color_depth {
        COLOR_DEPTH_888 => REG_UPDATE_2!(enc401, HDMI_TB_ENC_PIXEL_FORMAT,
            HDMI_DEEP_COLOR_DEPTH, 0, HDMI_DEEP_COLOR_ENABLE, 0),
        COLOR_DEPTH_101010 => REG_UPDATE_2!(enc401, HDMI_TB_ENC_PIXEL_FORMAT,
            HDMI_DEEP_COLOR_DEPTH, 1,
            HDMI_DEEP_COLOR_ENABLE, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 0 } else { 1 }),
        COLOR_DEPTH_121212 => REG_UPDATE_2!(enc401, HDMI_TB_ENC_PIXEL_FORMAT,
            HDMI_DEEP_COLOR_DEPTH, 2,
            HDMI_DEEP_COLOR_ENABLE, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 0 } else { 1 }),
        _ => {}
    }

    if (*crtc_timing).flags.DSC {
        REG_UPDATE_2!(enc401, HDMI_TB_ENC_PIXEL_FORMAT,
            HDMI_DEEP_COLOR_DEPTH, 0, HDMI_DEEP_COLOR_ENABLE, 0);
    }

    h_active = (*crtc_timing).h_addressable + (*crtc_timing).h_border_left + (*crtc_timing).h_border_right;
    h_blank = (*crtc_timing).h_total - h_active;
    if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 ||
        (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 {
        h_active /= 2;
        h_blank /= 2;
    }
    REG_SET_2!(enc401, HDMI_TB_ENC_H_ACTIVE_BLANK, 0,
        HDMI_H_ACTIVE, h_active, HDMI_H_BLANK, h_blank);

    REG_UPDATE!(enc401, HDMI_TB_ENC_MODE, HDMI_BORROW_MODE, (*borrow_params).borrow_mode);
    REG_UPDATE!(enc401, HDMI_TB_ENC_PACKET_CONTROL, HDMI_MAX_PACKETS_PER_LINE, (*borrow_params).audio_packets_line);
    REG_SET_2!(enc401, HDMI_TB_ENC_HC_ACTIVE_BLANK, 0,
        HDMI_HC_ACTIVE, (*borrow_params).hc_active_target, HDMI_HC_BLANK, (*borrow_params).hc_blank_target);
    REG_UPDATE_2!(enc401, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_GC_CONT, 1, HDMI_GC_SEND, 1);
    REG_UPDATE!(enc401, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_ACP_SEND, 0);
    REG_UPDATE!(enc401, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_AUDIO_INFO_SEND, 1);
    REG_UPDATE!(enc401, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_AUDIO_INFO_LINE, VBI_LINE_0 + 2);
    REG_UPDATE!(enc401, HDMI_TB_ENC_GC_CONTROL, HDMI_GC_AVMUTE, 0);
    DC_LOG_DEBUG!("Exiting [{}]\n", "hpo_enc60_set_hdmi_stream_attribute");
}

unsafe fn hpo_enc60_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool) {
    ASSERT!((*enc).apg);
    if mute { (*(*enc).apg).funcs.disable_apg((*enc).apg); }
    else { (*(*enc).apg).funcs.enable_apg((*enc).apg); }
}

static FRL_AUDIO_CLOCK_INFO_TABLE: [frl_audio_clock_info; 16] = [
    frl_audio_clock_info { frl_character_clock_kHz: 166666, n_32khz: 4224, cts_32khz: 171875, n_44khz: 5292, cts_44khz: 156250, n_48khz: 5760, cts_48khz: 156250 },
    frl_audio_clock_info { frl_character_clock_kHz: 166667, n_32khz: 4224, cts_32khz: 171875, n_44khz: 5292, cts_44khz: 156250, n_48khz: 5760, cts_48khz: 156250 },
    frl_audio_clock_info { frl_character_clock_kHz: 333333, n_32khz: 4032, cts_32khz: 328125, n_44khz: 5292, cts_44khz: 312500, n_48khz: 6048, cts_48khz: 328125 },
    frl_audio_clock_info { frl_character_clock_kHz: 333334, n_32khz: 4032, cts_32khz: 328125, n_44khz: 5292, cts_44khz: 312500, n_48khz: 6048, cts_48khz: 328125 },
    frl_audio_clock_info { frl_character_clock_kHz: 444444, n_32khz: 4032, cts_32khz: 437500, n_44khz: 3969, cts_44khz: 312500, n_48khz: 6048, cts_48khz: 437500 },
    frl_audio_clock_info { frl_character_clock_kHz: 444445, n_32khz: 4032, cts_32khz: 437500, n_44khz: 3969, cts_44khz: 312500, n_48khz: 6048, cts_48khz: 437500 },
    frl_audio_clock_info { frl_character_clock_kHz: 555555, n_32khz: 3456, cts_32khz: 468750, n_44khz: 3969, cts_44khz: 390625, n_48khz: 5184, cts_48khz: 468750 },
    frl_audio_clock_info { frl_character_clock_kHz: 555556, n_32khz: 3456, cts_32khz: 468750, n_44khz: 3969, cts_44khz: 390625, n_48khz: 5184, cts_48khz: 468750 },
    frl_audio_clock_info { frl_character_clock_kHz: 666666, n_32khz: 3072, cts_32khz: 500000, n_44khz: 3969, cts_44khz: 468750, n_48khz: 4752, cts_48khz: 515625 },
    frl_audio_clock_info { frl_character_clock_kHz: 666667, n_32khz: 3072, cts_32khz: 500000, n_44khz: 3969, cts_44khz: 468750, n_48khz: 4752, cts_48khz: 515625 },
    frl_audio_clock_info { frl_character_clock_kHz: 888888, n_32khz: 4032, cts_32khz: 875000, n_44khz: 3969, cts_44khz: 625000, n_48khz: 6048, cts_48khz: 875000 },
    frl_audio_clock_info { frl_character_clock_kHz: 888889, n_32khz: 4032, cts_32khz: 875000, n_44khz: 3969, cts_44khz: 625000, n_48khz: 6048, cts_48khz: 875000 },
    frl_audio_clock_info { frl_character_clock_kHz: 1111110, n_32khz: 3456, cts_32khz: 937500, n_44khz: 3969, cts_44khz: 781250, n_48khz: 5184, cts_48khz: 937500 },
    frl_audio_clock_info { frl_character_clock_kHz: 1111111, n_32khz: 3456, cts_32khz: 937500, n_44khz: 3969, cts_44khz: 781250, n_48khz: 5184, cts_48khz: 937500 },
    frl_audio_clock_info { frl_character_clock_kHz: 1333332, n_32khz: 3072, cts_32khz: 1000000, n_44khz: 3969, cts_44khz: 937500, n_48khz: 4752, cts_48khz: 1031250 },
    frl_audio_clock_info { frl_character_clock_kHz: 1333333, n_32khz: 3072, cts_32khz: 1000000, n_44khz: 3969, cts_44khz: 937500, n_48khz: 4752, cts_48khz: 1031250 },
];

unsafe fn get_audio_clock_info(color_depth: dc_color_depth, frl_character_clock_kHz: u32, audio_clock_info: *mut frl_audio_clock_info) {
    let _ = color_depth;
    for clock_info in FRL_AUDIO_CLOCK_INFO_TABLE.iter() {
        if clock_info.frl_character_clock_kHz > frl_character_clock_kHz { break; }
        if clock_info.frl_character_clock_kHz == frl_character_clock_kHz {
            *audio_clock_info = *clock_info;
            return;
        }
    }
    BREAK_TO_DEBUGGER!();
}

unsafe fn hpo_enc60_setup_hdmi_audio(enc: *mut hpo_frl_stream_encoder, crtc_info: *const audio_crtc_info) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    let mut audio_clock_info: frl_audio_clock_info = core::mem::zeroed();
    let _ = enc401;
    REG_UPDATE_3!(enc401, HDMI_TB_ENC_ACR_PACKET_CONTROL, HDMI_ACR_AUTO_SEND, 1, HDMI_ACR_SOURCE, 0, HDMI_ACR_AUDIO_PRIORITY, 0);
    get_audio_clock_info((*crtc_info).color_depth, (*crtc_info).frl_character_clock_kHz, &mut audio_clock_info);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_32_0, HDMI_ACR_CTS_32, audio_clock_info.cts_32khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_32_1, HDMI_ACR_N_32, audio_clock_info.n_32khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_44_0, HDMI_ACR_CTS_44, audio_clock_info.cts_44khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_44_1, HDMI_ACR_N_44, audio_clock_info.n_44khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_48_0, HDMI_ACR_CTS_48, audio_clock_info.cts_48khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_48_1, HDMI_ACR_N_48, audio_clock_info.n_48khz);
}

unsafe fn hpo_enc60_hdmi_audio_setup(enc: *mut hpo_frl_stream_encoder, az_inst: u32, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    REG_UPDATE_2!(enc401, HDMI_STREAM_ENC_AUDIO_CONTROL, HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, az_inst, HDMI_STREAM_ENC_APG_CLOCK_EN, 1);
    hpo_enc60_setup_hdmi_audio(enc, audio_crtc_info);
    ASSERT!((*enc).apg);
    (*(*enc).apg).funcs.se_audio_setup((*enc).apg, az_inst, info);
}

unsafe fn hpo_enc60_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    ASSERT!((*enc).apg);
    if !(*enc).apg.is_null() { (*(*enc).apg).funcs.disable_apg((*enc).apg); }
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_AUDIO_CONTROL, HDMI_STREAM_ENC_APG_CLOCK_EN, 0);
}

static DCN401_STR_ENC_FUNCS: hpo_frl_stream_encoder_funcs = hpo_frl_stream_encoder_funcs {
    hdmi_frl_enable: hpo_enc401_enable,
    hdmi_frl_unblank: hpo_enc401_unblank,
    hdmi_frl_blank: hpo_enc401_blank,
    hdmi_frl_set_stream_attribute: hpo_enc60_set_hdmi_stream_attribute,
    validate_hdmi_frl_output: hpo_enc3_validate_hdmi_frl_output,
    update_hdmi_info_packets: hpo_enc401_update_hdmi_info_packets,
    stop_hdmi_info_packets: hpo_enc401_stop_hdmi_info_packets,
    audio_mute_control: hpo_enc60_audio_mute_control,
    hdmi_audio_setup: hpo_enc60_hdmi_audio_setup,
    hdmi_audio_disable: hpo_enc60_hdmi_audio_disable,
    set_avmute: enc401_stream_encoder_set_avmute,
    read_state: hpo_enc401_read_state,
    hdmi_frl_set_dsc_config: hpo_enc401_hdmi_set_dsc_config,
    set_dynamic_metadata: hpo_enc401_set_dynamic_metadata,
};

pub unsafe fn dcn60_hpo_frl_stream_encoder_construct(
    enc401: *mut dcn401_hpo_frl_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios,
    eng_id: engine_id, vpg: *mut vpg, apg: *mut apg,
    regs: *const dcn30_hpo_frl_stream_enc_registers,
    hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift,
    hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask,
) {
    (*enc401).base.funcs = &DCN401_STR_ENC_FUNCS;
    (*enc401).base.ctx = ctx;
    (*enc401).base.id = eng_id;
    (*enc401).base.bp = bp;
    (*enc401).base.vpg = vpg;
    (*enc401).base.apg = apg;
    (*enc401).regs = regs;
    (*enc401).hpo_se_shift = hpo_se_shift;
    (*enc401).hpo_se_mask = hpo_se_mask;
    (*enc401).base.stream_enc_inst = (*vpg).inst;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
