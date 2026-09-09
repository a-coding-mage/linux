// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

/// amdgpu_mm_rdoorbell - read a doorbell dword
pub unsafe fn amdgpu_mm_rdoorbell(adev: *mut amdgpu_device, index: u32) -> u32 {
    if amdgpu_device_skip_hw_access(adev) { return 0; }
    if index < (*adev).doorbell.num_kernel_doorbells {
        return readl((*adev).doorbell.cpu_addr.add(index as usize));
    }
    dev_err((*adev).dev, "reading beyond doorbell aperture: 0x%08x!\n", index);
    0
}

/// amdgpu_mm_wdoorbell - write a doorbell dword
pub unsafe fn amdgpu_mm_wdoorbell(adev: *mut amdgpu_device, index: u32, v: u32) {
    if amdgpu_device_skip_hw_access(adev) { return; }
    if index < (*adev).doorbell.num_kernel_doorbells {
        writel(v, (*adev).doorbell.cpu_addr.add(index as usize));
    } else {
        dev_err((*adev).dev, "writing beyond doorbell aperture: 0x%08x!\n", index);
    }
}

/// amdgpu_mm_rdoorbell64 - read a doorbell Qword
pub unsafe fn amdgpu_mm_rdoorbell64(adev: *mut amdgpu_device, index: u32) -> u64 {
    if amdgpu_device_skip_hw_access(adev) { return 0; }
    if index < (*adev).doorbell.num_kernel_doorbells {
        return atomic64_read((*adev).doorbell.cpu_addr.add(index as usize) as *mut atomic64_t);
    }
    dev_err((*adev).dev, "reading beyond doorbell aperture: 0x%08x!\n", index);
    0
}

/// amdgpu_mm_wdoorbell64 - write a doorbell Qword
pub unsafe fn amdgpu_mm_wdoorbell64(adev: *mut amdgpu_device, index: u32, v: u64) {
    if amdgpu_device_skip_hw_access(adev) { return; }
    if index < (*adev).doorbell.num_kernel_doorbells {
        atomic64_set((*adev).doorbell.cpu_addr.add(index as usize) as *mut atomic64_t, v);
    } else {
        dev_err((*adev).dev, "writing beyond doorbell aperture: 0x%08x!\n", index);
    }
}

/// amdgpu_doorbell_index_on_bar - Find doorbell's absolute offset in BAR
pub unsafe fn amdgpu_doorbell_index_on_bar(
    _adev: *mut amdgpu_device, db_bo: *mut amdgpu_bo, doorbell_index: u32, db_size: u32,
) -> u32 {
    let db_bo_offset: i32 = amdgpu_bo_gpu_offset(db_bo);
    (db_bo_offset as u32) / size_of::<u32>() as u32
        + doorbell_index * ((db_size + 4 - 1) / 4)
}

/// amdgpu_doorbell_create_kernel_doorbells - Create kernel doorbells for graphics
pub unsafe fn amdgpu_doorbell_create_kernel_doorbells(adev: *mut amdgpu_device) -> i32 {
    let mut r: i32;
    let mut size: i32;
    if (*adev).doorbell.num_kernel_doorbells == 0 { return 0; }
    size = align((*adev).doorbell.num_kernel_doorbells * size_of::<u32>() as u32, PAGE_SIZE) as i32;
    (*adev).mes.db_start_dw_offset = (size as usize / size_of::<u32>()) as _;
    size += PAGE_SIZE as i32;
    r = amdgpu_bo_create_kernel(adev, size as _, PAGE_SIZE, AMDGPU_GEM_DOMAIN_DOORBELL,
        &mut (*adev).doorbell.kernel_doorbells, core::ptr::null_mut(),
        &mut (*adev).doorbell.cpu_addr as *mut _ as *mut *mut core::ffi::c_void);
    if r != 0 { dev_err((*adev).dev, "Failed to allocate kernel doorbells, err=%d\n", r); return r; }
    (*adev).doorbell.num_kernel_doorbells = (size as u32) / size_of::<u32>() as u32;
    0
}

/// amdgpu_doorbell_init - Init doorbell driver information.
pub unsafe fn amdgpu_doorbell_init(adev: *mut amdgpu_device) -> i32 {
    if (*adev).asic_type < CHIP_BONAIRE {
        (*adev).doorbell.base = 0; (*adev).doorbell.size = 0;
        (*adev).doorbell.num_kernel_doorbells = 0; return 0;
    }
    if pci_resource_flags((*adev).pdev, 2) & IORESOURCE_UNSET != 0 { return -EINVAL; }
    amdgpu_asic_init_doorbell_index(adev);
    (*adev).doorbell.base = pci_resource_start((*adev).pdev, 2);
    (*adev).doorbell.size = pci_resource_len((*adev).pdev, 2);
    (*adev).doorbell.num_kernel_doorbells = core::cmp::min(
        (*adev).doorbell.size / size_of::<u32>() as _, (*adev).doorbell_index.max_assignment + 1);
    if (*adev).doorbell.num_kernel_doorbells == 0 { return -EINVAL; }
    if (*adev).asic_type >= CHIP_VEGA10 { (*adev).doorbell.num_kernel_doorbells += 0x400; }
    0
}

/// amdgpu_doorbell_fini - Tear down doorbell driver information.
pub unsafe fn amdgpu_doorbell_fini(adev: *mut amdgpu_device) {
    amdgpu_bo_free_kernel(&mut (*adev).doorbell.kernel_doorbells, core::ptr::null_mut(),
        &mut (*adev).doorbell.cpu_addr as *mut _ as *mut *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
