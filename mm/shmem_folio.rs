// SPDX-License-Identifier: GPL-2.0-only
// C:1767-1817,1925-2640. Every acquisition has original error-path release.
unsafe fn shmem_get_sbmpol(sb: *mut shmem_sb_info) -> *mut mempolicy {
    #[cfg(all(CONFIG_NUMA, CONFIG_TMPFS))]
    {
        let mut p = null_mut();
        if !(*sb).mpol.is_null() {
            rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
            p = (*sb).mpol;
            mpol_get(p);
            rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
        }
        return p;
    }
    #[cfg(not(all(CONFIG_NUMA, CONFIG_TMPFS)))]
    {
        let _ = sb;
        null_mut()
    }
}
unsafe fn shmem_show_mpol(seq: *mut seq_file, p: *mut mempolicy) {
    #[cfg(all(CONFIG_NUMA, CONFIG_TMPFS))]
    {
        if p.is_null() || (*p).mode == MPOL_DEFAULT as _ {
            return;
        }
        let mut buffer = [0 as c_char; 64];
        mpol_to_str(buffer.as_mut_ptr(), buffer.len() as _, p);
        seq_printf(seq, c",mpol=%s".as_char_ptr(), buffer.as_ptr());
    }
    #[cfg(not(all(CONFIG_NUMA, CONFIG_TMPFS)))]
    {
        let _ = (seq, p);
    }
}
unsafe fn shmem_swapin_cluster(
    swap: swp_entry_t,
    gfp: gfp_t,
    info: *mut shmem_inode_info,
    index: Pgoff,
) -> *mut folio {
    let mut ilx = 0;
    let policy = shmem_get_pgoff_policy(info, index, 0, &mut ilx);
    let f = swap_cluster_readahead(swap, gfp, policy, ilx);
    mpol_cond_put(policy);
    f
}
unsafe fn shmem_alloc_folio(
    gfp: gfp_t,
    order: c_int,
    info: *mut shmem_inode_info,
    index: Pgoff,
) -> *mut folio {
    let mut ilx = 0;
    let policy = shmem_get_pgoff_policy(info, index, order as _, &mut ilx);
    let f = rust_shmem_folio_alloc_mpol(gfp, order as _, policy, ilx, numa_node_id());
    mpol_cond_put(policy);
    f
}
unsafe fn shmem_alloc_and_add_folio(
    vmf: *mut vm_fault,
    mut gfp: gfp_t,
    inode: *mut inode,
    mut index: Pgoff,
    mm: *mut mm_struct,
    mut orders: c_ulong,
) -> *mut folio {
    let mapping = (*inode).i_mapping;
    let info = SHMEM_I(inode);
    let mut f = null_mut();
    let mut pages: c_long = 1;
    if !cfg!(CONFIG_TRANSPARENT_HUGEPAGE) {
        orders = 0;
    }
    if orders != 0 {
        let mut suitable = shmem_suitable_orders(inode, vmf, mapping, index, orders);
        let mut order = highest_order(suitable);
        while suitable != 0 {
            pages = 1 << order;
            let aligned = round_down(index, pages as c_ulong);
            f = shmem_alloc_folio(gfp, order as c_int, info, aligned);
            if !f.is_null() {
                index = aligned;
                break;
            }
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            if pages == RUST_SHMEM_HPAGE_PMD_NR as c_long {
                count_vm_event(RUST_SHMEM_THP_FILE_FALLBACK);
            }
            count_mthp_stat(order as _, MTHP_STAT_SHMEM_FALLBACK);
            order = next_order(&mut suitable, order);
        }
    } else {
        f = shmem_alloc_folio(gfp, 0, info, index);
    }
    if f.is_null() {
        return err_ptr(-(RUST_SHMEM_ENOMEM as c_int));
    }
    __folio_set_locked(f);
    __folio_set_swapbacked(f);
    gfp &= GFP_RECLAIM_MASK as gfp_t;
    let mut error = mem_cgroup_charge(f, mm, gfp);
    'finish: {
        if error != 0 {
            let last = index.wrapping_add(pages as c_ulong).wrapping_sub(1);
            if !xa_find(
                addr_of_mut!((*mapping).i_pages),
                &mut index,
                last,
                RUST_SHMEM_XA_PRESENT,
            )
            .is_null()
            {
                error = -(RUST_SHMEM_EEXIST as c_int);
            } else if pages > 1 {
                #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
                if pages == RUST_SHMEM_HPAGE_PMD_NR as c_long {
                    count_vm_event(RUST_SHMEM_THP_FILE_FALLBACK);
                    count_vm_event(RUST_SHMEM_THP_FILE_FALLBACK_CHARGE);
                }
                count_mthp_stat(folio_order(f) as _, MTHP_STAT_SHMEM_FALLBACK);
                count_mthp_stat(folio_order(f) as _, MTHP_STAT_SHMEM_FALLBACK_CHARGE);
            }
            break 'finish;
        }
        error = shmem_add_to_page_cache(f, mapping, index, null_mut(), gfp);
        if error != 0 {
            break 'finish;
        }
        error = shmem_inode_acct_blocks(inode, pages);
        if error != 0 {
            shmem_unused_huge_shrink(SHMEM_SB((*inode).i_sb), null_mut(), pages as c_ulong);
            spin_lock(addr_of_mut!((*info).lock));
            let freed = (pages as c_ulong)
                .wrapping_add((*info).alloced)
                .wrapping_sub((*info).swapped)
                .wrapping_sub(rust_shmem_mapping_nrpages(mapping))
                as c_long;
            if freed > 0 {
                (*info).alloced = (*info).alloced.wrapping_sub(freed as c_ulong);
            }
            spin_unlock(addr_of_mut!((*info).lock));
            if freed > 0 {
                shmem_inode_unacct_blocks(inode, freed);
            }
            error = shmem_inode_acct_blocks(inode, pages);
            if error != 0 {
                filemap_remove_folio(f);
                break 'finish;
            }
        }
        shmem_recalc_inode(inode, pages, 0);
        folio_add_lru(f);
        return f;
    }
    folio_unlock(f);
    folio_put(f);
    err_ptr(error)
}
unsafe fn shmem_swap_alloc_folio(
    inode: *mut inode,
    vmf: *mut vm_fault,
    index: Pgoff,
    entry: swp_entry_t,
    mut order: c_int,
    gfp: gfp_t,
) -> *mut folio {
    let info = SHMEM_I(inode);
    if (!vmf.is_null() && userfaultfd_armed(rust_shmem_vmf_vma(vmf))) || !zswap_never_enabled() {
        order = 0;
    }
    loop {
        let mut ilx = 0;
        let p = shmem_get_pgoff_policy(info, index, order as _, &mut ilx);
        let f = swapin_sync(entry, gfp, 1 << order, vmf, p, ilx);
        mpol_cond_put(p);
        if !IS_ERR(f.cast()) || order == 0 {
            return f;
        }
        order = 0;
    }
}
unsafe fn shmem_should_replace_folio(f: *mut folio, gfp: gfp_t) -> bool {
    folio_zonenum(f) > gfp_zone(gfp)
}
unsafe fn shmem_replace_folio(
    foliop: *mut *mut folio,
    mut gfp: gfp_t,
    info: *mut shmem_inode_info,
    index: Pgoff,
    vma: *mut vm_area_struct,
) -> c_int {
    let old = *foliop;
    let entry = rust_shmem_folio_swap(old);
    let nr = folio_nr_pages(old) as c_int;
    gfp &= !(GFP_CONSTRAINT_MASK as gfp_t);
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if nr > 1 {
        gfp = thp_shmem_limit_gfp_mask(vma_thp_gfp_mask(vma), gfp);
    }
    let new = shmem_alloc_folio(gfp, folio_order(old) as c_int, info, index);
    if new.is_null() {
        return -(RUST_SHMEM_ENOMEM as c_int);
    }
    folio_ref_add(new, nr);
    folio_copy(new, old);
    flush_dcache_folio(new);
    __folio_set_locked(new);
    __folio_set_swapbacked(new);
    folio_mark_uptodate(new);
    rust_shmem_set_folio_swap(new, entry);
    folio_set_swapcache(new);
    let ci = swap_cluster_get_and_lock_irq(old);
    __swap_cache_replace_folio(ci, old, new);
    mem_cgroup_replace_folio(old, new);
    shmem_update_stats(new, nr);
    shmem_update_stats(old, -nr);
    swap_cluster_unlock_irq(ci);
    folio_add_lru(new);
    *foliop = new;
    folio_clear_swapcache(old);
    rust_shmem_clear_folio_private(old);
    folio_unlock(old);
    folio_put_refs(old, nr + 1);
    0
}
unsafe fn shmem_set_folio_swapin_error(
    inode: *mut inode,
    index: Pgoff,
    f: *mut folio,
    swap: swp_entry_t,
) {
    let old = xa_cmpxchg_irq(
        addr_of_mut!((*(*inode).i_mapping).i_pages),
        index,
        swp_to_radix_entry(swap),
        swp_to_radix_entry(make_poisoned_swp_entry()),
        0,
    );
    if old != swp_to_radix_entry(swap) {
        return;
    }
    let nr = folio_nr_pages(f) as c_long;
    folio_wait_writeback(f);
    folio_put_swap(f, null_mut());
    swap_cache_del_folio(f);
    shmem_recalc_inode(inode, -nr, -nr);
}
unsafe fn shmem_split_large_entry(
    inode: *mut inode,
    index: Pgoff,
    swap: swp_entry_t,
    gfp: gfp_t,
) -> c_int {
    let mapping = (*inode).i_mapping;
    let mut xs = MaybeUninit::<xa_state>::uninit();
    let xas = xs.as_mut_ptr();
    rust_shmem_xas_init(xas, addr_of_mut!((*mapping).i_pages), index, 0);
    loop {
        rust_shmem_xas_lock_irq(xas);
        'split: {
            let old = xas_load(xas);
            if !xa_is_value(old) || swp_to_radix_entry(swap) != old {
                xas_set_err(xas, -(RUST_SHMEM_EEXIST as c_int));
                break 'split;
            }
            let mut cur = xas_get_order(xas);
            if cur == 0 {
                break 'split;
            }
            let swap_index = round_down(index, 1 << cur);
            let mut order = xas_try_split_min_order(cur);
            while cur > 0 {
                let aligned = round_down(index, 1 << cur);
                let offset = aligned.wrapping_sub(swap_index);
                xas_set_order(xas, index, order);
                xas_try_split(xas, old, cur);
                if xas_error(xas) != 0 {
                    break 'split;
                }
                let mut i: c_ulong = 0;
                while i < 1 << cur {
                    let tmp = swp_entry(
                        swp_type(swap),
                        swp_offset(swap).wrapping_add(offset).wrapping_add(i),
                    );
                    __xa_store(
                        addr_of_mut!((*mapping).i_pages),
                        aligned.wrapping_add(i),
                        swp_to_radix_entry(tmp),
                        0,
                    );
                    i += 1 << order;
                }
                cur = order;
                order = xas_try_split_min_order(order);
            }
        }
        rust_shmem_xas_unlock_irq(xas);
        if !xas_nomem(xas, gfp & GFP_RECLAIM_MASK as gfp_t) {
            break;
        }
    }
    xas_error(xas)
}
unsafe fn shmem_swapin_folio(
    inode: *mut inode,
    mut index: Pgoff,
    foliop: *mut *mut folio,
    sgp: sgp_type,
    gfp: gfp_t,
    vmf: *mut vm_fault,
    fault_type: *mut vm_fault_t,
) -> c_int {
    let mapping = (*inode).i_mapping;
    let vma = if vmf.is_null() {
        null_mut()
    } else {
        rust_shmem_vmf_vma(vmf)
    };
    let mm = if vmf.is_null() {
        null_mut()
    } else {
        (*vma).vm_mm
    };
    let info = SHMEM_I(inode);
    vm_bug!((*foliop).is_null() || !xa_is_value((*foliop).cast()));
    let original = radix_to_swp_entry((*foliop).cast());
    let mut swap = original;
    *foliop = null_mut();
    if softleaf_is_poison_marker(original) {
        return -(RUST_SHMEM_EIO as c_int);
    }
    let si = get_swap_device(original);
    let order = shmem_confirm_swap(mapping, index, original);
    if si.is_null() {
        return if order < 0 {
            -(RUST_SHMEM_EEXIST as c_int)
        } else {
            -(RUST_SHMEM_EINVAL as c_int)
        };
    }
    if order < 0 {
        put_swap_device(si);
        return -(RUST_SHMEM_EEXIST as c_int);
    }
    if order != 0 {
        swap = swp_entry(
            swp_type(swap),
            swp_offset(swap).wrapping_add(index.wrapping_sub(round_down(index, 1 << order))),
        );
    }
    let mut f = swap_cache_get_folio(swap);
    let mut locked = false;
    let mut recheck = true;
    let mut error;
    'attempt: {
        if f.is_null() {
            f = if rust_shmem_swap_synchronous(si) {
                shmem_swap_alloc_folio(inode, vmf, index, swap, order, gfp)
            } else {
                shmem_swapin_cluster(swap, gfp, info, index)
            };
            if IS_ERR_OR_NULL(f.cast()) {
                error = if f.is_null() {
                    -(RUST_SHMEM_ENOMEM as c_int)
                } else {
                    ptr_err(f)
                };
                f = null_mut();
                break 'attempt;
            }
            if !fault_type.is_null() {
                *fault_type |= VM_FAULT_MAJOR as vm_fault_t;
                count_vm_event(PGMAJFAULT);
                count_memcg_event_mm(mm, PGMAJFAULT);
            }
        } else {
            swap_update_readahead(f, null_mut(), 0);
        }
        if order > folio_order(f) as c_int {
            error = shmem_split_large_entry(inode, index, original, gfp);
            if error != 0 {
                recheck = false;
                break 'attempt;
            }
        }
        let nr = folio_nr_pages(f) as c_long;
        if nr > 1 {
            swap.val = round_down(swap.val, nr as c_ulong);
            index = round_down(index, nr as c_ulong);
        }
        folio_lock(f);
        locked = true;
        if !folio_matches_swap_entry(f, swap) || shmem_confirm_swap(mapping, index, swap) < 0 {
            error = -(RUST_SHMEM_EEXIST as c_int);
            recheck = false;
            break 'attempt;
        }
        if !folio_test_uptodate(f) {
            error = -(RUST_SHMEM_EIO as c_int);
            break 'attempt;
        }
        folio_wait_writeback(f);
        arch_swap_restore(folio_swap(swap, f), f);
        if shmem_should_replace_folio(f, gfp) {
            error = shmem_replace_folio(&mut f, gfp, info, index, vma);
            if error != 0 {
                break 'attempt;
            }
        }
        error = shmem_add_to_page_cache(f, mapping, index, swp_to_radix_entry(swap), gfp);
        if error != 0 {
            break 'attempt;
        }
        shmem_recalc_inode(inode, 0, -nr);
        if sgp == SGP_WRITE {
            folio_mark_accessed(f);
        }
        folio_put_swap(f, null_mut());
        swap_cache_del_folio(f);
        folio_mark_dirty(f);
        put_swap_device(si);
        *foliop = f;
        return 0;
    }
    if recheck {
        if shmem_confirm_swap(mapping, index, swap) < 0 {
            error = -(RUST_SHMEM_EEXIST as c_int);
        }
        if error == -(RUST_SHMEM_EIO as c_int) {
            shmem_set_folio_swapin_error(inode, index, f, swap);
        }
    }
    if !f.is_null() {
        if locked {
            folio_unlock(f);
        }
        folio_put(f);
    }
    put_swap_device(si);
    error
}
unsafe fn shmem_get_folio_gfp(
    inode: *mut inode,
    index: Pgoff,
    write_end: loff_t,
    foliop: *mut *mut folio,
    mut sgp: sgp_type,
    gfp: gfp_t,
    vmf: *mut vm_fault,
    fault_type: *mut vm_fault_t,
) -> c_int {
    let vma = if vmf.is_null() {
        null_mut()
    } else {
        rust_shmem_vmf_vma(vmf)
    };
    if rust_shmem_warn_mapping(!shmem_mapping((*inode).i_mapping)) {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    if index as u64 > (RUST_SHMEM_MAX_LFS_FILESIZE as u64 >> RUST_SHMEM_PAGE_SHIFT) {
        return -(RUST_SHMEM_EFBIG as c_int);
    }
    loop {
        if sgp <= SGP_CACHE && ((index as loff_t) << RUST_SHMEM_PAGE_SHIFT) >= i_size_read(inode) {
            return -(RUST_SHMEM_EINVAL as c_int);
        }
        let mm = if vma.is_null() {
            null_mut()
        } else {
            (*vma).vm_mm
        };
        let mut allocated = false;
        let mut f = filemap_get_entry((*inode).i_mapping, index);
        if !f.is_null() && !vma.is_null() && userfaultfd_minor(vma) {
            if !xa_is_value(f.cast()) {
                folio_put(f);
            }
            *fault_type = handle_userfault(vmf, RUST_SHMEM_VM_UFFD_MINOR);
            return 0;
        }
        if xa_is_value(f.cast()) {
            let error = shmem_swapin_folio(inode, index, &mut f, sgp, gfp, vmf, fault_type);
            if error == -(RUST_SHMEM_EEXIST as c_int) {
                continue;
            }
            *foliop = f;
            return error;
        }
        let mut clear = false;
        if !f.is_null() {
            folio_lock(f);
            if (*rust_shmem_folio_mapping_ptr(f)) != (*inode).i_mapping {
                folio_unlock(f);
                folio_put(f);
                continue;
            }
            if sgp == SGP_WRITE {
                folio_mark_accessed(f);
            }
            if folio_test_uptodate(f) {
                *foliop = f;
                return 0;
            }
            if sgp != SGP_READ {
                clear = true;
            } else {
                folio_unlock(f);
                folio_put(f);
            }
        }
        if !clear {
            *foliop = null_mut();
            if sgp == SGP_READ {
                return 0;
            }
            if sgp == SGP_NOALLOC {
                return -(RUST_SHMEM_ENOENT as c_int);
            }
            if !vma.is_null() && userfaultfd_missing(vma) {
                *fault_type = handle_userfault(vmf, RUST_SHMEM_VM_UFFD_MISSING);
                return 0;
            }
            let orders = shmem_allowable_huge_orders(inode, vma, index, write_end, false);
            let mut huge = false;
            if orders != 0 {
                let hgfp = thp_shmem_limit_gfp_mask(vma_thp_gfp_mask(vma), gfp);
                f = shmem_alloc_and_add_folio(vmf, hgfp, inode, index, mm, orders);
                if !IS_ERR(f.cast()) {
                    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
                    if folio_test_pmd_mappable(f) {
                        count_vm_event(RUST_SHMEM_THP_FILE_ALLOC);
                    }
                    count_mthp_stat(folio_order(f) as _, MTHP_STAT_SHMEM_ALLOC);
                    huge = true;
                } else if ptr_err(f) == -(RUST_SHMEM_EEXIST as c_int) {
                    continue;
                }
            }
            if !huge {
                f = shmem_alloc_and_add_folio(vmf, gfp, inode, index, mm, 0);
                if IS_ERR(f.cast()) {
                    let error = ptr_err(f);
                    if error == -(RUST_SHMEM_EEXIST as c_int) {
                        continue;
                    }
                    shmem_recalc_inode(inode, 0, 0);
                    return error;
                }
            }
            allocated = true;
            if folio_test_large(f)
                && (i_size_read(inode) as u64).wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
                    / (RUST_SHMEM_PAGE_SIZE as u64)
                    < folio_next_index(f) as u64
            {
                let sb = SHMEM_SB((*inode).i_sb);
                let node = rust_shmem_inode_shrinklist(SHMEM_I(inode));
                spin_lock(addr_of_mut!((*sb).shrinklist_lock));
                if rust_shmem_list_empty_careful(node) {
                    list_add_tail(node, addr_of_mut!((*sb).shrinklist));
                    (*sb).shrinklist_len = (*sb).shrinklist_len.wrapping_add(1);
                }
                spin_unlock(addr_of_mut!((*sb).shrinklist_lock));
            }
            if sgp == SGP_WRITE {
                folio_set_referenced(f);
            }
            if sgp == SGP_FALLOC {
                sgp = SGP_WRITE;
            }
        }
        if sgp != SGP_WRITE && !folio_test_uptodate(f) {
            for i in 0..folio_nr_pages(f) {
                clear_highpage(rust_shmem_folio_page(f, i as _));
            }
            flush_dcache_folio(f);
            folio_mark_uptodate(f);
        }
        if sgp <= SGP_CACHE && ((index as loff_t) << RUST_SHMEM_PAGE_SHIFT) >= i_size_read(inode) {
            if allocated {
                filemap_remove_folio(f);
            }
            shmem_recalc_inode(inode, 0, 0);
            folio_unlock(f);
            folio_put(f);
            return -(RUST_SHMEM_EINVAL as c_int);
        }
        *foliop = f;
        return 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_get_folio(
    inode: *mut inode,
    index: Pgoff,
    end: loff_t,
    foliop: *mut *mut folio,
    sgp: sgp_type,
) -> c_int {
    shmem_get_folio_gfp(
        inode,
        index,
        end,
        foliop,
        sgp,
        mapping_gfp_mask((*inode).i_mapping),
        null_mut(),
        null_mut(),
    )
}
