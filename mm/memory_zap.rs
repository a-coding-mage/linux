// SPDX-License-Identifier: GPL-2.0-only
// mm/memory.c: should_zap_cows through zap_special_vma_range.
unsafe fn should_zap_cows(details: *mut zap_details) -> bool {
    if details.is_null() {
        return true;
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_skip_cows_reclaim(
        rust_memory_zap_skip_cows(details) && rust_memory_zap_reclaim_pt(details),
    );
    !rust_memory_zap_skip_cows(details)
}
unsafe fn should_zap_folio(details: *mut zap_details, folio: *mut folio) -> bool {
    should_zap_cows(details) || !rust_memory_folio_test_anon(folio)
}
unsafe fn zap_drop_markers(details: *mut zap_details) -> bool {
    !details.is_null()
        && (*details).zap_flags & RUST_MEMORY_ZAP_FLAG_DROP_MARKER as zap_flags_t != 0
}
#[no_mangle]
pub unsafe extern "C" fn cond_install_uffd_wp_ptes(
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    mut ptep: *mut pte_t,
    pte: pte_t,
    mut nr_ptes: c_ulong,
) -> bool {
    if !rust_memory_uffd_supports_wp_marker() {
        return false;
    }
    rust_memory_warn_uffd_not_cleared(!rust_memory_pte_none(rust_memory_ptep_get(ptep)));
    if rust_memory_vma_is_anonymous(vma) || !rust_memory_userfaultfd_wp(vma) {
        return false;
    }
    let arm = (rust_memory_pte_present(pte) && rust_memory_pte_uffd(pte))
        || rust_memory_pte_swp_uffd_any(pte);
    if !arm {
        return false;
    }
    loop {
        rust_memory_set_pte_at(
            (*vma).vm_mm,
            addr,
            ptep,
            rust_memory_make_pte_marker(RUST_MEMORY_PTE_MARKER_UFFD_WP as _),
        );
        nr_ptes = nr_ptes.wrapping_sub(1);
        if nr_ptes == 0 {
            break;
        }
        ptep = ptep.add(1);
        addr = addr.wrapping_add(PAGE_SIZE);
    }
    true
}
unsafe fn zap_install_uffd_wp_if_needed(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: *mut pte_t,
    nr: c_int,
    details: *mut zap_details,
    pteval: pte_t,
) -> bool {
    if zap_drop_markers(details) {
        return false;
    }
    cond_install_uffd_wp_ptes(vma, addr, pte, pteval, nr as c_ulong)
}
#[inline(always)]
unsafe fn zap_present_folio_ptes(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    folio: *mut folio,
    page: *mut page,
    pte: *mut pte_t,
    mut ptent: pte_t,
    nr: c_uint,
    addr: c_ulong,
    details: *mut zap_details,
    rss: *mut c_int,
    force_flush: *mut bool,
    force_break: *mut bool,
    any_skipped: *mut bool,
) {
    let mm = (*tlb).mm;
    let mut delay_rmap = false;
    if !rust_memory_folio_test_anon(folio) {
        ptent = rust_memory_get_and_clear_full_ptes(mm, addr, pte, nr, rust_memory_tlb_fullmm(tlb));
        if rust_memory_pte_dirty(ptent) {
            rust_memory_folio_mark_dirty(folio);
            if rust_memory_tlb_delay_rmap(tlb) {
                delay_rmap = true;
                *force_flush = true;
            }
        }
        if rust_memory_pte_young(ptent) && rust_memory_vma_has_recency(vma) {
            rust_memory_folio_mark_accessed(folio);
        }
        let counter = rss.add(rust_memory_mm_counter(folio) as usize);
        *counter = (*counter as c_uint).wrapping_sub(nr) as c_int;
    } else {
        rust_memory_clear_full_ptes(mm, addr, pte, nr, rust_memory_tlb_fullmm(tlb));
        let counter = rss.add(RUST_MEMORY_MM_ANONPAGES as usize);
        *counter = (*counter as c_uint).wrapping_sub(nr) as c_int;
    }
    rust_memory_arch_check_zapped_pte(vma, ptent);
    rust_memory_tlb_remove_tlb_entries(tlb, pte, nr, addr);
    if rust_memory_userfaultfd_pte_wp(vma, ptent) {
        *any_skipped = zap_install_uffd_wp_if_needed(vma, addr, pte, nr as c_int, details, ptent);
    }
    if !delay_rmap {
        rust_memory_folio_remove_rmap_ptes(folio, page, nr as c_int, vma);
        if rust_memory_folio_mapcount(folio) < 0 {
            print_bad_pte(vma, addr, ptent, page);
        }
    }
    if rust_memory_tlb_remove_folio_pages(tlb, page, nr, delay_rmap) {
        *force_flush = true;
        *force_break = true;
    }
}
unsafe fn zap_present_ptes(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pte: *mut pte_t,
    ptent: pte_t,
    max_nr: c_uint,
    addr: c_ulong,
    details: *mut zap_details,
    rss: *mut c_int,
    force_flush: *mut bool,
    force_break: *mut bool,
    any_skipped: *mut bool,
) -> c_int {
    let mm = (*tlb).mm;
    let page = vm_normal_page(vma, addr, ptent);
    if page.is_null() {
        rust_memory_ptep_get_and_clear_full(mm, addr, pte, rust_memory_tlb_fullmm(tlb));
        rust_memory_arch_check_zapped_pte(vma, ptent);
        rust_memory_tlb_remove_tlb_entry(tlb, pte, addr);
        if rust_memory_userfaultfd_pte_wp(vma, ptent) {
            *any_skipped = zap_install_uffd_wp_if_needed(vma, addr, pte, 1, details, ptent);
        }
        rust_memory_ksm_might_unmap_zero_page(mm, ptent);
        return 1;
    }
    let folio = rust_memory_page_folio(page);
    if !should_zap_folio(details, folio) {
        *any_skipped = true;
        return 1;
    }
    let nr = if rust_memory_folio_test_large(folio) && max_nr != 1 {
        rust_memory_folio_pte_batch(folio, pte, ptent, max_nr as _)
    } else {
        1
    };
    zap_present_folio_ptes(
        tlb,
        vma,
        folio,
        page,
        pte,
        ptent,
        nr as c_uint,
        addr,
        details,
        rss,
        force_flush,
        force_break,
        any_skipped,
    );
    nr
}
unsafe fn zap_nonpresent_ptes(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pte: *mut pte_t,
    ptent: pte_t,
    max_nr: c_uint,
    addr: c_ulong,
    details: *mut zap_details,
    rss: *mut c_int,
    any_skipped: *mut bool,
) -> c_int {
    let mut nr = 1;
    *any_skipped = true;
    let entry = rust_memory_softleaf_from_pte(ptent);
    if rust_memory_softleaf_is_device_private(entry)
        || rust_memory_softleaf_is_device_exclusive(entry)
    {
        let page = rust_memory_softleaf_to_page(entry);
        let folio = rust_memory_page_folio(page);
        if !should_zap_folio(details, folio) {
            return 1;
        }
        rust_memory_warn_device_not_anon(!rust_memory_folio_test_anon(folio));
        *rss.add(rust_memory_mm_counter(folio) as usize) -= 1;
        rust_memory_folio_remove_rmap_pte(folio, page, vma);
        rust_memory_folio_put(folio);
    } else if rust_memory_softleaf_is_swap(entry) {
        if !should_zap_cows(details) {
            return 1;
        }
        nr = rust_memory_swap_pte_batch(pte, max_nr, ptent);
        *rss.add(RUST_MEMORY_MM_SWAPENTS as usize) -= nr;
        rust_memory_swap_put_entries_direct(entry, nr as _);
    } else if rust_memory_softleaf_is_migration(entry) {
        let folio = rust_memory_softleaf_to_folio(entry);
        if !should_zap_folio(details, folio) {
            return 1;
        }
        *rss.add(rust_memory_mm_counter(folio) as usize) -= 1;
    } else if rust_memory_softleaf_is_uffd_wp_marker(entry) {
        if !rust_memory_vma_is_anonymous(vma) && !zap_drop_markers(details) {
            return 1;
        }
    } else if rust_memory_softleaf_is_guard_marker(entry) {
        if !zap_drop_markers(details) {
            return 1;
        }
    } else if rust_memory_softleaf_is_hwpoison(entry)
        || rust_memory_softleaf_is_poison_marker(entry)
    {
        if !should_zap_cows(details) {
            return 1;
        }
    } else {
        rust_memory_log_unknown_swap(entry.val);
        rust_memory_warn_unknown_swap(true);
    }
    rust_memory_clear_nonpresent_ptes((*vma).vm_mm, addr, pte, nr as _);
    *any_skipped = zap_install_uffd_wp_if_needed(vma, addr, pte, nr, details, ptent);
    nr
}
unsafe fn do_zap_pte_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    mut pte: *mut pte_t,
    mut addr: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
    rss: *mut c_int,
    force_flush: *mut bool,
    force_break: *mut bool,
    any_skipped: *mut bool,
) -> c_int {
    let mut ptent = rust_memory_ptep_get(pte);
    let mut max_nr = (end.wrapping_sub(addr) / PAGE_SIZE) as c_int;
    let mut nr = 0;
    if rust_memory_pte_none(ptent) {
        nr = 1;
        while nr < max_nr {
            ptent = rust_memory_ptep_get(pte.offset(nr as isize));
            if !rust_memory_pte_none(ptent) {
                break;
            }
            nr += 1;
        }
        max_nr -= nr;
        if max_nr == 0 {
            return nr;
        }
        pte = pte.offset(nr as isize);
        addr = addr.wrapping_add((nr as c_ulong).wrapping_mul(PAGE_SIZE));
    }
    nr + if rust_memory_pte_present(ptent) {
        zap_present_ptes(
            tlb,
            vma,
            pte,
            ptent,
            max_nr as c_uint,
            addr,
            details,
            rss,
            force_flush,
            force_break,
            any_skipped,
        )
    } else {
        zap_nonpresent_ptes(
            tlb,
            vma,
            pte,
            ptent,
            max_nr as c_uint,
            addr,
            details,
            rss,
            any_skipped,
        )
    }
}
unsafe fn pte_table_reclaim_possible(
    start: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) -> bool {
    cfg!(CONFIG_PT_RECLAIM)
        && !details.is_null()
        && rust_memory_zap_reclaim_pt(details)
        && end.wrapping_sub(start) >= PMD_SIZE
}
unsafe fn zap_empty_pte_table(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    ptl: *mut spinlock_t,
    pmdval: *mut pmd_t,
) -> bool {
    let pml = rust_memory_pmd_lockptr(mm, pmd);
    if ptl != pml && !rust_memory_spin_trylock(pml) {
        return false;
    }
    *pmdval = rust_memory_pmdp_get(pmd);
    rust_memory_pmd_clear(pmd);
    if ptl != pml {
        rust_memory_spin_unlock(pml);
    }
    true
}
unsafe fn zap_pte_table_if_empty(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    addr: c_ulong,
    pmdval: *mut pmd_t,
) -> bool {
    let mut ptl = null_mut();
    let pml = rust_memory_pmd_lock(mm, pmd);
    let start_pte = rust_memory_pte_offset_map_rw_nolock(mm, pmd, addr, pmdval, &mut ptl);
    if !start_pte.is_null() {
        if ptl != pml {
            rust_memory_spin_lock_nested(ptl, RUST_MEMORY_SINGLE_DEPTH_NESTING as _);
        }
        let mut empty = true;
        for i in 0..RUST_MEMORY_PTRS_PER_PTE as usize {
            if !rust_memory_pte_none(rust_memory_ptep_get(start_pte.add(i))) {
                empty = false;
                break;
            }
        }
        if empty {
            rust_memory_pte_unmap(start_pte);
            rust_memory_pmd_clear(pmd);
            if ptl != pml {
                rust_memory_spin_unlock(ptl);
            }
            rust_memory_spin_unlock(pml);
            return true;
        }
        rust_memory_pte_unmap_unlock(start_pte, ptl);
    }
    if ptl != pml {
        rust_memory_spin_unlock(pml);
    }
    false
}
unsafe fn zap_pte_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) -> c_ulong {
    let mut can_reclaim_pt = pte_table_reclaim_possible(addr, end, details);
    let mut force_flush = false;
    let mut force_break = false;
    let mm = (*tlb).mm;
    let start = addr;
    let mut direct_reclaim = true;
    let mut pmdval: pmd_t = zeroed();
    loop {
        rust_memory_tlb_change_page_size(tlb, PAGE_SIZE);
        let mut rss = [0; NR_MM_COUNTERS];
        let mut ptl = null_mut();
        let start_pte = rust_memory_pte_offset_map_lock(mm, pmd, addr, &mut ptl);
        if start_pte.is_null() {
            return addr;
        }
        let mut pte = start_pte;
        rust_memory_flush_tlb_batched_pending(mm);
        rust_memory_lazy_mmu_mode_enable();
        loop {
            let mut any_skipped = false;
            if rust_memory_need_resched() {
                direct_reclaim = false;
                break;
            }
            let nr = do_zap_pte_range(
                tlb,
                vma,
                pte,
                addr,
                end,
                details,
                rss.as_mut_ptr(),
                &mut force_flush,
                &mut force_break,
                &mut any_skipped,
            );
            if any_skipped {
                can_reclaim_pt = false;
            }
            if force_break {
                addr = addr.wrapping_add((nr as c_ulong).wrapping_mul(PAGE_SIZE));
                direct_reclaim = false;
                break;
            }
            pte = pte.offset(nr as isize);
            addr = addr.wrapping_add(PAGE_SIZE.wrapping_mul(nr as c_ulong));
            if addr == end {
                break;
            }
        }
        if can_reclaim_pt && direct_reclaim && addr == end {
            direct_reclaim = zap_empty_pte_table(mm, pmd, ptl, &mut pmdval);
        }
        add_mm_rss_vec(mm, rss.as_mut_ptr());
        rust_memory_lazy_mmu_mode_disable();
        if force_flush {
            rust_memory_tlb_flush_mmu_tlbonly(tlb);
            rust_memory_tlb_flush_rmaps(tlb, vma);
        }
        rust_memory_pte_unmap_unlock(start_pte, ptl);
        if force_flush {
            rust_memory_tlb_flush_mmu(tlb);
        }
        if addr == end {
            break;
        }
        rust_memory_cond_resched();
        force_flush = false;
        force_break = false;
    }
    if can_reclaim_pt && (direct_reclaim || zap_pte_table_if_empty(mm, pmd, start, &mut pmdval)) {
        rust_memory_pte_free_tlb(tlb, rust_memory_pmd_pgtable(pmdval), start);
        rust_memory_mm_dec_nr_ptes(mm);
    }
    addr
}
unsafe fn zap_pmd_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pud: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) -> c_ulong {
    let mut pmd = rust_memory_pmd_offset(pud, addr);
    loop {
        let next = rust_memory_pmd_addr_end(addr, end);
        let mut huge_done = false;
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if rust_memory_pmd_is_huge(*pmd) {
            if next.wrapping_sub(addr) != RUST_MEMORY_HPAGE_PMD_SIZE {
                rust_memory_split_huge_pmd(vma, pmd, addr, false);
            } else if rust_memory_zap_huge_pmd(tlb, vma, pmd, addr) {
                addr = next;
                huge_done = true;
            }
        } else if !details.is_null()
            && !(*details).single_folio.is_null()
            && rust_memory_folio_test_pmd_mappable((*details).single_folio)
            && next.wrapping_sub(addr) == RUST_MEMORY_HPAGE_PMD_SIZE
            && rust_memory_pmd_none(*pmd)
        {
            rust_memory_sync_with_folio_pmd_zap((*tlb).mm, pmd);
        }
        if !huge_done {
            if rust_memory_pmd_none(*pmd) {
                addr = next;
            } else {
                addr = zap_pte_range(tlb, vma, pmd, addr, next, details);
            }
        }
        if addr == next {
            pmd = pmd.add(1);
        }
        rust_memory_cond_resched();
        if addr == end {
            break;
        }
    }
    addr
}
unsafe fn zap_pud_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    p4d: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) -> c_ulong {
    let mut pud = rust_memory_pud_offset(p4d, addr);
    loop {
        let mut next = rust_memory_pud_addr_end(addr, end);
        let mut huge_done = false;
        #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
        if rust_memory_pud_trans_huge(*pud) {
            if next.wrapping_sub(addr) != RUST_MEMORY_HPAGE_PUD_SIZE {
                rust_memory_split_huge_pud(vma, pud, addr);
            } else {
                huge_done = rust_memory_zap_huge_pud(tlb, vma, pud, addr);
            }
        }
        if huge_done {
            rust_memory_cond_resched();
        } else if !rust_memory_pud_none_or_clear_bad(pud) {
            next = zap_pmd_range(tlb, vma, pud, addr, next, details);
            rust_memory_cond_resched();
        }
        pud = pud.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    addr
}
unsafe fn zap_p4d_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pgd: *mut pgd_t,
    mut addr: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) -> c_ulong {
    let mut p4d = rust_memory_p4d_offset(pgd, addr);
    loop {
        let mut next = rust_memory_p4d_addr_end(addr, end);
        if !rust_memory_p4d_none_or_clear_bad(p4d) {
            next = zap_pud_range(tlb, vma, p4d, addr, next, details);
        }
        p4d = p4d.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    addr
}
unsafe fn __zap_vma_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
    details: *mut zap_details,
) {
    let reaping = !details.is_null() && rust_memory_zap_reaping(details);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_zap_range(start >= end || !rust_memory_range_in_vma(vma, start, end));
    if !(*vma).vm_file.is_null() && !reaping {
        rust_memory_uprobe_munmap(vma, start, end);
    }
    if rust_memory_is_vm_hugetlb_page(vma) {
        let flags = if details.is_null() {
            0
        } else {
            (*details).zap_flags
        };
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_hugetlb_reaping(reaping);
        if (*vma).vm_file.is_null() {
            return;
        }
        rust_memory_unmap_hugepage_range(tlb, vma, start, end, null_mut(), flags);
    } else {
        let mut addr = start;
        rust_memory_tlb_start_vma(tlb, vma);
        let mut pgd = rust_memory_pgd_offset((*vma).vm_mm, addr);
        loop {
            let mut next = rust_memory_pgd_addr_end(addr, end);
            if !rust_memory_pgd_none_or_clear_bad(pgd) {
                next = zap_p4d_range(tlb, vma, pgd, addr, next, details);
            }
            pgd = pgd.add(1);
            addr = next;
            if addr == end {
                break;
            }
        }
        rust_memory_tlb_end_vma(tlb, vma);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zap_vma_for_reaping(vma: *mut vm_area_struct) -> c_int {
    let mut details: zap_details = zeroed();
    rust_memory_zap_set_reaping(&mut details, true);
    let mut range: mmu_notifier_range = zeroed();
    let mut tlb: mmu_gather = zeroed();
    rust_memory_mmu_notifier_range_init(
        &mut range,
        RUST_MEMORY_MMU_NOTIFY_CLEAR as _,
        0,
        (*vma).vm_mm,
        rust_memory_vma_start(vma),
        rust_memory_vma_end(vma),
    );
    rust_memory_tlb_gather_mmu(&mut tlb, (*vma).vm_mm);
    if rust_memory_mmu_notifier_invalidate_range_start_nonblock(&mut range) != 0 {
        rust_memory_tlb_finish_mmu(&mut tlb);
        return -(EBUSY as c_int);
    }
    __zap_vma_range(&mut tlb, vma, range.start, range.end, &mut details);
    rust_memory_mmu_notifier_invalidate_range_end(&mut range);
    rust_memory_tlb_finish_mmu(&mut tlb);
    0
}
#[no_mangle]
pub unsafe extern "C" fn unmap_vmas(tlb: *mut mmu_gather, unmap: *mut unmap_desc) {
    let mut vma = (*unmap).first;
    let mut range: mmu_notifier_range = zeroed();
    let mut details: zap_details = zeroed();
    details.zap_flags = (RUST_MEMORY_ZAP_FLAG_DROP_MARKER | RUST_MEMORY_ZAP_FLAG_UNMAP) as _;
    rust_memory_mmu_notifier_range_init(
        &mut range,
        RUST_MEMORY_MMU_NOTIFY_UNMAP as _,
        0,
        (*vma).vm_mm,
        (*unmap).vma_start,
        (*unmap).vma_end,
    );
    rust_memory_mmu_notifier_invalidate_range_start(&mut range);
    loop {
        let mut start = core::cmp::max(rust_memory_vma_start(vma), (*unmap).vma_start);
        let mut end = core::cmp::min(rust_memory_vma_end(vma), (*unmap).vma_end);
        rust_memory_hugetlb_zap_begin(vma, &mut start, &mut end);
        __zap_vma_range(tlb, vma, start, end, &mut details);
        rust_memory_hugetlb_zap_end(vma, &mut details);
        vma = mas_find((*unmap).mas, (*unmap).tree_end.wrapping_sub(1)) as *mut vm_area_struct;
        if vma.is_null() {
            break;
        }
    }
    rust_memory_mmu_notifier_invalidate_range_end(&mut range);
}
#[no_mangle]
pub unsafe extern "C" fn zap_vma_range_batched(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    address: c_ulong,
    size: c_ulong,
    details: *mut zap_details,
) {
    let end = address.wrapping_add(size);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_zap_tlb(tlb.is_null() || (*tlb).mm != (*vma).vm_mm);
    if size == 0 {
        return;
    }
    let mut range: mmu_notifier_range = zeroed();
    rust_memory_mmu_notifier_range_init(
        &mut range,
        RUST_MEMORY_MMU_NOTIFY_CLEAR as _,
        0,
        (*vma).vm_mm,
        address,
        end,
    );
    rust_memory_hugetlb_zap_begin(vma, addr_of_mut!(range.start), addr_of_mut!(range.end));
    rust_memory_update_hiwater_rss((*vma).vm_mm);
    rust_memory_mmu_notifier_invalidate_range_start(&mut range);
    __zap_vma_range(tlb, vma, address, end, details);
    rust_memory_mmu_notifier_invalidate_range_end(&mut range);
    if rust_memory_is_vm_hugetlb_page(vma) {
        rust_memory_tlb_finish_mmu(tlb);
        rust_memory_hugetlb_zap_end(vma, details);
        rust_memory_tlb_gather_mmu(tlb, (*vma).vm_mm);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zap_vma_range(vma: *mut vm_area_struct, address: c_ulong, size: c_ulong) {
    let mut tlb: mmu_gather = zeroed();
    rust_memory_tlb_gather_mmu(&mut tlb, (*vma).vm_mm);
    zap_vma_range_batched(&mut tlb, vma, address, size, null_mut());
    rust_memory_tlb_finish_mmu(&mut tlb);
}
#[no_mangle]
pub unsafe extern "C" fn zap_special_vma_range(
    vma: *mut vm_area_struct,
    address: c_ulong,
    size: c_ulong,
) {
    if !rust_memory_range_in_vma(vma, address, address.wrapping_add(size))
        || rust_memory_vma_vm_flags(vma) & (RUST_MEMORY_VM_PFNMAP | RUST_MEMORY_VM_MIXEDMAP) == 0
    {
        return;
    }
    zap_vma_range(vma, address, size);
}
