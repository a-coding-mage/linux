/* Rust translation of dcn401_hpo_frl_stream_encoder.c. */

const VBI_LINE_0: u32 = 0;

pub unsafe fn hpo_enc401_enable(enc: *mut hpo_frl_stream_encoder, otg_inst: i32) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    DC_LOG_DEBUG!("Entering [{}]\n", "hpo_enc401_enable");
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_CONTROL, HDMI_STREAM_ENC_CLOCK_EN, 1);
    REG_UPDATE_2!(enc401, HDMI_TB_ENC_CONTROL, HDMI_RESET, 1, HDMI_TB_ENC_EN, 0);
    REG_WAIT!(enc401, HDMI_TB_ENC_CONTROL, HDMI_RESET_DONE, 1, 10, 100);
    REG_UPDATE!(enc401, HDMI_TB_ENC_CONTROL, HDMI_RESET, 0);
    REG_UPDATE_2!(enc401, HDMI_TB_ENC_CRC_CNTL, HDMI_CRC_EN, 1, HDMI_CRC_CONT_EN, 1);
    REG_UPDATE!(enc401, HDMI_TB_ENC_DB_CONTROL, HDMI_DB_DISABLE, 1);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_INPUT_MUX_CONTROL, HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL, otg_inst);
    DC_LOG_DEBUG!("Exiting [{}]\n", "hpo_enc401_enable");
}

pub unsafe fn hpo_enc401_unblank(enc: *mut hpo_frl_stream_encoder, _otg_inst: i32) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    DC_LOG_HDMI_FRL!("Entering [{}]\n", "hpo_enc401_unblank");
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 0);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 1);
    REG_WAIT!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 1, 10, 1000);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 0);
    REG_WAIT!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 0, 10, 1000);
    REG_UPDATE!(enc401, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, 1);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 1);
    DC_LOG_HDMI_FRL!("Exiting [{}]\n", "hpo_enc401_unblank");
}

pub unsafe fn hpo_enc401_blank(enc: *mut hpo_frl_stream_encoder) {
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 0);
    REG_UPDATE!(enc401, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, 0);
    REG_UPDATE!(enc401, HDMI_STREAM_ENC_CLOCK_CONTROL, HDMI_STREAM_ENC_CLOCK_EN, 0);
}

pub unsafe fn hpo_enc401_read_state(enc: *mut hpo_frl_stream_encoder, state: *mut hpo_frl_stream_encoder_state) {
    let mut pixel_encoding: u32 = 0;
    let mut color_depth: u32 = 0;
    let enc401 = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    ASSERT!(!state.is_null());
    REG_GET!(enc401, HDMI_TB_ENC_CONTROL, HDMI_TB_ENC_EN, &mut (*state).stream_enc_enabled);
    REG_GET!(enc401, HDMI_STREAM_ENC_INPUT_MUX_CONTROL, HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL, &mut (*state).otg_inst);
    REG_GET_2!(enc401, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, &mut pixel_encoding, HDMI_DEEP_COLOR_DEPTH, &mut color_depth);
    REG_GET_2!(enc401, HDMI_TB_ENC_H_ACTIVE_BLANK, HDMI_H_ACTIVE, &mut (*state).h_active, HDMI_H_BLANK, &mut (*state).h_blank);
    REG_GET!(enc401, HDMI_TB_ENC_MODE, HDMI_BORROW_MODE, &mut (*state).borrow_mode);
    (*state).pixel_format = if pixel_encoding == 0 { PIXEL_ENCODING_YCBCR444 } else if pixel_encoding == 1 { PIXEL_ENCODING_YCBCR422 } else { PIXEL_ENCODING_YCBCR420 };
    (*state).color_depth = if color_depth == 0 { 8 } else if color_depth == 1 { 10 } else { 12 };
}

