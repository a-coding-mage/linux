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

/* FILE POLICY AND INTENDED USAGE:
 * This file implements DP HPD short pulse handling sequence according to DP
 * specifications
 */

// C dependencies are supplied by the surrounding translation unit.

pub unsafe fn dp_parse_link_loss_status(
    link: *mut dc_link,
    hpd_irq_dpcd_data: *mut hpd_irq_data,
) -> bool {
    let mut irq_reg_rx_power_state: u8 = 0;
    let mut dpcd_result = DC_ERROR_UNEXPECTED;
    let mut lane_status = lane_status::default();
    let mut sink_status_changed = false;
    let mut return_code = false;

    if (*link).cur_link_settings.lane_count == 0 { return return_code; }

    for lane in 0..((*link).cur_link_settings.lane_count as u32) {
        lane_status.raw = dp_get_nibble_at_index(
            &(*hpd_irq_dpcd_data).bytes.lane01_status.raw, lane);
        if !lane_status.bits.CHANNEL_EQ_DONE_0 || !lane_status.bits.CR_DONE_0 ||
            !lane_status.bits.SYMBOL_LOCKED_0 {
            sink_status_changed = true;
            break;
        }
    }

    if link_dp_get_encoding_format(&(*link).cur_link_settings) == DP_128b_132b_ENCODING &&
        (!(*hpd_irq_dpcd_data).bytes.lane_status_updated.bits.EQ_INTERLANE_ALIGN_DONE_128b_132b ||
         !(*hpd_irq_dpcd_data).bytes.lane_status_updated.bits.CDS_INTERLANE_ALIGN_DONE_128b_132b) {
        sink_status_changed = true;
    } else if !(*hpd_irq_dpcd_data).bytes.lane_status_updated.bits.INTERLANE_ALIGN_DONE {
        sink_status_changed = true;
    }

    if sink_status_changed {
        DC_LOG_HW_HPD_IRQ!("%s: Link Status changed.\n", __func__);
        return_code = true;
        dpcd_result = core_link_read_dpcd(link, DP_SET_POWER,
            &mut irq_reg_rx_power_state, core::mem::size_of_val(&irq_reg_rx_power_state));
        if dpcd_result != DC_OK {
            DC_LOG_HW_HPD_IRQ!("%s: DPCD read failed to obtain power state.\n", __func__);
        } else if irq_reg_rx_power_state != DP_SET_POWER_D0 { return_code = false; }
    }
    return return_code;
}

unsafe fn handle_hpd_irq_psr_sink(link: *mut dc_link) -> bool {
    let mut psr_configuration = dpcd_psr_configuration::default();
    if !(*link).psr_settings.psr_feature_enabled { return false; }
    dm_helpers_dp_read_dpcd((*link).ctx, link, 368, &mut psr_configuration.raw,
        core::mem::size_of_val(&psr_configuration.raw));
    if psr_configuration.bits.ENABLE {
        let mut dpcdbuf = [0u8; 3];
        let mut psr_error_status = psr_error_status::default();
        let mut psr_sink_psr_status = psr_sink_psr_status::default();
        dm_helpers_dp_read_dpcd((*link).ctx, link, 0x2006, dpcdbuf.as_mut_ptr(), dpcdbuf.len());
        psr_error_status.raw = dpcdbuf[0];
        psr_sink_psr_status.raw = dpcdbuf[2];
        if psr_error_status.bits.LINK_CRC_ERROR || psr_error_status.bits.RFB_STORAGE_ERROR ||
            psr_error_status.bits.VSC_SDP_ERROR {
            dm_helpers_dp_write_dpcd((*link).ctx, link, 8198, &mut psr_error_status.raw,
                core::mem::size_of_val(&psr_error_status.raw));
            if (*link).psr_settings.psr_allow_active {
                let mut allow_active = false;
                edp_set_psr_allow_active(link, &mut allow_active, true, false, core::ptr::null_mut());
                allow_active = true;
                edp_set_psr_allow_active(link, &mut allow_active, true, false, core::ptr::null_mut());
            }
            return true;
        } else if psr_sink_psr_status.bits.SINK_SELF_REFRESH_STATUS == PSR_SINK_STATE_ACTIVE_DISPLAY_FROM_SINK_RFB {
            return true;
        }
    }
    false
}

