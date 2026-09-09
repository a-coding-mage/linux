/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/huge_mm.h. External kernel types and helpers are supplied elsewhere. */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    pub fn do_huge_pmd_anonymous_page(vmf: *mut vm_fault) -> vm_fault_t;
    pub fn copy_huge_pmd(dst_mm: *mut mm_struct, src_mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, addr: c_ulong, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct) -> c_int;
    pub fn huge_pmd_set_accessed(vmf: *mut vm_fault) -> bool;
    pub fn copy_huge_pud(dst_mm: *mut mm_struct, src_mm: *mut mm_struct, dst_pud: *mut pud_t, src_pud: *mut pud_t, addr: c_ulong, vma: *mut vm_area_struct) -> c_int;
    pub fn huge_pud_set_accessed(vmf: *mut vm_fault, orig_pud: pud_t);
    pub fn do_huge_pmd_wp_page(vmf: *mut vm_fault) -> vm_fault_t;
    pub fn madvise_free_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong, next: c_ulong) -> bool;
    pub fn zap_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong) -> bool;
    pub fn zap_huge_pud(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pud: *mut pud_t, addr: c_ulong) -> c_int;
    pub fn move_huge_pmd(vma: *mut vm_area_struct, old_addr: c_ulong, new_addr: c_ulong, old_pmd: *mut pmd_t, new_pmd: *mut pmd_t) -> bool;
    pub fn change_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_int;
    pub fn vmf_insert_pfn_pmd(vmf: *mut vm_fault, pfn: c_ulong, write: bool) -> vm_fault_t;
    pub fn vmf_insert_pfn_pud(vmf: *mut vm_fault, pfn: c_ulong, write: bool) -> vm_fault_t;
    pub fn vmf_insert_folio_pmd(vmf: *mut vm_fault, folio: *mut folio, write: bool) -> vm_fault_t;
    pub fn vmf_insert_folio_pud(vmf: *mut vm_fault, folio: *mut folio, write: bool) -> vm_fault_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum transparent_hugepage_flag { TRANSPARENT_HUGEPAGE_UNSUPPORTED, TRANSPARENT_HUGEPAGE_FLAG, TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG, TRANSPARENT_HUGEPAGE_DEFRAG_DIRECT_FLAG, TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_FLAG, TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_OR_MADV_FLAG, TRANSPARENT_HUGEPAGE_DEFRAG_REQ_MADV_FLAG, TRANSPARENT_HUGEPAGE_DEFRAG_KHUGEPAGED_FLAG, TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG }
pub fn thp_vma_allowable_order(vma: *mut vm_area_struct, vm_flags: vm_flags_t, ty: tva_type, order: c_ulong) -> bool { unsafe { thp_vma_allowable_orders(vma, vm_flags, ty, BIT(order)) != 0 } }
pub unsafe fn split_folio(f: *mut folio) -> c_int { split_folio_to_list(f, core::ptr::null_mut()) }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tva_type { TVA_SMAPS, TVA_PAGEFAULT, TVA_KHUGEPAGED, TVA_FORCED_COLLAPSE }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum split_type { SPLIT_TYPE_UNIFORM, SPLIT_TYPE_NON_UNIFORM }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mthp_stat_item { MTHP_STAT_ANON_FAULT_ALLOC, MTHP_STAT_ANON_FAULT_FALLBACK, MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE, MTHP_STAT_COLLAPSE_ALLOC, MTHP_STAT_COLLAPSE_ALLOC_FAILED, MTHP_STAT_ZSWPOUT, MTHP_STAT_SWPIN, MTHP_STAT_SWPIN_FALLBACK, MTHP_STAT_SWPIN_FALLBACK_CHARGE, MTHP_STAT_SWPOUT, MTHP_STAT_SWPOUT_FALLBACK, MTHP_STAT_SHMEM_ALLOC, MTHP_STAT_SHMEM_FALLBACK, MTHP_STAT_SHMEM_FALLBACK_CHARGE, MTHP_STAT_SPLIT, MTHP_STAT_SPLIT_FAILED, MTHP_STAT_SPLIT_DEFERRED, MTHP_STAT_NR_ANON, MTHP_STAT_NR_ANON_PARTIALLY_MAPPED, MTHP_STAT_COLLAPSE_EXCEED_SWAP, MTHP_STAT_COLLAPSE_EXCEED_NONE, MTHP_STAT_COLLAPSE_EXCEED_SHARED, __MTHP_STAT_COUNT }

