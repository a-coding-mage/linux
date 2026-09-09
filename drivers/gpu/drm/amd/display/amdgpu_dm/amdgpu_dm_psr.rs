// SPDX-License-Identifier: MIT
/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

extern "C" {
    static mut amdgpu_dc_debug_mask: u32;
    static mut amdgpu_dc_feature_mask: u32;

    fn is_psr_su_specific_panel(link: *mut dc_link) -> bool;
    fn dc_get_edp_link_panel_inst(dc: *mut dc, link: *mut dc_link, panel_inst: *mut u32) -> bool;
    fn mod_power_get_psr_event(power_module: *mut power_module, stream: *mut dc_stream_state, events: *mut u32) -> bool;
    fn mod_power_set_psr_event(power_module: *mut power_module, stream: *mut dc_stream_state, set_event: bool, event: psr_event, wait_for_disable: bool) -> bool;
}

// External types and constants are supplied by the translated dependency headers.
#[allow(non_camel_case_types)]
type psr_event = u32;

unsafe fn link_supports_psrsu(link: *mut dc_link) -> bool {
    let dc = (*(*link).ctx).dc;

    if !(*dc).caps.dmcub_support { return false; }
    if (*dc).ctx.dce_version < DCN_VERSION_3_1 { return false; }
    if !is_psr_su_specific_panel(link) { return false; }
    if !(*link).dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP ||
       !(*link).dpcd_caps.psr_info.psr_dpcd_caps.bits.Y_COORDINATE_REQUIRED { return false; }
    if (*link).dpcd_caps.psr_info.psr_dpcd_caps.bits.SU_GRANULARITY_REQUIRED &&
       !(*link).dpcd_caps.psr_info.psr2_su_y_granularity_cap { return false; }
    if amdgpu_dc_debug_mask & DC_DISABLE_PSR_SU != 0 { return false; }

    /* Temporarily disable PSR-SU to avoid glitches */
    false
}

unsafe fn amdgpu_dm_psr_fill_caps(link: *mut dc_link, caps: *mut psr_caps) {
    let dpcd_caps = &(*link).dpcd_caps;
    let mut power_opts: u32 = 0;

    if amdgpu_dc_feature_mask & DC_PSR_ALLOW_SMU_OPT != 0 { power_opts |= psr_power_opt_smu_opt_static_screen; }
    power_opts |= psr_power_opt_z10_static_screen;

    if (*link).psr_settings.psr_version == DC_PSR_VERSION_1 { (*caps).psr_version = 1; }
    else if (*link).psr_settings.psr_version == DC_PSR_VERSION_SU_1 { (*caps).psr_version = 2; }

    (*caps).psr_rfb_setup_time = (6 - dpcd_caps.psr_info.psr_dpcd_caps.bits.PSR_SETUP_TIME) * 55;
    (*caps).psr_exit_link_training_required = !dpcd_caps.psr_info.psr_dpcd_caps.bits.LINK_TRAINING_ON_EXIT_NOT_REQUIRED;
    (*caps).edp_revision = dpcd_caps.edp_rev;
    (*caps).support_ver = dpcd_caps.psr_info.psr_version;
    (*caps).su_granularity_required = dpcd_caps.psr_info.psr_dpcd_caps.bits.SU_GRANULARITY_REQUIRED;
    (*caps).y_coordinate_required = dpcd_caps.psr_info.psr_dpcd_caps.bits.Y_COORDINATE_REQUIRED;
    (*caps).su_y_granularity = dpcd_caps.psr_info.psr2_su_y_granularity_cap;
    (*caps).alpm_cap = dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP;
    (*caps).standby_support = dpcd_caps.alpm_caps.bits.PM_STATE_2A_SUPPORT;
    (*caps).rate_control_caps = 0; /* TODO: read in rc caps from aux */
    (*caps).psr_power_opt_flag = power_opts;
}

pub unsafe fn amdgpu_dm_set_psr_caps(link: *mut dc_link, aconnector: *mut amdgpu_dm_connector) -> bool {
    let mut panel_inst: u32 = 0;
    if link.is_null() || aconnector.is_null() { return false; }
    let dc = (*(*link).ctx).dc;
    (*link).psr_settings.psr_version = DC_PSR_VERSION_UNSUPPORTED;
    if !(*dc).caps.dmub_caps.psr { return false; }
    if (*link).connector_signal & SIGNAL_TYPE_EDP == 0 { return false; }
    if (*link).type_ == dc_connection_none { return false; }
    if (*link).dpcd_caps.psr_info.psr_version == 0 { return false; }
    /*disable allow psr/psrsu/replay on eDP1*/
    if dc_get_edp_link_panel_inst(dc, link, &mut panel_inst) && panel_inst == 1 { return false; }
    if link_supports_psrsu(link) { (*link).psr_settings.psr_version = DC_PSR_VERSION_SU_1; }
    else { (*link).psr_settings.psr_version = DC_PSR_VERSION_1; }
    amdgpu_dm_psr_fill_caps(link, &mut (*aconnector).psr_caps);
    true
}

pub unsafe fn amdgpu_dm_psr_is_active_allowed(dm: *mut amdgpu_display_manager) -> bool {
    for i in 0..(*(*dm).dc).current_state.stream_count {
        let link = (*(*(*dm).dc).current_state.streams[i]).link;
        if link.is_null() { continue; }
        if (*link).psr_settings.psr_feature_enabled && (*link).psr_settings.psr_allow_active { return true; }
    }
    false
}

pub unsafe fn amdgpu_dm_psr_set_event(dm: *mut amdgpu_display_manager, stream: *mut dc_stream_state, set_event: bool, event: psr_event, wait_for_disable: bool) -> bool {
    let mut psr_events: u32 = 0;
    if stream.is_null() || (*stream).link.is_null() || !(*(*stream).link).psr_settings.psr_feature_enabled { return false; }
    if !mod_power_get_psr_event((*dm).power_module, stream, &mut psr_events) { return false; }
    if (psr_events & event) == (if set_event { event } else { 0 }) { return true; }
    mod_power_set_psr_event((*dm).power_module, stream, set_event, event, wait_for_disable)
}

// The following accessors are compiled only when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub unsafe fn amdgpu_dm_psr_get_dc_feature_mask() -> u32 { amdgpu_dc_feature_mask }
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub unsafe fn amdgpu_dm_psr_set_dc_feature_mask(feature_mask: u32) { amdgpu_dc_feature_mask = feature_mask; }
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub unsafe fn amdgpu_dm_psr_get_dc_debug_mask() -> u32 { amdgpu_dc_debug_mask }
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub unsafe fn amdgpu_dm_psr_set_dc_debug_mask(debug_mask: u32) { amdgpu_dc_debug_mask = debug_mask; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
