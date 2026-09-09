/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Memory merging support.
 *
 * This code enables dynamic sharing of identical pages found in different
 * memory areas, even if they are not shared by fork().
 *
 * The original C declarations below depend on the corresponding Linux kernel
 * types and helpers supplied by the including build environment.
 */

#[cfg(CONFIG_KSM)]
extern "C" {
    pub fn ksm_madvise(
        vma: *mut vm_area_struct,
        start: c_ulong,
        end: c_ulong,
        advice: c_int,
        vm_flags: *mut vm_flags_t,
    ) -> c_int;
    pub fn ksm_vma_flags(
        mm: *mut mm_struct,
        file: *const file,
        vma_flags: vma_flags_t,
    ) -> vma_flags_t;
    pub fn ksm_enable_merge_any(mm: *mut mm_struct) -> c_int;
    pub fn ksm_disable_merge_any(mm: *mut mm_struct) -> c_int;
    pub fn ksm_disable(mm: *mut mm_struct) -> c_int;
    pub fn __ksm_enter(mm: *mut mm_struct) -> c_int;
    pub fn __ksm_exit(mm: *mut mm_struct);
    pub static mut ksm_zero_pages: atomic_long_t;
    pub fn ksm_might_need_to_copy(
        folio: *mut folio,
        vma: *mut vm_area_struct,
        addr: c_ulong,
    ) -> *mut folio;
    pub fn rmap_walk_ksm(folio: *mut folio, rwc: *mut rmap_walk_control);
    pub fn folio_migrate_ksm(newfolio: *mut folio, folio: *mut folio);
    pub fn collect_procs_ksm(
        folio: *const folio,
        page: *const page,
        to_kill: *mut list_head,
        force_early: c_int,
    );
    pub fn ksm_process_profit(mm: *mut mm_struct) -> c_long;
    pub fn ksm_process_mergeable(mm: *mut mm_struct) -> bool;
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn is_ksm_zero_pte(pte: pte_t) -> bool {
    is_zero_pfn(pte_pfn(pte)) && pte_dirty(pte)
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn ksm_map_zero_page(mm: *mut mm_struct) {
    atomic_long_inc(&raw mut ksm_zero_pages);
    atomic_long_inc(&raw mut (*mm).ksm_zero_pages);
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn ksm_might_unmap_zero_page(mm: *mut mm_struct, pte: pte_t) {
    if is_ksm_zero_pte(pte) {
        atomic_long_dec(&raw mut ksm_zero_pages);
        atomic_long_dec(&raw mut (*mm).ksm_zero_pages);
    }
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn mm_ksm_zero_pages(mm: *mut mm_struct) -> c_long {
    atomic_long_read(&raw const (*mm).ksm_zero_pages)
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn ksm_fork(mm: *mut mm_struct, oldmm: *mut mm_struct) {
    /* Adding mm to ksm is best effort on fork. */
    if mm_flags_test(MMF_VM_MERGEABLE, oldmm) {
        let nr_ksm_zero_pages = atomic_long_read(&raw const (*mm).ksm_zero_pages);
        (*mm).ksm_merging_pages = 0;
        (*mm).ksm_rmap_items = 0;
        atomic_long_add(nr_ksm_zero_pages, &raw mut ksm_zero_pages);
        __ksm_enter(mm);
    }
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn ksm_execve(mm: *mut mm_struct) -> c_int {
    if mm_flags_test(MMF_VM_MERGE_ANY, mm) { __ksm_enter(mm) } else { 0 }
}

#[cfg(CONFIG_KSM)]
#[inline]
pub unsafe fn ksm_exit(mm: *mut mm_struct) {
    if mm_flags_test(MMF_VM_MERGEABLE, mm) { __ksm_exit(mm); }
}

#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_vma_flags(_mm: *mut mm_struct, _file: *const file, vma_flags: vma_flags_t) -> vma_flags_t { vma_flags }
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_disable(_mm: *mut mm_struct) -> c_int { 0 }
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_fork(_mm: *mut mm_struct, _oldmm: *mut mm_struct) {}
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_execve(_mm: *mut mm_struct) -> c_int { 0 }
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_exit(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn ksm_might_unmap_zero_page(_mm: *mut mm_struct, _pte: pte_t) {}
#[cfg(not(CONFIG_KSM))]
#[inline]
pub unsafe fn collect_procs_ksm(_folio: *const folio, _page: *const page, _to_kill: *mut list_head, _force_early: c_int) {}

#[cfg(all(not(CONFIG_KSM), CONFIG_MMU))]
#[inline]
pub unsafe fn ksm_madvise(_vma: *mut vm_area_struct, _start: c_ulong, _end: c_ulong, _advice: c_int, _vm_flags: *mut vm_flags_t) -> c_int { 0 }
#[cfg(all(not(CONFIG_KSM), CONFIG_MMU))]
#[inline]
pub unsafe fn ksm_might_need_to_copy(folio: *mut folio, _vma: *mut vm_area_struct, _addr: c_ulong) -> *mut folio { folio }
#[cfg(all(not(CONFIG_KSM), CONFIG_MMU))]
#[inline]
pub unsafe fn rmap_walk_ksm(_folio: *mut folio, _rwc: *mut rmap_walk_control) {}
#[cfg(all(not(CONFIG_KSM), CONFIG_MMU))]
#[inline]
pub unsafe fn folio_migrate_ksm(_newfolio: *mut folio, _old: *mut folio) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
