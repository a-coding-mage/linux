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

// Dependencies supplied by the surrounding AMDGPU translation.

unsafe fn athub_v4_1_0_get_cg_cntl(adev: *mut amdgpu_device) -> u32 {
    let data: u32;

    match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        IP_VERSION(4, 1, 0) => {
            data = RREG32_SOC15(ATHUB, 0, regATHUB_MISC_CNTL);
        }
        _ => {
            data = 0;
        }
    }
    data
}

unsafe fn athub_v4_1_0_set_cg_cntl(adev: *mut amdgpu_device, data: u32) {
    match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        IP_VERSION(4, 1, 0) => {
            WREG32_SOC15(ATHUB, 0, regATHUB_MISC_CNTL, data);
        }
        _ => {}
    }
}

unsafe fn athub_v4_1_0_update_medium_grain_clock_gating(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    let def: u32;
    let mut data: u32;

    def = athub_v4_1_0_get_cg_cntl(adev);
    data = def;

    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_ATHUB_MGCG) != 0 {
        data |= ATHUB_MISC_CNTL__CG_ENABLE_MASK;
    } else {
        data &= !ATHUB_MISC_CNTL__CG_ENABLE_MASK;
    }

    if def != data {
        athub_v4_1_0_set_cg_cntl(adev, data);
    }
}

unsafe fn athub_v4_1_0_update_medium_grain_light_sleep(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    let def: u32;
    let mut data: u32;

    def = athub_v4_1_0_get_cg_cntl(adev);
    data = def;

    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_ATHUB_LS) != 0 {
        data |= ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK;
    } else {
        data &= !ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK;
    }

    if def != data {
        athub_v4_1_0_set_cg_cntl(adev, data);
    }
}

pub unsafe fn athub_v4_1_0_set_clockgating(
    adev: *mut amdgpu_device,
    state: amd_clockgating_state,
) -> i32 {
    if amdgpu_sriov_vf(adev) {
        return 0;
    }

    match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        IP_VERSION(4, 1, 0) => {
            athub_v4_1_0_update_medium_grain_clock_gating(
                adev,
                state == AMD_CG_STATE_GATE,
            );
            athub_v4_1_0_update_medium_grain_light_sleep(
                adev,
                state == AMD_CG_STATE_GATE,
            );
        }
        _ => {}
    }

    0
}

pub unsafe fn athub_v4_1_0_get_clockgating(adev: *mut amdgpu_device, flags: *mut u64) {
    let data: i32;

    /* AMD_CG_SUPPORT_ATHUB_MGCG */
    data = athub_v4_1_0_get_cg_cntl(adev) as i32;
    if (data & ATHUB_MISC_CNTL__CG_ENABLE_MASK as i32) != 0 {
        *flags |= AMD_CG_SUPPORT_ATHUB_MGCG as u64;
    }

    /* AMD_CG_SUPPORT_ATHUB_LS */
    if (data & ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK as i32) != 0 {
        *flags |= AMD_CG_SUPPORT_ATHUB_LS as u64;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
