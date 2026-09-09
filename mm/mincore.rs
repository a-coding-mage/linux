// SPDX-License-Identifier: GPL-2.0
/*
 *	linux/mm/mincore.c
 *
 * Copyright (C) 1994-2006  Linus Torvalds
 */

/*
 * The mincore() system call.
 */

unsafe fn mincore_hugetlb(
    pte: *mut pte_t,
    _hmask: c_ulong,
    addr: c_ulong,
    end: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        let nr: c_ulong = (end - addr) >> PAGE_SHIFT;
        let resident: c_uchar;
        let ptl: *mut spinlock_t;
        let ptep: pte_t;

        ptl = huge_pte_lock(hstate_vma((*walk).vma), (*walk).mm, pte);
        ptep = huge_ptep_get((*walk).mm, addr, pte);
        resident = (!huge_pte_none(ptep) && !pte_is_marker(ptep)) as c_uchar;
        memset((*walk).private as *mut c_void, resident as c_int, nr as usize);
        (*walk).private = (*walk).private.add(nr as usize);
        spin_unlock(ptl);
    }
    #[cfg(not(CONFIG_HUGETLB_PAGE))]
    {
        BUG();
    }
    0
}

unsafe fn mincore_swap(entry: swp_entry_t, shmem: bool) -> c_uchar {
    let mut si: *mut swap_info_struct;
    let mut folio: *mut folio = core::ptr::null_mut();
    let mut present: c_uchar = 0;

    /*
     * Shmem mapping may contain swapin error entries, which are
     * absent. Page table may contain migration or hwpoison
     * entries which are always uptodate.
     */
    if !softleaf_is_swap(entry) {
        return (!shmem) as c_uchar;
    }
    if !IS_ENABLED(CONFIG_SWAP) {
        WARN_ON(1);
        return 0;
    }
    /*
     * Shmem mapping lookup is lockless, so we need to grab the swap
     * device. mincore page table walk locks the PTL, and the swap
     * device is stable, avoid touching the si for better performance.
     */
    if shmem {
        si = get_swap_device(entry);
        if si.is_null() {
            return 0;
        }
    }
    folio = swap_cache_get_folio(entry);
    if shmem {
        put_swap_device(si);
    }
    if !folio.is_null() {
        present = folio_test_uptodate(folio) as c_uchar;
        folio_put(folio);
    }
    present
}

/*
 * Later we can get more picky about what "in core" means precisely.
 * For now, simply check to see if the page is in the page cache,
 * and is up to date; i.e. that no page-in operation would be required
 * at this time if an application were to map and access this page.
 */
unsafe fn mincore_page(mapping: *mut address_space, index: pgoff_t) -> c_uchar {
    let folio = filemap_get_entry(mapping, index);
    if folio.is_null() {
        return 0;
    }
    if xa_is_value(folio) {
        if !shmem_mapping(mapping) {
            return 0;
        }
        return mincore_swap(radix_to_swp_entry(folio), true);
    }
    let present = folio_test_uptodate(folio) as c_uchar;
    folio_put(folio);
    present
}

unsafe fn __mincore_unmapped_range(
    addr: c_ulong,
    end: c_ulong,
    vma: *mut vm_area_struct,
    vec: *mut c_uchar,
) -> c_int {
    let nr = (end - addr) >> PAGE_SHIFT;
    if !(*vma).vm_file.is_null() {
        let mut pgoff = linear_page_index(vma, addr);
        for i in 0..nr as usize {
            *vec.add(i) = mincore_page((*(*vma).vm_file).f_mapping, pgoff);
            pgoff += 1;
        }
    } else {
        for i in 0..nr as usize {
            *vec.add(i) = 0;
        }
    }
    nr as c_int
}

unsafe fn mincore_unmapped_range(
    addr: c_ulong,
    end: c_ulong,
    _depth: c_int,
    walk: *mut mm_walk,
) -> c_int {
    (*walk).private = (*walk).private.add(__mincore_unmapped_range(
        addr, end, (*walk).vma, (*walk).private,
    ) as usize);
    0
}

unsafe fn mincore_pud_entry(
    pudp: *mut pud_t,
    addr: c_ulong,
    end: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    if pud_is_huge(pudp_get(pudp)) {
        let nr = (end - addr) >> PAGE_SHIFT;
        memset((*walk).private as *mut c_void, 1, nr as usize);
        (*walk).private = (*walk).private.add(nr as usize);
        (*walk).action = ACTION_CONTINUE;
    }
    0
}

unsafe fn mincore_pte_range(
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let vma = (*walk).vma;
    let mut vec = (*walk).private as *mut c_uchar;
    let nr = ((end - addr) >> PAGE_SHIFT) as c_int;
    let mut ptl: *mut spinlock_t;
    ptl = pmd_trans_huge_lock(pmd, vma);
    if !ptl.is_null() {
        memset(vec as *mut c_void, 1, nr as usize);
        spin_unlock(ptl);
    } else {
        let mut ptep = pte_offset_map_lock((*walk).mm, pmd, addr, &mut ptl);
        if ptep.is_null() {
            (*walk).action = ACTION_AGAIN;
            return 0;
        }
        while addr != end {
            let pte = ptep_get(ptep);
            let mut step: c_ulong = 1;
            if pte_none(pte) || pte_is_marker(pte) {
                __mincore_unmapped_range(addr, addr + PAGE_SIZE, vma, vec);
            } else if pte_present(pte) {
                let batch = pte_batch_hint(ptep, pte);
                if batch > 1 {
                    let max_nr = (end - addr) >> PAGE_SHIFT;
                    step = core::cmp::min(batch as c_ulong, max_nr);
                }
                for i in 0..step as usize {
                    *vec.add(i) = 1;
                }
            } else {
                let entry = softleaf_from_pte(pte);
                *vec = mincore_swap(entry, false);
            }
            ptep = ptep.add(step as usize);
            addr += step * PAGE_SIZE;
            vec = vec.add(step as usize);
        }
        pte_unmap_unlock(ptep.sub(1), ptl);
    }
    (*walk).private = (*walk).private.add(nr as usize);
    cond_resched();
    0
}

unsafe fn can_do_mincore(vma: *mut vm_area_struct) -> bool {
    if vma_is_anonymous(vma) {
        return true;
    }
    if (*vma).vm_file.is_null() {
        return false;
    }
    file_owner_or_capable((*vma).vm_file)
        || file_permission((*vma).vm_file, MAY_WRITE) == 0
}

static mincore_walk_ops: mm_walk_ops = mm_walk_ops {
    pud_entry: Some(mincore_pud_entry),
    pmd_entry: Some(mincore_pte_range),
    pte_hole: Some(mincore_unmapped_range),
    hugetlb_entry: Some(mincore_hugetlb),
    walk_lock: PGWALK_RDLOCK,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
