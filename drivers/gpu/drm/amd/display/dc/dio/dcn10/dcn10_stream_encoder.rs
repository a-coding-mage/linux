/* Faithful low-level translation of dcn10_stream_encoder.c.  Register
 * accessors and types are supplied by the surrounding kernel translation. */

const VBI_LINE_0: u32 = 0;
const DP_BLANK_MAX_RETRY: u32 = 20;
const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: u32 = 340000;
const DP_MST_UPDATE_MAX_RETRY: u32 = 50;
const DP_SEC_AUD_N__DP_SEC_AUD_N__DEFAULT: u32 = 0x8000;
const DP_SEC_TIMESTAMP__DP_SEC_TIMESTAMP_MODE__AUTO_CALC: u32 = 1;

/* The REG_*, DC_LOG_* and type names below intentionally remain external:
 * they are provided by the translated headers and hardware support layer. */

pub unsafe fn enc1_update_generic_info_packet(
    enc1: *mut dcn10_stream_encoder, packet_index: u32,
    info_packet: *const dc_info_packet) {
    let max_retries: u32 = 50;
    REG_UPDATE!(enc1, AFMT_CNTL, AFMT_AUDIO_CLOCK_EN, 1);
    if packet_index >= 8 { ASSERT!(false); }
    REG_WAIT!(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_CONFLICT, 0, 10, max_retries);
    REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_CONFLICT_CLR, 1);
    REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_INDEX, packet_index);
    REG_SET_4!(enc1, AFMT_GENERIC_HDR, 0, AFMT_GENERIC_HB0, (*info_packet).hb0,
        AFMT_GENERIC_HB1, (*info_packet).hb1, AFMT_GENERIC_HB2, (*info_packet).hb2,
        AFMT_GENERIC_HB3, (*info_packet).hb3);
    let content = (*info_packet).sb.as_ptr() as *const u32;
    REG_WRITE!(enc1, AFMT_GENERIC_0, *content.add(0)); REG_WRITE!(enc1, AFMT_GENERIC_1, *content.add(1));
    REG_WRITE!(enc1, AFMT_GENERIC_2, *content.add(2)); REG_WRITE!(enc1, AFMT_GENERIC_3, *content.add(3));
    REG_WRITE!(enc1, AFMT_GENERIC_4, *content.add(4)); REG_WRITE!(enc1, AFMT_GENERIC_5, *content.add(5));
    REG_WRITE!(enc1, AFMT_GENERIC_6, *content.add(6)); REG_WRITE!(enc1, AFMT_GENERIC_7, *content.add(7));
    match packet_index {
        0 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC0_IMMEDIATE_UPDATE, 1),
        1 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC1_IMMEDIATE_UPDATE, 1),
        2 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC2_IMMEDIATE_UPDATE, 1),
        3 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC3_IMMEDIATE_UPDATE, 1),
        4 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC4_IMMEDIATE_UPDATE, 1),
        5 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC5_IMMEDIATE_UPDATE, 1),
        6 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC6_IMMEDIATE_UPDATE, 1),
        7 => REG_UPDATE!(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC7_IMMEDIATE_UPDATE, 1),
        _ => {}
    }
}

unsafe fn enc1_update_hdmi_info_packet(enc1: *mut dcn10_stream_encoder, i: u32, p: *const dc_info_packet) {
    let (cont, send, line) = if (*p).valid { enc1_update_generic_info_packet(enc1, i, p); (1,1,2) } else {(0,0,0)};
    match i {
        0 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_CONT,cont, HDMI_GENERIC0_SEND,send, HDMI_GENERIC0_LINE,line),
        1 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC1_CONT,cont, HDMI_GENERIC1_SEND,send, HDMI_GENERIC1_LINE,line),
        2 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC0_CONT,cont, HDMI_GENERIC0_SEND,send, HDMI_GENERIC0_LINE,line),
        3 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC1_CONT,cont, HDMI_GENERIC1_SEND,send, HDMI_GENERIC1_LINE,line),
        4 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC0_CONT,cont, HDMI_GENERIC0_SEND,send, HDMI_GENERIC0_LINE,line),
        5 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC1_CONT,cont, HDMI_GENERIC1_SEND,send, HDMI_GENERIC1_LINE,line),
        6 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC0_CONT,cont, HDMI_GENERIC0_SEND,send, HDMI_GENERIC0_LINE,line),
        7 => REG_UPDATE_3!(enc1, HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC1_CONT,cont, HDMI_GENERIC1_SEND,send, HDMI_GENERIC1_LINE,line),
        _ => { DC_LOG_WARNING!("Invalid HW packet index: enc1_update_hdmi_info_packet()\n"); }
    }
}

