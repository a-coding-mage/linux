/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding AMDGPU and DF headers:
// amdgpu.h, df_v4_15.h, df/df_4_15_offset.h, df/df_4_15_sh_mask.h

unsafe fn df_v4_15_hw_init(adev: *mut amdgpu_device) {
    if (*adev).have_atomics_support {
        let mut tmp: u32;
        let dis_lcl_proc: u32 = (1u32 << 1) | (1u32 << 2) | (1u32 << 13);

        tmp = RREG32_SOC15(DF, 0, regNCSConfigurationRegister1);
        tmp |= dis_lcl_proc << NCSConfigurationRegister1__DisIntAtomicsLclProcessing__SHIFT;
        WREG32_SOC15(DF, 0, regNCSConfigurationRegister1, tmp);
    }
}

pub static df_v4_15_funcs: amdgpu_df_funcs = amdgpu_df_funcs {
    hw_init: Some(df_v4_15_hw_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
