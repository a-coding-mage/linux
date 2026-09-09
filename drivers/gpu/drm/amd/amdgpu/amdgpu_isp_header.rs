/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2024 Advanced Micro Devices, Inc. All rights reserved.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 */

// C dependencies: <drm/amd/isp.h>, <linux/pm_domain.h>

pub const ISP_REGS_OFFSET_END: u32 = 0x629A4;

#[repr(C)]
pub struct isp_funcs {
    pub hw_init: Option<unsafe extern "C" fn(isp: *mut amdgpu_isp) -> core::ffi::c_int>,
    pub hw_fini: Option<unsafe extern "C" fn(isp: *mut amdgpu_isp) -> core::ffi::c_int>,
    pub hw_suspend: Option<unsafe extern "C" fn(isp: *mut amdgpu_isp) -> core::ffi::c_int>,
    pub hw_resume: Option<unsafe extern "C" fn(isp: *mut amdgpu_isp) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct amdgpu_isp {
    pub parent: *mut device,
    pub adev: *mut amdgpu_device,
    pub funcs: *const isp_funcs,
    pub isp_cell: *mut mfd_cell,
    pub isp_res: *mut resource,
    pub isp_i2c_res: *mut resource,
    pub isp_gpio_res: *mut resource,
    pub isp_pdata: *mut isp_platform_data,
    pub harvest_config: core::ffi::c_uint,
    pub fw: *const firmware,
    pub ispgpd: generic_pm_domain,
}

extern "C" {
    pub static isp_v4_1_0_ip_block: amdgpu_ip_block_version;
    pub static isp_v4_1_1_ip_block: amdgpu_ip_block_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