extern "C" {
    pub fn single_hugepage_flag_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: usize, flag: transparent_hugepage_flag) -> isize;
    pub fn single_hugepage_flag_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, flag: transparent_hugepage_flag) -> isize;
    pub static mut shmem_enabled_attr: kobj_attribute;
    pub static mut thpsize_shmem_enabled_attr: kobj_attribute;
    pub fn __thp_vma_allowable_orders(vma: *mut vm_area_struct, vm_flags: vm_flags_t, ty: tva_type, orders: c_ulong) -> c_ulong;
    pub fn thp_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    pub fn thp_get_unmapped_area_vmaflags(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong;
    pub fn __split_huge_page_to_list_to_order(page: *mut page, list: *mut list_head, new_order: c_uint) -> c_int;
    pub fn folio_split_unmapped(folio: *mut folio, new_order: c_uint) -> c_int;
    pub fn min_order_for_split(folio: *mut folio) -> c_uint;
    pub fn split_folio_to_list(folio: *mut folio, list: *mut list_head) -> c_int;
    pub fn folio_check_splittable(folio: *mut folio, new_order: c_uint, ty: split_type) -> c_int;
    pub fn folio_split(folio: *mut folio, new_order: c_uint, page: *mut page, list: *mut list_head) -> c_int;
    pub fn folio_memcg_alloc_deferred(folio: *mut folio) -> c_int;
    pub fn deferred_split_folio(folio: *mut folio, partially_mapped: bool);
    pub fn __split_huge_pmd(vma: *mut vm_area_struct, pmd: *mut pmd_t, address: c_ulong, freeze: bool);
    pub fn split_huge_pmd_address(vma: *mut vm_area_struct, address: c_ulong, freeze: bool);
    pub fn __split_huge_pud(vma: *mut vm_area_struct, pud: *mut pud_t, address: c_ulong);
    pub fn hugepage_madvise(vma: *mut vm_area_struct, vm_flags: *mut vm_flags_t, advice: c_int) -> c_int;
    pub fn madvise_collapse(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, lock_dropped: *mut bool) -> c_int;
    pub fn vma_adjust_trans_huge(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, next: *mut vm_area_struct);
    pub fn __pmd_trans_huge_lock(pmd: *mut pmd_t, vma: *mut vm_area_struct) -> *mut spinlock_t;
    pub fn __pud_trans_huge_lock(pud: *mut pud_t, vma: *mut vm_area_struct) -> *mut spinlock_t;
    pub fn do_huge_pmd_numa_page(vmf: *mut vm_fault) -> vm_fault_t;
    pub fn do_huge_pmd_uffd_rwp(vmf: *mut vm_fault) -> vm_fault_t;
    pub fn do_huge_pmd_device_private(vmf: *mut vm_fault) -> vm_fault_t;
    pub static mut huge_zero_folio: *mut folio;
    pub static mut huge_zero_pfn: c_ulong;
    pub fn mm_get_huge_zero_folio(mm: *mut mm_struct) -> *mut folio;
    pub fn mm_put_huge_zero_folio(mm: *mut mm_struct);
    pub fn split_huge_pmd_locked(vma: *mut vm_area_struct, address: c_ulong, pmd: *mut pmd_t, freeze: bool);
    pub fn unmap_huge_pmd_locked(vma: *mut vm_area_struct, addr: c_ulong, pmdp: *mut pmd_t, folio: *mut folio) -> bool;
    pub fn map_anon_folio_pmd_nopf(folio: *mut folio, pmd: *mut pmd_t, vma: *mut vm_area_struct, haddr: c_ulong);
}

/* Build-time constants and helpers are supplied by the kernel translation unit. */
pub const THP_ORDERS_ALL_ANON: c_ulong = (BIT(PMD_ORDER + 1) - 1) & !(BIT(0) | BIT(1));
pub const THP_ORDERS_ALL_SPECIAL_DAX: c_ulong = BIT(PMD_ORDER) | BIT(PUD_ORDER);
pub const THP_ORDERS_ALL_FILE_DEFAULT: c_ulong = (BIT(MAX_PAGECACHE_ORDER + 1) - 1) & !BIT(0);
pub const THP_ORDERS_ALL: c_ulong = THP_ORDERS_ALL_ANON | THP_ORDERS_ALL_SPECIAL_DAX | THP_ORDERS_ALL_FILE_DEFAULT;
pub const HPAGE_PMD_SHIFT: c_ulong = PMD_SHIFT;
pub const HPAGE_PUD_SHIFT: c_ulong = PUD_SHIFT;
pub const HPAGE_PMD_ORDER: c_ulong = HPAGE_PMD_SHIFT - PAGE_SHIFT;
pub const HPAGE_PMD_NR: c_ulong = 1 << HPAGE_PMD_ORDER;
pub const HPAGE_PMD_MASK: c_ulong = !(HPAGE_PMD_SIZE - 1);
pub const HPAGE_PMD_SIZE: c_ulong = 1 << HPAGE_PMD_SHIFT;
pub const HPAGE_PUD_ORDER: c_ulong = HPAGE_PUD_SHIFT - PAGE_SHIFT;
pub const HPAGE_PUD_NR: c_ulong = 1 << HPAGE_PUD_ORDER;
pub const HPAGE_PUD_MASK: c_ulong = !(HPAGE_PUD_SIZE - 1);
pub const HPAGE_PUD_SIZE: c_ulong = 1 << HPAGE_PUD_SHIFT;

