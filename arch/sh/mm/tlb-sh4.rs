// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/tlb-sh4.c
 *
 * SH-4 specific TLB operations
 *
 * Copyright (C) 1999  Niibe Yutaka
 * Copyright (C) 2002 - 2007 Paul Mundt
 */

// C dependencies supplied by the surrounding kernel translation.

pub unsafe fn __update_tlb(
    vma: *mut vm_area_struct,
    address: c_ulong,
    pte: pte_t,
) {
    let mut flags: c_ulong;
    let mut pteval: c_ulong;
    let mut vpn: c_ulong;

    /*
     * Handle debugger faulting in for debugee.
     */
    if !vma.is_null() && (*current).active_mm != (*vma).vm_mm {
        return;
    }

    local_irq_save(&mut flags);

    /* Set PTEH register */
    vpn = (address & MMU_VPN_MASK) | get_asid();
    __raw_writel(vpn, MMU_PTEH);

    pteval = pte.pte_low;

    /* Set PTEA register */
    #[cfg(CONFIG_X2TLB)]
    {
        /*
         * For the extended mode TLB this is trivial, only the ESZ and
         * EPR bits need to be written out to PTEA, with the remainder of
         * the protection bits (with the exception of the compat-mode SZ
         * and PR bits, which are cleared) being written out in PTEL.
         */
        __raw_writel(pte.pte_high, MMU_PTEA);
    }
    #[cfg(not(CONFIG_X2TLB))]
    {
        if ((*cpu_data).flags & CPU_HAS_PTEA) != 0 {
            /* The last 3 bits and the first one of pteval contains
             * the PTEA timing control and space attribute bits
             */
            __raw_writel(copy_ptea_attributes(pteval), MMU_PTEA);
        }
    }

    /* Set PTEL register */
    pteval &= _PAGE_FLAGS_HARDWARE_MASK; /* drop software flags */
    #[cfg(CONFIG_CACHE_WRITETHROUGH)]
    {
        pteval |= _PAGE_WT;
    }
    /* conveniently, we want all the software flags to be 0 anyway */
    __raw_writel(pteval, MMU_PTEL);

    /* Load the TLB */
    core::arch::asm!("ldtlb", options(nostack, preserves_flags));
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_one(asid: c_ulong, page: c_ulong) {
    let addr: c_ulong;
    let data: c_ulong;

    /*
     * NOTE: PTEH.ASID should be set to this MM
     *       _AND_ we need to write ASID to the array.
     *
     * It would be simple if we didn't need to set PTEH.ASID...
     */
    addr = MMU_UTLB_ADDRESS_ARRAY | MMU_PAGE_ASSOC_BIT;
    data = page | asid; /* VALID bit is off */
    jump_to_uncached();
    __raw_writel(data, addr);
    back_to_cached();
}

pub unsafe fn local_flush_tlb_all() {
    let mut flags: c_ulong;
    let mut status: c_ulong;
    let mut i: c_int;

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
    while i < status as c_int {
        __raw_writel(0x0, MMU_UTLB_ADDRESS_ARRAY | ((i as c_ulong) << 8));
        i += 1;
    }

    i = 0;
    while i < 4 {
        __raw_writel(0x0, MMU_ITLB_ADDRESS_ARRAY | ((i as c_ulong) << 8));
        i += 1;
    }

    back_to_cached();
    ctrl_barrier();
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
