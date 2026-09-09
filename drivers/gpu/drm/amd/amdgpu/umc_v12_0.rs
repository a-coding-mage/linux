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
 */

// Dependencies supplied by the surrounding translation unit.

pub unsafe fn umc_v12_0_is_uncorrectable_error(
    adev: *mut amdgpu_device,
    mc_umc_status: u64,
) -> bool {
    (REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1)
        && (REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1
            || REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1
            || REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1)
}

pub unsafe fn umc_v12_0_is_correctable_error(
    adev: *mut amdgpu_device,
    mc_umc_status: u64,
) -> bool {
    (REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1
        && (REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1
            || (REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1
                && REG_GET_FIELD!(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 0)
            // Identify data parity error in replay mode
            || ((REG_GET_FIELD!(
                mc_umc_status,
                MCA_UMC_UMC0_MCUMC_STATUST0,
                ErrorCodeExt
            ) == 0x5
                || REG_GET_FIELD!(
                    mc_umc_status,
                    MCA_UMC_UMC0_MCUMC_STATUST0,
                    ErrorCodeExt
                ) == 0xb)
                && !umc_v12_0_is_uncorrectable_error(adev, mc_umc_status))))
}

pub static mut umc_v12_0_ras: amdgpu_umc_ras = amdgpu_umc_ras {
    ras_block: amdgpu_ras_block {
        hw_ops: core::ptr::null_mut(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