extern "C" {
    pub static mut transparent_hugepage_flags: c_ulong;
    pub static mut huge_anon_orders_always: c_ulong;
    pub static mut huge_anon_orders_madvise: c_ulong;
    pub static mut huge_anon_orders_inherit: c_ulong;
}

#[inline] pub unsafe fn hugepage_global_enabled() -> bool { transparent_hugepage_flags & (BIT(TRANSPARENT_HUGEPAGE_FLAG as c_ulong) | BIT(TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG as c_ulong)) != 0 }
#[inline] pub unsafe fn hugepage_global_always() -> bool { transparent_hugepage_flags & BIT(TRANSPARENT_HUGEPAGE_FLAG as c_ulong) != 0 }
#[inline] pub fn highest_order(orders: c_ulong) -> c_int { unsafe { fls_long(orders) - 1 } }
#[inline] pub unsafe fn next_order(orders: *mut c_ulong, prev: c_int) -> c_int { *orders &= !BIT(prev as c_ulong); highest_order(*orders) }

pub unsafe fn thp_vma_suitable_order(vma: *mut vm_area_struct, addr: c_ulong, order: c_int) -> bool {
    let hpage_size = PAGE_SIZE << order;
    if !vma_is_anonymous(vma) && !IS_ALIGNED(((*vma).vm_start >> PAGE_SHIFT) - (*vma).vm_pgoff, hpage_size >> PAGE_SHIFT) { return false; }
    let haddr = ALIGN_DOWN(addr, hpage_size);
    !(haddr < (*vma).vm_start || haddr + hpage_size > (*vma).vm_end)
}
pub unsafe fn thp_vma_suitable_orders(vma: *mut vm_area_struct, addr: c_ulong, mut orders: c_ulong) -> c_ulong { let mut order = highest_order(orders); while orders != 0 { if thp_vma_suitable_order(vma, addr, order) { break; } order = next_order(&mut orders, order); } orders }
pub unsafe fn thp_vma_allowable_orders(vma: *mut vm_area_struct, vm_flags: vm_flags_t, ty: tva_type, mut orders: c_ulong) -> c_ulong { if ty as c_int != TVA_FORCED_COLLAPSE as c_int && vma_is_anonymous(vma) { let mut mask = READ_ONCE(huge_anon_orders_always); if vm_flags & VM_HUGEPAGE != 0 { mask |= READ_ONCE(huge_anon_orders_madvise); } if hugepage_global_always() || (vm_flags & VM_HUGEPAGE != 0 && hugepage_global_enabled()) { mask |= READ_ONCE(huge_anon_orders_inherit); } orders &= mask; if orders == 0 { return 0; } } __thp_vma_allowable_orders(vma, vm_flags, ty, orders) }
pub unsafe fn vma_thp_disabled(vma: *mut vm_area_struct, vm_flags: vm_flags_t, forced_collapse: bool) -> bool { if vm_flags & VM_NOHUGEPAGE != 0 { return true; } if mm_flags_test(MMF_DISABLE_THP_COMPLETELY, (*vma).vm_mm) { return true; } if vm_flags & VM_HUGEPAGE != 0 { return false; } if forced_collapse { return false; } mm_flags_test(MMF_DISABLE_THP_EXCEPT_ADVISED, (*vma).vm_mm) }
pub unsafe fn thp_disabled_by_hw() -> bool { transparent_hugepage_flags & BIT(TRANSPARENT_HUGEPAGE_UNSUPPORTED as c_ulong) != 0 }

