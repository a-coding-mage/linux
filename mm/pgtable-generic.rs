// SPDX-License-Identifier: GPL-2.0
/*
 *  mm/pgtable-generic.c
 *
 *  Generic pgtable methods declared in linux/pgtable.h
 *
 *  Copyright (C) 2010  Linus Torvalds
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/* If a p?d_bad entry is found while walking page tables, report the error,
 * before resetting entry to p?d_none. */
pub unsafe fn pgd_clear_bad(pgd: *mut pgd_t) {
    pgd_ERROR(*pgd);
    pgd_clear(pgd);
}

#[cfg(not(__PAGETABLE_P4D_FOLDED))]
pub unsafe fn p4d_clear_bad(p4d: *mut p4d_t) {
    p4d_ERROR(*p4d);
    p4d_clear(p4d);
}

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn pud_clear_bad(pud: *mut pud_t) {
    pud_ERROR(*pud);
    pud_clear(pud);
}

pub unsafe fn pmd_clear_bad(pmd: *mut pmd_t) {
    pmd_ERROR(*pmd);
    pmd_clear(pmd);
}

#[cfg(not(__HAVE_ARCH_PTEP_SET_ACCESS_FLAGS))]
pub unsafe fn ptep_set_access_flags(vma: *mut vm_area_struct, address: c_ulong,
                                    ptep: *mut pte_t, entry: pte_t, _dirty: c_int) -> c_int {
    let changed: c_int = (!pte_same(ptep_get(ptep), entry)) as c_int;
    if changed != 0 {
        set_pte_at((*vma).vm_mm, address, ptep, entry);
        flush_tlb_fix_spurious_fault(vma, address, ptep);
    }
    changed
}

#[cfg(not(__HAVE_ARCH_PTEP_CLEAR_YOUNG_FLUSH))]
pub unsafe fn ptep_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong,
                                     ptep: *mut pte_t) -> bool {
    let young = ptep_test_and_clear_young(vma, address, ptep);
    if young { flush_tlb_page(vma, address); }
    young
}

