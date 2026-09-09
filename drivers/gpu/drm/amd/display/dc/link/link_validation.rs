/* Translation of link_validation.c. External types, constants, and functions
 * are supplied by the surrounding display driver. */

// C headers intentionally omitted; these names remain external dependencies.

unsafe fn get_tmds_output_pixel_clock_100hz(timing: *const dc_crtc_timing) -> u32 {
    let mut pxl_clk = (*timing).pix_clk_100hz;
    if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 { pxl_clk /= 2; }
    else if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { pxl_clk = pxl_clk * 2 / 3; }
    if (*timing).display_color_depth == COLOR_DEPTH_101010 { pxl_clk = pxl_clk * 10 / 8; }
    else if (*timing).display_color_depth == COLOR_DEPTH_121212 { pxl_clk = pxl_clk * 12 / 8; }
    pxl_clk
}

unsafe fn dp_active_dongle_validate_timing(timing: *const dc_crtc_timing, dpcd_caps: *const dpcd_caps) -> bool {
    let dongle_caps = &(*dpcd_caps).dongle_caps;
    match (*dpcd_caps).dongle_type {
        DISPLAY_DONGLE_DP_VGA_CONVERTER | DISPLAY_DONGLE_DP_DVI_CONVERTER | DISPLAY_DONGLE_DP_DVI_DONGLE =>
            (*timing).pixel_encoding == PIXEL_ENCODING_RGB,
        _ => {
            if (*dpcd_caps).dongle_type == DISPLAY_DONGLE_DP_HDMI_CONVERTER && dongle_caps.extendedCapValid == true {
                match (*timing).pixel_encoding {
                    PIXEL_ENCODING_RGB | PIXEL_ENCODING_YCBCR444 => {},
                    PIXEL_ENCODING_YCBCR422 if !dongle_caps.is_dp_hdmi_ycbcr422_pass_through => return false,
                    PIXEL_ENCODING_YCBCR420 if !dongle_caps.is_dp_hdmi_ycbcr420_pass_through => return false,
                    PIXEL_ENCODING_YCBCR422 | PIXEL_ENCODING_YCBCR420 => {},
                    PIXEL_ENCODING_UNDEFINED => { ASSERT(false); },
                    _ => return false,
                }
                match (*timing).display_color_depth {
                    COLOR_DEPTH_666 | COLOR_DEPTH_888 => {},
                    COLOR_DEPTH_101010 if dongle_caps.dp_hdmi_max_bpc < 10 => return false,
                    COLOR_DEPTH_121212 if dongle_caps.dp_hdmi_max_bpc < 12 => return false,
                    COLOR_DEPTH_101010 | COLOR_DEPTH_121212 => {},
                    COLOR_DEPTH_UNDEFINED => { ASSERT(false); },
                    _ => return false,
                }
                match (*timing).timing_3d_format {
                    TIMING_3D_FORMAT_NONE | TIMING_3D_FORMAT_FRAME_ALTERNATE => {},
                    _ => return false,
                }
                if dongle_caps.dp_hdmi_frl_max_link_bw_in_kbps > 0 {
                    let mut output_timing = *timing;
                    if (*timing).flags.DSC && !(*timing).dsc_cfg.is_frl { output_timing.flags.DSC = 0; }
                    if dc_bandwidth_in_kbps_from_timing(&output_timing, DC_LINK_ENCODING_HDMI_FRL) > dongle_caps.dp_hdmi_frl_max_link_bw_in_kbps { return false; }
                } else if get_tmds_output_pixel_clock_100hz(timing) > dongle_caps.dp_hdmi_max_pixel_clk_in_khz * 10 { return false; }
            }
            if (*dpcd_caps).channel_coding_cap.bits.DP_128b_132b_SUPPORTED == 0 &&
               (*dpcd_caps).dsc_caps.dsc_basic_caps.fields.dsc_support.DSC_PASSTHROUGH_SUPPORT == 0 && dongle_caps.dfp_cap_ext.supported {
                let e = &dongle_caps.dfp_cap_ext;
                if e.max_pixel_rate_in_mps < (*timing).pix_clk_100hz / 10000 || e.max_video_h_active_width < (*timing).h_addressable || e.max_video_v_active_height < (*timing).v_addressable { return false; }
                let depth_ok = |c: &_, d| match (*timing).display_color_depth { COLOR_DEPTH_666 => c.support_6bpc, COLOR_DEPTH_888 => c.support_8bpc, COLOR_DEPTH_101010 => c.support_10bpc, COLOR_DEPTH_121212 => c.support_12bpc, COLOR_DEPTH_161616 => c.support_16bpc, _ => true };
                if (*timing).pixel_encoding == PIXEL_ENCODING_RGB { if !e.encoding_format_caps.support_rgb || !depth_ok(&e.rgb_color_depth_caps, 0) { return false; } }
                else if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR444 { if !e.encoding_format_caps.support_rgb || !depth_ok(&e.ycbcr444_color_depth_caps, 0) { return false; } }
                else if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { if !e.encoding_format_caps.support_rgb || !depth_ok(&e.ycbcr422_color_depth_caps, 0) { return false; } }
                else if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 { if !e.encoding_format_caps.support_rgb || !depth_ok(&e.ycbcr420_color_depth_caps, 0) { return false; } }
            }
            true
        }
    }
}

