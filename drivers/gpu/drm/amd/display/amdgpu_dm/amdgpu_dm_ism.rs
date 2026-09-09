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

// External kernel, DRM, and amdgpu declarations are supplied by surrounding bindings.

pub unsafe fn dm_ism_next_state(
    current_state: amdgpu_dm_ism_state,
    event: amdgpu_dm_ism_event,
    next_state: *mut amdgpu_dm_ism_state,
) -> bool {
    match STATE_EVENT(current_state, event) {
        STATE_EVENT(DM_ISM_STATE_FULL_POWER_RUNNING, DM_ISM_EVENT_ENTER_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_WAITING;
        }
        STATE_EVENT(DM_ISM_STATE_FULL_POWER_RUNNING, DM_ISM_EVENT_BEGIN_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_FULL_POWER_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_FULL_POWER_BUSY, DM_ISM_EVENT_ENTER_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_FULL_POWER_BUSY, DM_ISM_EVENT_END_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_FULL_POWER_RUNNING;
        }
        STATE_EVENT(DM_ISM_STATE_HYSTERESIS_WAITING, DM_ISM_EVENT_EXIT_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_TIMER_ABORTED;
        }
        STATE_EVENT(DM_ISM_STATE_HYSTERESIS_WAITING, DM_ISM_EVENT_BEGIN_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_HYSTERESIS_WAITING, DM_ISM_EVENT_TIMER_ELAPSED)
        | STATE_EVENT(DM_ISM_STATE_HYSTERESIS_WAITING, DM_ISM_EVENT_IMMEDIATE) => {
            *next_state = DM_ISM_STATE_OPTIMIZED_IDLE;
        }
        STATE_EVENT(DM_ISM_STATE_HYSTERESIS_BUSY, DM_ISM_EVENT_EXIT_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_FULL_POWER_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_HYSTERESIS_BUSY, DM_ISM_EVENT_END_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_WAITING;
        }
        STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE, DM_ISM_EVENT_EXIT_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_FULL_POWER_RUNNING;
        }
        STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE, DM_ISM_EVENT_BEGIN_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE, DM_ISM_EVENT_SSO_TIMER_ELAPSED)
        | STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE, DM_ISM_EVENT_IMMEDIATE) => {
            *next_state = DM_ISM_STATE_OPTIMIZED_IDLE_SSO;
        }
        STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE_SSO, DM_ISM_EVENT_EXIT_IDLE_REQUESTED) => {
            *next_state = DM_ISM_STATE_FULL_POWER_RUNNING;
        }
        STATE_EVENT(DM_ISM_STATE_OPTIMIZED_IDLE_SSO, DM_ISM_EVENT_BEGIN_CURSOR_UPDATE) => {
            *next_state = DM_ISM_STATE_HYSTERESIS_BUSY;
        }
        STATE_EVENT(DM_ISM_STATE_TIMER_ABORTED, DM_ISM_EVENT_IMMEDIATE) => {
            *next_state = DM_ISM_STATE_FULL_POWER_RUNNING;
        }
        _ => return false,
    }
    true
}

pub unsafe fn dm_ism_get_sso_delay(ism: *const amdgpu_dm_ism, stream: *const dc_stream_state) -> u64 {
    if stream.is_null() || (*ism).config.sso_num_frames == 0 { return 0; }
    let one_frame_ns = div64_u64((*stream).timing.v_total as u64 * (*stream).timing.h_total as u64 * 10000000u64,
        (*stream).timing.pix_clk_100hz as u64);
    (*ism).config.sso_num_frames as u64 * one_frame_ns
}

