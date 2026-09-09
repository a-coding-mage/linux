// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit.

const MAX_PIPES: usize = 6;
const GPINT_RETRY_NUM: u32 = 20;

static DP_SINK_DEVICE_STR_ID_1: [u8; 5] = [7, 1, 8, 7, 3];
static DP_SINK_DEVICE_STR_ID_2: [u8; 5] = [7, 1, 8, 7, 5];

/* Get Replay state from firmware. */
unsafe fn dmub_replay_get_state(dmub: *mut dmub_replay, state: *mut replay_state, panel_inst: u8) {
    let mut retry_count: u32 = 0;
    loop {
        // Send gpint command and wait for ack
        if !dc_wake_and_execute_gpint((*dmub).ctx, DMUB_GPINT__GET_REPLAY_STATE, panel_inst,
            state as *mut u32, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) {
            // Return invalid state when GPINT times out
            *state = REPLAY_STATE_INVALID;
        }
        retry_count += 1;
        if !(retry_count <= 1000 && *state == REPLAY_STATE_INVALID) { break; }
    }
    // Assert if max retry hit
    if retry_count >= 1000 && *state == REPLAY_STATE_INVALID {
        ASSERT(0);
        /* To-do: Add retry fail log */
    }
}

/* Enable/Disable Replay. */
unsafe fn dmub_replay_enable(dmub: *mut dmub_replay, enable: bool, wait: bool, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*dmub).ctx;
    let mut retry_count: u32;
    let mut state: replay_state = REPLAY_STATE_0;
    cmd.replay_enable.header.type_ = DMUB_CMD__REPLAY;
    cmd.replay_enable.data.panel_inst = panel_inst;
    cmd.replay_enable.header.sub_type = DMUB_CMD__REPLAY_ENABLE;
    cmd.replay_enable.data.enable = if enable { REPLAY_ENABLE } else { REPLAY_DISABLE };
    cmd.replay_enable.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_enable_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    if wait {
        retry_count = 0;
        while retry_count <= 1000 {
            dmub_replay_get_state(dmub, &mut state, panel_inst);
            if (enable && state != REPLAY_STATE_0) || (!enable && state == REPLAY_STATE_0) { break; }
            // must *not* be fsleep - this can be called from high irq levels
            udelay(500);
            retry_count += 1;
        }
        // assert if max retry hit
        if retry_count >= 1000 { ASSERT(0); }
    }
}

/* Set REPLAY power optimization flags. */
unsafe fn dmub_replay_set_power_opt(dmub: *mut dmub_replay, power_opt: u32, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*dmub).ctx;
    cmd.replay_set_power_opt.header.type_ = DMUB_CMD__REPLAY;
    cmd.replay_set_power_opt.header.sub_type = DMUB_CMD__SET_REPLAY_POWER_OPT;
    cmd.replay_set_power_opt.header.payload_bytes = core::mem::size_of::<dmub_cmd_replay_set_power_opt_data>();
    cmd.replay_set_power_opt.replay_set_power_opt_data.power_opt = power_opt;
    cmd.replay_set_power_opt.replay_set_power_opt_data.panel_inst = panel_inst;
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