pub unsafe fn hpo_enc401_set_hdmi_stream_attribute(enc: *mut hpo_frl_stream_encoder, crtc_timing: *mut dc_crtc_timing, borrow_params: *mut frl_borrow_params, _odm_combine_num_segments: i32) {
    let e = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    match (*crtc_timing).pixel_encoding {
        PIXEL_ENCODING_YCBCR422 => { REG_UPDATE!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 1); REG_UPDATE!(e, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 0); }
        PIXEL_ENCODING_YCBCR420 => { REG_UPDATE!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 2); REG_UPDATE!(e, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 1); }
        _ => { REG_UPDATE!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_PIXEL_ENCODING, 0); REG_UPDATE!(e, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_UNCOMPRESSED_PIXEL_FORMAT, 0); }
    }
    match (*crtc_timing).display_color_depth {
        COLOR_DEPTH_888 => REG_UPDATE_2!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_DEEP_COLOR_DEPTH, 0, HDMI_DEEP_COLOR_ENABLE, 0),
        COLOR_DEPTH_101010 => REG_UPDATE_2!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_DEEP_COLOR_DEPTH, 1, HDMI_DEEP_COLOR_ENABLE, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 0 } else { 1 }),
        COLOR_DEPTH_121212 => REG_UPDATE_2!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_DEEP_COLOR_DEPTH, 2, HDMI_DEEP_COLOR_ENABLE, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 0 } else { 1 }),
        _ => {}
    }
    if (*crtc_timing).flags.DSC { REG_UPDATE_2!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_DEEP_COLOR_DEPTH, 0, HDMI_DEEP_COLOR_ENABLE, 0); }
    let mut h_active = (*crtc_timing).h_addressable + (*crtc_timing).h_border_left + (*crtc_timing).h_border_right;
    let mut h_blank = (*crtc_timing).h_total - h_active;
    if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 || (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { h_active /= 2; h_blank /= 2; }
    REG_SET_2!(e, HDMI_TB_ENC_H_ACTIVE_BLANK, 0, HDMI_H_ACTIVE, h_active, HDMI_H_BLANK, h_blank);
    REG_UPDATE!(e, HDMI_TB_ENC_MODE, HDMI_BORROW_MODE, (*borrow_params).borrow_mode);
    REG_UPDATE!(e, HDMI_TB_ENC_PACKET_CONTROL, HDMI_MAX_PACKETS_PER_LINE, (*borrow_params).audio_packets_line);
    REG_SET_2!(e, HDMI_TB_ENC_HC_ACTIVE_BLANK, 0, HDMI_HC_ACTIVE, (*borrow_params).hc_active_target, HDMI_HC_BLANK, (*borrow_params).hc_blank_target);
    REG_UPDATE_2!(e, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_GC_CONT, 1, HDMI_GC_SEND, 1);
    REG_UPDATE!(e, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_ACP_SEND, 0);
    REG_UPDATE!(e, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_AUDIO_INFO_SEND, 1);
    if !(*enc).afmt.is_null() && !(*(*enc).afmt).funcs.audio_info_immediate_update.is_none() { ((*(*enc).afmt).funcs.audio_info_immediate_update.unwrap())((*enc).afmt); }
    REG_UPDATE!(e, HDMI_TB_ENC_VBI_PACKET_CONTROL1, HDMI_AUDIO_INFO_LINE, VBI_LINE_0 + 2);
    REG_UPDATE!(e, HDMI_TB_ENC_GC_CONTROL, HDMI_GC_AVMUTE, 0);
}

pub unsafe fn hpo_enc401_update_hdmi_info_packet(enc401: *mut dcn401_hpo_frl_stream_encoder, packet_index: u32, info_packet: *const dc_info_packet) {
    let (cont, send, line) = if (*info_packet).valid { (*(*enc401).base.vpg).funcs.update_generic_info_packet.unwrap()((*enc401).base.vpg, packet_index, info_packet, true); (1,1,2) } else { (0,0,0) };
    match packet_index {
        0..=7 => { let c = packet_index; REG_UPDATE_2!(enc401, HDMI_TB_ENC_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_CONT + c * 2, cont, HDMI_GENERIC0_SEND + c * 2, send); REG_UPDATE!(enc401, HDMI_TB_ENC_GENERIC_PACKET0_1_LINE + (c / 2), HDMI_GENERIC0_LINE + c, line); }
        8..=14 => { let c = packet_index - 8; REG_UPDATE_2!(enc401, HDMI_TB_ENC_GENERIC_PACKET_CONTROL1, HDMI_GENERIC8_CONT + c * 2, cont, HDMI_GENERIC8_SEND + c * 2, send); REG_UPDATE!(enc401, HDMI_TB_ENC_GENERIC_PACKET8_9_LINE + (c / 2), HDMI_GENERIC8_LINE + c, line); }
        _ => { DC_LOG_WARNING!("Invalid HW packet index: %s()\n", "hpo_enc401_update_hdmi_info_packet"); }
    }
}