pub unsafe fn dm_ism_get_idle_allow_delay(ism: *const amdgpu_dm_ism, stream: *const dc_stream_state) -> u64 {
    let config = &(*ism).config;
    if stream.is_null() || config.filter_num_frames == 0 || config.filter_entry_count == 0 || config.activation_num_delay_frames == 0 { return 0; }
    let one_frame_ns = div64_u64((*stream).timing.v_total as u64 * (*stream).timing.h_total as u64 * 10000000u64,
        (*stream).timing.pix_clk_100hz as u64);
    let short_idle_ns = config.filter_num_frames as u64 * one_frame_ns;
    let old_hist_ns = config.filter_old_history_threshold as u64 * one_frame_ns;
    let history_size = core::cmp::min(core::cmp::max(config.filter_history_size, config.filter_entry_count), AMDGPU_DM_IDLE_HIST_LEN);
    let mut pos = (*ism).next_record_idx;
    let mut short_idle_count = 0;
    for _ in 0..history_size {
        if pos <= 0 || pos > AMDGPU_DM_IDLE_HIST_LEN { pos = AMDGPU_DM_IDLE_HIST_LEN; }
        pos -= 1;
        if (*ism).records[pos as usize].duration_ns <= short_idle_ns { short_idle_count += 1; }
        if short_idle_count >= config.filter_entry_count { break; }
        if old_hist_ns > 0 && (*ism).last_idle_timestamp_ns - (*ism).records[pos as usize].timestamp_ns > old_hist_ns { break; }
    }
    if short_idle_count >= config.filter_entry_count { config.activation_num_delay_frames as u64 * one_frame_ns } else { 0 }
}

pub unsafe fn dm_ism_insert_record(ism: *mut amdgpu_dm_ism) {
    if (*ism).next_record_idx < 0 || (*ism).next_record_idx >= AMDGPU_DM_IDLE_HIST_LEN { (*ism).next_record_idx = 0; }
    let record = &mut (*ism).records[(*ism).next_record_idx as usize];
    (*ism).next_record_idx += 1;
    record.timestamp_ns = ktime_get_ns();
    record.duration_ns = record.timestamp_ns - (*ism).last_idle_timestamp_ns;
}

pub unsafe fn dm_ism_set_last_idle_ts(ism: *mut amdgpu_dm_ism) { (*ism).last_idle_timestamp_ns = ktime_get_ns(); }

pub unsafe fn dm_ism_trigger_event(ism: *mut amdgpu_dm_ism, event: amdgpu_dm_ism_event) -> bool {
    let mut next_state = (*ism).current_state;
    if dm_ism_next_state((*ism).current_state, event, &mut next_state) {
        (*ism).previous_state = (*ism).current_state;
        (*ism).current_state = next_state;
        true
    } else { false }
}

unsafe fn dm_ism_dispatch_next_event(current_state: amdgpu_dm_ism_state, delay_ns: u64, sso_delay_ns: u64) -> amdgpu_dm_ism_event {
    match current_state {
        DM_ISM_STATE_HYSTERESIS_WAITING if delay_ns == 0 => DM_ISM_EVENT_IMMEDIATE,
        DM_ISM_STATE_OPTIMIZED_IDLE if sso_delay_ns == 0 => DM_ISM_EVENT_IMMEDIATE,
        DM_ISM_STATE_TIMER_ABORTED => DM_ISM_EVENT_IMMEDIATE,
        _ => DM_ISM_NUM_EVENTS,
    }
}

unsafe fn dm_ism_commit_idle_optimization_state(ism: *mut amdgpu_dm_ism, stream: *mut dc_stream_state, vblank_enabled: bool, mut allow_panel_sso: bool) {
    let acrtc = ism_to_amdgpu_crtc(ism); let adev = drm_to_adev((*acrtc).base.dev); let dm = &mut (*adev).dm;
    trace_amdgpu_dm_ism_commit(dm.active_vblank_irq_count, vblank_enabled, allow_panel_sso);
    if vblank_enabled || allow_panel_sso { dc_allow_idle_optimizations(dm.dc, false); }
    if !stream.is_null() && !(*stream).link.is_null() {
        allow_panel_sso = allow_panel_sso && !vblank_enabled;
        amdgpu_dm_crtc_set_static_screen_optimze(dm, stream, allow_panel_sso, (*acrtc).dm_irq_params.allow_sr_entry);
    }
    if !vblank_enabled && dm.active_vblank_irq_count == 0 {
        dc_post_update_surfaces_to_stream(dm.dc); dc_allow_idle_optimizations(dm.dc, true);
    }
}

