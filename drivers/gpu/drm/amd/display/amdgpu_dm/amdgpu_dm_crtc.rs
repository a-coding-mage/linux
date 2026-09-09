// SPDX-License-Identifier: MIT
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

// C header dependencies are supplied by the surrounding kernel translation.

const HPD_DETECTION_PERIOD_US: u32 = 2_000_000;
const HPD_DETECTION_TIME_US: u32 = 100_000;

pub unsafe fn amdgpu_dm_crtc_handle_vblank(acrtc: *mut amdgpu_crtc) {
    let crtc = &mut (*acrtc).base as *mut drm_crtc;
    let dev = (*crtc).dev;
    let mut flags: c_ulong = 0;
    drm_crtc_handle_vblank(crtc);
    spin_lock_irqsave(&mut (*dev).event_lock, &mut flags);
    // Send completion event for cursor-only commits
    if !(*acrtc).event.is_null() && (*acrtc).pflip_status != AMDGPU_FLIP_SUBMITTED {
        drm_crtc_send_vblank_event(crtc, (*acrtc).event);
        drm_crtc_vblank_put(crtc);
        (*acrtc).event = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&mut (*dev).event_lock, flags);
}

pub unsafe fn amdgpu_dm_crtc_modeset_required(
    crtc_state: *mut drm_crtc_state,
    _new_stream: *mut dc_stream_state,
    _old_stream: *mut dc_stream_state,
) -> bool {
    (*crtc_state).active && drm_atomic_crtc_needs_modeset(crtc_state)
}

pub unsafe fn amdgpu_dm_crtc_vrr_active_irq(acrtc: *mut amdgpu_crtc) -> bool {
    (*acrtc).dm_irq_params.freesync_config.state == VRR_STATE_ACTIVE_VARIABLE ||
        (*acrtc).dm_irq_params.freesync_config.state == VRR_STATE_ACTIVE_FIXED
}

pub unsafe fn amdgpu_dm_crtc_set_vupdate_irq(crtc: *mut drm_crtc, enable: bool) -> c_int {
    let acrtc = to_amdgpu_crtc(crtc);
    let adev = drm_to_adev((*crtc).dev);
    if (*acrtc).otg_inst == -1 { return 0; }
    let irq_source = IRQ_TYPE_VUPDATE + (*acrtc).otg_inst;
    let rc = if dc_interrupt_set((*adev).dm.dc, irq_source, enable) { 0 } else { -EBUSY };
    DRM_DEBUG_VBL!("crtc {} - vupdate irq {}abling: r={}\n", (*acrtc).crtc_id, if enable { "en" } else { "dis" }, rc);
    rc
}

pub unsafe fn amdgpu_dm_crtc_vrr_active(dm_state: *const dm_crtc_state) -> bool {
    (*dm_state).freesync_config.state == VRR_STATE_ACTIVE_VARIABLE ||
        (*dm_state).freesync_config.state == VRR_STATE_ACTIVE_FIXED
}

/** amdgpu_dm_crtc_set_static_screen_optimze() - Toggle static screen optimizations. */
pub unsafe fn amdgpu_dm_crtc_set_static_screen_optimze(
    dm: *mut amdgpu_display_manager, stream: *mut dc_stream_state,
    sso_enable: bool, allow_sr_entry: bool,
) {
    let link = (*stream).link;
    let set_vsync_event = !sso_enable;
    if sso_enable && !allow_sr_entry { return; }
    amdgpu_dm_replay_set_event(dm, stream, set_vsync_event, replay_event_vsync, set_vsync_event);
    if (*link).psr_settings.psr_version < DC_PSR_VERSION_SU_1 {
        amdgpu_dm_psr_set_event(dm, stream, set_vsync_event, psr_event_vsync, set_vsync_event);
    }
}

pub unsafe fn amdgpu_dm_is_headless(adev: *mut amdgpu_device) -> bool {
    if adev.is_null() { return true; }
    let dev = (*adev).dm.ddev;
    let mut iter = core::mem::zeroed::<drm_connector_list_iter>();
    let mut connector: *mut drm_connector = core::ptr::null_mut();
    let mut is_headless = true;
    drm_connector_list_iter_begin(dev, &mut iter);
    while drm_for_each_connector_iter(&mut connector, &mut iter) {
        if (*connector).connector_type == DRM_MODE_CONNECTOR_WRITEBACK { continue; }
        if (*connector).status == connector_status_connected { is_headless = false; break; }
    }
    drm_connector_list_iter_end(&mut iter);
    is_headless
}

