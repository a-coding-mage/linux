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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

/* FILE POLICY AND INTENDED USAGE:
 * This file implements basic dp phy functionality such as enable/disable phy
 * output and set lane/drive settings. This file is responsible for maintaining
 * and update software state representing current phy status such as current
 * link settings.
 */

// C dependencies supplied by the surrounding translation unit:
// link_dp_phy.h, link_dpcd.h, link_dp_training.h, link_dp_capability.h,
// clk_mgr.h, resource.h, link_enc_cfg.h, atomfirmware.h

pub unsafe fn dpcd_write_rx_power_ctrl(link: *mut dc_link, on: bool) {
    let state: u8 = if on { DP_POWER_STATE_D0 } else { DP_POWER_STATE_D3 };

    if (*link).sync_lt_in_progress {
        return;
    }

    core_link_write_dpcd(link, DP_SET_POWER, &state as *const u8, core::mem::size_of::<u8>());
}

pub unsafe fn dp_enable_link_phy(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
    clock_source: clock_source_id,
    link_settings: *const dc_link_settings,
) {
    (*link).cur_link_settings = *link_settings;
    ((*(*link).dc).hwss.enable_dp_link_output)(link, link_res, signal, clock_source, link_settings);
    dpcd_write_rx_power_ctrl(link, true);
}

pub unsafe fn dp_disable_link_phy(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
) {
    let dc: *mut dc = (*link).ctx.dc;

    if !(*link).wa_flags.dp_keep_receiver_powered
        && !(*link).skip_implict_edp_power_control
        && (*link).type_ != dc_connection_none
    {
        dpcd_write_rx_power_ctrl(link, false);
    }

    ((*dc).hwss.disable_link_output)(link, link_res, signal);
    // Clear current link setting.
    core::ptr::write_bytes(
        &mut (*link).cur_link_settings as *mut _,
        0,
        core::mem::size_of_val(&(*link).cur_link_settings),
    );

    if let Some(notify_link_rate_change) = (*(*dc).clk_mgr).funcs.notify_link_rate_change {
        notify_link_rate_change((*dc).clk_mgr, link);
    }
}

#[inline]
unsafe fn is_immediate_downstream(link: *mut dc_link, offset: u32) -> bool {
    dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt) == offset
}

pub unsafe fn dp_set_hw_lane_settings(
    link: *mut dc_link,
    link_res: *const link_resource,
    link_settings: *const link_training_settings,
    offset: u32,
) {
    let link_hwss: *const link_hwss = get_link_hwss(link, link_res);

    // Don't return here if using FIXED_VS link HWSS and encoding is 128b/132b
    if (*link_settings).lttpr_mode == LTTPR_MODE_NON_TRANSPARENT
        && !is_immediate_downstream(link, offset)
        && (!(((*link).chip_caps & AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK)
            == AMD_EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN)
            || link_dp_get_encoding_format(&(*link_settings).link_settings)
                == DP_8b_10b_ENCODING)
    {
        return;
    }

    if let Some(set_dp_lane_settings) = (*link_hwss).ext.set_dp_lane_settings {
        set_dp_lane_settings(
            link,
            link_res,
            &(*link_settings).link_settings,
            (*link_settings).hw_lane_settings,
        );
    }

    core::ptr::copy_nonoverlapping(
        (*link_settings).hw_lane_settings.as_ptr(),
        (*link).cur_lane_setting.as_mut_ptr(),
        (*link).cur_lane_setting.len(),
    );
}

pub unsafe fn dp_set_drive_settings(
    link: *mut dc_link,
    link_res: *const link_resource,
    lt_settings: *mut link_training_settings,
) {
    // program ASIC PHY settings
    dp_set_hw_lane_settings(link, link_res, lt_settings, DPRX);

    dp_hw_to_dpcd_lane_settings(
        lt_settings,
        (*lt_settings).hw_lane_settings,
        (*lt_settings).dpcd_lane_settings,
    );

    // Notify DP sink the PHY settings from source
    dpcd_set_lane_settings(link, lt_settings, DPRX);
}

pub unsafe fn dp_set_fec_ready(
    link: *mut dc_link,
    link_res: *const link_resource,
    ready: bool,
) -> dc_status {
    /* FEC has to be "set ready" before the link training.
     * The policy is to always train with FEC
     * if the sink supports it and leave it enabled on link.
     * If FEC is not supported, disable it.
     */
    let mut link_enc: *mut link_encoder = (*link_res).dio_link_enc;
    let mut status = DC_OK;
    let mut fec_config: u8 = 0;

    if !(*link).dc.config.unify_link_enc_assignment {
        link_enc = link_enc_cfg_get_link_enc(link);
    }
    ASSERT(!link_enc.is_null());
    if (*(*link_enc).funcs).fec_set_ready.is_none() {
        return DC_NOT_SUPPORTED;
    }

    if ready && dp_should_enable_fec(link) {
        fec_config = 1;
        status = core_link_write_dpcd(link, DP_FEC_CONFIGURATION, &fec_config, core::mem::size_of::<u8>());

        if status == DC_OK {
            ((*(*link_enc).funcs).fec_set_ready.unwrap())(link_enc, true);
            (*link).fec_state = dc_link_fec_ready;
        }
    } else if (*link).fec_state == dc_link_fec_ready {
        fec_config = 0;
        if (*link).type_ != dc_connection_none {
            core_link_write_dpcd(link, DP_FEC_CONFIGURATION, &fec_config, core::mem::size_of::<u8>());
        }

        ((*(*link_enc).funcs).fec_set_ready.unwrap())(link_enc, false);
        (*link).fec_state = dc_link_fec_not_ready;
    }

    status
}

pub unsafe fn dp_set_fec_enable(link: *mut dc_link, link_res: *const link_resource, enable: bool) {
    let mut link_enc: *mut link_encoder = (*link_res).dio_link_enc;

    if !(*link).dc.config.unify_link_enc_assignment {
        link_enc = link_enc_cfg_get_link_enc(link);
    }

    if link_enc.is_null() || (*link_enc).funcs.is_null() || (*(*link_enc).funcs).fec_set_enable.is_none() {
        return;
    }

    if enable && dp_should_enable_fec(link) {
        if (*link).fec_state == dc_link_fec_ready {
            /* According to DP spec, FEC enable sequence can first
             * be transmitted anytime after 1000 LL codes have been
             * transmitted on the link after link training
             * completion. Using 1 lane RBR should have the maximum
             * time for transmitting 1000 LL codes which is 6.173 us.
             * So use 7 microseconds delay instead.
             */
            udelay(7);
            ((*(*link_enc).funcs).fec_set_enable.unwrap())(link_enc, true);
            (*link).fec_state = dc_link_fec_enabled;
        }
    } else if (*link).fec_state == dc_link_fec_enabled {
        ((*(*link_enc).funcs).fec_set_enable.unwrap())(link_enc, false);
        (*link).fec_state = dc_link_fec_ready;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