unsafe fn dm_ism_dispatch_power_state(ism: *mut amdgpu_dm_ism, acrtc_state: *mut dm_crtc_state) -> amdgpu_dm_ism_event {
    let config = &(*ism).config; let mut delay_ns = 0; let mut sso_delay_ns = 0;
    match (*ism).previous_state {
        DM_ISM_STATE_HYSTERESIS_WAITING => { if (*ism).current_state != DM_ISM_STATE_OPTIMIZED_IDLE && (*ism).current_state != DM_ISM_STATE_OPTIMIZED_IDLE_SSO { cancel_delayed_work(&mut (*ism).delayed_work); } }
        DM_ISM_STATE_OPTIMIZED_IDLE => { if (*ism).current_state != DM_ISM_STATE_OPTIMIZED_IDLE_SSO { cancel_delayed_work(&mut (*ism).sso_delayed_work); dm_ism_insert_record(ism); dm_ism_commit_idle_optimization_state(ism, (*acrtc_state).stream, true, false); } }
        DM_ISM_STATE_OPTIMIZED_IDLE_SSO => { dm_ism_insert_record(ism); dm_ism_commit_idle_optimization_state(ism, (*acrtc_state).stream, true, false); }
        _ => {}
    }
    match (*ism).current_state {
        DM_ISM_STATE_HYSTERESIS_WAITING => { dm_ism_set_last_idle_ts(ism); delay_ns = dm_ism_get_idle_allow_delay(ism, (*acrtc_state).stream); if delay_ns > 0 { mod_delayed_work(system_dfl_wq, &mut (*ism).delayed_work, nsecs_to_jiffies(delay_ns)); } }
        DM_ISM_STATE_OPTIMIZED_IDLE => { sso_delay_ns = dm_ism_get_sso_delay(ism, (*acrtc_state).stream); if sso_delay_ns > 0 { if config.sso_num_frames >= config.filter_num_frames { dm_ism_commit_idle_optimization_state(ism, (*acrtc_state).stream, false, false); } mod_delayed_work(system_dfl_wq, &mut (*ism).sso_delayed_work, nsecs_to_jiffies(sso_delay_ns)); } }
        DM_ISM_STATE_OPTIMIZED_IDLE_SSO => dm_ism_commit_idle_optimization_state(ism, (*acrtc_state).stream, false, true),
        DM_ISM_STATE_TIMER_ABORTED => { dm_ism_insert_record(ism); dm_ism_commit_idle_optimization_state(ism, (*acrtc_state).stream, true, false); }
        _ => {}
    }
    dm_ism_dispatch_next_event((*ism).current_state, delay_ns, sso_delay_ns)
}

unsafe fn dm_ism_delayed_work_func(work: *mut work_struct) { let ism = container_of!(work, amdgpu_dm_ism, delayed_work.work); amdgpu_dm_ism_commit_event(ism, DM_ISM_EVENT_TIMER_ELAPSED); }
unsafe fn dm_ism_sso_delayed_work_func(work: *mut work_struct) { let ism = container_of!(work, amdgpu_dm_ism, sso_delayed_work.work); amdgpu_dm_ism_commit_event(ism, DM_ISM_EVENT_SSO_TIMER_ELAPSED); }

static mut dm_ism_events_str: [&str; DM_ISM_NUM_EVENTS as usize] = [
    "IMMEDIATE", "ENTER_IDLE_REQUESTED", "EXIT_IDLE_REQUESTED", "BEGIN_CURSOR_UPDATE",
    "END_CURSOR_UPDATE", "TIMER_ELAPSED", "SSO_TIMER_ELAPSED",
];
static mut dm_ism_states_str: [&str; DM_ISM_NUM_STATES as usize] = [
    "FULL_POWER_RUNNING", "FULL_POWER_BUSY", "HYSTERESIS_WAITING", "HYSTERESIS_BUSY",
    "OPTIMIZED_IDLE", "OPTIMIZED_IDLE_SSO", "TIMER_ABORTED",
];

