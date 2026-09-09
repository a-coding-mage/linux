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
 */

// Dependencies: dc_state.h and dc_stream.h.

#[repr(C)]
pub enum dc_stream_state {}
#[repr(C)]
pub enum dc_state {}
#[repr(C)]
pub enum dc_plane_state {}
#[repr(C)]
pub enum dc {}
#[repr(C)]
pub enum pipe_ctx {}
#[repr(C)]
pub enum mall_stream_type {}
#[repr(C)]
pub enum dc_status {}

extern "C" {
    pub fn dc_state_get_stream_from_id(
        state: *const dc_state,
        id: ::core::ffi::c_uint,
    ) -> *mut dc_stream_state;

    // Get the type of the provided resource (none, phantom, main) based on the
    // provided context. If the context is unavailable, determine only if
    // phantom or not.
    pub fn dc_state_get_pipe_subvp_type(
        state: *const dc_state,
        pipe_ctx: *const pipe_ctx,
    ) -> mall_stream_type;
    pub fn dc_state_get_stream_subvp_type(
        state: *const dc_state,
        stream: *const dc_stream_state,
    ) -> mall_stream_type;

    // Gets the phantom stream if main is provided, gets the main if phantom
    // is provided.
    pub fn dc_state_get_paired_subvp_stream(
        state: *const dc_state,
        stream: *const dc_stream_state,
    ) -> *mut dc_stream_state;

    // allocate's phantom stream or plane and returns pointer to the object
    pub fn dc_state_create_phantom_stream(
        dc: *const dc,
        state: *mut dc_state,
        main_stream: *mut dc_stream_state,
    ) -> *mut dc_stream_state;
    pub fn dc_state_create_phantom_plane(
        dc: *const dc,
        state: *mut dc_state,
        main_plane: *mut dc_plane_state,
    ) -> *mut dc_plane_state;

    // deallocate's phantom stream or plane
    pub fn dc_state_release_phantom_stream(
        dc: *const dc,
        state: *mut dc_state,
        phantom_stream: *mut dc_stream_state,
    );
    pub fn dc_state_release_phantom_plane(
        dc: *const dc,
        state: *mut dc_state,
        phantom_plane: *mut dc_plane_state,
    );

    // add/remove phantom stream to context and generate subvp meta data
    pub fn dc_state_add_phantom_stream(
        dc: *const dc,
        state: *mut dc_state,
        phantom_stream: *mut dc_stream_state,
        main_stream: *mut dc_stream_state,
    ) -> dc_status;
    pub fn dc_state_remove_phantom_stream(
        dc: *const dc,
        state: *mut dc_state,
        phantom_stream: *mut dc_stream_state,
    ) -> dc_status;

    pub fn dc_state_add_phantom_plane(
        dc: *const dc,
        phantom_stream: *mut dc_stream_state,
        phantom_plane: *mut dc_plane_state,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_remove_phantom_plane(
        dc: *const dc,
        phantom_stream: *mut dc_stream_state,
        phantom_plane: *mut dc_plane_state,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_rem_all_phantom_planes_for_stream(
        dc: *const dc,
        phantom_stream: *mut dc_stream_state,
        state: *mut dc_state,
        should_release_planes: bool,
    ) -> bool;
    pub fn dc_state_add_all_phantom_planes_for_stream(
        dc: *const dc,
        phantom_stream: *mut dc_stream_state,
        phantom_planes: *const *mut dc_plane_state,
        plane_count: ::core::ffi::c_int,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_remove_phantom_streams_and_planes(
        dc: *const dc,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_release_phantom_streams_and_planes(
        dc: *const dc,
        state: *mut dc_state,
    );
    pub fn dc_state_is_fams2_in_use(
        dc: *const dc,
        state: *const dc_state,
    ) -> bool;

    pub fn dc_state_set_stream_subvp_cursor_limit(
        stream: *const dc_stream_state,
        state: *mut dc_state,
        limit: bool,
    );
    pub fn dc_state_get_stream_subvp_cursor_limit(
        stream: *const dc_stream_state,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_set_stream_cursor_subvp_limit(
        stream: *const dc_stream_state,
        state: *mut dc_state,
        limit: bool,
    );
    pub fn dc_state_get_stream_cursor_subvp_limit(
        stream: *const dc_stream_state,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_can_clear_stream_cursor_subvp_limit(
        stream: *const dc_stream_state,
        state: *mut dc_state,
    ) -> bool;
    pub fn dc_state_is_subvp_in_use(state: *mut dc_state) -> bool;
    pub fn dc_state_is_alt_in_use(
        dc: *const dc,
        state: *const dc_state,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