pub unsafe fn hpo_enc401_update_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder, info_frame: *const encoder_info_frame) {
    let e = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    hpo_enc401_update_hdmi_info_packet(e, 0, &(*info_frame).avi); hpo_enc401_update_hdmi_info_packet(e, 1, &(*info_frame).vendor); hpo_enc401_update_hdmi_info_packet(e, 2, &(*info_frame).gamut); hpo_enc401_update_hdmi_info_packet(e, 3, &(*info_frame).spd); hpo_enc401_update_hdmi_info_packet(e, 4, &(*info_frame).hdrsmd); hpo_enc401_update_hdmi_info_packet(e, 11, &(*info_frame).hfvsif); hpo_enc401_update_hdmi_info_packet(e, 12, &(*info_frame).vtem);
}

pub unsafe fn hpo_enc401_hdmi_set_dsc_config(enc: *mut hpo_frl_stream_encoder, timing: *mut dc_crtc_timing, dsc_packed_pps: *mut u8) {
    let e = DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);
    let dsc_mode = if dsc_packed_pps.is_null() { OPTC_DSC_DISABLED } else if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 || ((*timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 && !(*timing).dsc_cfg.ycbcr422_simple) { OPTC_DSC_ENABLED_NATIVE_SUBSAMPLED } else { OPTC_DSC_ENABLED_444 };
    let (pet, cpf) = match dsc_mode { OPTC_DSC_DISABLED => (0,0), OPTC_DSC_ENABLED_NATIVE_SUBSAMPLED => (1,1), _ => (1,0) };
    REG_UPDATE_2!(e, HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_PIXEL_ENCODING_TYPE, pet, FIFO_COMPRESSED_PIXEL_FORMAT, cpf); REG_UPDATE!(e, HDMI_TB_ENC_PIXEL_FORMAT, HDMI_DSC_MODE, dsc_mode);
    if dsc_mode != OPTC_DSC_DISABLED {
        let mut p: dc_info_packet = core::mem::zeroed(); let mut pad = (*timing).h_addressable % (*timing).dsc_cfg.num_slices_h; if pad != 0 { pad = (*timing).dsc_cfg.num_slices_h - pad; } let sw = ((*timing).h_addressable + pad) / (*timing).dsc_cfg.num_slices_h; if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 && sw % 2 != 0 { pad += (*timing).dsc_cfg.num_slices_h; }
        let pic = (*timing).h_addressable + (*timing).h_border_left + (*timing).h_border_right + pad; let back = (*timing).h_total - pic - (*timing).h_sync_width - (*timing).h_front_porch + pad; let hc = (*timing).dsc_cfg.num_slices_h * ((pic / (*timing).dsc_cfg.num_slices_h * (*timing).dsc_cfg.bits_per_pixel + 127) / 128);
        p.valid=true; p.hb0=0x7f; p.hb1=1<<7; p.sb[0]=(1<<1)|(1<<2)|(1<<7); p.sb[2]=1; p.sb[4]=2; p.sb[6]=136; core::ptr::copy_nonoverlapping(dsc_packed_pps, p.sb.as_mut_ptr().add(7), 21); hpo_enc401_update_hdmi_info_packet(e,5,&p);
        p.hb1=0; for i in 1..4 { p.hb2=i; core::ptr::copy_nonoverlapping(dsc_packed_pps.add(21+28*(i-1)), p.sb.as_mut_ptr(), 28); hpo_enc401_update_hdmi_info_packet(e,5+i,&p); }
        p.hb2=4; core::ptr::copy_nonoverlapping(dsc_packed_pps.add(105),p.sb.as_mut_ptr(),23); p.sb[23]=(*timing).h_front_porch as u8; p.sb[24]=((*timing).h_front_porch>>8) as u8; p.sb[25]=(*timing).h_sync_width as u8; p.sb[26]=((*timing).h_sync_width>>8) as u8; p.sb[27]=back as u8; hpo_enc401_update_hdmi_info_packet(e,9,&p);
        p.hb1=1<<6; p.hb2=5; p.sb[0]=(back>>8) as u8; p.sb[1]=hc as u8; p.sb[2]=(hc>>8) as u8; hpo_enc401_update_hdmi_info_packet(e,10,&p);
        p.hb1=1<<7; p.hb2=0; p.sb[0]=(1<<1)|(1<<2); core::ptr::copy_nonoverlapping(dsc_packed_pps,p.sb.as_mut_ptr().add(7),21); hpo_enc401_update_hdmi_info_packet(e,5,&p);
    }
}

