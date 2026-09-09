/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic TLB shootdown code. Direct Rust translation of asm-generic/tlb.h. */

#[cfg(not(feature = "nmi_uaccess_okay"))]
#[inline]
pub const fn nmi_uaccess_okay() -> bool { true }

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct mmu_table_batch {
    #[cfg(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE")]
    pub rcu: rcu_head,
    pub nr: c_uint,
    pub tables: [*mut c_void; 0],
}

#[cfg(feature = "CONFIG_MMU")]
pub const MAX_TABLE_BATCH: usize = (PAGE_SIZE - core::mem::size_of::<mmu_table_batch>()) / core::mem::size_of::<*mut c_void>();

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_HAVE_ARCH_TLB_REMOVE_TABLE")))]
#[inline]
pub unsafe fn __tlb_remove_table(table: *mut c_void) {
    let ptdesc = table as *mut ptdesc;
    pagetable_dtor_free(ptdesc);
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_MMU_GATHER_TABLE_FREE"))]
extern "C" { pub fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut c_void); }

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_TABLE_FREE")))]
extern "C" { pub fn tlb_remove_page(tlb: *mut mmu_gather, page: *mut page); }

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_TABLE_FREE")))]
#[inline]
pub unsafe fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut c_void) {
    let ptdesc = table as *mut ptdesc;
    pagetable_dtor(ptdesc);
    tlb_remove_page(tlb, ptdesc_page(ptdesc));
}

#[cfg(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE")]
#[inline]
pub const fn tlb_needs_table_invalidate() -> bool { true }

#[cfg(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE")]
extern "C" {
    pub fn tlb_remove_table_sync_one();
    pub fn tlb_remove_table_sync_rcu();
}

#[cfg(not(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE"))]
#[inline] pub fn tlb_remove_table_sync_one() {}
#[cfg(not(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE"))]
#[inline] pub fn tlb_remove_table_sync_rcu() {}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_NO_GATHER")))]
pub const MMU_GATHER_BUNDLE: usize = 8;

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_NO_GATHER")))]
#[repr(C)]
pub struct mmu_gather_batch {
    pub next: *mut mmu_gather_batch,
    pub nr: c_uint,
    pub max: c_uint,
    pub encoded_pages: [*mut encoded_page; 0],
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_NO_GATHER")))]
pub const MAX_GATHER_BATCH: usize = (PAGE_SIZE - core::mem::size_of::<mmu_gather_batch>()) / core::mem::size_of::<*mut c_void>();
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_NO_GATHER")))]
pub const MAX_GATHER_BATCH_COUNT: usize = 10000usize / MAX_GATHER_BATCH;

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_MMU_GATHER_NO_GATHER")))]
extern "C" {
    pub fn __tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: c_int) -> bool;
    pub fn __tlb_remove_folio_pages(tlb: *mut mmu_gather, page: *mut page, nr_pages: c_uint, delay_rmap: bool) -> bool;
}

#[repr(C)]
pub struct mmu_gather {
    pub mm: *mut mm_struct,
    #[cfg(feature = "CONFIG_MMU_GATHER_TABLE_FREE")]
    pub batch: *mut mmu_table_batch,
    pub start: c_ulong,
    pub end: c_ulong,
    pub fullmm: c_uint,
    pub need_flush_all: c_uint,
    pub freed_tables: c_uint,
    pub delayed_rmap: c_uint,
    pub cleared_ptes: c_uint,
    pub cleared_pmds: c_uint,
    pub cleared_puds: c_uint,
    pub cleared_p4ds: c_uint,
    pub vma_exec: c_uint,
    pub vma_huge: c_uint,
    pub vma_pfn: c_uint,
    pub unshared_tables: c_uint,
    pub fully_unshared_tables: c_uint,
    pub batch_count: c_uint,
    #[cfg(not(feature = "CONFIG_MMU_GATHER_NO_GATHER"))]
    pub active: *mut mmu_gather_batch,
    #[cfg(not(feature = "CONFIG_MMU_GATHER_NO_GATHER"))]
    pub local: mmu_gather_batch,
    #[cfg(not(feature = "CONFIG_MMU_GATHER_NO_GATHER"))]
    pub __pages: [*mut page; MMU_GATHER_BUNDLE],
    #[cfg(all(not(feature = "CONFIG_MMU_GATHER_NO_GATHER"), feature = "CONFIG_MMU_GATHER_PAGE_SIZE"))]
    pub page_size: c_uint,
}

