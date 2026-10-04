// SPDX-License-Identifier: GPL-2.0
#[no_mangle]
pub unsafe extern "C" fn page_address_in_vma(
    folio: *const folio,
    page: *const page,
    vma: *const vm_area_struct,
) -> c_ulong {
    if rust_rmap_folio_test_anon(folio) {
        let av = rust_rmap_folio_anon_vma(folio);
        if (*vma).anon_vma.is_null() || av.is_null() || (*(*vma).anon_vma).root != (*av).root {
            return (-(EFAULT as c_long)) as c_ulong;
        }
        return rust_rmap_vma_anon_address(vma, rust_rmap_page_pgoff(folio, page), 1);
    } else if (*vma).vm_file.is_null()
        || rust_rmap_file_mapping((*vma).vm_file) != rust_rmap_folio_mapping_field(folio)
    {
        return (-(EFAULT as c_long)) as c_ulong;
    }
    rust_rmap_vma_filebacked_address(vma, rust_rmap_page_pgoff(folio, page), 1)
}
#[no_mangle]
pub unsafe extern "C" fn mm_find_pmd(mm: *mut mm_struct, address: c_ulong) -> *mut pmd_t {
    let pgd = rust_rmap_pgd_offset(mm, address);
    if !rust_rmap_pgd_present(*pgd) {
        return null_mut();
    }
    let p4d = rust_rmap_p4d_offset(pgd, address);
    if !rust_rmap_p4d_present(*p4d) {
        return null_mut();
    }
    let pud = rust_rmap_pud_offset(p4d, address);
    if !rust_rmap_pud_present(*pud) {
        return null_mut();
    }
    rust_rmap_pmd_offset(pud, address)
}
unsafe fn folio_vma_walk(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: c_uint,
) -> page_vma_mapped_walk {
    let mut walk: page_vma_mapped_walk = zeroed();
    walk.pfn = rust_rmap_folio_pfn(folio);
    walk.nr_pages = rust_rmap_folio_nr_pages(folio) as c_ulong;
    walk.pgoff = rust_rmap_folio_pgoff(folio);
    walk.vma = vma;
    walk.address = address;
    walk.flags = flags;
    rust_rmap_pvmw_set_pgoff_is_anon(&mut walk, rust_rmap_folio_test_anon(folio));
    walk
}
struct FolioReferencedArg {
    mapcount: c_int,
    referenced: c_int,
    vma_flags: vma_flags_t,
    memcg: *mut mem_cgroup,
}
unsafe extern "C" fn folio_referenced_one(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    mut address: c_ulong,
    arg: *mut c_void,
) -> bool {
    let pra = arg.cast::<FolioReferencedArg>();
    let mut pvmw = folio_vma_walk(folio, vma, address, 0);
    let mut ptes: c_int = 0;
    let mut referenced: c_int = 0;
    while rust_rmap_page_vma_mapped_walk(&mut pvmw) {
        address = pvmw.address;
        let mut nr: c_uint = 1;
        if rust_rmap_vma_test_locked(vma) {
            ptes = ptes.wrapping_add(1);
            (*pra).mapcount = (*pra).mapcount.wrapping_sub(1);
            if !pvmw.pte.is_null() && ptes as c_ulong != pvmw.nr_pages {
                continue;
            }
            if pvmw.flags & RUST_RMAP_PVMW_PGTABLE_CROSSED as c_uint != 0 {
                continue;
            }
            rust_rmap_mlock_vma_folio(folio, vma);
            rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
            rust_rmap_vma_flags_set_locked(addr_of_mut!((*pra).vma_flags));
            return false;
        }
        if (rust_rmap_mm_users((*vma).vm_mm) == 0
            || rust_rmap_check_stable_address_space((*vma).vm_mm) != 0)
            && rust_rmap_folio_test_anon(folio)
            && rust_rmap_folio_test_swapbacked(folio)
            && !rust_rmap_folio_maybe_mapped_shared(folio)
        {
            (*pra).referenced = -1;
            rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
            return false;
        }
        if !pvmw.pte.is_null() && rust_rmap_folio_test_large(folio) {
            let end = rust_rmap_pmd_addr_end(address, rust_rmap_vma_end(vma));
            let max_nr = (end.wrapping_sub(address) >> RUST_RMAP_PAGE_SHIFT) as c_uint;
            nr = rust_rmap_folio_pte_batch(folio, pvmw.pte, rust_rmap_ptep_get(pvmw.pte), max_nr);
        }
        if rust_rmap_lru_gen_enabled() && !rust_rmap_lru_gen_switching() && !pvmw.pte.is_null() {
            if rust_rmap_lru_gen_look_around(&mut pvmw, nr) {
                referenced = referenced.wrapping_add(1);
            }
        } else if !pvmw.pte.is_null() {
            if rust_rmap_clear_flush_young_ptes_notify(vma, address, pvmw.pte, nr) != 0 {
                referenced = referenced.wrapping_add(1);
            }
        } else {
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            {
                if rust_rmap_pmdp_clear_flush_young_notify(vma, address, pvmw.pmd) != 0 {
                    referenced = referenced.wrapping_add(1);
                }
            }
            #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
            {
                rust_rmap_diag_referenced_pmd();
            }
        }
        ptes = ptes.wrapping_add(nr as c_int);
        (*pra).mapcount = (*pra).mapcount.wrapping_sub(nr as c_int);
        if ptes as c_ulong == pvmw.nr_pages {
            rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
            break;
        }
        // Original pointer arithmetic is performed even for a PMD walk (nr=1).
        pvmw.pte = pvmw.pte.wrapping_add(nr.wrapping_sub(1) as usize);
        pvmw.address = pvmw.address.wrapping_add(
            (nr.wrapping_sub(1) as c_ulong).wrapping_mul(RUST_RMAP_PAGE_SIZE as c_ulong),
        );
    }
    if referenced != 0 {
        rust_rmap_folio_clear_idle(folio);
    }
    if rust_rmap_folio_test_clear_young(folio) {
        referenced = referenced.wrapping_add(1);
    }
    if referenced != 0 {
        let mut flags = rust_rmap_vma_flags_read(vma);
        (*pra).referenced = (*pra).referenced.wrapping_add(1);
        rust_rmap_vma_flags_clear_locked(&mut flags);
        rust_rmap_vma_flags_set_mask(addr_of_mut!((*pra).vma_flags), flags);
    }
    (*pra).mapcount != 0
}
unsafe extern "C" fn invalid_folio_referenced_vma(
    vma: *mut vm_area_struct,
    arg: *mut c_void,
) -> bool {
    let memcg = (*(arg.cast::<FolioReferencedArg>())).memcg;
    !rust_rmap_vma_has_recency(vma)
        || (!memcg.is_null() && !rust_rmap_mm_match_cgroup((*vma).vm_mm, memcg))
}
#[no_mangle]
pub unsafe extern "C" fn folio_referenced(
    folio: *mut folio,
    is_locked: c_int,
    memcg: *mut mem_cgroup,
    flags: *mut vma_flags_t,
) -> c_int {
    let mut pra = FolioReferencedArg {
        mapcount: rust_rmap_folio_mapcount(folio),
        referenced: 0,
        vma_flags: zeroed(),
        memcg,
    };
    let mut rwc: rmap_walk_control = zeroed();
    rwc.rmap_one = Some(folio_referenced_one);
    rwc.arg = addr_of_mut!(pra).cast();
    rwc.anon_lock = Some(folio_lock_anon_vma_read);
    rwc.try_lock = true;
    rwc.invalid_vma = Some(invalid_folio_referenced_vma);
    rust_rmap_diag_referenced_device(folio);
    rust_rmap_vma_flags_clear_all(flags);
    if pra.mapcount == 0 || rust_rmap_folio_raw_mapping(folio).is_null() {
        return 0;
    }
    let mut we_locked = false;
    if is_locked == 0 {
        we_locked = rust_rmap_folio_trylock(folio);
        if !we_locked {
            return 1;
        }
    }
    rmap_walk(folio, &mut rwc);
    rust_rmap_vma_flags_set_mask(flags, pra.vma_flags);
    if we_locked {
        rust_rmap_folio_unlock(folio);
    }
    if rwc.contended {
        -1
    } else {
        pra.referenced
    }
}
unsafe fn page_vma_mkclean_one(pvmw: *mut page_vma_mapped_walk) -> c_int {
    let vma = (*pvmw).vma;
    let mm = (*vma).vm_mm;
    let mut cleaned: c_int = 0;
    let mut range: mmu_notifier_range = zeroed();
    rust_rmap_mmu_notifier_range_init(
        &mut range,
        MMU_NOTIFY_PROTECTION_PAGE,
        0,
        mm,
        (*pvmw).address,
        rust_rmap_vma_address_end(pvmw),
    );
    rust_rmap_mmu_notifier_invalidate_range_start(&mut range);
    while rust_rmap_page_vma_mapped_walk(pvmw) {
        let address = (*pvmw).address;
        if !(*pvmw).pte.is_null() {
            let pte = (*pvmw).pte;
            let mut entry = rust_rmap_ptep_get(pte);
            if !rust_rmap_pte_present(entry)
                || (!rust_rmap_pte_dirty(entry) && !rust_rmap_pte_write(entry))
            {
                continue;
            }
            rust_rmap_flush_cache_page(vma, address, rust_rmap_pte_pfn(entry));
            entry = rust_rmap_ptep_clear_flush(vma, address, pte);
            entry = rust_rmap_pte_wrprotect(entry);
            entry = rust_rmap_pte_mkclean(entry);
            rust_rmap_set_pte_at(mm, address, pte, entry);
            cleaned = cleaned.wrapping_add(1);
        } else {
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            {
                let pmd = (*pvmw).pmd;
                let mut entry = rust_rmap_pmdp_get(pmd);
                if !rust_rmap_pmd_present(entry)
                    || (!rust_rmap_pmd_dirty(entry) && !rust_rmap_pmd_write(entry))
                {
                    continue;
                }
                rust_rmap_flush_cache_range(
                    vma,
                    address,
                    address.wrapping_add(RUST_RMAP_HPAGE_PMD_SIZE as c_ulong),
                );
                entry = rust_rmap_pmdp_invalidate(vma, address, pmd);
                entry = rust_rmap_pmd_wrprotect(entry);
                entry = rust_rmap_pmd_mkclean(entry);
                rust_rmap_set_pmd_at(mm, address, pmd, entry);
                cleaned = cleaned.wrapping_add(1);
            }
            #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
            {
                rust_rmap_diag_mkclean_pmd();
            }
        }
    }
    rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
    cleaned
}
unsafe extern "C" fn page_mkclean_one(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    arg: *mut c_void,
) -> bool {
    let mut pvmw = folio_vma_walk(folio, vma, address, RUST_RMAP_PVMW_SYNC as c_uint);
    let cleaned = arg.cast::<c_int>();
    let more = page_vma_mkclean_one(&mut pvmw);
    *cleaned = (*cleaned).wrapping_add(more);
    true
}
unsafe extern "C" fn invalid_mkclean_vma(vma: *mut vm_area_struct, _arg: *mut c_void) -> bool {
    rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_SHARED as c_ulong == 0
}
#[no_mangle]
pub unsafe extern "C" fn folio_mkclean(folio: *mut folio) -> c_int {
    let mut cleaned: c_int = 0;
    let mut rwc: rmap_walk_control = zeroed();
    rwc.arg = addr_of_mut!(cleaned).cast();
    rwc.rmap_one = Some(page_mkclean_one);
    rwc.invalid_vma = Some(invalid_mkclean_vma);
    rust_rmap_diag_mkclean_locked(folio);
    if !rust_rmap_folio_mapped(folio) || rust_rmap_folio_mapping(folio).is_null() {
        return 0;
    }
    rmap_walk(folio, &mut rwc);
    cleaned
}
struct WrprotectFileState {
    cleaned: c_int,
    pgoff: Pgoff,
    pfn: c_ulong,
    nr_pages: c_ulong,
}
unsafe extern "C" fn mapping_wrprotect_range_one(
    _folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    arg: *mut c_void,
) -> bool {
    let state = arg.cast::<WrprotectFileState>();
    let mut pvmw: page_vma_mapped_walk = zeroed();
    pvmw.pfn = (*state).pfn;
    pvmw.nr_pages = (*state).nr_pages;
    pvmw.pgoff = (*state).pgoff;
    pvmw.vma = vma;
    pvmw.address = address;
    pvmw.flags = RUST_RMAP_PVMW_SYNC as c_uint;
    rust_rmap_pvmw_set_pgoff_is_anon(&mut pvmw, false);
    let more = page_vma_mkclean_one(&mut pvmw);
    (*state).cleaned = (*state).cleaned.wrapping_add(more);
    true
}
#[no_mangle]
pub unsafe extern "C" fn mapping_wrprotect_range(
    mapping: *mut address_space,
    pgoff: Pgoff,
    pfn: c_ulong,
    nr_pages: c_ulong,
) -> c_int {
    let mut state = WrprotectFileState {
        cleaned: 0,
        pgoff,
        pfn,
        nr_pages,
    };
    let mut rwc: rmap_walk_control = zeroed();
    rwc.arg = addr_of_mut!(state).cast();
    rwc.rmap_one = Some(mapping_wrprotect_range_one);
    rwc.invalid_vma = Some(invalid_mkclean_vma);
    if mapping.is_null() {
        return 0;
    }
    __rmap_walk_file(null_mut(), mapping, pgoff, nr_pages, &mut rwc, false);
    state.cleaned
}
#[no_mangle]
pub unsafe extern "C" fn pfn_mkclean_range(
    pfn: c_ulong,
    nr_pages: c_ulong,
    pgoff: Pgoff,
    vma: *mut vm_area_struct,
) -> c_int {
    let mut pvmw: page_vma_mapped_walk = zeroed();
    pvmw.pfn = pfn;
    pvmw.nr_pages = nr_pages;
    pvmw.pgoff = pgoff;
    pvmw.vma = vma;
    pvmw.flags = RUST_RMAP_PVMW_SYNC as c_uint;
    rust_rmap_pvmw_set_pgoff_is_anon(&mut pvmw, false);
    if invalid_mkclean_vma(vma, null_mut()) {
        return 0;
    }
    pvmw.address = rust_rmap_vma_filebacked_address(vma, pgoff, nr_pages);
    rust_rmap_diag_pfn_mkclean_address(pvmw.address, vma);
    page_vma_mkclean_one(&mut pvmw)
}
unsafe fn rmap_walk_anon_lock(folio: *const folio, rwc: *mut rmap_walk_control) -> *mut anon_vma {
    if let Some(lock) = (*rwc).anon_lock {
        return lock(folio, rwc);
    }
    let av = rust_rmap_folio_anon_vma(folio);
    if av.is_null() {
        return null_mut();
    }
    if rust_rmap_anon_vma_trylock_read(av) {
        return av;
    }
    if (*rwc).try_lock {
        (*rwc).contended = true;
        return null_mut();
    }
    rust_rmap_anon_vma_lock_read(av);
    av
}
unsafe fn rmap_walk_anon(folio: *mut folio, rwc: *mut rmap_walk_control, locked: bool) {
    rust_rmap_diag_walk_anon_locked(folio);
    let av = if locked {
        let av = rust_rmap_folio_anon_vma(folio);
        rust_rmap_diag_walk_anon_present(folio, av);
        av
    } else {
        rmap_walk_anon_lock(folio, rwc)
    };
    if av.is_null() {
        return;
    }
    let start = rust_rmap_folio_pgoff(folio);
    let end = start
        .wrapping_add(rust_rmap_folio_nr_pages(folio) as c_ulong)
        .wrapping_sub(1);
    let mut avc = rust_rmap_anon_rmap_tree_iter_first(av, start, end);
    while !avc.is_null() {
        let vma = (*avc).vma;
        let address =
            rust_rmap_vma_anon_address(vma, start, rust_rmap_folio_nr_pages(folio) as c_ulong);
        rust_rmap_diag_walk_anon_address(address, vma);
        rust_rmap_cond_resched();
        let invalid = match (*rwc).invalid_vma {
            Some(check) => check(vma, (*rwc).arg),
            None => false,
        };
        if !invalid {
            if !((*rwc).rmap_one.unwrap_unchecked())(folio, vma, address, (*rwc).arg) {
                break;
            }
            if let Some(done) = (*rwc).done {
                if done(folio) != 0 {
                    break;
                }
            }
        }
        avc = rust_rmap_anon_rmap_tree_iter_next(avc, start, end);
    }
    if !locked {
        rust_rmap_anon_vma_unlock_read(av);
    }
}
unsafe fn __rmap_walk_file(
    folio: *mut folio,
    mapping: *mut address_space,
    start: Pgoff,
    nr_pages: c_ulong,
    rwc: *mut rmap_walk_control,
    locked: bool,
) {
    let end = start.wrapping_add(nr_pages).wrapping_sub(1);
    rust_rmap_diag_walk_file_mapping_1(folio, mapping, start, nr_pages);
    rust_rmap_diag_walk_file_mapping_2(folio, mapping, start, nr_pages);
    rust_rmap_diag_walk_file_mapping_3(folio, mapping, start, nr_pages);
    if !locked && !rust_rmap_i_mmap_trylock_read(mapping) {
        if (*rwc).try_lock {
            (*rwc).contended = true;
            return;
        }
        rust_rmap_i_mmap_lock_read(mapping);
    }
    let mut vma = rust_rmap_mapping_rmap_tree_iter_first(mapping, start, end);
    while !vma.is_null() {
        let address = rust_rmap_vma_filebacked_address(vma, start, nr_pages);
        rust_rmap_diag_walk_file_address(address, vma);
        rust_rmap_cond_resched();
        let invalid = match (*rwc).invalid_vma {
            Some(check) => check(vma, (*rwc).arg),
            None => false,
        };
        if !invalid {
            if !((*rwc).rmap_one.unwrap_unchecked())(folio, vma, address, (*rwc).arg) {
                break;
            }
            if let Some(done) = (*rwc).done {
                if done(folio) != 0 {
                    break;
                }
            }
        }
        vma = rust_rmap_mapping_rmap_tree_iter_next(vma, start, end);
    }
    if !locked {
        rust_rmap_i_mmap_unlock_read(mapping);
    }
}
unsafe fn rmap_walk_file(folio: *mut folio, rwc: *mut rmap_walk_control, locked: bool) {
    rust_rmap_diag_walk_file_locked(folio);
    let mapping = rust_rmap_folio_mapping_field(folio);
    if mapping.is_null() {
        return;
    }
    __rmap_walk_file(
        folio,
        mapping,
        rust_rmap_folio_index(folio),
        rust_rmap_folio_nr_pages(folio) as c_ulong,
        rwc,
        locked,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rmap_walk(folio: *mut folio, rwc: *mut rmap_walk_control) {
    if rust_rmap_folio_test_ksm(folio) {
        rust_rmap_rmap_walk_ksm(folio, rwc);
    } else if rust_rmap_folio_test_anon(folio) {
        rmap_walk_anon(folio, rwc, false);
    } else {
        rmap_walk_file(folio, rwc, false);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rmap_walk_locked(folio: *mut folio, rwc: *mut rmap_walk_control) {
    rust_rmap_diag_walk_locked_ksm(folio);
    if rust_rmap_folio_test_anon(folio) {
        rmap_walk_anon(folio, rwc, true);
    } else {
        rmap_walk_file(folio, rwc, true);
    }
}

// huge_ptep_get has only a declaration (no definition) without HUGETLB_PAGE.
// Original C eliminates that call through its constant false folio predicate;
// mirror the configured branch explicitly across the Rust/native boundary.
#[inline(always)]
unsafe fn rmap_ptep_get(
    folio: *mut folio,
    mm: *mut mm_struct,
    address: c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        if rust_rmap_folio_test_hugetlb(folio) {
            return rust_rmap_huge_ptep_get(mm, address, ptep);
        }
    }
    #[cfg(not(CONFIG_HUGETLB_PAGE))]
    {
        let _ = (folio, mm, address);
    }
    rust_rmap_ptep_get(ptep)
}
