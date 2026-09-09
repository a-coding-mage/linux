// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C headers and build-time dependencies are supplied by the surrounding tree.

const LOW_REFRESH_RATE_DURATION_US_UPPER_BOUND: u32 = 25000;

unsafe fn mod_power_set_replay_active(stream: *mut dc_stream_state, replay_active: bool, wait: bool, force_static: bool) -> bool {
    let mut state: u64 = 0;
    let max_retry: u32 = 1000;
    if stream.is_null() { return false; }
    let link = dc_stream_get_link(stream);
    if link.is_null() { return false; }
    if !dc_link_set_replay_allow_active(link, &mut replay_active as *mut bool, false, force_static, core::ptr::null_mut()) { return false; }
    if wait {
        let mut retry_count = 0;
        while retry_count <= max_retry {
            dc_link_get_replay_state(link, &mut state);
            if replay_active {
                if state != REPLAY_STATE_0 && (!force_static || state == REPLAY_STATE_3) { break; }
            } else if state == REPLAY_STATE_0 { break; }
            udelay(500);
            retry_count += 1;
        }
        if retry_count >= max_retry { ASSERT(0); }
    }
    true
}

unsafe fn mod_power_replay_setup_power_opt(link: *mut dc_link, active_replay_events: u32, is_ultra_sleep_mode: bool) -> u32 {
    let mut power_opt = 0;
    if is_ultra_sleep_mode { power_opt |= replay_power_opt_smu_opt_static_screen | replay_power_opt_z10_static_screen; }
    else if active_replay_events & replay_event_test_harness_ultra_sleep != 0 { power_opt |= replay_power_opt_z10_static_screen; }
    power_opt & (*link).replay_settings.config.replay_power_opt_supported
}

unsafe fn mod_power_replay_set_power_opt(_mod_power: *mut mod_power, stream: *mut dc_stream_state, events: u32, ultra: bool) -> bool {
    if stream.is_null() { return false; }
    let link = dc_stream_get_link(stream);
    if link.is_null() || !(*link).replay_settings.replay_feature_enabled { return false; }
    let mut opt = mod_power_replay_setup_power_opt(link, events, ultra);
    dc_link_set_replay_allow_active(link, core::ptr::null_mut(), false, false, &mut opt)
}

pub unsafe fn mod_power_get_replay_event(mp: *mut mod_power, stream: *mut dc_stream_state, active: *mut u32) -> bool {
    if mp.is_null() { return false; }
    let cp = MOD_POWER_TO_CORE(mp);
    if (*cp).num_entities == 0 { return false; }
    let i = map_index_from_stream(cp, stream);
    *active = (*cp).map[i].replay_events;
    true
}

