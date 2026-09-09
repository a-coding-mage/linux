// SPDX-License-Identifier: GPL-2.0

// Linux and RISC-V dependencies supplied by other translation units.

// #define has_svinval() riscv_has_extension_unlikely(RISCV_ISA_EXT_SVINVAL)
#[inline]
unsafe fn has_svinval() -> bool {
    riscv_has_extension_unlikely(RISCV_ISA_EXT_SVINVAL)
}

/*
 * Flush entire TLB if number of entries to be flushed is greater
 * than the threshold below.
 */
#[no_mangle]
pub static mut tlb_flush_all_threshold: c_ulong = 64;

unsafe fn local_flush_tlb_range_threshold_asid(
    mut start: c_ulong,
    size: c_ulong,
    stride: c_ulong,
    asid: c_ulong,
) {
    let nr_ptes_in_range = size.div_ceil(stride);
    let mut i: c_int = 0;

    if nr_ptes_in_range > tlb_flush_all_threshold {
        local_flush_tlb_all_asid(asid);
        return;
    }

    if has_svinval() {
        local_sfence_w_inval();
        while i < nr_ptes_in_range as c_int {
            local_sinval_vma(start, asid);
            start += stride;
            i += 1;
        }
        local_sfence_inval_ir();
        return;
    }

    i = 0;
    while i < nr_ptes_in_range as c_int {
        local_flush_tlb_page_asid(start, asid);
        start += stride;
        i += 1;
    }
}

#[inline]
unsafe fn local_flush_tlb_range_asid(
    start: c_ulong,
    size: c_ulong,
    stride: c_ulong,
    asid: c_ulong,
) {
    if size <= stride {
        local_flush_tlb_page_asid(start, asid);
    } else if size == FLUSH_TLB_MAX_SIZE {
        local_flush_tlb_all_asid(asid);
    } else {
        local_flush_tlb_range_threshold_asid(start, size, stride, asid);
    }
}

/* Flush a range of kernel pages without broadcasting */
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    local_flush_tlb_range_asid(start, end - start, PAGE_SIZE, FLUSH_TLB_NO_ASID);
}

unsafe extern "C" fn __ipi_flush_tlb_all(_info: *mut c_void) {
    local_flush_tlb_all();
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_all() {
    if num_online_cpus() < 2 {
        local_flush_tlb_all();
    } else if riscv_use_sbi_for_rfence() {
        sbi_remote_sfence_vma_asid(ptr::null(), 0, FLUSH_TLB_MAX_SIZE, FLUSH_TLB_NO_ASID);
    } else {
        on_each_cpu(__ipi_flush_tlb_all, ptr::null_mut(), 1);
    }
}

#[repr(C)]
pub struct flush_tlb_range_data {
    pub asid: c_ulong,
    pub start: c_ulong,
    pub size: c_ulong,
    pub stride: c_ulong,
}

unsafe extern "C" fn __ipi_flush_tlb_range_asid(info: *mut c_void) {
    let d = &*(info as *const flush_tlb_range_data);
    local_flush_tlb_range_asid(d.start, d.size, d.stride, d.asid);
}

unsafe fn __flush_tlb_range(
    mm: *mut mm_struct,
    cmask: *const cpumask,
    start: c_ulong,
    size: c_ulong,
    stride: c_ulong,
) {
    let asid = get_mm_asid(mm);
    let cpu: c_uint;

    if cpumask_empty(cmask) {
        return;
    }

    cpu = get_cpu();

    /* Check if the TLB flush needs to be sent to other CPUs. */
    if cpumask_any_but(cmask, cpu) >= nr_cpu_ids {
        local_flush_tlb_range_asid(start, size, stride, asid);
    } else if riscv_use_sbi_for_rfence() {
        sbi_remote_sfence_vma_asid(cmask, start, size, asid);
    } else {
        let ftd = flush_tlb_range_data { asid, start, size, stride };
        on_each_cpu_mask(cmask, __ipi_flush_tlb_range_asid, &ftd as *const _ as *mut c_void, 1);
    }

    put_cpu();

    if !mm.is_null() {
        mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, start + size);
    }
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    __flush_tlb_range(mm, mm_cpumask(mm), 0, FLUSH_TLB_MAX_SIZE, PAGE_SIZE);
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_mm_range(
    mm: *mut mm_struct,
    start: c_ulong,
    end: c_ulong,
    page_size: c_uint,
) {
    __flush_tlb_range(mm, mm_cpumask(mm), start, end - start, page_size as c_ulong);
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong) {
    __flush_tlb_range((*vma).vm_mm, mm_cpumask((*vma).vm_mm), addr, PAGE_SIZE, PAGE_SIZE);
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_range(
    vma: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
) {
    let mut stride_size: c_ulong;

    if !is_vm_hugetlb_page(vma) {
        stride_size = PAGE_SIZE;
    } else {
        stride_size = huge_page_size(hstate_vma(vma));

        /*
         * As stated in the privileged specification, every PTE in a
         * NAPOT region must be invalidated, so reset the stride in that
         * case.
         */
        if has_svnapot() {
            if stride_size >= PGDIR_SIZE {
                stride_size = PGDIR_SIZE;
            } else if stride_size >= P4D_SIZE {
                stride_size = P4D_SIZE;
            } else if stride_size >= PUD_SIZE {
                stride_size = PUD_SIZE;
            } else if stride_size >= PMD_SIZE {
                stride_size = PMD_SIZE;
            } else {
                stride_size = PAGE_SIZE;
            }
        }
    }

    __flush_tlb_range((*vma).vm_mm, mm_cpumask((*vma).vm_mm), start, end - start, stride_size);
}

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    __flush_tlb_range(ptr::null_mut(), cpu_online_mask, start, end - start, PAGE_SIZE);
}

// CONFIG_TRANSPARENT_HUGEPAGE
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[no_mangle]
pub unsafe extern "C" fn flush_pmd_tlb_range(
    vma: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
) {
    __flush_tlb_range((*vma).vm_mm, mm_cpumask((*vma).vm_mm), start, end - start, PMD_SIZE);
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[no_mangle]
pub unsafe extern "C" fn flush_pud_tlb_range(
    vma: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
) {
    __flush_tlb_range((*vma).vm_mm, mm_cpumask((*vma).vm_mm), start, end - start, PUD_SIZE);
}

#[no_mangle]
pub unsafe extern "C" fn arch_tlbbatch_should_defer(_mm: *mut mm_struct) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn arch_tlbbatch_add_pending(
    batch: *mut arch_tlbflush_unmap_batch,
    mm: *mut mm_struct,
    start: c_ulong,
    end: c_ulong,
) {
    cpumask_or(&mut (*batch).cpumask, &(*batch).cpumask, mm_cpumask(mm));
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, end);
}

#[no_mangle]
pub unsafe extern "C" fn arch_tlbbatch_flush(batch: *mut arch_tlbflush_unmap_batch) {
    __flush_tlb_range(ptr::null_mut(), &(*batch).cpumask, 0, FLUSH_TLB_MAX_SIZE, PAGE_SIZE);
    cpumask_clear(&mut (*batch).cpumask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
