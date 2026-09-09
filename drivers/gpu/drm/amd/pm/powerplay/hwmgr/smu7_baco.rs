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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// amdgpu.h, smu7_baco.h, tonga_baco.h, fiji_baco.h, polaris_baco.h,
// ci_baco.h, bif/bif_5_0_d.h, bif/bif_5_0_sh_mask.h,
// smu/smu_7_1_2_d.h, and smu/smu_7_1_2_sh_mask.h.

pub unsafe fn smu7_get_bamaco_support(hwmgr: *mut pp_hwmgr) -> i32 {
    let adev: *mut amdgpu_device = (*hwmgr).adev as *mut amdgpu_device;
    let reg: u32;

    if !phm_cap_enabled(
        (*hwmgr).platform_descriptor.platformCaps,
        PHM_PlatformCaps_BACO,
    ) {
        return 0;
    }

    reg = RREG32!(adev, mmCC_BIF_BX_FUSESTRAP0);

    if reg & CC_BIF_BX_FUSESTRAP0__STRAP_BIF_PX_CAPABLE_MASK != 0 {
        return BACO_SUPPORT;
    }

    0
}

pub unsafe fn smu7_baco_get_state(
    hwmgr: *mut pp_hwmgr,
    state: *mut BACO_STATE,
) -> i32 {
    let adev: *mut amdgpu_device = (*hwmgr).adev as *mut amdgpu_device;
    let reg: u32 = RREG32!(adev, mmBACO_CNTL);

    if reg & BACO_CNTL__BACO_MODE_MASK != 0 {
        // gfx has already entered BACO state
        *state = BACO_STATE_IN;
    } else {
        *state = BACO_STATE_OUT;
    }
    0
}

pub unsafe fn smu7_baco_set_state(hwmgr: *mut pp_hwmgr, state: BACO_STATE) -> i32 {
    let adev: *mut amdgpu_device = (*hwmgr).adev as *mut amdgpu_device;

    match (*adev).asic_type {
        CHIP_TOPAZ | CHIP_TONGA => tonga_baco_set_state(hwmgr, state),
        CHIP_FIJI => fiji_baco_set_state(hwmgr, state),
        CHIP_POLARIS10 | CHIP_POLARIS11 | CHIP_POLARIS12 | CHIP_VEGAM => {
            polaris_baco_set_state(hwmgr, state)
        }
        // CONFIG_DRM_AMDGPU_CIK conditional retained from the C source.
        #[cfg(CONFIG_DRM_AMDGPU_CIK)]
        CHIP_BONAIRE | CHIP_HAWAII => ci_baco_set_state(hwmgr, state),
        _ => -EINVAL,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
