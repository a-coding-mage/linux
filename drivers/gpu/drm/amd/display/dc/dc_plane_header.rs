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
 *
 */

// Dependency supplied by the corresponding translated hardware-types header.

#[repr(C)]
pub struct dc;

#[repr(C)]
pub struct dc_plane_state;

#[repr(C)]
pub struct dc_plane_status;

#[repr(C)]
pub struct dc_plane_status_update_flags_bits {
    // C bit-fields occupy the low-order bits of the containing uint32_t.
    pub address: u32,
    pub histogram: u32,
}

#[repr(C)]
pub union dc_plane_status_update_flags {
    pub bits: dc_plane_status_update_flags_bits,
    pub raw: u32,
}

extern "C" {
    pub fn dc_create_plane_state(dc: *const dc) -> *mut dc_plane_state;
    pub fn dc_plane_get_status(
        plane_state: *const dc_plane_state,
        flags: dc_plane_status_update_flags,
    ) -> *const dc_plane_status;
    pub fn dc_plane_state_retain(plane_state: *mut dc_plane_state);
    pub fn dc_plane_state_release(plane_state: *mut dc_plane_state);

    pub fn dc_plane_force_dcc_and_tiling_disable(
        plane_state: *mut dc_plane_state,
        clear_tiling: bool,
    );

    pub fn dc_plane_copy_config(dst: *mut dc_plane_state, src: *const dc_plane_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
