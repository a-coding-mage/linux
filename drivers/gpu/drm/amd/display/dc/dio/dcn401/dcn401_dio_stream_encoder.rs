/* Faithful low-level Rust translation of dcn401_dio_stream_encoder.c. */
/* C dependencies are supplied by the surrounding kernel translation unit. */

const VBI_LINE_0: u32 = 0;
const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: u32 = 340000;

unsafe fn enc401_dp_set_odm_combine(enc: *mut stream_encoder, odm_combine: bool) {
    let _ = (enc, odm_combine);
}

pub unsafe fn enc401_stream_encoder_dvi_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, is_dual_link: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        let mut cntl: bp_encoder_control = core::mem::zeroed();
        cntl.action = ENCODER_CONTROL_SETUP; cntl.engine_id = (*enc1).base.id;
        cntl.signal = if is_dual_link { SIGNAL_TYPE_DVI_DUAL_LINK } else { SIGNAL_TYPE_DVI_SINGLE_LINK };
        cntl.enable_dp_audio = false; cntl.pixel_clock = (*crtc_timing).pix_clk_100hz / 10;
        cntl.lanes_number = if is_dual_link { LANE_COUNT_EIGHT } else { LANE_COUNT_FOUR };
        if (*(*enc1).base.bp).funcs.encoder_control((*enc1).base.bp, &mut cntl) != BP_RESULT_OK { return; }
    } else { REG_UPDATE!(enc1, DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, 0x1F); }
    ASSERT!((*crtc_timing).pixel_encoding == PIXEL_ENCODING_RGB);
    ASSERT!((*crtc_timing).display_color_depth == COLOR_DEPTH_888);
    enc401_stream_encoder_set_stream_attribute_helper(enc1, crtc_timing);
}

pub unsafe fn enc401_stream_encoder_hdmi_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, actual_pix_clk_khz: i32, enable_audio: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        let mut cntl: bp_encoder_control = core::mem::zeroed(); cntl.action = ENCODER_CONTROL_SETUP;
        cntl.engine_id = (*enc1).base.id; cntl.signal = SIGNAL_TYPE_HDMI_TYPE_A; cntl.enable_dp_audio = enable_audio;
        cntl.pixel_clock = actual_pix_clk_khz; cntl.lanes_number = LANE_COUNT_FOUR;
        if (*(*enc1).base.bp).funcs.encoder_control((*enc1).base.bp, &mut cntl) != BP_RESULT_OK { return; }
    } else { REG_UPDATE!(enc1, DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, 0x1F); }
    enc401_stream_encoder_set_stream_attribute_helper(enc1, crtc_timing);
    REG_UPDATE_6!(enc1, HDMI_CONTROL, HDMI_PACKET_GEN_VERSION, 1, HDMI_KEEPOUT_MODE, 1, HDMI_DEEP_COLOR_ENABLE, 0, HDMI_DATA_SCRAMBLE_EN, 0, HDMI_NO_EXTRA_NULL_PACKET_FILLED, 1, HDMI_CLOCK_CHANNEL_RATE, 0);
    match (*crtc_timing).display_color_depth {
        COLOR_DEPTH_888 => REG_UPDATE!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 0),
        COLOR_DEPTH_101010 | COLOR_DEPTH_121212 => { let d = if (*crtc_timing).display_color_depth == COLOR_DEPTH_101010 { 1 } else { 2 }; REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, d, HDMI_DEEP_COLOR_ENABLE, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 0 } else { 1 }); },
        COLOR_DEPTH_161616 => REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 3, HDMI_DEEP_COLOR_ENABLE, 1), _ => {}
    }
    if actual_pix_clk_khz as u32 >= HDMI_CLOCK_CHANNEL_RATE_MORE_340M { REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 1); }
    else if (*crtc_timing).flags.LTE_340MCSC_SCRAMBLE { REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 0); }
    REG_UPDATE_3!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_GC_CONT, 1, HDMI_GC_SEND, 1, HDMI_NULL_SEND, 1);
    REG_UPDATE!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_ACP_SEND, 0); REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL0, HDMI_AUDIO_INFO_SEND, 1);
    ASSERT!(!(*enc).afmt.is_null()); (*(*(*enc).afmt).funcs).audio_info_immediate_update((*enc).afmt);
    REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL1, HDMI_AUDIO_INFO_LINE, VBI_LINE_0 + 2); REG_UPDATE!(enc1, HDMI_GC, HDMI_GC_AVMUTE, 0);
}