pub unsafe fn hpo_enc401_stop_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder) { let e=DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc); for i in 0..15 { let c=i%8; let r=if i<8 { HDMI_TB_ENC_GENERIC_PACKET_CONTROL0 } else { HDMI_TB_ENC_GENERIC_PACKET_CONTROL1 }; REG_UPDATE_2!(e,r,HDMI_GENERIC0_CONT+i*2,0,HDMI_GENERIC0_SEND+i*2,0); REG_UPDATE!(e,HDMI_TB_ENC_GENERIC_PACKET0_1_LINE+(i/2),HDMI_GENERIC0_LINE+i,0); let _=c; } }

static FRL_AUDIO_CLOCK_INFO_TABLE: [frl_audio_clock_info; 10] = [
    frl_audio_clock_info{frl_character_clock_kHz:166666, n_32khz:4224, cts_32khz:171875, n_44khz:5292, cts_44khz:156250, n_48khz:5760, cts_48khz:156250}, frl_audio_clock_info{frl_character_clock_kHz:166667,n_32khz:4224,cts_32khz:171875,n_44khz:5292,cts_44khz:156250,n_48khz:5760,cts_48khz:156250}, frl_audio_clock_info{frl_character_clock_kHz:333333,n_32khz:4032,cts_32khz:328125,n_44khz:5292,cts_44khz:312500,n_48khz:6048,cts_48khz:328125}, frl_audio_clock_info{frl_character_clock_kHz:333334,n_32khz:4032,cts_32khz:328125,n_44khz:5292,cts_44khz:312500,n_48khz:6048,cts_48khz:328125}, frl_audio_clock_info{frl_character_clock_kHz:444444,n_32khz:4032,cts_32khz:437500,n_44khz:3969,cts_44khz:312500,n_48khz:6048,cts_48khz:437500}, frl_audio_clock_info{frl_character_clock_kHz:444445,n_32khz:4032,cts_32khz:437500,n_44khz:3969,cts_44khz:312500,n_48khz:6048,cts_48khz:437500}, frl_audio_clock_info{frl_character_clock_kHz:555555,n_32khz:3456,cts_32khz:468750,n_44khz:3969,cts_44khz:390625,n_48khz:5184,cts_48khz:468750}, frl_audio_clock_info{frl_character_clock_kHz:555556,n_32khz:3456,cts_32khz:468750,n_44khz:3969,cts_44khz:390625,n_48khz:5184,cts_48khz:468750}, frl_audio_clock_info{frl_character_clock_kHz:666666,n_32khz:3072,cts_32khz:500000,n_44khz:3969,cts_44khz:468750,n_48khz:4752,cts_48khz:515625}, frl_audio_clock_info{frl_character_clock_kHz:666667,n_32khz:3072,cts_32khz:500000,n_44khz:3969,cts_44khz:468750,n_48khz:4752,cts_48khz:515625}];