unsafe fn handle_hpd_irq_vesa_replay_sink(link: *mut dc_link) {
    let mut pr_error_status = pr_error_status::default();
    if !(*link).replay_settings.replay_feature_enabled ||
        (*link).replay_settings.config.replay_version != DC_VESA_PANEL_REPLAY { return; }
    dm_helpers_dp_read_dpcd((*link).ctx, link, DP_PR_ERROR_STATUS, &mut pr_error_status.raw,
        core::mem::size_of_val(&pr_error_status.raw));
    if pr_error_status.bits.LINK_CRC_ERROR || pr_error_status.bits.RFB_STORAGE_ERROR ||
        pr_error_status.bits.VSC_SDP_ERROR || pr_error_status.bits.ASSDP_MISSING_ERROR {
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PR_ERROR_STATUS, &mut pr_error_status.raw,
            core::mem::size_of_val(&pr_error_status.raw));
        if (*link).replay_settings.replay_allow_active { dp_pr_enable(link, false); dp_pr_enable(link, true); }
    }
}

unsafe fn handle_hpd_irq_replay_sink(link: *mut dc_link, need_re_enable: *mut bool, replay_esd_detection_needed: *mut bool) {
    let mut replay_configuration = dpcd_replay_configuration::default();
    let mut replay_sink_status = dpcd_replay_configuration::default();
    let mut replay_error_status = psr_error_status::default();
    let mut ret = false;
    let mut retries = 0;
    if !(*link).replay_settings.replay_feature_enabled { return; }
    if (*link).replay_settings.config.replay_version != DC_FREESYNC_REPLAY {
        handle_hpd_irq_vesa_replay_sink(link); return;
    }
    while retries < 10 {
        ret = dm_helpers_dp_read_dpcd((*link).ctx, link, DP_SINK_PR_REPLAY_STATUS,
            &mut replay_configuration.raw, core::mem::size_of_val(&replay_configuration.raw));
        if ret { break; }
        retries += 1;
    }
    if !ret { DC_LOG_WARNING!("[%s][%d] DPCD read addr.0x%x failed with %d retries\n", __func__, __LINE__, DP_SINK_PR_REPLAY_STATUS, retries); }
    dm_helpers_dp_read_dpcd((*link).ctx, link, DP_PSR_ERROR_STATUS, &mut replay_error_status.raw, core::mem::size_of_val(&replay_error_status.raw));
    dm_helpers_dp_read_dpcd((*link).ctx, link, DP_PR_REPLAY_SINK_STATUS, &mut replay_sink_status.raw, 1);
    if replay_error_status.bits.LINK_CRC_ERROR || replay_configuration.bits.DESYNC_ERROR_STATUS ||
        replay_configuration.bits.STATE_TRANSITION_ERROR_STATUS || replay_sink_status.bits.SINK_DEVICE_REPLAY_STATUS == 0x7 {
        (*link).replay_settings.config.replay_error_status.raw |= replay_error_status.raw;
        if replay_configuration.bits.DESYNC_ERROR_STATUS { (*link).replay_settings.replay_desync_error_fail_count += 1; }
        if (*link).replay_settings.config.force_disable_desync_error_check { return; }
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_SINK_PR_REPLAY_STATUS, &mut replay_configuration.raw, core::mem::size_of_val(&replay_configuration.raw));
        dm_helpers_dp_write_dpcd((*link).ctx, link, DP_PSR_ERROR_STATUS, &mut replay_error_status.raw, core::mem::size_of_val(&replay_error_status.raw));
        if (*link).replay_settings.replay_allow_active {
            let mut allow_active = false;
            edp_set_replay_allow_active(link, &mut allow_active, true, false, core::ptr::null_mut());
            *need_re_enable = true;
        }
    }
    if (*link).ctx.as_ref().unwrap().dc.as_ref().unwrap().debug.enable_replay_esd_recovery &&
        !(*link).replay_settings.replay_allow_active && replay_sink_status.bits.SINK_DEVICE_REPLAY_STATUS == 0x7 {
        *replay_esd_detection_needed = true;
    }
}

