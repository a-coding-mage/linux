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

pub const VCNIPJPEG_CGC_GATE: u32 = 0x4160;
pub const VCNIPJPEG_CGC_CTRL: u32 = 0x4161;
pub const VCNIPJPEG_SYS_INT_EN: u32 = 0x4141;
pub const VCNIPUVD_NO_OP: u32 = 0x0029;
pub const VCNIPJPEG_DEC_GFX10_ADDR_CONFIG: u32 = 0x404A;

extern "C" {
    pub static jpeg_v5_0_0_ip_block: amdgpu_ip_block_version;

    pub fn jpeg_v5_0_0_process_interrupt(
        adev: *mut amdgpu_device,
        source: *mut amdgpu_irq_src,
        entry: *mut amdgpu_iv_entry,
    ) -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
