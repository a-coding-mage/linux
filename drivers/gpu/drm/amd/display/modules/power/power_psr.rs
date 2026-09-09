// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit.

#[inline]
unsafe fn calc_psr_num_static_frames(vsync_rate_hz: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut num_frames_static: ::core::ffi::c_uint = 2;
    if vsync_rate_hz != 0 {
        num_frames_static = (30000u32.wrapping_mul(vsync_rate_hz).wrapping_add(1000000 - 1)) / 1000000;
    }
    num_frames_static
}

pub unsafe fn mod_power_psr_notify_mode_change(
    mod_power: *mut mod_power,
    stream: *const dc_stream_state,
    link: *mut dc_link,
    stream_index: ::core::ffi::c_uint,
) -> bool {
    if mod_power.is_null() || stream.is_null() || link.is_null() { return false; }
    let core_power = MOD_POWER_TO_CORE(mod_power);
    let dc = (*core_power).dc;
    let mut psr_config: psr_config = ::core::mem::zeroed();
    let mut psr_context: psr_context = ::core::mem::zeroed();
    let active_psr_events = (*core_power).map[stream_index as usize].psr_events;
    if active_psr_events & psr_event_os_override_hold != 0 { return false; }
    mod_power_calc_psr_configs(&mut psr_config, link, stream);
    psr_config.psr_exit_link_training_required = (*core_power).map[stream_index as usize].caps.psr_exit_link_training_required;
    if (*(*dc).ctx).asic_id.chip_family >= AMDGPU_FAMILY_GC_11_0_1 {
        psr_config.allow_smu_optimizations = (*core_power).psr_smu_optimizations_support && dc_is_embedded_signal((*stream).signal);
    } else {
        psr_config.allow_smu_optimizations = (*core_power).psr_smu_optimizations_support && mod_power_only_edp((*dc).current_state, stream);
    }
    psr_config.allow_multi_disp_optimizations = (*core_power).multi_disp_optimizations_support;
    psr_config.rate_control_caps = (*core_power).map[stream_index as usize].caps.rate_control_caps;
    if active_psr_events & psr_event_os_request_force_ffu != 0 { psr_config.os_request_force_ffu = true; }
    psr_su_set_dsc_slice_height(dc, link, stream as *mut dc_stream_state, &mut psr_config);
    dc_link_setup_psr(link, stream, &mut psr_config, &mut psr_context);
    true
}

unsafe fn mod_power_psr_set_power_opt(mod_power: *mut mod_power, stream: *mut dc_stream_state, active_psr_events: ::core::ffi::c_uint, _psr_enable_request: bool) {
    if stream.is_null() { return; }
    let core_power = MOD_POWER_TO_CORE(mod_power);
    let stream_index = map_index_from_stream(core_power, stream);
    if (*core_power).map[stream_index as usize].caps.psr_version == 0 { return; }
    let link = dc_stream_get_link(stream);
    let mut power_opt = 0;
    if active_psr_events == 0 { power_opt |= psr_power_opt_smu_opt_static_screen | psr_power_opt_z10_static_screen | psr_power_opt_ds_disable_allow; }
    power_opt &= (*core_power).map[stream_index as usize].caps.psr_power_opt_flag;
    if (*core_power).map[stream_index as usize].psr_power_opt != power_opt {
        dc_link_set_psr_allow_active(link, ::core::ptr::null_mut(), false, false, &mut power_opt);
        (*core_power).map[stream_index as usize].psr_power_opt = power_opt;
    }
}

unsafe fn set_psr_enable(mod_power: *mut mod_power, stream: *mut dc_stream_state, psr_enable: bool, wait: bool, force_static: bool) -> bool {
    if mod_power.is_null() { return false; }
    let core_power = MOD_POWER_TO_CORE(mod_power);
    if (*core_power).num_entities == 0 { return false; }
    if psr_enable {
        let vsync_rate_hz = (((*stream).timing.pix_clk_100hz as u64 * 100) / (*stream).timing.v_total as u64 / (*stream).timing.h_total as u64) as u32;
        let mut params: dc_static_screen_params = ::core::mem::zeroed();
        params.triggers.cursor_update = true; params.triggers.overlay_update = true; params.triggers.surface_update = true;
        params.num_frames = calc_psr_num_static_frames(vsync_rate_hz);
        dc_stream_set_static_screen_params((*core_power).dc, &mut (stream as *const dc_stream_state), 1, &mut params);
    }
    let link = dc_stream_get_link(stream);
    if !dc_link_set_psr_allow_active(link, &mut (psr_enable as bool), false, force_static, ::core::ptr::null_mut()) { return false; }
    if wait {
        let mut state = PSR_STATE0;
        let max_retry = 1000;
        let mut retry_count = 0;
        while retry_count <= max_retry {
            dc_link_get_psr_state(link, &mut state);
            if psr_enable { if state != PSR_STATE0 && (!force_static || state == PSR_STATE3) { break; } } else if state == PSR_STATE0 { break; }
            udelay(500); retry_count += 1;
        }
        if retry_count >= max_retry { ASSERT(0); }
    }
    true
}

