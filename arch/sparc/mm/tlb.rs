// SPDX-License-Identifier: GPL-2.0
/* arch/sparc64/mm/tlb.c
 *
 * Copyright (C) 2004 David S. Miller <davem@redhat.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

/* Heavily inspired by the ppc64 code.  */

static DEFINE_PER_CPU!(struct tlb_batch, tlb_batch);

pub unsafe fn flush_tlb_pending() {
    let tb: *mut tlb_batch = &mut *get_cpu_var(tlb_batch);
    let mm: *mut mm_struct = (*tb).mm;

    if (*tb).tlb_nr == 0 {
        put_cpu_var(tlb_batch);
        return;
    }

    flush_tsb_user(tb);

    if CTX_VALID((*mm).context) {
        if (*tb).tlb_nr == 1 {
            global_flush_tlb_page(mm, (*tb).vaddrs[0]);
        } else {
            // CONFIG_SMP selects the SMP implementation in the kernel build.
            #[cfg(CONFIG_SMP)]
            smp_flush_tlb_pending((*tb).mm, (*tb).tlb_nr, &mut (*tb).vaddrs[0]);
            #[cfg(not(CONFIG_SMP))]
            __flush_tlb_pending(
                CTX_HWBITS((*tb).mm.context),
                (*tb).tlb_nr,
                &mut (*tb).vaddrs[0],
            );
        }
    }

    (*tb).tlb_nr = 0;
    put_cpu_var(tlb_batch);
}

pub unsafe fn arch_enter_lazy_mmu_mode() {
    preempt_disable();
}
// For lazy_mmu_mode KUnit tests
EXPORT_SYMBOL_IF_KUNIT!(arch_enter_lazy_mmu_mode);

pub unsafe fn arch_flush_lazy_mmu_mode() {
    let tb: *mut tlb_batch = this_cpu_ptr(&raw mut tlb_batch);

    if (*tb).tlb_nr != 0 {
        flush_tlb_pending();
    }
}
EXPORT_SYMBOL_IF_KUNIT!(arch_flush_lazy_mmu_mode);

pub unsafe fn arch_leave_lazy_mmu_mode() {
    arch_flush_lazy_mmu_mode();
    preempt_enable();
}
EXPORT_SYMBOL_IF_KUNIT!(arch_leave_lazy_mmu_mode);

unsafe fn tlb_batch_add_one(
    mm: *mut mm_struct,
    mut vaddr: c_ulong,
    exec: bool,
    hugepage_shift: c_uint,
) {
    let tb: *mut tlb_batch = &mut *get_cpu_var(tlb_batch);
    let mut nr: c_ulong;

    vaddr &= PAGE_MASK;
    if exec {
        vaddr |= 0x1;
    }

    nr = (*tb).tlb_nr;

    if unlikely(nr != 0 && mm != (*tb).mm) {
        flush_tlb_pending();
        nr = 0;
    }

    if !is_lazy_mmu_mode_active() {
        flush_tsb_user_page(mm, vaddr, hugepage_shift);
        global_flush_tlb_page(mm, vaddr);
        put_cpu_var(tlb_batch);
        return;
    }

    if nr == 0 {
        (*tb).mm = mm;
        (*tb).hugepage_shift = hugepage_shift;
    }

    if (*tb).hugepage_shift != hugepage_shift {
        flush_tlb_pending();
        (*tb).hugepage_shift = hugepage_shift;
        nr = 0;
    }

    (*tb).vaddrs[nr as usize] = vaddr;
    nr = nr.wrapping_add(1);
    (*tb).tlb_nr = nr;
    if nr >= TLB_BATCH_NR {
        flush_tlb_pending();
    }

    put_cpu_var(tlb_batch);
}

