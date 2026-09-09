/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TLB flushing on s390 is complicated. The following requirement
 * from the principles of operation is the most arduous:
 *
 * "A valid table entry must not be changed while it is attached
 * to any CPU and may be used for translation by that CPU except to
 * (1) invalidate the entry by using INVALIDATE PAGE TABLE ENTRY,
 * or INVALIDATE DAT TABLE ENTRY, (2) alter bits 56-63 of a page
 * table entry, or (3) make a change by means of a COMPARE AND SWAP
 * AND PURGE instruction that purges the TLB."
 *
 * The modification of a pte of an active mm struct therefore is
 * a two step process: i) invalidate the pte, ii) store the new pte.
 * This is true for the page protection bit as well.
 * The only possible optimization is to flush at the beginning of
 * a tlb_gather_mmu cycle if the mm_struct is currently not in use.
 *
 * Pages used for the page tables is a different story. FIXME: more
 */

// Dependencies supplied by the surrounding kernel translation are intentionally external.
unsafe extern "C" {
    fn __tlb_flush_mm_lazy(mm: *mut mm_struct);
    fn __tlb_adjust_range(tlb: *mut mmu_gather, address: c_ulong, size: c_ulong);
    fn free_folio_and_swap_cache(folio: *mut folio);
    fn page_folio(page: *mut page) -> *mut folio;
    fn encode_page(page: *mut page, bit: c_int) -> *mut encoded_page;
    fn encode_nr_pages(nr_pages: c_uint) -> *mut encoded_page;
    fn free_pages_and_swap_cache(pages: *mut *mut encoded_page, count: usize);
    fn mm_pmd_folded(mm: *mut mm_struct) -> bool;
    fn mm_p4d_folded(mm: *mut mm_struct) -> bool;
    fn mm_pud_folded(mm: *mut mm_struct) -> bool;
    fn virt_to_ptdesc(table: *mut core::ffi::c_void) -> *mut page;
    fn tlb_remove_ptdesc(tlb: *mut mmu_gather, ptdesc: *mut page);
    fn vm_warn_on_once(condition: bool);
}

pub const ENCODED_PAGE_BIT_NR_PAGES_NEXT: c_int = 0;

pub unsafe fn __tlb_remove_page_size(
    _tlb: *mut mmu_gather,
    page: *mut page,
    _page_size: c_int,
) -> bool {
    free_folio_and_swap_cache(page_folio(page));
    false
}

pub unsafe fn __tlb_remove_folio_pages(
    _tlb: *mut mmu_gather,
    page: *mut page,
    nr_pages: c_uint,
    delay_rmap: bool,
) -> bool {
    let encoded_pages: [*mut encoded_page; 2] = [
        encode_page(page, ENCODED_PAGE_BIT_NR_PAGES_NEXT),
        encode_nr_pages(nr_pages),
    ];

    vm_warn_on_once(delay_rmap);
    vm_warn_on_once(page_folio(page) != page_folio(page.add(nr_pages as usize - 1)));

    free_pages_and_swap_cache(encoded_pages.as_ptr() as *mut *mut encoded_page, encoded_pages.len());
    false
}

pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    __tlb_flush_mm_lazy((*tlb).mm);
}

/*
 * pte_free_tlb frees a pte table and clears the CRSTE for the
 * page table from the tlb.
 */
pub unsafe fn pte_free_tlb(tlb: *mut mmu_gather, pte: pgtable_t, address: c_ulong) {
    __tlb_adjust_range(tlb, address, PAGE_SIZE);
    (*tlb).mm.context.flush_mm = 1;
    (*tlb).freed_tables = 1;
    (*tlb).cleared_pmds = 1;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pte as *mut core::ffi::c_void));
}

/*
 * pmd_free_tlb frees a pmd table and clears the CRSTE for the
 * segment table entry from the tlb.
 * If the mm uses a two level page table the single pmd is freed
 * as the pgd. pmd_free_tlb checks the asce_limit against 2GB
 * to avoid the double free of the pmd in this case.
 */
pub unsafe fn pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t, address: c_ulong) {
    if mm_pmd_folded((*tlb).mm) { return; }
    __tlb_adjust_range(tlb, address, PAGE_SIZE);
    (*tlb).mm.context.flush_mm = 1;
    (*tlb).freed_tables = 1;
    (*tlb).cleared_puds = 1;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pmd as *mut core::ffi::c_void));
}

/*
 * p4d_free_tlb frees a pud table and clears the CRSTE for the
 * region second table entry from the tlb.
 * If the mm uses a four level page table the single p4d is freed
 * as the pgd. p4d_free_tlb checks the asce_limit against 8PB
 * to avoid the double free of the p4d in this case.
 */
pub unsafe fn p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t, address: c_ulong) {
    if mm_p4d_folded((*tlb).mm) { return; }
    __tlb_adjust_range(tlb, address, PAGE_SIZE);
    (*tlb).mm.context.flush_mm = 1;
    (*tlb).freed_tables = 1;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(p4d as *mut core::ffi::c_void));
}

/*
 * pud_free_tlb frees a pud table and clears the CRSTE for the
 * region third table entry from the tlb.
 * If the mm uses a three level page table the single pud is freed
 * as the pgd. pud_free_tlb checks the asce_limit against 4TB
 * to avoid the double free of the pud in this case.
 */
pub unsafe fn pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t, address: c_ulong) {
    if mm_pud_folded((*tlb).mm) { return; }
    __tlb_adjust_range(tlb, address, PAGE_SIZE);
    (*tlb).mm.context.flush_mm = 1;
    (*tlb).freed_tables = 1;
    (*tlb).cleared_p4ds = 1;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pud as *mut core::ffi::c_void));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