pub unsafe fn mod_power_get_psr_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, active_psr_events: *mut ::core::ffi::c_uint) -> bool {
    if mod_power.is_null() { return false; }
    let core_power = MOD_POWER_TO_CORE(mod_power); if (*core_power).num_entities == 0 { return false; }
    let i = map_index_from_stream(core_power, stream); if (*core_power).map[i as usize].caps.psr_version == 0 { return false; }
    *active_psr_events = (*core_power).map[i as usize].psr_events; true
}

pub unsafe fn mod_power_set_psr_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, set_event: bool, event: psr_event, wait: bool) -> bool {
    if mod_power.is_null() || stream.is_null() { return false; }
    let core_power = MOD_POWER_TO_CORE(mod_power); let i = map_index_from_stream(core_power, stream);
    if (*core_power).num_entities == 0 || (*core_power).map[i as usize].caps.psr_version == 0 { return false; }
    if set_event { (*core_power).map[i as usize].psr_events |= event; } else { (*core_power).map[i as usize].psr_events &= !event; }
    let active = (*core_power).map[i as usize].psr_events;
    if active & psr_event_dynamic_display_switch != 0 && event != psr_event_dynamic_display_switch { return false; }
    if active & psr_event_os_override_hold != 0 && event != psr_event_os_override_hold { return false; }
    if active & psr_event_dynamic_link_rate_control != 0 && event != psr_event_dynamic_link_rate_control && event != psr_event_dds_defer_stream_enable && event != psr_event_dynamic_display_switch { return false; }
    let mut enable = true; let mut force_static = false;
    if active & (psr_event_test_harness_disable_psr | psr_event_os_request_disable | psr_event_pause | psr_event_edp_panel_off_disable_psr | psr_event_hw_programming | psr_event_defer_enable | psr_event_dds_defer_stream_enable | psr_event_vrr_transition | psr_event_immediate_flip | psr_event_full_screen | psr_event_vsync | psr_event_crc_window_active) != 0 { enable = false; }
    else if active & (psr_event_dynamic_display_switch | psr_event_dynamic_link_rate_control) != 0 { enable = true; force_static = true; }
    else if active & (psr_event_test_harness_enable_psr | psr_event_big_screen_video | psr_event_mpo_video_selective_update) != 0 { enable = true; }
    mod_power_psr_set_power_opt(mod_power, stream, active, enable);
    if (*core_power).map[i as usize].psr_enabled != enable || force_static { if set_psr_enable(mod_power, stream, enable, wait, force_static) { (*core_power).map[i as usize].psr_enabled = enable; } }
    true
}

pub unsafe fn mod_power_get_psr_state(mod_power: *mut mod_power, stream: *const dc_stream_state, state: *mut dc_psr_state) -> bool {
    if stream.is_null() || mod_power.is_null() { return false; } let core = MOD_POWER_TO_CORE(mod_power); if (*core).num_entities == 0 { return false; }
    dc_link_get_psr_state(dc_stream_get_link(stream), state)
}

pub unsafe fn mod_power_get_psr_enabled_status(mod_power: *mut mod_power, stream: *const dc_stream_state, enabled: *mut bool) -> bool {
    if mod_power.is_null() { return false; } let core = MOD_POWER_TO_CORE(mod_power); if (*core).num_entities == 0 { return false; }
    let i = map_index_from_stream(core, stream); if (*core).map[i as usize].caps.psr_version == 0 { return false; } *enabled = (*core).map[i as usize].psr_enabled; true
}

pub unsafe fn mod_power_psr_residency(mod_power: *mut mod_power, stream: *const dc_stream_state, residency: *mut ::core::ffi::c_uint, mode: u8) {
    if stream.is_null() || mod_power.is_null() { return; } let core = MOD_POWER_TO_CORE(mod_power); if (*core).num_entities == 0 { return; }
    let link = dc_stream_get_link(stream); if !link.is_null() { ((*(*link).dc).link_srv).edp_get_psr_residency(link, residency, mode); }
}

pub unsafe fn mod_power_psr_get_active_psr_events(mod_power: *mut mod_power, stream: *const dc_stream_state, active: *mut ::core::ffi::c_uint) -> bool {
    if stream.is_null() || mod_power.is_null() || active.is_null() { return false; } let core = MOD_POWER_TO_CORE(mod_power); if (*core).num_entities == 0 { return false; }
    let i = map_index_from_stream(core, stream); *active = (*core).map[i as usize].psr_events; true
}

