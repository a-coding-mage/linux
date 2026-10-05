// SPDX-License-Identifier: GPL-2.0-only
// C:1455-1765. stop_eviction protects unmount as well as unlink.
unsafe fn shmem_find_swap_entries(
    mapping: *mut address_space,
    start: Pgoff,
    fb: *mut folio_batch,
    indices: *mut Pgoff,
    ty: c_uint,
) -> c_uint {
    let mut xs = MaybeUninit::<xa_state>::uninit();
    let xas = xs.as_mut_ptr();
    rust_shmem_xas_init(xas, addr_of_mut!((*mapping).i_pages), start, 0);
    rcu_read_lock();
    let mut entry = xas_find(xas, c_ulong::MAX);
    while !entry.is_null() {
        if !xas_retry(xas, entry) && xa_is_value(entry) && swp_type(radix_to_swp_entry(entry)) == ty
        {
            *indices.add(folio_batch_count(fb) as usize) = (*xas).xa_index;
            if folio_batch_add(fb, entry.cast()) == 0 {
                break;
            }
            if need_resched() {
                xas_pause(xas);
                cond_resched_rcu();
            }
        }
        entry = xas_next_entry(xas, c_ulong::MAX);
    }
    rcu_read_unlock();
    folio_batch_count(fb)
}
unsafe fn shmem_unuse_swap_entries(
    inode: *mut inode,
    fb: *mut folio_batch,
    indices: *mut Pgoff,
) -> c_int {
    let mut count = 0;
    for i in 0..folio_batch_count(fb) as usize {
        let mut f = (*fb).folios[i];
        let error = shmem_swapin_folio(
            inode,
            *indices.add(i),
            &mut f,
            SGP_CACHE,
            mapping_gfp_mask((*inode).i_mapping),
            null_mut(),
            null_mut(),
        );
        if error == 0 {
            folio_unlock(f);
            folio_put(f);
            count += 1;
        }
        if error == -(RUST_SHMEM_ENOMEM as c_int) {
            return error;
        }
    }
    count
}
unsafe fn shmem_unuse_inode(inode: *mut inode, ty: c_uint) -> c_int {
    let mut start = 0;
    let mut batch = MaybeUninit::<folio_batch>::uninit();
    let fb = batch.as_mut_ptr();
    let mut indices = [0 as Pgoff; RUST_SHMEM_FOLIO_BATCH_SIZE as usize];
    loop {
        folio_batch_init(fb);
        if shmem_find_swap_entries((*inode).i_mapping, start, fb, indices.as_mut_ptr(), ty) == 0 {
            return 0;
        }
        let ret = shmem_unuse_swap_entries(inode, fb, indices.as_mut_ptr());
        if ret < 0 {
            return ret;
        }
        start = indices[folio_batch_count(fb) as usize - 1];
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_unuse(ty: c_uint) -> c_int {
    let head = addr_of_mut!(rust_shmem_data_shmem_swaplist);
    let mut error = 0;
    if rust_shmem_list_empty(head) {
        return 0;
    }
    spin_lock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
    let mut pos = (*head).next;
    while pos != head {
        let info = rust_shmem_swaplist_inode(pos);
        let mut next = (*pos).next;
        if (*info).swapped == 0 {
            list_del_init(pos);
            pos = next;
            continue;
        }
        atomic_inc(addr_of_mut!((*info).stop_eviction));
        spin_unlock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
        error = shmem_unuse_inode(addr_of_mut!((*info).vfs_inode), ty);
        rust_shmem_cond_resched();
        spin_lock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
        if atomic_dec_and_test(addr_of_mut!((*info).stop_eviction)) {
            rust_shmem_wake_eviction(info);
        }
        if error != 0 {
            break;
        }
        if rust_shmem_list_empty(pos) {
            pos = (*head).next;
            continue;
        }
        next = (*pos).next;
        if (*info).swapped == 0 {
            list_del_init(pos);
        }
        pos = next;
    }
    spin_unlock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
    error
}
#[no_mangle]
pub unsafe extern "C" fn shmem_writeout(
    ctx: *mut swap_io_ctx,
    folio: *mut folio,
    folio_list: *mut list_head,
) -> c_int {
    let mapping = (*rust_shmem_folio_mapping_ptr(folio));
    let inode = (*mapping).host;
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    'write: {
        if (*info).flags & RUST_SHMEM_SHMEM_F_LOCKED as c_ulong != 0
            || (*sb).noswap
            || total_swap_pages == 0
        {
            break 'write;
        }
        let mut split = false;
        if folio_test_large(folio) {
            let index = shmem_fallocend(
                inode,
                (i_size_read(inode) as u64)
                    .wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
                    .wrapping_shr(RUST_SHMEM_PAGE_SHIFT as u32) as Pgoff,
            );
            split = (index > (*rust_shmem_folio_index_ptr(folio))
                && index < folio_next_index(folio))
                || !cfg!(CONFIG_THP_SWAP);
        }
        loop {
            if split {
                let order = folio_order(folio);
                folio_test_set_dirty(folio);
                if split_folio_to_list(folio, folio_list) != 0 {
                    break 'write;
                }
                #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
                if order >= RUST_SHMEM_HPAGE_PMD_ORDER as c_uint {
                    count_memcg_folio_events(folio, THP_SWPOUT_FALLBACK, 1);
                    count_vm_event(THP_SWPOUT_FALLBACK);
                }
                count_mthp_stat(order as _, MTHP_STAT_SWPOUT_FALLBACK);
                folio_clear_dirty(folio);
            }
            let index = (*rust_shmem_folio_index_ptr(folio));
            let nr = folio_nr_pages(folio) as c_int;
            if !folio_test_uptodate(folio) {
                if !rust_shmem_read_private(inode).is_null() {
                    spin_lock(addr_of_mut!((*inode).i_lock));
                    let mut falloc = (*inode).i_private as *mut shmem_falloc;
                    if !falloc.is_null()
                        && (*falloc).waitq.is_null()
                        && index >= (*falloc).start
                        && index < (*falloc).next
                    {
                        (*falloc).nr_unswapped = (*falloc).nr_unswapped.wrapping_add(nr as Pgoff);
                    } else {
                        falloc = null_mut();
                    }
                    spin_unlock(addr_of_mut!((*inode).i_lock));
                    if !falloc.is_null() {
                        break 'write;
                    }
                }
                folio_zero_range(folio, 0, folio_size(folio));
                flush_dcache_folio(folio);
                folio_mark_uptodate(folio);
            }
            if folio_alloc_swap(folio) == 0 {
                if shmem_recalc_inode(inode, 0, nr as c_long) {
                    spin_lock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
                    let node = rust_shmem_inode_swaplist(info);
                    if rust_shmem_list_empty(node) {
                        list_add(node, addr_of_mut!(rust_shmem_data_shmem_swaplist));
                    }
                    spin_unlock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
                }
                folio_dup_swap(folio, null_mut());
                shmem_delete_from_page_cache(
                    folio,
                    swp_to_radix_entry(rust_shmem_folio_swap(folio)),
                );
                rust_shmem_bug(folio_mapped(folio));
                let error = swap_writeout(ctx, folio);
                if error != AOP_WRITEPAGE_ACTIVATE as c_int {
                    return error;
                }
                let error = shmem_add_to_page_cache(
                    folio,
                    mapping,
                    index,
                    swp_to_radix_entry(rust_shmem_folio_swap(folio)),
                    (RUST_SHMEM___GFP_HIGH | RUST_SHMEM___GFP_NOMEMALLOC | RUST_SHMEM___GFP_NOWARN)
                        as gfp_t,
                );
                if error == 0 {
                    shmem_recalc_inode(inode, 0, -(nr as c_long));
                    folio_put_swap(folio, null_mut());
                }
                swap_cache_del_folio(folio);
                break 'write;
            }
            if nr <= 1 {
                break 'write;
            }
            split = true;
        }
    }
    folio_mark_dirty(folio);
    AOP_WRITEPAGE_ACTIVATE as c_int
}
#[no_mangle]
pub unsafe extern "C" fn shmem_write_folio(folio: *mut folio) -> c_int {
    let mut ctx: swap_io_ctx = zeroed();
    let err = shmem_writeout(&mut ctx, folio, null_mut());
    swap_write_submit(&mut ctx);
    err
}
