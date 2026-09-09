/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * Author: Huang Rui
 *
 */

// Dependency supplied by the surrounding translation unit: `amdgpu_psp.h`.

pub const PSP_DIRECTORY_TABLE_ENTRIES: i32 = 4;
pub const PSP_BINARY_ALIGNMENT: i32 = 64;
pub const PSP_BOOTLOADER_1_MEG_ALIGNMENT: i32 = 0x100000;
pub const PSP_BOOTLOADER_8_MEM_ALIGNMENT: i32 = 0x800000;

extern "C" {
    pub fn psp_v3_1_set_psp_funcs(psp: *mut psp_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