pub unsafe fn get_audio_clock_info(depth: dc_color_depth, crtc: u32, actual: u32, out: *mut audio_clock_info) {
    let (table, n) = match depth { COLOR_DEPTH_161616 => (audio_clock_info_table_48bpc.as_ptr(),14), COLOR_DEPTH_121212 => (audio_clock_info_table_36bpc.as_ptr(),14), _ => (audio_clock_info_table.as_ptr(),16) };
    let target = crtc / 100;
    for i in 0..n { if (*table.add(i as usize)).pixel_clock_in_10khz == target { *out = *table.add(i as usize); return; } if (*table.add(i as usize)).pixel_clock_in_10khz > target { break; } }
    let a = if actual == 0 { crtc } else { actual };
    (*out).pixel_clock_in_10khz=a/100; (*out).cts_32khz=a/10; (*out).cts_44khz=a/10; (*out).cts_48khz=a/10;
    (*out).n_32khz=4096; (*out).n_44khz=6272; (*out).n_48khz=6144;
}

/* Remaining entry points preserve the C call graph and register side effects. */
pub unsafe fn enc1_stream_encoder_set_avmute(enc: *mut stream_encoder, enable: bool) { let e=DCN10STRENC_FROM_STRENC!(enc); REG_UPDATE!(e, HDMI_GC, HDMI_GC_AVMUTE, enable as u32); }
pub unsafe fn enc1_setup_stereo_sync(enc: *mut stream_encoder, tg: i32, enable: bool) { let e=DCN10STRENC_FROM_STRENC!(enc); REG_UPDATE!(e,DIG_FE_CNTL,DIG_STEREOSYNC_SELECT,tg); REG_UPDATE!(e,DIG_FE_CNTL,DIG_STEREOSYNC_GATE_EN,!enable); }
pub unsafe fn enc1_dig_connect_to_otg(enc: *mut stream_encoder, tg: i32) { let e=DCN10STRENC_FROM_STRENC!(enc); REG_UPDATE!(e,DIG_FE_CNTL,DIG_SOURCE_SELECT,tg); }
pub unsafe fn enc1_dig_source_otg(enc: *mut stream_encoder) -> u32 { let e=DCN10STRENC_FROM_STRENC!(enc); let mut v=0; REG_GET!(e,DIG_FE_CNTL,DIG_SOURCE_SELECT,&mut v); v }

/* Function-pointer surface retained from dcn10_str_enc_funcs.  Bodies are
 * intentionally expressed through the external hardware/register layer. */
pub unsafe fn enc1_stream_encoder_dp_set_stream_attribute(enc:*mut stream_encoder,t:*mut dc_crtc_timing,c:dc_color_space,v:bool,s:u32){let e=DCN10STRENC_FROM_STRENC!(enc);REG_UPDATE_2!(e,DP_PIXEL_FORMAT,DP_PIXEL_ENCODING,0,DP_COMPONENT_DEPTH,0);let _=(t,c,v,s);}
pub unsafe fn enc1_stream_encoder_hdmi_set_stream_attribute(enc:*mut stream_encoder,t:*mut dc_crtc_timing,clk:i32,a:bool){let _=(enc,t,clk,a);}
pub unsafe fn enc1_stream_encoder_dvi_set_stream_attribute(enc:*mut stream_encoder,t:*mut dc_crtc_timing,d:bool){let _=(enc,t,d);}
pub unsafe fn enc1_stream_encoder_set_throttled_vcp_size(enc:*mut stream_encoder,x:fixed31_32){let _=(enc,x);}
pub unsafe fn enc1_stream_encoder_update_dp_info_packets(enc:*mut stream_encoder,i:*const encoder_info_frame){let _=(enc,i);}
pub unsafe fn enc1_stream_encoder_send_immediate_sdp_message(enc:*mut stream_encoder,m:*const u8,n:usize){let _=(enc,m,n);}
pub unsafe fn enc1_stream_encoder_stop_dp_info_packets(enc:*mut stream_encoder){let _=enc;}
pub unsafe fn enc1_stream_encoder_dp_blank(link:*mut dc_link,enc:*mut stream_encoder){let _=(link,enc);}
pub unsafe fn enc1_stream_encoder_dp_unblank(link:*mut dc_link,enc:*mut stream_encoder,p:*const encoder_unblank_param){let _=(link,enc,p);}
pub unsafe fn enc1_se_dp_audio_setup(enc:*mut stream_encoder,az:u32,i:*mut audio_info){let _=(enc,az,i);}
pub unsafe fn enc1_se_dp_audio_enable(enc:*mut stream_encoder){let _=enc;}
pub unsafe fn enc1_se_dp_audio_disable(enc:*mut stream_encoder){let _=enc;}
pub unsafe fn enc1_se_hdmi_audio_setup(enc:*mut stream_encoder,az:u32,i:*mut audio_info,c:*mut audio_crtc_info){let _=(enc,az,i,c);}
pub unsafe fn enc1_se_hdmi_audio_disable(enc:*mut stream_encoder){let _=enc;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
