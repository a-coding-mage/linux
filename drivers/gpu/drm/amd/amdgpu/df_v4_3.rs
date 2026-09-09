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
 */

// Dependencies supplied by the surrounding amdgpu and DF modules:
// amdgpu.h, df_v4_3.h, df/df_4_3_offset.h, df/df_4_3_sh_mask.h

unsafe fn df_v4_3_query_ras_poison_mode(adev: *mut amdgpu_device) -> bool {
    let hw_assert_msklo: u32;
    let hw_assert_mskhi: u32;
    let v0: u32;
    let v1: u32;
    let v28: u32;
    let v31: u32;

    hw_assert_msklo = RREG32_SOC15(
        DF,
        0,
        regDF_CS_UMC_AON0_HardwareAssertMaskLow,
    );
    hw_assert_mskhi = RREG32_SOC15(
        DF,
        0,
        regDF_NCS_PG0_HardwareAssertMaskHigh,
    );

    v0 = REG_GET_FIELD(
        hw_assert_msklo,
        DF_CS_UMC_AON0_HardwareAssertMaskLow,
        HWAssertMsk0,
    );
    v1 = REG_GET_FIELD(
        hw_assert_msklo,
        DF_CS_UMC_AON0_HardwareAssertMaskLow,
        HWAssertMsk1,
    );
    v28 = REG_GET_FIELD(
        hw_assert_mskhi,
        DF_NCS_PG0_HardwareAssertMaskHigh,
        HWAssertMsk28,
    );
    v31 = REG_GET_FIELD(
        hw_assert_mskhi,
        DF_NCS_PG0_HardwareAssertMaskHigh,
        HWAssertMsk31,
    );

    if v0 != 0 && v1 != 0 && v28 != 0 && v31 != 0 {
        true
    } else if v0 == 0 && v1 == 0 && v28 == 0 && v31 == 0 {
        false
    } else {
        dev_warn(
            (*adev).dev,
            "DF poison setting is inconsistent({}:{}:{}:{})!\n",
            v0,
            v1,
            v28,
            v31,
        );
        false
    }
}

pub static df_v4_3_funcs: amdgpu_df_funcs = amdgpu_df_funcs {
    query_ras_poison_mode: Some(df_v4_3_query_ras_poison_mode),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
