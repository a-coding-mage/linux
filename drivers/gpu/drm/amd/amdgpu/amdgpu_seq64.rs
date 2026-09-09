// SPDX-License-Identifier: MIT
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

/* Dependencies corresponding to amdgpu.h, amdgpu_seq64.h, and drm_exec.h. */

/// DOC: amdgpu_seq64
///
/// amdgpu_seq64 allocates a 64bit memory on each request in sequence order.
/// seq64 driver is required for user queue fence memory allocation, TLB
/// counters and VM updates. It has maximum count of 32768 64 bit slots.

#[inline]
unsafe fn amdgpu_seq64_get_va_base(adev: *mut amdgpu_device) -> u64 {
    let mut addr = AMDGPU_VA_RESERVED_SEQ64_START(adev);
    addr = amdgpu_gmc_sign_extend(addr);
    addr
}

pub unsafe fn amdgpu_seq64_map(
    adev: *mut amdgpu_device,
    vm: *mut amdgpu_vm,
    bo_va: *mut *mut amdgpu_bo_va,
) -> i32 {
    let bo: *mut amdgpu_bo;
    let mut exec: drm_exec = core::mem::zeroed();
    let mut seq64_addr: u64;
    let mut r: i32;

    bo = (*adev).seq64.sbo;
    if bo.is_null() {
        return -EINVAL;
    }

    drm_exec_init(&mut exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked!(&mut exec, {
        r = amdgpu_vm_lock_pd(vm, &mut exec, 0);
        if likely(r == 0) {
            r = drm_exec_lock_obj(&mut exec, &mut (*bo).tbo.base);
        }
        drm_exec_retry_on_contention!(&mut exec);
        if unlikely(r != 0) {
            break 'error;
        }
    });

    *bo_va = amdgpu_vm_bo_add(adev, vm, bo);
    if (*bo_va).is_null() {
        r = -ENOMEM;
        goto_error!();
    }

    seq64_addr = amdgpu_seq64_get_va_base(adev) & AMDGPU_GMC_HOLE_MASK;
    r = amdgpu_vm_bo_map(
        adev,
        *bo_va,
        seq64_addr,
        0,
        AMDGPU_VA_RESERVED_SEQ64_SIZE,
        AMDGPU_VM_PAGE_READABLE | AMDGPU_VM_MTYPE_UC,
    );
    if r != 0 {
        DRM_ERROR!("failed to do bo_map on userq sem, err=%d\n", r);
        amdgpu_vm_bo_del(adev, *bo_va);
        goto_error!();
    }

    r = amdgpu_vm_bo_update(adev, *bo_va, false);
    if r != 0 {
        DRM_ERROR!("failed to do vm_bo_update on userq sem\n");
        amdgpu_vm_bo_del(adev, *bo_va);
        goto_error!();
    }

    drm_exec_fini(&mut exec);
    return r;
}

pub unsafe fn amdgpu_seq64_unmap(adev: *mut amdgpu_device, fpriv: *mut amdgpu_fpriv) {
    let mut vm: *mut amdgpu_vm;
    let bo: *mut amdgpu_bo;
    let mut exec: drm_exec = core::mem::zeroed();
    let mut r: i32;

    if (*fpriv).seq64_va.is_null() { return; }
    bo = (*adev).seq64.sbo;
    if bo.is_null() { return; }
    vm = &mut (*fpriv).vm;

    drm_exec_init(&mut exec, 0, 0);
    drm_exec_until_all_locked!(&mut exec, {
        r = amdgpu_vm_lock_pd(vm, &mut exec, 0);
        if likely(r == 0) { r = drm_exec_lock_obj(&mut exec, &mut (*bo).tbo.base); }
        drm_exec_retry_on_contention!(&mut exec);
        if unlikely(r != 0) { break 'error; }
    });
    amdgpu_vm_bo_del(adev, (*fpriv).seq64_va);
    (*fpriv).seq64_va = core::ptr::null_mut();
    drm_exec_fini(&mut exec);
}

pub unsafe fn amdgpu_seq64_alloc(adev: *mut amdgpu_device, va: *mut u64, gpu_addr: *mut u64, cpu_addr: *mut *mut u64) -> i32 {
    let mut bit_pos: usize = 0;
    loop {
        bit_pos = find_next_zero_bit((*adev).seq64.used.as_ptr(), (*adev).seq64.num_sem, bit_pos);
        if bit_pos >= (*adev).seq64.num_sem { return -ENOSPC; }
        if !test_and_set_bit(bit_pos, (*adev).seq64.used.as_mut_ptr()) { break; }
        bit_pos += 1;
    }
    *va = (bit_pos * core::mem::size_of::<u64>()) as u64 + amdgpu_seq64_get_va_base(adev);
    if !gpu_addr.is_null() { *gpu_addr = (bit_pos * core::mem::size_of::<u64>()) as u64 + (*adev).seq64.gpu_addr; }
    *cpu_addr = (*adev).seq64.cpu_base_addr.add(bit_pos);
    0
}

pub unsafe fn amdgpu_seq64_free(adev: *mut amdgpu_device, va: u64) {
    let bit_pos = ((va - amdgpu_seq64_get_va_base(adev)) / core::mem::size_of::<u64>() as u64) as usize;
    if bit_pos < (*adev).seq64.num_sem { clear_bit(bit_pos, (*adev).seq64.used.as_mut_ptr()); }
}

pub unsafe fn amdgpu_seq64_fini(adev: *mut amdgpu_device) {
    amdgpu_bo_free_kernel(&mut (*adev).seq64.sbo, core::ptr::null_mut(), &mut (*adev).seq64.cpu_base_addr as *mut _ as *mut *mut core::ffi::c_void);
}

pub unsafe fn amdgpu_seq64_init(adev: *mut amdgpu_device) -> i32 {
    let mut r: i32;
    if !(*adev).seq64.sbo.is_null() { return 0; }
    r = amdgpu_bo_create_kernel(adev, AMDGPU_VA_RESERVED_SEQ64_SIZE, PAGE_SIZE, AMDGPU_GEM_DOMAIN_GTT, &mut (*adev).seq64.sbo, &mut (*adev).seq64.gpu_addr, &mut (*adev).seq64.cpu_base_addr as *mut _ as *mut *mut core::ffi::c_void);
    if r != 0 { dev_warn!((*adev).dev, "(%d) create seq64 failed\n", r); return r; }
    core::ptr::write_bytes((*adev).seq64.cpu_base_addr, 0, AMDGPU_VA_RESERVED_SEQ64_SIZE as usize);
    (*adev).seq64.num_sem = AMDGPU_MAX_SEQ64_SLOTS;
    core::ptr::write_bytes((*adev).seq64.used.as_mut_ptr(), 0, core::mem::size_of_val(&(*adev).seq64.used));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
