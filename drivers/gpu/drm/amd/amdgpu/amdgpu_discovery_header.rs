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

// Dependency declarations supplied by the surrounding translation unit.

pub const DISCOVERY_TMR_SIZE: u32 = 10 << 10;
pub const DISCOVERY_TMR_OFFSET: u32 = 64 << 10;

pub struct ip_discovery_top;
pub struct drm_printer;

#[repr(C)]
pub struct amdgpu_discovery_info {
    pub debugfs_blob: debugfs_blob_wrapper,
    pub ip_top: *mut ip_discovery_top,
    pub offset: u64,
    pub size: u32,
    pub bin: *mut u8,
    pub reserve_tmr: bool,
}

extern "C" {
    pub fn amdgpu_discovery_sysfs_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_discovery_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_discovery_set_ip_blocks(adev: *mut amdgpu_device) -> i32;

    pub fn amdgpu_discovery_get_nps_info(
        adev: *mut amdgpu_device,
        nps_type: *mut u32,
        ranges: *mut amdgpu_gmc_memrange,
        range_cnt: *mut i32,
        refresh: bool,
    ) -> i32;
    pub fn amdgpu_discovery_get_gc_major_minor_version(
        adev: *mut amdgpu_device,
        major: *mut u16,
        minor: *mut u16,
    ) -> i32;

    pub fn amdgpu_discovery_dump(adev: *mut amdgpu_device, p: *mut drm_printer);

    /* Early sysfs functions for persistent ip_discovery export */
    pub fn amdgpu_discovery_sysfs_early_init(
        adev: *mut amdgpu_device,
        pdev: *mut pci_dev,
    ) -> i32;
    pub fn amdgpu_discovery_sysfs_early_fini(pdev: *mut pci_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