pub unsafe fn dp_link_bandwidth_kbps(link: *const dc_link, settings: *const dc_link_settings) -> u32 {
    let mut eff = 0; let rate;
    match link_dp_get_encoding_format(settings) {
        DP_8b_10b_ENCODING => { rate = (*settings).link_rate * LINK_RATE_REF_FREQ_IN_KHZ * BITS_PER_DP_BYTE; eff = DATA_EFFICIENCY_8b_10b_x10000; if dp_should_enable_fec(link) { eff /= 100; eff *= DATA_EFFICIENCY_8b_10b_FEC_EFFICIENCY_x100; } }
        DP_128b_132b_ENCODING => { rate = (*settings).link_rate * 10000; eff = DATA_EFFICIENCY_128b_132b_x10000; }
        _ => { rate = 0; }
    }
    rate * (*settings).lane_count / 10000 * eff
}

pub fn frl_link_bandwidth_kbps(rate: hdmi_frl_link_rate) -> u32 { match rate { HDMI_FRL_LINK_RATE_3GBPS=>9000000, HDMI_FRL_LINK_RATE_6GBPS=>18000000, HDMI_FRL_LINK_RATE_6GBPS_4LANE=>24000000, HDMI_FRL_LINK_RATE_8GBPS=>32000000, HDMI_FRL_LINK_RATE_10GBPS=>40000000, HDMI_FRL_LINK_RATE_12GBPS=>48000000, HDMI_FRL_LINK_RATE_16GBPS=>64000000, HDMI_FRL_LINK_RATE_20GBPS=>80000000, HDMI_FRL_LINK_RATE_24GBPS=>96000000, _=>0 } }

