// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding Linux kernel headers:
// linux/mm.h, linux/hugetlb.h, linux/security.h, asm/cacheflush.h,
// asm/machdep.h, asm/mman.h, and asm/tlb.h.

pub unsafe fn radix__flush_hugetlb_page(
    vma: *mut vm_area_struct,
    vmaddr: ::core::ffi::c_ulong,
) {
    let psize: ::core::ffi::c_int;
    let hstate: *mut hstate = hstate_file((*vma).vm_file);

    psize = hstate_get_psize(hstate);
    radix__flush_tlb_page_psize((*vma).vm_mm, vmaddr, psize);
}

pub unsafe fn radix__local_flush_hugetlb_page(
    vma: *mut vm_area_struct,
    vmaddr: ::core::ffi::c_ulong,
) {
    let psize: ::core::ffi::c_int;
    let hstate: *mut hstate = hstate_file((*vma).vm_file);

    psize = hstate_get_psize(hstate);
    radix__local_flush_tlb_page_psize((*vma).vm_mm, vmaddr, psize);
}

pub unsafe fn radix__flush_hugetlb_tlb_range(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    let psize: ::core::ffi::c_int;
    let hstate: *mut hstate = hstate_file((*vma).vm_file);

    psize = hstate_get_psize(hstate);
    /*
     * Flush PWC even if we get PUD_SIZE hugetlb invalidate to keep this simpler.
     */
    if end.wrapping_sub(start) >= PUD_SIZE {
        radix__flush_tlb_pwc_range_psize((*vma).vm_mm, start, end, psize);
    } else {
        radix__flush_tlb_range_psize((*vma).vm_mm, start, end, psize);
    }
    mmu_notifier_arch_invalidate_secondary_tlbs((*vma).vm_mm, start, end);
}

pub unsafe fn radix__huge_ptep_modify_prot_commit(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    old_pte: pte_t,
    pte: pte_t,
) {
    let mm: *mut mm_struct = (*vma).vm_mm;
    let psize: ::core::ffi::c_ulong = huge_page_size(hstate_vma(vma));

    /*
     * POWER9 NMMU must flush the TLB after clearing the PTE before
     * installing a PTE with more relaxed access permissions, see
     * radix__ptep_set_access_flags.
     */
    if !cpu_has_feature(CPU_FTR_ARCH_31)
        && is_pte_rw_upgrade(pte_val(old_pte), pte_val(pte))
        && atomic_read(&(*mm).context.copros) > 0
    {
        radix__flush_hugetlb_page(vma, addr);
    }

    set_huge_pte_at((*vma).vm_mm, addr, ptep, pte, psize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
