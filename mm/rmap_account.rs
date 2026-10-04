// SPDX-License-Identifier: GPL-2.0
unsafe fn __folio_mod_stat(folio: *mut folio, nr: c_int, nr_pmdmapped: c_int) {
    if nr != 0 {
        let idx = if rust_rmap_folio_test_anon(folio) {
            NR_ANON_MAPPED
        } else {
            NR_FILE_MAPPED
        };
        rust_rmap_lruvec_stat_mod_folio(folio, idx, nr);
    }
    if nr_pmdmapped != 0 {
        if rust_rmap_folio_test_anon(folio) {
            rust_rmap_lruvec_stat_mod_folio(folio, NR_ANON_THPS, nr_pmdmapped);
        } else {
            let idx = if rust_rmap_folio_test_swapbacked(folio) {
                NR_SHMEM_PMDMAPPED
            } else {
                NR_FILE_PMDMAPPED
            };
            rust_rmap___mod_node_page_state(
                rust_rmap_folio_pgdat(folio),
                idx,
                nr_pmdmapped as c_long,
            );
        }
    }
}
#[inline(always)]
unsafe fn __folio_add_rmap<const LEVEL: pgtable_level>(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
) {
    const {
        assert!(
            LEVEL == PGTABLE_LEVEL_PTE || LEVEL == PGTABLE_LEVEL_PMD || LEVEL == PGTABLE_LEVEL_PUD
        );
    }
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut page = page;
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut nr_pages = nr_pages;
    let _mapped = rust_rmap_folio_nr_pages_mapped_ptr(folio);
    let orig_nr_pages = nr_pages;
    #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
    let mut nr: c_int;
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut nr: c_int = 0;
    let mut nr_pmdmapped: c_int = 0;
    rmap_sanity_checks::<LEVEL>(folio, page, nr_pages);
    match LEVEL {
        PGTABLE_LEVEL_PTE => {
            if !rust_rmap_folio_test_large(folio) {
                nr = rust_rmap_atomic_inc_and_test(rust_rmap_folio_mapcount_ptr(folio)) as c_int;
            } else {
                #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
                {
                    nr = rust_rmap_folio_add_return_large_mapcount(folio, orig_nr_pages, vma);
                    nr = if nr == orig_nr_pages {
                        rust_rmap_folio_large_nr_pages(folio) as c_int
                    } else {
                        0
                    };
                }
                #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
                {
                    let mut first: c_int = 0;
                    loop {
                        first = first.wrapping_add(rust_rmap_atomic_inc_and_test(
                            rust_rmap_page_mapcount_ptr(page),
                        ) as c_int);
                        page = rust_rmap_page_nth(page, 1);
                        nr_pages = nr_pages.wrapping_sub(1);
                        if nr_pages <= 0 {
                            break;
                        }
                    }
                    if first != 0
                        && rust_rmap_atomic_add_return_relaxed(first, _mapped)
                            < RUST_RMAP_ENTIRELY_MAPPED as c_int
                    {
                        nr = first;
                    }
                    rust_rmap_folio_add_large_mapcount(folio, orig_nr_pages, vma);
                }
            }
        }
        PGTABLE_LEVEL_PMD | PGTABLE_LEVEL_PUD => {
            let first = rust_rmap_atomic_inc_and_test(rust_rmap_folio_entire_mapcount_ptr(folio));
            {
                #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
                {
                    if LEVEL == PGTABLE_LEVEL_PMD && first {
                        nr_pmdmapped = rust_rmap_folio_large_nr_pages(folio) as c_int;
                    }
                    nr = rust_rmap_folio_inc_return_large_mapcount(folio, vma);
                    nr = if nr == 1 {
                        rust_rmap_folio_large_nr_pages(folio) as c_int
                    } else {
                        0
                    };
                }
                #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
                {
                    if first {
                        nr = rust_rmap_atomic_add_return_relaxed(
                            RUST_RMAP_ENTIRELY_MAPPED as c_int,
                            _mapped,
                        );
                        if nr < (RUST_RMAP_ENTIRELY_MAPPED as c_int) * 2 {
                            nr_pages = rust_rmap_folio_large_nr_pages(folio) as c_int;
                            if LEVEL == PGTABLE_LEVEL_PMD {
                                nr_pmdmapped = nr_pages;
                            }
                            nr = nr_pages.wrapping_sub(nr & RUST_RMAP_FOLIO_PAGES_MAPPED as c_int);
                            if nr < 0 {
                                nr = 0;
                            }
                        } else {
                            nr = 0;
                        }
                    }
                    rust_rmap_folio_inc_large_mapcount(folio, vma);
                }
            }
        }
        _ => core::hint::unreachable_unchecked(),
    }
    __folio_mod_stat(folio, nr, nr_pmdmapped);
}
#[no_mangle]
pub unsafe extern "C" fn folio_move_anon_rmap(folio: *mut folio, vma: *mut vm_area_struct) {
    let av = (*vma).anon_vma;
    rust_rmap_diag_move_anon_1(folio, vma, av);
    rust_rmap_diag_move_anon_2(folio, vma, av);
    rust_rmap_folio_mapping_write_once(
        folio,
        (av as c_ulong).wrapping_add(RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong) as *mut address_space,
    );
}
unsafe fn __folio_set_anon(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    exclusive: bool,
) {
    let mut av = (*vma).anon_vma;
    rust_rmap_diag_set_anon(av);
    if !exclusive {
        av = (*av).root;
    }
    rust_rmap_folio_mapping_write_once(
        folio,
        (av as c_ulong).wrapping_add(RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong) as *mut address_space,
    );
    rust_rmap_folio_set_index(folio, rust_rmap_linear_anon_page_index(vma, address));
}
unsafe fn __page_check_anon_rmap(
    folio: *const folio,
    page: *const page,
    vma: *mut vm_area_struct,
    address: c_ulong,
) {
    rust_rmap_diag_check_anon_1(folio, page, vma, address);
    rust_rmap_diag_check_anon_2(folio, page, vma, address);
}
#[inline(always)]
unsafe fn __folio_add_anon_rmap<const LEVEL: pgtable_level>(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: rmap_t,
) {
    const {
        assert!(
            LEVEL == PGTABLE_LEVEL_PTE || LEVEL == PGTABLE_LEVEL_PMD || LEVEL == PGTABLE_LEVEL_PUD
        );
    }
    rust_rmap_diag_add_anon(folio);
    __folio_add_rmap::<LEVEL>(folio, page, nr_pages, vma);
    if !rust_rmap_folio_test_ksm(folio) {
        __page_check_anon_rmap(folio, page, vma, address);
    }
    if flags & RUST_RMAP_RMAP_EXCLUSIVE as rmap_t != 0 {
        match LEVEL {
            PGTABLE_LEVEL_PTE => {
                for i in 0..nr_pages {
                    rust_rmap_set_page_anon_exclusive(rust_rmap_page_nth(page, i as c_long));
                }
            }
            PGTABLE_LEVEL_PMD => {
                rust_rmap_set_page_anon_exclusive(page);
            }
            PGTABLE_LEVEL_PUD => {
                rust_rmap_diag_add_anon_pud();
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
    rust_rmap_diag_add_anon_small_count(folio, page);
    for i in 0..nr_pages {
        let cur_page = rust_rmap_page_nth(page, i as c_long);
        rust_rmap_diag_add_anon_entire_count(folio, cur_page);
        if cfg!(CONFIG_NO_PAGE_MAPCOUNT) {
            continue;
        }
        rust_rmap_diag_add_anon_page_count(folio, cur_page);
    }
    if rust_rmap_folio_nr_pages(folio) == nr_pages as c_ulong {
        rust_rmap_mlock_vma_folio(folio, vma);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_anon_rmap_ptes(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: rmap_t,
) {
    __folio_add_anon_rmap::<{ PGTABLE_LEVEL_PTE }>(folio, page, nr_pages, vma, address, flags);
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_anon_rmap_pmd(
    folio: *mut folio,
    page: *mut page,
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: rmap_t,
) {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        __folio_add_anon_rmap::<{ PGTABLE_LEVEL_PMD }>(
            folio,
            page,
            RUST_RMAP_HPAGE_PMD_NR as c_int,
            vma,
            address,
            flags,
        );
    }
    #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
    {
        let _ = (folio, page, vma, address, flags);
        rust_rmap_diag_add_anon_pmd_unsupported();
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_new_anon_rmap(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: rmap_t,
) {
    let exclusive = flags & RUST_RMAP_RMAP_EXCLUSIVE as rmap_t != 0;
    let mut nr: c_int = 1;
    let mut nr_pmdmapped: c_int = 0;
    rust_rmap_diag_add_new_anon_1(folio, exclusive);
    rust_rmap_diag_add_new_anon_2(folio, exclusive);
    if !rust_rmap_folio_test_swapbacked(folio)
        && rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_DROPPABLE as c_ulong == 0
    {
        rust_rmap___folio_set_swapbacked(folio);
    }
    __folio_set_anon(folio, vma, address, exclusive);
    if !rust_rmap_folio_test_large(folio) {
        rust_rmap_atomic_set(rust_rmap_folio_mapcount_ptr(folio), 0);
        if exclusive {
            rust_rmap_set_page_anon_exclusive(rust_rmap_folio_page(folio, 0));
        }
    } else if !rust_rmap_folio_test_pmd_mappable(folio) {
        nr = rust_rmap_folio_large_nr_pages(folio) as c_int;
        for i in 0..nr {
            let page = rust_rmap_folio_page(folio, i as c_ulong);
            if cfg!(CONFIG_PAGE_MAPCOUNT) {
                rust_rmap_atomic_set(rust_rmap_page_mapcount_ptr(page), 0);
            }
            if exclusive {
                rust_rmap_set_page_anon_exclusive(page);
            }
        }
        rust_rmap_folio_set_large_mapcount(folio, nr, vma);
        if cfg!(CONFIG_PAGE_MAPCOUNT) {
            rust_rmap_atomic_set(rust_rmap_folio_nr_pages_mapped_ptr(folio), nr);
        }
    } else {
        nr = rust_rmap_folio_large_nr_pages(folio) as c_int;
        rust_rmap_atomic_set(rust_rmap_folio_entire_mapcount_ptr(folio), 0);
        rust_rmap_folio_set_large_mapcount(folio, 1, vma);
        if cfg!(CONFIG_PAGE_MAPCOUNT) {
            rust_rmap_atomic_set(
                rust_rmap_folio_nr_pages_mapped_ptr(folio),
                RUST_RMAP_ENTIRELY_MAPPED as c_int,
            );
        }
        if exclusive {
            rust_rmap_set_page_anon_exclusive(rust_rmap_folio_page(folio, 0));
        }
        nr_pmdmapped = nr;
    }
    rust_rmap_diag_add_new_anon_range(address, nr, vma);
    __folio_mod_stat(folio, nr, nr_pmdmapped);
    rust_rmap_mod_mthp_stat(rust_rmap_folio_order(folio) as c_int, MTHP_STAT_NR_ANON, 1);
}
#[inline(always)]
unsafe fn __folio_add_file_rmap<const LEVEL: pgtable_level>(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
) {
    const {
        assert!(
            LEVEL == PGTABLE_LEVEL_PTE || LEVEL == PGTABLE_LEVEL_PMD || LEVEL == PGTABLE_LEVEL_PUD
        );
    }
    rust_rmap_diag_add_file(folio);
    __folio_add_rmap::<LEVEL>(folio, page, nr_pages, vma);
    if rust_rmap_folio_nr_pages(folio) == nr_pages as c_ulong {
        rust_rmap_mlock_vma_folio(folio, vma);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_ptes(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
) {
    __folio_add_file_rmap::<{ PGTABLE_LEVEL_PTE }>(folio, page, nr_pages, vma);
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_pmd(
    folio: *mut folio,
    page: *mut page,
    vma: *mut vm_area_struct,
) {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        __folio_add_file_rmap::<{ PGTABLE_LEVEL_PMD }>(
            folio,
            page,
            RUST_RMAP_HPAGE_PMD_NR as c_int,
            vma,
        );
    }
    #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
    {
        let _ = (folio, page, vma);
        rust_rmap_diag_add_file_pmd_unsupported();
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_pud(
    folio: *mut folio,
    page: *mut page,
    vma: *mut vm_area_struct,
) {
    #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
    {
        __folio_add_file_rmap::<{ PGTABLE_LEVEL_PUD }>(
            folio,
            page,
            RUST_RMAP_HPAGE_PUD_NR as c_int,
            vma,
        );
    }
    #[cfg(not(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)))]
    {
        let _ = (folio, page, vma);
        rust_rmap_diag_add_file_pud_unsupported();
    }
}
#[inline(always)]
unsafe fn __folio_remove_rmap<const LEVEL: pgtable_level>(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
) {
    const {
        assert!(
            LEVEL == PGTABLE_LEVEL_PTE || LEVEL == PGTABLE_LEVEL_PMD || LEVEL == PGTABLE_LEVEL_PUD
        );
    }
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut page = page;
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut nr_pages = nr_pages;
    let _mapped = rust_rmap_folio_nr_pages_mapped_ptr(folio);
    #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
    let mut nr: c_int;
    #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
    let mut nr: c_int = 0;
    let mut nr_pmdmapped: c_int = 0;
    let mut partially_mapped = false;
    rmap_sanity_checks::<LEVEL>(folio, page, nr_pages);
    match LEVEL {
        PGTABLE_LEVEL_PTE => {
            if !rust_rmap_folio_test_large(folio) {
                nr =
                    rust_rmap_atomic_add_negative(-1, rust_rmap_folio_mapcount_ptr(folio)) as c_int;
            } else {
                #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
                {
                    nr = rust_rmap_folio_sub_return_large_mapcount(folio, nr_pages, vma);
                    if nr == 0 {
                        nr = rust_rmap_folio_large_nr_pages(folio) as c_int;
                    } else {
                        partially_mapped = (nr as c_ulong) < rust_rmap_folio_large_nr_pages(folio)
                            && rust_rmap_folio_entire_mapcount(folio) == 0;
                        nr = 0;
                    }
                }
                #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
                {
                    rust_rmap_folio_sub_large_mapcount(folio, nr_pages, vma);
                    let mut last: c_int = 0;
                    loop {
                        last = last.wrapping_add(rust_rmap_atomic_add_negative(
                            -1,
                            rust_rmap_page_mapcount_ptr(page),
                        ) as c_int);
                        page = rust_rmap_page_nth(page, 1);
                        nr_pages = nr_pages.wrapping_sub(1);
                        if nr_pages <= 0 {
                            break;
                        }
                    }
                    if last != 0
                        && rust_rmap_atomic_sub_return_relaxed(last, _mapped)
                            < RUST_RMAP_ENTIRELY_MAPPED as c_int
                    {
                        nr = last;
                    }
                    partially_mapped = nr != 0 && rust_rmap_atomic_read(_mapped) != 0;
                }
            }
        }
        PGTABLE_LEVEL_PMD | PGTABLE_LEVEL_PUD => {
            #[cfg(CONFIG_NO_PAGE_MAPCOUNT)]
            {
                let last =
                    rust_rmap_atomic_add_negative(-1, rust_rmap_folio_entire_mapcount_ptr(folio));
                if LEVEL == PGTABLE_LEVEL_PMD && last {
                    nr_pmdmapped = rust_rmap_folio_large_nr_pages(folio) as c_int;
                }
                nr = rust_rmap_folio_dec_return_large_mapcount(folio, vma);
                if nr == 0 {
                    nr = rust_rmap_folio_large_nr_pages(folio) as c_int;
                } else {
                    partially_mapped =
                        last && (nr as c_ulong) < rust_rmap_folio_large_nr_pages(folio);
                    nr = 0;
                }
            }
            #[cfg(not(CONFIG_NO_PAGE_MAPCOUNT))]
            {
                rust_rmap_folio_dec_large_mapcount(folio, vma);
                let last =
                    rust_rmap_atomic_add_negative(-1, rust_rmap_folio_entire_mapcount_ptr(folio));
                if last {
                    nr = rust_rmap_atomic_sub_return_relaxed(
                        RUST_RMAP_ENTIRELY_MAPPED as c_int,
                        _mapped,
                    );
                    if nr < RUST_RMAP_ENTIRELY_MAPPED as c_int {
                        nr_pages = rust_rmap_folio_large_nr_pages(folio) as c_int;
                        if LEVEL == PGTABLE_LEVEL_PMD {
                            nr_pmdmapped = nr_pages;
                        }
                        nr = nr_pages.wrapping_sub(nr);
                        if nr < 0 {
                            nr = 0;
                        }
                    } else {
                        nr = 0;
                    }
                }
                partially_mapped = nr != 0 && nr < nr_pmdmapped;
            }
        }
        _ => core::hint::unreachable_unchecked(),
    }
    if partially_mapped
        && rust_rmap_folio_test_anon(folio)
        && !rust_rmap_folio_test_partially_mapped(folio)
        && !rust_rmap_folio_is_device_private(folio)
    {
        rust_rmap_deferred_split_folio(folio, true);
    }
    __folio_mod_stat(folio, nr.wrapping_neg(), nr_pmdmapped.wrapping_neg());
    rust_rmap_munlock_vma_folio(folio, vma);
}
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_ptes(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
    vma: *mut vm_area_struct,
) {
    __folio_remove_rmap::<{ PGTABLE_LEVEL_PTE }>(folio, page, nr_pages, vma);
}
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_pmd(
    folio: *mut folio,
    page: *mut page,
    vma: *mut vm_area_struct,
) {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        __folio_remove_rmap::<{ PGTABLE_LEVEL_PMD }>(
            folio,
            page,
            RUST_RMAP_HPAGE_PMD_NR as c_int,
            vma,
        );
    }
    #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
    {
        let _ = (folio, page, vma);
        rust_rmap_diag_remove_pmd_unsupported();
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_pud(
    folio: *mut folio,
    page: *mut page,
    vma: *mut vm_area_struct,
) {
    #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
    {
        __folio_remove_rmap::<{ PGTABLE_LEVEL_PUD }>(
            folio,
            page,
            RUST_RMAP_HPAGE_PUD_NR as c_int,
            vma,
        );
    }
    #[cfg(not(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)))]
    {
        let _ = (folio, page, vma);
        rust_rmap_diag_remove_pud_unsupported();
    }
}
#[cfg(CONFIG_HUGETLB_PAGE)]
#[no_mangle]
pub unsafe extern "C" fn hugetlb_add_anon_rmap(
    folio: *mut folio,
    _vma: *mut vm_area_struct,
    _address: c_ulong,
    flags: rmap_t,
) {
    rust_rmap_diag_hugetlb_add_1(folio);
    rust_rmap_diag_hugetlb_add_2(folio);
    rust_rmap_atomic_inc(rust_rmap_folio_entire_mapcount_ptr(folio));
    rust_rmap_atomic_inc(rust_rmap_folio_large_mapcount_ptr(folio));
    if flags & RUST_RMAP_RMAP_EXCLUSIVE as rmap_t != 0 {
        rust_rmap_set_page_anon_exclusive(rust_rmap_folio_page(folio, 0));
    }
    rust_rmap_diag_hugetlb_add_exclusive(folio);
}
#[cfg(CONFIG_HUGETLB_PAGE)]
#[no_mangle]
pub unsafe extern "C" fn hugetlb_add_new_anon_rmap(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    address: c_ulong,
) {
    rust_rmap_diag_hugetlb_new_1(folio, vma, address);
    rust_rmap_diag_hugetlb_new_2(folio, vma, address);
    rust_rmap_atomic_set(rust_rmap_folio_entire_mapcount_ptr(folio), 0);
    rust_rmap_atomic_set(rust_rmap_folio_large_mapcount_ptr(folio), 0);
    rust_rmap_folio_clear_hugetlb_restore_reserve(folio);
    __folio_set_anon(folio, vma, address, true);
    rust_rmap_set_page_anon_exclusive(rust_rmap_folio_page(folio, 0));
}

// The header contains BUILD_BUG() for a nonconstant/invalid level. Keep each
// native invocation constant, and reject unsupported Rust instantiations.
#[inline(always)]
unsafe fn rmap_sanity_checks<const LEVEL: pgtable_level>(
    folio: *mut folio,
    page: *mut page,
    nr_pages: c_int,
) {
    const {
        assert!(
            LEVEL == PGTABLE_LEVEL_PTE
                || (cfg!(CONFIG_TRANSPARENT_HUGEPAGE) && LEVEL == PGTABLE_LEVEL_PMD)
                || (cfg!(all(
                    CONFIG_TRANSPARENT_HUGEPAGE,
                    CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD
                )) && LEVEL == PGTABLE_LEVEL_PUD)
        );
    }
    match LEVEL {
        PGTABLE_LEVEL_PTE => rust_rmap_rmap_sanity_pte(folio, page, nr_pages),
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        PGTABLE_LEVEL_PMD => rust_rmap_rmap_sanity_pmd(folio, page, nr_pages),
        #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
        PGTABLE_LEVEL_PUD => rust_rmap_rmap_sanity_pud(folio, page, nr_pages),
        _ => core::hint::unreachable_unchecked(),
    }
}
