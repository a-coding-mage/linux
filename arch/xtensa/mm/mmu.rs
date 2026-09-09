// SPDX-License-Identifier: GPL-2.0
/*
 * xtensa mmu stuff
 *
 * Extracted from init.c
 */

// C headers provide the declarations and architecture-specific definitions
// referenced below.

// DEFINE_PER_CPU(unsigned long, asid_cache) = ASID_USER_FIRST;
#[no_mangle]
pub static mut asid_cache: ::core::ffi::c_ulong = ASID_USER_FIRST;

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn init_pmd(vaddr: ::core::ffi::c_ulong, mut n_pages: ::core::ffi::c_ulong) -> *mut pte_t {
    let mut pmd: *mut pmd_t = pmd_off_k(vaddr);
    let pte: *mut pte_t;
    let mut i: ::core::ffi::c_ulong;

    n_pages = ALIGN(n_pages, PTRS_PER_PTE);

    pr_debug!(
        "%s: vaddr: 0x%08lx, n_pages: %ld\n",
        __func__, vaddr, n_pages
    );

    pte = memblock_alloc_low(n_pages * ::core::mem::size_of::<pte_t>() as _, PAGE_SIZE);
    if pte.is_null() {
        panic!(
            "%s: Failed to allocate %lu bytes align=%lx\n",
            __func__,
            n_pages * ::core::mem::size_of::<pte_t>() as _,
            PAGE_SIZE
        );
    }

    i = 0;
    while i < n_pages {
        pte_clear(::core::ptr::null_mut(), 0, pte.add(i as usize));
        i += 1;
    }

    i = 0;
    while i < n_pages {
        let cur_pte: *mut pte_t = pte.add(i as usize);

        BUG_ON(pmd_none(*pmd));
        set_pmd(pmd, __pmd((cur_pte as ::core::ffi::c_ulong) & PAGE_MASK));
        BUG_ON(cur_pte != pte_offset_kernel(pmd, 0));
        pr_debug!("%s: pmd: 0x%p, pte: 0x%p\n", __func__, pmd, cur_pte);

        i += PTRS_PER_PTE;
        pmd = pmd.add(1);
    }
    pte
}

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn fixedrange_init() {
    BUILD_BUG_ON!(FIXADDR_START < TLBTEMP_BASE_1 + TLBTEMP_SIZE);
    init_pmd(FIXADDR_START, __end_of_fixed_addresses);
}

pub unsafe fn paging_init() {
    // #ifdef CONFIG_HIGHMEM
    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        fixedrange_init();
        pkmap_page_table = init_pmd(PKMAP_BASE, LAST_PKMAP);
        kmap_init();
    }
}

/*
 * Flush the mmu and reset associated register to default values.
 */
pub unsafe fn init_mmu() {
    // #if !(XCHAL_HAVE_PTP_MMU && XCHAL_HAVE_SPANNING_WAY)
    #[cfg(not(all(feature = "XCHAL_HAVE_PTP_MMU", feature = "XCHAL_HAVE_SPANNING_WAY")))]
    {
        /*
         * Writing zeros to the instruction and data TLBCFG special
         * registers ensure that valid values exist in the register.
         *
         * For existing PGSZID<w> fields, zero selects the first element
         * of the page-size array.  For nonexistent PGSZID<w> fields,
         * zero is the best value to write.  Also, when changing PGSZID<w>
         * fields, the corresponding TLB must be flushed.
         */
        set_itlbcfg_register(0);
        set_dtlbcfg_register(0);
    }
    init_kio();
    local_flush_tlb_all();

    /* Set rasid register to a known value. */
    set_rasid_register(ASID_INSERT(ASID_USER_FIRST));

    /* Set PTEVADDR special register to the start of the page
     * table, which is in kernel mappable space (ie. not
     * statically mapped).  This register's value is undefined on
     * reset.
     */
    set_ptevaddr_register(XCHAL_PAGE_TABLE_VADDR);
}

pub unsafe fn init_kio() {
    // #if XCHAL_HAVE_PTP_MMU && XCHAL_HAVE_SPANNING_WAY && defined(CONFIG_USE_OF)
    #[cfg(all(
        feature = "XCHAL_HAVE_PTP_MMU",
        feature = "XCHAL_HAVE_SPANNING_WAY",
        feature = "CONFIG_USE_OF"
    ))]
    {
        /*
         * Update the IO area mapping in case xtensa_kio_paddr has changed
         */
        write_dtlb_entry(__pte(xtensa_kio_paddr + CA_WRITEBACK), XCHAL_KIO_CACHED_VADDR + 6);
        write_itlb_entry(__pte(xtensa_kio_paddr + CA_WRITEBACK), XCHAL_KIO_CACHED_VADDR + 6);
        write_dtlb_entry(__pte(xtensa_kio_paddr + CA_BYPASS), XCHAL_KIO_BYPASS_VADDR + 6);
        write_itlb_entry(__pte(xtensa_kio_paddr + CA_BYPASS), XCHAL_KIO_BYPASS_VADDR + 6);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