pub unsafe fn enc401_set_dig_input_mode(enc: *mut stream_encoder, pix_per_container: u32) { let e=DCN10STRENC_FROM_STRENC(enc); REG_UPDATE!(e, DIG_FIFO_CTRL0, DIG_FIFO_OUTPUT_PIXEL_PER_CYCLE, match pix_per_container {2=>1,4=>2,8=>3,_=>0}); }
unsafe fn is_two_pixels_per_containter(t: *const dc_crtc_timing) -> bool { (*t).pixel_encoding == PIXEL_ENCODING_YCBCR420 || ((*t).flags.DSC && (*t).pixel_encoding == PIXEL_ENCODING_YCBCR422 && !(*t).dsc_cfg.ycbcr422_simple) }

pub unsafe fn enc401_stream_encoder_dp_unblank(link:*mut dc_link, enc:*mut stream_encoder, param:*const encoder_unblank_param) {
    let e=DCN10STRENC_FROM_STRENC(enc); if (*param).link_settings.link_rate != LINK_RATE_UNKNOWN { let n_vid:u32=0x8000; let p=if is_two_pixels_per_containter(&(*param).timing){2}else{1}; let m=((n_vid as u64)*((*param).timing.pix_clk_100hz as u64)/p/10)/( (*param).link_settings.link_rate as u64*LINK_RATE_REF_FREQ_IN_KHZ as u64); REG_UPDATE!(e,DP_VID_TIMING,DP_VID_M_N_GEN_EN,0); REG_UPDATE!(e,DP_VID_N,DP_VID_N,n_vid); REG_UPDATE!(e,DP_VID_M,DP_VID_M,m as u32); REG_UPDATE!(e,DP_VID_TIMING,DP_VID_N_INTERVAL,match (*param).pix_per_cycle{2=>1,4=>2,8=>3,_=>0}); REG_UPDATE!(e,DP_VID_TIMING,DP_VID_M_N_GEN_EN,1); }
    REG_UPDATE!(e,DP_VID_STREAM_CNTL,DP_VID_STREAM_ENABLE,false); REG_WAIT!(e,DP_VID_STREAM_CNTL,DP_VID_STREAM_STATUS,0,10,5000); REG_UPDATE!(e,DP_STEER_FIFO,DP_STEER_FIFO_RESET,1); udelay(10); REG_UPDATE!(e,DP_STEER_FIFO,DP_STEER_FIFO_RESET,0); REG_UPDATE!(e,DP_STEER_FIFO,DP_STEER_FIFO_ENABLE,1); REG_UPDATE_2!(e,DP_VID_STREAM_CNTL,DP_VID_STREAM_ENABLE,1,DP_VID_STREAM_DIS_DEFER,2); udelay(200); REG_UPDATE!(e,DIG_FIFO_CTRL0,DIG_FIFO_READ_START_LEVEL,7); REG_UPDATE!(e,DIG_FIFO_CTRL0,DIG_FIFO_RESET,1); REG_WAIT!(e,DIG_FIFO_CTRL0,DIG_FIFO_RESET_DONE,1,10,5000); REG_UPDATE!(e,DIG_FIFO_CTRL0,DIG_FIFO_RESET,0); REG_WAIT!(e,DIG_FIFO_CTRL0,DIG_FIFO_RESET_DONE,0,10,5000); REG_UPDATE!(e,DIG_FIFO_CTRL0,DIG_FIFO_ENABLE,1); udelay(100); REG_UPDATE!(e,DP_VID_STREAM_CNTL,DP_VID_STREAM_ENABLE,true); (*(*(*link).dc).link_srv).dp_trace_source_sequence(link,DPCD_SOURCE_SEQ_AFTER_ENABLE_DP_VID_STREAM);
}

