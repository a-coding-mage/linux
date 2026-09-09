/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn smu9_get_bamaco_support(hwmgr: *mut pp_hwmgr) -> i32 {
    let adev = (*hwmgr).adev as *mut amdgpu_device;
    let mut reg: u32;
    let mut data: u32;

    if !phm_cap_enabled(
        (*hwmgr).platform_descriptor.platformCaps,
        PHM_PlatformCaps_BACO,
    ) {
        return 0;
    }

    WREG32(0x12074, 0xFFF0003B);
    data = RREG32(0x12075);

    if data == 0x1 {
        reg = RREG32_SOC15(NBIF, 0, mmRCC_BIF_STRAP0);

        if reg & RCC_BIF_STRAP0__STRAP_PX_CAPABLE_MASK != 0 {
            return BACO_SUPPORT;
        }
    }

    0
}

pub unsafe fn smu9_baco_get_state(
    hwmgr: *mut pp_hwmgr,
    state: *mut BACO_STATE,
) -> i32 {
    let _adev = (*hwmgr).adev as *mut amdgpu_device;
    let reg: u32;

    reg = RREG32_SOC15(NBIF, 0, mmBACO_CNTL);

    if reg & BACO_CNTL__BACO_MODE_MASK != 0 {
        // gfx has already entered BACO state
        *state = BACO_STATE_IN;
    } else {
        *state = BACO_STATE_OUT;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