extern "C" { pub fn tlb_flush_mmu(tlb: *mut mmu_gather); }

#[inline]
pub unsafe fn __tlb_adjust_range(tlb: *mut mmu_gather, address: c_ulong, range_size: c_uint) {
    (*tlb).start = core::cmp::min((*tlb).start, address);
    (*tlb).end = core::cmp::max((*tlb).end, address.wrapping_add(range_size as c_ulong));
}

#[inline]
pub unsafe fn __tlb_reset_range(tlb: *mut mmu_gather) {
    if (*tlb).fullmm != 0 { (*tlb).start = !0; (*tlb).end = !0; }
    else { (*tlb).start = TASK_SIZE; (*tlb).end = 0; }
    (*tlb).freed_tables = 0; (*tlb).cleared_ptes = 0; (*tlb).cleared_pmds = 0;
    (*tlb).cleared_puds = 0; (*tlb).cleared_p4ds = 0; (*tlb).unshared_tables = 0;
}

#[cfg(feature = "CONFIG_MMU_GATHER_NO_RANGE")]
#[inline] pub unsafe fn tlb_flush(tlb: *mut mmu_gather) { if (*tlb).end != 0 { flush_tlb_mm((*tlb).mm); } }

#[cfg(all(not(feature = "CONFIG_MMU_GATHER_NO_RANGE"), not(feature = "tlb_flush")))]
#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    if (*tlb).fullmm != 0 || (*tlb).need_flush_all != 0 { flush_tlb_mm((*tlb).mm); }
    else if (*tlb).end != 0 {
        let vma = vm_area_struct { vm_mm: (*tlb).mm, vm_flags: (if (*tlb).vma_exec != 0 { VM_EXEC } else { 0 }) | (if (*tlb).vma_huge != 0 { VM_HUGETLB } else { 0 }) };
        flush_tlb_range(&vma, (*tlb).start, (*tlb).end);
    }
}

#[inline]
pub unsafe fn tlb_update_vma_flags(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    (*tlb).vma_huge = is_vm_hugetlb_page(vma) as c_uint;
    (*tlb).vma_exec = (((*vma).vm_flags & VM_EXEC) != 0) as c_uint;
    (*tlb).vma_pfn |= (((*vma).vm_flags & (VM_PFNMAP | VM_MIXEDMAP)) != 0) as c_uint;
}

#[inline]
pub unsafe fn tlb_flush_mmu_tlbonly(tlb: *mut mmu_gather) {
    if (*tlb).freed_tables == 0 && (*tlb).cleared_ptes == 0 && (*tlb).cleared_pmds == 0 && (*tlb).cleared_puds == 0 && (*tlb).cleared_p4ds == 0 && (*tlb).unshared_tables == 0 { return; }
    tlb_flush(tlb); __tlb_reset_range(tlb);
}

#[inline]
pub unsafe fn tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: c_int) {
    if __tlb_remove_page_size(tlb, page, page_size) { tlb_flush_mmu(tlb); }
}
#[inline] pub unsafe fn tlb_remove_page(tlb: *mut mmu_gather, page: *mut page) { tlb_remove_page_size(tlb, page, PAGE_SIZE as c_int); }
#[inline] pub unsafe fn tlb_remove_ptdesc(tlb: *mut mmu_gather, pt: *mut ptdesc) { tlb_remove_table(tlb, pt as *mut c_void); }

#[inline]
pub unsafe fn tlb_change_page_size(tlb: *mut mmu_gather, page_size: c_uint) {
    #[cfg(feature = "CONFIG_MMU_GATHER_PAGE_SIZE")]
    { if (*tlb).page_size != 0 && (*tlb).page_size != page_size && (*tlb).fullmm == 0 && (*tlb).need_flush_all == 0 { tlb_flush_mmu(tlb); } (*tlb).page_size = page_size; }
}

#[inline] pub unsafe fn tlb_get_unmap_shift(tlb: *mut mmu_gather) -> c_ulong { if (*tlb).cleared_ptes != 0 { PAGE_SHIFT } else if (*tlb).cleared_pmds != 0 { PMD_SHIFT } else if (*tlb).cleared_puds != 0 { PUD_SHIFT } else if (*tlb).cleared_p4ds != 0 { P4D_SHIFT } else { PAGE_SHIFT } }
#[inline] pub unsafe fn tlb_get_unmap_size(tlb: *mut mmu_gather) -> c_ulong { 1u64.wrapping_shl(tlb_get_unmap_shift(tlb) as u32) as c_ulong }

