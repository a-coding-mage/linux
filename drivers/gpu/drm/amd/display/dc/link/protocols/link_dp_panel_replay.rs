/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit.

const DP_SINK_PR_ENABLE_AND_CONFIGURATION: u32 = 0x37B;
const DP_SINK_ENABLE_FRAME_SKIPPING_MODE_SHIFT: u32 = 5;

unsafe fn dp_pr_calc_num_static_frames(vsync_rate_hz: u32) -> u32 {
    // at least 2 frames for static screen
    let mut num_frames = 2;

    // get number of frames for at least 50ms
    if vsync_rate_hz > 40 {
        num_frames = (vsync_rate_hz + 10) / 20;
    }

    num_frames
}

unsafe fn dp_pr_set_static_screen_param(link: *mut dc_link) {
    let mut params: dc_static_screen_params = core::mem::zeroed();
    let dc = (*(*link).ctx).dc;
    // only support DP sst for now
    if !dc_is_dp_sst_signal((*link).connector_signal) {
        return;
    }

    for i in 0..MAX_PIPES {
        if (*(*dc).current_state).res_ctx.pipe_ctx[i].stream != core::ptr::null_mut()
            && (*(*(*dc).current_state).res_ctx.pipe_ctx[i].stream).link == link
        {
            let stream = (*(*dc).current_state).res_ctx.pipe_ctx[i].stream;
            let vsync_rate_hz = div64_u64(
                div64_u64(
                    (*stream).timing.pix_clk_100hz * 100u64,
                    (*stream).timing.v_total as u64,
                ),
                (*stream).timing.h_total as u64,
            ) as u32;
            params.triggers.cursor_update = true;
            params.triggers.overlay_update = true;
            params.triggers.surface_update = true;
            params.num_frames = dp_pr_calc_num_static_frames(vsync_rate_hz);
            dc_stream_set_static_screen_params(dc, &mut stream, 1, &mut params);
            break;
        }
    }
}

