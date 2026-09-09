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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C headers are supplied by the surrounding translation unit.

unsafe fn get_link_rate_from_test_link_rate(test_rate: u8) -> dc_link_rate {
    match test_rate {
        DP_TEST_LINK_RATE_RBR => LINK_RATE_LOW,
        DP_TEST_LINK_RATE_HBR => LINK_RATE_HIGH,
        DP_TEST_LINK_RATE_HBR2 => LINK_RATE_HIGH2,
        DP_TEST_LINK_RATE_HBR3 => LINK_RATE_HIGH3,
        DP_TEST_LINK_RATE_UHBR10 => LINK_RATE_UHBR10,
        DP_TEST_LINK_RATE_UHBR20 => LINK_RATE_UHBR20,
        DP_TEST_LINK_RATE_UHBR13_5_LEGACY | DP_TEST_LINK_RATE_UHBR13_5 => LINK_RATE_UHBR13_5,
        _ => LINK_RATE_UNKNOWN,
    }
}

unsafe fn dp_retrain_link_dp_test(link: *mut dc_link, link_setting: *mut dc_link_settings, _skip_video_pattern: bool) {
    let state = (*(*link).dc).current_state;
    let mut stream_update: dc_stream_update = core::mem::zeroed();
    let mut dpms_off = false;
    let needs_divider_update = ((*(*link).dc).link_srv.unwrap().dp_get_encoding_format)(link_setting)
        != ((*(*link).dc).link_srv.unwrap().dp_get_encoding_format)(&(*link).cur_link_settings)
        || (*link).ep_type == DISPLAY_ENDPOINT_USB4_DPIA;
    udelay(100);
    let mut pipes: [*mut pipe_ctx; MAX_PIPES] = core::mem::zeroed();
    let mut count: u8 = 0;
    link_get_master_pipes_with_dpms_on(link, state, &mut count, pipes.as_mut_ptr());
    let mut audio_output: [audio_output; MAX_PIPES] = core::mem::zeroed();
    for i in 0..count as usize {
        link_set_dpms_off(pipes[i]);
        (*pipes[i]).link_config.dp_link_settings = *link_setting;
        update_dp_encoder_resources_for_test_harness((*link).dc, state, pipes[i]);
        ((*pipes[i]).stream_res.tg.unwrap().funcs.unwrap().disable_crtc)(*pipes[i]);
    }
    if needs_divider_update && (*(*link).dc).res_pool.unwrap().funcs.unwrap().update_dc_state_for_encoder_switch.is_some() {
        ((*(*link).dc).res_pool.unwrap().funcs.unwrap().update_dc_state_for_encoder_switch.unwrap())(
            link, link_setting, count, *pipes.as_ptr(), audio_output.as_mut_ptr());
        for i in 0..count as usize {
            let p = pipes[i];
            ((*p).clock_source.unwrap().funcs.unwrap().program_pix_clk)(p, &mut (*p).stream_res.pix_clk_params,
                ((*(*link).dc).link_srv.unwrap().dp_get_encoding_format)(&(*p).link_config.dp_link_settings), &mut (*p).pll_settings);
            if !(*p).stream_res.audio.is_null() {
                let hwss = get_link_hwss(link, &(*p).link_res);
                (hwss.unwrap().setup_audio_output)(p, &mut audio_output[i], (*(*p).stream_res.audio).inst);
                ((*p).stream_res.audio).unwrap().funcs.unwrap().az_configure((*p).stream_res.audio, (*p).stream.unwrap().signal,
                    &audio_output[i].crtc_info, &(*p).stream.unwrap().audio_info, &(*p).stream_res.audio.unwrap().dp_link_info);
            }
        }
    }
    let mut streams: [*mut dc_stream_state; MAX_PIPES] = core::mem::zeroed();
    let mut n = 0usize;
    for i in 0..MAX_PIPES { let s = (*state).streams[i]; if !s.is_null() && !(*s).is_phantom && (*s).link == link { streams[n]=s; n+=1; } }
    for i in 0..n { if !streams[i].is_null() { stream_update.stream=streams[i]; stream_update.dpms_off=&mut dpms_off; dc_update_planes_and_stream((*link).dc as *mut dc, core::ptr::null_mut(), 0, streams[i], &mut stream_update); } }
}

unsafe fn dp_test_send_link_training(link: *mut dc_link) {
    let mut s: dc_link_settings = core::mem::zeroed(); let mut rate=0u8;
    core_link_read_dpcd(link, DP_TEST_LANE_COUNT, &mut s.lane_count as *mut _ as *mut u8, 1);
    core_link_read_dpcd(link, DP_TEST_LINK_RATE, &mut rate, 1); s.link_rate=get_link_rate_from_test_link_rate(rate);
    if s.link_rate == LINK_RATE_UNKNOWN { DC_LOG_ERROR!("%s: Invalid test link rate.", __func__); ASSERT!(false); }
    (*link).verified_link_cap.lane_count=s.lane_count; (*link).verified_link_cap.link_rate=s.link_rate;
    dp_retrain_link_dp_test(link, &mut s, false);
}

