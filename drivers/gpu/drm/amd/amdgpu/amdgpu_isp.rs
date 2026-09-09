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
 */

const ISP_MC_ADDR_ALIGN: u64 = 1024 * 32;

static unsafe fn isp_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let isp = &mut (*adev).isp;
    if !(*isp).funcs.is_null() && (*(*isp).funcs).hw_init.is_some() {
        return ((*(*isp).funcs).hw_init.unwrap())(isp);
    }
    -ENODEV
}

static unsafe fn isp_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let isp = &mut (*(*ip_block).adev).isp;
    if !(*isp).funcs.is_null() && (*(*isp).funcs).hw_fini.is_some() {
        return ((*(*isp).funcs).hw_fini.unwrap())(isp);
    }
    -ENODEV
}

static unsafe fn isp_load_fw_by_psp(adev: *mut amdgpu_device) -> i32 {
    let mut ucode_prefix = [0i8; 10];
    let mut r = 0i32;
    amdgpu_ucode_ip_version_decode(adev, ISP_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    r = amdgpu_ucode_request(
        adev,
        &mut (*adev).isp.fw,
        AMDGPU_UCODE_OPTIONAL,
        b"amdgpu/%s.bin\0".as_ptr() as *const i8,
        ucode_prefix.as_mut_ptr(),
    );
    if r != 0 {
        amdgpu_ucode_release(&mut (*adev).isp.fw);
        return r;
    }
    let hdr = (*adev).isp.fw.data as *const common_firmware_header;
    (*adev).firmware.ucode[AMDGPU_UCODE_ID_ISP].ucode_id = AMDGPU_UCODE_ID_ISP;
    (*adev).firmware.ucode[AMDGPU_UCODE_ID_ISP].fw = (*adev).isp.fw;
    (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*hdr).ucode_size_bytes), PAGE_SIZE);
    r
}

static unsafe fn isp_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let isp = &mut (*adev).isp;
    match amdgpu_ip_version(adev, ISP_HWIP, 0) {
        v if v == IP_VERSION(4, 1, 0) => isp_v4_1_0_set_isp_funcs(isp),
        v if v == IP_VERSION(4, 1, 1) => isp_v4_1_1_set_isp_funcs(isp),
        _ => return -EINVAL,
    }
    (*isp).adev = adev;
    (*isp).parent = (*adev).dev;
    if isp_load_fw_by_psp(adev) != 0 {
        DRM_DEBUG_DRIVER(b"%s: isp fw load failed\\n\0".as_ptr() as *const i8, b"isp_early_init\0".as_ptr() as *const i8);
        return -ENOENT;
    }
    0
}

static unsafe fn isp_is_idle(_ip_block: *mut amdgpu_ip_block) -> bool { true }

static unsafe fn isp_set_clockgating_state(
    _ip_block: *mut amdgpu_ip_block,
    _state: amd_clockgating_state,
) -> i32 { 0 }

static unsafe fn isp_set_powergating_state(
    _ip_block: *mut amdgpu_ip_block,
    _state: amd_powergating_state,
) -> i32 { 0 }

static unsafe fn is_valid_isp_device(isp_parent: *mut device, amdgpu_dev: *mut device) -> i32 {
    if isp_parent != amdgpu_dev { return -EINVAL; }
    0
}

pub unsafe fn isp_user_buffer_alloc(
    dev: *mut device, dmabuf: *mut c_void, buf_obj: *mut *mut c_void, buf_addr: *mut u64,
) -> i32 {
    let ispdev = to_platform_device(dev);
    if WARN_ON(ispdev.is_null()) { return -ENODEV; }
    if WARN_ON(buf_obj.is_null()) { return -EINVAL; }
    if WARN_ON(buf_addr.is_null()) { return -EINVAL; }
    let mfd_cell = &mut (*ispdev).mfd_cell[0];
    if mfd_cell.is_null() { return -ENODEV; }
    let isp_pdata = (*mfd_cell).platform_data as *const isp_platform_data;
    let adev = (*isp_pdata).adev;
    let ret = is_valid_isp_device((*ispdev).dev.parent, (*adev).dev);
    if ret != 0 { return ret; }
    let mut bo: *mut amdgpu_bo = core::ptr::null_mut();
    let mut gpu_addr = 0u64;
    let ret = amdgpu_bo_create_isp_user(adev, dmabuf, AMDGPU_GEM_DOMAIN_GTT, &mut bo, &mut gpu_addr);
    if ret != 0 {
        drm_err(&(*adev).ddev, b"failed to alloc gart user buffer (%d)\0".as_ptr() as *const i8, ret);
        return ret;
    }
    *buf_obj = bo as *mut c_void;
    *buf_addr = gpu_addr;
    0
}