pub unsafe fn dp_handle_link_loss(link: *mut dc_link) {
    let mut pipes: [*mut pipe_ctx; MAX_PIPES] = [core::ptr::null_mut(); MAX_PIPES];
    let state = (*link).dc.as_ref().unwrap().current_state;
    let mut count = 0u8;
    link_get_master_pipes_with_dpms_on(link, state, &mut count, pipes.as_mut_ptr());
    for i in 0..count as usize { link_set_dpms_off(pipes[i]); }
    for i in (0..count as usize).rev() {
        if (*link).skip_fallback_on_link_loss {
            (*pipes[i]).link_config.dp_link_settings.lane_count = (*link).verified_link_cap.lane_count;
            (*pipes[i]).link_config.dp_link_settings.link_rate = (*link).verified_link_cap.link_rate;
            (*pipes[i]).link_config.dp_link_settings.link_spread = (*link).verified_link_cap.link_spread;
        }
        link_set_dpms_on((*link).dc.as_ref().unwrap().current_state, pipes[i]);
    }
}

unsafe fn dp_handle_tunneling_irq(link: *mut dc_link) {
    let mut tunneling_status = 0u8;
    let retval = core_link_read_dpcd(link, DP_TUNNELING_STATUS, &mut tunneling_status, 1);
    if retval == DC_OK {
        DC_LOG_HW_HPD_IRQ!("%s: Got DP tunneling status on link %d status=0x%x", __func__, (*link).link_index, tunneling_status);
        if tunneling_status & DP_TUNNELING_BW_ALLOC_BITS_MASK != 0 { link_dp_dpia_handle_bw_alloc_status(link, tunneling_status); }
    }
    tunneling_status = DP_TUNNELING_IRQ;
    core_link_write_dpcd(link, DP_LINK_SERVICE_IRQ_VECTOR_ESI0, &mut tunneling_status, 1);
}

unsafe fn read_dpcd204h_on_irq_hpd(link: *mut dc_link, irq_data: *mut hpd_irq_data) {
    let mut dpcd_lane_status_updated = lane_align_status_updated::default();
    if core_link_read_dpcd(link, DP_LANE_ALIGN_STATUS_UPDATED, &mut dpcd_lane_status_updated.raw,
        core::mem::size_of_val(&dpcd_lane_status_updated.raw)) == DC_OK {
        (*irq_data).bytes.lane_status_updated.bits.EQ_INTERLANE_ALIGN_DONE_128b_132b = dpcd_lane_status_updated.bits.EQ_INTERLANE_ALIGN_DONE_128b_132b;
        (*irq_data).bytes.lane_status_updated.bits.CDS_INTERLANE_ALIGN_DONE_128b_132b = dpcd_lane_status_updated.bits.CDS_INTERLANE_ALIGN_DONE_128b_132b;
    }
}