pub unsafe fn mod_power_psr_set_sink_vtotal_in_psr_active(mod_power: *mut mod_power, stream: *const dc_stream_state, idle: u16, su: u16) -> bool {
    if stream.is_null() || mod_power.is_null() { return false; } let core = MOD_POWER_TO_CORE(mod_power); if (*core).num_entities == 0 { return false; }
    let i = map_index_from_stream(core, stream); if (*core).map[i as usize].caps.psr_version == 0 { return false; }
    let link = dc_stream_get_link(stream); ((*(*link).dc).link_srv).edp_set_sink_vtotal_in_psr_active(link, idle, su)
}

pub unsafe fn is_psr_su_specific_panel(link: *mut dc_link) -> bool {
    let caps = &mut (*link).dpcd_caps; let mut supported = false;
    if caps.edp_rev >= DP_EDP_14 { if caps.psr_info.psr_version >= DP_PSR2_WITH_Y_COORD_ET_SUPPORTED { supported = true; }
        if caps.sink_dev_id == DP_BRANCH_DEVICE_ID_001CF8 { if caps.psr_info.psr_version < DP_PSR2_WITH_Y_COORD_IS_SUPPORTED { supported = false; }
            else if caps.dsc_caps.dsc_basic_caps.fields.dsc_support.DSC_SUPPORT && ((caps.sink_dev_id_str[1] == 0x08 && caps.sink_dev_id_str[0] == 0x08) || (caps.sink_dev_id_str[1] == 0x08 && caps.sink_dev_id_str[0] == 0x07)) { supported = false; }
            else if caps.sink_dev_id_str[1] == 0x08 && caps.sink_dev_id_str[0] == 0x03 { supported = false; }
            else if caps.sink_dev_id_str[1] == 0x08 && caps.sink_dev_id_str[0] == 0x01 { supported = false; }
            else if caps.psr_info.force_psrsu_cap == 0x1 { supported = true; }
        }
    } supported
}

pub unsafe fn mod_power_calc_psr_configs(config: *mut psr_config, link: *mut dc_link, stream: *const dc_stream_state) {
    let caps = &mut (*link).dpcd_caps;
    let num_vblank_lines = (*stream).timing.v_total - (*stream).timing.v_addressable - (*stream).timing.v_border_top - (*stream).timing.v_border_bottom;
    let vblank_time = ((*stream).timing.h_total * num_vblank_lines * 1000) / ((*stream).timing.pix_clk_100hz / 10);
    let line_time = ((*stream).timing.h_total * 1000) / ((*stream).timing.pix_clk_100hz / 10) + 1;
    (*config).psr_rfb_setup_time = (6 - caps.psr_info.psr_dpcd_caps.bits.PSR_SETUP_TIME) * 55;
    if (*config).psr_rfb_setup_time > vblank_time { (*link).psr_settings.psr_frame_capture_indication_req = true; (*link).psr_settings.psr_sdp_transmit_line_num_deadline = num_vblank_lines; }
    else { (*link).psr_settings.psr_frame_capture_indication_req = false; (*link).psr_settings.psr_sdp_transmit_line_num_deadline = (vblank_time - (*config).psr_rfb_setup_time) / line_time; }
    (*config).psr_sdp_transmit_line_num_deadline = (*link).psr_settings.psr_sdp_transmit_line_num_deadline;
    (*config).line_time_in_us = line_time; (*config).su_y_granularity = caps.psr_info.psr2_su_y_granularity_cap; (*config).su_granularity_required = caps.psr_info.psr_dpcd_caps.bits.SU_GRANULARITY_REQUIRED; (*config).psr_frame_capture_indication_req = (*link).psr_settings.psr_frame_capture_indication_req; (*config).psr_exit_link_training_required = !caps.psr_info.psr_dpcd_caps.bits.LINK_TRAINING_ON_EXIT_NOT_REQUIRED;
}

pub unsafe fn psr_su_set_dsc_slice_height(dc: *mut dc, link: *mut dc_link, stream: *mut dc_stream_state, config: *mut psr_config) -> bool {
    (*config).dsc_slice_height = 0;
    if (*link).connector_signal & SIGNAL_TYPE_EDP == 0 || !(*dc).caps.edp_dsc_support || (*link).panel_config.dsc.disable_dsc_edp || !(*link).dpcd_caps.dsc_caps.dsc_basic_caps.fields.dsc_support.DSC_SUPPORT || (*stream).timing.dsc_cfg.num_slices_v == 0 { return true; }
    let pic_height = (*stream).timing.v_addressable + (*stream).timing.v_border_top + (*stream).timing.v_border_bottom;
    if (*stream).timing.dsc_cfg.num_slices_v == 0 { return false; }
    let slice_height = pic_height / (*stream).timing.dsc_cfg.num_slices_v; (*config).dsc_slice_height = slice_height as u16;
    if slice_height != 0 && (*config).su_y_granularity != 0 && slice_height % (*config).su_y_granularity != 0 { ASSERT(0); return false; } true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