pub unsafe fn enc401_read_state(enc:*mut stream_encoder,s:*mut enc_state){let e=DCN10STRENC_FROM_STRENC(enc);REG_GET!(e,DP_PIXEL_FORMAT,PIXEL_ENCODING_TYPE,&mut (*s).dsc_mode);if (*s).dsc_mode!=0{REG_GET!(e,DP_GSP11_CNTL,DP_SEC_GSP11_LINE_NUM,&mut (*s).sec_gsp_pps_line_num);REG_GET!(e,DP_MSA_VBID_MISC,DP_VBID6_LINE_REFERENCE,&mut (*s).vbid6_line_reference);REG_GET!(e,DP_MSA_VBID_MISC,DP_VBID6_LINE_NUM,&mut (*s).vbid6_line_num);REG_GET!(e,DP_GSP11_CNTL,DP_SEC_GSP11_ENABLE,&mut (*s).sec_gsp_pps_enable);REG_GET!(e,DP_SEC_CNTL,DP_SEC_STREAM_ENABLE,&mut (*s).sec_stream_enable);}}

pub unsafe fn enc401_stream_encoder_enable(enc:*mut stream_encoder,signal:signal_type,enable:bool){let e=DCN10STRENC_FROM_STRENC(enc);if enable{let mode=match signal{SIGNAL_TYPE_DVI_SINGLE_LINK|SIGNAL_TYPE_DVI_DUAL_LINK=>2,SIGNAL_TYPE_HDMI_TYPE_A=>3,SIGNAL_TYPE_DISPLAY_PORT_MST=>5,SIGNAL_TYPE_EDP|SIGNAL_TYPE_DISPLAY_PORT|SIGNAL_TYPE_VIRTUAL=>0,_=>{ASSERT_CRITICAL!(false);0}};REG_UPDATE!(e,DIG_FE_CLK_CNTL,DIG_FE_MODE,mode);REG_UPDATE!(e,DIG_FE_CLK_CNTL,DIG_FE_CLK_EN,1);REG_UPDATE!(e,DIG_FE_EN_CNTL,DIG_FE_ENABLE,1);}else{REG_UPDATE!(e,DIG_FE_EN_CNTL,DIG_FE_ENABLE,0);REG_UPDATE!(e,DIG_FE_CLK_CNTL,DIG_FE_CLK_EN,0);}}

