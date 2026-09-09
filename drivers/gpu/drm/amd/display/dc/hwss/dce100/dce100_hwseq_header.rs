/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// C dependencies: "core_types.h" and "hw_sequencer_private.h".

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_bios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_plane_state {
    _private: [u8; 0],
}

// Declaration supplied by the included C headers.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum pipe_gating_control {}

extern "C" {
    pub fn dce100_hw_sequencer_construct(dc: *mut dc);

    pub fn dce100_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dce100_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dce100_enable_display_power_gating(
        dc: *mut dc,
        controller_id: u8,
        dcb: *mut dc_bios,
        power_gating: pipe_gating_control,
    ) -> bool;

    pub fn dce100_reset_surface_dcc_and_tiling(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *mut dc_plane_state,
        clear_tiling: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
