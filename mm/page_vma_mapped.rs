// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux MM, rmap, hugetlb, swap, leafops, and
// internal interfaces are intentionally referenced but not defined here.

unsafe fn not_found(pvmw: *mut page_vma_mapped_walk) -> bool {
    page_vma_mapped_walk_done(pvmw);
    false
}

unsafe fn map_pte(
    pvmw: *mut page_vma_mapped_walk,
    pmdvalp: *mut pmd_t,
    ptlp: *mut *mut spinlock_t,
) -> bool {
    let is_migration: bool;
    let mut ptent: pte_t;

    if (*pvmw).flags & PVMW_SYNC != 0 {
        /* Use the stricter lookup */
        (*pvmw).pte = pte_offset_map_lock((*pvmw).vma.vm_mm, (*pvmw).pmd,
                                          (*pvmw).address, &mut (*pvmw).ptl);
        *ptlp = (*pvmw).ptl;
        return !(*pvmw).pte.is_null();
    }

    is_migration = (*pvmw).flags & PVMW_MIGRATION != 0;
    'again: loop {
        /*
         * It is important to return the ptl corresponding to pte,
         * in case *pvmw->pmd changes underneath us; so we need to
         * return it even when choosing not to lock, in case caller
         * proceeds to loop over next ptes, and finds a match later.
         * Though, in most cases, page lock already protects this.
         */
        (*pvmw).pte = pte_offset_map_rw_nolock((*pvmw).vma.vm_mm, (*pvmw).pmd,
                                               (*pvmw).address, pmdvalp, ptlp);
        if (*pvmw).pte.is_null() { return false; }
        ptent = ptep_get_lockless((*pvmw).pte);
        if pte_none(ptent) {
            return false;
        } else if pte_present(ptent) {
            if is_migration { return false; }
        } else if !is_migration {
            let entry = softleaf_from_pte(ptent);
            /* Handle un-addressable ZONE_DEVICE memory */
            if !softleaf_is_device_private(entry) && !softleaf_is_device_exclusive(entry) {
                return false;
            }
        }
        spin_lock(*ptlp);
        if !pmd_same(*pmdvalp, pmdp_get_lockless((*pvmw).pmd)) {
            pte_unmap_unlock((*pvmw).pte, *ptlp);
            continue 'again;
        }
        (*pvmw).ptl = *ptlp;
        return true;
    }
}

/* Check whether the PTE maps the requested PFN range. */
unsafe fn check_pte(pvmw: *mut page_vma_mapped_walk, pte_nr: c_ulong) -> bool {
    let pfn: c_ulong;
    let ptent = if is_vm_hugetlb_page((*pvmw).vma) {
        huge_ptep_get((*pvmw).vma.vm_mm, (*pvmw).address, (*pvmw).pte)
    } else { ptep_get((*pvmw).pte) };
    if (*pvmw).flags & PVMW_MIGRATION != 0 {
        let entry = softleaf_from_pte(ptent);
        if !softleaf_is_migration(entry) { return false; }
        pfn = softleaf_to_pfn(entry);
    } else if pte_present(ptent) {
        pfn = pte_pfn(ptent);
    } else {
        let entry = softleaf_from_pte(ptent);
        if !softleaf_is_device_private(entry) && !softleaf_is_device_exclusive(entry) { return false; }
        pfn = softleaf_to_pfn(entry);
    }
    if pfn + pte_nr - 1 < (*pvmw).pfn { return false; }
    if pfn > (*pvmw).pfn + (*pvmw).nr_pages - 1 { return false; }
    true
}

/* Returns true if the two ranges overlap. Careful to not overflow. */
unsafe fn check_pmd(pfn: c_ulong, pvmw: *mut page_vma_mapped_walk) -> bool {
    if pfn + HPAGE_PMD_NR - 1 < (*pvmw).pfn { return false; }
    if pfn > (*pvmw).pfn + (*pvmw).nr_pages - 1 { return false; }
    true
}

unsafe fn step_forward(pvmw: *mut page_vma_mapped_walk, size: c_ulong) {
    (*pvmw).address = ((*pvmw).address + size) & !(size - 1);
    if (*pvmw).address == 0 { (*pvmw).address = ULONG_MAX; }
}

