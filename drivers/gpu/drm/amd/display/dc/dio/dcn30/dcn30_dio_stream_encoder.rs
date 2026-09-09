/* Rust translation of dcn30_dio_stream_encoder.c. */

// Dependencies supplied by the surrounding display driver are intentionally external.
use core::ffi::c_void;

pub const VBI_LINE_0: u32 = 0;
pub const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: u32 = 340000;
pub const DP_SEC_AUD_N__DP_SEC_AUD_N__DEFAULT: u32 = 0x8000;
pub const DP_SEC_TIMESTAMP__DP_SEC_TIMESTAMP_MODE__AUTO_CALC: u32 = 1;

/* Register helpers and all hardware symbols below are provided by the driver bindings. */
macro_rules! REG_UPDATE { ($($x:tt)*) => { unsafe { reg_update!($($x)*) } } }
macro_rules! REG_UPDATE_2 { ($($x:tt)*) => { unsafe { reg_update_2!($($x)*) } } }
macro_rules! REG_UPDATE_3 { ($($x:tt)*) => { unsafe { reg_update_3!($($x)*) } } }
macro_rules! REG_UPDATE_6 { ($($x:tt)*) => { unsafe { reg_update_6!($($x)*) } } }
macro_rules! REG_SET { ($($x:tt)*) => { unsafe { reg_set!($($x)*) } } }
macro_rules! REG_SET_2 { ($($x:tt)*) => { unsafe { reg_set_2!($($x)*) } } }
macro_rules! REG_SET_4 { ($($x:tt)*) => { unsafe { reg_set_4!($($x)*) } } }
macro_rules! REG_GET { ($($x:tt)*) => { unsafe { reg_get!($($x)*) } } }
macro_rules! REG_READ { ($($x:tt)*) => { unsafe { reg_read!($($x)*) } } }

pub unsafe fn enc3_update_hdmi_info_packet(enc1: *mut dcn10_stream_encoder, packet_index: u32, info_packet: *const dc_info_packet) {
    let (cont, send, line) = if (*info_packet).valid {
        (*enc1).base.vpg.update_generic_info_packet((*enc1).base.vpg, packet_index, info_packet, true);
        (1u32, 1u32, 2u32)
    } else { (0, 0, 0) };
    match packet_index {
        0 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_CONT, cont, HDMI_GENERIC0_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC0_LINE, line); }
        1 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC1_CONT, cont, HDMI_GENERIC1_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC1_LINE, line); }
        2 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC2_CONT, cont, HDMI_GENERIC2_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC2_LINE, line); }
        3 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC3_CONT, cont, HDMI_GENERIC3_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC3_LINE, line); }
        4 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC4_CONT, cont, HDMI_GENERIC4_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC4_LINE, line); }
        5 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC5_CONT, cont, HDMI_GENERIC5_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC5_LINE, line); }
        6 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC6_CONT, cont, HDMI_GENERIC6_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC6_LINE, line); }
        7 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC7_CONT, cont, HDMI_GENERIC7_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC7_LINE, line); }
        8 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC8_CONT, cont, HDMI_GENERIC8_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL7, HDMI_GENERIC8_LINE, line); }
        9 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC9_CONT, cont, HDMI_GENERIC9_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL7, HDMI_GENERIC9_LINE, line); }
        10 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC10_CONT, cont, HDMI_GENERIC10_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL8, HDMI_GENERIC10_LINE, line); }
        11 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC11_CONT, cont, HDMI_GENERIC11_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL8, HDMI_GENERIC11_LINE, line); }
        12 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC12_CONT, cont, HDMI_GENERIC12_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL9, HDMI_GENERIC12_LINE, line); }
        13 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC13_CONT, cont, HDMI_GENERIC13_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL9, HDMI_GENERIC13_LINE, line); }
        14 => { REG_UPDATE_2!(HDMI_GENERIC_PACKET_CONTROL6, HDMI_GENERIC14_CONT, cont, HDMI_GENERIC14_SEND, send); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL10, HDMI_GENERIC14_LINE, line); }
        _ => { unsafe { dc_log_warning!("Invalid HW packet index: enc3_update_hdmi_info_packet()\n"); } }
    }
}

pub unsafe fn enc3_stream_encoder_update_hdmi_info_packets(enc: *mut stream_encoder, f: *const encoder_info_frame) {
    let e = DCN10STRENC_FROM_STRENC(enc); REG_UPDATE!(HDMI_DB_CONTROL, HDMI_DB_DISABLE, 1); REG_UPDATE!(AFMT_CNTL, AFMT_AUDIO_CLOCK_EN, 1);
    enc3_update_hdmi_info_packet(e, 0, &(*f).avi); enc3_update_hdmi_info_packet(e, 5, &(*f).hfvsif); enc3_update_hdmi_info_packet(e, 2, &(*f).gamut); enc3_update_hdmi_info_packet(e, 1, &(*f).vendor); enc3_update_hdmi_info_packet(e, 3, &(*f).spd); enc3_update_hdmi_info_packet(e, 4, &(*f).hdrsmd); enc3_update_hdmi_info_packet(e, 6, &(*f).vtem);
}