#[repr(C)] pub struct thpsize { pub kobj: kobject, pub node: list_head, pub order: c_int }
pub unsafe fn transparent_hugepage_use_zero_page() -> bool { transparent_hugepage_flags & BIT(TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG as c_ulong) != 0 }
pub unsafe fn thp_shmem_limit_gfp_mask(huge_gfp: gfp_t, limit_gfp: gfp_t) -> gfp_t { let allowflags = __GFP_IO | __GFP_FS | __GFP_RECLAIM; let denyflags = __GFP_NOWARN | __GFP_NORETRY; let zoneflags = limit_gfp & GFP_ZONEMASK; let mut result = huge_gfp & !(allowflags | GFP_ZONEMASK); result |= zoneflags; result |= limit_gfp & denyflags; result |= (huge_gfp & limit_gfp) & allowflags; result }
pub unsafe fn split_huge_page_to_list_to_order(page: *mut page, list: *mut list_head, new_order: c_uint) -> c_int { __split_huge_page_to_list_to_order(page, list, new_order) }
pub unsafe fn split_huge_page_to_order(page: *mut page, new_order: c_uint) -> c_int { split_huge_page_to_list_to_order(page, core::ptr::null_mut(), new_order) }
pub unsafe fn split_huge_page(page: *mut page) -> c_int { split_huge_page_to_order(page, 0) }
pub unsafe fn pmd_is_huge(pmd: pmd_t) -> bool { if pmd_present(pmd) { pmd_trans_huge(pmd) } else if !pmd_none(pmd) { true } else { false } }
pub unsafe fn pud_is_huge(pud: pud_t) -> bool { if pud_present(pud) { pud_trans_huge(pud) } else if !pud_none(pud) { true } else { false } }
pub unsafe fn __split_huge_pmd_noop(_vma: *mut vm_area_struct, _pmd: *mut pmd_t, _address: c_ulong, _freeze: bool) {}
pub unsafe fn split_huge_pmd_address_noop(_vma: *mut vm_area_struct, _address: c_ulong, _freeze: bool) {}
pub unsafe fn split_huge_pmd_locked_noop(_vma: *mut vm_area_struct, _address: c_ulong, _pmd: *mut pmd_t, _freeze: bool) {}
pub unsafe fn unmap_huge_pmd_locked_noop(_vma: *mut vm_area_struct, _addr: c_ulong, _pmdp: *mut pmd_t, _folio: *mut folio) -> bool { false }
pub unsafe fn pmd_trans_huge_lock(pmd: *mut pmd_t, vma: *mut vm_area_struct) -> *mut spinlock_t { if pmd_is_huge(*pmd) { __pmd_trans_huge_lock(pmd, vma) } else { core::ptr::null_mut() } }
pub unsafe fn pud_trans_huge_lock(pud: *mut pud_t, vma: *mut vm_area_struct) -> *mut spinlock_t { if pud_trans_huge(*pud) { __pud_trans_huge_lock(pud, vma) } else { core::ptr::null_mut() } }
pub unsafe fn folio_test_pmd_mappable(folio: *mut folio) -> bool { folio_order(folio) >= HPAGE_PMD_ORDER }
pub unsafe fn is_huge_zero_folio(folio: *const folio) -> bool { READ_ONCE(huge_zero_folio) == folio as *mut folio }
pub unsafe fn is_huge_zero_pfn(pfn: c_ulong) -> bool { READ_ONCE(huge_zero_pfn) == (pfn & !(HPAGE_PMD_NR - 1)) }
pub unsafe fn is_huge_zero_pmd(pmd: pmd_t) -> bool { pmd_present(pmd) && is_huge_zero_pfn(pmd_pfn(pmd)) }
pub unsafe fn get_persistent_huge_zero_folio() -> *mut folio { if !IS_ENABLED(CONFIG_PERSISTENT_HUGE_ZERO_FOLIO) || huge_zero_folio.is_null() { core::ptr::null_mut() } else { huge_zero_folio } }
pub fn thp_migration_supported() -> bool { IS_ENABLED(CONFIG_ARCH_HAS_PMD_SOFTLEAVES) }
pub fn is_pmd_order(order: c_uint) -> bool { order as c_ulong == HPAGE_PMD_ORDER }
pub unsafe fn split_folio_to_order(folio: *mut folio, new_order: c_int) -> c_int { split_huge_page_to_list_to_order(&mut (*folio).page, core::ptr::null_mut(), new_order as c_uint) }
pub unsafe fn largest_zero_folio() -> *mut folio { let folio = get_persistent_huge_zero_folio(); if !folio.is_null() { folio } else { page_folio(ZERO_PAGE(0)) } }

/* The CONFIG_TRANSPARENT_HUGEPAGE-disabled branch supplies zero/error stubs in C. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