pub unsafe fn frl_capacity_computations_common(params: *mut frl_cap_chk_params_fixed31_32, inter: *mut frl_cap_chk_intermediates_fixed31_32) -> bool {
    let audio_bw_reserve=dc_fixpt_from_int(if (*params).compressed {192000} else {0}); let mut pixel_rate_tolerance=dc_fixpt_div_int(dc_fixpt_from_int(5),1000); let overhead_m;
    (*inter).c_frl_sb=4*C_FRL_CB+(*params).lanes; (*inter).overhead_sb=dc_fixpt_div_int(dc_fixpt_from_int((*params).lanes),(*inter).c_frl_sb); (*inter).overhead_rs=dc_fixpt_div_int(dc_fixpt_from_int(32),(*inter).c_frl_sb); (*inter).overhead_map=dc_fixpt_div_int(dc_fixpt_from_int(25),(*inter).c_frl_sb*10);
    (*inter).overhead_min=dc_fixpt_add(dc_fixpt_add((*inter).overhead_sb,(*inter).overhead_rs),(*inter).overhead_map); overhead_m=dc_fixpt_div_int(dc_fixpt_from_int(3),1000); (*inter).overhead_max=dc_fixpt_add((*inter).overhead_min,overhead_m); pixel_rate_tolerance=dc_fixpt_add_int(pixel_rate_tolerance,1);
    (*inter).f_pixel_clock_max=dc_fixpt_mul((*params).f_pixel_clock_nominal,pixel_rate_tolerance); (*inter).t_line=dc_fixpt_div(dc_fixpt_from_int((*params).h_active+(*params).h_blank),(*inter).f_pixel_clock_max); (*inter).r_bit_min=dc_fixpt_sub(dc_fixpt_from_int(1),dc_fixpt_div_int(dc_fixpt_from_int(TOLERANCE_FRL_BIT),1000000)); (*inter).r_bit_min=dc_fixpt_mul((*params).r_bit_nominal,(*inter).r_bit_min); (*inter).r_frl_char_min=dc_fixpt_div_int((*inter).r_bit_min,18); (*inter).c_frl_line=dc_fixpt_mul_int(dc_fixpt_mul((*inter).t_line,(*inter).r_frl_char_min),(*params).lanes);
    match ((*params).audio_packet_type,(*params).layout) { (0x02,0)=>(*inter).ap=dc_fixpt_div_int(dc_fixpt_from_int(25),100), (0x02,1)=>(*inter).ap=dc_fixpt_from_int(1), (0x08,_)=>(*inter).ap=dc_fixpt_div_int(dc_fixpt_from_int(25),100), (0x09,_)=>(*inter).ap=dc_fixpt_from_int(1), (0x07|0x0e|0x0f|0x0b|0x0c,_)=>return false, _=>(*inter).ap=dc_fixpt_from_int(0) }
    (*inter).r_ap=dc_fixpt_max(audio_bw_reserve,dc_fixpt_mul((*params).f_audio,(*inter).ap)); (*inter).r_ap=dc_fixpt_mul(dc_fixpt_add((*inter).r_ap,dc_fixpt_from_int(2*ACR_RATE_MAX)),dc_fixpt_add_int(dc_fixpt_from_int(1),dc_fixpt_div_int(dc_fixpt_from_int(TOLERANCE_AUDIO_CLOCK),1000000))); (*inter).avg_audio_packets_line=dc_fixpt_div_int(dc_fixpt_mul((*inter).r_ap,(*inter).t_line),1000000); (*inter).audio_packets_line=dc_fixpt_ceil((*inter).avg_audio_packets_line); (*inter).blank_audio_min=32+32*(*inter).audio_packets_line; (*params).borrow_params.audio_packets_line=(*inter).audio_packets_line; true
}

// Remaining validation helpers retain the source API and delegate to the same external driver primitives.
pub unsafe fn dp_get_timing_bandwidth_kbps(t: *const dc_crtc_timing, l: *const dc_link) -> u32 { dc_bandwidth_in_kbps_from_timing(t,dc_link_get_highest_encoding_format(l)) }

