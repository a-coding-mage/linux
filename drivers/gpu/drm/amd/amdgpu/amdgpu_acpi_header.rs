/* SPDX-License-Identifier: GPL-2.0 OR MIT
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
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

// Linux header dependencies are supplied by the surrounding translation unit.
#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct acpi_device { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_dm_backlight_caps { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }

pub const MAX_UMA_OPTION_NAME: usize = 28;
pub const MAX_UMA_OPTION_ENTRIES: usize = 19;
pub const AMDGPU_UMA_FLAG_AUTO: u32 = 1 << 1;
pub const AMDGPU_UMA_FLAG_CUSTOM: u32 = 1 << 0;

pub const AMDGPU_ATCS_PSC_DEV_STATE_D0: i32 = 0;
pub const AMDGPU_ATCS_PSC_DEV_STATE_D3_HOT: i32 = 3;
pub const AMDGPU_ATCS_PSC_DRV_STATE_OPR: i32 = 0;
pub const AMDGPU_ATCS_PSC_DRV_STATE_NOT_OPR: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_ss {
    AMDGPU_SS_DRV_LOAD,
    AMDGPU_SS_DEV_D0,
    AMDGPU_SS_DEV_D3,
    AMDGPU_SS_DRV_UNLOAD,
}

#[repr(C)]
pub struct amdgpu_uma_carveout_option {
    pub name: [core::ffi::c_char; MAX_UMA_OPTION_NAME],
    pub memory_carved_mb: u32,
    pub flags: u8,
}

#[repr(C)]
pub struct amdgpu_uma_carveout_info {
    pub num_entries: u8,
    pub uma_option_index: u8,
    pub update_lock: mutex,
    pub entries: [amdgpu_uma_carveout_option; MAX_UMA_OPTION_ENTRIES],
}

#[repr(C)]
pub struct amdgpu_numa_info {
    pub size: u64,
    pub pxm: i32,
    pub nid: i32,
}

// CONFIG_ACPI declarations and their !CONFIG_ACPI inline definitions.
#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn amdgpu_acpi_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_acpi_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_acpi_is_pcie_performance_request_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_acpi_is_power_shift_control_supported() -> bool;
    pub fn amdgpu_acpi_is_set_uma_allocation_size_supported() -> bool;
    pub fn amdgpu_acpi_pcie_performance_request(adev: *mut amdgpu_device, perf_req: u8, advertise: bool) -> i32;
    pub fn amdgpu_acpi_power_shift_control(adev: *mut amdgpu_device, dev_state: u8, drv_state: bool) -> i32;
    pub fn amdgpu_acpi_smart_shift_update(adev: *mut amdgpu_device, ss_state: amdgpu_ss) -> i32;
    pub fn amdgpu_acpi_set_uma_allocation_size(adev: *mut amdgpu_device, index: u8, type_: u8) -> i32;
    pub fn amdgpu_acpi_pcie_notify_device_ready(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_acpi_get_tmr_info(adev: *mut amdgpu_device, tmr_offset: *mut u64, tmr_size: *mut u64) -> i32;
    pub fn amdgpu_acpi_get_mem_info(adev: *mut amdgpu_device, xcc_id: i32, numa_info: *mut amdgpu_numa_info) -> i32;
    pub fn amdgpu_acpi_get_backlight_caps(caps: *mut amdgpu_dm_backlight_caps);
    pub fn amdgpu_acpi_should_gpu_reset(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_acpi_detect();
    pub fn amdgpu_acpi_release();
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_init(_: *mut amdgpu_device) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_get_tmr_info(_: *mut amdgpu_device, _: *mut u64, _: *mut u64) -> i32 { -22 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_get_mem_info(_: *mut amdgpu_device, _: i32, _: *mut amdgpu_numa_info) -> i32 { -22 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_fini(_: *mut amdgpu_device) {}
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_should_gpu_reset(_: *mut amdgpu_device) -> bool { false }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_detect() {}
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_release() {}
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_is_power_shift_control_supported() -> bool { false }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_is_set_uma_allocation_size_supported() -> bool { false }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_power_shift_control(_: *mut amdgpu_device, _: u8, _: bool) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_smart_shift_update(_: *mut amdgpu_device, _: amdgpu_ss) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_set_uma_allocation_size(_: *mut amdgpu_device, _: u8, _: u8) -> i32 { -22 }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn amdgpu_acpi_get_backlight_caps(_: *mut amdgpu_dm_backlight_caps) {}

#[cfg(all(CONFIG_ACPI, CONFIG_SUSPEND))]
extern "C" {
    pub fn amdgpu_acpi_is_s3_active(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_acpi_is_s0ix_active(adev: *mut amdgpu_device) -> bool;
}
#[cfg(not(all(CONFIG_ACPI, CONFIG_SUSPEND)))]
pub unsafe fn amdgpu_acpi_is_s0ix_active(_: *mut amdgpu_device) -> bool { false }
#[cfg(not(all(CONFIG_ACPI, CONFIG_SUSPEND)))]
pub unsafe fn amdgpu_acpi_is_s3_active(_: *mut amdgpu_device) -> bool { false }

#[cfg(CONFIG_DRM_AMD_ISP)]
extern "C" {
    pub fn amdgpu_acpi_get_isp4_dev(dev: *mut *mut acpi_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
