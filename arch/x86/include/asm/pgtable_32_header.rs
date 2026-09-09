/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/pgtable_32.h.
//
// The Linux memory management assumes a three-level page table setup. On the
// i386, the mid level is folded into the top-level page table, so that the
// physical page table remains the two-level page table expected by the i386
// MMU.

// Dependencies supplied by the surrounding translation unit:
// pgd_t, pmd_t, pte_t, PTRS_PER_PMD, PTRS_PER_PGD, __PAGE_OFFSET,
// PAGE_SHIFT, _ULL, init_mm, pte_clear, and flush_tlb_one_kernel.

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut swapper_pg_dir: [pgd_t; 1024];
    pub static mut initial_page_table: [pgd_t; 1024];
    pub static mut initial_pg_pmd: [pmd_t; 0];

    pub fn paging_init();
    pub fn sync_initial_page_table();
}

/* Clear a kernel PTE and flush it from the TLB. */
#[inline]
pub unsafe fn kpte_clear_flush(ptep: *mut pte_t, vaddr: usize) {
    pte_clear(&mut init_mm, vaddr, ptep);
    flush_tlb_one_kernel(vaddr);
}

/*
 * This is used to calculate the .brk reservation for initial pagetables.
 * Enough space is reserved to allocate pagetables sufficient to cover all of
 * LOWMEM_PAGES, which is an upper bound on the size of the direct map of
 * lowmem.
 *
 * With PAE paging (PTRS_PER_PMD > 1), we allocate PTRS_PER_PGD == 4 pages for
 * the PMD's in addition to the pages required for the last level pagetables.
 */
#[inline]
pub const fn page_table_size(pages: usize) -> usize {
    if PTRS_PER_PMD > 1 {
        (pages / PTRS_PER_PMD) + PTRS_PER_PGD
    } else {
        pages / PTRS_PER_PGD
    }
}

/*
 * Number of possible pages in the lowmem region.
 *
 * We shift 2 by 31 instead of 1 by 32 to the left in order to avoid a
 * gas warning about overflowing shift count when gas has been compiled
 * with only a host target support using a 32-bit type for internal
 * representation.
 */
pub const LOWMEM_PAGES: usize = (((2u64 << 31) - __PAGE_OFFSET) >> PAGE_SHIFT);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
