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

#[repr(C)]
pub enum imu_work_mode {
    DEBUG_MODE,
    MISSION_MODE,
}

#[repr(C)]
pub struct amdgpu_imu_funcs {
    pub init_microcode: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub load_microcode: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub setup_imu: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub start_imu: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub program_rlc_ram: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub wait_for_reset_status: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub switch_compute_partition: Option<
        unsafe extern "C" fn(
            adev: *mut amdgpu_device,
            num_xccs_per_xcp: i32,
            compute_partition_mode: i32,
        ) -> i32,
    >,
}

#[repr(C)]
pub struct imu_rlc_ram_golden {
    pub hwip: u32,
    pub instance: u32,
    pub segment: u32,
    pub reg: u32,
    pub data: u32,
    pub addr_mask: u32,
}

/* C token-pasting macro: { ip##_HWIP, inst, reg##_BASE_IDX, reg, data, addr_mask }.
 * Rust callers provide the already-resolved HWIP and BASE_IDX expressions. */
#[macro_export]
macro_rules! IMU_RLC_RAM_GOLDEN_VALUE {
    ($ip_hwip:expr, $inst:expr, $reg_base_idx:expr, $reg:expr, $data:expr, $addr_mask:expr) => {
        $crate::imu_rlc_ram_golden {
            hwip: $ip_hwip,
            instance: $inst,
            segment: $reg_base_idx,
            reg: $reg,
            data: $data,
            addr_mask: $addr_mask,
        }
    };
}

#[repr(C)]
pub struct amdgpu_imu {
    pub funcs: *const amdgpu_imu_funcs,
    pub mode: imu_work_mode,
}

/* Opaque type supplied by the including translation unit. */
pub struct amdgpu_device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
