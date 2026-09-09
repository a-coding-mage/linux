/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/tlb.h
 *
 * Copyright (C) 2002 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency supplied by the Linux pagemap interfaces.
// Dependency supplied by asm-generic/tlb.h.

// #define tlb_flush tlb_flush

// The following types, constants, fields, and functions are supplied by
// dependent architecture and generic-MMU interfaces.

/*
 * get the tlbi levels in arm64.  Default value is TLBI_TTL_UNKNOWN if more than
 * one of cleared_* is set or neither is set - this elides the level hinting to
 * the hardware.
 */
#[inline]
pub unsafe fn tlb_get_level(tlb: *mut mmu_gather) -> ::core::ffi::c_int {
    /* The TTL field is only valid for the leaf entry. */
    if (*tlb).freed_tables {
        return TLBI_TTL_UNKNOWN;
    }

    if (*tlb).cleared_ptes && !((*tlb).cleared_pmds
        || (*tlb).cleared_puds
        || (*tlb).cleared_p4ds)
    {
        return 3;
    }

    if (*tlb).cleared_pmds && !((*tlb).cleared_ptes
        || (*tlb).cleared_puds
        || (*tlb).cleared_p4ds)
    {
        return 2;
    }

    if (*tlb).cleared_puds && !((*tlb).cleared_ptes
        || (*tlb).cleared_pmds
        || (*tlb).cleared_p4ds)
    {
        return 1;
    }

    if (*tlb).cleared_p4ds && !((*tlb).cleared_ptes
        || (*tlb).cleared_pmds
        || (*tlb).cleared_puds)
    {
        return 0;
    }

    TLBI_TTL_UNKNOWN
}

#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    let vma: vm_area_struct = TLB_FLUSH_VMA((*tlb).mm, 0);
    let flags: tlbf_t = if (*tlb).freed_tables || (*tlb).unshared_tables {
        TLBF_NONE
    } else {
        TLBF_NOWALKCACHE
    };
    let stride: ::core::ffi::c_ulong = tlb_get_unmap_size(tlb);
    let tlb_level: ::core::ffi::c_int = tlb_get_level(tlb);

    /*
     * If we're tearing down the address space then we only care about
     * invalidating the walk-cache, since the ASID allocator won't reallocate
     * our ASID without invalidating the entire TLB.
     */
    if (*tlb).fullmm {
        if (*tlb).freed_tables {
            flush_tlb_mm((*tlb).mm);
        }
        return;
    }

    __flush_tlb_range(&vma, (*tlb).start, (*tlb).end, stride, tlb_level, flags);
}

#[inline]
pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: pgtable_t,
    _addr: ::core::ffi::c_ulong,
) {
    let ptdesc: *mut ptdesc = page_ptdesc(pte);

    tlb_remove_ptdesc(tlb, ptdesc);
}

// #if CONFIG_PGTABLE_LEVELS > 2
#[inline]
pub unsafe fn __pmd_free_tlb(
    tlb: *mut mmu_gather,
    pmdp: *mut pmd_t,
    _addr: ::core::ffi::c_ulong,
) {
    let ptdesc: *mut ptdesc = virt_to_ptdesc(pmdp);

    tlb_remove_ptdesc(tlb, ptdesc);
}
// #endif

// #if CONFIG_PGTABLE_LEVELS > 3
#[inline]
pub unsafe fn __pud_free_tlb(
    tlb: *mut mmu_gather,
    pudp: *mut pud_t,
    _addr: ::core::ffi::c_ulong,
) {
    let ptdesc: *mut ptdesc = virt_to_ptdesc(pudp);

    if !pgtable_l4_enabled() {
        return;
    }

    tlb_remove_ptdesc(tlb, ptdesc);
}
// #endif

// #if CONFIG_PGTABLE_LEVELS > 4
#[inline]
pub unsafe fn __p4d_free_tlb(
    tlb: *mut mmu_gather,
    p4dp: *mut p4d_t,
    _addr: ::core::ffi::c_ulong,
) {
    let ptdesc: *mut ptdesc = virt_to_ptdesc(p4dp);

    if !pgtable_l5_enabled() {
        return;
    }

    tlb_remove_ptdesc(tlb, ptdesc);
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
