// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Synopsys, Inc. (www.synopsys.com)
 */

/* Dependencies supplied by the corresponding Linux/ARC headers. */

/*
 * HIGHMEM API:
 *
 * kmap() API provides sleep semantics hence referred to as "permanent maps"
 * It allows mapping LAST_PKMAP pages, using @last_pkmap_nr as the cursor
 * for book-keeping
 *
 * kmap_atomic() can't sleep (calls pagefault_disable()), thus it provides
 * shortlived ala "temporary mappings" which historically were implemented as
 * fixmaps (compile time addr etc). Their book-keeping is done per cpu.
 *
 *	Both these facts combined (preemption disabled and per-cpu allocation)
 *	means the total number of concurrent fixmaps will be limited to max
 *	such allocations in a single control path. Thus KM_TYPE_NR (another
 *	historic relic) is a small'ish number which caps max percpu fixmaps
 *
 * ARC HIGHMEM Details
 *
 * - the kernel vaddr space from 0x7z to 0x8z (currently used by vmalloc/module)
 *   is now shared between vmalloc and kmap (non overlapping though)
 *
 * - Both fixmap/pkmap use a dedicated page table each, hooked up to swapper PGD
 *   This means each only has 1 PGDIR_SIZE worth of kvaddr mappings, which means
 *   2M of kvaddr space for typical config (8K page and 11:8:13 traversal split)
 *
 * - The fixed KMAP slots for kmap_local/atomic() require KM_MAX_IDX slots per
 *   CPU. So the number of CPUs sharing a single PTE page is limited.
 *
 * - pkmap being preemptible, in theory could do with more than 256 concurrent
 *   mappings. However, generic pkmap code: map_new_virtual(), doesn't traverse
 *   the PGD and only works with a single page table @pkmap_page_table, hence
 *   sets the limit
 */

extern "C" {
    static mut pkmap_page_table: *mut pte_t;

    fn pmd_off_k(kvaddr: c_ulong) -> *mut pmd_t;
    fn memblock_alloc_low(size: c_ulong, align: c_ulong) -> *mut core::ffi::c_void;
    fn panic(format: *const core::ffi::c_char, ...);
    fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t);
}

#[inline(never)]
unsafe fn alloc_kmap_pgtable(kvaddr: c_ulong) -> *mut pte_t {
    let pmd_k: *mut pmd_t = pmd_off_k(kvaddr);
    let pte_k: *mut pte_t;

    pte_k = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
    if pte_k.is_null() {
        panic(
            b"%s: Failed to allocate %lu bytes align=0x%lx\n\0".as_ptr()
                as *const core::ffi::c_char,
            b"alloc_kmap_pgtable\0".as_ptr(),
            PAGE_SIZE,
            PAGE_SIZE,
        );
    }

    pmd_populate_kernel(&raw mut init_mm, pmd_k, pte_k);
    pte_k
}

pub unsafe fn kmap_init() {
    /* Due to recursive include hell, we can't do this in processor.h */
    /* BUILD_BUG_ON(PAGE_OFFSET < (VMALLOC_END + FIXMAP_SIZE + PKMAP_SIZE)); */
    /* BUILD_BUG_ON(LAST_PKMAP > PTRS_PER_PTE); */
    /* BUILD_BUG_ON(FIX_KMAP_SLOTS > PTRS_PER_PTE); */

    pkmap_page_table = alloc_kmap_pgtable(PKMAP_BASE);
    alloc_kmap_pgtable(FIXMAP_BASE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
