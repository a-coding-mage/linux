// SPDX-License-Identifier: MIT
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub unsafe fn amdgpu_dm_is_dc_timing_adjust_needed(
    old_state: *mut dm_crtc_state,
    new_state: *mut dm_crtc_state,
) -> bool {
    if (*(*new_state).stream).adjust.timing_adjust_pending {
        return true;
    }
    if (*new_state).freesync_config.state == VRR_STATE_ACTIVE_FIXED {
        return true;
    } else if amdgpu_dm_crtc_vrr_active(old_state) != amdgpu_dm_crtc_vrr_active(new_state) {
        return true;
    } else {
        false
    }
}

pub unsafe fn amdgpu_dm_is_timing_unchanged_for_freesync(
    old_crtc_state: *mut drm_crtc_state,
    new_crtc_state: *mut drm_crtc_state,
) -> bool {
    if old_crtc_state.is_null() || new_crtc_state.is_null() {
        return false;
    }

    let old_mode = &(*old_crtc_state).mode;
    let new_mode = &(*new_crtc_state).mode;

    if old_mode.clock == new_mode.clock
        && old_mode.hdisplay == new_mode.hdisplay
        && old_mode.vdisplay == new_mode.vdisplay
        && old_mode.htotal == new_mode.htotal
        && old_mode.vtotal != new_mode.vtotal
        && old_mode.hsync_start == new_mode.hsync_start
        && old_mode.vsync_start != new_mode.vsync_start
        && old_mode.hsync_end == new_mode.hsync_end
        && old_mode.vsync_end != new_mode.vsync_end
        && old_mode.hskew == new_mode.hskew
        && old_mode.vscan == new_mode.vscan
        && old_mode.vsync_end - old_mode.vsync_start
            == new_mode.vsync_end - new_mode.vsync_start
    {
        return true;
    }
    false
}

pub unsafe fn amdgpu_dm_set_freesync_fixed_config(dm_new_crtc_state: *mut dm_crtc_state) {
    let new_crtc_state = &mut (*dm_new_crtc_state).base;
    (*dm_new_crtc_state).freesync_config.state = VRR_STATE_ACTIVE_FIXED;

    let num: u64 = (new_crtc_state.mode.clock as u64) * 1000 * 1_000_000;
    let den: u64 = (new_crtc_state.mode.htotal as u64) * (new_crtc_state.mode.vtotal as u64);
    let res = div_u64(num, den);
    (*dm_new_crtc_state).freesync_config.fixed_refresh_in_uhz = res;
}

pub unsafe fn amdgpu_dm_reset_freesync_config_for_crtc(new_crtc_state: *mut dm_crtc_state) {
    (*new_crtc_state).vrr_supported = false;
    core::ptr::write_bytes(
        &mut (*new_crtc_state).vrr_infopacket as *mut _,
        0,
        core::mem::size_of_val(&(*new_crtc_state).vrr_infopacket),
    );
}

pub unsafe fn amdgpu_dm_get_freesync_config_for_crtc(
    new_crtc_state: *mut dm_crtc_state,
    new_con_state: *mut dm_connector_state,
) {
    let mut config: mod_freesync_config = core::mem::zeroed();
    let connector = (*new_con_state).base.connector;
    let mode = &mut (*new_crtc_state).base.mode;
    let vrefresh = drm_mode_vrefresh(mode);
    let mut fs_vid_mode = false;

    if (*connector).connector_type == DRM_MODE_CONNECTOR_WRITEBACK {
        return;
    }

    let aconnector = to_amdgpu_dm_connector(connector);
    (*new_crtc_state).vrr_supported = (*new_con_state).freesync_capable
        && vrefresh >= (*aconnector).min_vfreq
        && vrefresh <= (*aconnector).max_vfreq;

    if (*new_crtc_state).vrr_supported {
        (*(*new_crtc_state).stream).ignore_msa_timing_param = true;
        fs_vid_mode = (*new_crtc_state).freesync_config.state == VRR_STATE_ACTIVE_FIXED;
        config.min_refresh_in_uhz = (*aconnector).min_vfreq * 1_000_000;
        config.max_refresh_in_uhz = (*aconnector).max_vfreq * 1_000_000;
        config.vsif_supported = true;
        config.btr = true;

        if fs_vid_mode {
            config.state = VRR_STATE_ACTIVE_FIXED;
            config.fixed_refresh_in_uhz = (*new_crtc_state).freesync_config.fixed_refresh_in_uhz;
        } else if (*new_crtc_state).base.vrr_enabled {
            config.state = VRR_STATE_ACTIVE_VARIABLE;
        } else {
            config.state = VRR_STATE_INACTIVE;
        }
    } else {
        config.state = VRR_STATE_UNSUPPORTED;
    }
    (*new_crtc_state).freesync_config = config;
}

