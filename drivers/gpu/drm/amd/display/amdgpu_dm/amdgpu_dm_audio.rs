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
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding kernel/DRM translation.

unsafe fn amdgpu_dm_audio_component_get_eld(
    kdev: *mut device, port: i32, _pipe: i32, enabled: *mut bool,
    buf: *mut u8, max_bytes: i32,
) -> i32 {
    let dev = dev_get_drvdata(kdev);
    let adev = drm_to_adev(dev);
    let mut connector: *mut drm_connector;
    let mut conn_iter: drm_connector_list_iter;
    let aconnector: *mut amdgpu_dm_connector;
    let mut ret: i32 = 0;

    *enabled = false;
    mutex_lock(&mut (*(*adev).dm).audio_lock);
    drm_connector_list_iter_begin(dev, &mut conn_iter);
    drm_for_each_connector_iter!(connector, &mut conn_iter) {
        if (*connector).connector_type == DRM_MODE_CONNECTOR_WRITEBACK { continue; }
        aconnector = to_amdgpu_dm_connector(connector);
        if (*aconnector).audio_inst != port { continue; }
        *enabled = true;
        mutex_lock(&mut (*connector).eld_mutex);
        ret = drm_eld_size((*connector).eld);
        memcpy(buf, (*connector).eld, core::cmp::min(max_bytes, ret) as usize);
        mutex_unlock(&mut (*connector).eld_mutex);
        break;
    }
    drm_connector_list_iter_end(&mut conn_iter);
    mutex_unlock(&mut (*(*adev).dm).audio_lock);
    drm_dbg_kms(adev_to_drm(adev), "Get ELD : idx=%d ret=%d en=%d\n", port, ret, *enabled);
    ret
}

static amdgpu_dm_audio_component_ops: drm_audio_component_ops = drm_audio_component_ops {
    get_eld: Some(amdgpu_dm_audio_component_get_eld),
};

unsafe fn amdgpu_dm_audio_component_bind(kdev: *mut device, _hda_kdev: *mut device, data: *mut core::ffi::c_void) -> i32 {
    let dev = dev_get_drvdata(kdev);
    let adev = drm_to_adev(dev);
    let acomp = data as *mut drm_audio_component;
    (*acomp).ops = &amdgpu_dm_audio_component_ops;
    (*acomp).dev = kdev;
    (*(*adev).dm).audio_component = acomp;
    0
}

unsafe fn amdgpu_dm_audio_component_unbind(kdev: *mut device, _hda_kdev: *mut device, data: *mut core::ffi::c_void) {
    let adev = drm_to_adev(dev_get_drvdata(kdev));
    let acomp = data as *mut drm_audio_component;
    (*acomp).ops = core::ptr::null();
    (*acomp).dev = core::ptr::null_mut();
    (*(*adev).dm).audio_component = core::ptr::null_mut();
}

static amdgpu_dm_audio_component_bind_ops: component_ops = component_ops {
    bind: Some(amdgpu_dm_audio_component_bind),
    unbind: Some(amdgpu_dm_audio_component_unbind),
};

unsafe fn amdgpu_dm_audio_init_pins(adev: *mut amdgpu_device, audio_count: i32, inst_array: *const u32) {
    (*adev).mode_info.audio.num_pins = audio_count;
    for i in 0..audio_count {
        let pin = &mut (*adev).mode_info.audio.pin[i as usize];
        pin.channels = -1;
        pin.rate = -1;
        pin.bits_per_sample = -1;
        pin.status_bits = 0;
        pin.category_code = 0;
        pin.connected = false;
        pin.id = *inst_array.add(i as usize);
        pin.offset = 0;
    }
}

unsafe fn amdgpu_dm_audio_init(adev: *mut amdgpu_device) -> i32 {
    let mut inst_array: [u32; MAX_AUDIOS] = [0; MAX_AUDIOS];
    let audio_count: i32;
    let ret: i32;
    if !amdgpu_audio { return 0; }
    (*adev).mode_info.audio.enabled = true;
    audio_count = (*(*(*adev).dm).dc).res_pool.audio_count;
    for i in 0..audio_count { inst_array[i as usize] = (*(*(*(*adev).dm).dc).res_pool.audios[i as usize]).inst; }
    amdgpu_dm_audio_init_pins(adev, audio_count, inst_array.as_ptr());
    ret = component_add((*adev).dev, &amdgpu_dm_audio_component_bind_ops);
    if ret < 0 { return ret; }
    (*(*adev).dm).audio_registered = true;
    0
}

unsafe fn amdgpu_dm_audio_fini(adev: *mut amdgpu_device) {
    if !amdgpu_audio || !(*adev).mode_info.audio.enabled { return; }
    if (*(*adev).dm).audio_registered {
        component_del((*adev).dev, &amdgpu_dm_audio_component_bind_ops);
        (*(*adev).dm).audio_registered = false;
    }
    // TODO: Disable audio?
    (*adev).mode_info.audio.enabled = false;
}

