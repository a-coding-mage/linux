/* SPDX-License-Identifier: MIT */
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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
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

pub const AMDGPU_DM_IDLE_HIST_LEN: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_dm_ism_state {
    DM_ISM_STATE_FULL_POWER_RUNNING,
    DM_ISM_STATE_FULL_POWER_BUSY,
    DM_ISM_STATE_HYSTERESIS_WAITING,
    DM_ISM_STATE_HYSTERESIS_BUSY,
    DM_ISM_STATE_OPTIMIZED_IDLE,
    DM_ISM_STATE_OPTIMIZED_IDLE_SSO,
    DM_ISM_STATE_TIMER_ABORTED,
    DM_ISM_NUM_STATES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_dm_ism_event {
    DM_ISM_EVENT_IMMEDIATE,
    DM_ISM_EVENT_ENTER_IDLE_REQUESTED,
    DM_ISM_EVENT_EXIT_IDLE_REQUESTED,
    DM_ISM_EVENT_BEGIN_CURSOR_UPDATE,
    DM_ISM_EVENT_END_CURSOR_UPDATE,
    DM_ISM_EVENT_TIMER_ELAPSED,
    DM_ISM_EVENT_SSO_TIMER_ELAPSED,
    DM_ISM_NUM_EVENTS,
}

#[inline]
pub const fn STATE_EVENT(state: u32, event: u32) -> u32 {
    (state << 8) | event
}

#[repr(C)]
pub struct amdgpu_dm_ism_config {
    pub filter_num_frames: ::core::ffi::c_uint,
    pub filter_history_size: ::core::ffi::c_uint,
    pub filter_entry_count: ::core::ffi::c_uint,
    pub activation_num_delay_frames: ::core::ffi::c_uint,
    pub filter_old_history_threshold: ::core::ffi::c_uint,
    pub sso_num_frames: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct amdgpu_dm_ism_record {
    pub timestamp_ns: u64,
    pub duration_ns: u64,
}

#[repr(C)]
pub struct amdgpu_dm_ism {
    pub config: amdgpu_dm_ism_config,
    pub last_idle_timestamp_ns: u64,
    pub current_state: amdgpu_dm_ism_state,
    pub previous_state: amdgpu_dm_ism_state,
    pub records: [amdgpu_dm_ism_record; AMDGPU_DM_IDLE_HIST_LEN],
    pub next_record_idx: ::core::ffi::c_int,
    pub delayed_work: delayed_work,
    pub sso_delayed_work: delayed_work,
}

// Supplied by the Linux workqueue and DRM dependencies.
pub enum delayed_work {}
pub enum amdgpu_crtc {}
pub enum amdgpu_display_manager {}
pub enum dc_stream_state {}

// Equivalent of container_of(ism_ptr, struct amdgpu_crtc, ism).
#[macro_export]
macro_rules! ism_to_amdgpu_crtc {
    ($ism_ptr:expr) => {
        unsafe { container_of!($ism_ptr, amdgpu_crtc, ism) }
    };
}

unsafe extern "C" {
    pub fn amdgpu_dm_ism_init(ism: *mut amdgpu_dm_ism, config: *mut amdgpu_dm_ism_config);
    pub fn amdgpu_dm_ism_fini(ism: *mut amdgpu_dm_ism);
    pub fn amdgpu_dm_ism_commit_event(ism: *mut amdgpu_dm_ism, event: amdgpu_dm_ism_event);
    pub fn amdgpu_dm_ism_disable(dm: *mut amdgpu_display_manager);
    pub fn amdgpu_dm_ism_force_full_power(dm: *mut amdgpu_display_manager);
    pub fn amdgpu_dm_ism_enable(dm: *mut amdgpu_display_manager);

    // Preserved from: IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
    pub fn dm_ism_next_state(
        current_state: amdgpu_dm_ism_state,
        event: amdgpu_dm_ism_event,
        next_state: *mut amdgpu_dm_ism_state,
    ) -> bool;
    pub fn dm_ism_get_sso_delay(ism: *const amdgpu_dm_ism, stream: *const dc_stream_state) -> u64;
    pub fn dm_ism_get_idle_allow_delay(ism: *const amdgpu_dm_ism, stream: *const dc_stream_state) -> u64;
    pub fn dm_ism_insert_record(ism: *mut amdgpu_dm_ism);
    pub fn dm_ism_set_last_idle_ts(ism: *mut amdgpu_dm_ism);
    pub fn dm_ism_trigger_event(ism: *mut amdgpu_dm_ism, event: amdgpu_dm_ism_event) -> bool;
    pub fn dm_ism_dispatch_next_event(
        current_state: amdgpu_dm_ism_state,
        delay_ns: u64,
        sso_delay_ns: u64,
    ) -> amdgpu_dm_ism_event;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
