/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit:
// #include "../display_mode_enums.h"

// To enable a lot of debug msg
// #define __DML_VBA_DEBUG__
// For DML-C changes that hasn't been propagated to VBA yet
// #define __DML_VBA_ALLOW_DELTA__

// Move these to ip parameters/constant
// At which vstartup the DML start to try if the mode can be supported
pub const __DML_VBA_MIN_VSTARTUP__: i32 = 9;

// Delay in DCFCLK from ARB to DET (1st num is ARB to SDPIF, 2nd number is SDPIF to DET)
pub const __DML_ARB_TO_RET_DELAY__: i32 = 7 + 95;

// fudge factor for min dcfclk calclation
pub const __DML_MIN_DCFCLK_FACTOR__: f64 = 1.15;

// Prefetch schedule max vratio
pub const __DML_MAX_VRATIO_PRE__: f64 = 7.9;
pub const __DML_MAX_BW_RATIO_PRE__: f64 = 4.0;

pub const __DML_VBA_MAX_DST_Y_PRE__: f64 = 63.75;

pub const BPP_INVALID: u32 = 0;
pub const BPP_BLENDED_PIPE: u32 = 0xffff_ffff;

pub const MEM_STROBE_FREQ_MHZ: i32 = 1600;
pub const DCFCLK_FREQ_EXTRA_PREFETCH_REQ_MHZ: i32 = 300;
pub const MEM_STROBE_MAX_DELIVERY_TIME_US: f64 = 60.0;

#[repr(C)]
pub struct display_mode_lib {
    _private: [u8; 0],
}

extern "C" {
    pub fn dml32_ModeSupportAndSystemConfigurationFull(mode_lib: *mut display_mode_lib);
    pub fn dml32_recalculate(mode_lib: *mut display_mode_lib);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
