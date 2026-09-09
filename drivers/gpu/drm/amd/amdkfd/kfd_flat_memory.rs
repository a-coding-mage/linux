// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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

// Linux kernel dependencies and local headers are supplied by the surrounding translation unit.

#[inline]
fn make_gpuvm_app_base_vi(gpu_num: u64) -> u64 {
    (gpu_num << 61).wrapping_add(0x1000000000000)
}

#[inline]
fn make_gpuvm_app_limit(base: u64, size: u64) -> u64 {
    (base & 0xFFFFFF0000000000).wrapping_add(size).wrapping_sub(1)
}

#[inline]
fn make_scratch_app_base_vi() -> u64 {
    (1u64 << 61).wrapping_add(0x100000000)
}

#[inline]
fn make_scratch_app_limit(base: u64) -> u64 {
    (base & 0xFFFFFFFF00000000) | 0xFFFFFFFF
}

#[inline]
fn make_lds_app_base_vi() -> u64 {
    (1u64 << 61)
}

#[inline]
fn make_lds_app_limit(base: u64) -> u64 {
    (base & 0xFFFFFFFF00000000) | 0xFFFFFFFF
}

/* On GFXv9 the LDS and scratch apertures are programmed independently
 * using the high 16 bits of the 64-bit virtual address. They must be
 * in the hole, which will be the case as long as the high 16 bits are
 * not 0.
 *
 * The aperture sizes are still 4GB implicitly.
 *
 * A GPUVM aperture is not applicable on GFXv9.
 */
#[inline]
fn make_lds_app_base_v9() -> u64 { 1u64 << 48 }

#[inline]
fn make_scratch_app_base_v9() -> u64 { 2u64 << 48 }

// User mode manages most of the SVM aperture address space. The low
// 16MB are reserved for kernel use (CWSR trap handler and kernel IB for now).
// SVM_USER_BASE = (KFD_CWSR_TBA_TMA_SIZE + 2 * PAGE_SIZE) as u64
// SVM_CWSR_BASE = SVM_USER_BASE - KFD_CWSR_TBA_TMA_SIZE
// SVM_IB_BASE = SVM_CWSR_BASE - PAGE_SIZE

unsafe fn kfd_init_apertures_vi(pdd: *mut kfd_process_device, _id: u8) {
    (*pdd).lds_base = make_lds_app_base_vi();
    (*pdd).lds_limit = make_lds_app_limit((*pdd).lds_base);
    (*pdd).gpuvm_base = max(SVM_USER_BASE, AMDGPU_VA_RESERVED_BOTTOM);
    (*pdd).gpuvm_limit = (*(*pdd).dev).kfd.shared_resources.gpuvm_size - 1;
    (*pdd).qpd.cwsr_base = SVM_CWSR_BASE;
    (*pdd).qpd.ib_base = SVM_IB_BASE;
    (*pdd).scratch_base = make_scratch_app_base_vi();
    (*pdd).scratch_limit = make_scratch_app_limit((*pdd).scratch_base);
}

unsafe fn kfd_init_apertures_v9(pdd: *mut kfd_process_device, _id: u8) {
    (*pdd).lds_base = make_lds_app_base_v9();
    (*pdd).lds_limit = make_lds_app_limit((*pdd).lds_base);
    (*pdd).gpuvm_base = AMDGPU_VA_RESERVED_BOTTOM;
    (*pdd).gpuvm_limit = (*(*pdd).dev).kfd.shared_resources.gpuvm_size - 1;
    (*pdd).scratch_base = make_scratch_app_base_v9();
    (*pdd).scratch_limit = make_scratch_app_limit((*pdd).scratch_base);
    (*pdd).qpd.cwsr_base = AMDGPU_VA_RESERVED_TRAP_START((*(*pdd).dev).adev);
}

unsafe fn kfd_init_apertures_v12(pdd: *mut kfd_process_device, _id: u8) {
    (*pdd).lds_base = (*(*pdd).dev).adev.gmc.shared_aperture_start;
    (*pdd).lds_limit = (*(*pdd).dev).adev.gmc.shared_aperture_end;
    (*pdd).gpuvm_base = AMDGPU_VA_RESERVED_BOTTOM;
    (*pdd).gpuvm_limit = (*(*pdd).dev).kfd.shared_resources.gpuvm_size - 1;
    (*pdd).scratch_base = (*(*pdd).dev).adev.gmc.private_aperture_start;
    (*pdd).scratch_limit = (*(*pdd).dev).adev.gmc.private_aperture_end;
    (*pdd).qpd.cwsr_base = AMDGPU_VA_RESERVED_TRAP_START((*(*pdd).dev).adev);
}

pub unsafe fn kfd_init_apertures(process: *mut kfd_process) -> i32 {
    let mut id: u8 = 0;
    let mut dev: *mut kfd_node = core::ptr::null_mut();
    while kfd_topology_enum_kfd_devices(id, &mut dev) == 0 {
        if dev.is_null() || kfd_devcgroup_check_permission(dev) {
            id = id.wrapping_add(1);
            continue;
        }
        let pdd = kfd_create_process_device_data(dev, process);
        if pdd.is_null() {
            dev_err((*dev).adev.dev, "Failed to create process device data\n");
            return -ENOMEM;
        }
        if (*process).is_32bit_user_mode {
            (*pdd).lds_base = 0; (*pdd).lds_limit = 0;
            (*pdd).gpuvm_base = 0; (*pdd).gpuvm_limit = 0;
            (*pdd).scratch_base = 0; (*pdd).scratch_limit = 0;
        } else {
            match (*dev).adev.asic_type {
                CHIP_KAVERI | CHIP_HAWAII | CHIP_CARRIZO | CHIP_TONGA |
                CHIP_FIJI | CHIP_POLARIS10 | CHIP_POLARIS11 | CHIP_POLARIS12 |
                CHIP_VEGAM => kfd_init_apertures_vi(pdd, id),
                _ => {
                    if KFD_GC_VERSION(dev) >= IP_VERSION(12, 1, 0) {
                        kfd_init_apertures_v12(pdd, id);
                    } else if KFD_GC_VERSION(dev) >= IP_VERSION(9, 0, 1) {
                        kfd_init_apertures_v9(pdd, id);
                    } else {
                        WARN(1, "Unexpected ASIC family %u", (*dev).adev.asic_type);
                        return -EINVAL;
                    }
                }
            }
        }
        dev_dbg(kfd_device, "node id %u\n", id);
        dev_dbg(kfd_device, "gpu id %u\n", (*pdd).dev.id);
        dev_dbg(kfd_device, "lds_base %llX\n", (*pdd).lds_base);
        dev_dbg(kfd_device, "lds_limit %llX\n", (*pdd).lds_limit);
        dev_dbg(kfd_device, "gpuvm_base %llX\n", (*pdd).gpuvm_base);
        dev_dbg(kfd_device, "gpuvm_limit %llX\n", (*pdd).gpuvm_limit);
        dev_dbg(kfd_device, "scratch_base %llX\n", (*pdd).scratch_base);
        dev_dbg(kfd_device, "scratch_limit %llX\n", (*pdd).scratch_limit);
        id = id.wrapping_add(1);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