/* Setup Replay by programming phy registers and sending replay hw context values to firmware. */
unsafe fn dmub_replay_copy_settings(dmub: *mut dmub_replay, link: *mut dc_link,
    replay_context: *mut replay_context, panel_inst: u8) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*dmub).ctx;
    let copy = &mut cmd.replay_copy_settings.replay_copy_settings_data;
    let mut pipe_ctx: *mut pipe_ctx = core::ptr::null_mut();
    let res_ctx = &mut (*(*(*link).ctx).dc).current_state.res_ctx;
    for i in 0..MAX_PIPES {
        let p = &mut res_ctx.pipe_ctx[i];
        if !p.stream.is_null() && !(*p.stream).link.is_null() && (*p.stream).link == link &&
            (*(*p.stream).link).connector_signal == SIGNAL_TYPE_EDP {
            pipe_ctx = p; // TODO: refactor for multi edp support
            break;
        }
    }
    if pipe_ctx.is_null() { return false; }
    cmd.replay_copy_settings.header.type_ = DMUB_CMD__REPLAY;
    cmd.replay_copy_settings.header.sub_type = DMUB_CMD__REPLAY_COPY_SETTINGS;
    cmd.replay_copy_settings.header.payload_bytes = core::mem::size_of::<dmub_cmd_replay_copy_settings_data>();
    copy.aux_inst = (*replay_context).aux_inst;
    copy.digbe_inst = (*replay_context).digbe_inst;
    copy.digfe_inst = (*replay_context).digfe_inst;
    copy.dpp_inst = if !(*pipe_ctx).plane_res.dpp.is_null() { (*(*pipe_ctx).plane_res.dpp).inst as u8 } else { 0 };
    copy.otg_inst = if !(*pipe_ctx).stream_res.tg.is_null() { (*(*pipe_ctx).stream_res.tg).inst as u8 } else { 0 };
    copy.dpphy_inst = (*link).link_enc.transmitter;
    copy.line_time_in_ns = (*replay_context).line_time_in_ns as u16;
    copy.panel_inst = panel_inst as u16;
    copy.debug.u32All = (*link).replay_settings.config.debug_flags;
    copy.pixel_deviation_per_line = (*link).dpcd_caps.pr_info.pixel_deviation_per_line;
    copy.max_deviation_line = (*link).dpcd_caps.pr_info.max_deviation_line as u16;
    copy.smu_optimizations_en = (*link).replay_settings.replay_smu_opt_enable;
    copy.replay_timing_sync_supported = (*link).replay_settings.config.replay_timing_sync_supported;
    copy.replay_support_fast_resync_in_ultra_sleep_mode = (*link).replay_settings.config.replay_support_fast_resync_in_ultra_sleep_mode;
    copy.debug.bitfields.enable_ips_visual_confirm = (*dc).dc.debug.enable_ips_visual_confirm;
    copy.flags.u32All = 0;
    copy.flags.bitfields.fec_enable_status = (*link).fec_state == dc_link_fec_enabled;
    copy.flags.bitfields.dsc_enable_status = (*pipe_ctx).stream.timing.flags.DSC == 1;
    copy.flags.bitfields.force_wakeup_by_tps3 = if (*link).dpcd_caps.fec_cap.bits.FEC_CAPABLE && !(*link).dc.debug.disable_fec &&
        (*link).dpcd_caps.dsc_caps.dsc_basic_caps.fields.dsc_support.DSC_SUPPORT && !(*link).panel_config.dsc.disable_dsc_edp &&
        (*link).dc.caps.edp_dsc_support && (*link).dpcd_caps.sink_dev_id == DP_DEVICE_ID_38EC11 &&
        (memcmp((*link).dpcd_caps.sink_dev_id_str.as_ptr(), DP_SINK_DEVICE_STR_ID_1.as_ptr(), 5) == 0 ||
         memcmp((*link).dpcd_caps.sink_dev_id_str.as_ptr(), DP_SINK_DEVICE_STR_ID_2.as_ptr(), 5) == 0) { 1 } else { 0 };
    copy.flags.bitfields.alpm_mode = (*link).replay_settings.config.alpm_mode as dmub_alpm_mode;
    if (*link).replay_settings.config.alpm_mode == DC_ALPM_AUXLESS {
        copy.auxless_alpm_data.lfps_setup_ns = (*dc).dc.debug.auxless_alpm_lfps_setup_ns as u16;
        copy.auxless_alpm_data.lfps_period_ns = (*dc).dc.debug.auxless_alpm_lfps_period_ns as u16;
        copy.auxless_alpm_data.lfps_silence_ns = (*dc).dc.debug.auxless_alpm_lfps_silence_ns as u16;
        copy.auxless_alpm_data.lfps_t1_t2_override_us = (*dc).dc.debug.auxless_alpm_lfps_t1t2_us as u16;
        copy.auxless_alpm_data.lfps_t1_t2_offset_us = (*dc).dc.debug.auxless_alpm_lfps_t1t2_offset_us as u16;
        copy.auxless_alpm_data.lttpr_count = (*(*link).dc.link_srv).dp_get_lttpr_count(link);
    }
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

