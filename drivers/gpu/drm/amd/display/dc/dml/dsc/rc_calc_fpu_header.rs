/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit:
// #include "os_types.h"
// #include <drm/display/drm_dsc.h>

pub const QP_SET_SIZE: usize = 15;

pub type QpSet = [::core::ffi::c_int; QP_SET_SIZE];

#[repr(C)]
pub struct RcParams {
    pub rc_quant_incr_limit0: ::core::ffi::c_int,
    pub rc_quant_incr_limit1: ::core::ffi::c_int,
    pub initial_fullness_offset: ::core::ffi::c_int,
    pub initial_xmit_delay: ::core::ffi::c_int,
    pub first_line_bpg_offset: ::core::ffi::c_int,
    pub second_line_bpg_offset: ::core::ffi::c_int,
    pub flatness_min_qp: ::core::ffi::c_int,
    pub flatness_max_qp: ::core::ffi::c_int,
    pub flatness_det_thresh: ::core::ffi::c_int,
    pub qp_min: QpSet,
    pub qp_max: QpSet,
    pub ofs: QpSet,
    pub rc_model_size: ::core::ffi::c_int,
    pub rc_edge_factor: ::core::ffi::c_int,
    pub rc_tgt_offset_hi: ::core::ffi::c_int,
    pub rc_tgt_offset_lo: ::core::ffi::c_int,
    pub rc_buf_thresh: [::core::ffi::c_int; QP_SET_SIZE - 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColourMode {
    CmRgb, // 444 RGB
    Cm444,  // 444 YUV or simple 422
    Cm422,  // native 422
    Cm420,  // native 420
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BitsPerComp {
    Bpc8 = 8,
    Bpc10 = 10,
    Bpc12 = 12,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MaxMin {
    DalMmMin = 0,
    DalMmMax = 1,
}

#[repr(C)]
pub struct QpEntry {
    pub bpp: f32,
    pub qps: QpSet,
}

// C declaration: typedef struct qp_entry qp_table[];
pub type QpTable = [QpEntry];

unsafe extern "C" {
    pub fn _do_calc_rc_params(
        rc: *mut RcParams,
        cm: ColourMode,
        bpc: BitsPerComp,
        drm_bpp: u16,
        is_navite_422_or_420: bool,
        slice_width: ::core::ffi::c_int,
        slice_height: ::core::ffi::c_int,
        minor_version: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
