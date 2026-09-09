/*
 * arch/sh/mm/tlb-pteaex.c
 *
 * TLB operations for SH-X3 CPUs featuring PTE ASID Extensions.
 *
 * Copyright (C) 2009 Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn __update_tlb(
    vma: *mut vm_area_struct,
    address: ::core::ffi::c_ulong,
    pte: pte_t,
) {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut pteval: ::core::ffi::c_ulong;
    let mut vpn: ::core::ffi::c_ulong;

    /*
     * Handle debugger faulting in for debugee.
     */
    if !vma.is_null() && (*current).active_mm != (*vma).vm_mm {
        return;
    }

    local_irq_save(&mut flags);

    /* Set PTEH register */
    vpn = address & MMU_VPN_MASK;
    __raw_writel(vpn, MMU_PTEH);

    /* Set PTEAEX */
    __raw_writel(get_asid(), MMU_PTEAEX);

    pteval = pte.pte_low;

    /* Set PTEA register */
    #[cfg(feature = "CONFIG_X2TLB")]
    /*
     * For the extended mode TLB this is trivial, only the ESZ and EPR bits
     * need to be written out to PTEA, with the remainder of the protection
     * bits (with the exception of the compat-mode SZ and PR bits, which are
     * cleared) being written out in PTEL.
     */
    {
        __raw_writel(pte.pte_high, MMU_PTEA);
    }

    /* Set PTEL register */
    pteval &= _PAGE_FLAGS_HARDWARE_MASK; /* drop software flags */
    #[cfg(feature = "CONFIG_CACHE_WRITETHROUGH")]
    {
        pteval |= _PAGE_WT;
    }
    /* conveniently, we want all the software flags to be 0 anyway */
    __raw_writel(pteval, MMU_PTEL);

    /* Load the TLB */
    ::core::arch::asm!("ldtlb", options(nostack, preserves_flags));
    local_irq_restore(flags);
}

/*
 * While SH-X2 extended TLB mode splits out the memory-mapped I/UTLB
 * data arrays, SH-X3 cores with PTEAEX split out the memory-mapped
 * address arrays. In compat mode the second array is inaccessible, while
 * in extended mode, the legacy 8-bit ASID field in address array 1 has
 * undefined behaviour.
 */
pub unsafe fn local_flush_tlb_one(
    asid: ::core::ffi::c_ulong,
    page: ::core::ffi::c_ulong,
) {
    jump_to_uncached();
    __raw_writel(page, MMU_UTLB_ADDRESS_ARRAY | MMU_PAGE_ASSOC_BIT);
    __raw_writel(asid, MMU_UTLB_ADDRESS_ARRAY2 | MMU_PAGE_ASSOC_BIT);
    __raw_writel(page, MMU_ITLB_ADDRESS_ARRAY | MMU_PAGE_ASSOC_BIT);
    __raw_writel(asid, MMU_ITLB_ADDRESS_ARRAY2 | MMU_PAGE_ASSOC_BIT);
    back_to_cached();
}

pub unsafe fn local_flush_tlb_all() {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut status: ::core::ffi::c_ulong;
    let mut i: i32;

    /*
     * Flush all the TLB.
     */
    local_irq_save(&mut flags);
    jump_to_uncached();

    status = __raw_readl(MMUCR);
    status = (status & MMUCR_URB) >> MMUCR_URB_SHIFT;

    if status == 0 {
        status = MMUCR_URB_NENTRIES;
    }

    i = 0;
    while (i < status as i32) {
        __raw_writel(0x0, MMU_UTLB_ADDRESS_ARRAY | ((i as ::core::ffi::c_ulong) << 8));
        i += 1;
    }

    i = 0;
    while i < 4 {
        __raw_writel(0x0, MMU_ITLB_ADDRESS_ARRAY | ((i as ::core::ffi::c_ulong) << 8));
        i += 1;
    }

    back_to_cached();
    ctrl_barrier();
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
