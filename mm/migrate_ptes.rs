// SPDX-License-Identifier: GPL-2.0
unsafe fn try_to_map_unused_to_zeropage(
    pvmw: *mut page_vma_mapped_walk,
    folio: *mut folio,
    old_pte: pte_t,
    idx: c_ulong,
) -> bool {
    let page = folio_page(folio, idx);
    if page_compound(page) || page_hwpoison(page) {
        return false;
    }
    vm_diag!(bug_zero_anon(!page_anon(page), page));
    vm_diag!(bug_zero_locked(!page_locked(page), page));
    vm_diag!(bug_zero_present(pte_present(old_pte), page));
    vm_diag!(warn_zero_device(folio_is_device_private(folio), folio));
    let vma = (*pvmw).vma;
    if folio_test_mlocked(folio)
        || vma_flags(vma) & RUST_MIGRATE_VM_LOCKED != 0
        || mm_forbids_zeropage(vma_mm(vma))
    {
        return false;
    }
    if pages_identical(page, zero_page(0)) == 0 {
        return false;
    }
    let mut newpte = pte_mkspecial(pfn_pte(zero_pfn((*pvmw).address), vma_page_prot(vma)));
    if pte_swp_soft_dirty(old_pte) {
        newpte = pte_mksoft_dirty(newpte);
    }
    if pte_swp_uffd(old_pte) {
        newpte = pte_mkuffd(newpte);
    }
    if pte_swp_uffd(old_pte) && userfaultfd_rwp(vma) {
        newpte = pte_modify(newpte, page_none());
    }
    set_pte_at(vma_mm(vma), (*pvmw).address, (*pvmw).pte, newpte);
    dec_mm_counter(vma_mm(vma), mm_counter(folio));
    true
}

// Rust-private argument object; never guessed to match a C ABI object.
struct RmapWalkArg {
    folio: *mut folio,
    map_unused_to_zeropage: bool,
}