unsafe fn dp_setup_panel_replay(link: *mut dc_link, stream: *const dc_stream_state) -> bool {
    /* To-do: Setup Replay */
    let mut dc: *mut dc;
    let mut replay: *mut dmub_replay;
    let mut panel_inst: u32 = 0;
    let mut replay_context: replay_context = core::mem::zeroed();
    let mut line_time_in_ns: u32 = 0;
    let mut pr_config_1: panel_replay_enable_and_configuration_1 = core::mem::zeroed();
    let mut pr_config_2: panel_replay_enable_and_configuration_2 = core::mem::zeroed();
    let mut alpm_config: dpcd_alpm_configuration = core::mem::zeroed();
    let mut data: u8 = 0;

    replay_context.controllerId = CONTROLLER_ID_UNDEFINED;
    if link.is_null() { return false; }

    dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_1,
        &mut pr_config_1.raw as *mut _, core::mem::size_of::<u8>());
    dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_2,
        &mut pr_config_2.raw as *mut _, core::mem::size_of::<u8>());
    if !(*link).replay_settings.config.replay_supported { return false; }
    dc = (*(*link).ctx).dc;
    replay = (*(*dc).res_pool).replay;
    if replay.is_null() { return false; }
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    replay_context.aux_inst = link_get_ddc_aux_inst(link) as enum_channel_id;
    replay_context.digbe_inst = (*(*link).link_enc).transmitter;
    replay_context.digfe_inst = (*(*link).link_enc).preferred_engine;
    for i in 0..MAX_PIPES {
        if (*(*dc).current_state).res_ctx.pipe_ctx[i].stream == stream {
            replay_context.controllerId = (*(*(*dc).current_state).res_ctx.pipe_ctx[i].stream_res.tg).inst + 1;
            break;
        }
    }
    line_time_in_ns = (((*stream).timing.h_total * 1_000_000) /
        ((*stream).timing.pix_clk_100hz / 10)) + 1;
    replay_context.line_time_in_ns = line_time_in_ns;
    (*link).replay_settings.replay_feature_enabled = dp_pr_copy_settings(link, &mut replay_context);
    if (*link).replay_settings.replay_feature_enabled {
        if dc_is_embedded_signal((*link).connector_signal) {
            pr_config_1.bits.PANEL_REPLAY_ENABLE = 1;
            pr_config_1.bits.PANEL_REPLAY_CRC_ENABLE = 1;
            pr_config_1.bits.IRQ_HPD_ASSDP_MISSING = 1;
            pr_config_1.bits.IRQ_HPD_VSCSDP_UNCORRECTABLE_ERROR = 1;
            pr_config_1.bits.IRQ_HPD_RFB_ERROR = 1;
            pr_config_1.bits.IRQ_HPD_ACTIVE_FRAME_CRC_ERROR = 1;
            pr_config_1.bits.PANEL_REPLAY_SELECTIVE_UPDATE_ENABLE = 1;
            pr_config_1.bits.PANEL_REPLAY_EARLY_TRANSPORT_ENABLE = 1;
        } else { pr_config_1.bits.PANEL_REPLAY_ENABLE = 1; }
        pr_config_2.bits.SINK_REFRESH_RATE_UNLOCK_GRANTED = 0;
        if (*link).dpcd_caps.vesa_replay_caps.bits.SU_Y_GRANULARITY_EXT_CAP_SUPPORTED {
            pr_config_2.bits.SU_Y_GRANULARITY_EXT_VALUE_ENABLED = 1;
        }
        pr_config_2.bits.SU_REGION_SCAN_LINE_CAPTURE_INDICATION = 0;
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_1,
            &mut pr_config_1.raw as *mut _, core::mem::size_of::<u8>());
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_2,
            &mut pr_config_2.raw as *mut _, core::mem::size_of::<u8>());
        alpm_config = core::mem::zeroed();
        alpm_config.bits.ENABLE = ((*link).replay_settings.config.alpm_mode != DC_ALPM_UNSUPPORTED) as u32;
        if (*link).replay_settings.config.alpm_mode == DC_ALPM_AUXLESS {
            alpm_config.bits.ALPM_MODE_SEL = 1;
            alpm_config.bits.ACDS_PERIOD_DURATION = 1;
        }
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_RECEIVER_ALPM_CONFIG,
            &mut alpm_config.raw as *mut _, core::mem::size_of_val(&alpm_config.raw));
        if (*link).replay_settings.config.frame_skip_supported { data |= 1 << DP_SINK_ENABLE_FRAME_SKIPPING_MODE_SHIFT; }
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_SINK_PR_ENABLE_AND_CONFIGURATION,
            &mut data as *mut _, core::mem::size_of::<u8>());
    }
    true
}

pub unsafe fn dp_pr_get_panel_inst(dc: *const dc, link: *const dc_link, inst_out: *mut u32) -> bool {
    if dc.is_null() || link.is_null() || inst_out.is_null() { return false; }
    if !(*dc).config.frame_update_cmd_version2 { return dc_get_edp_link_panel_inst(dc, link, inst_out); }
    if !dc_is_dp_sst_signal((*link).connector_signal) { return false; }
    for i in 0..MAX_PIPES {
        let pipe = &(*(*dc).current_state).res_ctx.pipe_ctx[i];
        if !pipe.stream.is_null() && (*pipe.stream).link == link {
            *inst_out = if !pipe.stream_res.tg.is_null() { (*pipe.stream_res.tg).inst } else { 0 };
            return true;
        }
    }
    false
}

pub unsafe fn dp_setup_replay(link: *mut dc_link, stream: *const dc_stream_state) -> bool {
    if link.is_null() { return false; }
    if (*link).replay_settings.config.replay_version == DC_VESA_PANEL_REPLAY { dp_setup_panel_replay(link, stream) }
    else if (*link).replay_settings.config.replay_version == DC_FREESYNC_REPLAY { edp_setup_freesync_replay(link, stream) }
    else { false }
}

