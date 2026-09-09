/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// C header guard: __DML31_DISPLAY_MODE_VBA_H__

extern "C" {
    pub fn dml31_recalculate(mode_lib: *mut crate::display_mode_lib);

    pub fn dml31_ModeSupportAndSystemConfigurationFull(
        mode_lib: *mut crate::display_mode_lib,
    );

    pub fn dml31_CalculateWriteBackDISPCLK(
        writeback_pixel_format: crate::source_format_class,
        pixel_clock: f64,
        writeback_h_ratio: f64,
        writeback_v_ratio: f64,
        writeback_h_taps: u32,
        writeback_v_taps: u32,
        writeback_source_width: i64,
        writeback_destination_width: i64,
        h_total: u32,
        writeback_line_buffer_size: u32,
    ) -> f64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
