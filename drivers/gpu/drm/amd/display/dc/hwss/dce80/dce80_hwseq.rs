/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// C dependencies supplied by the surrounding driver translation unit:
// dm_services.h, dc.h, core_types.h, dce80_hwseq.h,
// dce/dce_hwseq.h, dce110/dce110_hwseq.h, dce100/dce100_hwseq.h,
// and the DCE8 register headers.

use core::ffi::c_void;

#[repr(C)]
pub struct dc {
    pub hwseq: *mut dc_hwseq,
    pub hwss: dc_hwss,
}

#[repr(C)]
pub struct dc_hwseq {
    pub funcs: dc_hwseq_funcs,
}

#[repr(C)]
pub struct dc_hwseq_funcs {
    pub enable_display_power_gating: Option<unsafe extern "C" fn(*mut dc, *mut c_void)>,
}

#[repr(C)]
pub struct dc_hwss {
    pub pipe_control_lock: Option<unsafe extern "C" fn(*mut dc, bool)>,
    pub prepare_bandwidth: Option<unsafe extern "C" fn(*mut dc, *mut c_void)>,
    pub optimize_bandwidth: Option<unsafe extern "C" fn(*mut dc, *mut c_void)>,
    pub clear_surface_dcc_and_tiling: Option<unsafe extern "C" fn(*mut dc, *mut c_void)>,
}

extern "C" {
    pub fn dce110_hw_sequencer_construct(dc: *mut dc);
    pub fn dce100_enable_display_power_gating(dc: *mut dc, context: *mut c_void);
    pub fn dce_pipe_control_lock(dc: *mut dc, lock: bool);
    pub fn dce100_prepare_bandwidth(dc: *mut dc, context: *mut c_void);
    pub fn dce100_optimize_bandwidth(dc: *mut dc, context: *mut c_void);
    pub fn dce100_reset_surface_dcc_and_tiling(dc: *mut dc, context: *mut c_void);
}

pub unsafe fn dce80_hw_sequencer_construct(dc: *mut dc) {
    dce110_hw_sequencer_construct(dc);

    (*(*dc).hwseq).funcs.enable_display_power_gating =
        Some(dce100_enable_display_power_gating);
    (*dc).hwss.pipe_control_lock = Some(dce_pipe_control_lock);
    (*dc).hwss.prepare_bandwidth = Some(dce100_prepare_bandwidth);
    (*dc).hwss.optimize_bandwidth = Some(dce100_optimize_bandwidth);
    (*dc).hwss.clear_surface_dcc_and_tiling = Some(dce100_reset_surface_dcc_and_tiling);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