pub unsafe fn tlb_batch_add(
    mm: *mut mm_struct,
    vaddr: c_ulong,
    _ptep: *mut pte_t,
    orig: pte_t,
    fullmm: c_int,
    hugepage_shift: c_uint,
) {
    if tlb_type != hypervisor && pte_dirty(orig) {
        let (paddr, pfn): (c_ulong, c_ulong);
        let mapping: *mut address_space;
        let page: *mut page;
        let folio: *mut folio;

        pfn = pte_pfn(orig);
        if !pfn_valid(pfn) {
            goto_no_cache_flush!();
        } else {
            page = pfn_to_page(pfn);
            if PageReserved(page) {
                goto_no_cache_flush!();
            } else {
                /* A real file page? */
                folio = page_folio(page);
                mapping = folio_flush_mapping(folio);
                if mapping.is_null() {
                    goto_no_cache_flush!();
                } else {
                    paddr = page_address(page) as c_ulong;
                    if (paddr ^ vaddr) & (1 << 13) != 0 {
                        flush_dcache_folio_all(mm, folio);
                    }
                }
            }
        }
    }

    if fullmm == 0 {
        tlb_batch_add_one(mm, vaddr, pte_exec(orig), hugepage_shift);
    }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn tlb_batch_pmd_scan(mm: *mut mm_struct, mut vaddr: c_ulong, pmd: pmd_t) {
    let end: c_ulong;
    let mut pte: *mut pte_t;

    pte = pte_offset_map(&pmd, vaddr);
    if pte.is_null() {
        return;
    }
    end = vaddr + HPAGE_SIZE;
    while vaddr < end {
        if pte_val(*pte) & _PAGE_VALID != 0 {
            let exec = pte_exec(*pte);
            tlb_batch_add_one(mm, vaddr, exec, PAGE_SHIFT);
        }
        pte = pte.add(1);
        vaddr += PAGE_SIZE;
    }
    pte_unmap(pte);
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn __set_pmd_acct(mm: *mut mm_struct, mut addr: c_ulong, orig: pmd_t, pmd: pmd_t) {
    if mm == &raw mut init_mm {
        return;
    }

    if (pmd_val(pmd) ^ pmd_val(orig)) & _PAGE_PMD_HUGE != 0 {
        if pmd_val(pmd) & _PAGE_PMD_HUGE != 0 {
            if is_huge_zero_pmd(pmd) {
                (*mm).context.hugetlb_pte_count += 1;
            } else {
                (*mm).context.thp_pte_count += 1;
            }
        } else if is_huge_zero_pmd(orig) {
            (*mm).context.hugetlb_pte_count -= 1;
        } else {
            (*mm).context.thp_pte_count -= 1;
        }
    }

    if !pmd_none(orig) {
        addr &= HPAGE_MASK;
        if pmd_trans_huge(orig) {
            let orig_pte = __pte(pmd_val(orig));
            let exec = pte_exec(orig_pte);
            tlb_batch_add_one(mm, addr, exec, REAL_HPAGE_SHIFT);
            tlb_batch_add_one(mm, addr + REAL_HPAGE_SIZE, exec, REAL_HPAGE_SHIFT);
        } else {
            tlb_batch_pmd_scan(mm, addr, orig);
        }
    }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn set_pmd_at(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, pmd: pmd_t) {
    let orig = *pmdp;
    *pmdp = pmd;
    __set_pmd_acct(mm, addr, orig, pmd);
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn pmdp_establish(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t, pmd: pmd_t) -> pmd_t {
    let mut old: pmd_t;
    loop {
        old = *pmdp;
        if cmpxchg64(&mut (*pmdp).pmd, old.pmd, pmd.pmd) == old.pmd {
            break;
        }
    }
    __set_pmd_acct((*vma).vm_mm, address, old, pmd);
    old
}

/* This routine is only called when splitting a THP */
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmdp_invalidate(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    let old: pmd_t;
    let entry: pmd_t;
    VM_WARN_ON_ONCE!(!pmd_present(*pmdp));
    entry = __pmd(pmd_val(*pmdp) & !_PAGE_VALID);
    old = pmdp_establish(vma, address, pmdp, entry);
    flush_tlb_range(vma, address, address + HPAGE_PMD_SIZE);

    /* set_pmd_at() will not be called in a way to decrement
     * thp_pte_count when splitting a THP, so do it now.
     * Sanity check pmd before doing the actual decrement. */
    if pmd_val(entry) & _PAGE_PMD_HUGE != 0 && !is_huge_zero_pmd(entry) {
        (*(*vma).vm_mm).context.thp_pte_count -= 1;
    }
    old
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t) {
    let lh = pgtable as *mut list_head;
    assert_spin_locked(&mut (*mm).page_table_lock);
    if pmd_huge_pte(mm, pmdp).is_null() {
        INIT_LIST_HEAD(lh);
    } else {
        list_add(lh, pmd_huge_pte(mm, pmdp) as *mut list_head);
    }
    *pmd_huge_pte(mm, pmdp) = pgtable;
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t {
    let pgtable = pmd_huge_pte(mm, pmdp);
    let lh = pgtable as *mut list_head;
    assert_spin_locked(&mut (*mm).page_table_lock);
    if list_empty(lh) {
        *pmd_huge_pte(mm, pmdp) = core::ptr::null_mut();
    } else {
        *pmd_huge_pte(mm, pmdp) = (*lh).next as pgtable_t;
        list_del(lh);
    }
    pte_val_mut(&mut (*pgtable.add(0))) = 0;
    pte_val_mut(&mut (*pgtable.add(1))) = 0;
    pgtable
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