#[inline]
pub unsafe fn tlb_start_vma(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    if (*tlb).fullmm != 0 { return; }
    tlb_update_vma_flags(tlb, vma);
    #[cfg(not(feature = "CONFIG_MMU_GATHER_NO_FLUSH_CACHE"))]
    flush_cache_range(vma, (*vma).vm_start, (*vma).vm_end);
}
#[inline] pub unsafe fn tlb_end_vma(tlb: *mut mmu_gather, _vma: *mut vm_area_struct) { if (*tlb).fullmm == 0 && !cfg!(feature = "CONFIG_MMU_GATHER_MERGE_VMAS") { tlb_flush_mmu_tlbonly(tlb); } }
#[inline] pub unsafe fn tlb_free_vmas(tlb: *mut mmu_gather) { if (*tlb).fullmm == 0 && (*tlb).vma_pfn != 0 { tlb_flush_mmu_tlbonly(tlb); } }

#[inline] pub unsafe fn tlb_flush_pte_range(tlb: *mut mmu_gather, address: c_ulong, size: c_ulong) { __tlb_adjust_range(tlb, address, size as c_uint); (*tlb).cleared_ptes = 1; }
#[inline] pub unsafe fn tlb_flush_pmd_range(tlb: *mut mmu_gather, address: c_ulong, size: c_ulong) { __tlb_adjust_range(tlb, address, size as c_uint); (*tlb).cleared_pmds = 1; }
#[inline] pub unsafe fn tlb_flush_pud_range(tlb: *mut mmu_gather, address: c_ulong, size: c_ulong) { __tlb_adjust_range(tlb, address, size as c_uint); (*tlb).cleared_puds = 1; }
#[inline] pub unsafe fn tlb_flush_p4d_range(tlb: *mut mmu_gather, address: c_ulong, size: c_ulong) { __tlb_adjust_range(tlb, address, size as c_uint); (*tlb).cleared_p4ds = 1; }

#[cfg(not(feature = "__tlb_remove_tlb_entry"))]
#[inline] pub unsafe fn __tlb_remove_tlb_entry(_tlb: *mut mmu_gather, _ptep: *mut pte_t, _address: c_ulong) {}

#[inline]
pub unsafe fn tlb_remove_tlb_entries(tlb: *mut mmu_gather, mut ptep: *mut pte_t, mut nr: c_uint, mut address: c_ulong) {
    tlb_flush_pte_range(tlb, address, PAGE_SIZE.wrapping_mul(nr as usize) as c_ulong);
    loop { __tlb_remove_tlb_entry(tlb, ptep, address); nr -= 1; if nr == 0 { break; } ptep = ptep.add(1); address = address.wrapping_add(PAGE_SIZE as c_ulong); }
}

#[inline] pub unsafe fn pte_needs_flush(_oldpte: pte_t, _newpte: pte_t) -> bool { true }
#[inline] pub unsafe fn huge_pmd_needs_flush(_oldpmd: pmd_t, _newpmd: pmd_t) -> bool { true }

#[cfg(feature = "CONFIG_HUGETLB_PMD_PAGE_TABLE_SHARING")]
pub unsafe fn tlb_unshare_pmd_ptdesc(tlb: *mut mmu_gather, pt: *mut ptdesc, addr: c_ulong) {
    VM_WARN_ON_ONCE(!ptdesc_pmd_is_shared(pt)); ptdesc_pmd_pts_dec(pt);
    tlb_flush_pmd_range(tlb, addr & PUD_MASK, PUD_SIZE);
    if !ptdesc_pmd_is_shared(pt) { (*tlb).fully_unshared_tables = 1; }
    (*tlb).unshared_tables = 1;
}
#[cfg(feature = "CONFIG_HUGETLB_PMD_PAGE_TABLE_SHARING")]
pub unsafe fn tlb_flush_unshared_tables(tlb: *mut mmu_gather) {
    if (*tlb).unshared_tables != 0 { tlb_flush_mmu_tlbonly(tlb); }
    if (*tlb).fully_unshared_tables != 0 { tlb_remove_table_sync_one(); (*tlb).fully_unshared_tables = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
