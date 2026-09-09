/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 */

// C header guard: __JPEG_V1_0_H__

extern "C" {
    pub fn jpeg_v1_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn jpeg_v1_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn jpeg_v1_0_sw_fini(ip_block: *mut amdgpu_ip_block);
    pub fn jpeg_v1_0_start(adev: *mut amdgpu_device, mode: i32);
}

pub const JPEG_V1_REG_RANGE_START: u32 = 0x8000;
pub const JPEG_V1_REG_RANGE_END: u32 = 0x803f;

pub const JPEG_V1_LMI_JPEG_WRITE_64BIT_BAR_HIGH: u32 = 0x8238;
pub const JPEG_V1_LMI_JPEG_WRITE_64BIT_BAR_LOW: u32 = 0x8239;
pub const JPEG_V1_LMI_JPEG_READ_64BIT_BAR_HIGH: u32 = 0x825a;
pub const JPEG_V1_LMI_JPEG_READ_64BIT_BAR_LOW: u32 = 0x825b;
pub const JPEG_V1_REG_CTX_INDEX: u32 = 0x8328;
pub const JPEG_V1_REG_CTX_DATA: u32 = 0x8329;
pub const JPEG_V1_REG_SOFT_RESET: u32 = 0x83a0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
