// SPDX-License-Identifier: GPL-2.0-only
// C:498-536,881-1114. XArray cursor storage is native and config-derived.
unsafe fn shmem_replace_entry(
    mapping: *mut address_space,
    index: Pgoff,
    expected: *mut c_void,
    replacement: *mut c_void,
) -> c_int {
    let mut xas = MaybeUninit::<xa_state>::uninit();
    rust_shmem_xas_init(xas.as_mut_ptr(), addr_of_mut!((*mapping).i_pages), index, 0);
    let xas = xas.as_mut_ptr();
    vm_bug!(expected.is_null());
    vm_bug!(replacement.is_null());
    if xas_load(xas) != expected {
        return -(RUST_SHMEM_ENOENT as c_int);
    }
    xas_store(xas, replacement);
    0
}
unsafe fn shmem_confirm_swap(
    mapping: *mut address_space,
    index: Pgoff,
    swap: swp_entry_t,
) -> c_int {
    let mut xas = MaybeUninit::<xa_state>::uninit();
    rust_shmem_xas_init(xas.as_mut_ptr(), addr_of_mut!((*mapping).i_pages), index, 0);
    let xas = xas.as_mut_ptr();
    let mut ret = -1;
    rcu_read_lock();
    loop {
        let entry = xas_load(xas);
        if entry == swp_to_radix_entry(swap) {
            ret = xas_get_order(xas) as c_int;
        }
        if !xas_retry(xas, entry) {
            break;
        }
    }
    rcu_read_unlock();
    ret
}
unsafe fn shmem_update_stats(folio: *mut folio, nr: c_int) {
    if folio_test_pmd_mappable(folio) {
        lruvec_stat_mod_folio(folio, NR_SHMEM_THPS, nr as _);
    }
    lruvec_stat_mod_folio(folio, NR_FILE_PAGES, nr as _);
    lruvec_stat_mod_folio(folio, NR_SHMEM, nr as _);
}
#[no_mangle]
pub unsafe extern "C" fn shmem_add_to_page_cache(
    folio: *mut folio,
    mapping: *mut address_space,
    index: Pgoff,
    expected: *mut c_void,
    mut gfp: gfp_t,
) -> c_int {
    let mut xs = MaybeUninit::<xa_state>::uninit();
    rust_shmem_xas_init(
        xs.as_mut_ptr(),
        addr_of_mut!((*mapping).i_pages),
        index,
        folio_order(folio),
    );
    let xas = xs.as_mut_ptr();
    let nr = folio_nr_pages(folio) as c_ulong;
    vm_bug_folio!(index != round_down(index, nr), folio);
    vm_bug_folio!(!folio_test_locked(folio), folio);
    vm_bug_folio!(!folio_test_swapbacked(folio), folio);
    folio_ref_add(folio, nr as _);
    (*rust_shmem_folio_mapping_ptr(folio)) = mapping;
    (*rust_shmem_folio_index_ptr(folio)) = index;
    gfp &= GFP_RECLAIM_MASK as gfp_t;
    folio_throttle_swaprate(folio, gfp);
    let swap = radix_to_swp_entry(expected);
    loop {
        let mut iter = swap;
        rust_shmem_xas_lock_irq(xas);
        'insert: {
            loop {
                let entry = xas_find_conflict(xas);
                if entry.is_null() {
                    break;
                }
                if expected.is_null() || entry != swp_to_radix_entry(iter) {
                    xas_set_err(xas, -(RUST_SHMEM_EEXIST as c_int));
                    break 'insert;
                }
                iter.val = iter.val.wrapping_add(1 << xas_get_order(xas));
            }
            if !expected.is_null() && iter.val.wrapping_sub(nr) != swap.val {
                xas_set_err(xas, -(RUST_SHMEM_EEXIST as c_int));
                break 'insert;
            }
            xas_store(xas, folio.cast());
            if xas_error(xas) != 0 {
                break 'insert;
            }
            shmem_update_stats(folio, nr as c_int);
            (*mapping).nrpages = (*mapping).nrpages.wrapping_add(nr);
        }
        rust_shmem_xas_unlock_irq(xas);
        if !xas_nomem(xas, gfp) {
            break;
        }
    }
    let err = xas_error(xas);
    if err != 0 {
        (*rust_shmem_folio_mapping_ptr(folio)) = null_mut();
        folio_ref_sub(folio, nr as _);
    }
    err
}
unsafe fn shmem_delete_from_page_cache(folio: *mut folio, radswap: *mut c_void) {
    let mapping = (*rust_shmem_folio_mapping_ptr(folio));
    let nr = folio_nr_pages(folio) as c_long;
    rust_shmem_xa_lock_irq(addr_of_mut!((*mapping).i_pages));
    let error = shmem_replace_entry(
        mapping,
        (*rust_shmem_folio_index_ptr(folio)),
        folio.cast(),
        radswap,
    );
    (*rust_shmem_folio_mapping_ptr(folio)) = null_mut();
    (*mapping).nrpages = (*mapping).nrpages.wrapping_sub(nr as c_ulong);
    shmem_update_stats(folio, nr.wrapping_neg() as c_int);
    rust_shmem_xa_unlock_irq(addr_of_mut!((*mapping).i_pages));
    folio_put_refs(folio, nr as _);
    rust_shmem_bug(error != 0);
}
unsafe fn shmem_free_swap(
    mapping: *mut address_space,
    index: Pgoff,
    end: Pgoff,
    radswap: *mut c_void,
) -> c_long {
    let mut xs = MaybeUninit::<xa_state>::uninit();
    rust_shmem_xas_init(xs.as_mut_ptr(), addr_of_mut!((*mapping).i_pages), index, 0);
    let xas = xs.as_mut_ptr();
    let mut nr: c_uint = 0;
    rust_shmem_xas_lock_irq(xas);
    if xas_load(xas) == radswap {
        nr = 1 << xas_get_order(xas);
        let base = round_down((*xas).xa_index, nr as c_ulong);
        if base < index || base.wrapping_add(nr as c_ulong).wrapping_sub(1) > end {
            nr = 0;
        } else {
            xas_store(xas, null_mut());
        }
    }
    rust_shmem_xas_unlock_irq(xas);
    if nr != 0 {
        swap_put_entries_direct(radix_to_swp_entry(radswap), nr);
    }
    nr as c_long
}
#[no_mangle]
pub unsafe extern "C" fn shmem_partial_swap_usage(
    mapping: *mut address_space,
    start: Pgoff,
    end: Pgoff,
) -> c_ulong {
    let mut xs = MaybeUninit::<xa_state>::uninit();
    rust_shmem_xas_init(xs.as_mut_ptr(), addr_of_mut!((*mapping).i_pages), start, 0);
    let xas = xs.as_mut_ptr();
    let mut swapped: c_ulong = 0;
    let max = end.wrapping_sub(1);
    rcu_read_lock();
    let mut entry = xas_find(xas, max);
    while !entry.is_null() {
        if !xas_retry(xas, entry) {
            if xa_is_value(entry) {
                swapped = swapped.wrapping_add(1 << xas_get_order(xas));
            }
            if (*xas).xa_index == max {
                break;
            }
            if need_resched() {
                xas_pause(xas);
                cond_resched_rcu();
            }
        }
        entry = xas_next_entry(xas, max);
    }
    rcu_read_unlock();
    swapped << RUST_SHMEM_PAGE_SHIFT
}
#[no_mangle]
pub unsafe extern "C" fn shmem_swap_usage(vma: *mut vm_area_struct) -> c_ulong {
    let inode = file_inode((*vma).vm_file);
    let info = SHMEM_I(inode);
    let swapped = rust_shmem_read_swapped(info);
    let start = vma_start_pgoff(vma);
    if swapped == 0 {
        return 0;
    }
    if start == 0
        && rust_shmem_vma_vm_end(vma).wrapping_sub(rust_shmem_vma_vm_start(vma)) as u64
            >= (*inode).i_size as u64
    {
        return swapped << RUST_SHMEM_PAGE_SHIFT;
    }
    shmem_partial_swap_usage((*inode).i_mapping, start, vma_end_pgoff(vma))
}
#[no_mangle]
pub unsafe extern "C" fn shmem_unlock_mapping(mapping: *mut address_space) {
    let mut batch = MaybeUninit::<folio_batch>::uninit();
    let fb = batch.as_mut_ptr();
    let mut index: Pgoff = 0;
    folio_batch_init(fb);
    while !mapping_unevictable(mapping)
        && filemap_get_folios(mapping, &mut index, c_ulong::MAX, fb) != 0
    {
        check_move_unevictable_folios(fb);
        folio_batch_release(fb);
        rust_shmem_cond_resched();
    }
}
unsafe fn shmem_get_partial_folio(inode: *mut inode, index: Pgoff) -> *mut folio {
    let mut folio = filemap_get_entry((*inode).i_mapping, index);
    if folio.is_null() {
        return folio;
    }
    if !xa_is_value(folio.cast()) {
        folio_lock(folio);
        if (*rust_shmem_folio_mapping_ptr(folio)) == (*inode).i_mapping {
            return folio;
        }
        folio_unlock(folio);
        folio_put(folio);
    }
    folio = null_mut();
    shmem_get_folio(inode, index, 0, &mut folio, SGP_READ);
    folio
}