unsafe fn mod_power_update_replay_active_status(events: u32, link: *mut dc_link, coasting: *mut u32, full_video: *mut bool, ultra: *mut bool, skip: *mut u16, playback: *mut bool) -> bool {
    if link.is_null() || coasting.is_null() || full_video.is_null() || playback.is_null() { return false; }
    if (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_STATIC] == 0 || (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_NOM] == 0 { return false; }
    let option = (*link).replay_settings.config.replay_enable_option;
    *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_NOM];
    ASSERT((*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_NOM] <= 0xffff);
    *skip = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_NOM] as u16;
    (*link).replay_settings.config.replay_timing_sync_supported = false;
    *full_video = false; *ultra = false; *playback = false;
    if events & replay_event_test_harness_mode != 0 {
        if (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_TEST_HARNESS] != 0 { *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_TEST_HARNESS]; }
        ASSERT((*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_TEST_HARNESS] <= 0xffff);
        *skip = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_TEST_HARNESS] as u16;
        if events & replay_event_test_harness_enable_replay != 0 {
            if events & replay_event_test_harness_ultra_sleep != 0 && !(*link).replay_settings.config.replay_support_fast_resync_in_ultra_sleep_mode { (*link).replay_settings.config.replay_timing_sync_supported = false; }
            return true;
        }
        return false;
    } else if events & replay_event_test_harness_enable_replay != 0 {
        if (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_TEST_HARNESS] != 0 { *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_TEST_HARNESS]; }
        let n = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_TEST_HARNESS];
        if n != 0 { ASSERT(n <= 0xffff); *skip = n as u16; }
        if events & replay_event_test_harness_ultra_sleep != 0 && !(*link).replay_settings.config.replay_support_fast_resync_in_ultra_sleep_mode { (*link).replay_settings.config.replay_timing_sync_supported = false; }
        return true;
    } else if events & (replay_event_test_harness_disable_replay | replay_event_os_request_disable) != 0 { return false; }
    if events & (replay_event_edp_panel_off_disable_psr | replay_event_hw_programming | replay_event_vrr | replay_event_immediate_flip | replay_event_prepare_vtotal | replay_event_vrr_transition | replay_event_pause | replay_event_disable_replay_while_DPMS | replay_event_sleep_resume | replay_event_disable_in_AC | replay_event_disable_replay_while_detect_display | replay_event_infopacket | replay_event_crc_window_active) != 0 { return false; }
    if events & replay_event_full_screen != 0 && option & pr_enable_option_full_screen == 0 { return false; }
    if events & replay_event_big_screen_video != 0 {
        (*link).replay_settings.config.replay_timing_sync_supported = false;
        if option & pr_enable_option_full_screen_video_coasting != 0 { let n = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_FULL_SCREEN_VIDEO]; *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_FULL_SCREEN_VIDEO]; ASSERT(n <= 0xffff); *skip = n as u16; }
        *playback = true; *full_video = option & pr_enable_option_full_screen_video != 0 && option & pr_enable_option_full_screen_video_coasting != 0; return *full_video;
    }
    if events & replay_event_mpo_video_selective_update != 0 && events & replay_event_full_screen == 0 {
        (*link).replay_settings.config.replay_timing_sync_supported = false; *playback = true;
        if option & pr_enable_option_mpo_video_coasting != 0 { *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_NOM]; let n = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_NOM]; ASSERT(n <= 0xffff); *skip = n as u16; }
        return option & pr_enable_option_mpo_video != 0;
    }
    if events & replay_event_vsync == 0 {
        if option & pr_enable_option_static_screen_coasting != 0 {
            if (*link).replay_settings.config.replay_power_opt_supported & replay_power_opt_z10_static_screen == 0 || events & replay_event_cursor_updating != 0 { *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_NOM]; let n = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_NOM]; ASSERT(n <= 0xffff); *skip = n as u16; }
            else { *coasting = (*link).replay_settings.coasting_vtotal_table[PR_COASTING_TYPE_STATIC]; let n = (*link).replay_settings.frame_skip_number_table[PR_COASTING_TYPE_STATIC]; ASSERT(n <= 0xffff); *skip = n as u16; *ultra = true; }
        }
        if option & pr_enable_option_static_screen != 0 { if !(*link).replay_settings.config.replay_support_fast_resync_in_ultra_sleep_mode { (*link).replay_settings.config.replay_timing_sync_supported = false; } return true; }
        return false;
    }
    if events & replay_event_general_ui != 0 { return option & pr_enable_option_general_ui != 0; }
    false
}

pub unsafe fn mod_power_replay_set_coasting_vtotal(mp: *mut mod_power, stream: *const dc_stream_state, vtotal: u32, skip: u16) -> bool { if stream.is_null() || mp.is_null() { return false; } let link = dc_stream_get_link(stream); if link.is_null() || !(*link).replay_settings.replay_feature_enabled { return false; } let cp = MOD_POWER_TO_CORE(mp); if (*cp).num_entities == 0 { return false; } (*link).dc.link_srv.edp_set_coasting_vtotal(link, vtotal, skip) }

unsafe fn mod_power_replay_set_general_cmd(mp: *mut mod_power, stream: *const dc_stream_state, typ: dmub_cmd_replay_general_subtype, p1: u32, p2: u32) { if stream.is_null() || mp.is_null() { return; } let cp = MOD_POWER_TO_CORE(mp); if (*cp).num_entities == 0 { return; } if map_index_from_stream(cp, stream) > (*cp).num_entities { return; } let link = dc_stream_get_link(stream); if link.is_null() || !(*link).replay_settings.replay_feature_enabled { return; } let mut cmd: dmub_replay_cmd_set = core::mem::zeroed(); cmd.set_general_cmd_data.subtype = typ; cmd.set_general_cmd_data.param1 = p1; cmd.set_general_cmd_data.param2 = p2; (*link).dc.link_srv.edp_send_replay_cmd(link, Replay_Set_General_Cmd, &mut cmd); }

