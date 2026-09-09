/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Page table support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the corresponding architecture and generic headers
// are intentionally left external to this translation.

extern "C" {
    static mut kmap_generation: u64;
}

/*
 * Page table creation interface
 */
#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let pgd: *mut pgd_t;

    pgd = __pgd_alloc(mm, 0);

    /*
     * There may be better ways to do this, but to ensure
     * that new address spaces always contain the kernel
     * base mapping, and to ensure that the user area is
     * initially marked invalid, initialize the new map
     * map with a copy of the kernel's persistent map.
     */

    memcpy(
        pgd as *mut core::ffi::c_void,
        swapper_pg_dir as *const core::ffi::c_void,
        PTRS_PER_PGD * core::mem::size_of::<pgd_t>(),
    );
    (*mm).context.generation = kmap_generation;

    /* Physical version is what is passed to virtual machine on switch */
    (*mm).context.ptbase = __pa(pgd);

    pgd
}

#[inline]
pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    /*
     * Conveniently, zero in 3 LSB means indirect 4K page table.
     * Not so convenient when you're trying to vary the page size.
     */
    set_pmd(
        pmd,
        __pmd(((page_to_pfn(pte) as c_ulong) << PAGE_SHIFT) | HEXAGON_L1_PTE_SIZE),
    );
}

/*
 * Other architectures seem to have ways of making all processes
 * share the same pmd's for their kernel mappings, but the v0.3
 * Hexagon VM spec has a "monolithic" L1 table for user and kernel
 * segments.  We track "generations" of the kernel map to minimize
 * overhead, and update the "slave" copies of the kernel mappings
 * as part of switch_mm.  However, we still need to update the
 * kernel map of the active thread who's calling pmd_populate_kernel...
 */
#[inline]
pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    extern "C" {
        static mut kmap_gen_lock: spinlock_t;
    }
    let ppmd: *mut pmd_t;
    let pmdindex: isize;

    spin_lock(&mut kmap_gen_lock);
    kmap_generation = kmap_generation.wrapping_add(1);
    (*mm).context.generation = kmap_generation;
    (*(*current).active_mm).context.generation = kmap_generation;
    spin_unlock(&mut kmap_gen_lock);

    set_pmd(pmd, __pmd((__pa(pte) as c_ulong) | HEXAGON_L1_PTE_SIZE));

    /*
     * Now the "slave" copy of the current thread.
     * This is pointer arithmetic, not byte addresses!
     */
    pmdindex = (pmd as *mut pgd_t).offset_from((*mm).pgd);
    ppmd = ((*(*current).active_mm).pgd as *mut pmd_t).offset(pmdindex);
    set_pmd(ppmd, __pmd((__pa(pte) as c_ulong) | HEXAGON_L1_PTE_SIZE));
    if pmdindex > max_kernel_seg {
        max_kernel_seg = pmdindex;
    }
}

#[inline]
pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, pte: pgtable_t, addr: c_ulong) {
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
