// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translation unit.

const DEBUG_FRL_CAP_CHK: i32 = 1;
const VBI_LINE_0: i32 = 0;

pub unsafe fn hpo_enc42_unblank(enc: *mut hpo_frl_stream_encoder, otg_inst: i32) {
    let _ = otg_inst;
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);

    DC_LOG_HDMI_FRL!("Entering [{}]\n", "hpo_enc42_unblank");

    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_ENABLE, 0);

    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_RESET, 1);
    REG_WAIT!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_RESET_DONE, 1, 10, 1000);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_RESET, 0);
    REG_WAIT!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_RESET_DONE, 0, 10, 1000);

    REG_UPDATE!(enc401, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, 1);

    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0,
        FIFO_ENABLE, 1);

    DC_LOG_HDMI_FRL!("Exiting [{}]\n", "hpo_enc42_unblank");
}

pub unsafe fn hpo_enc42_setup_hdmi_audio(
    enc: *mut hpo_frl_stream_encoder,
    crtc_info: *const audio_crtc_info,
) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    let mut audio_clock_info: frl_audio_clock_info = core::mem::zeroed();

    DC_LOG_DEBUG!("Entering [{}]\n", "hpo_enc42_setup_hdmi_audio");
    // TODO: HDMI_AUDIO_DELAY_EN bit only in DIG -- not in HPO?
    ASSERT!((*enc).apg);

    REG_UPDATE_3!(enc401, HDMI_TB_ENC_ACR_PACKET_CONTROL,
        HDMI_ACR_AUTO_SEND, 1, HDMI_ACR_SOURCE, 0, HDMI_ACR_AUDIO_PRIORITY, 0);

    frl_get_audio_clock_info((*crtc_info).color_depth,
        (*crtc_info).frl_character_clock_kHz, &mut audio_clock_info);
    DC_LOG_HW_AUDIO!("\n%s:Input::requested_pixel_clock_100Hz = %dcalculated_pixel_clock_100Hz = %d \n",
        "hpo_enc42_setup_hdmi_audio", (*crtc_info).requested_pixel_clock_100Hz,
        (*crtc_info).calculated_pixel_clock_100Hz);

    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_32_0, HDMI_ACR_CTS_32, audio_clock_info.cts_32khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_32_1, HDMI_ACR_N_32, audio_clock_info.n_32khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_44_0, HDMI_ACR_CTS_44, audio_clock_info.cts_44khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_44_1, HDMI_ACR_N_44, audio_clock_info.n_44khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_48_0, HDMI_ACR_CTS_48, audio_clock_info.cts_48khz);
    REG_UPDATE!(enc401, HDMI_TB_ENC_ACR_48_1, HDMI_ACR_N_48, audio_clock_info.n_48khz);

    // HDMI_TB_ENC_ACR_PACKET_CONTROL::ACR_N_MULTIPLE is programmed in the interrupt callback.
    DC_LOG_DEBUG!("Exiting [{}]\n", "hpo_enc42_setup_hdmi_audio");
}

pub unsafe fn hpo_enc42_hdmi_audio_setup(
    enc: *mut hpo_frl_stream_encoder, az_inst: u32, info: *mut audio_info,
    audio_crtc_info: *mut audio_crtc_info,
) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    REG_UPDATE_2!(enc401, HDMI_STREAM_ENC_AUDIO_CONTROL,
        HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, az_inst,
        HDMI_STREAM_ENC_APG_CLOCK_EN, 1);
    hpo_enc42_setup_hdmi_audio(enc, audio_crtc_info);
    ASSERT!((*enc).apg);
    ((*(*enc).apg).funcs).se_audio_setup((*enc).apg, az_inst, info);
}

pub unsafe fn hpo_enc42_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder) {
    ASSERT!((*enc).apg);
    if ((*(*enc).apg).funcs).disable_apg.is_some() {
        ((*(*enc).apg).funcs).disable_apg.unwrap()((*enc).apg);
    }
}

pub unsafe fn hpo_enc42_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool) {
    ASSERT!((*enc).apg);
    if mute { ((*(*enc).apg).funcs).disable_apg.unwrap()((*enc).apg); }
    else { ((*(*enc).apg).funcs).enable_apg((*enc).apg); }
}

static dcn42_str_enc_funcs: hpo_frl_stream_encoder_funcs = hpo_frl_stream_encoder_funcs {
    hdmi_frl_enable: hpo_enc401_enable,
    hdmi_frl_unblank: hpo_enc42_unblank,
    hdmi_frl_blank: hpo_enc401_blank,
    hdmi_frl_set_stream_attribute: hpo_enc401_set_hdmi_stream_attribute,
    validate_hdmi_frl_output: hpo_enc3_validate_hdmi_frl_output,
    update_hdmi_info_packets: hpo_enc401_update_hdmi_info_packets,
    stop_hdmi_info_packets: hpo_enc401_stop_hdmi_info_packets,
    audio_mute_control: hpo_enc42_audio_mute_control,
    hdmi_audio_setup: hpo_enc42_hdmi_audio_setup,
    hdmi_audio_disable: hpo_enc42_hdmi_audio_disable,
    set_avmute: enc401_stream_encoder_set_avmute,
    read_state: hpo_enc401_read_state,
    hdmi_frl_set_dsc_config: hpo_enc401_hdmi_set_dsc_config,
    set_dynamic_metadata: hpo_enc401_set_dynamic_metadata,
};

pub unsafe fn dcn42_hpo_frl_stream_encoder_construct(
    enc42: *mut dcn42_hpo_frl_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios,
    eng_id: engine_id, vpg: *mut vpg, apg: *mut apg,
    regs: *const dcn30_hpo_frl_stream_enc_registers,
    hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift,
    hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask,
) {
    (*enc42).base.funcs = &dcn42_str_enc_funcs;
    (*enc42).base.ctx = ctx;
    (*enc42).base.id = eng_id;
    (*enc42).base.bp = bp;
    (*enc42).base.vpg = vpg;
    (*enc42).base.apg = apg;
    (*enc42).regs = regs;
    (*enc42).hpo_se_shift = hpo_se_shift;
    (*enc42).hpo_se_mask = hpo_se_mask;
    (*enc42).base.stream_enc_inst = (*vpg).inst;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
