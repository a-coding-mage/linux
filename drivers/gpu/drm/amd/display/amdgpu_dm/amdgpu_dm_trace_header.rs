// SPDX-License-Identifier: MIT
/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

//! Rust translation of the Linux tracepoint header `amdgpu_dm_trace.h`.
//! The C tracepoint registration and formatting machinery is supplied by the
//! kernel tracing environment; these declarations preserve its ABI-facing
//! event payloads and externally visible event names.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct AmdgpuDcRegTemplate { pub reg: u32, pub value: u32 }

#[repr(C)]
pub struct AmdgpuDcPerformance {
    pub reads: u32, pub writes: u32, pub read_delta: u32, pub write_delta: u32,
    pub func: *const c_char, pub line: u32,
}

#[repr(C)]
pub struct AmdgpuDmConnectorAtomicCheck {
    pub conn_id: u32, pub conn_state: *const c_void, pub state: *const c_void,
    pub commit: *const c_void, pub crtc_id: u32, pub best_encoder_id: u32,
    pub link_status: c_int, pub self_refresh_aware: bool,
    pub picture_aspect_ratio: c_int, pub content_type: u32,
    pub hdcp_content_type: u32, pub content_protection: u32,
    pub scaling_mode: u32, pub colorspace: u32, pub max_requested_bpc: u8,
    pub max_bpc: u8,
}

#[repr(C)]
pub struct AmdgpuDmCrtcAtomicCheck {
    pub state: *const c_void, pub crtc_state: *const c_void,
    pub commit: *const c_void, pub crtc_id: u32,
    pub enable: bool, pub active: bool, pub planes_changed: bool,
    pub mode_changed: bool, pub active_changed: bool, pub connectors_changed: bool,
    pub zpos_changed: bool, pub color_mgmt_changed: bool, pub no_vblank: bool,
    pub async_flip: bool, pub vrr_enabled: bool, pub self_refresh_active: bool,
    pub plane_mask: u32, pub connector_mask: u32, pub encoder_mask: u32,
}

#[repr(C)]
pub struct AmdgpuDmPlaneStateTemplate {
    pub plane_id: u32, pub plane_type: c_int, pub plane_state: *const c_void,
    pub state: *const c_void, pub crtc_id: u32, pub fb_id: u32,
    pub fb_format: u32, pub fb_planes: u8, pub fb_modifier: u64,
    pub fence: *const c_void, pub crtc_x: i32, pub crtc_y: i32,
    pub crtc_w: u32, pub crtc_h: u32, pub src_x: u32, pub src_y: u32,
    pub src_w: u32, pub src_h: u32, pub alpha: u32, pub pixel_blend_mode: u32,
    pub rotation: u32, pub zpos: u32, pub normalized_zpos: u32,
    pub color_encoding: c_int, pub color_range: c_int, pub visible: bool,
}

#[repr(C)]
pub struct AmdgpuDmAtomicStateTemplate {
    pub state: *const c_void, pub allow_modeset: bool,
    pub legacy_cursor_update: bool, pub async_update: bool, pub duplicated: bool,
    pub num_connector: c_int, pub num_private_objs: c_int,
}

#[repr(C)]
pub struct AmdgpuDmAtomicCheckFinish {
    pub state: *const c_void, pub res: c_int, pub async_update: bool,
    pub allow_modeset: bool,
}

#[repr(C)]
pub struct AmdgpuDmubTraceHighIrq {
    pub trace_code: u32, pub tick_count: u32, pub param0: u32, pub param1: u32,
}

#[repr(C)]
pub struct AmdgpuRefreshRateTrack {
    pub crtc_index: c_int, pub refresh_rate_ns: i64, pub refresh_rate_hz: u32,
}

#[repr(C)]
pub struct DcnFpu {
    pub begin: bool, pub function: *const c_char, pub line: c_int,
    pub recursion_depth: c_int,
}