// The following worker and vblank implementation retain the kernel workqueue
// and synchronization behavior; referenced types/functions are external.
unsafe fn amdgpu_dm_idle_worker(work: *mut work_struct) {
    let idle_work = container_of!(work, idle_workqueue, work);
    (*(*idle_work).dm).idle_workqueue.running = true;
    while (*idle_work).enable {
        fsleep(HPD_DETECTION_PERIOD_US);
        mutex_lock(&mut (*(*idle_work).dm).dc_lock);
        if !(*(*idle_work).dm).dc.idle_optimizations_allowed { mutex_unlock(&mut (*(*idle_work).dm).dc_lock); break; }
        dc_allow_idle_optimizations((*idle_work).dm.dc, false);
        mutex_unlock(&mut (*(*idle_work).dm).dc_lock);
        fsleep(HPD_DETECTION_TIME_US);
        mutex_lock(&mut (*(*idle_work).dm).dc_lock);
        if !amdgpu_dm_is_headless((*idle_work).dm.adev) && !amdgpu_dm_psr_is_active_allowed((*idle_work).dm) {
            mutex_unlock(&mut (*(*idle_work).dm).dc_lock); break;
        }
        if (*idle_work).enable {
            dc_post_update_surfaces_to_stream((*idle_work).dm.dc);
            dc_allow_idle_optimizations((*idle_work).dm.dc, true);
        }
        mutex_unlock(&mut (*(*idle_work).dm).dc_lock);
    }
    (*(*idle_work).dm).idle_workqueue.running = false;
}

pub unsafe fn idle_create_workqueue(adev: *mut amdgpu_device) -> *mut idle_workqueue {
    let idle_work = kzalloc_obj::<idle_workqueue>();
    if idle_work.is_null() { return core::ptr::null_mut(); }
    (*idle_work).dm = &mut (*adev).dm;
    (*idle_work).enable = false;
    (*idle_work).running = false;
    INIT_WORK!(&mut (*idle_work).work, amdgpu_dm_idle_worker);
    idle_work
}

unsafe fn amdgpu_dm_crtc_vblank_control_worker(work: *mut work_struct) {
    let vblank_work = container_of!(work, vblank_control_work, work);
    let dm = (*vblank_work).dm;
    mutex_lock(&mut (*dm).dc_lock);
    if (*vblank_work).enable {
        (*dm).active_vblank_irq_count += 1;
        amdgpu_dm_ism_commit_event(&mut (*(*vblank_work).acrtc).ism, DM_ISM_EVENT_EXIT_IDLE_REQUESTED);
    } else {
        if (*dm).active_vblank_irq_count > 0 { (*dm).active_vblank_irq_count -= 1; }
        amdgpu_dm_ism_commit_event(&mut (*(*vblank_work).acrtc).ism, DM_ISM_EVENT_ENTER_IDLE_REQUESTED);
    }
    mutex_unlock(&mut (*dm).dc_lock);
    dc_stream_release((*vblank_work).stream);
    kfree(vblank_work);
}