pub unsafe fn amdgpu_dm_ism_commit_event(ism: *mut amdgpu_dm_ism, mut event: amdgpu_dm_ism_event) {
    let acrtc = ism_to_amdgpu_crtc(ism);
    let adev = drm_to_adev((*acrtc).base.dev);
    let dm = &mut (*adev).dm;
    let acrtc_state = to_dm_crtc_state((*acrtc).base.state);
    lockdep_assert_held(&dm.dc_lock);
    ASSERT(dm.dc);
    if acrtc_state.is_null() { trace_amdgpu_dm_ism_event((*acrtc).crtc_id, "NO_STATE", "NO_STATE", "N/A"); return; }
    loop {
        let transition = dm_ism_trigger_event(ism, event);
        let mut next_event = DM_ISM_NUM_EVENTS;
        if transition {
            trace_amdgpu_dm_ism_event((*acrtc).crtc_id, dm_ism_states_str[(*ism).previous_state as usize], dm_ism_states_str[(*ism).current_state as usize], dm_ism_events_str[event as usize]);
            next_event = dm_ism_dispatch_power_state(ism, acrtc_state);
        } else {
            trace_amdgpu_dm_ism_event((*acrtc).crtc_id, dm_ism_states_str[(*ism).current_state as usize], dm_ism_states_str[(*ism).current_state as usize], dm_ism_events_str[event as usize]);
        }
        event = next_event;
        if next_event >= DM_ISM_NUM_EVENTS { break; }
    }
}

// The remaining worker/lifecycle operations preserve the original kernel calls and ordering.
pub unsafe fn amdgpu_dm_ism_disable(dm: *mut amdgpu_display_manager) {
    lockdep_assert_not_held(&(*dm).dc_lock);
    let mut crtc = core::ptr::null_mut();
    drm_for_each_crtc!(crtc, (*dm).ddev, {
        let acrtc = to_amdgpu_crtc(crtc); let ism = &mut (*acrtc).ism;
        disable_delayed_work_sync(&mut ism.delayed_work); disable_delayed_work_sync(&mut ism.sso_delayed_work);
    });
}

pub unsafe fn amdgpu_dm_ism_force_full_power(dm: *mut amdgpu_display_manager) {
    lockdep_assert_held(&(*dm).dc_lock);
    let mut crtc = core::ptr::null_mut();
    drm_for_each_crtc!(crtc, (*dm).ddev, { let acrtc = to_amdgpu_crtc(crtc); amdgpu_dm_ism_commit_event(&mut (*acrtc).ism, DM_ISM_EVENT_EXIT_IDLE_REQUESTED); });
}

pub unsafe fn amdgpu_dm_ism_enable(dm: *mut amdgpu_display_manager) {
    let mut crtc = core::ptr::null_mut();
    drm_for_each_crtc!(crtc, (*dm).ddev, { let acrtc = to_amdgpu_crtc(crtc); let ism = &mut (*acrtc).ism; enable_delayed_work(&mut ism.delayed_work); enable_delayed_work(&mut ism.sso_delayed_work); });
}

pub unsafe fn amdgpu_dm_ism_init(ism: *mut amdgpu_dm_ism, config: *mut amdgpu_dm_ism_config) {
    (*ism).config = *config;
    (*ism).current_state = DM_ISM_STATE_FULL_POWER_RUNNING; (*ism).previous_state = DM_ISM_STATE_FULL_POWER_RUNNING;
    (*ism).next_record_idx = 0; (*ism).last_idle_timestamp_ns = 0;
    INIT_DELAYED_WORK!(&mut (*ism).delayed_work, dm_ism_delayed_work_func);
    INIT_DELAYED_WORK!(&mut (*ism).sso_delayed_work, dm_ism_sso_delayed_work_func);
}

pub unsafe fn amdgpu_dm_ism_fini(ism: *mut amdgpu_dm_ism) {
    cancel_delayed_work_sync(&mut (*ism).sso_delayed_work); cancel_delayed_work_sync(&mut (*ism).delayed_work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