pub unsafe fn enc3_stream_encoder_stop_hdmi_info_packets(enc: *mut stream_encoder) { let _e = DCN10STRENC_FROM_STRENC(enc);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL0,0,HDMI_GENERIC0_CONT,0,HDMI_GENERIC0_SEND,0,HDMI_GENERIC1_CONT,0,HDMI_GENERIC1_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL1,0,HDMI_GENERIC0_LINE,0,HDMI_GENERIC1_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL0,0,HDMI_GENERIC2_CONT,0,HDMI_GENERIC2_SEND,0,HDMI_GENERIC3_CONT,0,HDMI_GENERIC3_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL2,0,HDMI_GENERIC2_LINE,0,HDMI_GENERIC3_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL0,0,HDMI_GENERIC4_CONT,0,HDMI_GENERIC4_SEND,0,HDMI_GENERIC5_CONT,0,HDMI_GENERIC5_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL3,0,HDMI_GENERIC4_LINE,0,HDMI_GENERIC5_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL0,0,HDMI_GENERIC6_CONT,0,HDMI_GENERIC6_SEND,0,HDMI_GENERIC7_CONT,0,HDMI_GENERIC7_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL4,0,HDMI_GENERIC6_LINE,0,HDMI_GENERIC7_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL6,0,HDMI_GENERIC8_CONT,0,HDMI_GENERIC8_SEND,0,HDMI_GENERIC9_CONT,0,HDMI_GENERIC9_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL7,0,HDMI_GENERIC8_LINE,0,HDMI_GENERIC9_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL6,0,HDMI_GENERIC10_CONT,0,HDMI_GENERIC10_SEND,0,HDMI_GENERIC11_CONT,0,HDMI_GENERIC11_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL8,0,HDMI_GENERIC10_LINE,0,HDMI_GENERIC11_LINE,0);
    REG_SET_4!(HDMI_GENERIC_PACKET_CONTROL6,0,HDMI_GENERIC12_CONT,0,HDMI_GENERIC12_SEND,0,HDMI_GENERIC13_CONT,0,HDMI_GENERIC13_SEND,0); REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL9,0,HDMI_GENERIC12_LINE,0,HDMI_GENERIC13_LINE,0);
    REG_SET_2!(HDMI_GENERIC_PACKET_CONTROL6,0,HDMI_GENERIC14_CONT,0,HDMI_GENERIC14_SEND,0); REG_UPDATE!(HDMI_GENERIC_PACKET_CONTROL10,HDMI_GENERIC14_LINE,0);
}

pub unsafe fn enc3_dp_set_dsc_config(enc: *mut stream_encoder, mode: optc_dsc_mode, bpp: u32, width: u32) { let _e=DCN10STRENC_FROM_STRENC(enc); REG_UPDATE_2!(DP_DSC_CNTL,DP_DSC_MODE,mode,DP_DSC_SLICE_WIDTH,width); REG_SET!(DP_DSC_BYTES_PER_PIXEL,0,DP_DSC_BYTES_PER_PIXEL,bpp); }

pub unsafe fn enc3_dp_set_dsc_pps_info_packet(enc: *mut stream_encoder, enable: bool, pps: *mut u8, immediate: bool) { let e=DCN10STRENC_FROM_STRENC(enc); if enable { REG_UPDATE!(DP_SEC_CNTL2,DP_SEC_GSP11_PPS,1); REG_UPDATE!(AFMT_CNTL,AFMT_AUDIO_CLOCK_EN,1); let mut p=dc_info_packet::default(); p.valid=true; p.hb1=DC_DP_INFOFRAME_TYPE_PPS; p.hb2=127; for i in 0..4 { core::ptr::copy_nonoverlapping(pps.add(i*32),p.sb.as_mut_ptr(),32); (*e).base.vpg.update_generic_info_packet((*e).base.vpg,11+i as u32,&p,immediate); } REG_UPDATE!(DP_GSP11_CNTL,DP_SEC_GSP11_LINE_NUM,2); REG_UPDATE_2!(DP_MSA_VBID_MISC,DP_VBID6_LINE_REFERENCE,0,DP_VBID6_LINE_NUM,3); REG_UPDATE!(DP_GSP11_CNTL,DP_SEC_GSP11_ENABLE,1); REG_UPDATE!(DP_SEC_CNTL,DP_SEC_STREAM_ENABLE,1); } else { REG_UPDATE!(DP_GSP11_CNTL,DP_SEC_GSP11_ENABLE,0); REG_UPDATE!(DP_SEC_CNTL2,DP_SEC_GSP11_PPS,0); } }

// Remaining declarations retain the source interfaces; their external helper bodies are supplied by the driver.
pub unsafe fn enc3_stream_encoder_update_dp_info_packets_sdp_line_num(enc:*mut stream_encoder, f:*mut encoder_info_frame){let _e=DCN10STRENC_FROM_STRENC(enc);if (*f).adaptive_sync.valid&&(*f).sdp_line_num.adaptive_sync_line_num_valid{REG_UPDATE!(DP_SEC_CNTL1,DP_SEC_GSP5_LINE_REFERENCE,1);REG_UPDATE!(DP_SEC_CNTL5,DP_SEC_GSP5_LINE_NUM,(*f).sdp_line_num.adaptive_sync_line_num);}}
pub unsafe fn enc3_audio_mute_control(enc:*mut stream_encoder,mute:bool){(*enc).afmt.audio_mute_control((*enc).afmt,mute)}
pub unsafe fn enc3_se_dp_audio_enable(enc:*mut stream_encoder){enc1_se_enable_audio_clock(enc,true);enc1_se_enable_dp_audio(enc);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