pub unsafe fn frl_get_audio_clock_info(_color_depth: dc_color_depth, clock:u32, out:*mut frl_audio_clock_info) { for x in FRL_AUDIO_CLOCK_INFO_TABLE.iter() { if x.frl_character_clock_kHz==clock { *out=*x; return; } if x.frl_character_clock_kHz>clock { break; } } BREAK_TO_DEBUGGER!(); }
pub unsafe fn hpo_enc401_setup_hdmi_audio(enc:*mut hpo_frl_stream_encoder, info:*const audio_crtc_info) { let e=DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc); ASSERT!(!(*enc).afmt.is_null()); ((*(*enc).afmt).funcs.setup_hdmi_audio.unwrap())((*enc).afmt); REG_UPDATE_3!(e,HDMI_TB_ENC_ACR_PACKET_CONTROL,HDMI_ACR_AUTO_SEND,1,HDMI_ACR_SOURCE,0,HDMI_ACR_AUDIO_PRIORITY,0); let mut a:frl_audio_clock_info=core::mem::zeroed(); frl_get_audio_clock_info((*info).color_depth,(*info).frl_character_clock_kHz,&mut a); REG_UPDATE!(e,HDMI_TB_ENC_ACR_32_0,HDMI_ACR_CTS_32,a.cts_32khz); REG_UPDATE!(e,HDMI_TB_ENC_ACR_32_1,HDMI_ACR_N_32,a.n_32khz); REG_UPDATE!(e,HDMI_TB_ENC_ACR_44_0,HDMI_ACR_CTS_44,a.cts_44khz); REG_UPDATE!(e,HDMI_TB_ENC_ACR_44_1,HDMI_ACR_N_44,a.n_44khz); REG_UPDATE!(e,HDMI_TB_ENC_ACR_48_0,HDMI_ACR_CTS_48,a.cts_48khz); REG_UPDATE!(e,HDMI_TB_ENC_ACR_48_1,HDMI_ACR_N_48,a.n_48khz); }
pub unsafe fn hpo_enc401_hdmi_audio_setup(enc:*mut hpo_frl_stream_encoder, az:u32, info:*mut audio_info, ci:*mut audio_crtc_info){hpo_enc401_setup_hdmi_audio(enc,ci);ASSERT!(!(*enc).afmt.is_null());((*(*enc).afmt).funcs.se_audio_setup.unwrap())((*enc).afmt,az,info);}
pub unsafe fn hpo_enc401_hdmi_audio_disable(enc:*mut hpo_frl_stream_encoder){ASSERT!(!(*enc).afmt.is_null());if let Some(f)=(*(*enc).afmt).funcs.afmt_powerdown{f((*enc).afmt);}}
pub unsafe fn hpo_enc401_audio_mute_control(enc:*mut hpo_frl_stream_encoder,mute:bool){ASSERT!(!(*enc).afmt.is_null());((*(*enc).afmt).funcs.audio_mute_control.unwrap())((*enc).afmt,mute);}
pub unsafe fn enc401_stream_encoder_set_avmute(enc:*mut hpo_frl_stream_encoder,enable:bool){let e=DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);REG_UPDATE!(e,HDMI_TB_ENC_GC_CONTROL,HDMI_GC_AVMUTE,if enable{1}else{0});}
pub unsafe fn hpo_enc401_set_dynamic_metadata(enc:*mut hpo_frl_stream_encoder,en:bool,id:u32,mode:dynamic_metadata_mode){let e=DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(enc);if en{REG_UPDATE_2!(e,DME_CONTROL,METADATA_HUBP_REQUESTOR_ID,id,METADATA_STREAM_TYPE,if mode==dmdata_dolby_vision{1}else{0});REG_UPDATE_3!(e,HDMI_TB_ENC_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,1,HDMI_METADATA_PACKET_LINE_REFERENCE,0,HDMI_METADATA_PACKET_LINE,2);REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,1);}else{REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,0);REG_UPDATE!(e,HDMI_TB_ENC_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,0);}}

pub unsafe fn dcn401_hpo_frl_stream_encoder_construct(enc401:*mut dcn401_hpo_frl_stream_encoder,ctx:*mut dc_context,bp:*mut dc_bios,eng_id:engine_id,vpg:*mut vpg,afmt:*mut afmt,regs:*const dcn30_hpo_frl_stream_enc_registers,shift:*const dcn401_hpo_frl_stream_encoder_shift,mask:*const dcn401_hpo_frl_stream_encoder_mask){(*enc401).base.ctx=ctx;(*enc401).base.id=eng_id;(*enc401).base.bp=bp;(*enc401).base.vpg=vpg;(*enc401).base.afmt=afmt;(*enc401).regs=regs;(*enc401).hpo_se_shift=shift;(*enc401).hpo_se_mask=mask;(*enc401).base.stream_enc_inst=(*vpg).inst;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