#[repr(C)]
pub struct DcnOptcLockUnlockState {
    pub function: *const c_char, pub instance: c_int, pub lock: bool,
    pub line: c_int, pub opp_count: c_int, pub max_h_total: c_int,
    pub max_v_total: c_int, pub min_h_blank: c_int, pub min_h_sync_width: c_int,
    pub min_v_sync_width: c_int, pub min_v_blank: c_int,
    pub min_v_blank_interlace: c_int, pub vstartup_start: c_int,
    pub vupdate_offset: c_int, pub vupdate_width: c_int, pub vready_offset: c_int,
}

#[repr(C)]
pub struct AmdgpuDmBrightness {
    pub function: *mut c_void, pub user_brightness: u32,
    pub converted_brightness: u32, pub aux: bool, pub ac: bool,
}

#[repr(C)]
pub struct AmdgpuDmIsmCommit {
    pub active_vblank_irq_count: c_int, pub vblank_enabled: bool,
    pub allow_panel_sso: bool,
}

#[repr(C)]
pub struct AmdgpuDmIsmEvent {
    pub crtc_id: c_int, pub prev_state: *const c_char,
    pub curr_state: *const c_char, pub event: *const c_char,
}

// The following event declarations correspond one-for-one to the C
// DECLARE_EVENT_CLASS, DEFINE_EVENT, and TRACE_EVENT entries.  Their argument
// and payload types are intentionally opaque where they refer to kernel/DC
// structures supplied by other headers.
extern "C" {
    pub fn amdgpu_dc_rreg(count: *mut c_ulong, reg: u32, value: u32);
    pub fn amdgpu_dc_wreg(count: *mut c_ulong, reg: u32, value: u32);
    pub fn amdgpu_dc_performance(read_count: c_ulong, write_count: c_ulong,
        last_read: *mut c_ulong, last_write: *mut c_ulong,
        func: *const c_char, line: u32);
    pub fn amdgpu_dm_connector_atomic_check(state: *const c_void);
    pub fn amdgpu_dm_crtc_atomic_check(state: *const c_void);
    pub fn amdgpu_dm_plane_atomic_check(state: *const c_void);
    pub fn amdgpu_dm_atomic_update_cursor(state: *const c_void);
    pub fn amdgpu_dm_atomic_commit_tail_begin(state: *const c_void);
    pub fn amdgpu_dm_atomic_commit_tail_finish(state: *const c_void);
    pub fn amdgpu_dm_atomic_check_begin(state: *const c_void);
    pub fn amdgpu_dm_atomic_check_finish(state: *const c_void, res: c_int);
    pub fn amdgpu_dm_dc_pipe_state(pipe_idx: c_int, plane_state: *const c_void,
        stream: *const c_void, plane_res: *const c_void, update_flags: c_int);
    pub fn amdgpu_dm_dc_clocks_state(clk: *const c_void);
    pub fn amdgpu_dm_dce_clocks_state(clk: *const c_void);
    pub fn amdgpu_dmub_trace_high_irq(trace_code: u32, tick_count: u32,
        param0: u32, param1: u32);
    pub fn amdgpu_refresh_rate_track(crtc_index: c_int, refresh_rate_ns: i64,
        refresh_rate_hz: u32);
    pub fn dcn_fpu(begin: bool, function: *const c_char, line: c_int,
        recursion_depth: c_int);
    pub fn dcn_optc_lock_unlock_state(optc_state: *const c_void, instance: c_int,
        lock: bool, function: *const c_char, line: c_int);
    pub fn amdgpu_dm_brightness(function: *mut c_void, user_brightness: u32,
        converted_brightness: u32, aux: bool, ac: bool);
    pub fn amdgpu_dm_ism_commit(active_vblank_irq_count: c_int,
        vblank_enabled: bool, allow_panel_sso: bool);
    pub fn amdgpu_dm_ism_event(crtc_id: c_int, prev_state: *const c_char,
        curr_state: *const c_char, event: *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
