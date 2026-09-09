// SPDX-License-Identifier: MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding display subsystem are intentionally
// referenced here rather than reimplemented in this translation.

pub unsafe fn amdgpu_dm_link_supports_replay(
    link: *mut dc_link,
    aconnector: *mut amdgpu_dm_connector,
) -> bool {
    let state = to_dm_connector_state((*(*aconnector).base.state));
    let dpcd_caps = &(*link).dpcd_caps;
    let as_caps = &(*link).dpcd_caps.adaptive_sync_caps;

    if !state.freesync_capable {
        return false;
    }

    if !(*aconnector).vsdb_info.replay_mode {
        return false;
    }

    // Check the eDP version
    if dpcd_caps.edp_rev < EDP_REVISION_13 {
        return false;
    }

    if !dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP {
        return false;
    }

    // Check adaptive sync support cap
    if !as_caps.dp_adap_sync_caps.bits.ADAPTIVE_SYNC_SDP_SUPPORT {
        return false;
    }

    // Sink shall populate line deviation information
    if dpcd_caps.pr_info.pixel_deviation_per_line == 0
        || dpcd_caps.pr_info.max_deviation_line == 0
    {
        return false;
    }

    true
}

pub unsafe fn amdgpu_dm_set_replay_caps(
    link: *mut dc_link,
    aconnector: *mut amdgpu_dm_connector,
) -> bool {
    let mut pr_config: replay_config = core::mem::zeroed();
    let mut debug_flags: *mut replay_debug_flags = core::ptr::null_mut();
    let dc = (*(*link).ctx).dc;

    // If Replay is already set to support, return true to skip checks
    if (*link).replay_settings.config.replay_supported {
        return true;
    }

    if !dc_is_embedded_signal((*link).connector_signal) {
        return false;
    }

    if (*link).panel_config.psr.disallow_replay {
        return false;
    }

    if !amdgpu_dm_link_supports_replay(link, aconnector) {
        return false;
    }

    if (*dc).ctx.dmub_srv.is_null()
        || (*(*dc).ctx.dmub_srv).dmub.is_null()
        || !(*(*(*dc).ctx.dmub_srv).dmub).feature_caps.replay_supported
    {
        return false;
    }

    /* Mark Replay is supported in link and update related attributes
     * This flag presents DPCD caps & amd_vsdb caps satisfy replay requirement.
     */
    pr_config.replay_cap_support = true;
    // Mark Replay is supported in pr_config
    pr_config.replay_supported = true;
    pr_config.replay_enable_option = pr_enable_option_general_ui
        | pr_enable_option_static_screen
        | pr_enable_option_static_screen_coasting;
    pr_config.replay_power_opt_supported = replay_power_opt_smu_opt_static_screen
        | replay_power_opt_z10_static_screen;
    pr_config.replay_smu_opt_supported = false;
    pr_config.replay_support_fast_resync_in_ultra_sleep_mode =
        (*aconnector).max_vfreq >= 2 * (*aconnector).min_vfreq;
    pr_config.force_disable_desync_error_check = false;

    debug_flags = &mut pr_config.debug_flags as *mut _ as *mut replay_debug_flags;
    (*debug_flags).u32All = 0;
    (*debug_flags).bitfields.visual_confirm = (*dc).debug.visual_confirm == VISUAL_CONFIRM_REPLAY;
    (*debug_flags).bitfields.skip_crtc_disabled = (*dc).debug.replay_skip_crtc_disabled;

    init_replay_config(link, &mut pr_config);
    true
}

pub unsafe fn amdgpu_dm_link_setup_replay(
    stream: *mut dc_stream_state,
    vrr_params: *mut mod_vrr_params,
) -> bool {
    let link: *mut dc_link;
    let static_coasting_vtotal: u32;

    if stream.is_null() || (*stream).link.is_null() || vrr_params.is_null() {
        return false;
    }
    link = (*stream).link;
    if !(*link).replay_settings.config.replay_supported {
        return false;
    }
    if (*link).replay_settings.replay_feature_enabled {
        return true;
    }
    calculate_replay_link_off_frame_count(link, (*stream).timing.v_total, (*stream).timing.h_total);
    static_coasting_vtotal = mod_freesync_calc_v_total_from_refresh(
        stream, (*vrr_params).min_refresh_in_uhz);
    set_replay_coasting_vtotal(link, PR_COASTING_TYPE_NOM, (*stream).timing.v_total);
    set_replay_coasting_vtotal(link, PR_COASTING_TYPE_STATIC, static_coasting_vtotal);
    true
}

pub unsafe fn amdgpu_dm_replay_set_event(
    dm: *mut amdgpu_display_manager,
    stream: *mut dc_stream_state,
    set_event: bool,
    event: replay_event,
    wait_for_disable: bool,
) -> bool {
    let mut replay_events: u32 = 0;
    if stream.is_null() || (*stream).link.is_null()
        || !(*(*stream).link).replay_settings.replay_feature_enabled {
        return false;
    }
    if !mod_power_get_replay_event((*dm).power_module, stream, &mut replay_events) {
        return false;
    }
    if (replay_events & event as u32) == (if set_event { event as u32 } else { 0 }) {
        return true;
    }
    mod_power_set_replay_event((*dm).power_module, stream, set_event, event, wait_for_disable)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
