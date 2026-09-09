/* SPDX-License-Identifier: GPL-2.0 OR MIT
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 */

// Dependency intent from <linux/types.h>: u32 is supplied by the surrounding
// translation environment.

macro_rules! codec_info_build {
    ($type:expr, $width:expr, $height:expr, $level:expr) => {
        .codec_type = $type,
        .max_width = $width,
        .max_height = $height,
        .max_pixels_per_frame = $height * $width,
        .max_level = $level,
    };
}

#[repr(C)]
pub struct amdgpu_video_codec_info {
    pub codec_type: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_pixels_per_frame: u32,
    pub max_level: u32,
}

#[repr(C)]
pub struct amdgpu_video_codecs {
    pub codec_count: u32,
    pub codec_array: *const amdgpu_video_codec_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