unsafe extern "C" fn remove_migration_pte(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    arg: *mut c_void,
) -> bool {
    let arg = &*(arg as *const RmapWalkArg);
    let mut pvmw = folio_vma_walk(arg.folio, vma, addr, (PVMW_SYNC | PVMW_MIGRATION) as _);
    while page_vma_mapped_walk(&mut pvmw) {
        let mut rmap_flags = RUST_MIGRATE_RMAP_NONE as rmap_t;
        let mut idx: c_ulong = 0;
        #[cfg(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)]
        if pvmw.pte.is_null() {
            vm_diag!(bug_migration_pmd(
                folio_test_hugetlb(folio) || !folio_test_pmd_mappable(folio),
                folio
            ));
            remove_migration_pmd(&mut pvmw, folio);
            continue;
        }
        let old_pte = if folio_test_hugetlb(folio) {
            huge_ptep_get(vma_mm(vma), pvmw.address, pvmw.pte)
        } else {
            ptep_get(pvmw.pte)
        };
        let mut entry = softleaf_from_pte(old_pte);
        if folio_test_large(folio) && !folio_test_hugetlb(folio) {
            idx = softleaf_to_pfn(entry).wrapping_sub(pvmw.pfn);
        }
        if arg.map_unused_to_zeropage
            && try_to_map_unused_to_zeropage(&mut pvmw, folio, old_pte, idx)
        {
            continue;
        }
        folio_get(folio);
        let new = folio_page(folio, idx);
        let mut pte = mk_pte(new, vma_page_prot_read_once(vma));
        if !softleaf_is_migration_young(entry) {
            pte = pte_mkold(pte);
        }
        if folio_test_dirty(folio) && softleaf_is_migration_dirty(entry) {
            pte = pte_mkdirty(pte);
        }
        pte = if pte_swp_soft_dirty(old_pte) {
            pte_mksoft_dirty(pte)
        } else {
            pte_clear_soft_dirty(pte)
        };
        if softleaf_is_migration_write(entry) {
            pte = pte_mkwrite(pte, vma);
        } else if pte_swp_uffd(old_pte) {
            pte = pte_mkuffd(pte);
        }
        if pte_swp_uffd(old_pte) && userfaultfd_rwp(vma) {
            pte = pte_modify(pte, page_none());
        }
        if folio_test_anon(folio) && !softleaf_is_migration_read(entry) {
            rmap_flags |= RUST_MIGRATE_RMAP_EXCLUSIVE as rmap_t;
        }
        if is_device_private_page(new) {
            entry = if pte_write(pte) {
                make_writable_device_private_entry(page_to_pfn(new))
            } else {
                make_readable_device_private_entry(page_to_pfn(new))
            };
            pte = softleaf_to_pte(entry);
            if pte_swp_soft_dirty(old_pte) {
                pte = pte_swp_mksoft_dirty(pte);
            }
            if pte_swp_uffd(old_pte) {
                pte = pte_swp_mkuffd(pte);
            }
        }
        #[cfg(not(CONFIG_HUGETLB_PAGE))]
        let installed_huge = false;
        #[cfg(CONFIG_HUGETLB_PAGE)]
        let mut installed_huge = false;
        #[cfg(CONFIG_HUGETLB_PAGE)]
        if folio_test_hugetlb(folio) {
            let h = hstate_vma(vma);
            let shift = huge_page_shift(h);
            let psize = huge_page_size(h);
            pte = arch_make_huge_pte(pte, shift, vma_flags(vma));
            if folio_test_anon(folio) {
                hugetlb_add_anon_rmap(folio, vma, pvmw.address, rmap_flags);
            } else {
                hugetlb_add_file_rmap(folio);
            }
            set_huge_pte_at(vma_mm(vma), pvmw.address, pvmw.pte, pte, psize);
            installed_huge = true;
        }
        if !installed_huge {
            if folio_test_anon(folio) {
                folio_add_anon_rmap_pte(folio, new, vma, pvmw.address, rmap_flags);
            } else {
                folio_add_file_rmap_pte(folio, new, vma);
            }
            set_pte_at(vma_mm(vma), pvmw.address, pvmw.pte, pte);
        }
        if vma_flags_read_once(vma) & RUST_MIGRATE_VM_LOCKED != 0 {
            mlock_drain_local();
        }
        trace_remove_migration_pte(
            pvmw.address,
            pte_val(pte) as _,
            compound_order(new) as c_int,
        );
        update_mmu_cache(vma, pvmw.address, pvmw.pte);
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn remove_migration_ptes(src: *mut folio, dst: *mut folio, flags: ttu_flags) {
    let mut arg = RmapWalkArg {
        folio: src,
        map_unused_to_zeropage: flags & TTU_USE_SHARED_ZEROPAGE != 0,
    };
    let mut rwc: rmap_walk_control = zeroed();
    rwc.rmap_one = Some(remove_migration_pte);
    rwc.arg = addr_of_mut!(arg).cast();
    vm_diag!(bug_shared_zero_destination(
        flags & TTU_USE_SHARED_ZEROPAGE != 0 && src != dst,
        src
    ));
    if flags & TTU_RMAP_LOCKED != 0 {
        rmap_walk_locked(dst, &mut rwc);
    } else {
        rmap_walk(dst, &mut rwc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn migration_entry_wait(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    address: c_ulong,
) {
    let mut ptl = null_mut();
    let ptep = pte_offset_map_lock(mm, pmd, address, &mut ptl);
    if ptep.is_null() {
        return;
    }
    let pte = ptep_get(ptep);
    pte_unmap(ptep);
    if !pte_none(pte) && !pte_present(pte) {
        let entry = softleaf_from_pte(pte);
        if softleaf_is_migration(entry) {
            softleaf_entry_wait_on_locked(entry, ptl);
            return;
        }
    }
    spin_unlock(ptl);
}

#[cfg(CONFIG_HUGETLB_PAGE)]
#[no_mangle]
pub unsafe extern "C" fn migration_entry_wait_huge(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
) {
    let ptl = huge_pte_lockptr(hstate_vma(vma), vma_mm(vma), ptep);
    hugetlb_vma_assert_locked(vma);
    spin_lock(ptl);
    let pte = huge_ptep_get(vma_mm(vma), addr, ptep);
    if !huge_pte_none(pte) {
        let entry = softleaf_from_pte(pte);
        if softleaf_is_migration(entry) {
            hugetlb_vma_unlock_read(vma);
            softleaf_entry_wait_on_locked(entry, ptl);
            return;
        }
    }
    spin_unlock(ptl);
    hugetlb_vma_unlock_read(vma);
}

#[cfg(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)]
#[no_mangle]
pub unsafe extern "C" fn pmd_migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t) {
    let ptl = pmd_lock(mm, pmd);
    if pmd_is_migration_entry(*pmd) {
        softleaf_entry_wait_on_locked(softleaf_from_pmd(*pmd), ptl);
    } else {
        spin_unlock(ptl);
    }
}
