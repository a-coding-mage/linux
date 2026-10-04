// SPDX-License-Identifier: GPL-2.0
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_flush() {
    let ubc = rust_rmap_current_tlb_ubc();
    if !(*ubc).flush_required {
        return;
    }
    rust_rmap_arch_tlbbatch_flush(addr_of_mut!((*ubc).arch));
    (*ubc).flush_required = false;
    (*ubc).writable = false;
}
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_flush_dirty() {
    if (*rust_rmap_current_tlb_ubc()).writable {
        try_to_unmap_flush();
    }
}
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
unsafe fn set_tlb_ubc_flush_pending(
    mm: *mut mm_struct,
    pteval: pte_t,
    start: c_ulong,
    end: c_ulong,
) {
    let ubc = rust_rmap_current_tlb_ubc();
    let writable = rust_rmap_pte_dirty(pteval);
    if !rust_rmap_pte_accessible(mm, pteval) {
        return;
    }
    rust_rmap_arch_tlbbatch_add_pending(addr_of_mut!((*ubc).arch), mm, start, end);
    (*ubc).flush_required = true;
    rust_rmap_barrier();
    let counter = rust_rmap_mm_tlb_flush_batched(mm);
    let mut batch = rust_rmap_atomic_read(counter);
    loop {
        if batch & TLB_FLUSH_BATCH_PENDING_MASK > TLB_FLUSH_BATCH_PENDING_LARGE {
            if !rust_rmap_atomic_try_cmpxchg(counter, &mut batch, 1) {
                continue;
            }
        } else {
            rust_rmap_atomic_inc(counter);
        }
        break;
    }
    if writable {
        (*ubc).writable = true;
    }
}
#[cfg(not(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH))]
unsafe fn set_tlb_ubc_flush_pending(
    _mm: *mut mm_struct,
    _pteval: pte_t,
    _start: c_ulong,
    _end: c_ulong,
) {
    // Original !CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH empty definition.
}
unsafe fn should_defer_flush(mm: *mut mm_struct, flags: ttu_flags) -> bool {
    #[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
    {
        flags & TTU_BATCH_FLUSH != 0 && rust_rmap_arch_tlbbatch_should_defer(mm)
    }
    #[cfg(not(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH))]
    {
        let _ = (mm, flags);
        false
    }
}
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_batched_pending(mm: *mut mm_struct) {
    let counter = rust_rmap_mm_tlb_flush_batched(mm);
    let batch = rust_rmap_atomic_read(counter);
    let pending = batch & TLB_FLUSH_BATCH_PENDING_MASK;
    let flushed = batch >> TLB_FLUSH_BATCH_FLUSHED_SHIFT;
    if pending != flushed {
        rust_rmap_flush_tlb_mm(mm);
        rust_rmap_atomic_cmpxchg(
            counter,
            batch,
            pending | (pending << TLB_FLUSH_BATCH_FLUSHED_SHIFT),
        );
    }
}
unsafe fn folio_unmap_pte_batch(
    folio: *mut folio,
    pvmw: *mut page_vma_mapped_walk,
    flags: ttu_flags,
    mut pte: pte_t,
) -> c_uint {
    if flags & TTU_HWPOISON != 0 || !rust_rmap_folio_test_large(folio) {
        return 1;
    }
    let vma = (*pvmw).vma;
    let addr = (*pvmw).address;
    let max_nr = (rust_rmap_pmd_addr_end(addr, rust_rmap_vma_end(vma)).wrapping_sub(addr)
        >> RUST_RMAP_PAGE_SHIFT) as c_uint;
    if rust_rmap_folio_test_anon(folio) && rust_rmap_folio_test_swapbacked(folio) {
        return 1;
    }
    if rust_rmap_pte_unused(pte) {
        return 1;
    }
    rust_rmap_folio_pte_batch_flags(
        folio,
        vma,
        (*pvmw).pte,
        &mut pte,
        max_nr,
        (RUST_RMAP_FPB_RESPECT_WRITE | RUST_RMAP_FPB_RESPECT_SOFT_DIRTY) as fpb_t,
    )
}
#[cfg(CONFIG_HUGETLB_PAGE)]
unsafe extern "C" fn try_to_unmap_poisoned_hugetlb_one(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    arg: *mut c_void,
) -> bool {
    let mut pvmw = folio_vma_walk(folio, vma, address, 0);
    let hsz = rust_rmap_huge_page_size(rust_rmap_hstate_vma(vma));
    let flags = arg as c_long as ttu_flags;
    let mm = (*vma).vm_mm;
    let mut range: mmu_notifier_range = zeroed();
    let mut ret = true;
    rust_rmap_diag_poisoned_hugetlb_1(folio, flags);
    rust_rmap_diag_poisoned_hugetlb_2(folio, flags);
    let end = rust_rmap_vma_address_end(&mut pvmw);
    rust_rmap_mmu_notifier_range_init(&mut range, MMU_NOTIFY_CLEAR, 0, mm, address, end);
    rust_rmap_adjust_range_if_pmd_sharing_possible(vma, &mut range.start, &mut range.end);
    rust_rmap_mmu_notifier_invalidate_range_start(&mut range);
    if rust_rmap_page_vma_mapped_walk(&mut pvmw) {
        'mapped: {
            rust_rmap_diag_poisoned_address(address, pvmw.address);
            let mut pteval = rust_rmap_huge_ptep_get(mm, address, pvmw.pte);
            rust_rmap_diag_poisoned_pte_1(pteval, folio);
            rust_rmap_diag_poisoned_pte_2(pteval, folio);
            rust_rmap_flush_cache_range(vma, range.start, range.end);
            if !rust_rmap_folio_test_anon(folio) {
                rust_rmap_diag_poisoned_rmap_locked(flags);
                if !rust_rmap_hugetlb_vma_trylock_write(vma) {
                    ret = false;
                    break 'mapped;
                }
                let mut tlb: mmu_gather = zeroed();
                rust_rmap_tlb_gather_mmu_vma(&mut tlb, vma);
                if rust_rmap_huge_pmd_unshare(&mut tlb, vma, address, pvmw.pte) != 0 {
                    rust_rmap_hugetlb_vma_unlock_write(vma);
                    rust_rmap_huge_pmd_unshare_flush(&mut tlb, vma);
                    rust_rmap_tlb_finish_mmu(&mut tlb);
                    break 'mapped;
                }
                rust_rmap_hugetlb_vma_unlock_write(vma);
                rust_rmap_tlb_finish_mmu(&mut tlb);
            }
            pteval = rust_rmap_huge_ptep_clear_flush(vma, address, pvmw.pte);
            if rust_rmap_huge_pte_dirty(pteval) {
                rust_rmap_folio_mark_dirty(folio);
            }
            pteval = rust_rmap_swp_entry_to_pte(rust_rmap_make_hwpoison_entry(
                rust_rmap_folio_page(folio, 0),
            ));
            rust_rmap_hugetlb_count_sub(rust_rmap_folio_nr_pages(folio) as c_long, mm);
            rust_rmap_set_huge_pte_at(mm, address, pvmw.pte, pteval, hsz);
            rust_rmap_hugetlb_remove_rmap(folio);
            rust_rmap_folio_put_refs(folio, 1);
        }
        rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
    }
    rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
    ret
}
unsafe fn ttu_anon_lazyfree_folio(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    nr_pages: c_ulong,
) -> bool {
    rust_rmap_smp_mb();
    let ref_count = rust_rmap_folio_ref_count(folio);
    let map_count = rust_rmap_folio_mapcount(folio);
    rust_rmap_smp_rmb();
    if rust_rmap_folio_test_dirty(folio)
        && rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_DROPPABLE as c_ulong == 0
    {
        rust_rmap_folio_set_swapbacked(folio);
        return false;
    }
    if ref_count != map_count.wrapping_add(1) {
        return false;
    }
    rust_rmap_add_mm_counter(
        (*vma).vm_mm,
        MM_ANONPAGES as c_int,
        nr_pages.wrapping_neg() as c_long,
    );
    true
}
unsafe fn swp_pte_prepare(entry: swp_entry_t, old_pte: pte_t, anon_exclusive: bool) -> pte_t {
    let mut pte = rust_rmap_swp_entry_to_pte(entry);
    if anon_exclusive {
        pte = rust_rmap_pte_swp_mkexclusive(pte);
    }
    if rust_rmap_pte_present(old_pte) {
        if rust_rmap_pte_soft_dirty(old_pte) {
            pte = rust_rmap_pte_swp_mksoft_dirty(pte);
        }
        if rust_rmap_pte_uffd(old_pte) {
            pte = rust_rmap_pte_swp_mkuffd(pte);
        }
    } else {
        if rust_rmap_pte_swp_soft_dirty(old_pte) {
            pte = rust_rmap_pte_swp_mksoft_dirty(pte);
        }
        if rust_rmap_pte_swp_uffd(old_pte) {
            pte = rust_rmap_pte_swp_mkuffd(pte);
        }
    }
    pte
}
unsafe fn ttu_anon_swapbacked_folio(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    page: *mut page,
    address: c_ulong,
    ptep: *mut pte_t,
    pteval: pte_t,
) -> bool {
    let exclusive = rust_rmap_folio_test_anon(folio) && rust_rmap_page_anon_exclusive(page);
    let entry = rust_rmap_page_swap_entry(page);
    let mm = (*vma).vm_mm;
    if rust_rmap_folio_dup_swap(folio, page) < 0 {
        return false;
    }
    if rust_rmap_arch_unmap_one(mm, vma, address, pteval) < 0 {
        rust_rmap_folio_put_swap(folio, page);
        return false;
    }
    if exclusive && rust_rmap_folio_try_share_anon_rmap_pte(folio, page) != 0 {
        rust_rmap_folio_put_swap(folio, page);
        return false;
    }
    rust_rmap_mm_prepare_for_swap_entries(mm);
    rust_rmap_dec_mm_counter(mm, MM_ANONPAGES as c_int);
    rust_rmap_inc_mm_counter(mm, MM_SWAPENTS as c_int);
    rust_rmap_set_pte_at(mm, address, ptep, swp_pte_prepare(entry, pteval, exclusive));
    true
}
unsafe fn ttu_anon_folio(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    page: *mut page,
    address: c_ulong,
    ptep: *mut pte_t,
    pteval: pte_t,
    nr_pages: c_ulong,
) -> bool {
    if rust_rmap_diag_swapbacked_swapcache(folio) {
        return false;
    }
    if !rust_rmap_folio_test_swapbacked(folio) {
        return ttu_anon_lazyfree_folio(vma, folio, nr_pages);
    }
    ttu_anon_swapbacked_folio(vma, folio, page, address, ptep, pteval)
}
unsafe extern "C" fn try_to_unmap_one(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    mut address: c_ulong,
    arg: *mut c_void,
) -> bool {
    let mm = (*vma).vm_mm;
    let mut pvmw = folio_vma_walk(folio, vma, address, 0);
    let mut ret = true;
    let mut range: mmu_notifier_range = zeroed();
    let mut flags = arg as c_long as ttu_flags;
    let mut ptes: c_int = 0;
    if flags & TTU_SYNC != 0 {
        pvmw.flags = RUST_RMAP_PVMW_SYNC as c_uint;
    }
    let end = rust_rmap_vma_address_end(&mut pvmw);
    rust_rmap_mmu_notifier_range_init(&mut range, MMU_NOTIFY_CLEAR, 0, mm, address, end);
    rust_rmap_mmu_notifier_invalidate_range_start(&mut range);
    while rust_rmap_page_vma_mapped_walk(&mut pvmw) {
        let mut nr_pages: c_ulong = 1;
        if flags & TTU_IGNORE_MLOCK == 0
            && rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_LOCKED as c_ulong != 0
        {
            ptes = ptes.wrapping_add(1);
            ret = false;
            if !pvmw.pte.is_null() && ptes as c_ulong != pvmw.nr_pages {
                continue;
            }
            if pvmw.flags & RUST_RMAP_PVMW_PGTABLE_CROSSED as c_uint == 0 {
                rust_rmap_mlock_vma_folio(folio, vma);
            }
            rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
            break;
        }
        if pvmw.pte.is_null() {
            if rust_rmap_folio_test_lazyfree(folio) {
                if !rust_rmap_unmap_huge_pmd_locked(vma, pvmw.address, pvmw.pmd, folio) {
                    ret = false;
                }
                rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                break;
            }
            if flags & TTU_SPLIT_HUGE_PMD != 0 {
                rust_rmap_split_huge_pmd_locked(vma, pvmw.address, pvmw.pmd, false);
                flags &= !TTU_SPLIT_HUGE_PMD;
                rust_rmap_page_vma_mapped_walk_restart(&mut pvmw);
                continue;
            }
        }
        rust_rmap_diag_unmap_pte(folio, pvmw.pte);
        address = pvmw.address;
        let mut pteval = rmap_ptep_get(folio, mm, address, pvmw.pte);
        let pfn = if rust_rmap_pte_present(pteval) {
            rust_rmap_pte_pfn(pteval)
        } else {
            rust_rmap_softleaf_to_pfn(rust_rmap_softleaf_from_pte(pteval))
        };
        let page = rust_rmap_folio_page(folio, pfn.wrapping_sub(rust_rmap_folio_pfn(folio)));
        if rust_rmap_pte_present(pteval) {
            nr_pages = folio_unmap_pte_batch(folio, &mut pvmw, flags, pteval) as c_ulong;
            let end_addr =
                address.wrapping_add(nr_pages.wrapping_mul(RUST_RMAP_PAGE_SIZE as c_ulong));
            rust_rmap_flush_cache_range(vma, address, end_addr);
            pteval = rust_rmap_get_and_clear_ptes(mm, address, pvmw.pte, nr_pages as c_uint);
            if should_defer_flush(mm, flags) {
                set_tlb_ubc_flush_pending(mm, pteval, address, end_addr);
            } else {
                rust_rmap_flush_tlb_range(vma, address, end_addr);
            }
            if rust_rmap_pte_dirty(pteval) {
                rust_rmap_folio_mark_dirty(folio);
            }
        } else {
            rust_rmap_pte_clear(mm, address, pvmw.pte);
        }
        rust_rmap_cond_install_uffd_wp_ptes(vma, address, pvmw.pte, pteval, nr_pages as c_uint);
        rust_rmap_update_hiwater_rss(mm);
        if rust_rmap_folio_test_hwpoison(folio) && flags & TTU_HWPOISON != 0 {
            pteval = rust_rmap_swp_entry_to_pte(rust_rmap_make_hwpoison_entry(page));
            rust_rmap_dec_mm_counter(mm, rust_rmap_mm_counter(folio));
            rust_rmap_set_pte_at(mm, address, pvmw.pte, pteval);
        } else if rust_rmap_pte_present(pteval)
            && rust_rmap_pte_unused(pteval)
            && !rust_rmap_userfaultfd_armed(vma)
        {
            rust_rmap_dec_mm_counter(mm, rust_rmap_mm_counter(folio));
        } else if rust_rmap_folio_test_anon(folio) {
            if !ttu_anon_folio(vma, folio, page, address, pvmw.pte, pteval, nr_pages) {
                rust_rmap_set_ptes(mm, address, pvmw.pte, pteval, nr_pages as c_uint);
                ret = false;
                rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                break;
            }
        } else {
            rust_rmap_add_mm_counter(
                mm,
                rust_rmap_mm_counter_file(folio),
                nr_pages.wrapping_neg() as c_long,
            );
        }
        folio_remove_rmap_ptes(folio, page, nr_pages as c_int, vma);
        if rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_LOCKED as c_ulong != 0 {
            rust_rmap_mlock_drain_local();
        }
        rust_rmap_folio_put_refs(folio, nr_pages as c_int);
        if nr_pages == rust_rmap_folio_nr_pages(folio) as c_ulong {
            rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
            break;
        }
    }
    rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
    ret
}
unsafe extern "C" fn invalid_migration_vma(vma: *mut vm_area_struct, _arg: *mut c_void) -> bool {
    rust_rmap_vma_is_temporary_stack(vma)
}
unsafe extern "C" fn folio_not_mapped(folio: *mut folio) -> c_int {
    (!rust_rmap_folio_mapped(folio)) as c_int
}
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap(folio: *mut folio, flags: ttu_flags) {
    let mut rwc: rmap_walk_control = zeroed();
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        rwc.rmap_one = Some(if rust_rmap_folio_test_hugetlb(folio) {
            try_to_unmap_poisoned_hugetlb_one
        } else {
            try_to_unmap_one
        });
    }
    #[cfg(not(CONFIG_HUGETLB_PAGE))]
    {
        rwc.rmap_one = Some(try_to_unmap_one);
    }
    rwc.arg = flags as usize as *mut c_void;
    rwc.done = Some(folio_not_mapped);
    rwc.anon_lock = Some(folio_lock_anon_vma_read);
    if flags & TTU_RMAP_LOCKED != 0 {
        rmap_walk_locked(folio, &mut rwc);
    } else {
        rmap_walk(folio, &mut rwc);
    }
}
