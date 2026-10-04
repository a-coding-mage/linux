// SPDX-License-Identifier: GPL-2.0
// Rust owner of mm/mremap.c, source 0db90fa02d8bc839349c44c13904a548f7dd062a.
// The syscall wrapper and named native/header boundaries are inventoried separately.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/mremap_native_generated.rs"
    ));
}
use b::*;
use core::mem::zeroed;
use core::ptr::{addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_ulong, c_void};
const PAGE_SIZE: c_ulong = RUST_MREMAP_PAGE_SIZE as c_ulong;
const PAGE_SHIFT: u32 = RUST_MREMAP_PAGE_SHIFT as u32;
// The native binding header verifies each mask against its native size.
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const PMD_SIZE: c_ulong = RUST_MREMAP_PMD_SIZE as c_ulong;
const PMD_MASK: c_ulong = !(PMD_SIZE - 1);
const PUD_SIZE: c_ulong = RUST_MREMAP_PUD_SIZE as c_ulong;
const PUD_MASK: c_ulong = !(PUD_SIZE - 1);
#[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
const HPAGE_PMD_SIZE: c_ulong = RUST_MREMAP_HPAGE_PMD_SIZE as c_ulong;
#[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
const HPAGE_PUD_SIZE: c_ulong = RUST_MREMAP_HPAGE_PUD_SIZE as c_ulong;
#[inline]
fn err(e: u32) -> c_ulong {
    (-(e as c_long)) as c_ulong
}
#[inline]
fn ierr(e: u32) -> c_int {
    -(e as c_int)
}
#[inline]
fn align(n: c_ulong, a: c_ulong) -> c_ulong {
    n.wrapping_add(a - 1) & !(a - 1)
}
#[inline]
unsafe fn mm() -> *mut mm_struct {
    rust_mremap_current_mm()
}
#[inline]
unsafe fn start(v: *mut vm_area_struct) -> c_ulong {
    *rust_mremap_vma_vm_start(v)
}
#[inline]
unsafe fn end(v: *mut vm_area_struct) -> c_ulong {
    *rust_mremap_vma_vm_end(v)
}
#[inline]
unsafe fn flags(v: *mut vm_area_struct) -> vm_flags_t {
    *rust_mremap_vma_vm_flags(v)
}
#[inline]
unsafe fn test(v: *mut vm_area_struct, bit: vma_flag_t) -> bool {
    rust_mremap_vma_test(v, bit)
}
#[inline]
unsafe fn is_err(n: c_ulong) -> bool {
    rust_mremap_is_err_value(n)
}
unsafe fn iterator(mm: *mut mm_struct, a: c_ulong) -> vma_iterator {
    // Exact VMA_ITERATOR initializer, not vma_iter_init (last/max differ).
    let mut i: vma_iterator = zeroed();
    i.mas.tree = rust_mremap_mm_mm_mt(mm);
    i.mas.index = a;
    i.mas.status = ma_start;
    i
}
unsafe fn pmc_init(
    old: *mut vm_area_struct,
    new: *mut vm_area_struct,
    old_addr: c_ulong,
    new_addr: c_ulong,
    len: c_ulong,
) -> pagetable_move_control {
    let mut p: pagetable_move_control = zeroed();
    p.old = old;
    p.new = new;
    p.old_addr = old_addr;
    p.old_end = old_addr.wrapping_add(len);
    p.new_addr = new_addr;
    p.len_in = len;
    p
}
unsafe fn get_old_pud(mm: *mut mm_struct, addr: c_ulong) -> *mut pud_t {
    let pgd = rust_mremap_pgd_offset(mm, addr);
    if rust_mremap_pgd_none_or_clear_bad(pgd) {
        return null_mut();
    }
    let p4d = rust_mremap_p4d_offset(pgd, addr);
    if rust_mremap_p4d_none_or_clear_bad(p4d) {
        return null_mut();
    }
    let pud = rust_mremap_pud_offset(p4d, addr);
    if rust_mremap_pud_none_or_clear_bad(pud) {
        return null_mut();
    }
    pud
}
unsafe fn get_old_pmd(mm: *mut mm_struct, addr: c_ulong) -> *mut pmd_t {
    let pud = get_old_pud(mm, addr);
    if pud.is_null() {
        return null_mut();
    }
    let pmd = rust_mremap_pmd_offset(pud, addr);
    if rust_mremap_pmd_none(*pmd) {
        return null_mut();
    }
    pmd
}
unsafe fn alloc_new_pud(mm: *mut mm_struct, addr: c_ulong) -> *mut pud_t {
    let pgd = rust_mremap_pgd_offset(mm, addr);
    let p4d = rust_mremap_p4d_alloc(mm, pgd, addr);
    if p4d.is_null() {
        return null_mut();
    }
    rust_mremap_pud_alloc(mm, p4d, addr)
}
unsafe fn alloc_new_pmd(mm: *mut mm_struct, addr: c_ulong) -> *mut pmd_t {
    let pud = alloc_new_pud(mm, addr);
    if pud.is_null() {
        return null_mut();
    }
    let pmd = rust_mremap_pmd_alloc(mm, pud, addr);
    if pmd.is_null() {
        return null_mut();
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_mremap_bug_new_pmd_huge(rust_mremap_pmd_trans_huge(*pmd));
    pmd
}
unsafe fn take_rmap_locks(vma: *mut vm_area_struct) {
    if !(*vma).vm_file.is_null() {
        rust_mremap_i_mmap_lock_write((*(*vma).vm_file).f_mapping);
    }
    if !(*vma).anon_vma.is_null() {
        rust_mremap_anon_vma_lock_write((*vma).anon_vma);
    }
}
unsafe fn drop_rmap_locks(vma: *mut vm_area_struct) {
    if !(*vma).anon_vma.is_null() {
        rust_mremap_anon_vma_unlock_write((*vma).anon_vma);
    }
    if !(*vma).vm_file.is_null() {
        rust_mremap_i_mmap_unlock_write((*(*vma).vm_file).f_mapping);
    }
}
unsafe fn move_soft_dirty_pte(mut pte: pte_t) -> pte_t {
    if rust_mremap_pte_none(pte) {
        return pte;
    }
    if rust_mremap_pgtable_supports_soft_dirty() {
        pte = if rust_mremap_pte_present(pte) {
            rust_mremap_pte_mksoft_dirty(pte)
        } else {
            rust_mremap_pte_swp_mksoft_dirty(pte)
        };
    }
    pte
}
unsafe fn mremap_folio_pte_batch(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    mut pte: pte_t,
    max_nr: c_int,
) -> c_int {
    if max_nr == 1 {
        return 1;
    }
    if rust_mremap_pte_batch_hint(ptep, pte) == 1 {
        return 1;
    }
    let folio = vm_normal_folio(vma, addr, pte);
    if folio.is_null() || !rust_mremap_folio_test_large(folio) {
        return 1;
    }
    rust_mremap_folio_pte_batch_flags(
        folio,
        null_mut(),
        ptep,
        &mut pte,
        max_nr as u32,
        RUST_MREMAP_FPB_RESPECT_WRITE as fpb_t,
    ) as c_int
}
unsafe fn move_ptes(
    pmc: *mut pagetable_move_control,
    extent: c_ulong,
    old_pmd: *mut pmd_t,
    new_pmd: *mut pmd_t,
) -> c_int {
    let vma = (*pmc).old;
    let need_clear = rust_mremap_vma_has_uffd_without_event_remap(vma);
    let mm = (*vma).vm_mm;
    let mut old_addr = (*pmc).old_addr;
    let mut new_addr = (*pmc).new_addr;
    let old_end = old_addr.wrapping_add(extent);
    let len = old_end.wrapping_sub(old_addr);
    let mut old_ptl = null_mut();
    let mut new_ptl = null_mut();
    let mut dummy: pmd_t = zeroed();
    let mut force_flush = false;
    let mut result = 0;
    if (*pmc).need_rmap_locks {
        take_rmap_locks(vma);
    }
    'move_locked: {
        let mut old_ptep = rust_mremap_pte_offset_map_lock(mm, old_pmd, old_addr, &mut old_ptl);
        if old_ptep.is_null() {
            result = ierr(EAGAIN);
            break 'move_locked;
        }
        let mut new_ptep =
            rust_mremap_pte_offset_map_rw_nolock(mm, new_pmd, new_addr, &mut dummy, &mut new_ptl);
        if new_ptep.is_null() {
            rust_mremap_pte_unmap_unlock(old_ptep, old_ptl);
            result = ierr(EAGAIN);
            break 'move_locked;
        }
        if new_ptl != old_ptl {
            rust_mremap_spin_lock_nested(new_ptl);
        }
        rust_mremap_flush_tlb_batched_pending(mm);
        rust_mremap_lazy_mmu_mode_enable();
        while old_addr < old_end {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_mremap_warn_destination_pte(!rust_mremap_pte_none(rust_mremap_ptep_get(new_ptep)));
            let mut nr = 1;
            let max = (old_end - old_addr) >> PAGE_SHIFT;
            let old_pte = rust_mremap_ptep_get(old_ptep);
            if !rust_mremap_pte_none(old_pte) {
                if rust_mremap_pte_present(old_pte) {
                    nr = mremap_folio_pte_batch(vma, old_addr, old_ptep, old_pte, max as c_int);
                    force_flush = true;
                }
                let mut pte = rust_mremap_get_and_clear_ptes(mm, old_addr, old_ptep, nr);
                pte = rust_mremap_move_pte(pte, old_addr, new_addr);
                pte = move_soft_dirty_pte(pte);
                if need_clear && rust_mremap_pte_is_uffd_wp_marker(pte) {
                    rust_mremap_pte_clear(mm, new_addr, new_ptep);
                } else {
                    if need_clear {
                        if rust_mremap_pte_present(pte) {
                            if rust_mremap_userfaultfd_rwp(vma) && rust_mremap_pte_uffd(pte) {
                                pte = rust_mremap_pte_modify(pte, (*vma).vm_page_prot);
                            }
                            pte = rust_mremap_pte_clear_uffd(pte);
                        } else {
                            pte = rust_mremap_pte_swp_clear_uffd(pte);
                        }
                    }
                    rust_mremap_set_ptes(mm, new_addr, new_ptep, pte, nr);
                }
            }
            let bytes = (nr as c_ulong).wrapping_mul(PAGE_SIZE);
            old_addr = old_addr.wrapping_add(bytes);
            new_addr = new_addr.wrapping_add(bytes);
            old_ptep = old_ptep.add(nr as usize);
            new_ptep = new_ptep.add(nr as usize);
        }
        rust_mremap_lazy_mmu_mode_disable();
        if force_flush {
            rust_mremap_flush_tlb_range(vma, old_end.wrapping_sub(len), old_end);
        }
        if new_ptl != old_ptl {
            rust_mremap_spin_unlock(new_ptl);
        }
        rust_mremap_pte_unmap(new_ptep.sub(1));
        rust_mremap_pte_unmap_unlock(old_ptep.sub(1), old_ptl);
    }
    if (*pmc).need_rmap_locks {
        drop_rmap_locks(vma);
    }
    result
}
unsafe fn arch_supports_page_table_move() -> bool {
    #[cfg(RUST_MREMAP_ARCH_SUPPORTS_PAGE_TABLE_MOVE)]
    {
        return rust_mremap_arch_supports_page_table_move();
    }
    #[cfg(not(RUST_MREMAP_ARCH_SUPPORTS_PAGE_TABLE_MOVE))]
    {
        cfg!(CONFIG_HAVE_MOVE_PMD) || cfg!(CONFIG_HAVE_MOVE_PUD)
    }
}
unsafe fn uffd_supports_page_table_move(pmc: *mut pagetable_move_control) -> bool {
    !rust_mremap_vma_has_uffd_without_event_remap((*pmc).old)
        && !rust_mremap_vma_has_uffd_without_event_remap((*pmc).new)
}
#[cfg(CONFIG_HAVE_MOVE_PMD)]
unsafe fn move_normal_pmd(
    pmc: *mut pagetable_move_control,
    old: *mut pmd_t,
    new: *mut pmd_t,
) -> bool {
    if !arch_supports_page_table_move() || !uffd_supports_page_table_move(pmc) {
        return false;
    }
    if rust_mremap_warn_destination_pmd(!rust_mremap_pmd_none(*new)) {
        return false;
    }
    let vma = (*pmc).old;
    let mm = (*vma).vm_mm;
    let old_ptl = rust_mremap_pmd_lock(mm, old);
    let new_ptl = rust_mremap_pmd_lockptr(mm, new);
    if new_ptl != old_ptl {
        rust_mremap_spin_lock_nested(new_ptl);
    }
    let pmd = *old;
    let mut result = false;
    if rust_mremap_pmd_present(pmd) && !rust_mremap_pmd_leaf(pmd) {
        rust_mremap_pmd_clear(old);
        result = true;
        #[cfg(CONFIG_DEBUG_VM)]
        rust_mremap_bug_destination_pmd(!rust_mremap_pmd_none(*new));
        rust_mremap_pmd_populate(mm, new, rust_mremap_pmd_pgtable(pmd));
        rust_mremap_flush_tlb_range(vma, (*pmc).old_addr, (*pmc).old_addr.wrapping_add(PMD_SIZE));
    }
    if new_ptl != old_ptl {
        rust_mremap_spin_unlock(new_ptl);
    }
    rust_mremap_spin_unlock(old_ptl);
    result
}
#[cfg(not(CONFIG_HAVE_MOVE_PMD))]
unsafe fn move_normal_pmd(_: *mut pagetable_move_control, _: *mut pmd_t, _: *mut pmd_t) -> bool {
    false
}
#[cfg(RUST_MREMAP_MOVE_NORMAL_PUD)]
unsafe fn move_normal_pud(
    pmc: *mut pagetable_move_control,
    old: *mut pud_t,
    new: *mut pud_t,
) -> bool {
    if !arch_supports_page_table_move() || !uffd_supports_page_table_move(pmc) {
        return false;
    }
    if rust_mremap_warn_destination_pud(!rust_mremap_pud_none(*new)) {
        return false;
    }
    let vma = (*pmc).old;
    let mm = (*vma).vm_mm;
    let old_ptl = rust_mremap_pud_lock(mm, old);
    let new_ptl = rust_mremap_pud_lockptr(mm, new);
    if new_ptl != old_ptl {
        rust_mremap_spin_lock_nested(new_ptl);
    }
    let pud = *old;
    rust_mremap_pud_clear(old);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_mremap_bug_destination_pud(!rust_mremap_pud_none(*new));
    rust_mremap_pud_populate(mm, new, rust_mremap_pud_pgtable(pud));
    rust_mremap_flush_tlb_range(vma, (*pmc).old_addr, (*pmc).old_addr.wrapping_add(PUD_SIZE));
    if new_ptl != old_ptl {
        rust_mremap_spin_unlock(new_ptl);
    }
    rust_mremap_spin_unlock(old_ptl);
    true
}
#[cfg(not(RUST_MREMAP_MOVE_NORMAL_PUD))]
unsafe fn move_normal_pud(_: *mut pagetable_move_control, _: *mut pud_t, _: *mut pud_t) -> bool {
    false
}
#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
unsafe fn move_huge_pud(
    pmc: *mut pagetable_move_control,
    old: *mut pud_t,
    new: *mut pud_t,
) -> bool {
    if rust_mremap_warn_destination_huge_pud(!rust_mremap_pud_none(*new)) {
        return false;
    }
    let vma = (*pmc).old;
    let mm = (*vma).vm_mm;
    let old_ptl = rust_mremap_pud_lock(mm, old);
    let new_ptl = rust_mremap_pud_lockptr(mm, new);
    if new_ptl != old_ptl {
        rust_mremap_spin_lock_nested(new_ptl);
    }
    let pud = *old;
    rust_mremap_pud_clear(old);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_mremap_bug_destination_huge_pud(!rust_mremap_pud_none(*new));
    rust_mremap_set_pud_at(mm, (*pmc).new_addr, new, pud);
    rust_mremap_flush_pud_tlb_range(
        vma,
        (*pmc).old_addr,
        (*pmc).old_addr.wrapping_add(HPAGE_PUD_SIZE),
    );
    if new_ptl != old_ptl {
        rust_mremap_spin_unlock(new_ptl);
    }
    rust_mremap_spin_unlock(old_ptl);
    true
}
#[cfg(not(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)))]
unsafe fn move_huge_pud(_: *mut pagetable_move_control, _: *mut pud_t, _: *mut pud_t) -> bool {
    rust_mremap_warn_huge_pud_unsupported();
    false
}
unsafe fn get_extent(entry: pgt_entry, pmc: *mut pagetable_move_control) -> c_ulong {
    let (mask, size) = match entry {
        NORMAL_PMD | HPAGE_PMD => (PMD_MASK, PMD_SIZE),
        NORMAL_PUD | HPAGE_PUD => (PUD_MASK, PUD_SIZE),
        _ => unreachable!(), // C BUILD_BUG: all callers use one of the four enum values.
    };
    let old = (*pmc).old_addr;
    let new = (*pmc).new_addr;
    let next = old.wrapping_add(size) & mask;
    let mut extent = next.wrapping_sub(old);
    extent = extent.min((*pmc).old_end.wrapping_sub(old));
    let next = new.wrapping_add(size) & mask;
    extent.min(next.wrapping_sub(new))
}
unsafe fn should_take_rmap_locks(pmc: *mut pagetable_move_control, entry: pgt_entry) -> bool {
    match entry {
        NORMAL_PMD | NORMAL_PUD => true,
        _ => (*pmc).need_rmap_locks,
    }
}
unsafe fn move_pgt_entry(
    pmc: *mut pagetable_move_control,
    entry: pgt_entry,
    old: *mut c_void,
    new: *mut c_void,
) -> bool {
    let need_locks = should_take_rmap_locks(pmc, entry);
    if need_locks {
        take_rmap_locks((*pmc).old);
    }
    let moved = match entry {
        NORMAL_PMD => move_normal_pmd(pmc, old.cast(), new.cast()),
        NORMAL_PUD => move_normal_pud(pmc, old.cast(), new.cast()),
        HPAGE_PMD => {
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            {
                rust_mremap_move_huge_pmd(
                    (*pmc).old,
                    (*pmc).old_addr,
                    (*pmc).new_addr,
                    old.cast(),
                    new.cast(),
                )
            }
            #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
            {
                false
            }
        }
        HPAGE_PUD => {
            cfg!(CONFIG_TRANSPARENT_HUGEPAGE) && move_huge_pud(pmc, old.cast(), new.cast())
        }
        _ => {
            rust_mremap_warn_invalid_pgt_entry();
            false
        }
    };
    if need_locks {
        drop_rmap_locks((*pmc).old);
    }
    moved
}
unsafe fn can_align_down(
    pmc: *mut pagetable_move_control,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    mask: c_ulong,
) -> bool {
    let masked = addr & mask;
    if !(*pmc).for_stack && start(vma) != addr {
        return false;
    }
    if (*pmc).for_stack && masked >= start(vma) {
        return true;
    }
    find_vma_intersection((*vma).vm_mm, masked, start(vma)).is_null()
}
unsafe fn can_realign_addr(pmc: *mut pagetable_move_control, mask: c_ulong) -> bool {
    let align_mask = !mask;
    let old_align = (*pmc).old_addr & align_mask;
    let new_align = (*pmc).new_addr & align_mask;
    let size = align_mask.wrapping_add(1);
    if (*pmc).len_in < size.wrapping_sub(old_align) {
        return false;
    }
    if old_align == 0 || old_align != new_align {
        return false;
    }
    can_align_down(pmc, (*pmc).old, (*pmc).old_addr, mask)
        && can_align_down(pmc, (*pmc).new, (*pmc).new_addr, mask)
}
unsafe fn try_realign_addr(pmc: *mut pagetable_move_control, mask: c_ulong) {
    if !can_realign_addr(pmc, mask) {
        return;
    }
    (*pmc).old_addr &= mask;
    (*pmc).new_addr &= mask;
}
unsafe fn pmc_done(pmc: *mut pagetable_move_control) -> bool {
    (*pmc).old_addr >= (*pmc).old_end
}
unsafe fn pmc_next(pmc: *mut pagetable_move_control, extent: c_ulong) {
    (*pmc).old_addr = (*pmc).old_addr.wrapping_add(extent);
    (*pmc).new_addr = (*pmc).new_addr.wrapping_add(extent);
}
unsafe fn pmc_progress(pmc: *mut pagetable_move_control) -> c_ulong {
    let original = (*pmc).old_end.wrapping_sub((*pmc).len_in);
    if (*pmc).old_addr < original {
        0
    } else {
        (*pmc).old_addr - original
    }
}
#[no_mangle]
pub unsafe extern "C" fn move_page_tables(pmc: *mut pagetable_move_control) -> c_ulong {
    let mm = (*(*pmc).old).vm_mm;
    if (*pmc).len_in == 0 {
        return 0;
    }
    if rust_mremap_is_vm_hugetlb_page((*pmc).old) {
        return rust_mremap_move_hugetlb_page_tables(
            (*pmc).old,
            (*pmc).new,
            (*pmc).old_addr,
            (*pmc).new_addr,
            (*pmc).len_in,
        );
    }
    try_realign_addr(pmc, PMD_MASK);
    rust_mremap_flush_cache_range((*pmc).old, (*pmc).old_addr, (*pmc).old_end);
    let mut range: mmu_notifier_range = zeroed();
    rust_mremap_mmu_notifier_range_init(&mut range, mm, (*pmc).old_addr, (*pmc).old_end);
    rust_mremap_mmu_notifier_invalidate_range_start(&mut range);
    'walk: while !pmc_done(pmc) {
        rust_mremap_cond_resched();
        let mut extent = get_extent(NORMAL_PUD, pmc);
        let old_pud = get_old_pud(mm, (*pmc).old_addr);
        if old_pud.is_null() {
            pmc_next(pmc, extent);
            continue;
        }
        let new_pud = alloc_new_pud(mm, (*pmc).new_addr);
        if new_pud.is_null() {
            break;
        }
        if rust_mremap_pud_trans_huge(*old_pud) {
            #[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
            if extent == HPAGE_PUD_SIZE {
                // Original semantics intentionally ignore the huge-PUD move result.
                move_pgt_entry(pmc, HPAGE_PUD, old_pud.cast(), new_pud.cast());
                pmc_next(pmc, extent);
                continue;
            }
        } else if cfg!(CONFIG_HAVE_MOVE_PUD)
            && extent == PUD_SIZE
            && move_pgt_entry(pmc, NORMAL_PUD, old_pud.cast(), new_pud.cast())
        {
            pmc_next(pmc, extent);
            continue;
        }
        extent = get_extent(NORMAL_PMD, pmc);
        let old_pmd = get_old_pmd(mm, (*pmc).old_addr);
        if old_pmd.is_null() {
            pmc_next(pmc, extent);
            continue;
        }
        let new_pmd = alloc_new_pmd(mm, (*pmc).new_addr);
        if new_pmd.is_null() {
            break;
        }
        loop {
            if rust_mremap_pmd_is_huge(*old_pmd) {
                #[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
                if extent == HPAGE_PMD_SIZE
                    && move_pgt_entry(pmc, HPAGE_PMD, old_pmd.cast(), new_pmd.cast())
                {
                    break;
                }
                rust_mremap_split_huge_pmd((*pmc).old, old_pmd, (*pmc).old_addr);
            } else if cfg!(CONFIG_HAVE_MOVE_PMD)
                && extent == PMD_SIZE
                && move_pgt_entry(pmc, NORMAL_PMD, old_pmd.cast(), new_pmd.cast())
            {
                break;
            }
            if rust_mremap_pmd_none(*old_pmd) {
                break;
            }
            if rust_mremap_pte_alloc((*(*pmc).new).vm_mm, new_pmd) != 0 {
                break 'walk;
            }
            if move_ptes(pmc, extent, old_pmd, new_pmd) >= 0 {
                break;
            }
            // A failed PTE map retries this PMD, without advancing the PMC.
        }
        pmc_next(pmc, extent);
    }
    rust_mremap_mmu_notifier_invalidate_range_end(&mut range);
    pmc_progress(pmc)
}
unsafe fn vrm_set_delta(v: *mut vma_remap_struct) {
    (*v).delta = (*v).old_len.abs_diff((*v).new_len);
}
unsafe fn vrm_remap_type(v: *mut vma_remap_struct) -> mremap_type {
    if (*v).delta == 0 {
        MREMAP_NO_RESIZE
    } else if (*v).old_len > (*v).new_len {
        MREMAP_SHRINK
    } else {
        MREMAP_EXPAND
    }
}
unsafe fn vrm_overlaps(v: *mut vma_remap_struct) -> bool {
    (*v).addr.wrapping_add((*v).old_len) > (*v).new_addr
        && (*v).new_addr.wrapping_add((*v).new_len) > (*v).addr
}
unsafe fn vrm_implies_new_addr(v: *mut vma_remap_struct) -> bool {
    (*v).flags & ((MREMAP_FIXED | MREMAP_DONTUNMAP) as c_ulong) != 0
}
unsafe fn vrm_set_new_addr(v: *mut vma_remap_struct) -> c_ulong {
    let vma = (*v).vma;
    let pgoff = rust_mremap_linear_page_index(vma, (*v).addr);
    let mut map_flags: c_ulong = 0;
    let new_addr = if vrm_implies_new_addr(v) {
        (*v).new_addr
    } else {
        0
    };
    if (*v).flags & MREMAP_FIXED as c_ulong != 0 {
        map_flags |= MAP_FIXED as c_ulong;
    }
    if test(vma, VMA_MAYSHARE_BIT as vma_flag_t) {
        map_flags |= MAP_SHARED as c_ulong;
    }
    let res =
        rust_mremap_get_unmapped_area((*vma).vm_file, new_addr, (*v).new_len, pgoff, map_flags);
    if is_err(res) {
        return res;
    }
    (*v).new_addr = res;
    0
}
unsafe fn vrm_calc_charge(v: *mut vma_remap_struct) -> bool {
    if !test((*v).vma, VMA_ACCOUNT_BIT as vma_flag_t) {
        return true;
    }
    let charged = if (*v).flags & MREMAP_DONTUNMAP as c_ulong != 0 {
        (*v).new_len
    } else {
        (*v).delta
    } >> PAGE_SHIFT;
    if rust_mremap_security_vm_enough_memory_mm(mm(), charged as c_long) != 0 {
        return false;
    }
    (*v).charged = charged;
    true
}
unsafe fn vrm_uncharge(v: *mut vma_remap_struct) {
    if !test((*v).vma, VMA_ACCOUNT_BIT as vma_flag_t) {
        return;
    }
    rust_mremap_vm_unacct_memory((*v).charged as c_long);
    (*v).charged = 0;
}
unsafe fn vrm_stat_account(v: *mut vma_remap_struct, bytes: c_ulong) {
    let pages = bytes >> PAGE_SHIFT;
    let mm = mm();
    let vma = (*v).vma;
    vm_stat_account(mm, flags(vma), pages as c_long);
    if test(vma, VMA_LOCKED_BIT as vma_flag_t) {
        let locked = rust_mremap_mm_locked_vm(mm);
        *locked = (*locked).wrapping_add(pages);
    }
}
unsafe fn __check_map_count_against_split(mm: *mut mm_struct, before_unmaps: bool) -> bool {
    let limit = rust_mremap_get_sysctl_max_map_count();
    let mut count = *rust_mremap_mm_map_count(mm);
    rust_mremap_mmap_assert_write_locked(mm);
    if before_unmaps {
        count = count.wrapping_add(2);
    }
    count.wrapping_add(2) <= limit
}
unsafe fn check_map_count_against_split() -> bool {
    __check_map_count_against_split(mm(), false)
}
unsafe fn check_map_count_against_split_early() -> bool {
    __check_map_count_against_split(mm(), true)
}
unsafe fn prep_move_vma(v: *mut vma_remap_struct) -> c_ulong {
    let vma = (*v).vma;
    let old = (*v).addr;
    let len = (*v).old_len;
    let mut dummy = flags(vma);
    if !check_map_count_against_split() {
        return err(ENOMEM);
    }
    if !(*vma).vm_ops.is_null() {
        if let Some(may_split) = (*(*vma).vm_ops).may_split {
            let mut res = 0;
            if start(vma) != old {
                res = may_split(vma, old);
            }
            if res == 0 && end(vma) != old.wrapping_add(len) {
                res = may_split(vma, old.wrapping_add(len));
            }
            if res != 0 {
                return res as c_long as c_ulong;
            }
        }
    }
    let res = rust_mremap_ksm_madvise(
        vma,
        old,
        old.wrapping_add(len),
        MADV_UNMERGEABLE as c_int,
        &mut dummy,
    );
    if res != 0 {
        return res as c_long as c_ulong;
    }
    0
}
unsafe fn unmap_source_vma(v: *mut vma_remap_struct) {
    let mm = mm();
    let addr = (*v).addr;
    let len = (*v).old_len;
    let vma = (*v).vma;
    let mut vmi = iterator(mm, addr);
    let accountable =
        test(vma, VMA_ACCOUNT_BIT as vma_flag_t) && (*v).flags & MREMAP_DONTUNMAP as c_ulong == 0;
    let mut vm_start = 0;
    let mut vm_end = 0;
    if accountable {
        rust_mremap_vma_clear_flag(vma, VMA_ACCOUNT_BIT as vma_flag_t);
        vm_start = start(vma);
        vm_end = end(vma);
    }
    let res = do_vmi_munmap(&mut vmi, mm, addr, len, (*v).uf_unmap, false);
    (*v).vma = null_mut();
    (*v).vmi_needs_invalidate = true;
    if res != 0 {
        rust_mremap_vm_acct_memory((len >> PAGE_SHIFT) as c_long);
        return;
    }
    if accountable {
        let end = addr.wrapping_add(len);
        if vm_start < addr {
            let prev = rust_mremap_vma_prev(&mut vmi);
            rust_mremap_vma_start_write(prev);
            rust_mremap_vma_set_flag(prev, VMA_ACCOUNT_BIT as vma_flag_t);
        }
        if vm_end > end {
            let next = rust_mremap_vma_next(&mut vmi);
            rust_mremap_vma_start_write(next);
            rust_mremap_vma_set_flag(next, VMA_ACCOUNT_BIT as vma_flag_t);
        }
    }
}
unsafe fn copy_vma_and_data(v: *mut vma_remap_struct, out: *mut *mut vm_area_struct) -> c_int {
    let pgoff = rust_mremap_linear_page_index((*v).vma, (*v).addr);
    let anon_pgoff = rust_mremap_linear_anon_page_index((*v).vma, (*v).addr);
    let mut vma = (*v).vma;
    let mut pmc = pmc_init(
        null_mut(),
        null_mut(),
        (*v).addr,
        (*v).new_addr,
        (*v).old_len,
    );
    let new = copy_vma(
        &mut vma,
        (*v).new_addr,
        (*v).new_len,
        pgoff,
        anon_pgoff,
        &mut pmc.need_rmap_locks,
    );
    if new.is_null() {
        vrm_uncharge(v);
        *out = null_mut();
        return ierr(ENOMEM);
    }
    if vma != (*v).vma {
        (*v).vmi_needs_invalidate = true;
    }
    (*v).vma = vma;
    pmc.old = vma;
    pmc.new = new;
    let moved = move_page_tables(&mut pmc);
    let mut res = 0;
    if moved < (*v).old_len {
        res = ierr(ENOMEM);
    } else if !(*vma).vm_ops.is_null() {
        if let Some(remap) = (*(*vma).vm_ops).mremap {
            res = remap(new);
        }
    }
    if res != 0 {
        let mut revert = pmc_init(new, vma, (*v).new_addr, (*v).addr, moved);
        revert.need_rmap_locks = true;
        move_page_tables(&mut revert);
        (*v).vma = new;
        (*v).old_len = (*v).new_len;
        (*v).addr = (*v).new_addr;
    } else {
        rust_mremap_mremap_userfaultfd_prep(new, (*v).uf);
    }
    rust_mremap_fixup_hugetlb_reservations(vma);
    *out = new;
    res
}
unsafe fn dontunmap_complete(v: *mut vma_remap_struct, new: *mut vm_area_struct) {
    let old_start = start((*v).vma);
    let old_end = end((*v).vma);
    rust_mremap_vma_clear_locked((*v).vma);
    if new != (*v).vma && (*v).addr == old_start && (*v).addr.wrapping_add((*v).old_len) == old_end
    {
        rust_mremap_unlink_anon_vmas((*v).vma);
    }
}
unsafe fn move_vma(v: *mut vma_remap_struct) -> c_ulong {
    let mm = mm();
    let res = prep_move_vma(v);
    if res != 0 {
        return res;
    }
    if !vrm_calc_charge(v) {
        return err(ENOMEM);
    }
    rust_mremap_vma_start_write((*v).vma);
    let mut new = null_mut();
    let res = copy_vma_and_data(v, &mut new);
    if res != 0 && new.is_null() {
        return res as c_long as c_ulong;
    }
    let hiwater = *rust_mremap_mm_hiwater_vm(mm);
    vrm_stat_account(v, (*v).new_len);
    if res == 0 && (*v).flags & MREMAP_DONTUNMAP as c_ulong != 0 {
        dontunmap_complete(v, new);
    } else {
        unmap_source_vma(v);
    }
    *rust_mremap_mm_hiwater_vm(mm) = hiwater;
    if res != 0 {
        res as c_long as c_ulong
    } else {
        (*v).new_addr
    }
}
unsafe fn shrink_vma(v: *mut vma_remap_struct, drop_lock: bool) -> c_ulong {
    let mm = mm();
    let unmap_start = (*v).addr.wrapping_add((*v).new_len);
    let mut vmi = iterator(mm, unmap_start);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_mremap_bug_not_shrink((*v).remap_type != MREMAP_SHRINK);
    let res = do_vmi_munmap(
        &mut vmi,
        mm,
        unmap_start,
        (*v).delta,
        (*v).uf_unmap,
        drop_lock,
    );
    (*v).vma = null_mut();
    if res != 0 {
        return res as c_long as c_ulong;
    }
    if drop_lock {
        (*v).mmap_locked = false;
    } else {
        (*v).vma = rust_mremap_vma_lookup(mm, (*v).addr);
        if (*v).vma.is_null() {
            return err(EFAULT);
        }
    }
    0
}
unsafe fn mremap_to(v: *mut vma_remap_struct) -> c_ulong {
    let mm = mm();
    if (*v).flags & MREMAP_FIXED as c_ulong != 0 {
        let res = do_munmap(mm, (*v).new_addr, (*v).new_len, (*v).uf_unmap_early);
        (*v).vma = null_mut();
        (*v).vmi_needs_invalidate = true;
        if res != 0 {
            return res as c_long as c_ulong;
        }
        (*v).vma = rust_mremap_vma_lookup(mm, (*v).addr);
        if (*v).vma.is_null() {
            return err(EFAULT);
        }
    }
    if (*v).remap_type == MREMAP_SHRINK {
        let res = shrink_vma(v, false);
        if res != 0 {
            return res;
        }
        (*v).old_len = (*v).new_len;
    }
    if (*v).flags & MREMAP_DONTUNMAP as c_ulong != 0 {
        let vma_flags = *rust_mremap_vma_flags((*v).vma);
        if !may_expand_vm(mm, &vma_flags, (*v).old_len >> PAGE_SHIFT) {
            return err(ENOMEM);
        }
    }
    let res = vrm_set_new_addr(v);
    if res != 0 {
        return res;
    }
    move_vma(v)
}
unsafe fn vma_expandable(vma: *mut vm_area_struct, delta: c_ulong) -> c_int {
    let new_end = end(vma).wrapping_add(delta);
    if new_end < end(vma) {
        return 0;
    }
    if !find_vma_intersection((*vma).vm_mm, end(vma), new_end).is_null() {
        return 0;
    }
    if rust_mremap_get_unmapped_area(
        null_mut(),
        start(vma),
        new_end.wrapping_sub(start(vma)),
        0,
        MAP_FIXED as c_ulong,
    ) & !PAGE_MASK
        != 0
    {
        return 0;
    }
    1
}
unsafe fn vrm_can_expand_in_place(v: *mut vma_remap_struct) -> bool {
    if end((*v).vma).wrapping_sub((*v).addr) != (*v).old_len {
        return false;
    }
    vma_expandable((*v).vma, (*v).delta) != 0
}
unsafe fn expand_vma_in_place(v: *mut vma_remap_struct) -> c_ulong {
    let mut vmi = iterator(mm(), end((*v).vma));
    if !vrm_calc_charge(v) {
        return err(ENOMEM);
    }
    let vma = vma_merge_extend(&mut vmi, (*v).vma, (*v).delta);
    if vma.is_null() {
        vrm_uncharge(v);
        return err(ENOMEM);
    }
    (*v).vma = vma;
    vrm_stat_account(v, (*v).delta);
    0
}
unsafe fn align_hugetlb(v: *mut vma_remap_struct) -> bool {
    let h = rust_mremap_hstate_vma((*v).vma);
    (*v).old_len = align((*v).old_len, rust_mremap_huge_page_size(h));
    (*v).new_len = align((*v).new_len, rust_mremap_huge_page_size(h));
    if (*v).addr & !rust_mremap_huge_page_mask(h) != 0 {
        return false;
    }
    if (*v).new_addr & !rust_mremap_huge_page_mask(h) != 0 {
        return false;
    }
    (*v).new_len <= (*v).old_len
}
unsafe fn expand_vma(v: *mut vma_remap_struct) -> c_ulong {
    if vrm_can_expand_in_place(v) {
        let res = expand_vma_in_place(v);
        return if res != 0 { res } else { (*v).addr };
    }
    if (*v).flags & MREMAP_MAYMOVE as c_ulong == 0 {
        return err(ENOMEM);
    }
    let res = vrm_set_new_addr(v);
    if res != 0 {
        return res;
    }
    move_vma(v)
}
unsafe fn mremap_at(v: *mut vma_remap_struct) -> c_ulong {
    match (*v).remap_type {
        MREMAP_NO_RESIZE => return (*v).addr,
        MREMAP_SHRINK => {
            let res = shrink_vma(v, true);
            return if res != 0 { res } else { (*v).addr };
        }
        MREMAP_EXPAND => return expand_vma(v),
        _ => {}
    }
    rust_mremap_warn_invalid_remap_type();
    err(EINVAL)
}
unsafe fn vrm_will_map_new(v: *mut vma_remap_struct) -> bool {
    (*v).remap_type == MREMAP_EXPAND || vrm_implies_new_addr(v)
}
unsafe fn vrm_move_only(v: *mut vma_remap_struct) -> bool {
    (*v).flags & MREMAP_FIXED as c_ulong != 0 && (*v).old_len == (*v).new_len
}
unsafe fn notify_uffd(v: *mut vma_remap_struct, failed: bool) {
    rust_mremap_userfaultfd_unmap_complete(mm(), (*v).uf_unmap_early);
    if failed {
        rust_mremap_mremap_userfaultfd_fail((*v).uf);
    } else {
        rust_mremap_mremap_userfaultfd_complete((*v).uf, (*v).addr, (*v).new_addr, (*v).old_len);
    }
    rust_mremap_userfaultfd_unmap_complete(mm(), (*v).uf_unmap);
}
unsafe fn vma_multi_allowed(vma: *mut vm_area_struct) -> bool {
    let file = (*vma).vm_file;
    if rust_mremap_userfaultfd_armed(vma) {
        return false;
    }
    if file.is_null() || (*(*file).f_op).get_unmapped_area.is_none() {
        return true;
    }
    if rust_mremap_vma_is_shmem(vma) || rust_mremap_is_vm_hugetlb_page(vma) {
        return true;
    }
    rust_mremap_file_uses_thp_get_unmapped_area(file)
}
unsafe fn check_prep_vma(v: *mut vma_remap_struct) -> c_int {
    let vma = (*v).vma;
    let mm = mm();
    let addr = (*v).addr;
    if vma.is_null() {
        return ierr(EFAULT);
    }
    if rust_mremap_vma_is_sealed(vma) {
        return ierr(EPERM);
    }
    if rust_mremap_is_vm_hugetlb_page(vma) && !align_hugetlb(v) {
        return ierr(EINVAL);
    }
    vrm_set_delta(v);
    (*v).remap_type = vrm_remap_type(v);
    if !vrm_implies_new_addr(v) {
        (*v).new_addr = addr;
    }
    if !vrm_will_map_new(v) {
        return 0;
    }
    let mut old_len = (*v).old_len;
    let new_len = (*v).new_len;
    if old_len == 0
        && !test(vma, VMA_SHARED_BIT as vma_flag_t)
        && !test(vma, VMA_MAYSHARE_BIT as vma_flag_t)
    {
        rust_mremap_warn_private_duplication();
        return ierr(EINVAL);
    }
    if (*v).flags & MREMAP_DONTUNMAP as c_ulong != 0
        && (test(vma, VMA_DONTEXPAND_BIT as vma_flag_t) || test(vma, VMA_PFNMAP_BIT as vma_flag_t))
    {
        return ierr(EINVAL);
    }
    if (*v).remap_type == MREMAP_SHRINK {
        old_len = new_len;
    }
    if old_len > end(vma).wrapping_sub(addr) {
        return ierr(EFAULT);
    }
    if new_len == old_len {
        return 0;
    }
    if test(vma, VMA_LOCKED_BIT as vma_flag_t) {
        (*v).populate_expand = true;
    }
    let pgoff = rust_mremap_linear_page_index(vma, addr);
    if pgoff.wrapping_add(new_len >> PAGE_SHIFT) < pgoff {
        return ierr(EINVAL);
    }
    if test(vma, VMA_DONTEXPAND_BIT as vma_flag_t) || test(vma, VMA_PFNMAP_BIT as vma_flag_t) {
        return ierr(EFAULT);
    }
    if !mlock_future_ok(mm, test(vma, VMA_LOCKED_BIT as vma_flag_t), (*v).delta) {
        return ierr(EAGAIN);
    }
    if !may_expand_vm(mm, rust_mremap_vma_flags(vma), (*v).delta >> PAGE_SHIFT) {
        return ierr(ENOMEM);
    }
    0
}
unsafe fn check_mremap_params(v: *mut vma_remap_struct) -> c_ulong {
    let flags = (*v).flags;
    if flags & !((MREMAP_FIXED | MREMAP_MAYMOVE | MREMAP_DONTUNMAP) as c_ulong) != 0 {
        return err(EINVAL);
    }
    if (*v).addr & !PAGE_MASK != 0 {
        return err(EINVAL);
    }
    if (*v).new_len == 0 || (*v).new_len > rust_mremap_task_size() {
        return err(EINVAL);
    }
    if !vrm_implies_new_addr(v) {
        return 0;
    }
    if (*v).new_addr > rust_mremap_task_size().wrapping_sub((*v).new_len) {
        return err(EINVAL);
    }
    if (*v).new_addr & !PAGE_MASK != 0 {
        return err(EINVAL);
    }
    if flags & MREMAP_MAYMOVE as c_ulong == 0 {
        return err(EINVAL);
    }
    if flags & MREMAP_DONTUNMAP as c_ulong != 0 && (*v).old_len != (*v).new_len {
        return err(EINVAL);
    }
    if vrm_overlaps(v) {
        return err(EINVAL);
    }
    0
}
unsafe fn remap_move(v: *mut vma_remap_struct) -> c_ulong {
    let source_start = (*v).addr;
    let source_end = source_start.wrapping_add((*v).old_len);
    let new_addr = (*v).new_addr;
    let mut target_addr = new_addr;
    let mut res = err(EFAULT);
    let mut last_end = 0;
    let mut seen = false;
    let mut vmi = iterator(mm(), source_start);
    loop {
        let vma = rust_mremap_vma_find(&mut vmi, source_end);
        if vma.is_null() {
            break;
        }
        let addr = start(vma).max(source_start);
        let len = source_end.min(end(vma)).wrapping_sub(addr);
        if !seen && source_start < start(vma) {
            return err(EFAULT);
        }
        let offset = if seen {
            start(vma).wrapping_sub(last_end)
        } else {
            0
        };
        last_end = end(vma);
        (*v).vma = vma;
        (*v).addr = addr;
        (*v).new_addr = target_addr.wrapping_add(offset);
        (*v).old_len = len;
        (*v).new_len = len;
        let multi = vma_multi_allowed(vma);
        if !multi && (seen || end(vma) < source_end) {
            return err(EFAULT);
        }
        let mut res_vma = check_prep_vma(v) as c_long as c_ulong;
        if res_vma == 0 {
            res_vma = mremap_to(v);
        }
        if is_err(res_vma) {
            return res_vma;
        }
        if !seen {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_mremap_warn_multi_address(multi && res_vma != new_addr);
            res = res_vma;
        }
        #[cfg(CONFIG_DEBUG_VM)]
        rust_mremap_warn_move_unlocked(!(*v).mmap_locked);
        #[cfg(CONFIG_DEBUG_VM)]
        rust_mremap_warn_move_expanded((*v).populate_expand);
        if (*v).vmi_needs_invalidate {
            rust_mremap_vma_iter_invalidate(&mut vmi);
            (*v).vmi_needs_invalidate = false;
        }
        seen = true;
        target_addr = res_vma.wrapping_add((*v).new_len);
    }
    res
}
unsafe fn do_mremap(v: *mut vma_remap_struct) -> c_ulong {
    let mm = mm();
    (*v).old_len = align((*v).old_len, PAGE_SIZE);
    (*v).new_len = align((*v).new_len, PAGE_SIZE);
    let mut res = check_mremap_params(v);
    if res != 0 {
        return res;
    }
    if rust_mremap_mmap_write_lock_killable(mm) != 0 {
        return err(EINTR);
    }
    (*v).mmap_locked = true;
    if !check_map_count_against_split_early() {
        rust_mremap_mmap_write_unlock(mm);
        return err(ENOMEM);
    }
    if vrm_move_only(v) {
        res = remap_move(v);
    } else {
        (*v).vma = rust_mremap_vma_lookup(mm, (*v).addr);
        res = check_prep_vma(v) as c_long as c_ulong;
        if res == 0 {
            res = if vrm_implies_new_addr(v) {
                mremap_to(v)
            } else {
                mremap_at(v)
            };
        }
    }
    let failed = is_err(res);
    if (*v).mmap_locked {
        rust_mremap_mmap_write_unlock(mm);
    }
    if !failed && (*v).populate_expand {
        rust_mremap_mm_populate((*v).new_addr.wrapping_add((*v).old_len), (*v).delta);
    }
    notify_uffd(v, failed);
    res
}
#[no_mangle]
pub unsafe extern "C" fn rust_mremap_sys_mremap(
    addr: c_ulong,
    old_len: c_ulong,
    new_len: c_ulong,
    flags: c_ulong,
    new_addr: c_ulong,
) -> c_long {
    let mut uf = rust_mremap_null_vm_uffd_ctx();
    let mut early: list_head = zeroed();
    let mut unmap: list_head = zeroed();
    early.next = &mut early;
    early.prev = &mut early;
    unmap.next = &mut unmap;
    unmap.prev = &mut unmap;
    let mut v: vma_remap_struct = zeroed();
    // The old address loses its tag; the new address retains it for rejection.
    v.addr = rust_mremap_untagged_addr(addr);
    v.old_len = old_len;
    v.new_len = new_len;
    v.flags = flags;
    v.new_addr = new_addr;
    v.uf = &mut uf;
    v.uf_unmap_early = &mut early;
    v.uf_unmap = &mut unmap;
    v.remap_type = MREMAP_INVALID;
    do_mremap(&mut v) as c_long
}