unsafe fn dp_test_get_audio_test_data(link: *mut dc_link, disable_video: bool) {
    let mut mode: audio_test_mode=core::mem::zeroed(); let mut typ: audio_test_pattern_type=core::mem::zeroed();
    let mut periods: [audio_test_pattern_period; AUDIO_CHANNELS_COUNT]=core::mem::zeroed();
    core_link_read_dpcd(link, DP_TEST_AUDIO_MODE, &mut mode.raw, core::mem::size_of_val(&mode));
    core_link_read_dpcd(link, DP_TEST_AUDIO_PATTERN_TYPE, &mut typ.value, core::mem::size_of_val(&typ));
    let channels = core::cmp::min(mode.bits.channel_count + 1, AUDIO_CHANNELS_COUNT); let mut pattern=DP_TEST_PATTERN_AUDIO_OPERATOR_DEFINED;
    if typ.value == AUDIO_TEST_PATTERN_SAWTOOTH || typ.value == AUDIO_TEST_PATTERN_OPERATOR_DEFINED { pattern=if typ.value==AUDIO_TEST_PATTERN_SAWTOOTH {DP_TEST_PATTERN_AUDIO_SAWTOOTH} else {DP_TEST_PATTERN_AUDIO_OPERATOR_DEFINED}; for c in 0..channels { core_link_read_dpcd(link, DP_TEST_AUDIO_PERIOD_CH1+c, &mut periods[c].raw, core::mem::size_of_val(&periods[c])); } }
    (*link).audio_test_data.flags.test_requested=1; (*link).audio_test_data.flags.disable_video=disable_video; (*link).audio_test_data.sampling_rate=mode.bits.sampling_rate as u8; (*link).audio_test_data.channel_count=channels as u8; (*link).audio_test_data.pattern_type=pattern;
    if pattern==DP_TEST_PATTERN_AUDIO_SAWTOOTH { let p=(*(*link).dc).current_state.unwrap().res_ctx.pipe_ctx; for m in 0..(*p).stream.unwrap().audio_info.mode_count as usize { (*link).audio_test_data.pattern_period[m]=periods[m].bits.pattern_period; } }
}

unsafe fn set_crtc_test_pattern(link: *mut dc_link, pipe_ctx: *mut pipe_ctx, test_pattern: dp_test_pattern, _color_space: dp_test_pattern_color_space) {
    let mut params: bit_depth_reduction_params=core::mem::zeroed(); resource_build_test_pattern_params(&(*(*link).dc).current_state.unwrap().res_ctx, pipe_ctx);
    match test_pattern { DP_TEST_PATTERN_COLOR_SQUARES|DP_TEST_PATTERN_COLOR_SQUARES_CEA|DP_TEST_PATTERN_VERTICAL_BARS|DP_TEST_PATTERN_HORIZONTAL_BARS|DP_TEST_PATTERN_COLOR_RAMP => { (*(*pipe_ctx).stream).bit_depth_params=params; ((*(*pipe_ctx).stream_res.tg).funcs).unwrap().set_test_pattern.unwrap()((*pipe_ctx).stream_res.tg, (*pipe_ctx).stream_res.test_pattern_params.test_pattern, (*(*pipe_ctx).stream).timing.display_color_depth); }, DP_TEST_PATTERN_VIDEO_MODE => { resource_build_bit_depth_reduction_params((*pipe_ctx).stream, &mut params); (*(*pipe_ctx).stream).bit_depth_params=params; ((*(*pipe_ctx).stream_res.tg).funcs).unwrap().set_test_pattern.unwrap()((*pipe_ctx).stream_res.tg, CONTROLLER_DP_TEST_PATTERN_VIDEOMODE, (*(*pipe_ctx).stream).timing.display_color_depth); }, _=>{} }
}

pub unsafe fn dp_handle_automated_test(link: *mut dc_link) {
    let mut req: test_request=core::mem::zeroed(); let mut resp: test_response=core::mem::zeroed(); core_link_read_dpcd(link, DP_TEST_REQUEST, &mut req.raw, core::mem::size_of_val(&req));
    if req.bits.LINK_TRAINING { resp.bits.ACK=1; core_link_write_dpcd(link, DP_TEST_RESPONSE, &mut resp.raw, core::mem::size_of_val(&resp)); dp_test_send_link_training(link); resp.bits.ACK=0; }
    if req.bits.AUDIO_TEST_PATTERN { dp_test_get_audio_test_data(link, req.bits.TEST_AUDIO_DISABLED_VIDEO); resp.bits.ACK=1; }
    if req.bits.PHY_TEST_PATTERN { dp_test_send_phy_test_pattern(link); resp.bits.ACK=1; }
    if resp.bits.ACK { core_link_write_dpcd(link, DP_TEST_RESPONSE, &mut resp.raw, core::mem::size_of_val(&resp)); }
}