#[cfg(not(__HAVE_ARCH_PTEP_CLEAR_FLUSH))]
pub unsafe fn ptep_clear_flush(vma: *mut vm_area_struct, address: c_ulong,
                               ptep: *mut pte_t) -> pte_t {
    let mm: *mut mm_struct = (*vma).vm_mm;
    let pte = ptep_get_and_clear(mm, address, ptep);
    if pte_accessible(mm, pte) { flush_tlb_page(vma, address); }
    pte
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[cfg(not(__HAVE_ARCH_PMDP_SET_ACCESS_FLAGS))]
pub unsafe fn pmdp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong,
                                    pmdp: *mut pmd_t, entry: pmd_t, _dirty: c_int) -> c_int {
    let changed: c_int = (!pmd_same(*pmdp, entry)) as c_int;
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    if changed != 0 {
        set_pmd_at((*vma).vm_mm, address, pmdp, entry);
        flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    }
    changed
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[cfg(not(__HAVE_ARCH_PMDP_CLEAR_YOUNG_FLUSH))]
pub unsafe fn pmdp_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong,
                                     pmdp: *mut pmd_t) -> bool {
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    let young = pmdp_test_and_clear_young(vma, address, pmdp);
    if young { flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE); }
    young
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[cfg(not(__HAVE_ARCH_PMDP_HUGE_CLEAR_FLUSH))]
pub unsafe fn pmdp_huge_clear_flush(vma: *mut vm_area_struct, address: c_ulong,
                                     pmdp: *mut pmd_t) -> pmd_t {
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_present(*pmdp) && !pmd_trans_huge(*pmdp));
    let pmd = pmdp_huge_get_and_clear((*vma).vm_mm, address, pmdp);
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    pmd
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
pub unsafe fn pudp_huge_clear_flush(vma: *mut vm_area_struct, address: c_ulong,
                                     pudp: *mut pud_t) -> pud_t {
    VM_BUG_ON(address & !HPAGE_PUD_MASK);
    VM_BUG_ON(!pud_trans_huge(*pudp));
    let pud = pudp_huge_get_and_clear((*vma).vm_mm, address, pudp);
    flush_pud_tlb_range(vma, address, address + HPAGE_PUD_SIZE);
    pud
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(__HAVE_ARCH_PGTABLE_DEPOSIT)))]
pub unsafe fn pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t,
                                         pgtable: pgtable_t) {
    assert_spin_locked(pmd_lockptr(mm, pmdp));
    if pmd_huge_pte(mm, pmdp).is_null() {
        INIT_LIST_HEAD(&mut (*pgtable).lru);
    } else {
        list_add(&mut (*pgtable).lru, &mut (*pmd_huge_pte(mm, pmdp)).lru);
    }
    pmd_huge_pte_set(mm, pmdp, pgtable);
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(__HAVE_ARCH_PGTABLE_WITHDRAW)))]
pub unsafe fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t {
    assert_spin_locked(pmd_lockptr(mm, pmdp));
    let pgtable = pmd_huge_pte(mm, pmdp);
    let first = list_first_entry_or_null(&mut (*pgtable).lru, page, lru);
    pmd_huge_pte_set(mm, pmdp, first);
    if !pmd_huge_pte(mm, pmdp).is_null() { list_del(&mut (*pgtable).lru); }
    pgtable
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(__HAVE_ARCH_PMDP_INVALIDATE)))]
pub unsafe fn pmdp_invalidate(vma: *mut vm_area_struct, address: c_ulong,
                              pmdp: *mut pmd_t) -> pmd_t {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
    let old = pmdp_establish(vma, address, pmdp, pmd_mkinvalid(*pmdp));
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    old
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(__HAVE_ARCH_PMDP_INVALIDATE_AD)))]
pub unsafe fn pmdp_invalidate_ad(vma: *mut vm_area_struct, address: c_ulong,
                                 pmdp: *mut pmd_t) -> pmd_t {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
    pmdp_invalidate(vma, address, pmdp)
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(pmdp_collapse_flush)))]
pub unsafe fn pmdp_collapse_flush(vma: *mut vm_area_struct, address: c_ulong,
                                  pmdp: *mut pmd_t) -> pmd_t {
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_trans_huge(*pmdp));
    let pmd = pmdp_huge_get_and_clear((*vma).vm_mm, address, pmdp);
    flush_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    pmd
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(pte_free_defer)))]
unsafe fn pte_free_now(head: *mut rcu_head) {
    let page = container_of(head, page, rcu_head);
    pte_free(core::ptr::null_mut(), page as pgtable_t);
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(pte_free_defer)))]
pub unsafe fn pte_free_defer(_mm: *mut mm_struct, pgtable: pgtable_t) {
    let page = pgtable as *mut page;
    call_rcu(&mut (*page).rcu_head, pte_free_now);
}

#[cfg(all(CONFIG_GUP_GET_PXX_LOW_HIGH, any(CONFIG_SMP, CONFIG_PREEMPT_RCU)))]
unsafe fn pmdp_get_lockless_start() -> c_ulong { let mut irqflags = 0; local_irq_save(&mut irqflags); irqflags }
#[cfg(all(CONFIG_GUP_GET_PXX_LOW_HIGH, any(CONFIG_SMP, CONFIG_PREEMPT_RCU)))]
unsafe fn pmdp_get_lockless_end(irqflags: c_ulong) { local_irq_restore(irqflags); }
#[cfg(not(all(CONFIG_GUP_GET_PXX_LOW_HIGH, any(CONFIG_SMP, CONFIG_PREEMPT_RCU))))]
unsafe fn pmdp_get_lockless_start() -> c_ulong { 0 }
#[cfg(not(all(CONFIG_GUP_GET_PXX_LOW_HIGH, any(CONFIG_SMP, CONFIG_PREEMPT_RCU))))]
unsafe fn pmdp_get_lockless_end(_irqflags: c_ulong) {}

