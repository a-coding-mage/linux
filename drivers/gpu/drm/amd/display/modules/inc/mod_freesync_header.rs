/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency declarations from mod_shared.h are supplied by the containing build.

#[repr(C)]
pub struct mod_freesync {
    pub dummy: ::core::ffi::c_int,
}

// TODO: References to this should be removed
#[repr(C)]
pub struct mod_freesync_caps {
    pub supported: bool,
    pub min_refresh_in_micro_hz: u32,
    pub max_refresh_in_micro_hz: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mod_vrr_state {
    VRR_STATE_UNSUPPORTED = 0,
    VRR_STATE_DISABLED,
    VRR_STATE_INACTIVE,
    VRR_STATE_ACTIVE_VARIABLE,
    VRR_STATE_ACTIVE_FIXED,
}

#[repr(C)]
pub struct mod_freesync_config {
    pub state: mod_vrr_state,
    pub vsif_supported: bool,
    pub ramping: bool,
    pub btr: bool,
    pub min_refresh_in_uhz: u32,
    pub max_refresh_in_uhz: u32,
    pub fixed_refresh_in_uhz: u32,
}

#[repr(C)]
pub struct mod_vrr_params_btr {
    pub btr_enabled: bool,
    pub btr_active: bool,
    pub mid_point_in_us: u32,
    pub inserted_duration_in_us: u32,
    pub frames_to_insert: u32,
    pub frame_counter: u32,
    pub margin_in_us: u32,
}

#[repr(C)]
pub struct mod_vrr_params_fixed_refresh {
    pub fixed_active: bool,
    pub ramping_active: bool,
    pub ramping_done: bool,
    pub target_refresh_in_uhz: u32,
    pub frame_counter: u32,
}

#[repr(C)]
pub struct mod_vrr_params_flip_interval {
    pub flip_interval_workaround_active: bool,
    pub program_flip_interval_workaround: bool,
    pub do_flip_interval_workaround_cleanup: bool,
    pub flip_interval_detect_counter: u32,
    pub vsyncs_between_flip: u32,
    pub vsync_to_flip_in_us: u32,
    pub v_update_timestamp_in_us: u32,
}

#[repr(C)]
pub struct mod_vrr_params {
    pub supported: bool,
    pub send_info_frame: bool,
    // This is used when m_const is set up in OPTC so no overriding happens from FreeSync Module.
    pub m_const_engaged: bool,
    pub state: mod_vrr_state,

    pub min_refresh_in_uhz: u32,
    pub max_duration_in_us: u32,
    pub max_refresh_in_uhz: u32,
    pub min_duration_in_us: u32,
    pub fixed_refresh_in_uhz: u32,

    pub m_const: u32,

    pub adjust: dc_crtc_timing_adjust,
    pub fixed: mod_vrr_params_fixed_refresh,
    pub btr: mod_vrr_params_btr,
    pub flip_interval: mod_vrr_params_flip_interval,
}

extern "C" {
    pub fn mod_freesync_create(dc: *mut dc) -> *mut mod_freesync;
    pub fn mod_freesync_destroy(mod_freesync: *mut mod_freesync);

    pub fn mod_freesync_build_vrr_infopacket(
        mod_freesync: *mut mod_freesync,
        stream: *const dc_stream_state,
        vrr: *const mod_vrr_params,
        packet_type: vrr_packet_type,
        app_tf: color_transfer_func,
        infopacket: *mut dc_info_packet,
        pack_sdp_v1_3: bool,
    );

    pub fn mod_freesync_build_vrr_params(
        mod_freesync: *mut mod_freesync,
        stream: *const dc_stream_state,
        in_config: *mut mod_freesync_config,
        in_out_vrr: *mut mod_vrr_params,
    );

    pub fn mod_freesync_handle_preflip(
        mod_freesync: *mut mod_freesync,
        plane: *const dc_plane_state,
        stream: *const dc_stream_state,
        curr_time_stamp_in_us: u32,
        in_out_vrr: *mut mod_vrr_params,
    );

    pub fn mod_freesync_handle_v_update(
        mod_freesync: *mut mod_freesync,
        stream: *const dc_stream_state,
        in_out_vrr: *mut mod_vrr_params,
    );

    pub fn mod_freesync_calc_nominal_field_rate(stream: *const dc_stream_state) -> u64;

    pub fn mod_freesync_calc_v_total_from_refresh(
        stream: *const dc_stream_state,
        refresh_in_uhz: u32,
    ) -> u32;

    // Returns true when FreeSync is supported and enabled (even if it is inactive)
    pub fn mod_freesync_get_freesync_enabled(pVrr: *mut mod_vrr_params) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