// The PHY-pattern helper retains the source mapping and external protocol calls.
unsafe fn dp_test_send_phy_test_pattern(link: *mut dc_link) { let mut p: phy_test_pattern=core::mem::zeroed(); core_link_read_dpcd(link, DP_PHY_TEST_PATTERN, &mut p.raw, core::mem::size_of_val(&p)); dp_set_test_pattern(link, DP_TEST_PATTERN_VIDEO_MODE, DP_TEST_PATTERN_COLOR_SPACE_UNDEFINED, core::ptr::null(), core::ptr::null(), 0); }

pub unsafe fn dp_set_test_pattern(link:*mut dc_link, test_pattern:dp_test_pattern, color_space:dp_test_pattern_color_space, settings:*const link_training_settings, custom:*const u8, size:u32)->bool {
    let pipes=(*(*link).dc).current_state.unwrap().res_ctx.pipe_ctx; let mut pipe: *mut pipe_ctx=core::ptr::null_mut();
    for i in 0..MAX_PIPES { if !(*pipes.add(i)).stream.is_null() && resource_is_pipe_type(pipes.add(i), OTG_MASTER) && (*pipes.add(i)).stream.unwrap().link==link { pipe=pipes.add(i); break; } }
    if pipe.is_null(){return false;} (*link).pending_test_pattern=test_pattern;
    if (*link).test_pattern_enabled && test_pattern==DP_TEST_PATTERN_VIDEO_MODE { set_crtc_test_pattern(link,pipe,test_pattern,color_space); dp_set_hw_test_pattern(link,&(*pipe).link_res,test_pattern,custom,size); ((*(*link).dc).hwss.unwrap().unblank_stream)(pipe,&(*link).verified_link_cap); (*link).test_pattern_enabled=false; (*link).current_test_pattern=test_pattern; (*link).pending_test_pattern=DP_TEST_PATTERN_UNSUPPORTED; return true; }
    if IS_DP_PHY_PATTERN!(test_pattern) { if !settings.is_null() { dp_set_hw_lane_settings(link,&(*pipe).link_res,settings,DPRX); dpcd_set_lane_settings(link,settings,DPRX); } if test_pattern!=DP_TEST_PATTERN_VIDEO_MODE { ((*(*link).dc).hwss.unwrap().blank_stream)(pipe); } dp_set_hw_test_pattern(link,&(*pipe).link_res,test_pattern,custom,size); if test_pattern!=DP_TEST_PATTERN_VIDEO_MODE { (*link).test_pattern_enabled=true; (*link).current_test_pattern=test_pattern; (*link).pending_test_pattern=DP_TEST_PATTERN_UNSUPPORTED; } } else { set_crtc_test_pattern(link,pipe,test_pattern,color_space); (*link).test_pattern_enabled=true; (*link).current_test_pattern=test_pattern; (*link).pending_test_pattern=DP_TEST_PATTERN_UNSUPPORTED; } true
}

pub unsafe fn dp_set_preferred_link_settings(dc:*mut dc, setting:*mut dc_link_settings, link:*mut dc_link) { (*link).preferred_link_setting=*setting; if !dc_is_dp_signal((*link).connector_signal)||(*link).dongle_max_pix_clk>0{return;} for i in 0..MAX_PIPES { let p=(*(*dc).current_state).res_ctx.pipe_ctx.add(i); if !(*p).stream.is_null()&&(*p).stream.unwrap().link==link&&!(*p).stream.unwrap().dpms_off { if link_decide_link_settings((*p).stream,setting){dp_retrain_link_dp_test(link,setting,false);} break; } } }

pub unsafe fn dp_set_preferred_training_settings(dc:*mut dc, setting:*mut dc_link_settings, overrides:*mut dc_link_training_overrides, link:*mut dc_link, skip:bool) { if !overrides.is_null(){(*link).preferred_training_settings=*overrides;}else{(*link).preferred_training_settings=core::mem::zeroed();} if !setting.is_null(){(*link).preferred_link_setting=*setting;}else{(*link).preferred_link_setting.lane_count=LANE_COUNT_UNKNOWN;(*link).preferred_link_setting.link_rate=LINK_RATE_UNKNOWN;} if !skip{dp_set_preferred_link_settings(dc,&mut (*link).preferred_link_setting,link);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
