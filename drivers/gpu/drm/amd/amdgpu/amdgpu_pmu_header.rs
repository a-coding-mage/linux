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
 */

/* PMU types. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amdgpu_pmu_perf_type {
    AMDGPU_PMU_PERF_TYPE_NONE = 0,
    AMDGPU_PMU_PERF_TYPE_DF,
    AMDGPU_PMU_PERF_TYPE_ALL,
}

/*
 * PMU type AMDGPU_PMU_PERF_TYPE_ALL can hold events of different "type"
 * configurations.  Event config types are parsed from the 64-bit raw
 * config (See EVENT_CONFIG_TYPE_SHIFT and EVENT_CONFIG_TYPE_MASK) and
 * are registered into the HW perf events config_base.
 *
 * PMU types with only a single event configuration type
 * (non-AMDGPU_PMU_PERF_TYPE_ALL) have their event config type auto generated
 * when the performance counter is added.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amdgpu_pmu_event_config_type {
    AMDGPU_PMU_EVENT_CONFIG_TYPE_NONE = 0,
    AMDGPU_PMU_EVENT_CONFIG_TYPE_DF,
    AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI,
    AMDGPU_PMU_EVENT_CONFIG_TYPE_MAX,
}

pub const AMDGPU_PMU_EVENT_CONFIG_TYPE_SHIFT: u32 = 56;
pub const AMDGPU_PMU_EVENT_CONFIG_TYPE_MASK: u32 = 0xff;

/* Supplied by the AMDGPU implementation. */
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_pmu_init(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_pmu_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