/* Set coasting vtotal. */
unsafe fn dmub_replay_set_coasting_vtotal(dmub: *mut dmub_replay, coasting_vtotal: u32, _panel_inst: u8, frame_skip_number: u16) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*dmub).ctx;
    cmd.replay_set_coasting_vtotal.header.type_ = DMUB_CMD__REPLAY;
    cmd.replay_set_coasting_vtotal.header.sub_type = DMUB_CMD__REPLAY_SET_COASTING_VTOTAL;
    cmd.replay_set_coasting_vtotal.header.payload_bytes = core::mem::size_of::<dmub_cmd_replay_set_coasting_vtotal_data>();
    cmd.replay_set_coasting_vtotal.replay_set_coasting_vtotal_data.coasting_vtotal = coasting_vtotal & 0xffff;
    cmd.replay_set_coasting_vtotal.replay_set_coasting_vtotal_data.coasting_vtotal_high = (coasting_vtotal & 0xffff0000) >> 16;
    cmd.replay_set_coasting_vtotal.replay_set_coasting_vtotal_data.frame_skip_number = frame_skip_number;
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

/* Get Replay residency from firmware. */
unsafe fn dmub_replay_residency(dmub: *mut dmub_replay, panel_inst: u8, residency: *mut u32, is_start: bool, mode: pr_residency_mode) {
    let mut param = (panel_inst as u16) << 8;
    match mode { PR_RESIDENCY_MODE_PHY => param |= REPLAY_RESIDENCY_FIELD_MODE_PHY, PR_RESIDENCY_MODE_ALPM => param |= REPLAY_RESIDENCY_FIELD_MODE_ALPM,
        PR_RESIDENCY_MODE_IPS2 => { param |= REPLAY_RESIDENCY_REVISION_1 | REPLAY_RESIDENCY_FIELD_MODE2_IPS; },
        PR_RESIDENCY_MODE_FRAME_CNT => { param |= REPLAY_RESIDENCY_REVISION_1 | REPLAY_RESIDENCY_FIELD_MODE2_FRAME_CNT; },
        PR_RESIDENCY_MODE_ENABLEMENT_PERIOD => { param |= REPLAY_RESIDENCY_REVISION_1 | REPLAY_RESIDENCY_FIELD_MODE2_EN_PERIOD; }, _ => {} }
    if is_start { param |= REPLAY_RESIDENCY_ENABLE; }
    for _ in 0..GPINT_RETRY_NUM { if dc_wake_and_execute_gpint((*dmub).ctx, DMUB_GPINT__REPLAY_RESIDENCY, param, residency, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) { return; } udelay(100); }
    *residency = 0;
}

// The remaining command helpers retain the firmware command layout and dispatch semantics.
unsafe fn dmub_replay_set_power_opt_and_coasting_vtotal(dmub: *mut dmub_replay, power_opt: u32, panel_inst: u8, coasting_vtotal: u32, frame_skip_number: u16) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed(); let dc = (*dmub).ctx;
    cmd.replay_set_power_opt_and_coasting_vtotal.header.type_ = DMUB_CMD__REPLAY;
    cmd.replay_set_power_opt_and_coasting_vtotal.header.sub_type = DMUB_CMD__REPLAY_SET_POWER_OPT_AND_COASTING_VTOTAL;
    cmd.replay_set_power_opt_and_coasting_vtotal.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_set_power_opt_and_coasting_vtotal>() - core::mem::size_of::<dmub_cmd_header>();
    cmd.replay_set_power_opt_and_coasting_vtotal.replay_set_power_opt_data.power_opt = power_opt;
    cmd.replay_set_power_opt_and_coasting_vtotal.replay_set_power_opt_data.panel_inst = panel_inst;
    cmd.replay_set_power_opt_and_coasting_vtotal.replay_set_coasting_vtotal_data.coasting_vtotal = coasting_vtotal & 0xffff;
    cmd.replay_set_power_opt_and_coasting_vtotal.replay_set_coasting_vtotal_data.coasting_vtotal_high = (coasting_vtotal & 0xffff0000) >> 16;
    cmd.replay_set_power_opt_and_coasting_vtotal.replay_set_coasting_vtotal_data.frame_skip_number = frame_skip_number;
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