unsafe fn page_vma_mapped_walk(pvmw: *mut page_vma_mapped_walk) -> bool {
    let vma = (*pvmw).vma;
    let mm = (*vma).vm_mm;
    let end: c_ulong;
    let mut ptl: *mut spinlock_t;
    let mut pteval: pte_t;
    let mut pgd: *mut pgd_t;
    let mut p4d: *mut p4d_t;
    let mut pud: *mut pud_t;
    let mut pmde: pmd_t;

    if !(*pvmw).pmd.is_null() && (*pvmw).pte.is_null() { return not_found(pvmw); }
    if is_vm_hugetlb_page(vma) {
        let hstate = hstate_vma(vma);
        let size = huge_page_size(hstate);
        if !(*pvmw).pte.is_null() { return not_found(pvmw); }
        (*pvmw).pte = hugetlb_walk(vma, (*pvmw).address, size);
        if (*pvmw).pte.is_null() { return false; }
        (*pvmw).ptl = huge_pte_lock(hstate, mm, (*pvmw).pte);
        if !check_pte(pvmw, pages_per_huge_page(hstate)) { return not_found(pvmw); }
        return true;
    }
    end = vma_address_end(pvmw);
    // When a PTE is already mapped, the following walk continues at the next
    // PTE; the loop below performs that same continuation after remapping.
    'restart: loop {
        loop {
            pgd = pgd_offset(mm, (*pvmw).address);
            if !pgd_present(*pgd) { step_forward(pvmw, PGDIR_SIZE); continue; }
            p4d = p4d_offset(pgd, (*pvmw).address);
            if !p4d_present(*p4d) { step_forward(pvmw, P4D_SIZE); continue; }
            pud = pud_offset(p4d, (*pvmw).address);
            if !pud_present(*pud) { step_forward(pvmw, PUD_SIZE); continue; }
            (*pvmw).pmd = pmd_offset(pud, (*pvmw).address);
            pmde = pmdp_get_lockless((*pvmw).pmd);
            if IS_ENABLED(CONFIG_TRANSPARENT_HUGEPAGE) && (pmd_trans_huge(pmde) || pmd_is_migration_entry(pmde) || pmd_is_device_private_entry(pmde)) {
                (*pvmw).ptl = pmd_lock(mm, (*pvmw).pmd);
                pmde = *(*pvmw).pmd;
                if pmd_is_migration_entry(pmde) {
                    if (*pvmw).flags & PVMW_MIGRATION == 0 { return not_found(pvmw); }
                    if !check_pmd(softleaf_to_pfn(softleaf_from_pmd(pmde)), pvmw) { return not_found(pvmw); }
                    return true;
                } else if pmd_is_device_private_entry(pmde) {
                    if (*pvmw).flags & PVMW_MIGRATION != 0 { return not_found(pvmw); }
                    if !check_pmd(softleaf_to_pfn(softleaf_from_pmd(pmde)), pvmw) { return not_found(pvmw); }
                    return true;
                } else if !pmd_present(pmde) { return not_found(pvmw); }
                if pmd_trans_huge(pmde) {
                    if (*pvmw).flags & PVMW_MIGRATION != 0 || !check_pmd(pmd_pfn(pmde), pvmw) { return not_found(pvmw); }
                    return true;
                }
                spin_unlock((*pvmw).ptl); (*pvmw).ptl = core::ptr::null_mut();
            } else if !pmd_present(pmde) {
                if (*pvmw).flags & PVMW_SYNC != 0 && thp_vma_suitable_order(vma, (*pvmw).address, PMD_ORDER) && (*pvmw).nr_pages >= HPAGE_PMD_NR { sync_with_folio_pmd_zap(mm, (*pvmw).pmd); }
                step_forward(pvmw, PMD_SIZE); continue;
            }
            if !map_pte(pvmw, &mut pmde, &mut ptl) {
                if (*pvmw).pte.is_null() { continue 'restart; }
            }
            if check_pte(pvmw, 1) { return true; }
            'next_pte: loop {
                (*pvmw).address += PAGE_SIZE;
                if (*pvmw).address >= end { return not_found(pvmw); }
                if ((*pvmw).address & (PMD_SIZE - PAGE_SIZE)) == 0 {
                    if !(*pvmw).ptl.is_null() { spin_unlock((*pvmw).ptl); (*pvmw).ptl = core::ptr::null_mut(); }
                    pte_unmap((*pvmw).pte); (*pvmw).pte = core::ptr::null_mut();
                    (*pvmw).flags |= PVMW_PGTABLE_CROSSED;
                    continue 'restart;
                }
                (*pvmw).pte = (*pvmw).pte.add(1);
                pteval = if (*pvmw).ptl.is_null() { ptep_get_lockless((*pvmw).pte) } else { ptep_get((*pvmw).pte) };
                if pte_none(pteval) { continue 'next_pte; }
                if (*pvmw).ptl.is_null() {
                    spin_lock(ptl);
                    if !pmd_same(pmde, pmdp_get_lockless((*pvmw).pmd)) { pte_unmap_unlock((*pvmw).pte, ptl); (*pvmw).pte = core::ptr::null_mut(); continue 'restart; }
                    (*pvmw).ptl = ptl;
                }
                if check_pte(pvmw, 1) { return true; }
            }
        }
    }
}

#[cfg(CONFIG_MEMORY_FAILURE)]
unsafe fn page_mapped_in_vma(page: *const page, vma: *mut vm_area_struct) -> c_ulong {
    let folio = page_folio(page);
    let pgoff = page_pgoff(folio, page);
    let mut pvmw = page_vma_mapped_walk {
        pfn: page_to_pfn(page), nr_pages: 1, vma, flags: PVMW_SYNC,
        address: 0, pmd: core::ptr::null_mut(), pte: core::ptr::null_mut(),
        ptl: core::ptr::null_mut(),
    };
    pvmw.address = if folio_test_anon(folio) {
        vma_anon_address(vma, pgoff, 1)
    } else { vma_filebacked_address(vma, pgoff, 1) };
    if pvmw.address == (-EFAULT as c_ulong) { return pvmw.address; }
    if !page_vma_mapped_walk(&mut pvmw) { return -EFAULT as c_ulong; }
    page_vma_mapped_walk_done(&mut pvmw);
    pvmw.address
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