unsafe fn amdgpu_dm_crtc_set_vblank(crtc: *mut drm_crtc, enable: bool) -> c_int {
    let acrtc = to_amdgpu_crtc(crtc);
    let adev = drm_to_adev((*crtc).dev);
    let acrtc_state = to_dm_crtc_state((*crtc).state);
    let dm = &mut (*adev).dm;
    let irq_type = amdgpu_display_crtc_idx_to_irq_type(adev, (*acrtc).crtc_id);
    if enable && !(*crtc).enabled { drm_dbg_vbl!((*crtc).dev, "Reject vblank enable on unconfigured CRTC {} (enabled={})\n", (*acrtc).crtc_id, (*crtc).enabled); return -EINVAL; }
    let mut rc = 0;
    if amdgpu_ip_version(adev, DCE_HWIP, 0) != 0 {
        rc = if enable { amdgpu_irq_get(adev, &mut (*adev).vupdate_irq, irq_type) } else { amdgpu_irq_put(adev, &mut (*adev).vupdate_irq, irq_type) };
    } else if dc_supports_vrr((*dm).dc.ctx.dce_version) {
        if enable && amdgpu_dm_crtc_vrr_active(acrtc_state) { rc = amdgpu_dm_crtc_set_vupdate_irq(crtc, true); }
        if !enable { rc = amdgpu_dm_crtc_set_vupdate_irq(crtc, false); }
    }
    if rc != 0 { return rc; }
    if amdgpu_ip_version(adev, DCE_HWIP, 0) == 0 {
        rc = if enable { amdgpu_irq_get(adev, &mut (*adev).crtc_irq, irq_type) } else { amdgpu_irq_put(adev, &mut (*adev).crtc_irq, irq_type) };
        if rc != 0 { return rc; }
        rc = if enable { amdgpu_irq_get(adev, &mut (*adev).pageflip_irq, irq_type) } else { amdgpu_irq_put(adev, &mut (*adev).pageflip_irq, irq_type) };
        if rc != 0 { return rc; }
    }
    if amdgpu_in_reset(adev) { return 0; }
    if !dm.vblank_control_workqueue.is_null() {
        let work = kzalloc_obj::<vblank_control_work>();
        if work.is_null() { return -ENOMEM; }
        INIT_WORK!(&mut (*work).work, amdgpu_dm_crtc_vblank_control_worker);
        (*work).dm = dm; (*work).acrtc = acrtc; (*work).enable = enable;
        (*work).stream = (*acrtc_state).stream;
        if !(*work).stream.is_null() { dc_stream_retain((*work).stream); }
        queue_work(dm.vblank_control_workqueue, &mut (*work).work);
    }
    0
}

pub unsafe fn amdgpu_dm_crtc_enable_vblank(crtc: *mut drm_crtc) -> c_int { amdgpu_dm_crtc_set_vblank(crtc, true) }
pub unsafe fn amdgpu_dm_crtc_disable_vblank(crtc: *mut drm_crtc) { amdgpu_dm_crtc_set_vblank(crtc, false); }

unsafe fn amdgpu_dm_crtc_destroy_state(_crtc: *mut drm_crtc, state: *mut drm_crtc_state) {
    let cur = to_dm_crtc_state(state);
    // TODO Destroy dc_stream objects as stream object is flattened
    if !(*cur).stream.is_null() { dc_stream_release((*cur).stream); }
    __drm_atomic_helper_crtc_destroy_state(state);
    kfree(state as *mut c_void);
}

unsafe fn amdgpu_dm_crtc_duplicate_state(crtc: *mut drm_crtc) -> *mut drm_crtc_state {
    let cur = to_dm_crtc_state((*crtc).state);
    if (*crtc).state.is_null() || WARN_ON!((*crtc).state.is_null()) { return core::ptr::null_mut(); }
    let state = kzalloc_obj::<dm_crtc_state>();
    if state.is_null() { return core::ptr::null_mut(); }
    __drm_atomic_helper_crtc_duplicate_state(crtc, &mut (*state).base);
    if !(*cur).stream.is_null() { (*state).stream = (*cur).stream; dc_stream_retain((*state).stream); }
    (*state).active_planes = (*cur).active_planes; (*state).vrr_infopacket = (*cur).vrr_infopacket;
    (*state).abm_level = (*cur).abm_level; (*state).vrr_supported = (*cur).vrr_supported;
    (*state).freesync_config = (*cur).freesync_config; (*state).cm_has_degamma = (*cur).cm_has_degamma;
    (*state).cm_is_degamma_srgb = (*cur).cm_is_degamma_srgb; (*state).regamma_tf = (*cur).regamma_tf;
    (*state).crc_skip_count = (*cur).crc_skip_count; (*state).mpo_requested = (*cur).mpo_requested;
    (*state).cursor_mode = (*cur).cursor_mode;
    &mut (*state).base
}

unsafe fn amdgpu_dm_crtc_destroy(crtc: *mut drm_crtc) { drm_crtc_cleanup(crtc); kfree(crtc as *mut c_void); }

unsafe fn amdgpu_dm_crtc_reset_state(crtc: *mut drm_crtc) {
    let state = kzalloc_obj::<dm_crtc_state>();
    if state.is_null() { return; }
    if !(*crtc).state.is_null() { amdgpu_dm_crtc_destroy_state(crtc, (*crtc).state); }
    __drm_atomic_helper_crtc_reset(crtc, &mut (*state).base);
}