pub unsafe fn amdgpu_dm_update_freesync_state_on_stream(
    dm: *mut amdgpu_display_manager,
    new_crtc_state: *mut dm_crtc_state,
    new_stream: *mut dc_stream_state,
    surface: *mut dc_plane_state,
    flip_timestamp_in_us: u32,
) {
    let mut vrr_params: mod_vrr_params;
    let mut vrr_infopacket: dc_info_packet = core::mem::zeroed();
    let adev = (*dm).adev;
    let acrtc = to_amdgpu_crtc((*new_crtc_state).base.crtc);
    let mut flags: c_ulong = 0;
    let mut pack_sdp_v1_3 = false;
    let mut aconn: *mut amdgpu_dm_connector;
    let mut packet_type = PACKET_TYPE_VRR;

    if new_stream.is_null() || (*new_stream).timing.h_total == 0 || (*new_stream).timing.v_total == 0 {
        return;
    }

    spin_lock_irqsave(&mut (*adev_to_drm(adev)).event_lock, &mut flags);
    vrr_params = (*acrtc).dm_irq_params.vrr_params;
    if !surface.is_null() {
        mod_freesync_handle_preflip((*dm).freesync_module, surface, new_stream,
            flip_timestamp_in_us, &mut vrr_params);
        if (*adev).family < AMDGPU_FAMILY_AI && amdgpu_dm_crtc_vrr_active(new_crtc_state) {
            mod_freesync_handle_v_update((*dm).freesync_module, new_stream, &mut vrr_params);
            dc_stream_adjust_vmin_vmax((*dm).dc, (*new_crtc_state).stream, &mut vrr_params.adjust);
        }
    }
    aconn = (*new_stream).dm_stream_context as *mut amdgpu_dm_connector;
    if !aconn.is_null() && ((*aconn).as_type == FREESYNC_TYPE_PCON_IN_WHITELIST || (*aconn).vsdb_info.replay_mode) {
        pack_sdp_v1_3 = (*aconn).pack_sdp_v1_3;
        if (*aconn).vsdb_info.amd_vsdb_version == 1 { packet_type = PACKET_TYPE_FS_V1; }
        else if (*aconn).vsdb_info.amd_vsdb_version == 2 { packet_type = PACKET_TYPE_FS_V2; }
        else if (*aconn).vsdb_info.amd_vsdb_version == 3 { packet_type = PACKET_TYPE_FS_V3; }
        mod_build_adaptive_sync_infopacket(new_stream, (*aconn).as_type, core::ptr::null_mut(), &mut (*new_stream).adaptive_sync_infopacket);
    }
    mod_freesync_build_vrr_infopacket((*dm).freesync_module, new_stream, &vrr_params, packet_type,
        TRANSFER_FUNC_UNKNOWN, &mut vrr_infopacket, pack_sdp_v1_3);
    (*new_crtc_state).freesync_vrr_info_changed |=
        core::ptr::read((&(*new_crtc_state).vrr_infopacket as *const _ as *const u8)) !=
        core::ptr::read((&vrr_infopacket as *const _ as *const u8));
    (*acrtc).dm_irq_params.vrr_params = vrr_params;
    (*new_crtc_state).vrr_infopacket = vrr_infopacket;
    (*new_stream).vrr_infopacket = vrr_infopacket;
    (*new_stream).allow_freesync = mod_freesync_get_freesync_enabled(&vrr_params);
    spin_unlock_irqrestore(&mut (*adev_to_drm(adev)).event_lock, flags);
}

