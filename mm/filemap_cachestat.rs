// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:4605-4801, CONFIG_CACHESTAT_SYSCALL.
unsafe fn filemap_cachestat(
    mapping: *mut address_space,
    first_index: Pgoff,
    last_index: Pgoff,
    cs: *mut cachestat,
) {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), first_index);
    mem_cgroup_flush_stats_ratelimited(null_mut());
    rcu_read_lock();
    let mut folio = xas_find(&mut xas, last_index) as *mut folio;
    while !folio.is_null() {
        if !xas_retry(&mut xas, folio.cast()) {
            let order = xas_get_order(&mut xas);
            let mut nr_pages = (1 as c_int).wrapping_shl(order as c_uint) as c_ulong;
            let folio_first_index = xas.xa_index & !nr_pages.wrapping_sub(1);
            let folio_last_index = folio_first_index.wrapping_add(nr_pages).wrapping_sub(1);
            if folio_first_index < first_index {
                nr_pages = nr_pages.wrapping_sub(first_index.wrapping_sub(folio_first_index));
            }
            if folio_last_index > last_index {
                nr_pages = nr_pages.wrapping_sub(folio_last_index.wrapping_sub(last_index));
            }
            'resched: {
                if xa_is_value(folio.cast()) {
                    let shadow = folio.cast();
                    let mut workingset = MaybeUninit::<bool>::uninit();
                    (*cs).nr_evicted = (*cs).nr_evicted.wrapping_add(nr_pages as u64);
                    #[cfg(CONFIG_SWAP)]
                    let shadow = if shmem_mapping(mapping) {
                        let swp = radix_to_swp_entry(folio.cast());
                        if !softleaf_is_swap(swp) {
                            break 'resched;
                        }
                        let shadow = swap_cache_get_shadow(swp);
                        if shadow.is_null() {
                            break 'resched;
                        }
                        shadow
                    } else {
                        shadow
                    };
                    if workingset_test_recent(shadow, true, workingset.as_mut_ptr(), false) {
                        (*cs).nr_recently_evicted =
                            (*cs).nr_recently_evicted.wrapping_add(nr_pages as u64);
                    }
                    break 'resched;
                }
                (*cs).nr_cache = (*cs).nr_cache.wrapping_add(nr_pages as u64);
                if xas_get_mark(&xas, RUST_FILEMAP_PAGECACHE_TAG_DIRTY) {
                    (*cs).nr_dirty = (*cs).nr_dirty.wrapping_add(nr_pages as u64);
                }
                if xas_get_mark(&xas, RUST_FILEMAP_PAGECACHE_TAG_WRITEBACK) {
                    (*cs).nr_writeback = (*cs).nr_writeback.wrapping_add(nr_pages as u64);
                }
            }
            if need_resched() {
                xas_pause(&mut xas);
                cond_resched_rcu();
            }
        }
        folio = xas_next_entry(&mut xas, last_index) as *mut folio;
    }
    rcu_read_unlock();
}
unsafe fn can_do_cachestat(f: *mut file) -> bool {
    if file_mode(f) & RUST_FILEMAP_FMODE_WRITE != 0 || file_owner_or_capable(f) {
        return true;
    }
    file_permission(f, RUST_FILEMAP_MAY_WRITE as c_int) == 0
}
// The native SYSCALL_DEFINE4 metadata/ABI thunk invokes this Rust body.
#[no_mangle]
pub unsafe extern "C" fn rust_filemap_cachestat_syscall(
    fd: c_uint,
    cstat_range: *mut cachestat_range,
    cstat: *mut cachestat,
    flags: c_uint,
) -> c_long {
    let f = fdget(fd);
    let ret = 'done: {
        if fd_empty(f) {
            break 'done -(EBADF as c_long);
        }
        let mut csr = MaybeUninit::<cachestat_range>::uninit();
        if copy_from_user(
            csr.as_mut_ptr().cast(),
            cstat_range.cast(),
            size_of::<cachestat_range>() as c_ulong,
        ) != 0
        {
            break 'done -(EFAULT as c_long);
        }
        if is_file_hugepages(fd_file(f)) {
            break 'done -(EOPNOTSUPP as c_long);
        }
        if !can_do_cachestat(fd_file(f)) {
            break 'done -(EPERM as c_long);
        }
        if flags != 0 {
            break 'done -(EINVAL as c_long);
        }
        let csr = csr.assume_init();
        let first_index = (csr.off >> PAGE_SHIFT) as Pgoff;
        let last_index = if csr.len == 0 {
            c_ulong::MAX
        } else {
            (csr.off.wrapping_add(csr.len).wrapping_sub(1) >> PAGE_SHIFT) as Pgoff
        };
        let mut cs: cachestat = zeroed();
        filemap_cachestat(file_mapping(fd_file(f)), first_index, last_index, &mut cs);
        if copy_to_user(
            cstat.cast(),
            addr_of!(cs).cast(),
            size_of::<cachestat>() as c_ulong,
        ) != 0
        {
            break 'done -(EFAULT as c_long);
        }
        0
    };
    fdput(f);
    ret
}