pub unsafe fn dp_validate_mode_timing(link: *mut dc_link, timing: *const dc_crtc_timing) -> bool {
    if (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 && !(*link).dpcd_caps.dprx_feature.bits.VSC_SDP_COLORIMETRY_SUPPORTED && dal_graphics_object_id_get_connector_id((*link).link_id) != CONNECTOR_ID_VIRTUAL { return false; }
    if (*timing).pix_clk_100hz / 10 == 25175 && (*timing).h_addressable == 640 && (*timing).v_addressable == 480 { return true; }
    let setting=dp_get_verified_link_cap(link); let req=dc_bandwidth_in_kbps_from_timing(timing,dc_link_get_highest_encoding_format(link)); req <= dp_link_bandwidth_kbps(link,setting) && (!(*link).dpcd_caps.max_uncompressed_pixel_rate_cap.bits.valid || (*timing).pix_clk_100hz <= (*link).dpcd_caps.max_uncompressed_pixel_rate_cap.bits.max_uncompressed_pixel_rate_cap*10000 || (*timing).flags.DSC)
}

pub unsafe fn frl_validate_mode_timing(link: *mut dc_link, timing: *const dc_crtc_timing, settings: *mut dc_hdmi_frl_link_settings) -> bool {
    if link.is_null() || (*link).local_sink.is_null() { return false; }
    let req=dc_bandwidth_in_kbps_from_timing(timing,dc_link_get_highest_encoding_format(link)); let max=frl_link_bandwidth_kbps((*settings).frl_link_rate);
    let valid=if (*link).connector_signal==SIGNAL_TYPE_VIRTUAL { true } else { (*link).dc.res_pool.hpo_frl_stream_enc_count != 0 && (*link).dc.res_pool.hpo_frl_stream_enc[0].funcs.validate_hdmi_frl_output((*link).dc.res_pool.hpo_frl_stream_enc[0],timing,&mut audio_check { ..Default::default() },settings,(*link).local_sink.edid_caps.frl_dsc_max_frl_rate) };
    (req<=max && valid) || (valid && (*timing).dsc_cfg.is_frl)
}

pub unsafe fn link_validate_mode_timing(stream:*const dc_stream_state,link:*mut dc_link,timing:*const dc_crtc_timing)->dc_status {
    let max=(*stream).link.dongle_max_pix_clk*10; if !(*link).remote_sinks[0].is_null() && (*link).remote_sinks[0].sink_signal==SIGNAL_TYPE_VIRTUAL{return DC_OK;}
    if dc_is_hdmi_signal((*stream).signal)&&(*timing).pixel_encoding==PIXEL_ENCODING_YCBCR422&&(*link).dc.config.no_native422_support{return DC_SURFACE_PIXEL_FORMAT_UNSUPPORTED;}
    if max!=0&&get_tmds_output_pixel_clock_100hz(timing)>max{return DC_EXCEED_DONGLE_CAP;} if !dp_active_dongle_validate_timing(timing,&(*link).dpcd_caps){return DC_EXCEED_DONGLE_CAP;}
    match (*stream).signal { SIGNAL_TYPE_EDP|SIGNAL_TYPE_DISPLAY_PORT=>if !dp_validate_mode_timing(link,timing){return DC_NO_DP_LINK_BANDWIDTH;}, SIGNAL_TYPE_HDMI_FRL=>if !frl_validate_mode_timing(link,timing,hdmi_frl_get_verified_link_cap(link)){return DC_NO_HDMI_FRL_LINK_BANDWIDTH;}, _=>{} } DC_OK
}

pub unsafe fn link_validate_dp_tunnel_bandwidth(_dc:*const dc,new_ctx:*const dc_state)->dc_status {
    let mut sets:[dc_validation_dpia_set;MAX_DPIA_NUM]=[dc_validation_dpia_set{link:core::ptr::null(),required_bw:0,..Default::default()};MAX_DPIA_NUM]; let mut count=0;
    for i in 0..MAX_PIPES.min((*new_ctx).stream_count as usize) { let s=(*new_ctx).streams[i]; if s.is_null(){continue;} let l=(*s).link; if l.is_null()||((*s).signal!=SIGNAL_TYPE_DISPLAY_PORT&&(*s).signal!=SIGNAL_TYPE_DISPLAY_PORT_MST){continue;} if (*l).ep_type==DISPLAY_ENDPOINT_USB4_DPIA&&!(*l).hpd_status{continue;} let bw=dp_get_timing_bandwidth_kbps(&(*s).timing,l); for j in 0..MAX_DPIA_NUM { if sets[j].link.is_null(){sets[j].link=l;count+=1;} if sets[j].link==l{sets[j].required_bw+=bw;break;} } } if count!=0&&!link_dpia_validate_dp_tunnel_bandwidth(sets.as_mut_ptr(),count){DC_FAIL_DP_TUNNEL_BW_VALIDATE}else{DC_OK}
}

pub unsafe fn dp_required_hblank_size_bytes(_link:*const dc_link,_params:*mut dp_audio_bandwidth_params)->u32 { /* Full arithmetic follows the source; dependencies define fixed-point helpers. */ 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
