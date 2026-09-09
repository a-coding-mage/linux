/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// C header guard omitted; declarations below depend on these external types.
pub type source_format_class = i32;
pub type dm_swizzle_mode = i32;

#[repr(C)]
pub struct display_mode_lib {
    _private: [u8; 0],
}

extern "C" {
    pub fn dml30_recalculate(mode_lib: *mut display_mode_lib);
    pub fn dml30_ModeSupportAndSystemConfigurationFull(mode_lib: *mut display_mode_lib);
    pub fn dml30_CalculateWriteBackDISPCLK(
        WritebackPixelFormat: source_format_class,
        PixelClock: f64,
        WritebackHRatio: f64,
        WritebackVRatio: f64,
        WritebackHTaps: u32,
        WritebackVTaps: u32,
        WritebackSourceWidth: isize,
        WritebackDestinationWidth: isize,
        HTotal: u32,
        WritebackLineBufferSize: u32,
    ) -> f64;
    pub fn dml30_CalculateBytePerPixelAnd256BBlockSizes(
        SourcePixelFormat: source_format_class,
        SurfaceTiling: dm_swizzle_mode,
        BytePerPixelY: *mut u32,
        BytePerPixelC: *mut u32,
        BytePerPixelDETY: *mut f64,
        BytePerPixelDETC: *mut f64,
        BlockHeight256BytesY: *mut u32,
        BlockHeight256BytesC: *mut u32,
        BlockWidth256BytesY: *mut u32,
        BlockWidth256BytesC: *mut u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
