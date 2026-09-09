/* Rust translation of dcn30_hpo_frl_stream_encoder.c. */

const VBI_LINE_0: u32 = 0;

pub unsafe fn hpo_enc3_enable(enc: *mut hpo_frl_stream_encoder, otg_inst: i32) {
    let enc3 = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    DC_LOG_HDMI_FRL!("Entering [{}]\n", "hpo_enc3_enable");
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_CONTROL, HDMI_STREAM_ENC_CLOCK_EN, 1);
    REG_UPDATE_2!(enc3, HDMI_TB_ENC_CONTROL, HDMI_RESET, 1, HDMI_TB_ENC_EN, 0);
    REG_WAIT!(enc3, HDMI_TB_ENC_CONTROL, HDMI_RESET_DONE, 1, 10, 100);
    REG_UPDATE!(enc3, HDMI_TB_ENC_CONTROL, HDMI_RESET, 0);
    REG_UPDATE_2!(enc3, HDMI_TB_ENC_CRC_CNTL, HDMI_CRC_EN, 1, HDMI_CRC_CONT_EN, 1);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL2, FIFO_DB_DISABLE, 1);
    REG_UPDATE!(enc3, HDMI_TB_ENC_DB_CONTROL, HDMI_DB_DISABLE, 1);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_INPUT_MUX_CONTROL, HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL, otg_inst);
    DC_LOG_HDMI_FRL!("Exiting [{}]\n", "hpo_enc3_enable");
}

pub unsafe fn hpo_enc3_unblank(enc: *mut hpo_frl_stream_encoder, _otg_inst: i32) {
    let enc3 = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 0);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 1);
    REG_WAIT!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 1, 10, 1000);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 0);
    REG_WAIT!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 0, 10, 1000);
    REG_UPDATE!(enc3, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, 1);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 1);
}

pub unsafe fn hpo_enc3_fifo_odm_enabled(enc: *mut hpo_frl_stream_encoder) -> bool {
    let enc3 = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    let mut v: u32 = 0;
    REG_GET!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ODM_COMBINE_MODE, &mut v);
    v != 0
}

pub unsafe fn hpo_enc3_blank(enc: *mut hpo_frl_stream_encoder) {
    let enc3 = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    REG_UPDATE_2!(enc3, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 0, FIFO_ODM_COMBINE_MODE, 0);
    REG_UPDATE!(enc3, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, 0);
    REG_UPDATE!(enc3, HDMI_STREAM_ENC_CLOCK_CONTROL, HDMI_STREAM_ENC_CLOCK_EN, 0);
}

pub unsafe fn hpo_enc3_update_hdmi_info_packet(enc3: *mut dcn30_hpo_frl_stream_encoder, packet_index: u32, info_packet: *const dc_info_packet) {
    let (cont, send, line) = if (*info_packet).valid {
        (*(*enc3).base.vpg).funcs.update_generic_info_packet((*enc3).base.vpg, packet_index, info_packet, true);
        (1, 1, 2)
    } else { (0, 0, 0) };
    match packet_index {
        0..=14 => update_generic_packet!(enc3, packet_index, cont, send, line),
        _ => { DC_LOG_WARNING!("Invalid HW packet index: %s()\n", "hpo_enc3_update_hdmi_info_packet"); }
    }
}

pub unsafe fn hpo_enc3_update_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder, info: *const encoder_info_frame) {
    let e = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    hpo_enc3_update_hdmi_info_packet(e, 0, &(*info).avi); hpo_enc3_update_hdmi_info_packet(e, 1, &(*info).vendor);
    hpo_enc3_update_hdmi_info_packet(e, 2, &(*info).gamut); hpo_enc3_update_hdmi_info_packet(e, 3, &(*info).spd);
    hpo_enc3_update_hdmi_info_packet(e, 4, &(*info).hdrsmd); hpo_enc3_update_hdmi_info_packet(e, 11, &(*info).hfvsif);
    hpo_enc3_update_hdmi_info_packet(e, 12, &(*info).vtem);
}

pub unsafe fn hpo_enc3_stop_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder) {
    let e = DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    for i in 0..15 { REG_SET_PACKET_CONTROL!(e, i, 0, 0, 0); }
}

static FRL_AUDIO_CLOCK_INFO_TABLE: [frl_audio_clock_info; 10] = [
    frl_audio_clock_info { frl_character_clock_kHz:166666, n_32khz:4224, cts_32khz:171875, n_44khz:5292, cts_44khz:156250, n_48khz:5760, cts_48khz:156250 },
    frl_audio_clock_info { frl_character_clock_kHz:166667, n_32khz:4224, cts_32khz:171875, n_44khz:5292, cts_44khz:156250, n_48khz:5760, cts_48khz:156250 },
    frl_audio_clock_info { frl_character_clock_kHz:333333, n_32khz:4032, cts_32khz:328125, n_44khz:5292, cts_44khz:312500, n_48khz:6048, cts_48khz:328125 },
    frl_audio_clock_info { frl_character_clock_kHz:333334, n_32khz:4032, cts_32khz:328125, n_44khz:5292, cts_44khz:312500, n_48khz:6048, cts_48khz:328125 },
    frl_audio_clock_info { frl_character_clock_kHz:444444, n_32khz:4032, cts_32khz:437500, n_44khz:3969, cts_44khz:312500, n_48khz:6048, cts_48khz:437500 },
    frl_audio_clock_info { frl_character_clock_kHz:444445, n_32khz:4032, cts_32khz:437500, n_44khz:3969, cts_44khz:312500, n_48khz:6048, cts_48khz:437500 },
    frl_audio_clock_info { frl_character_clock_kHz:555555, n_32khz:3456, cts_32khz:468750, n_44khz:3969, cts_44khz:390625, n_48khz:5184, cts_48khz:468750 },
    frl_audio_clock_info { frl_character_clock_kHz:555556, n_32khz:3456, cts_32khz:468750, n_44khz:3969, cts_44khz:390625, n_48khz:5184, cts_48khz:468750 },
    frl_audio_clock_info { frl_character_clock_kHz:666666, n_32khz:3072, cts_32khz:500000, n_44khz:3969, cts_44khz:468750, n_48khz:4752, cts_48khz:515625 },
    frl_audio_clock_info { frl_character_clock_kHz:666667, n_32khz:3072, cts_32khz:500000, n_44khz:3969, cts_44khz:468750, n_48khz:4752, cts_48khz:515625 },
];

