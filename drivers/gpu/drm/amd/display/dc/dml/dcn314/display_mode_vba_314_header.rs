/* SPDX-License-Identifier: MIT */
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

// The C header guard is omitted; this file is intended to be included through
// the surrounding Rust module structure.

unsafe extern "C" {
    pub fn dml314_recalculate(mode_lib: *mut display_mode_lib);
    pub fn dml314_ModeSupportAndSystemConfigurationFull(
        mode_lib: *mut display_mode_lib,
    );
    pub fn dml314_CalculateWriteBackDISPCLK(
        WritebackPixelFormat: source_format_class,
        PixelClock: f64,
        WritebackHRatio: f64,
        WritebackVRatio: f64,
        WritebackHTaps: u32,
        WritebackVTaps: u32,
        WritebackSourceWidth: i64,
        WritebackDestinationWidth: i64,
        HTotal: u32,
        WritebackLineBufferSize: u32,
    ) -> f64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
