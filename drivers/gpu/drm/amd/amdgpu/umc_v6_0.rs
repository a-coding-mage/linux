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

unsafe fn umc_v6_0_init_registers(adev: *mut amdgpu_device) {
    let mut i: u32;
    let mut j: u32;

    i = 0;
    while i < 4 {
        j = 0;
        while j < 4 {
            WREG32!((i.wrapping_mul(0x100000)
                .wrapping_add(0x5010c)
                .wrapping_add(j.wrapping_mul(0x2000))) / 4, 0x1002);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}

pub static umc_v6_0_funcs: amdgpu_umc_funcs = amdgpu_umc_funcs {
    init_registers: Some(umc_v6_0_init_registers),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