pub unsafe fn dp_pr_enable(link: *mut dc_link, enable: bool) -> bool {
    let dc = (*(*link).ctx).dc;
    let mut panel_inst = 0;
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    if (*link).replay_settings.replay_allow_active == enable { return true; }
    if enable && !dc_is_embedded_signal((*link).connector_signal) { dp_pr_set_static_screen_param(link); }
    cmd.pr_enable.header.r#type = DMUB_CMD__PR;
    cmd.pr_enable.header.sub_type = DMUB_CMD__PR_ENABLE;
    cmd.pr_enable.header.payload_bytes = core::mem::size_of::<dmub_cmd_pr_enable_data>();
    cmd.pr_enable.data.panel_inst = panel_inst as u8;
    cmd.pr_enable.data.enable = enable as u8;
    dc_wake_and_execute_dmub_cmd((*dc).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    (*link).replay_settings.replay_allow_active = enable;
    true
}

pub unsafe fn dp_pr_copy_settings(link: *mut dc_link, replay_context: *mut replay_context) -> bool {
    let dc = (*(*link).ctx).dc;
    let mut panel_inst = 0;
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let mut pipe_ctx: *mut pipe_ctx = core::ptr::null_mut();
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    for i in 0..MAX_PIPES {
        let pipe = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i];
        if !pipe.stream.is_null() && !(*pipe.stream).link.is_null() && (*pipe.stream).link == link
            && dc_is_dp_sst_signal((*(*pipe.stream).link).connector_signal) { pipe_ctx = pipe; break; }
    }
    if pipe_ctx.is_null() { return false; }
    cmd.pr_copy_settings.header.r#type = DMUB_CMD__PR;
    cmd.pr_copy_settings.header.sub_type = DMUB_CMD__PR_COPY_SETTINGS;
    cmd.pr_copy_settings.header.payload_bytes = core::mem::size_of::<dmub_cmd_pr_copy_settings_data>();
    cmd.pr_copy_settings.data.panel_inst = panel_inst as u8;
    cmd.pr_copy_settings.data.aux_inst = (*replay_context).aux_inst;
    cmd.pr_copy_settings.data.digbe_inst = (*replay_context).digbe_inst;
    cmd.pr_copy_settings.data.digfe_inst = (*replay_context).digfe_inst;
    cmd.pr_copy_settings.data.dpp_inst = if !(*pipe_ctx).plane_res.dpp.is_null() { (*(*pipe_ctx).plane_res.dpp).inst as u8 } else { 0 };
    cmd.pr_copy_settings.data.otg_inst = if !(*pipe_ctx).stream_res.tg.is_null() { (*(*pipe_ctx).stream_res.tg).inst as u8 } else { 0 };
    cmd.pr_copy_settings.data.dpphy_inst = (*link).link_enc.transmitter;
    cmd.pr_copy_settings.data.line_time_in_ns = (*replay_context).line_time_in_ns;
    cmd.pr_copy_settings.data.flags.bitfields.fec_enable_status = ((*link).fec_state == dc_link_fec_enabled) as u32;
    cmd.pr_copy_settings.data.flags.bitfields.dsc_enable_status = ((*(*pipe_ctx).stream).timing.flags.DSC == 1) as u32;
    cmd.pr_copy_settings.data.debug.u32All = (*link).replay_settings.config.debug_flags;
    cmd.pr_copy_settings.data.flags.bitfields.alpm_mode = (*link).replay_settings.config.alpm_mode as enum_dmub_alpm_mode;
    if (*link).replay_settings.config.alpm_mode == DC_ALPM_AUXLESS {
        cmd.pr_copy_settings.data.auxless_alpm_data.lfps_setup_ns = (*dc).debug.auxless_alpm_lfps_setup_ns as u16;
        cmd.pr_copy_settings.data.auxless_alpm_data.lfps_period_ns = (*dc).debug.auxless_alpm_lfps_period_ns as u16;
        cmd.pr_copy_settings.data.auxless_alpm_data.lfps_silence_ns = (*dc).debug.auxless_alpm_lfps_silence_ns as u16;
        cmd.pr_copy_settings.data.auxless_alpm_data.lfps_t1_t2_override_us = (*dc).debug.auxless_alpm_lfps_t1t2_us as u16;
        cmd.pr_copy_settings.data.auxless_alpm_data.lfps_t1_t2_offset_us = (*dc).debug.auxless_alpm_lfps_t1t2_offset_us as u16;
        cmd.pr_copy_settings.data.auxless_alpm_data.lttpr_count = (*(*link).dc).link_srv.dp_get_lttpr_count(link);
    }
    cmd.pr_copy_settings.data.su_granularity_needed = (*link).dpcd_caps.vesa_replay_caps.bits.PR_SU_GRANULARITY_NEEDED;
    cmd.pr_copy_settings.data.su_x_granularity = (*link).dpcd_caps.vesa_replay_su_info.pr_su_x_granularity;
    cmd.pr_copy_settings.data.su_y_granularity = (*link).dpcd_caps.vesa_replay_su_info.pr_su_y_granularity;
    cmd.pr_copy_settings.data.su_y_granularity_extended_caps = (*link).dpcd_caps.vesa_replay_su_info.pr_su_y_granularity_extended_caps;
    if (*(*pipe_ctx).stream).timing.dsc_cfg.num_slices_v > 0 { cmd.pr_copy_settings.data.dsc_slice_height = (((*(*pipe_ctx).stream).timing.v_addressable + (*(*pipe_ctx).stream).timing.v_border_top + (*(*pipe_ctx).stream).timing.v_border_bottom) / (*(*pipe_ctx).stream).timing.dsc_cfg.num_slices_v) as u16; }
    cmd.pr_copy_settings.data.main_link_activity_option = if dc_is_embedded_signal((*link).connector_signal) { OPTION_1C } else { OPTION_1A };
    dc_wake_and_execute_dmub_cmd((*dc).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dp_pr_update_state(link: *mut dc_link, update_state_data: *mut dmub_cmd_pr_update_state_data) -> bool {
    let dc = (*(*link).ctx).dc; let mut panel_inst = 0; let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    core::ptr::copy_nonoverlapping(update_state_data, &mut cmd.pr_update_state.data, 1);
    cmd.pr_update_state.header.r#type = DMUB_CMD__PR; cmd.pr_update_state.header.sub_type = DMUB_CMD__PR_UPDATE_STATE; cmd.pr_update_state.header.payload_bytes = core::mem::size_of::<dmub_cmd_pr_update_state_data>(); cmd.pr_update_state.data.panel_inst = panel_inst as u8;
    dc_wake_and_execute_dmub_cmd((*dc).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT); true
}

pub unsafe fn dp_pr_set_general_cmd(link: *mut dc_link, general_cmd_data: *mut dmub_cmd_pr_general_cmd_data) -> bool {
    let dc = (*(*link).ctx).dc; let mut panel_inst = 0; let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    core::ptr::copy_nonoverlapping(general_cmd_data, &mut cmd.pr_general_cmd.data, 1);
    cmd.pr_general_cmd.header.r#type = DMUB_CMD__PR; cmd.pr_general_cmd.header.sub_type = DMUB_CMD__PR_GENERAL_CMD; cmd.pr_general_cmd.header.payload_bytes = core::mem::size_of::<dmub_cmd_pr_general_cmd_data>(); cmd.pr_general_cmd.data.panel_inst = panel_inst as u8;
    dc_wake_and_execute_dmub_cmd((*dc).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT); true
}

pub unsafe fn dp_pr_get_state(link: *const dc_link, state: *mut u64) -> bool {
    let dc = (*(*link).ctx).dc; let mut panel_inst = 0; let mut retry_count: u32 = 0; let mut replay_state = PR_STATE_INVALID;
    if !dp_pr_get_panel_inst(dc, link, &mut panel_inst) { return false; }
    loop {
        if !dc_wake_and_execute_gpint((*dc).ctx, DMUB_GPINT__GET_REPLAY_STATE, panel_inst as u16, &mut replay_state, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) { replay_state = PR_STATE_INVALID; }
        *state = replay_state as u64;
        retry_count = retry_count.wrapping_add(1);
        if !(retry_count <= 1000 && *state == PR_STATE_INVALID as u64) { break; }
    }
    if retry_count >= 1000 && *state == PR_STATE_INVALID as u64 { ASSERT(0); }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