pub unsafe fn isp_user_buffer_free(buf_obj: *mut c_void) { amdgpu_bo_free_isp_user(buf_obj); }

pub unsafe fn isp_kernel_buffer_alloc(
    dev: *mut device, size: u64, buf_obj: *mut *mut c_void,
    gpu_addr: *mut u64, cpu_addr: *mut *mut c_void,
) -> i32 {
    let ispdev = to_platform_device(dev);
    let bo = buf_obj as *mut *mut amdgpu_bo;
    if WARN_ON(ispdev.is_null()) { return -ENODEV; }
    if WARN_ON(buf_obj.is_null()) { return -EINVAL; }
    if WARN_ON(gpu_addr.is_null()) { return -EINVAL; }
    if WARN_ON(cpu_addr.is_null()) { return -EINVAL; }
    let mfd_cell = &mut (*ispdev).mfd_cell[0];
    if mfd_cell.is_null() { return -ENODEV; }
    let isp_pdata = (*mfd_cell).platform_data as *const isp_platform_data;
    let adev = (*isp_pdata).adev;
    let ret = is_valid_isp_device((*ispdev).dev.parent, (*adev).dev);
    if ret != 0 { return ret; }
    *bo = core::ptr::null_mut();
    let ret = amdgpu_bo_create_kernel(adev, size, ISP_MC_ADDR_ALIGN, AMDGPU_GEM_DOMAIN_GTT, bo, gpu_addr, cpu_addr);
    if cpu_addr.is_null() || ret != 0 {
        drm_err(&(*adev).ddev, b"failed to alloc gart kernel buffer (%d)\0".as_ptr() as *const i8, ret);
        return ret;
    }
    0
}

pub unsafe fn isp_kernel_buffer_free(buf_obj: *mut *mut c_void, gpu_addr: *mut u64, cpu_addr: *mut *mut c_void) {
    amdgpu_bo_free_kernel(buf_obj as *mut *mut amdgpu_bo, gpu_addr, cpu_addr);
}

static unsafe fn isp_resume(ip_block: *mut amdgpu_ip_block) -> i32 {
    let isp = &mut (*(*ip_block).adev).isp;
    if !(*isp).funcs.is_null() && (*(*isp).funcs).hw_resume.is_some() { return ((*(*isp).funcs).hw_resume.unwrap())(isp); }
    -ENODEV
}

static unsafe fn isp_suspend(ip_block: *mut amdgpu_ip_block) -> i32 {
    let isp = &mut (*(*ip_block).adev).isp;
    if !(*isp).funcs.is_null() && (*(*isp).funcs).hw_suspend.is_some() { return ((*(*isp).funcs).hw_suspend.unwrap())(isp); }
    -ENODEV
}

static isp_ip_funcs: amd_ip_funcs = amd_ip_funcs {
    name: b"isp_ip\0".as_ptr() as *const i8,
    early_init: Some(isp_early_init), hw_init: Some(isp_hw_init), hw_fini: Some(isp_hw_fini),
    is_idle: Some(isp_is_idle), suspend: Some(isp_suspend), resume: Some(isp_resume),
    set_clockgating_state: Some(isp_set_clockgating_state), set_powergating_state: Some(isp_set_powergating_state),
};

pub static isp_v4_1_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_ISP, major: 4, minor: 1, rev: 0, funcs: &isp_ip_funcs };
pub static isp_v4_1_1_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_ISP, major: 4, minor: 1, rev: 1, funcs: &isp_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
