/*
 * Xtensa KASAN shadow map initialization
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2017 Cadence Design Systems Inc.
 */

// The following types, constants, globals, and functions are supplied by the
// corresponding kernel dependencies.

pub unsafe fn kasan_early_init() {
    let mut vaddr: usize = KASAN_SHADOW_START;
    let mut pmd: *mut pmd_t = pmd_off_k(vaddr);
    let mut i: i32;

    i = 0;
    while i < PTRS_PER_PTE {
        set_pte(
            kasan_early_shadow_pte.add(i as usize),
            mk_pte(virt_to_page(kasan_early_shadow_page), PAGE_KERNEL),
        );
        i += 1;
    }

    vaddr = 0;
    while vaddr < KASAN_SHADOW_SIZE {
        BUG_ON(!pmd_none(*pmd));
        set_pmd(pmd, __pmd(kasan_early_shadow_pte as usize));
        vaddr += PMD_SIZE;
        pmd = pmd.add(1);
    }
}

unsafe fn populate(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) {
    let n_pages: usize = (end as usize - start as usize) / PAGE_SIZE;
    let n_pmds: usize = n_pages / PTRS_PER_PTE as usize;
    let mut i: usize;
    let mut j: usize;
    let vaddr: usize = start as usize;
    let pmd: *mut pmd_t = pmd_off_k(vaddr);
    let mut pte: *mut pte_t = memblock_alloc_or_panic(
        n_pages * core::mem::size_of::<pte_t>(),
        PAGE_SIZE,
    );

    pr_debug!("%s: %p - %p\n", "populate", start, end);

    i = 0;
    j = 0;
    while i < n_pmds {
        let mut k: i32 = 0;

        while k < PTRS_PER_PTE {
            let phys: phys_addr_t = memblock_phys_alloc_range(
                PAGE_SIZE,
                PAGE_SIZE,
                0,
                MEMBLOCK_ALLOC_ANYWHERE,
            );

            if phys == 0 {
                panic!("Failed to allocate page table page\n");
            }

            set_pte(
                pte.add(j),
                pfn_pte(PHYS_PFN(phys), PAGE_KERNEL),
            );
            k += 1;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < n_pmds {
        set_pmd(pmd.add(i), __pmd(pte.add(i * PTRS_PER_PTE) as usize));
        i += 1;
    }

    local_flush_tlb_all();
    memset(start, 0, end as usize - start as usize);
}

pub unsafe fn kasan_init() {
    let mut i: i32;

    BUILD_BUG_ON!(KASAN_SHADOW_OFFSET != KASAN_SHADOW_START -
        (KASAN_START_VADDR >> KASAN_SHADOW_SCALE_SHIFT));
    BUILD_BUG_ON!(VMALLOC_START < KASAN_START_VADDR);

    /*
     * Replace shadow map pages that cover addresses from VMALLOC area
     * start to the end of KSEG with clean writable pages.
     */
    populate(
        kasan_mem_to_shadow(VMALLOC_START as *mut core::ffi::c_void),
        kasan_mem_to_shadow(XCHAL_KSEG_BYPASS_VADDR as *mut core::ffi::c_void),
    );

    /*
     * Write protect kasan_early_shadow_page and zero-initialize it again.
     */
    i = 0;
    while i < PTRS_PER_PTE {
        set_pte(
            kasan_early_shadow_pte.add(i as usize),
            mk_pte(virt_to_page(kasan_early_shadow_page), PAGE_KERNEL_RO),
        );
        i += 1;
    }

    local_flush_tlb_all();
    memset(kasan_early_shadow_page, 0, PAGE_SIZE);

    /* At this point kasan is fully initialized. Enable error messages. */
    (*current).kasan_depth = 0;
    kasan_init_generic();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