unsafe fn amdgpu_dm_crtc_helper_disable(_crtc: *mut drm_crtc) {}

unsafe fn amdgpu_dm_crtc_count_crtc_active_planes(new_crtc_state: *mut drm_crtc_state) -> c_int {
    let state = (*new_crtc_state).state; let mut num_active = 0;
    let mut plane: *mut drm_plane = core::ptr::null_mut();
    drm_for_each_plane_mask!(plane, (*state).dev, (*new_crtc_state).plane_mask, {
        if (*plane).type_ == DRM_PLANE_TYPE_CURSOR { continue; }
        let new_plane_state = drm_atomic_get_new_plane_state(state, plane);
        num_active += if new_plane_state.is_null() { 1 } else { !(*new_plane_state).fb.is_null() as c_int };
    });
    num_active
}

unsafe fn amdgpu_dm_crtc_update_crtc_active_planes(_crtc: *mut drm_crtc, new_crtc_state: *mut drm_crtc_state) {
    let dm_state = to_dm_crtc_state(new_crtc_state); (*dm_state).active_planes = 0;
    if !(*dm_state).stream.is_null() { (*dm_state).active_planes = amdgpu_dm_crtc_count_crtc_active_planes(new_crtc_state); }
}

unsafe fn amdgpu_dm_crtc_helper_mode_fixup(_crtc: *mut drm_crtc, _mode: *const drm_display_mode, _adjusted_mode: *mut drm_display_mode) -> bool { true }

unsafe fn amdgpu_dm_crtc_helper_atomic_check(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) -> c_int {
    let crtc_state = drm_atomic_get_new_crtc_state(state, crtc); let adev = drm_to_adev((*crtc).dev);
    let dm_state = to_dm_crtc_state(crtc_state); let mut ret = -EINVAL;
    trace_amdgpu_dm_crtc_atomic_check(crtc_state); amdgpu_dm_crtc_update_crtc_active_planes(crtc, crtc_state);
    if WARN_ON!(unlikely!((*dm_state).stream.is_null() && amdgpu_dm_crtc_modeset_required(crtc_state, core::ptr::null_mut(), (*dm_state).stream))) { return ret; }
    if (*crtc_state).enable && ((*crtc_state).plane_mask & drm_plane_mask((*crtc).primary)) == 0 { return -EINVAL; }
    if (*crtc_state).async_flip && (*dm_state).update_type != UPDATE_TYPE_FAST { return -EINVAL; }
    if !(*state).legacy_cursor_update && amdgpu_dm_crtc_vrr_active(dm_state) {
        let primary_state = drm_atomic_get_plane_state(state, (*crtc).primary); if IS_ERR!(primary_state) { return PTR_ERR!(primary_state); }
    }
    if (*dm_state).stream.is_null() { return 0; }
    if dc_validate_stream((*adev).dm.dc, (*dm_state).stream) == DC_OK { return 0; }
    ret
}

#[cfg(feature = "debug_fs")]
unsafe fn amdgpu_dm_crtc_late_register(crtc: *mut drm_crtc) -> c_int {
    crtc_debugfs_init(crtc);
    0
}

#[cfg(feature = "amd_private_color")]
unsafe fn dm_crtc_additional_color_mgmt(crtc: *mut drm_crtc) {
    let adev = drm_to_adev((*crtc).dev);
    if (*adev).dm.dc.caps.color.mpc.ogam_ram {
        drm_object_attach_property(&mut (*crtc).base, (*adev).mode_info.regamma_tf_property,
                                   AMDGPU_TRANSFER_FUNCTION_DEFAULT);
    }
}

#[cfg(feature = "amd_private_color")]
unsafe fn amdgpu_dm_atomic_crtc_set_property(crtc: *mut drm_crtc, state: *mut drm_crtc_state,
                                              property: *mut drm_property, val: u64) -> c_int {
    let adev = drm_to_adev((*crtc).dev); let acrtc_state = to_dm_crtc_state(state);
    if property == (*adev).mode_info.regamma_tf_property {
        if (*acrtc_state).regamma_tf != val { (*acrtc_state).regamma_tf = val; (*acrtc_state).base.color_mgmt_changed |= 1; }
    } else { return -EINVAL; }
    0
}

