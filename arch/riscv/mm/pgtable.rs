// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this file.

pub unsafe fn ptep_set_access_flags(
    vma: *mut vm_area_struct,
    address: c_ulong,
    ptep: *mut pte_t,
    entry: pte_t,
    _dirty: c_int,
) -> c_int {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SVVPTC) {
        if !pte_same(ptep_get(ptep), entry) {
            __set_pte_at((*vma).vm_mm, ptep, entry);
            /* Here only not svadu is impacted */
            flush_tlb_page(vma, address);
            return 1;
        }

        return 0;
    }

    if !pte_same(ptep_get(ptep), entry) {
        __set_pte_at((*vma).vm_mm, ptep, entry);
    }
    /*
     * update_mmu_cache will unconditionally execute, handling both
     * the case that the PTE changed and the spurious fault case.
     */
    1
}

pub unsafe fn ptep_test_and_clear_young(
    _vma: *mut vm_area_struct,
    _address: c_ulong,
    ptep: *mut pte_t,
) -> bool {
    if !pte_young(ptep_get(ptep)) {
        return false;
    }
    test_and_clear_bit(_PAGE_ACCESSED_OFFSET, &mut (*ptep).val) != 0
}

// EXPORT_SYMBOL_GPL(ptep_test_and_clear_young);

#[cfg(target_pointer_width = "64")]
pub unsafe fn pud_offset(p4d: *mut p4d_t, address: c_ulong) -> *mut pud_t {
    if pgtable_l4_enabled {
        return p4d_pgtable(p4dp_get(p4d)).add(pud_index(address));
    }

    p4d as *mut pud_t
}

// EXPORT_SYMBOL_GPL(pud_offset);

#[cfg(target_pointer_width = "64")]
pub unsafe fn p4d_offset(pgd: *mut pgd_t, address: c_ulong) -> *mut p4d_t {
    if pgtable_l5_enabled {
        return pgd_pgtable(pgdp_get(pgd)).add(p4d_index(address));
    }

    pgd as *mut p4d_t
}

// EXPORT_SYMBOL_GPL(p4d_offset);

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn p4d_set_huge(_p4d: *mut p4d_t, _addr: phys_addr_t, _prot: pgprot_t) -> c_int {
    0
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn p4d_clear_huge(_p4d: *mut p4d_t) {}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pud_set_huge(pud: *mut pud_t, phys: phys_addr_t, prot: pgprot_t) -> c_int {
    let new_pud = pfn_pud(__phys_to_pfn(phys), prot);

    set_pud(pud, new_pud);
    1
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pud_clear_huge(pud: *mut pud_t) -> c_int {
    if !pud_leaf(pudp_get(pud)) {
        return 0;
    }
    pud_clear(pud);
    1
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pud_free_pmd_page(pud: *mut pud_t, addr: c_ulong) -> c_int {
    let pmd = pud_pgtable(pudp_get(pud));

    pud_clear(pud);

    flush_tlb_kernel_range(addr, addr.wrapping_add(PUD_SIZE));

    for i in 0..PTRS_PER_PMD {
        if !pmd_none(*pmd.add(i)) {
            let pte = pmd_page_vaddr(*pmd.add(i)) as *mut pte_t;

            pte_free_kernel(core::ptr::null_mut(), pte);
        }
    }

    pmd_free(core::ptr::null_mut(), pmd);

    1
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pmd_set_huge(pmd: *mut pmd_t, phys: phys_addr_t, prot: pgprot_t) -> c_int {
    let new_pmd = pfn_pmd(__phys_to_pfn(phys), prot);

    set_pmd(pmd, new_pmd);
    1
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pmd_clear_huge(pmd: *mut pmd_t) -> c_int {
    if !pmd_leaf(pmdp_get(pmd)) {
        return 0;
    }
    pmd_clear(pmd);
    1
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
pub unsafe fn pmd_free_pte_page(pmd: *mut pmd_t, addr: c_ulong) -> c_int {
    let pte = pmd_page_vaddr(pmdp_get(pmd)) as *mut pte_t;

    pmd_clear(pmd);

    flush_tlb_kernel_range(addr, addr.wrapping_add(PMD_SIZE));
    pte_free_kernel(core::ptr::null_mut(), pte);
    1
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn pmdp_collapse_flush(
    vma: *mut vm_area_struct,
    address: c_ulong,
    pmdp: *mut pmd_t,
) -> pmd_t {
    let pmd = pmdp_huge_get_and_clear((*vma).vm_mm, address, pmdp);

    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_trans_huge(pmdp_get(pmdp)));
    /*
     * When leaf PTE entries (regular pages) are collapsed into a leaf
     * PMD entry (huge page), a valid non-leaf PTE is converted into a
     * valid leaf PTE at the level 1 page table.  Since the sfence.vma
     * forms that specify an address only apply to leaf PTEs, we need a
     * global flush here.  collapse_huge_page() assumes these flushes are
     * eager, so just do the fence here.
     */
    flush_tlb_mm((*vma).vm_mm);
    pmd
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn pudp_invalidate(
    vma: *mut vm_area_struct,
    address: c_ulong,
    pudp: *mut pud_t,
) -> pud_t {
    VM_WARN_ON_ONCE(!pud_present(*pudp));
    let old = pudp_establish(vma, address, pudp, pud_mkinvalid(*pudp));

    flush_pud_tlb_range(vma, address, address.wrapping_add(HPAGE_PUD_SIZE));
    old
}

pub unsafe fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t {
    if (*vma).vm_flags & VM_SHADOW_STACK != 0 {
        return pte_mkwrite_shstk(pte);
    }

    pte_mkwrite_novma(pte)
}

pub unsafe fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t {
    if (*vma).vm_flags & VM_SHADOW_STACK != 0 {
        return pmd_mkwrite_shstk(pmd);
    }

    pmd_mkwrite_novma(pmd)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
