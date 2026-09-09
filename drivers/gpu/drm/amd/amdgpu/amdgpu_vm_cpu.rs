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

/// amdgpu_vm_cpu_map_table - make sure new PDs/PTs are kmapped
///
/// `table`: newly allocated or validated PD/PT
unsafe fn amdgpu_vm_cpu_map_table(table: *mut amdgpu_bo_vm) -> i32 {
    (*table).bo.flags |= AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED;
    amdgpu_bo_kmap(&mut (*table).bo, core::ptr::null_mut())
}

/// amdgpu_vm_cpu_prepare - prepare page table update with the CPU
///
/// `p`: see amdgpu_vm_update_params definition
/// `sync`: sync obj with fences to wait on
/// `k_job_id`: the id for tracing/debug purposes
///
/// Returns:
/// Negative errno, 0 for success.
unsafe fn amdgpu_vm_cpu_prepare(
    _p: *mut amdgpu_vm_update_params,
    sync: *mut amdgpu_sync,
    _k_job_id: u64,
) -> i32 {
    if sync.is_null() {
        return 0;
    }

    amdgpu_sync_wait(sync, true)
}

/// amdgpu_vm_cpu_update - helper to update page tables via CPU
///
/// `p`: see amdgpu_vm_update_params definition
/// `vmbo`: PD/PT to update
/// `pe`: byte offset of the PDE/PTE, relative to start of PDB/PTB
/// `addr`: dst addr to write into pe
/// `count`: number of page entries to update
/// `incr`: increase next addr by incr bytes
/// `flags`: hw access flags
///
/// Write count number of PT/PD entries directly.
unsafe fn amdgpu_vm_cpu_update(
    p: *mut amdgpu_vm_update_params,
    vmbo: *mut amdgpu_bo_vm,
    mut pe: u64,
    mut addr: u64,
    count: u32,
    incr: u32,
    mut flags: u64,
) -> i32 {
    let r = dma_resv_wait_timeout(
        (*(*vmbo).bo.tbo.base.resv),
        DMA_RESV_USAGE_KERNEL,
        true,
        MAX_SCHEDULE_TIMEOUT,
    );
    if r < 0 {
        return r as i32;
    }

    pe = pe.wrapping_add(amdgpu_bo_kptr(&mut (*vmbo).bo) as usize as u64);

    trace_amdgpu_vm_set_ptes(pe, addr, count, incr, flags, (*p).immediate);

    if (*p).pages_addr.is_null() && (*p).override_pte {
        amdgpu_gmc_override_vm_pte_flags((*p).adev, (*p).vm, addr, &mut flags);
    }

    for i in 0..count {
        let mut oflags = flags;
        let value = if !(*p).pages_addr.is_null() {
            amdgpu_vm_map_gart((*p).pages_addr, addr)
        } else {
            addr
        };

        if !(*p).pages_addr.is_null() && (*p).override_pte {
            amdgpu_gmc_override_vm_pte_flags((*p).adev, (*p).vm, value, &mut oflags);
        }

        amdgpu_gmc_set_pte_pde(
            (*p).adev,
            pe as usize as *mut core::ffi::c_void,
            i,
            value,
            oflags,
        );
        addr = addr.wrapping_add(incr as u64);
    }
    0
}

/// amdgpu_vm_cpu_commit - commit page table update to the HW
///
/// `p`: see amdgpu_vm_update_params definition
/// `fence`: unused
///
/// Make sure that the hardware sees the page table updates.
unsafe fn amdgpu_vm_cpu_commit(
    p: *mut amdgpu_vm_update_params,
    _fence: *mut *mut dma_fence,
) -> i32 {
    let adev = (*p).adev;

    if (*p).needs_flush {
        atomic64_inc(&mut (*(*p).vm).tlb_seq);
    }

    mb();
    /* A reset flushed the HDP anyway, so that here can be skipped when a reset is ongoing */
    if !down_read_trylock(&mut (*(*adev).reset_domain).sem) {
        return 0;
    }

    amdgpu_device_flush_hdp(adev, core::ptr::null_mut());
    up_read(&mut (*(*adev).reset_domain).sem);

    0
}

pub static amdgpu_vm_cpu_funcs: amdgpu_vm_update_funcs = amdgpu_vm_update_funcs {
    map_table: Some(amdgpu_vm_cpu_map_table),
    prepare: Some(amdgpu_vm_cpu_prepare),
    update: Some(amdgpu_vm_cpu_update),
    commit: Some(amdgpu_vm_cpu_commit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