pub unsafe fn mod_power_replay_disabled_desync_error_detection(mp: *mut mod_power, stream: *const dc_stream_state, disabled: bool) { mod_power_replay_set_general_cmd(mp, stream, REPLAY_GENERAL_CMD_DISABLED_DESYNC_ERROR_DETECTION, disabled as u32, 0); }
pub unsafe fn mod_power_set_low_rr_activate(mp: *mut mod_power, stream: *const dc_stream_state, supported: bool) { if mp.is_null() || stream.is_null() { return; } if !dc_stream_get_link(stream).is_null() { mod_power_replay_set_general_cmd(mp, stream, REPLAY_GENERAL_CMD_SET_LOW_RR_ACTIVATE, supported as u32, 0); } }
pub unsafe fn mod_power_set_video_conferencing_activate(mp: *mut mod_power, stream: *const dc_stream_state, active: bool) { let l=dc_stream_get_link(stream); if mp.is_null()||stream.is_null()||l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;} mod_power_replay_set_general_cmd(mp,stream,REPLAY_GENERAL_CMD_VIDEO_CONFERENCING,active as u32,0); }
pub unsafe fn mod_power_set_coasting_vtotal_without_frame_update(mp:*mut mod_power,s:*const dc_stream_state,v:u32){let l=dc_stream_get_link(s);if mp.is_null()||s.is_null()||l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;}mod_power_replay_set_general_cmd(mp,s,REPLAY_GENERAL_CMD_SET_COASTING_VTOTAL_WITHOUT_FRAME_UPDATE,v,0);}
pub unsafe fn mod_power_set_replay_continuously_resync(mp:*mut mod_power,s:*const dc_stream_state,e:bool){let l=dc_stream_get_link(s);if mp.is_null()||s.is_null()||l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;}mod_power_replay_set_general_cmd(mp,s,REPLAY_GENERAL_CMD_SET_CONTINUOUSLY_RESYNC,e as u32,0);}

pub unsafe fn init_replay_config(link:*mut dc_link,c:*const replay_config){(*link).replay_settings.config=*c;}
pub unsafe fn set_replay_frame_skip_number(link:*mut dc_link,typ:replay_coasting_vtotal_type,coasting:u32,flicker:u32,defer:bool){if link.is_null()||!(*link).replay_settings.config.frame_skip_supported||flicker==0||coasting==0{return;}let a=if defer{(*link).replay_settings.defer_frame_skip_number_table.as_mut_ptr()}else{(*link).replay_settings.frame_skip_number_table.as_mut_ptr()};let n=(coasting+500000)/flicker;*a.add(typ as usize)=n.saturating_sub(1);}
pub unsafe fn set_replay_defer_update_coasting_vtotal(l:*mut dc_link,t:replay_coasting_vtotal_type,v:u32){(*l).replay_settings.defer_update_coasting_vtotal_table[t as usize]=v;}
pub unsafe fn update_replay_coasting_vtotal_from_defer(l:*mut dc_link,t:replay_coasting_vtotal_type){(*l).replay_settings.coasting_vtotal_table[t as usize]=(*l).replay_settings.defer_update_coasting_vtotal_table[t as usize];(*l).replay_settings.frame_skip_number_table[t as usize]=(*l).replay_settings.defer_frame_skip_number_table[t as usize];}
pub unsafe fn set_replay_coasting_vtotal(l:*mut dc_link,t:replay_coasting_vtotal_type,v:u32){(*l).replay_settings.coasting_vtotal_table[t as usize]=v;}
pub unsafe fn set_replay_low_rr_full_screen_video_src_vtotal(l:*mut dc_link,v:u16){(*l).replay_settings.low_rr_full_screen_video_pseudo_vtotal=v;}
pub unsafe fn calculate_replay_link_off_frame_count(l:*mut dc_link,v:u16,h:u16){if l.is_null()||(*l).replay_settings.config.replay_version!=DC_FREESYNC_REPLAY{return;}let d=(*l).dpcd_caps.pr_info.max_deviation_line;let p=(*l).dpcd_caps.pr_info.pixel_deviation_per_line;let n=if h!=0&&v!=0&&p!=0{h as u32*d as u32/(p as u32*v as u32)}else{ASSERT(0);0};(*l).replay_settings.link_off_frame_count=n;}
pub unsafe fn reset_replay_dsync_error_count(l:*mut dc_link){(*l).replay_settings.replay_desync_error_fail_count=0;}