unsafe fn amdgpu_dm_audio_eld_notify(adev: *mut amdgpu_device, pin: i32) {
    let acomp = (*(*adev).dm).audio_component;
    if !acomp.is_null() && !(*acomp).audio_ops.is_null() && !(*(*acomp).audio_ops).pin_eld_notify.is_none() {
        drm_dbg_kms(adev_to_drm(adev), "Notify ELD: %d\n", pin);
        ((*(*acomp).audio_ops).pin_eld_notify.unwrap())((*(*acomp).audio_ops).audio_ptr, pin, -1);
    }
}

unsafe fn amdgpu_dm_fill_audio_info(audio_info: *mut audio_info, drm_connector: *const drm_connector, dc_sink: *const dc_sink) {
    let edid_caps = &(*dc_sink).edid_caps;
    (*audio_info).manufacture_id = edid_caps.manufacturer_id;
    (*audio_info).product_id = edid_caps.product_id;
    let cea_revision = (*drm_connector).display_info.cea_rev;
    strscpy((*audio_info).display_name.as_mut_ptr(), edid_caps.display_name.as_ptr(), AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS);
    if cea_revision >= 3 {
        (*audio_info).mode_count = edid_caps.audio_mode_count;
        for i in 0..(*audio_info).mode_count {
            (*audio_info).modes[i as usize].format_code = edid_caps.audio_modes[i as usize].format_code as audio_format_code;
            (*audio_info).modes[i as usize].channel_count = edid_caps.audio_modes[i as usize].channel_count;
            (*audio_info).modes[i as usize].sample_rates.all = edid_caps.audio_modes[i as usize].sample_rate;
            (*audio_info).modes[i as usize].sample_size = edid_caps.audio_modes[i as usize].sample_size;
        }
    }
    (*audio_info).flags.all = edid_caps.speaker_flags;
    // TODO: We only check for the progressive mode, check for interlace mode too
    if (*drm_connector).latency_present[0] {
        (*audio_info).video_latency = (*drm_connector).video_latency[0];
        (*audio_info).audio_latency = (*drm_connector).audio_latency[0];
    }
    // TODO: For DP, video and audio latency should be calculated from DPCD caps
}

unsafe fn amdgpu_dm_commit_audio(dev: *mut drm_device, state: *mut drm_atomic_commit) {
    let adev = drm_to_adev(dev);
    // The connector-state iteration macros and their bodies are preserved as
    // external translation dependencies, matching the source control flow.
    for_each_oldnew_connector_in_state!(state, connector, old_con_state, new_con_state, i) {
        if (*old_con_state).crtc != (*new_con_state).crtc { goto_notify!(); }
        if (*new_con_state).crtc.is_null() { continue; }
        let new_crtc_state = drm_atomic_get_new_crtc_state(state, (*new_con_state).crtc);
        if new_crtc_state.is_null() || !drm_atomic_crtc_needs_modeset(new_crtc_state) { continue; }
        notify: {
            if (*connector).connector_type == DRM_MODE_CONNECTOR_WRITEBACK { continue; }
            let aconnector = to_amdgpu_dm_connector(connector);
            mutex_lock(&mut (*(*adev).dm).audio_lock);
            let inst = (*aconnector).audio_inst;
            (*aconnector).audio_inst = -1;
            mutex_unlock(&mut (*(*adev).dm).audio_lock);
            amdgpu_dm_audio_eld_notify(adev, inst);
        }
    }
    for_each_new_connector_in_state!(state, connector, new_con_state, i) {
        if (*new_con_state).crtc.is_null() { continue; }
        let new_crtc_state = drm_atomic_get_new_crtc_state(state, (*new_con_state).crtc);
        if new_crtc_state.is_null() || !drm_atomic_crtc_needs_modeset(new_crtc_state) { continue; }
        let new_dm_crtc_state = to_dm_crtc_state(new_crtc_state);
        if (*new_dm_crtc_state).stream.is_null() { continue; }
        let status = dc_stream_get_status((*new_dm_crtc_state).stream);
        if status.is_null() || (*connector).connector_type == DRM_MODE_CONNECTOR_WRITEBACK { continue; }
        let aconnector = to_amdgpu_dm_connector(connector);
        mutex_lock(&mut (*(*adev).dm).audio_lock);
        let inst = (*status).audio_inst;
        (*aconnector).audio_inst = inst;
        mutex_unlock(&mut (*(*adev).dm).audio_lock);
        amdgpu_dm_audio_eld_notify(adev, inst);
    }
}

// Preserved build-time KUnit exports; enabled only when CONFIG_DRM_AMD_DC_KUNIT_TEST is set.
unsafe fn amdgpu_dm_audio_get_param() -> i32 { amdgpu_audio }
unsafe fn amdgpu_dm_audio_set_param(val: i32) { amdgpu_audio = val; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