pub unsafe fn dp_read_hpd_rx_irq_data(link: *mut dc_link, irq_data: *mut hpd_irq_data) -> dc_status {
    static mut RETVAL: dc_status = DC_ERROR_UNEXPECTED;
    if (*link).dpcd_caps.dpcd_rev.raw < DPCD_REV_14 {
        RETVAL = core_link_read_dpcd(link, DP_SINK_COUNT, (*irq_data).raw.as_mut_ptr(), DP_SINK_STATUS - DP_SINK_COUNT + 1);
        if (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling {
            RETVAL = core_link_read_dpcd(link, DP_LINK_SERVICE_IRQ_VECTOR_ESI0, &mut (*irq_data).bytes.link_service_irq_esi0.raw, 1);
        }
    } else {
        let mut tmp = [0u8; DP_SINK_STATUS_ESI - DP_SINK_COUNT_ESI + 1];
        RETVAL = core_link_read_dpcd(link, DP_SINK_COUNT_ESI, tmp.as_mut_ptr(), tmp.len());
        if RETVAL != DC_OK { return RETVAL; }
        (*irq_data).bytes.sink_cnt.raw = tmp[DP_SINK_COUNT_ESI - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.device_service_irq.raw = tmp[DP_DEVICE_SERVICE_IRQ_VECTOR_ESI0 - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.lane01_status.raw = tmp[DP_LANE0_1_STATUS_ESI - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.lane23_status.raw = tmp[DP_LANE2_3_STATUS_ESI - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.lane_status_updated.raw = tmp[DP_LANE_ALIGN_STATUS_UPDATED_ESI - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.sink_status.raw = tmp[DP_SINK_STATUS_ESI - DP_SINK_COUNT_ESI];
        (*irq_data).bytes.link_service_irq_esi0.raw = tmp[DP_LINK_SERVICE_IRQ_VECTOR_ESI0 - DP_SINK_COUNT_ESI];
        if (*link).wa_flags.read_dpcd204h_on_irq_hpd { read_dpcd204h_on_irq_hpd(link, irq_data); }
    }
    RETVAL
}

pub unsafe fn dp_should_allow_hpd_rx_irq(link: *const dc_link) -> bool {
    (*link).cur_link_settings.lane_count != LANE_COUNT_UNKNOWN || is_dp_branch_device(link) || (*link).dpia_bw_alloc_config.bw_alloc_enabled
}

pub unsafe fn dp_handle_hpd_rx_irq(link: *mut dc_link, out_hpd_irq_dpcd_data: *mut hpd_irq_data, out_link_loss: *mut bool, defer_handling: bool, has_left_work: *mut bool) -> bool {
    let mut hpd_irq_dpcd_data = hpd_irq_data::default();
    let mut device_service_clear = device_service_irq::default();
    let mut replay_re_enable_needed = false;
    let mut replay_esd_detection_needed = false;
    if !out_link_loss.is_null() { *out_link_loss = false; }
    if !has_left_work.is_null() { *has_left_work = false; }
    DC_LOG_HW_HPD_IRQ!("%s: Got short pulse HPD on link %d\n", __func__, (*link).link_index);
    handle_hpd_irq_replay_sink(link, &mut replay_re_enable_needed, &mut replay_esd_detection_needed);
    let result = dp_read_hpd_rx_irq_data(link, &mut hpd_irq_dpcd_data);
    if !out_hpd_irq_dpcd_data.is_null() { *out_hpd_irq_dpcd_data = hpd_irq_dpcd_data; }
    if result != DC_OK { DC_LOG_HW_HPD_IRQ!("%s: DPCD read failed to obtain irq data\n", __func__); return false; }
    if hpd_irq_dpcd_data.bytes.device_service_irq.bits.AUTOMATED_TEST {
        if (*link).ep_type == DISPLAY_ENDPOINT_USB4_DPIA && !(*link).dc.as_ref().unwrap().config.enable_dpia_pre_training { (*link).skip_fallback_on_link_loss = true; }
        device_service_clear.bits.AUTOMATED_TEST = 1;
        core_link_write_dpcd(link, DP_DEVICE_SERVICE_IRQ_VECTOR, &mut device_service_clear.raw, core::mem::size_of_val(&device_service_clear.raw));
        device_service_clear.raw = 0;
        if defer_handling && !has_left_work.is_null() { *has_left_work = true; } else { dc_link_dp_handle_automated_test(link); }
        return false;
    }
    if !dp_should_allow_hpd_rx_irq(link) { DC_LOG_HW_HPD_IRQ!("%s: skipping HPD handling on %d\n", __func__, (*link).link_index); return false; }
    if handle_hpd_irq_psr_sink(link) { return true; }
    if hpd_irq_dpcd_data.bytes.device_service_irq.bits.UP_REQ_MSG_RDY { if defer_handling && !has_left_work.is_null() { *has_left_work = true; } return true; }
    if hpd_irq_dpcd_data.bytes.device_service_irq.bits.DOWN_REP_MSG_RDY { if defer_handling && !has_left_work.is_null() { *has_left_work = true; } return false; }
    let mut status = false;
    if (*link).connector_signal != SIGNAL_TYPE_EDP || replay_esd_detection_needed {
        if dp_parse_link_loss_status(link, &mut hpd_irq_dpcd_data) {
            CONN_DATA_LINK_LOSS!(link, hpd_irq_dpcd_data.raw, core::mem::size_of_val(&hpd_irq_dpcd_data), "Status: ");
            if defer_handling && !has_left_work.is_null() { *has_left_work = true; } else { dp_handle_link_loss(link); }
            if !out_link_loss.is_null() { *out_link_loss = true; }
            dp_trace_link_loss_increment(link);
        }
    }
    if (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling && hpd_irq_dpcd_data.bytes.link_service_irq_esi0.bits.DP_LINK_TUNNELING_IRQ { dp_handle_tunneling_irq(link); }
    if (*link).type == dc_connection_sst_branch && hpd_irq_dpcd_data.bytes.sink_cnt.bits.SINK_COUNT != (*link).dpcd_sink_count { status = true; }
    if replay_re_enable_needed { let mut allow_active = true; edp_set_replay_allow_active(link, &mut allow_active, true, false, core::ptr::null_mut()); }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