pub unsafe fn mod_power_replay_set_timing_sync_supported(mp:*mut mod_power,s:*const dc_stream_state){if mp.is_null()||s.is_null(){return;}let cp=MOD_POWER_TO_CORE(mp);if (*cp).num_entities==0{return;}let i=map_index_from_stream(cp,s);if i>(*cp).num_entities{return;}let l=dc_stream_get_link(s);if l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;}let mut c:dmub_replay_cmd_set=core::mem::zeroed();c.sync_data.timing_sync_supported=(*l).replay_settings.config.replay_timing_sync_supported;(*l).dc.link_srv.edp_send_replay_cmd(l,Replay_Set_Timing_Sync_Supported,&mut c);}
pub unsafe fn mod_power_replay_disabled_adaptive_sync_sdp(mp:*mut mod_power,s:*const dc_stream_state,f:bool){if mp.is_null()||s.is_null(){return;}let cp=MOD_POWER_TO_CORE(mp);if (*cp).num_entities==0{return;}let i=map_index_from_stream(cp,s);if i>(*cp).num_entities{return;}let l=dc_stream_get_link(s);if l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;}let mut c:dmub_replay_cmd_set=core::mem::zeroed();c.disabled_adaptive_sync_sdp_data.force_disabled=f;(*l).dc.link_srv.edp_send_replay_cmd(l,Replay_Disabled_Adaptive_Sync_SDP,&mut c);}
pub unsafe fn mod_power_replay_set_pseudo_vtotal(mp:*mut mod_power,s:*const dc_stream_state,v:u16){if mp.is_null()||s.is_null(){return;}let cp=MOD_POWER_TO_CORE(mp);if (*cp).num_entities==0{return;}let i=map_index_from_stream(cp,s);if i>(*cp).num_entities{return;}let l=dc_stream_get_link(s);if l.is_null()||!(*l).replay_settings.replay_feature_enabled{return;}if (*l).replay_settings.last_pseudo_vtotal!=v{(*l).replay_settings.last_pseudo_vtotal=v;let mut c:dmub_replay_cmd_set=core::mem::zeroed();c.pseudo_vtotal_data.vtotal=v;(*l).dc.link_srv.edp_send_replay_cmd(l,Replay_Set_Pseudo_VTotal,&mut c);}}
pub unsafe fn mod_power_replay_residency(s:*const dc_stream_state,r:*mut u32,start:bool,alpm:bool){if s.is_null(){return;}let l=dc_stream_get_link(s);let m=if alpm{PR_RESIDENCY_MODE_ALPM}else{PR_RESIDENCY_MODE_PHY};if !l.is_null()&&!(*l).dc.is_null()&&!(*l).dc.link_srv.is_null(){(*l).dc.link_srv.edp_replay_residency(l,r,start,m);}}
pub unsafe fn mod_power_replay_notify_mode_change(mp:*mut mod_power,dc:*mut dc,link:*mut dc_link,s:*const dc_stream_state,i:u32){if mp.is_null()||dc.is_null()||link.is_null()||s.is_null(){return;}let cp=MOD_POWER_TO_CORE(mp);let e=(*cp).map[i as usize].replay_events;if e&replay_event_os_override_hold!=0{return;}(*link).replay_settings.replay_smu_opt_enable=(*link).replay_settings.config.replay_smu_opt_supported&&mod_power_only_edp((*dc).current_state,s);if e&replay_event_os_request_force_ffu!=0{(*link).replay_settings.config.os_request_force_ffu=true;}if dc_is_embedded_signal((*s).signal){(*dc).link_srv.dp_setup_replay(link,s);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
