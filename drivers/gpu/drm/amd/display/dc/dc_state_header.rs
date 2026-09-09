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
 *
 */

// Dependency supplied by inc/core_status.h.

use core::ffi::c_int;

// Opaque types supplied by the surrounding translation unit.
#[repr(C)]
pub struct dc;
#[repr(C)]
pub struct dc_state;
#[repr(C)]
pub struct dc_state_create_params;
#[repr(C)]
pub struct dc_stream_state;
#[repr(C)]
pub struct dc_plane_state;
#[repr(C)]
pub struct dc_stream_status;
#[repr(C)]
pub struct dc_state_status;
#[repr(C)]
pub struct dc_get_status_options;

// `enum dc_status` is supplied by inc/core_status.h.
pub type dc_status = u32;

extern "C" {
    pub fn dc_state_create(
        dc: *mut dc,
        params: *mut dc_state_create_params,
    ) -> *mut dc_state;
    pub fn dc_state_copy(dst_state: *mut dc_state, src_state: *mut dc_state);
    pub fn dc_state_create_copy(src_state: *mut dc_state) -> *mut dc_state;
    pub fn dc_state_copy_current(dc: *mut dc, dst_state: *mut dc_state);
    pub fn dc_state_create_current_copy(dc: *mut dc) -> *mut dc_state;
    pub fn dc_state_construct(dc: *mut dc, state: *mut dc_state);
    pub fn dc_state_destruct(state: *mut dc_state);
    pub fn dc_state_retain(state: *mut dc_state);
    pub fn dc_state_release(state: *mut dc_state);

    pub fn dc_state_add_stream(
        dc: *const dc,
        state: *mut dc_state,
        stream: *mut dc_stream_state,
    ) -> dc_status;

    pub fn dc_state_remove_stream(
        dc: *const dc,
        state: *mut dc_state,
        stream: *mut dc_stream_state,
    ) -> dc_status;

    pub fn dc_state_add_plane(
        dc: *const dc,
        stream: *mut dc_stream_state,
        plane_state: *mut dc_plane_state,
        state: *mut dc_state,
    ) -> bool;

    pub fn dc_state_remove_plane(
        dc: *const dc,
        stream: *mut dc_stream_state,
        plane_state: *mut dc_plane_state,
        state: *mut dc_state,
    ) -> bool;

    pub fn dc_state_rem_all_planes_for_stream(
        dc: *const dc,
        stream: *mut dc_stream_state,
        state: *mut dc_state,
    ) -> bool;

    pub fn dc_state_add_all_planes_for_stream(
        dc: *const dc,
        stream: *mut dc_stream_state,
        plane_states: *const *mut dc_plane_state,
        plane_count: c_int,
        state: *mut dc_state,
    ) -> bool;

    pub fn dc_state_get_stream_status(
        state: *mut dc_state,
        stream: *const dc_stream_state,
    ) -> *mut dc_stream_status;

    pub fn dc_state_get_status(
        status: *mut dc_state_status,
        options: *const dc_get_status_options,
    ) -> dc_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
