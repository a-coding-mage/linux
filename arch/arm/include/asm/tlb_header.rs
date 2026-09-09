/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/tlb.h
 *
 *  Copyright (C) 2002 Russell King
 *
 *  Experimentation shows that on a StrongARM, it appears to be faster
 *  to use the "invalidate whole tlb" rather than "invalidate single
 *  tlb" for this.
 *
 *  This appears true for both the process fork+exit case, as well as
 *  the munmap-large-area case.
 */

// Dependency intent: asm/cacheflush.h, linux/pagemap.h, asm/tlbflush.h,
// and asm-generic/tlb.h supply the referenced types and functions.

// !CONFIG_MMU
#[cfg(not(feature = "mmu"))]
#[inline]
pub unsafe fn tlb_flush<T>(tlb: T) {
    let _ = tlb;
}

// CONFIG_MMU
#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: pgtable_t,
    mut addr: libc::c_ulong,
) {
    let ptdesc: *mut ptdesc = page_ptdesc(pte);

    // With the classic ARM MMU, a pte page has two corresponding pmd
    // entries, each covering 1MB.
    #[cfg(not(feature = "arm_lpae"))]
    {
        addr = (addr & PMD_MASK) + SZ_1M;
        __tlb_adjust_range(tlb, addr.wrapping_sub(PAGE_SIZE), 2 * PAGE_SIZE);
    }

    tlb_remove_ptdesc(tlb, ptdesc);
}

#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn __pmd_free_tlb(
    tlb: *mut mmu_gather,
    pmdp: *mut pmd_t,
    _addr: libc::c_ulong,
) {
    #[cfg(feature = "arm_lpae")]
    {
        let ptdesc: *mut ptdesc = virt_to_ptdesc(pmdp);

        tlb_remove_ptdesc(tlb, ptdesc);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