pub unsafe fn __pte_offset_map(pmd: *mut pmd_t, addr: c_ulong, pmdvalp: *mut pmd_t) -> *mut pte_t {
    rcu_read_lock();
    let irqflags = pmdp_get_lockless_start();
    let pmdval = pmdp_get_lockless(pmd);
    pmdp_get_lockless_end(irqflags);
    if !pmdvalp.is_null() { *pmdvalp = pmdval; }
    if unlikely(pmd_none(pmdval) || !pmd_present(pmdval) || pmd_trans_huge(pmdval)) { rcu_read_unlock(); return core::ptr::null_mut(); }
    if unlikely(pmd_bad(pmdval)) { pmd_clear_bad(pmd); rcu_read_unlock(); return core::ptr::null_mut(); }
    __pte_map(&pmdval, addr)
}

pub unsafe fn pte_offset_map_ro_nolock(mm: *mut mm_struct, pmd: *mut pmd_t, addr: c_ulong,
                                       ptlp: *mut *mut spinlock_t) -> *mut pte_t {
    let mut pmdval = core::mem::zeroed();
    let pte = __pte_offset_map(pmd, addr, &mut pmdval);
    if likely(!pte.is_null()) { *ptlp = pte_lockptr(mm, &pmdval); }
    pte
}

pub unsafe fn pte_offset_map_rw_nolock(mm: *mut mm_struct, pmd: *mut pmd_t, addr: c_ulong,
                                       pmdvalp: *mut pmd_t, ptlp: *mut *mut spinlock_t) -> *mut pte_t {
    VM_WARN_ON_ONCE(pmdvalp.is_null());
    let pte = __pte_offset_map(pmd, addr, pmdvalp);
    if likely(!pte.is_null()) { *ptlp = pte_lockptr(mm, pmdvalp); }
    pte
}

pub unsafe fn pte_offset_map_lock(mm: *mut mm_struct, pmd: *mut pmd_t, addr: c_ulong,
                                  ptlp: *mut *mut spinlock_t) -> *mut pte_t {
    loop {
        let mut pmdval = core::mem::zeroed();
        let pte = __pte_offset_map(pmd, addr, &mut pmdval);
        if unlikely(pte.is_null()) { return pte; }
        let ptl = pte_lockptr(mm, &pmdval);
        spin_lock(ptl);
        if likely(pmd_same(pmdval, pmdp_get_lockless(pmd))) { *ptlp = ptl; return pte; }
        pte_unmap_unlock(pte, ptl);
    }
}

#[cfg(CONFIG_ASYNC_KERNEL_PGTABLE_FREE)]
unsafe fn kernel_pgtable_work_func(work: *mut work_struct);

#[cfg(CONFIG_ASYNC_KERNEL_PGTABLE_FREE)]
#[repr(C)]
struct KernelPgtableWork {
    list: list_head,
    lock: spinlock_t,
    work: work_struct,
}

#[cfg(CONFIG_ASYNC_KERNEL_PGTABLE_FREE)]
static mut kernel_pgtable_work: KernelPgtableWork = KernelPgtableWork {
    list: LIST_HEAD_INIT(),
    lock: __SPIN_LOCK_UNLOCKED(),
    work: __WORK_INITIALIZER(kernel_pgtable_work_func),
};

#[cfg(CONFIG_ASYNC_KERNEL_PGTABLE_FREE)]
unsafe fn kernel_pgtable_work_func(work: *mut work_struct) {
    let mut page_list: list_head = LIST_HEAD_INIT();
    spin_lock(&mut kernel_pgtable_work.lock);
    list_splice_tail_init(&mut kernel_pgtable_work.list, &mut page_list);
    spin_unlock(&mut kernel_pgtable_work.lock);
    iommu_sva_invalidate_kva_range(PAGE_OFFSET, TLB_FLUSH_ALL);
    let mut pt: *mut ptdesc = core::ptr::null_mut();
    let mut next: *mut ptdesc = core::ptr::null_mut();
    list_for_each_entry_safe(&mut pt, &mut next, &mut page_list, pt_list) {
        __pagetable_free(pt);
    }
}

#[cfg(CONFIG_ASYNC_KERNEL_PGTABLE_FREE)]
pub unsafe fn pagetable_free_kernel(pt: *mut ptdesc) {
    spin_lock(&mut kernel_pgtable_work.lock);
    list_add(&mut (*pt).pt_list, &mut kernel_pgtable_work.list);
    spin_unlock(&mut kernel_pgtable_work.lock);
    schedule_work(&mut kernel_pgtable_work.work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