#[cfg(feature = "amd_private_color")]
unsafe fn amdgpu_dm_atomic_crtc_get_property(crtc: *mut drm_crtc, state: *const drm_crtc_state,
                                              property: *mut drm_property, val: *mut u64) -> c_int {
    let adev = drm_to_adev((*crtc).dev); let acrtc_state = to_dm_crtc_state(state as *mut drm_crtc_state);
    if property != (*adev).mode_info.regamma_tf_property { return -EINVAL; }
    *val = (*acrtc_state).regamma_tf; 0
}

// Implemented only the options currently available for the driver.
static amdgpu_dm_crtc_funcs: drm_crtc_funcs = drm_crtc_funcs {
    reset: Some(amdgpu_dm_crtc_reset_state), destroy: Some(amdgpu_dm_crtc_destroy),
    set_config: Some(drm_atomic_helper_set_config), page_flip: Some(drm_atomic_helper_page_flip),
    atomic_duplicate_state: Some(amdgpu_dm_crtc_duplicate_state),
    atomic_destroy_state: Some(amdgpu_dm_crtc_destroy_state),
    set_crc_source: Some(amdgpu_dm_crtc_set_crc_source),
    verify_crc_source: Some(amdgpu_dm_crtc_verify_crc_source),
    get_crc_sources: Some(amdgpu_dm_crtc_get_crc_sources),
    get_vblank_counter: Some(amdgpu_get_vblank_counter_kms),
    enable_vblank: Some(amdgpu_dm_crtc_enable_vblank), disable_vblank: Some(amdgpu_dm_crtc_disable_vblank),
    get_vblank_timestamp: Some(drm_crtc_vblank_helper_get_vblank_timestamp),
};

static amdgpu_dm_crtc_helper_funcs: drm_crtc_helper_funcs = drm_crtc_helper_funcs {
    disable: Some(amdgpu_dm_crtc_helper_disable), atomic_check: Some(amdgpu_dm_crtc_helper_atomic_check),
    mode_fixup: Some(amdgpu_dm_crtc_helper_mode_fixup), get_scanout_position: Some(amdgpu_crtc_get_scanout_position),
};

static mut default_ism_config: amdgpu_dm_ism_config = amdgpu_dm_ism_config {
    filter_num_frames: 4, filter_history_size: 8, filter_entry_count: 1,
    activation_num_delay_frames: 4, filter_old_history_threshold: 0, sso_num_frames: 11,
};

pub unsafe fn amdgpu_dm_crtc_init(dm: *mut amdgpu_display_manager, plane: *mut drm_plane, crtc_index: u32) -> c_int {
    let cursor_plane = kzalloc_obj::<drm_plane>(); if cursor_plane.is_null() { return -ENOMEM; }
    (*cursor_plane).type_ = DRM_PLANE_TYPE_CURSOR;
    let mut res = amdgpu_dm_plane_init(dm, cursor_plane, 0, core::ptr::null_mut());
    let acrtc = kzalloc_obj::<amdgpu_crtc>(); if acrtc.is_null() { kfree(cursor_plane as *mut c_void); return res; }
    res = drm_crtc_init_with_planes((*dm).ddev, &mut (*acrtc).base, plane, cursor_plane, core::ptr::null(), core::ptr::null_mut());
    if res != 0 { kfree(acrtc as *mut c_void); kfree(cursor_plane as *mut c_void); return res; }
    amdgpu_dm_ism_init(&mut (*acrtc).ism, &default_ism_config);
    (*acrtc).crtc_id = crtc_index; (*acrtc).base.enabled = false; (*acrtc).otg_inst = -1;
    (*(*dm).adev).mode_info.crtcs[crtc_index as usize] = acrtc;
    let has_degamma = (*(*dm).adev).dm.dc.caps.color.dpp.dcn_arch && (*(*dm).adev).dm.dc.ctx.dce_version != DCN_VERSION_4_01;
    drm_crtc_enable_color_mgmt(&mut (*acrtc).base, if has_degamma { MAX_COLOR_LUT_ENTRIES } else { 0 }, true, MAX_COLOR_LUT_ENTRIES);
    drm_mode_crtc_set_gamma_size(&mut (*acrtc).base, MAX_COLOR_LEGACY_LUT_ENTRIES);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