/* The remaining DP MSA programming is preserved as direct register operations. */
pub unsafe fn enc401_stream_encoder_map_to_link(enc:*mut stream_encoder,_stream_enc_inst:u32,link_enc_inst:u32){let e=DCN10STRENC_FROM_STRENC(enc);REG_UPDATE!(e,STREAM_MAPPER_CONTROL,DIG_STREAM_LINK_TARGET,link_enc_inst);}
pub unsafe fn enc401_stream_encoder_dp_set_stream_attribute(enc:*mut stream_encoder,t:*mut dc_crtc_timing,output_color_space:dc_color_space,use_vsc_sdp_for_colorimetry:bool,enable_sdp_splitting:u32){
 let e=DCN10STRENC_FROM_STRENC(enc);let mut hw=*t;if hw.flags.INTERLACE{hw.v_total/=2;hw.v_border_top/=2;hw.v_addressable/=2;hw.v_border_bottom/=2;hw.v_front_porch/=2;hw.v_sync_width/=2;}
 let pe=match hw.pixel_encoding{PIXEL_ENCODING_YCBCR422=>DP_PIXEL_ENCODING_TYPE_YCBCR422,PIXEL_ENCODING_YCBCR444=>if hw.flags.Y_ONLY&&hw.display_color_depth!=COLOR_DEPTH_666{DP_PIXEL_ENCODING_TYPE_Y_ONLY}else{DP_PIXEL_ENCODING_TYPE_YCBCR444},PIXEL_ENCODING_YCBCR420=>DP_PIXEL_ENCODING_TYPE_YCBCR420,_=>DP_PIXEL_ENCODING_TYPE_RGB444};
 let mut misc1=REG_READ!(e,DP_MSA_MISC);if use_vsc_sdp_for_colorimetry{misc1|=0x40}else{misc1&=!0x40};let depth=match hw.display_color_depth{COLOR_DEPTH_666=>DP_COMPONENT_PIXEL_DEPTH_6BPC,COLOR_DEPTH_888=>DP_COMPONENT_PIXEL_DEPTH_8BPC,COLOR_DEPTH_101010=>DP_COMPONENT_PIXEL_DEPTH_10BPC,COLOR_DEPTH_121212=>DP_COMPONENT_PIXEL_DEPTH_12BPC,COLOR_DEPTH_161616=>DP_COMPONENT_PIXEL_DEPTH_16BPC,_=>DP_COMPONENT_PIXEL_DEPTH_6BPC};let mut tr=0;let mut comp=0;if hw.flags.DSC{comp=match hw.pixel_encoding{PIXEL_ENCODING_YCBCR422 if hw.dsc_cfg.ycbcr422_simple=>0,PIXEL_ENCODING_YCBCR422|PIXEL_ENCODING_YCBCR420=>1,_=>0};}else{tr=match pe{DP_PIXEL_ENCODING_TYPE_RGB444|DP_PIXEL_ENCODING_TYPE_YCBCR444=>0,DP_PIXEL_ENCODING_TYPE_YCBCR422=>1,DP_PIXEL_ENCODING_TYPE_Y_ONLY=>3,DP_PIXEL_ENCODING_TYPE_YCBCR420=>2,_=>{ASSERT!(false);0}};}
 REG_UPDATE_4!(e,DP_PIXEL_FORMAT,PIXEL_ENCODING_TYPE,if hw.flags.DSC{1}else{0},UNCOMPRESSED_PIXEL_FORMAT,tr,UNCOMPRESSED_COMPONENT_DEPTH,depth,COMPRESSED_PIXEL_FORMAT,comp);let bpc=match hw.display_color_depth{COLOR_DEPTH_666=>0,COLOR_DEPTH_888=>1,COLOR_DEPTH_101010=>2,COLOR_DEPTH_121212=>3,_=>0};let mut misc0=bpc<<5;match output_color_space{COLOR_SPACE_SRGB=>misc1&=!0x80,COLOR_SPACE_SRGB_LIMITED=>{misc0|=8;misc1&=!0x80},COLOR_SPACE_YCBCR601|COLOR_SPACE_YCBCR601_LIMITED=>{misc0|=8;misc1&=!0x80},COLOR_SPACE_YCBCR709|COLOR_SPACE_YCBCR709_LIMITED=>{misc0|=0x18;misc1&=!0x80},_=>{}};REG_SET!(e,DP_MSA_COLORIMETRY,0,DP_MSA_MISC0,misc0);REG_WRITE!(e,DP_MSA_MISC,misc1);REG_SET_2!(e,DP_MSA_TIMING_PARAM1,0,DP_MSA_HTOTAL,hw.h_total,DP_MSA_VTOTAL,hw.v_total);let hb=hw.h_total-hw.h_border_left-hw.h_addressable-hw.h_border_right;let hp=hb-hw.h_front_porch-hw.h_sync_width;let hs=hw.h_sync_width+hp;let vs=hw.v_total-hw.v_border_top-hw.v_addressable-hw.v_border_bottom-hw.v_front_porch;REG_SET_2!(e,DP_MSA_TIMING_PARAM2,0,DP_MSA_HSTART,hs,DP_MSA_VSTART,vs);REG_SET_4!(e,DP_MSA_TIMING_PARAM3,0,DP_MSA_HSYNCWIDTH,hw.h_sync_width,DP_MSA_HSYNCPOLARITY,!hw.flags.HSYNC_POSITIVE_POLARITY,DP_MSA_VSYNCWIDTH,hw.v_sync_width,DP_MSA_VSYNCPOLARITY,!hw.flags.VSYNC_POSITIVE_POLARITY);REG_SET_2!(e,DP_MSA_TIMING_PARAM4,0,DP_MSA_HWIDTH,hw.h_border_left+hw.h_addressable+hw.h_border_right,DP_MSA_VHEIGHT,hw.v_border_top+hw.v_addressable+hw.v_border_bottom);REG_UPDATE!(e,DP_SEC_FRAMING4,DP_SST_SDP_SPLITTING,enable_sdp_splitting);
}
pub unsafe fn dcn401_dio_stream_encoder_construct(enc1:*mut dcn10_stream_encoder,ctx:*mut dc_context,bp:*mut dc_bios,eng_id:engine_id,vpg:*mut vpg,afmt:*mut afmt,regs:*const dcn10_stream_enc_registers,se_shift:*const dcn10_stream_encoder_shift,se_mask:*const dcn10_stream_encoder_mask){(*enc1).base.ctx=ctx;(*enc1).base.id=eng_id;(*enc1).base.bp=bp;(*enc1).base.vpg=vpg;(*enc1).base.afmt=afmt;(*enc1).regs=regs;(*enc1).se_shift=se_shift;(*enc1).se_mask=se_mask;(*enc1).base.stream_enc_inst=(*vpg).inst;}
pub unsafe fn enc401_set_dynamic_metadata(enc:*mut stream_encoder,enable_dme:bool,hubp_requestor_id:u32,dmdata_mode:dynamic_metadata_mode){let e=DCN10STRENC_FROM_STRENC(enc);if enable_dme{REG_UPDATE_2!(e,DME_CONTROL,METADATA_HUBP_REQUESTOR_ID,hubp_requestor_id,METADATA_STREAM_TYPE,if dmdata_mode==dmdata_dolby_vision{1}else{0});if dmdata_mode==dmdata_dp{REG_UPDATE_3!(e,DP_SEC_METADATA_TRANSMISSION,DP_SEC_METADATA_PACKET_ENABLE,1,DP_SEC_METADATA_PACKET_LINE_REFERENCE,0,DP_SEC_METADATA_PACKET_LINE,20);}else{REG_UPDATE_3!(e,HDMI_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,1,HDMI_METADATA_PACKET_LINE_REFERENCE,0,HDMI_METADATA_PACKET_LINE,2);if dmdata_mode==dmdata_dolby_vision{REG_UPDATE!(e,HDMI_CONTROL,DOLBY_VISION_EN,1);}}REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,1);}else{REG_UPDATE!(e,DME_CONTROL,METADATA_ENGINE_EN,0);if dmdata_mode==dmdata_dp{REG_UPDATE!(e,DP_SEC_METADATA_TRANSMISSION,DP_SEC_METADATA_PACKET_ENABLE,0);}else{REG_UPDATE!(e,HDMI_METADATA_PACKET_CONTROL,HDMI_METADATA_PACKET_ENABLE,0);REG_UPDATE!(e,HDMI_CONTROL,DOLBY_VISION_EN,0);}}}
pub unsafe fn enc401_stream_encoder_set_stream_attribute_helper(enc1:*mut dcn10_stream_encoder,t:*mut dc_crtc_timing){REG_UPDATE!(enc1,HDMI_CONTROL,TMDS_PIXEL_ENCODING,if (*t).pixel_encoding==PIXEL_ENCODING_YCBCR422{1}else{0});REG_UPDATE!(enc1,HDMI_CONTROL,TMDS_COLOR_FORMAT,0);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
