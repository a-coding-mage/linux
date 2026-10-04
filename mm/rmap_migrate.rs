// SPDX-License-Identifier: GPL-2.0
unsafe extern "C" fn try_to_migrate_one(
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
    let mut hsz: c_ulong = 0;
    if flags & TTU_SYNC != 0 {
        pvmw.flags = RUST_RMAP_PVMW_SYNC as c_uint;
    }
    let end = rust_rmap_vma_address_end(&mut pvmw);
    rust_rmap_mmu_notifier_range_init(&mut range, MMU_NOTIFY_CLEAR, 0, mm, address, end);
    if rust_rmap_folio_test_hugetlb(folio) {
        rust_rmap_adjust_range_if_pmd_sharing_possible(vma, &mut range.start, &mut range.end);
        hsz = rust_rmap_huge_page_size(rust_rmap_hstate_vma(vma));
    }
    rust_rmap_mmu_notifier_invalidate_range_start(&mut range);
    while rust_rmap_page_vma_mapped_walk(&mut pvmw) {
        if pvmw.pte.is_null() {
            if flags & TTU_SPLIT_HUGE_PMD != 0 {
                rust_rmap_split_huge_pmd_locked(vma, pvmw.address, pvmw.pmd, true);
                flags &= !TTU_SPLIT_HUGE_PMD;
                rust_rmap_page_vma_mapped_walk_restart(&mut pvmw);
                continue;
            }
            #[cfg(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)]
            {
                let pmdval = rust_rmap_pmdp_get(pvmw.pmd);
                let pfn = if rust_rmap_pmd_present(pmdval) {
                    rust_rmap_pmd_pfn(pmdval)
                } else {
                    rust_rmap_softleaf_to_pfn(rust_rmap_softleaf_from_pmd(pmdval))
                };
                let subpage =
                    rust_rmap_folio_page(folio, pfn.wrapping_sub(rust_rmap_folio_pfn(folio)));
                rust_rmap_diag_migrate_pmd(folio);
                if rust_rmap_set_pmd_migration_entry(&mut pvmw, subpage) != 0 {
                    ret = false;
                    rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                    break;
                }
                continue;
            }
        }
        rust_rmap_diag_migrate_pte(folio, pvmw.pte);
        address = pvmw.address;
        let mut pteval = rmap_ptep_get(folio, mm, address, pvmw.pte);
        let pfn = if rust_rmap_pte_present(pteval) {
            rust_rmap_pte_pfn(pteval)
        } else {
            let pfn = rust_rmap_softleaf_to_pfn(rust_rmap_softleaf_from_pte(pteval));
            rust_rmap_diag_migrate_nonpresent_huge(folio);
            pfn
        };
        let subpage = rust_rmap_folio_page(folio, pfn.wrapping_sub(rust_rmap_folio_pfn(folio)));
        let anon_exclusive =
            rust_rmap_folio_test_anon(folio) && rust_rmap_page_anon_exclusive(subpage);
        let writable;
        if rust_rmap_folio_test_hugetlb(folio) {
            let anon = rust_rmap_folio_test_anon(folio);
            rust_rmap_flush_cache_range(vma, range.start, range.end);
            if !anon {
                rust_rmap_diag_migrate_rmap_locked(flags);
                if !rust_rmap_hugetlb_vma_trylock_write(vma) {
                    rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                    ret = false;
                    break;
                }
                let mut tlb: mmu_gather = zeroed();
                rust_rmap_tlb_gather_mmu_vma(&mut tlb, vma);
                if rust_rmap_huge_pmd_unshare(&mut tlb, vma, address, pvmw.pte) != 0 {
                    rust_rmap_hugetlb_vma_unlock_write(vma);
                    rust_rmap_huge_pmd_unshare_flush(&mut tlb, vma);
                    rust_rmap_tlb_finish_mmu(&mut tlb);
                    rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                    break;
                }
                rust_rmap_hugetlb_vma_unlock_write(vma);
                rust_rmap_tlb_finish_mmu(&mut tlb);
            }
            pteval = rust_rmap_huge_ptep_clear_flush(vma, address, pvmw.pte);
            if rust_rmap_pte_dirty(pteval) {
                rust_rmap_folio_mark_dirty(folio);
            }
            writable = rust_rmap_pte_write(pteval);
        } else if rust_rmap_pte_present(pteval) {
            rust_rmap_flush_cache_page(vma, address, pfn);
            if should_defer_flush(mm, flags) {
                pteval = rust_rmap_ptep_get_and_clear(mm, address, pvmw.pte);
                set_tlb_ubc_flush_pending(
                    mm,
                    pteval,
                    address,
                    address.wrapping_add(RUST_RMAP_PAGE_SIZE as c_ulong),
                );
            } else {
                pteval = rust_rmap_ptep_clear_flush(vma, address, pvmw.pte);
            }
            if rust_rmap_pte_dirty(pteval) {
                rust_rmap_folio_mark_dirty(folio);
            }
            writable = rust_rmap_pte_write(pteval);
        } else {
            let entry = rust_rmap_softleaf_from_pte(pteval);
            rust_rmap_pte_clear(mm, address, pvmw.pte);
            writable = rust_rmap_softleaf_is_device_private_write(entry);
        }
        rust_rmap_diag_migrate_writable(folio, writable, anon_exclusive);
        rust_rmap_update_hiwater_rss(mm);
        if rust_rmap_page_hwpoison(subpage) {
            rust_rmap_diag_migrate_poison_private(folio);
            pteval = rust_rmap_swp_entry_to_pte(rust_rmap_make_hwpoison_entry(subpage));
            if rust_rmap_folio_test_hugetlb(folio) {
                rust_rmap_hugetlb_count_sub(rust_rmap_folio_nr_pages(folio) as c_long, mm);
                rust_rmap_set_huge_pte_at(mm, address, pvmw.pte, pteval, hsz);
            } else {
                rust_rmap_dec_mm_counter(mm, rust_rmap_mm_counter(folio));
                rust_rmap_set_pte_at(mm, address, pvmw.pte, pteval);
            }
        } else if rust_rmap_pte_present(pteval)
            && rust_rmap_pte_unused(pteval)
            && !rust_rmap_userfaultfd_armed(vma)
        {
            rust_rmap_dec_mm_counter(mm, rust_rmap_mm_counter(folio));
        } else {
            if rust_rmap_arch_unmap_one(mm, vma, address, pteval) < 0 {
                if rust_rmap_folio_test_hugetlb(folio) {
                    rust_rmap_set_huge_pte_at(mm, address, pvmw.pte, pteval, hsz);
                } else {
                    rust_rmap_set_pte_at(mm, address, pvmw.pte, pteval);
                }
                ret = false;
                rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                break;
            }
            if rust_rmap_folio_test_hugetlb(folio) {
                if anon_exclusive && rust_rmap_hugetlb_try_share_anon_rmap(folio) != 0 {
                    rust_rmap_set_huge_pte_at(mm, address, pvmw.pte, pteval, hsz);
                    ret = false;
                    rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                    break;
                }
            } else if anon_exclusive && rust_rmap_folio_try_share_anon_rmap_pte(folio, subpage) != 0
            {
                rust_rmap_set_pte_at(mm, address, pvmw.pte, pteval);
                ret = false;
                rust_rmap_page_vma_mapped_walk_done(&mut pvmw);
                break;
            }
            let mut entry = if writable {
                rust_rmap_make_writable_migration_entry(rust_rmap_page_to_pfn(subpage))
            } else if anon_exclusive {
                rust_rmap_make_readable_exclusive_migration_entry(rust_rmap_page_to_pfn(subpage))
            } else {
                rust_rmap_make_readable_migration_entry(rust_rmap_page_to_pfn(subpage))
            };
            let mut swp_pte;
            if rust_rmap_pte_present(pteval) {
                if rust_rmap_pte_young(pteval) {
                    entry = rust_rmap_make_migration_entry_young(entry);
                }
                if rust_rmap_pte_dirty(pteval) {
                    entry = rust_rmap_make_migration_entry_dirty(entry);
                }
                swp_pte = rust_rmap_swp_entry_to_pte(entry);
                if rust_rmap_pte_soft_dirty(pteval) {
                    swp_pte = rust_rmap_pte_swp_mksoft_dirty(swp_pte);
                }
                if rust_rmap_pte_uffd(pteval) {
                    swp_pte = rust_rmap_pte_swp_mkuffd(swp_pte);
                }
            } else {
                swp_pte = rust_rmap_swp_entry_to_pte(entry);
                if rust_rmap_pte_swp_soft_dirty(pteval) {
                    swp_pte = rust_rmap_pte_swp_mksoft_dirty(swp_pte);
                }
                if rust_rmap_pte_swp_uffd(pteval) {
                    swp_pte = rust_rmap_pte_swp_mkuffd(swp_pte);
                }
            }
            if rust_rmap_folio_test_hugetlb(folio) {
                rust_rmap_set_huge_pte_at(mm, address, pvmw.pte, swp_pte, hsz);
            } else {
                rust_rmap_set_pte_at(mm, address, pvmw.pte, swp_pte);
            }
            rust_rmap_trace_set_migration_pte(address, swp_pte, rust_rmap_folio_order(folio));
        }
        if rust_rmap_folio_test_hugetlb(folio) {
            rust_rmap_hugetlb_remove_rmap(folio);
        } else {
            folio_remove_rmap_ptes(folio, subpage, 1, vma);
        }
        if rust_rmap_vma_vm_flags(vma) & RUST_RMAP_VM_LOCKED as c_ulong != 0 {
            rust_rmap_mlock_drain_local();
        }
        rust_rmap_folio_put(folio);
    }
    rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn try_to_migrate(folio: *mut folio, flags: ttu_flags) {
    let mut rwc: rmap_walk_control = zeroed();
    rwc.rmap_one = Some(try_to_migrate_one);
    rwc.arg = flags as usize as *mut c_void;
    rwc.done = Some(folio_not_mapped);
    rwc.anon_lock = Some(folio_lock_anon_vma_read);
    if rust_rmap_diag_migrate_flags(flags) {
        return;
    }
    if rust_rmap_folio_is_zone_device(folio)
        && !rust_rmap_folio_is_device_private(folio)
        && !rust_rmap_folio_is_device_coherent(folio)
    {
        return;
    }
    if !rust_rmap_folio_test_ksm(folio) && rust_rmap_folio_test_anon(folio) {
        rwc.invalid_vma = Some(invalid_migration_vma);
    }
    if flags & TTU_RMAP_LOCKED != 0 {
        rmap_walk_locked(folio, &mut rwc);
    } else {
        rmap_walk(folio, &mut rwc);
    }
}
#[cfg(CONFIG_DEVICE_PRIVATE)]
#[no_mangle]
pub unsafe extern "C" fn make_device_exclusive(
    mm: *mut mm_struct,
    mut addr: c_ulong,
    owner: *mut c_void,
    foliop: *mut *mut folio,
) -> *mut page {
    rust_rmap_mmap_assert_locked(mm);
    addr &= !((RUST_RMAP_PAGE_SIZE as c_ulong) - 1);
    loop {
        let mut vma: *mut vm_area_struct = null_mut();
        let page = rust_rmap_get_user_page_vma_remote(
            mm,
            addr,
            (RUST_RMAP_FOLL_GET | RUST_RMAP_FOLL_WRITE | RUST_RMAP_FOLL_SPLIT_PMD) as c_uint,
            &mut vma,
        );
        if rust_rmap_is_err(page.cast()) {
            return page;
        }
        let folio = rust_rmap_page_folio(page);
        if !rust_rmap_folio_test_anon(folio) || rust_rmap_folio_test_hugetlb(folio) {
            rust_rmap_folio_put(folio);
            return rust_rmap_err_ptr(-(EOPNOTSUPP as c_long)).cast();
        }
        let ret = rust_rmap_folio_lock_killable(folio);
        if ret != 0 {
            rust_rmap_folio_put(folio);
            return rust_rmap_err_ptr(ret as c_long).cast();
        }
        let mut range: mmu_notifier_range = zeroed();
        rust_rmap_mmu_notifier_range_init_owner(
            &mut range,
            MMU_NOTIFY_EXCLUSIVE,
            0,
            mm,
            addr,
            addr.wrapping_add(RUST_RMAP_PAGE_SIZE as c_ulong),
            owner,
        );
        rust_rmap_mmu_notifier_invalidate_range_start(&mut range);
        let mut fw: folio_walk = zeroed();
        let fw_folio = rust_rmap_folio_walk_start(&mut fw, vma, addr, 0);
        if fw_folio != folio
            || fw.page != page
            || fw.level != FW_LEVEL_PTE
            || !rust_rmap_pte_write(rust_rmap_fw_pte(&fw))
        {
            if !fw_folio.is_null() {
                rust_rmap_folio_walk_end(&mut fw, vma);
            }
            rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
            rust_rmap_folio_unlock(folio);
            rust_rmap_folio_put(folio);
            continue;
        }
        rust_rmap_flush_cache_page(vma, addr, rust_rmap_page_to_pfn(page));
        let pte = rust_rmap_ptep_clear_flush(vma, addr, rust_rmap_fw_ptep(&fw));
        rust_rmap_fw_set_pte(&mut fw, pte);
        if rust_rmap_pte_dirty(pte) {
            rust_rmap_folio_mark_dirty(folio);
        }
        let entry = rust_rmap_make_device_exclusive_entry(rust_rmap_page_to_pfn(page));
        let mut swp_pte = rust_rmap_swp_entry_to_pte(entry);
        if rust_rmap_pte_soft_dirty(pte) {
            swp_pte = rust_rmap_pte_swp_mksoft_dirty(swp_pte);
        }
        rust_rmap_set_pte_at(mm, addr, rust_rmap_fw_ptep(&fw), swp_pte);
        rust_rmap_folio_walk_end(&mut fw, vma);
        rust_rmap_mmu_notifier_invalidate_range_end(&mut range);
        *foliop = folio;
        return page;
    }
}
