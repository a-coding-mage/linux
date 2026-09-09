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

use core::ffi::c_char;

// Supplied by the translated dependency represented by dm_services.h.
#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mod_stats {
    pub dummy: core::ffi::c_int,
}

#[repr(C)]
pub struct mod_stats_caps {
    pub dummy: bool,
}

#[repr(C)]
pub struct mod_stats_init_params {
    pub stats_enable: u32,
    pub stats_entries: u32,
}

unsafe extern "C" {
    pub fn mod_stats_create(
        dc: *mut dc,
        init_params: *mut mod_stats_init_params,
    ) -> *mut mod_stats;

    pub fn mod_stats_destroy(mod_stats: *mut mod_stats);

    pub fn mod_stats_init(mod_stats: *mut mod_stats) -> bool;

    pub fn mod_stats_dump(mod_stats: *mut mod_stats);

    pub fn mod_stats_reset_data(mod_stats: *mut mod_stats);

    pub fn mod_stats_update_event(
        mod_stats: *mut mod_stats,
        event_string: *const c_char,
        length: u32,
    );

    pub fn mod_stats_update_flip(mod_stats: *mut mod_stats, timestamp_in_ns: u64);

    pub fn mod_stats_update_vupdate(mod_stats: *mut mod_stats, timestamp_in_ns: u64);

    pub fn mod_stats_update_freesync(
        mod_stats: *mut mod_stats,
        v_total_min: u32,
        v_total_max: u32,
        event_triggers: u32,
        window_min: u32,
        window_max: u32,
        lfc_mid_point_in_us: u32,
        inserted_frames: u32,
        inserted_frame_duration_in_us: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