/* send Replay general cmd to DMUB. */
unsafe fn dmub_replay_send_cmd(dmub: *mut dmub_replay, msg: replay_FW_Message_type, cmd_element: *mut dmub_replay_cmd_set) {
    if dmub.is_null() || cmd_element.is_null() { return; }
    let ctx = (*dmub).ctx;
    if ctx.is_null() || msg == Replay_Msg_Not_Support { return; }
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.replay_set_timing_sync.header.type_ = DMUB_CMD__REPLAY;
    match msg {
        Replay_Set_Timing_Sync_Supported => { cmd.replay_set_timing_sync.header.sub_type = DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED; cmd.replay_set_timing_sync.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_set_timing_sync>() - core::mem::size_of::<dmub_cmd_header>(); cmd.replay_set_timing_sync.replay_set_timing_sync_data.panel_inst = (*cmd_element).sync_data.panel_inst; cmd.replay_set_timing_sync.replay_set_timing_sync_data.timing_sync_supported = (*cmd_element).sync_data.timing_sync_supported; }
        Replay_Set_Residency_Frameupdate_Timer => { cmd.replay_set_frameupdate_timer.header.sub_type = DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER; cmd.replay_set_frameupdate_timer.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_set_frameupdate_timer>() - core::mem::size_of::<dmub_cmd_header>(); cmd.replay_set_frameupdate_timer.data.panel_inst = (*cmd_element).panel_inst; cmd.replay_set_frameupdate_timer.data.enable = (*cmd_element).timer_data.enable; cmd.replay_set_frameupdate_timer.data.frameupdate_count = (*cmd_element).timer_data.frameupdate_count; }
        Replay_Set_Pseudo_VTotal => { cmd.replay_set_pseudo_vtotal.header.sub_type = DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL; cmd.replay_set_pseudo_vtotal.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_set_pseudo_vtotal>() - core::mem::size_of::<dmub_cmd_header>(); cmd.replay_set_pseudo_vtotal.data.panel_inst = (*cmd_element).pseudo_vtotal_data.panel_inst; cmd.replay_set_pseudo_vtotal.data.vtotal = (*cmd_element).pseudo_vtotal_data.vtotal; }
        Replay_Disabled_Adaptive_Sync_SDP => { cmd.replay_disabled_adaptive_sync_sdp.header.sub_type = DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP; cmd.replay_disabled_adaptive_sync_sdp.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_disabled_adaptive_sync_sdp>() - core::mem::size_of::<dmub_cmd_header>(); cmd.replay_disabled_adaptive_sync_sdp.data.panel_inst = (*cmd_element).disabled_adaptive_sync_sdp_data.panel_inst; cmd.replay_disabled_adaptive_sync_sdp.data.force_disabled = (*cmd_element).disabled_adaptive_sync_sdp_data.force_disabled; }
        Replay_Set_General_Cmd => { cmd.replay_set_general_cmd.header.sub_type = DMUB_CMD__REPLAY_SET_GENERAL_CMD; cmd.replay_set_general_cmd.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_replay_set_general_cmd>() - core::mem::size_of::<dmub_cmd_header>(); cmd.replay_set_general_cmd.data.panel_inst = (*cmd_element).set_general_cmd_data.panel_inst; cmd.replay_set_general_cmd.data.subtype = (*cmd_element).set_general_cmd_data.subtype; cmd.replay_set_general_cmd.data.param1 = (*cmd_element).set_general_cmd_data.param1; cmd.replay_set_general_cmd.data.param2 = (*cmd_element).set_general_cmd_data.param2; }
        Replay_Msg_Not_Support => return,
        _ => return,
    }
    dc_wake_and_execute_dmub_cmd(ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

static replay_funcs: dmub_replay_funcs = dmub_replay_funcs {
    replay_copy_settings: dmub_replay_copy_settings, replay_enable: dmub_replay_enable,
    replay_get_state: dmub_replay_get_state, replay_set_power_opt: dmub_replay_set_power_opt,
    replay_set_coasting_vtotal: dmub_replay_set_coasting_vtotal, replay_residency: dmub_replay_residency,
    replay_set_power_opt_and_coasting_vtotal: dmub_replay_set_power_opt_and_coasting_vtotal,
    replay_send_cmd: dmub_replay_send_cmd,
};

/* Construct Replay object. */
unsafe fn dmub_replay_construct(replay: *mut dmub_replay, ctx: *mut dc_context) { (*replay).ctx = ctx; (*replay).funcs = &replay_funcs; }

/* Allocate and initialize Replay object. */
unsafe fn dmub_replay_create(ctx: *mut dc_context) -> *mut dmub_replay {
    let replay = kzalloc_obj::<dmub_replay>();
    if replay.is_null() { BREAK_TO_DEBUGGER(); return core::ptr::null_mut(); }
    dmub_replay_construct(replay, ctx); replay
}

/* Deallocate Replay object. */
unsafe fn dmub_replay_destroy(dmub: *mut *mut dmub_replay) { kfree(*dmub); *dmub = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
