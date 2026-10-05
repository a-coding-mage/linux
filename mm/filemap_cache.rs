// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:129-345,811-1060.
unsafe fn page_cache_delete(mapping: *mut address_space, folio: *mut folio, shadow: *mut c_void) {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), folio_index(folio));
    mapping_set_update(&mut xas, mapping);
    xas_set_order(&mut xas, folio_index(folio), folio_order(folio));
    let nr = folio_nr_pages(folio);
    vm_bug_folio!((!folio_test_locked(folio)) as c_int, folio);
    xas_store(&mut xas, shadow);
    xas_init_marks(&mut xas);
    folio_set_mapping(folio, null_mut());
    // Deliberately retain index: truncation lookup depends on it.
    (*mapping_nrpages(mapping)) = (*mapping_nrpages(mapping)).wrapping_sub(nr);
}
unsafe fn filemap_unaccount_folio(mapping: *mut address_space, folio: *mut folio) {
    vm_bug_folio!(folio_mapped(folio) as c_int, folio);
    #[cfg(not(CONFIG_DEBUG_VM))]
    if folio_mapped(folio) {
        rust_filemap_bad_page_cache(folio);
        if mapping_exiting(mapping) != 0 && !folio_test_large(folio) {
            let mapcount = folio_mapcount(folio);
            if folio_ref_count(folio) >= mapcount.wrapping_add(2) {
                atomic_set(folio_mapcount_ptr(folio), -1);
                folio_ref_sub(folio, mapcount);
            }
        }
    }
    if folio_test_hugetlb(folio) {
        return;
    }
    let nr = folio_nr_pages(folio) as c_long;
    lruvec_stat_mod_folio(folio, NR_FILE_PAGES, nr.wrapping_neg() as c_int);
    if folio_test_swapbacked(folio) {
        lruvec_stat_mod_folio(folio, NR_SHMEM, nr.wrapping_neg() as c_int);
        if folio_test_pmd_mappable(folio) {
            lruvec_stat_mod_folio(folio, NR_SHMEM_THPS, nr.wrapping_neg() as c_int);
        }
    } else if folio_test_pmd_mappable(folio) {
        lruvec_stat_mod_folio(folio, NR_FILE_THPS, nr.wrapping_neg() as c_int);
    }
    if test_bit(
        AS_KERNEL_FILE as c_ulong,
        mapping_flags(folio_mapping_field(folio)),
    ) {
        mod_node_page_state(folio_pgdat(folio), NR_KERNEL_FILE_PAGES, nr.wrapping_neg());
    }
    if rust_filemap_warn_dirty_unaccount(folio_test_dirty(folio) && mapping_can_writeback(mapping))
    {
        folio_account_cleaned(folio, inode_to_wb(mapping_host(mapping)));
    }
}
#[no_mangle]
pub unsafe extern "C" fn __filemap_remove_folio(folio: *mut folio, shadow: *mut c_void) {
    let mapping = folio_mapping_field(folio);
    trace_mm_filemap_delete_from_page_cache(folio);
    filemap_unaccount_folio(mapping, folio);
    page_cache_delete(mapping, folio, shadow);
}
unsafe fn filemap_free_folio(mapping: *const address_space, folio: *mut folio) {
    if let Some(free_folio) = (*mapping_aops(mapping)).free_folio {
        free_folio(folio);
    }
    folio_put_refs(folio, folio_nr_pages(folio) as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn filemap_remove_folio(folio: *mut folio) {
    let mapping = folio_mapping_field(folio);
    rust_filemap_bug(!folio_test_locked(folio));
    spin_lock(inode_lock_ptr(mapping_host(mapping)));
    xa_lock_irq(mapping_i_pages(mapping));
    __filemap_remove_folio(folio, null_mut());
    xa_unlock_irq(mapping_i_pages(mapping));
    if mapping_shrinkable(mapping) {
        inode_lru_list_add(mapping_host(mapping));
    }
    spin_unlock(inode_lock_ptr(mapping_host(mapping)));
    filemap_free_folio(mapping, folio);
}
unsafe fn page_cache_delete_batch(mapping: *mut address_space, fbatch: *mut folio_batch) {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), folio_index((*fbatch).folios[0]));
    let mut total_pages: c_ulong = 0;
    let mut i: usize = 0;
    mapping_set_update(&mut xas, mapping);
    let mut folio = xas_find(&mut xas, c_ulong::MAX) as *mut folio;
    while !folio.is_null() {
        if i >= folio_batch_count(fbatch) as usize {
            break;
        }
        if !xa_is_value(folio.cast()) {
            if folio != (*fbatch).folios[i] {
                vm_bug_folio!(
                    (folio_index(folio) > folio_index((*fbatch).folios[i])) as c_int,
                    folio
                );
            } else {
                rust_filemap_warn_delete_unlocked(!folio_test_locked(folio));
                folio_set_mapping(folio, null_mut());
                i += 1;
                xas_store(&mut xas, null_mut());
                total_pages = total_pages.wrapping_add(folio_nr_pages(folio));
            }
        }
        folio = xas_next_entry(&mut xas, c_ulong::MAX) as *mut folio;
    }
    (*mapping_nrpages(mapping)) = (*mapping_nrpages(mapping)).wrapping_sub(total_pages);
}
#[no_mangle]
pub unsafe extern "C" fn delete_from_page_cache_batch(
    mapping: *mut address_space,
    fbatch: *mut folio_batch,
) {
    if folio_batch_count(fbatch) == 0 {
        return;
    }
    spin_lock(inode_lock_ptr(mapping_host(mapping)));
    xa_lock_irq(mapping_i_pages(mapping));
    for i in 0..folio_batch_count(fbatch) as usize {
        let folio = (*fbatch).folios[i];
        trace_mm_filemap_delete_from_page_cache(folio);
        filemap_unaccount_folio(mapping, folio);
    }
    page_cache_delete_batch(mapping, fbatch);
    xa_unlock_irq(mapping_i_pages(mapping));
    if mapping_shrinkable(mapping) {
        inode_lru_list_add(mapping_host(mapping));
    }
    spin_unlock(inode_lock_ptr(mapping_host(mapping)));
    for i in 0..folio_batch_count(fbatch) as usize {
        filemap_free_folio(mapping, (*fbatch).folios[i]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn replace_page_cache_folio(old: *mut folio, new: *mut folio) {
    let mapping = folio_mapping_field(old);
    let free_folio = (*mapping_aops(mapping)).free_folio;
    let offset = folio_index(old);
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), offset);
    vm_bug_folio!((!folio_test_locked(old)) as c_int, old);
    vm_bug_folio!((!folio_test_locked(new)) as c_int, new);
    vm_bug_folio!((!folio_mapping_field(new).is_null()) as c_int, new);
    folio_get(new);
    folio_set_mapping(new, mapping);
    folio_set_index(new, offset);
    mem_cgroup_replace_folio(old, new);
    xas_lock_irq(&mut xas);
    xas_store(&mut xas, new.cast());
    folio_set_mapping(old, null_mut());
    if !folio_test_hugetlb(old) {
        lruvec_stat_sub_folio(old, NR_FILE_PAGES);
    }
    if !folio_test_hugetlb(new) {
        lruvec_stat_add_folio(new, NR_FILE_PAGES);
    }
    if folio_test_swapbacked(old) {
        lruvec_stat_sub_folio(old, NR_SHMEM);
    }
    if folio_test_swapbacked(new) {
        lruvec_stat_add_folio(new, NR_SHMEM);
    }
    xas_unlock_irq(&mut xas);
    if let Some(free_folio) = free_folio {
        free_folio(old);
    }
    folio_put(old);
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn __filemap_add_folio(
    mapping: *mut address_space,
    folio: *mut folio,
    index: Pgoff,
    mut gfp: gfp_t,
    shadowp: *mut *mut c_void,
) -> c_int {
    let forder = folio_order(folio);
    let mut xas = rust_filemap_xa_state_order(mapping_i_pages(mapping), index, forder);
    vm_bug_folio!((!folio_test_locked(folio)) as c_int, folio);
    vm_bug_folio!(folio_test_swapbacked(folio) as c_int, folio);
    vm_bug_folio!((forder < mapping_min_folio_order(mapping)) as c_int, folio);
    mapping_set_update(&mut xas, mapping);
    vm_bug_folio!(index & folio_nr_pages(folio).wrapping_sub(1), folio);
    let huge = folio_test_hugetlb(folio);
    let nr = folio_nr_pages(folio);
    gfp &= RUST_FILEMAP_GFP_RECLAIM_MASK;
    folio_ref_add(folio, nr as c_int);
    folio_set_mapping(folio, mapping);
    folio_set_index(folio, xas.xa_index);
    loop {
        let mut order: c_int = -1;
        let mut old = null_mut();
        xas_lock_irq(&mut xas);
        'locked: {
            loop {
                let entry = xas_find_conflict(&mut xas);
                if entry.is_null() {
                    break;
                }
                old = entry;
                if !xa_is_value(entry) {
                    xas_set_err(&mut xas, -(EEXIST as c_long));
                    break 'locked;
                }
                if order == -1 {
                    order = xas_get_order(&mut xas) as c_int;
                }
            }
            if !old.is_null() {
                if order > 0 && order as c_uint > forder {
                    let mut split_order =
                        core::cmp::max(forder, xas_try_split_min_order(order as c_uint));
                    rust_filemap_bug(shmem_mapping(mapping));
                    while order as c_uint > forder {
                        xas_set_order(&mut xas, index, split_order);
                        xas_try_split(&mut xas, old, order as c_uint);
                        if xas_error(&mut xas) != 0 {
                            break 'locked;
                        }
                        order = split_order as c_int;
                        split_order = core::cmp::max(xas_try_split_min_order(split_order), forder);
                    }
                    xas_reset(&mut xas);
                }
                if !shadowp.is_null() {
                    *shadowp = old;
                }
            }
            xas_store(&mut xas, folio.cast());
            if xas_error(&mut xas) != 0 {
                break 'locked;
            }
            (*mapping_nrpages(mapping)) = (*mapping_nrpages(mapping)).wrapping_add(nr);
            if !huge {
                lruvec_stat_mod_folio(folio, NR_FILE_PAGES, nr as c_int);
                if folio_test_pmd_mappable(folio) {
                    lruvec_stat_mod_folio(folio, NR_FILE_THPS, nr as c_int);
                }
            }
        }
        xas_unlock_irq(&mut xas);
        if !xas_nomem(&mut xas, gfp) {
            break;
        }
        xas_set_order(&mut xas, index, forder);
    }
    let err = xas_error(&mut xas);
    if err != 0 {
        folio_set_mapping(folio, null_mut());
        folio_put_refs(folio, nr as c_int);
        return err;
    }
    trace_mm_filemap_add_to_page_cache(folio);
    0
}
#[no_mangle]
pub unsafe extern "C" fn filemap_add_folio(
    mapping: *mut address_space,
    folio: *mut folio,
    index: Pgoff,
    gfp: gfp_t,
) -> c_int {
    let mut shadow = null_mut();
    let kernel_file = test_bit(AS_KERNEL_FILE as c_ulong, mapping_flags(mapping));
    let mut tmp = null_mut();
    if kernel_file {
        tmp = set_active_memcg(rust_filemap_root_mem_cgroup());
    }
    let ret = mem_cgroup_charge(folio, null_mut(), gfp);
    if kernel_file {
        set_active_memcg(tmp);
    }
    if ret != 0 {
        return ret;
    }
    __folio_set_locked(folio);
    let ret = __filemap_add_folio(mapping, folio, index, gfp, &mut shadow);
    if ret != 0 {
        mem_cgroup_uncharge(folio);
        __folio_clear_locked(folio);
    } else {
        rust_filemap_warn_add_active(folio_test_active(folio));
        if gfp & RUST_FILEMAP___GFP_WRITE == 0 && !shadow.is_null() {
            workingset_refault(folio, shadow);
        }
        folio_add_lru(folio);
        if kernel_file {
            mod_node_page_state(
                folio_pgdat(folio),
                NR_KERNEL_FILE_PAGES,
                folio_nr_pages(folio) as c_long,
            );
        }
    }
    ret
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn filemap_alloc_folio_noprof(
    gfp: gfp_t,
    order: c_uint,
    policy: *mut mempolicy,
) -> *mut folio {
    if !policy.is_null() {
        return folio_alloc_mpol_noprof(
            gfp,
            order,
            policy,
            RUST_FILEMAP_NO_INTERLEAVE_INDEX,
            numa_node_id(),
        );
    }
    if cpuset_do_page_mem_spread() {
        loop {
            let cookie = read_mems_allowed_begin();
            let n = cpuset_mem_spread_node();
            let folio = __folio_alloc_node_noprof(gfp, order, n);
            if !folio.is_null() || !read_mems_allowed_retry(cookie) {
                return folio;
            }
        }
    }
    folio_alloc_noprof(gfp, order)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_lock_two(
    mut mapping1: *mut address_space,
    mut mapping2: *mut address_space,
) {
    if mapping1 > mapping2 {
        core::mem::swap(&mut mapping1, &mut mapping2);
    }
    if !mapping1.is_null() {
        down_write(mapping_invalidate_lock(mapping1));
    }
    if !mapping2.is_null() && mapping1 != mapping2 {
        down_write_nested(mapping_invalidate_lock(mapping2), 1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_unlock_two(
    mapping1: *mut address_space,
    mapping2: *mut address_space,
) {
    if !mapping1.is_null() {
        up_write(mapping_invalidate_lock(mapping1));
    }
    if !mapping2.is_null() && mapping1 != mapping2 {
        up_write(mapping_invalidate_lock(mapping2));
    }
}
