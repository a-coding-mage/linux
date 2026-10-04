// SPDX-License-Identifier: GPL-2.0-only
// mm/memory.c: restore_exclusive_pte through copy_page_range.
unsafe fn restore_exclusive_pte(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    page: *mut page,
    address: c_ulong,
    ptep: *mut pte_t,
    orig_pte: pte_t,
) {
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_exclusive_unlocked(!rust_memory_folio_test_locked(folio), folio);
    let mut pte = rust_memory_pte_mkold(rust_memory_mk_pte(
        page,
        rust_memory_vma_page_prot_read_once(vma),
    ));
    if rust_memory_pte_swp_soft_dirty(orig_pte) {
        pte = rust_memory_pte_mksoft_dirty(pte);
    }
    if rust_memory_pte_swp_uffd(orig_pte) {
        pte = rust_memory_pte_mkuffd(pte);
    }
    if rust_memory_pte_swp_uffd(orig_pte) && rust_memory_userfaultfd_rwp(vma) {
        pte = rust_memory_pte_modify(pte, rust_memory_page_none());
    }
    if rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_WRITE != 0
        && rust_memory_can_change_pte_writable(vma, address, pte)
    {
        if rust_memory_folio_test_dirty(folio) {
            pte = rust_memory_pte_mkdirty(pte);
        }
        pte = rust_memory_pte_mkwrite(pte, vma);
    }
    rust_memory_set_pte_at((*vma).vm_mm, address, ptep, pte);
    rust_memory_update_mmu_cache(vma, address, ptep);
}
unsafe fn try_restore_exclusive_pte(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    orig_pte: pte_t,
) -> c_int {
    let entry = rust_memory_softleaf_from_pte(orig_pte);
    let page = rust_memory_softleaf_to_page(entry);
    let folio = rust_memory_page_folio(page);
    if rust_memory_folio_trylock(folio) {
        restore_exclusive_pte(vma, folio, page, addr, ptep, orig_pte);
        rust_memory_folio_unlock(folio);
        return 0;
    }
    -(EBUSY as c_int)
}
unsafe fn copy_nonpresent_pte(
    dst_mm: *mut mm_struct,
    src_mm: *mut mm_struct,
    dst_pte: *mut pte_t,
    src_pte: *mut pte_t,
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    addr: c_ulong,
    rss: *mut c_int,
) -> c_ulong {
    let orig_pte = rust_memory_ptep_get(src_pte);
    let mut entry = rust_memory_softleaf_from_pte(orig_pte);
    let mut pte = orig_pte;
    if rust_memory_softleaf_is_swap(entry) {
        if rust_memory_swap_dup_entry_direct(entry) < 0 {
            return (-(EIO as c_long)) as c_ulong;
        }
        rust_memory_mm_prepare_for_swap_entries(dst_mm);
        if rust_memory_pte_swp_exclusive(orig_pte) {
            pte = rust_memory_pte_swp_clear_exclusive(orig_pte);
            rust_memory_set_pte_at(src_mm, addr, src_pte, pte);
        }
        *rss.add(RUST_MEMORY_MM_SWAPENTS as usize) += 1;
    } else if rust_memory_softleaf_is_migration(entry) {
        let folio = rust_memory_softleaf_to_folio(entry);
        *rss.add(rust_memory_mm_counter(folio) as usize) += 1;
        if !rust_memory_softleaf_is_migration_read(entry) && rust_memory_vma_is_cow_mapping(dst_vma)
        {
            entry = rust_memory_make_readable_migration_entry(rust_memory_swp_offset(entry));
            pte = rust_memory_softleaf_to_pte(entry);
            if rust_memory_pte_swp_soft_dirty(orig_pte) {
                pte = rust_memory_pte_swp_mksoft_dirty(pte);
            }
            if rust_memory_pte_swp_uffd(orig_pte) {
                pte = rust_memory_pte_swp_mkuffd(pte);
            }
            rust_memory_set_pte_at(src_mm, addr, src_pte, pte);
        }
    } else if rust_memory_softleaf_is_device_private(entry) {
        let page = rust_memory_softleaf_to_page(entry);
        let folio = rust_memory_page_folio(page);
        rust_memory_folio_get(folio);
        *rss.add(rust_memory_mm_counter(folio) as usize) += 1;
        rust_memory_folio_try_dup_anon_rmap_pte(folio, page, dst_vma, src_vma);
        if rust_memory_softleaf_is_device_private_write(entry)
            && rust_memory_vma_is_cow_mapping(dst_vma)
        {
            entry = rust_memory_make_readable_device_private_entry(rust_memory_swp_offset(entry));
            pte = rust_memory_swp_entry_to_pte(entry);
            if rust_memory_pte_swp_uffd(orig_pte) {
                pte = rust_memory_pte_swp_mkuffd(pte);
            }
            rust_memory_set_pte_at(src_mm, addr, src_pte, pte);
        }
    } else if rust_memory_softleaf_is_device_exclusive(entry) {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_bug_exclusive_not_cow(!rust_memory_vma_is_cow_mapping(src_vma));
        if try_restore_exclusive_pte(src_vma, addr, src_pte, orig_pte) != 0 {
            return (-(EBUSY as c_long)) as c_ulong;
        }
        return (-(ENOENT as c_long)) as c_ulong;
    } else if rust_memory_softleaf_is_marker(entry) {
        let marker = rust_memory_copy_pte_marker(entry, dst_vma);
        if marker != 0 {
            rust_memory_set_pte_at(dst_mm, addr, dst_pte, rust_memory_make_pte_marker(marker));
        }
        return 0;
    }
    if !rust_memory_userfaultfd_protected(dst_vma) {
        pte = rust_memory_pte_swp_clear_uffd(pte);
    }
    rust_memory_set_pte_at(dst_mm, addr, dst_pte, pte);
    0
}
unsafe fn copy_present_page(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pte: *mut pte_t,
    src_pte: *mut pte_t,
    addr: c_ulong,
    rss: *mut c_int,
    prealloc: *mut *mut folio,
    page: *mut page,
) -> c_int {
    let new_folio = *prealloc;
    if new_folio.is_null() {
        return -(EAGAIN as c_int);
    }
    if rust_memory_copy_mc_user_highpage(rust_memory_folio_page(new_folio, 0), page, addr, src_vma)
        != 0
    {
        return -(EHWPOISON as c_int);
    }
    *prealloc = null_mut();
    rust_memory_folio_mark_uptodate(new_folio);
    rust_memory_folio_add_new_anon_rmap(new_folio, dst_vma, addr, RUST_MEMORY_RMAP_EXCLUSIVE as _);
    rust_memory_folio_add_lru_vma(new_folio, dst_vma);
    *rss.add(RUST_MEMORY_MM_ANONPAGES as usize) += 1;
    let mut pte = rust_memory_folio_mk_pte(new_folio, (*dst_vma).vm_page_prot);
    pte = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(pte), dst_vma);
    if rust_memory_userfaultfd_protected(dst_vma)
        && rust_memory_pte_uffd(rust_memory_ptep_get(src_pte))
    {
        pte = rust_memory_pte_mkuffd(pte);
        if rust_memory_userfaultfd_rwp(dst_vma) {
            pte = rust_memory_pte_modify(pte, rust_memory_page_none());
        }
    }
    rust_memory_set_pte_at((*dst_vma).vm_mm, addr, dst_pte, pte);
    0
}
#[inline(always)]
unsafe fn __copy_present_ptes(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pte: *mut pte_t,
    src_pte: *mut pte_t,
    mut pte: pte_t,
    addr: c_ulong,
    nr: c_int,
) {
    let src_mm = (*src_vma).vm_mm;
    // Snapshot before RWP disarming can remove the architecture writable bit.
    let writable = rust_memory_pte_write(pte);
    if !rust_memory_userfaultfd_protected(dst_vma) {
        if rust_memory_userfaultfd_rwp(src_vma) && rust_memory_pte_uffd(pte) {
            pte = rust_memory_pte_modify(pte, (*dst_vma).vm_page_prot);
        }
        pte = rust_memory_pte_clear_uffd(pte);
    }
    if rust_memory_vma_is_cow_mapping(src_vma) && writable {
        rust_memory_wrprotect_ptes(src_mm, addr, src_pte, nr as _);
        pte = rust_memory_pte_wrprotect(pte);
    }
    if rust_memory_vma_vm_flags(src_vma) & RUST_MEMORY_VM_SHARED != 0 {
        pte = rust_memory_pte_mkclean(pte);
    }
    pte = rust_memory_pte_mkold(pte);
    rust_memory_set_ptes((*dst_vma).vm_mm, addr, dst_pte, pte, nr as _);
}
unsafe fn copy_present_ptes(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pte: *mut pte_t,
    src_pte: *mut pte_t,
    mut pte: pte_t,
    addr: c_ulong,
    max_nr: c_int,
    rss: *mut c_int,
    prealloc: *mut *mut folio,
) -> c_int {
    let mut flags = RUST_MEMORY_FPB_MERGE_WRITE as fpb_t;
    let page = vm_normal_page(src_vma, addr, pte);
    if !page.is_null() {
        let folio = rust_memory_page_folio(page);
        if (*prealloc).is_null() && rust_memory_folio_test_large(folio) && max_nr != 1 {
            if rust_memory_vma_vm_flags(src_vma) & RUST_MEMORY_VM_SHARED == 0 {
                flags |= RUST_MEMORY_FPB_RESPECT_DIRTY as fpb_t;
            }
            if rust_memory_vma_soft_dirty_enabled(src_vma) {
                flags |= RUST_MEMORY_FPB_RESPECT_SOFT_DIRTY as fpb_t;
            }
            let nr =
                rust_memory_folio_pte_batch_flags(folio, src_vma, src_pte, &mut pte, max_nr, flags);
            rust_memory_folio_ref_add(folio, nr);
            if rust_memory_folio_test_anon(folio) {
                if rust_memory_folio_try_dup_anon_rmap_ptes(folio, page, nr, dst_vma, src_vma) != 0
                {
                    rust_memory_folio_ref_sub(folio, nr);
                    return -(EAGAIN as c_int);
                }
                *rss.add(RUST_MEMORY_MM_ANONPAGES as usize) += nr;
                #[cfg(CONFIG_DEBUG_VM)]
                rust_memory_warn_copy_anon_exclusive(rust_memory_page_anon_exclusive(page), folio);
            } else {
                rust_memory_folio_dup_file_rmap_ptes(folio, page, nr, dst_vma);
                *rss.add(rust_memory_mm_counter_file(folio) as usize) += nr;
            }
            __copy_present_ptes(dst_vma, src_vma, dst_pte, src_pte, pte, addr, nr);
            return nr;
        }
        rust_memory_folio_get(folio);
        if rust_memory_folio_test_anon(folio) {
            if rust_memory_folio_try_dup_anon_rmap_pte(folio, page, dst_vma, src_vma) != 0 {
                rust_memory_folio_put(folio);
                let err = copy_present_page(
                    dst_vma, src_vma, dst_pte, src_pte, addr, rss, prealloc, page,
                );
                return if err != 0 { err } else { 1 };
            }
            *rss.add(RUST_MEMORY_MM_ANONPAGES as usize) += 1;
            #[cfg(CONFIG_DEBUG_VM)]
            rust_memory_warn_copy_single_exclusive(rust_memory_page_anon_exclusive(page), folio);
        } else {
            rust_memory_folio_dup_file_rmap_pte(folio, page, dst_vma);
            *rss.add(rust_memory_mm_counter_file(folio) as usize) += 1;
        }
    }
    __copy_present_ptes(dst_vma, src_vma, dst_pte, src_pte, pte, addr, 1);
    1
}
unsafe fn folio_prealloc(
    src_mm: *mut mm_struct,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    need_zero: bool,
) -> *mut folio {
    let new_folio = if need_zero {
        rust_memory_vma_alloc_zeroed_movable_folio(vma, addr)
    } else {
        rust_memory_vma_alloc_folio(RUST_MEMORY_GFP_HIGHUSER_MOVABLE as _, 0, vma, addr)
    };
    if new_folio.is_null() {
        return null_mut();
    }
    if rust_memory_mem_cgroup_charge(new_folio, src_mm, GFP_KERNEL) != 0 {
        rust_memory_folio_put(new_folio);
        return null_mut();
    }
    rust_memory_folio_throttle_swaprate(new_folio, GFP_KERNEL);
    new_folio
}
unsafe fn copy_pte_range(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pmd: *mut pmd_t,
    src_pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    let dst_mm = (*dst_vma).vm_mm;
    let src_mm = (*src_vma).vm_mm;
    let mut ret = 0;
    let mut entry = rust_memory_softleaf_mk_none();
    let mut prealloc: *mut folio = null_mut();
    'again: loop {
        let mut progress = 0;
        let mut rss = [0; NR_MM_COUNTERS];
        let mut dst_ptl = null_mut();
        let mut src_ptl = null_mut();
        let mut dst_pte = rust_memory_pte_alloc_map_lock(dst_mm, dst_pmd, addr, &mut dst_ptl);
        if dst_pte.is_null() {
            ret = -(ENOMEM as c_int);
            break;
        }
        let mut dummy_pmdval: pmd_t = zeroed();
        let mut src_pte = rust_memory_pte_offset_map_rw_nolock(
            src_mm,
            src_pmd,
            addr,
            &mut dummy_pmdval,
            &mut src_ptl,
        );
        if src_pte.is_null() {
            rust_memory_pte_unmap_unlock(dst_pte, dst_ptl);
            break;
        }
        rust_memory_spin_lock_nested(src_ptl, RUST_MEMORY_SINGLE_DEPTH_NESTING as _);
        let orig_src_pte = src_pte;
        let orig_dst_pte = dst_pte;
        rust_memory_lazy_mmu_mode_enable();
        loop {
            let mut nr = 1;
            if progress >= 32 {
                progress = 0;
                if rust_memory_need_resched()
                    || rust_memory_spin_needbreak(src_ptl)
                    || rust_memory_spin_needbreak(dst_ptl)
                {
                    break;
                }
            }
            let mut ptent = rust_memory_ptep_get(src_pte);
            if rust_memory_pte_none(ptent) {
                progress += 1;
            } else {
                let mut copied_nonpresent = false;
                if !rust_memory_pte_present(ptent) {
                    ret = copy_nonpresent_pte(
                        dst_mm,
                        src_mm,
                        dst_pte,
                        src_pte,
                        dst_vma,
                        src_vma,
                        addr,
                        rss.as_mut_ptr(),
                    ) as c_int;
                    if ret == -(EIO as c_int) {
                        entry = rust_memory_softleaf_from_pte(rust_memory_ptep_get(src_pte));
                        break;
                    }
                    if ret == -(EBUSY as c_int) {
                        break;
                    }
                    if ret == 0 {
                        progress += 8;
                        copied_nonpresent = true;
                    } else {
                        ptent = rust_memory_ptep_get(src_pte);
                        #[cfg(CONFIG_DEBUG_VM)]
                        rust_memory_warn_restored_nonpresent(!rust_memory_pte_present(ptent));
                        rust_memory_warn_restore_result(ret != -(ENOENT as c_int));
                    }
                }
                if !copied_nonpresent {
                    let max_nr = (end.wrapping_sub(addr) / PAGE_SIZE) as c_int;
                    ret = copy_present_ptes(
                        dst_vma,
                        src_vma,
                        dst_pte,
                        src_pte,
                        ptent,
                        addr,
                        max_nr,
                        rss.as_mut_ptr(),
                        &mut prealloc,
                    );
                    if ret == -(EAGAIN as c_int) || ret == -(EHWPOISON as c_int) {
                        break;
                    }
                    if !prealloc.is_null() {
                        rust_memory_folio_put(prealloc);
                        prealloc = null_mut();
                    }
                    nr = ret;
                    progress += 8 * nr;
                }
            }
            dst_pte = dst_pte.offset(nr as isize);
            src_pte = src_pte.offset(nr as isize);
            addr = addr.wrapping_add(PAGE_SIZE.wrapping_mul(nr as c_ulong));
            if addr == end {
                break;
            }
        }
        rust_memory_lazy_mmu_mode_disable();
        rust_memory_pte_unmap_unlock(orig_src_pte, src_ptl);
        add_mm_rss_vec(dst_mm, rss.as_mut_ptr());
        rust_memory_pte_unmap_unlock(orig_dst_pte, dst_ptl);
        rust_memory_cond_resched();
        if ret == -(EIO as c_int) {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_memory_warn_swap_retry_entry(entry.val == 0);
            if rust_memory_swap_retry_table_alloc(entry, GFP_KERNEL) < 0 {
                ret = -(ENOMEM as c_int);
                break 'again;
            }
            entry.val = 0;
        } else if ret == -(EBUSY as c_int) || ret == -(EHWPOISON as c_int) {
            break 'again;
        } else if ret == -(EAGAIN as c_int) {
            prealloc = folio_prealloc(src_mm, src_vma, addr, false);
            if prealloc.is_null() {
                return -(ENOMEM as c_int);
            }
        } else if ret < 0 {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_memory_warn_copy_unexpected(true);
        }
        ret = 0;
        if addr == end {
            break;
        }
    }
    if !prealloc.is_null() {
        rust_memory_folio_put(prealloc);
    }
    ret
}
unsafe fn copy_pmd_range(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pud: *mut pud_t,
    src_pud: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    let dst_mm = (*dst_vma).vm_mm;
    let src_mm = (*src_vma).vm_mm;
    let mut dst_pmd = rust_memory_pmd_alloc(dst_mm, dst_pud, addr);
    if dst_pmd.is_null() {
        return -(ENOMEM as c_int);
    }
    let mut src_pmd = rust_memory_pmd_offset(src_pud, addr);
    loop {
        let next = rust_memory_pmd_addr_end(addr, end);
        let mut copied = false;
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if rust_memory_pmd_is_huge(*src_pmd) {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_memory_bug_copy_pmd_size(
                next.wrapping_sub(addr) != RUST_MEMORY_HPAGE_PMD_SIZE,
                src_vma,
            );
            let err =
                rust_memory_copy_huge_pmd(dst_mm, src_mm, dst_pmd, src_pmd, addr, dst_vma, src_vma);
            if err == -(ENOMEM as c_int) {
                return -(ENOMEM as c_int);
            }
            copied = err == 0;
        }
        if !copied
            && !rust_memory_pmd_none_or_clear_bad(src_pmd)
            && copy_pte_range(dst_vma, src_vma, dst_pmd, src_pmd, addr, next) != 0
        {
            return -(ENOMEM as c_int);
        }
        dst_pmd = dst_pmd.add(1);
        src_pmd = src_pmd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    0
}
unsafe fn copy_pud_range(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_p4d: *mut p4d_t,
    src_p4d: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    let dst_mm = (*dst_vma).vm_mm;
    let src_mm = (*src_vma).vm_mm;
    let mut dst_pud = rust_memory_pud_alloc(dst_mm, dst_p4d, addr);
    if dst_pud.is_null() {
        return -(ENOMEM as c_int);
    }
    let mut src_pud = rust_memory_pud_offset(src_p4d, addr);
    loop {
        let next = rust_memory_pud_addr_end(addr, end);
        let mut copied = false;
        #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
        if rust_memory_pud_trans_huge(*src_pud) {
            #[cfg(CONFIG_DEBUG_VM)]
            rust_memory_bug_copy_pud_size(
                next.wrapping_sub(addr) != RUST_MEMORY_HPAGE_PUD_SIZE,
                src_vma,
            );
            let err = rust_memory_copy_huge_pud(dst_mm, src_mm, dst_pud, src_pud, addr, src_vma);
            if err == -(ENOMEM as c_int) {
                return -(ENOMEM as c_int);
            }
            copied = err == 0;
        }
        if !copied
            && !rust_memory_pud_none_or_clear_bad(src_pud)
            && copy_pmd_range(dst_vma, src_vma, dst_pud, src_pud, addr, next) != 0
        {
            return -(ENOMEM as c_int);
        }
        dst_pud = dst_pud.add(1);
        src_pud = src_pud.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    0
}
unsafe fn copy_p4d_range(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
    dst_pgd: *mut pgd_t,
    src_pgd: *mut pgd_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut dst_p4d = rust_memory_p4d_alloc((*dst_vma).vm_mm, dst_pgd, addr);
    if dst_p4d.is_null() {
        return -(ENOMEM as c_int);
    }
    let mut src_p4d = rust_memory_p4d_offset(src_pgd, addr);
    loop {
        let next = rust_memory_p4d_addr_end(addr, end);
        if !rust_memory_p4d_none_or_clear_bad(src_p4d)
            && copy_pud_range(dst_vma, src_vma, dst_p4d, src_p4d, addr, next) != 0
        {
            return -(ENOMEM as c_int);
        }
        dst_p4d = dst_p4d.add(1);
        src_p4d = src_p4d.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    0
}
unsafe fn vma_needs_copy(dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct) -> bool {
    rust_memory_vma_vm_flags(dst_vma) & RUST_MEMORY_VM_COPY_ON_FORK != 0
        || !(*src_vma).anon_vma.is_null()
}
#[no_mangle]
pub unsafe extern "C" fn copy_page_range(
    dst_vma: *mut vm_area_struct,
    src_vma: *mut vm_area_struct,
) -> c_int {
    let mut addr = rust_memory_vma_start(src_vma);
    let end = rust_memory_vma_end(src_vma);
    let dst_mm = (*dst_vma).vm_mm;
    let src_mm = (*src_vma).vm_mm;
    if !vma_needs_copy(dst_vma, src_vma) {
        return 0;
    }
    if rust_memory_is_vm_hugetlb_page(src_vma) {
        return rust_memory_copy_hugetlb_page_range(dst_mm, src_mm, dst_vma, src_vma);
    }
    let is_cow = rust_memory_vma_is_cow_mapping(src_vma);
    let mut range: mmu_notifier_range = zeroed();
    if is_cow {
        rust_memory_mmu_notifier_range_init(
            &mut range,
            RUST_MEMORY_MMU_NOTIFY_PROTECTION_PAGE as _,
            0,
            src_mm,
            addr,
            end,
        );
        rust_memory_mmu_notifier_invalidate_range_start(&mut range);
        rust_memory_vma_assert_write_locked(src_vma);
        rust_memory_mm_write_protect_seq_begin(src_mm);
    }
    let mut ret = 0;
    let mut dst_pgd = rust_memory_pgd_offset(dst_mm, addr);
    let mut src_pgd = rust_memory_pgd_offset(src_mm, addr);
    loop {
        let next = rust_memory_pgd_addr_end(addr, end);
        if !rust_memory_pgd_none_or_clear_bad(src_pgd)
            && copy_p4d_range(dst_vma, src_vma, dst_pgd, src_pgd, addr, next) != 0
        {
            ret = -(ENOMEM as c_int);
            break;
        }
        dst_pgd = dst_pgd.add(1);
        src_pgd = src_pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if is_cow {
        rust_memory_mm_write_protect_seq_end(src_mm);
        rust_memory_mmu_notifier_invalidate_range_end(&mut range);
    }
    ret
}