pub unsafe fn amdgpu_dm_update_stream_irq_parameters(dm: *mut amdgpu_display_manager, new_crtc_state: *mut dm_crtc_state) {
    let new_stream = (*new_crtc_state).stream;
    if new_stream.is_null() || (*new_stream).timing.h_total == 0 || (*new_stream).timing.v_total == 0 { return; }
    let mut vrr_params;
    let mut config = (*new_crtc_state).freesync_config;
    let adev = (*dm).adev;
    let acrtc = to_amdgpu_crtc((*new_crtc_state).base.crtc);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*adev_to_drm(adev)).event_lock, &mut flags);
    vrr_params = (*acrtc).dm_irq_params.vrr_params;
    if (*new_crtc_state).vrr_supported && config.min_refresh_in_uhz != 0 && config.max_refresh_in_uhz != 0 {
        if config.state == VRR_STATE_ACTIVE_FIXED && config.fixed_refresh_in_uhz != 0 &&
            (!drm_atomic_crtc_needs_modeset(&mut (*new_crtc_state).base) || (*new_crtc_state).freesync_config.state == VRR_STATE_ACTIVE_FIXED) {
            vrr_params.max_refresh_in_uhz = config.max_refresh_in_uhz;
            vrr_params.min_refresh_in_uhz = config.min_refresh_in_uhz;
            vrr_params.fixed_refresh_in_uhz = config.fixed_refresh_in_uhz;
            vrr_params.state = VRR_STATE_ACTIVE_FIXED;
        } else { config.state = if (*new_crtc_state).base.vrr_enabled { VRR_STATE_ACTIVE_VARIABLE } else { VRR_STATE_INACTIVE }; }
    } else { config.state = VRR_STATE_UNSUPPORTED; }
    mod_freesync_build_vrr_params((*dm).freesync_module, new_stream, &config, &mut vrr_params);
    (*new_crtc_state).freesync_config = config;
    (*acrtc).dm_irq_params.freesync_config = config;
    (*acrtc).dm_irq_params.active_planes = (*new_crtc_state).active_planes;
    (*acrtc).dm_irq_params.vrr_params = vrr_params;
    spin_unlock_irqrestore(&mut (*adev_to_drm(adev)).event_lock, flags);
}

pub unsafe fn amdgpu_dm_handle_vrr_transition(dm: *mut amdgpu_display_manager, old_state: *mut dm_crtc_state, new_state: *mut dm_crtc_state) {
    let adev = (*dm).adev;
    let old_vrr_active = amdgpu_dm_crtc_vrr_active(old_state);
    let new_vrr_active = amdgpu_dm_crtc_vrr_active(new_state);
    let vrr_gates_vupdate = amdgpu_ip_version(adev, DCE_HWIP, 0) == 0;
    if !old_vrr_active && new_vrr_active {
        if vrr_gates_vupdate { WARN_ON(amdgpu_dm_crtc_set_vupdate_irq((*new_state).base.crtc, true) != 0); }
        WARN_ON(drm_crtc_vblank_get((*new_state).base.crtc) != 0);
        let _lock = scoped_mutex_guard(&mut (*dm).dc_lock);
        dc_exit_ips_for_hw_access((*dm).dc);
        amdgpu_dm_psr_set_event(dm, (*new_state).stream, true, psr_event_vrr_transition, true);
        amdgpu_dm_replay_set_event(dm, (*new_state).stream, true, replay_event_vrr, true);
    } else if old_vrr_active && !new_vrr_active {
        if vrr_gates_vupdate { WARN_ON(amdgpu_dm_crtc_set_vupdate_irq((*new_state).base.crtc, false) != 0); }
        drm_crtc_vblank_put((*new_state).base.crtc);
        let _lock = scoped_mutex_guard(&mut (*dm).dc_lock);
        dc_exit_ips_for_hw_access((*dm).dc);
        amdgpu_dm_psr_set_event(dm, (*new_state).stream, false, psr_event_vrr_transition, false);
        amdgpu_dm_replay_set_event(dm, (*new_state).stream, false, replay_event_vrr, false);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
