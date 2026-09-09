/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// C dependencies supplied by the surrounding driver are intentionally left external.

use core::ffi::{c_int, c_uint};

pub const REG_ATHUB_MISC_CNTL_V3_0_1: u32 = 0x00d7;
pub const REG_ATHUB_MISC_CNTL_V3_0_1_BASE_IDX: u32 = 0;
pub const REG_ATHUB_MISC_CNTL_V3_3_0: u32 = 0x00d8;
pub const REG_ATHUB_MISC_CNTL_V3_3_0_BASE_IDX: u32 = 0;

#[repr(C)]
pub struct amdgpu_device {
    pub cg_flags: u32,
}

pub type amd_clockgating_state = c_int;
pub const AMD_CG_STATE_GATE: amd_clockgating_state = 1;

pub const ATHUB_HWIP: u32 = 0;
pub const AMD_CG_SUPPORT_ATHUB_MGCG: u32 = 1 << 0;
pub const AMD_CG_SUPPORT_ATHUB_LS: u32 = 1 << 1;
pub const ATHUB_MISC_CNTL__CG_ENABLE_MASK: u32 = 1 << 0;
pub const ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK: u32 = 1 << 1;
pub const REG_ATHUB_MISC_CNTL: u32 = 0;

const fn ip_version(major: u32, minor: u32, revision: u32) -> u32 {
    (major << 24) | (minor << 16) | revision
}

extern "C" {
    fn amdgpu_ip_version(adev: *mut amdgpu_device, hwip: u32, instance: u32) -> u32;
    fn amdgpu_sriov_vf(adev: *mut amdgpu_device) -> bool;
    fn rreg32_soc15(adev: *mut amdgpu_device, hwip: u32, instance: u32, reg: u32) -> u32;
    fn wreg32_soc15(adev: *mut amdgpu_device, hwip: u32, instance: u32, reg: u32, data: u32);
}

unsafe fn athub_v3_0_get_cg_cntl(adev: *mut amdgpu_device) -> u32 {
    match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        x if x == ip_version(3, 0, 1) => rreg32_soc15(adev, ATHUB_HWIP, 0, REG_ATHUB_MISC_CNTL_V3_0_1),
        x if x == ip_version(3, 3, 0) || x == ip_version(3, 4, 2) => {
            rreg32_soc15(adev, ATHUB_HWIP, 0, REG_ATHUB_MISC_CNTL_V3_3_0)
        }
        _ => rreg32_soc15(adev, ATHUB_HWIP, 0, REG_ATHUB_MISC_CNTL),
    }
}

unsafe fn athub_v3_0_set_cg_cntl(adev: *mut amdgpu_device, data: u32) {
    let reg = match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        x if x == ip_version(3, 0, 1) => REG_ATHUB_MISC_CNTL_V3_0_1,
        x if x == ip_version(3, 3, 0) || x == ip_version(3, 4, 2) => REG_ATHUB_MISC_CNTL_V3_3_0,
        _ => REG_ATHUB_MISC_CNTL,
    };
    wreg32_soc15(adev, ATHUB_HWIP, 0, reg, data);
}

unsafe fn athub_v3_0_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let def = athub_v3_0_get_cg_cntl(adev);
    let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_ATHUB_MGCG) != 0 {
        data |= ATHUB_MISC_CNTL__CG_ENABLE_MASK;
    } else {
        data &= !ATHUB_MISC_CNTL__CG_ENABLE_MASK;
    }
    if def != data { athub_v3_0_set_cg_cntl(adev, data); }
}

unsafe fn athub_v3_0_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    let def = athub_v3_0_get_cg_cntl(adev);
    let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_ATHUB_LS) != 0 {
        data |= ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK;
    } else {
        data &= !ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK;
    }
    if def != data { athub_v3_0_set_cg_cntl(adev, data); }
}

pub unsafe fn athub_v3_0_set_clockgating(adev: *mut amdgpu_device, state: amd_clockgating_state) -> c_int {
    if amdgpu_sriov_vf(adev) { return 0; }
    match amdgpu_ip_version(adev, ATHUB_HWIP, 0) {
        x if x == ip_version(3, 0, 0) || x == ip_version(3, 0, 1) || x == ip_version(3, 0, 2)
            || x == ip_version(3, 3, 0) || x == ip_version(3, 4, 2) => {
                let enable = state == AMD_CG_STATE_GATE;
                athub_v3_0_update_medium_grain_clock_gating(adev, enable);
                athub_v3_0_update_medium_grain_light_sleep(adev, enable);
            }
        _ => {}
    }
    0
}

pub unsafe fn athub_v3_0_get_clockgating(adev: *mut amdgpu_device, flags: *mut u64) {
    let data = athub_v3_0_get_cg_cntl(adev) as c_int;
    if (data as u32 & ATHUB_MISC_CNTL__CG_ENABLE_MASK) != 0 { *flags |= AMD_CG_SUPPORT_ATHUB_MGCG as u64; }
    if (data as u32 & ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK) != 0 { *flags |= AMD_CG_SUPPORT_ATHUB_LS as u64; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