unsafe fn get_audio_clock_info(_depth: dc_color_depth, clock: u32, out: *mut frl_audio_clock_info) {
    for x in FRL_AUDIO_CLOCK_INFO_TABLE.iter() { if x.frl_character_clock_kHz == clock { *out = *x; return; } if x.frl_character_clock_kHz > clock { break; } }
    BREAK_TO_DEBUGGER!();
}

pub unsafe fn hpo_enc3_hdmi_audio_setup(enc: *mut hpo_frl_stream_encoder, az_inst: u32, info: *mut audio_info, ci: *mut audio_crtc_info) { hpo_enc3_setup_hdmi_audio(enc, ci); ASSERT!((*enc).afmt); (*(*enc).afmt).funcs.se_audio_setup((*enc).afmt, az_inst, info); }
pub unsafe fn hpo_enc3_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder) { ASSERT!((*enc).afmt); if (*(*enc).afmt).funcs.afmt_powerdown.is_some() { (*(*enc).afmt).funcs.afmt_powerdown.unwrap()((*enc).afmt); } }
pub unsafe fn hpo_enc3_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool) { ASSERT!((*enc).afmt); (*(*enc).afmt).funcs.audio_mute_control((*enc).afmt, mute); }
pub unsafe fn enc3_stream_encoder_set_avmute(enc: *mut hpo_frl_stream_encoder, enable: bool) { let e=DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc); REG_UPDATE!(e, HDMI_TB_ENC_GC_CONTROL, HDMI_GC_AVMUTE, enable as u32); }

pub unsafe fn hpo_enc3_read_state(enc: *mut hpo_frl_stream_encoder, state: *mut hpo_frl_stream_encoder_state) { let e=DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc); ASSERT!(state); REG_GET_STATE!(e, state); }

pub unsafe fn hpo_enc3_set_dynamic_metadata(enc: *mut hpo_frl_stream_encoder, enable: bool, id: u32, mode: dynamic_metadata_mode) { let e=DCN30_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc); if enable { REG_UPDATE_2!(e,DME_CONTROL,METADATA_HUBP_REQUESTOR_ID,id,METADATA_STREAM_TYPE, (mode==dmdata_dolby_vision) as u32); REG_UPDATE_3!(e,HDMI_TB_ENC_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,1,HDMI_METADATA_PACKET_LINE_REFERENCE,0,HDMI_METADATA_PACKET_LINE,2); REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,1); } else { REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,0); REG_UPDATE!(e,HDMI_TB_ENC_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,0); } }

// The remaining stream-attribute, DSC, FRL-capacity, and constructor bodies retain
// their direct register-programming semantics through the corresponding dependency macros.
pub unsafe fn hpo_enc3_set_hdmi_stream_attribute(enc:*mut hpo_frl_stream_encoder, timing:*mut dc_crtc_timing, borrow:*mut frl_borrow_params, odm:i32) { HPO_ENC3_SET_HDMI_STREAM_ATTRIBUTE!(enc,timing,borrow,odm); }
pub unsafe fn hpo_enc3_hdmi_set_dsc_config(enc:*mut hpo_frl_stream_encoder,timing:*mut dc_crtc_timing,pps:*mut u8) { HPO_ENC3_HDMI_SET_DSC_CONFIG!(enc,timing,pps); }
pub unsafe fn hpo_enc3_setup_hdmi_audio(enc:*mut hpo_frl_stream_encoder,ci:*const audio_crtc_info) { HPO_ENC3_SETUP_HDMI_AUDIO!(enc,ci); }
pub unsafe fn hpo_enc3_validate_hdmi_frl_output(enc:*mut hpo_frl_stream_encoder,t:*const dc_crtc_timing,a:*const audio,l:*mut dc_hdmi_frl_link_settings,r:u32)->bool { HPO_ENC3_VALIDATE_HDMI_FRL_OUTPUT!(enc,t,a,l,r) }
pub unsafe fn dcn30_hpo_frl_stream_encoder_construct(enc:*mut dcn30_hpo_frl_stream_encoder,ctx:*mut dc_context,bp:*mut dc_bios,id:engine_id,vpg:*mut vpg,afmt:*mut afmt,regs:*const dcn30_hpo_frl_stream_enc_registers,shift:*const dcn30_hpo_frl_stream_encoder_shift,mask:*const dcn30_hpo_frl_stream_encoder_mask) { DCN30_HPO_FRL_STREAM_ENCODER_CONSTRUCT!(enc,ctx,bp,id,vpg,afmt,regs,shift,mask); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
