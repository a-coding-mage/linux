// SPDX-License-Identifier: GPL-2.0
/*
 * PPC64 Huge TLB Page Support for hash based MMUs (POWER4 and later)
 *
 * Copyright (C) 2003 David Gibson, IBM Corporation.
 *
 * Based on the IA-32 version:
 * Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut hpage_shift: ::core::ffi::c_uint = 0;

// CONFIG_PPC_64S_HASH_MMU
pub unsafe fn __hash_page_huge(
    ea: ::core::ffi::c_ulong,
    access: ::core::ffi::c_ulong,
    vsid: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    trap: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    ssize: ::core::ffi::c_int,
    shift: ::core::ffi::c_uint,
    mmu_psize: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut rpte: real_pte_t;
    let vpn: ::core::ffi::c_ulong;
    let mut old_pte: ::core::ffi::c_ulong;
    let mut new_pte: ::core::ffi::c_ulong;
    let mut rflags: ::core::ffi::c_ulong;
    let mut pa: ::core::ffi::c_ulong;
    let mut slot: ::core::ffi::c_long;
    let offset: ::core::ffi::c_long;

    BUG_ON(shift != mmu_psize_defs[mmu_psize as usize].shift);

    /* Search the Linux page table for a match with va */
    vpn = hpt_vpn(ea, vsid, ssize);

    /*
     * At this point, we have a pte (old_pte) which can be used to build
     * or update an HPTE. There are 2 cases:
     *
     * 1. There is a valid (present) pte with no associated HPTE (this is
     *	the most common case)
     * 2. There is a valid (present) pte with an associated HPTE. The
     * 	current values of the pp bits in the HPTE prevent access
     * 	because we are doing software DIRTY bit management and the
     * 	page is currently not DIRTY.
     */
    loop {
        old_pte = pte_val(*ptep);
        /* If PTE busy, retry the access */
        if unlikely(old_pte & H_PAGE_BUSY) { return 0; }
        /* If PTE permissions don't match, take page fault */
        if unlikely(!check_pte_access(access, old_pte)) { return 1; }
        /* If hash-4k, hugepages use seeral contiguous PxD entries */
        if IS_ENABLED(CONFIG_PPC_4K_PAGES) {
            if (old_pte & _PAGE_ACCESSED) == 0 { return 1; }
            if (access & _PAGE_WRITE) != 0 && (old_pte & _PAGE_DIRTY) == 0 { return 1; }
        }
        new_pte = old_pte | H_PAGE_BUSY | _PAGE_ACCESSED;
        if (access & _PAGE_WRITE) != 0 { new_pte |= _PAGE_DIRTY; }
        if pte_xchg(ptep, __pte(old_pte), __pte(new_pte)) != 0 { break; }
    }

    /* Make sure this is a hugetlb entry */
    if (old_pte & H_PAGE_THP_HUGE) != 0 { return 0; }

    rflags = htab_convert_pte_flags(new_pte, flags);
    if mmu_psize == MMU_PAGE_16G { offset = PTRS_PER_PUD; } else { offset = PTRS_PER_PMD; }
    rpte = __real_pte(__pte(old_pte), ptep, offset);

    if !cpu_has_feature(CPU_FTR_COHERENT_ICACHE) {
        /* No CPU has hugepages but lacks no execute, so we don't need to worry about that case */
        rflags = hash_page_do_lazy_icache(rflags, __pte(old_pte), trap);
    }

    if unlikely((old_pte & H_PAGE_HASHPTE) != 0) {
        let gslot = pte_get_hash_gslot(vpn, shift, ssize, rpte, 0);
        if mmu_hash_ops.hpte_updatepp(gslot, rflags, vpn, mmu_psize, mmu_psize, ssize, flags) == -1 {
            old_pte &= !_PAGE_HPTEFLAGS;
        }
    }

    if likely((old_pte & H_PAGE_HASHPTE) == 0) {
        let hash = hpt_hash(vpn, shift, ssize);
        pa = pte_pfn(__pte(old_pte)) << PAGE_SHIFT;
        new_pte = (new_pte & !_PAGE_HPTEFLAGS) | H_PAGE_HASHPTE;
        slot = hpte_insert_repeating(hash, vpn, pa, rflags, 0, mmu_psize, ssize);
        if unlikely(slot == -2) {
            *ptep = __pte(old_pte);
            hash_failure_debug(ea, access, vsid, trap, ssize, mmu_psize, mmu_psize, old_pte);
            return -1;
        }
        new_pte |= pte_set_hidx(ptep, rpte, 0, slot, offset);
    }

    /* No need to use ldarx/stdcx here */
    *ptep = __pte(new_pte & !H_PAGE_BUSY);
    0
}

pub unsafe fn huge_ptep_modify_prot_start(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t) -> pte_t {
    /* Clear _PAGE_PRESENT so no hardware parallel update is possible. */
    let pte_val = pte_update((*vma).vm_mm, addr, ptep, _PAGE_PRESENT, _PAGE_INVALID, 1);
    __pte(pte_val)
}

pub unsafe fn huge_ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t, old_pte: pte_t, pte: pte_t) {
    if radix_enabled() { return; }
    let psize = huge_page_size(hstate_vma(vma));
    set_huge_pte_at((*vma).vm_mm, addr, ptep, pte, psize);
}

pub unsafe fn hugetlbpage_init_defaultsize() {
    /* Set default large page size. Currently, we pick 16M or 1M depending on what is available */
    if mmu_psize_defs[MMU_PAGE_16M as usize].shift != 0 {
        hpage_shift = mmu_psize_defs[MMU_PAGE_16M as usize].shift;
    } else if mmu_psize_defs[MMU_PAGE_1M as usize].shift != 0 {
        hpage_shift = mmu_psize_defs[MMU_PAGE_1M as usize].shift;
    } else if mmu_psize_defs[MMU_PAGE_2M as usize].shift != 0 {
        hpage_shift = mmu_psize_defs[MMU_PAGE_2M as usize].shift;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
