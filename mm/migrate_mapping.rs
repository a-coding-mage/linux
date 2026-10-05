// SPDX-License-Identifier: GPL-2.0
unsafe fn __folio_migrate_mapping(
    mapping: *mut address_space,
    newfolio: *mut folio,
    folio: *mut folio,
    expected_count: c_int,
) -> c_int {
    // XA_STATE is initialized lazily for the mapping path: C's initializer only
    // forms the i_pages address when mapping is NULL; it never reads that field.
    let nr = folio_nr_pages(folio) as c_long;
    if mapping.is_null() {
        if folio_test_large(folio) && folio_test_large_rmappable(folio) {
            if !folio_ref_freeze(folio, expected_count) {
                return E_AGAIN;
            }
            folio_unqueue_deferred_split(folio);
            folio_ref_unfreeze(folio, expected_count);
        }
        folio_set_index(newfolio, folio_index(folio));
        folio_set_mapping_field(newfolio, folio_mapping_field(folio));
        if folio_test_anon(folio) && folio_test_large(folio) && !folio_test_hugetlb(folio) {
            mod_mthp_stat(folio_order(folio) as c_int, MTHP_STAT_NR_ANON, 1);
        }
        if folio_test_swapbacked(folio) {
            __folio_set_swapbacked(newfolio);
        }
        return 0;
    }
    let mut xas = xa_state(mapping, folio_index(folio));
    let oldzone = folio_zone(folio);
    let newzone = folio_zone(newfolio);
    let mut ci = null_mut();
    if folio_test_swapcache(folio) {
        ci = swap_cluster_get_and_lock_irq(folio);
    } else {
        xas_lock_irq(&mut xas);
    }
    if !folio_ref_freeze(folio, expected_count) {
        if !ci.is_null() {
            swap_cluster_unlock_irq(ci);
        } else {
            xas_unlock_irq(&mut xas);
        }
        return E_AGAIN;
    }
    folio_unqueue_deferred_split(folio);
    folio_set_index(newfolio, folio_index(folio));
    folio_set_mapping_field(newfolio, folio_mapping_field(folio));
    if folio_test_anon(folio) && folio_test_large(folio) {
        mod_mthp_stat(folio_order(folio) as c_int, MTHP_STAT_NR_ANON, 1);
    }
    folio_ref_add(newfolio, nr as c_int);
    if folio_test_swapbacked(folio) {
        __folio_set_swapbacked(newfolio);
    }
    if folio_test_swapcache(folio) {
        folio_set_swapcache(newfolio);
        folio_set_private_field(newfolio, folio_get_private(folio));
    }
    let dirty = folio_test_dirty(folio);
    if dirty {
        folio_clear_dirty(folio);
        folio_set_dirty(newfolio);
    }
    if folio_test_swapcache(folio) {
        __swap_cache_replace_folio(ci, folio, newfolio);
    } else {
        xas_store(&mut xas, newfolio.cast());
    }
    folio_ref_unfreeze(folio, (expected_count as c_long).wrapping_sub(nr) as c_int);
    if !ci.is_null() {
        swap_cluster_unlock(ci);
    } else {
        xas_unlock(&mut xas);
    }
    // IRQs remain disabled until node/zone accounting finishes.
    if newzone != oldzone {
        rcu_read_lock();
        let memcg = folio_memcg(folio);
        let old_lruvec = mem_cgroup_lruvec(memcg, zone_pgdat(oldzone));
        let new_lruvec = mem_cgroup_lruvec(memcg, zone_pgdat(newzone));
        mod_lruvec_state(old_lruvec, NR_FILE_PAGES, nr.wrapping_neg());
        mod_lruvec_state(new_lruvec, NR_FILE_PAGES, nr);
        if folio_test_swapbacked(folio) && !folio_test_swapcache(folio) {
            mod_lruvec_state(old_lruvec, NR_SHMEM, nr.wrapping_neg());
            mod_lruvec_state(new_lruvec, NR_SHMEM, nr);
            if folio_test_pmd_mappable(folio) {
                mod_lruvec_state(old_lruvec, NR_SHMEM_THPS, nr.wrapping_neg());
                mod_lruvec_state(new_lruvec, NR_SHMEM_THPS, nr);
            }
        }
        #[cfg(CONFIG_SWAP)]
        if folio_test_swapcache(folio) {
            mod_lruvec_state(old_lruvec, NR_SWAPCACHE, nr.wrapping_neg());
            mod_lruvec_state(new_lruvec, NR_SWAPCACHE, nr);
        }
        if dirty && mapping_can_writeback(mapping) {
            mod_lruvec_state(old_lruvec, NR_FILE_DIRTY, nr.wrapping_neg());
            __mod_zone_page_state(oldzone, NR_ZONE_WRITE_PENDING, nr.wrapping_neg());
            mod_lruvec_state(new_lruvec, NR_FILE_DIRTY, nr);
            __mod_zone_page_state(newzone, NR_ZONE_WRITE_PENDING, nr);
        }
        rcu_read_unlock();
    }
    local_irq_enable();
    0
}

#[no_mangle]
pub unsafe extern "C" fn folio_migrate_mapping(
    mapping: *mut address_space,
    newfolio: *mut folio,
    folio: *mut folio,
    extra_count: c_int,
) -> c_int {
    let expected_count = folio_expected_ref_count(folio)
        .wrapping_add(extra_count)
        .wrapping_add(1);
    if folio_ref_count(folio) != expected_count {
        return E_AGAIN;
    }
    __folio_migrate_mapping(mapping, newfolio, folio, expected_count)
}

#[no_mangle]
pub unsafe extern "C" fn migrate_huge_page_move_mapping(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
) -> c_int {
    let mut xas = xa_state(mapping, folio_index(src));
    let expected_count = folio_expected_ref_count(src).wrapping_add(1);
    if folio_ref_count(src) != expected_count {
        return E_AGAIN;
    }
    let rc = folio_mc_copy(dst, src);
    if rc != 0 {
        return rc;
    }
    xas_lock_irq(&mut xas);
    if !folio_ref_freeze(src, expected_count) {
        xas_unlock_irq(&mut xas);
        return E_AGAIN;
    }
    folio_set_index(dst, folio_index(src));
    folio_set_mapping_field(dst, folio_mapping_field(src));
    folio_ref_add(dst, folio_nr_pages(dst) as c_int);
    xas_store(&mut xas, dst.cast());
    folio_ref_unfreeze(
        src,
        (expected_count as c_ulong).wrapping_sub(folio_nr_pages(src)) as c_int,
    );
    xas_unlock_irq(&mut xas);
    0
}

#[no_mangle]
pub unsafe extern "C" fn folio_migrate_flags(newfolio: *mut folio, folio: *mut folio) {
    if folio_test_referenced(folio) {
        folio_set_referenced(newfolio);
    }
    if folio_test_uptodate(folio) {
        folio_mark_uptodate(newfolio);
    }
    if folio_test_clear_active(folio) {
        vm_diag!(bug_active_unevictable(folio_test_unevictable(folio), folio));
        folio_set_active(newfolio);
    } else if folio_test_clear_unevictable(folio) {
        folio_set_unevictable(newfolio);
    }
    if folio_test_workingset(folio) {
        folio_set_workingset(newfolio);
    }
    if folio_test_checked(folio) {
        folio_set_checked(newfolio);
    }
    if folio_test_mappedtodisk(folio) {
        folio_set_mappedtodisk(newfolio);
    }
    if folio_test_dirty(folio) {
        folio_set_dirty(newfolio);
    }
    if folio_test_young(folio) {
        folio_set_young(newfolio);
    }
    if folio_test_idle(folio) {
        folio_set_idle(newfolio);
    }
    folio_migrate_refs(newfolio, folio);
    let mut cpupid = folio_xchg_last_cpupid(folio, -1);
    if numa_balancing_mode() & RUST_MIGRATE_NUMA_BALANCING_MEMORY_TIERING as c_int != 0 {
        if node_is_toptier(folio_nid(folio)) != node_is_toptier(folio_nid(newfolio)) {
            cpupid = -1;
        }
    }
    folio_xchg_last_cpupid(newfolio, cpupid);
    folio_migrate_ksm(newfolio, folio);
    // KSM migration must precede clearing swapcache (ksm_get_folio ordering).
    if folio_test_swapcache(folio) {
        folio_clear_swapcache(folio);
    }
    folio_clear_private(folio);
    if !folio_test_hugetlb(folio) {
        folio_set_private_field(folio, null_mut());
    }
    if folio_test_writeback(newfolio) {
        folio_end_writeback(newfolio);
    }
    // End-writeback may clear the shared reclaim/readahead bit.
    if folio_test_readahead(folio) {
        folio_set_readahead(newfolio);
    }
    folio_copy_owner(newfolio, folio);
    pgalloc_tag_swap(newfolio, folio);
    mem_cgroup_migrate(folio, newfolio);
}

unsafe fn __migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    src_private: *mut c_void,
    _mode: migrate_mode,
) -> c_int {
    let expected_count = folio_expected_ref_count(src).wrapping_add(1);
    if folio_ref_count(src) != expected_count {
        return E_AGAIN;
    }
    let rc = folio_mc_copy(dst, src);
    if rc != 0 {
        return rc;
    }
    let rc = __folio_migrate_mapping(mapping, dst, src, expected_count);
    if rc != 0 {
        return rc;
    }
    if !src_private.is_null() {
        folio_attach_private(dst, folio_detach_private(src));
    }
    folio_migrate_flags(dst, src);
    0
}

#[no_mangle]
pub unsafe extern "C" fn migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
) -> c_int {
    bug_migrate_writeback(folio_test_writeback(src));
    __migrate_folio(mapping, dst, src, null_mut(), mode)
}

#[cfg(CONFIG_BUFFER_HEAD)]
unsafe fn buffer_migrate_lock_buffers(head: *mut buffer_head, mode: migrate_mode) -> bool {
    let mut bh = head;
    loop {
        if !trylock_buffer(bh) {
            if mode == MIGRATE_ASYNC || (mode == MIGRATE_SYNC_LIGHT && !buffer_uptodate(bh)) {
                let failed_bh = bh;
                bh = head;
                while bh != failed_bh {
                    unlock_buffer(bh);
                    bh = bh_next(bh);
                }
                return false;
            }
            lock_buffer(bh);
        }
        bh = bh_next(bh);
        if bh == head {
            return true;
        }
    }
}

#[cfg(CONFIG_BUFFER_HEAD)]
unsafe fn __buffer_migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
    check_refs: bool,
) -> c_int {
    let head = folio_buffers(src);
    if head.is_null() {
        return migrate_folio(mapping, dst, src, mode);
    }
    if folio_ref_count(src) != folio_expected_ref_count(src).wrapping_add(1) {
        return E_AGAIN;
    }
    if !buffer_migrate_lock_buffers(head, mode) {
        return E_AGAIN;
    }
    let rc = 'work: {
        if check_refs {
            let migrating = test_and_set_bit_lock(RUST_MIGRATE_BH_Migrate as _, bh_state(head));
            if migrating {
                vm_diag!(warn_buffer_migrating(true));
            }
            let mut invalidated = false;
            loop {
                let mut busy = false;
                spin_lock(mapping_private_lock(mapping));
                let mut bh = head;
                loop {
                    if atomic_read(bh_count(bh)) != 0 {
                        busy = true;
                        break;
                    }
                    bh = bh_next(bh);
                    if bh == head {
                        break;
                    }
                }
                spin_unlock(mapping_private_lock(mapping));
                if !busy {
                    break;
                }
                if invalidated {
                    break 'work E_AGAIN;
                }
                invalidate_bh_lrus();
                invalidated = true;
            }
        }
        let rc = filemap_migrate_folio(mapping, dst, src, mode);
        if rc != 0 {
            break 'work rc;
        }
        let mut bh = head;
        loop {
            folio_set_bh(bh, dst, bh_offset(bh));
            bh = bh_next(bh);
            if bh == head {
                break;
            }
        }
        0
    };
    if check_refs {
        clear_bit_unlock(RUST_MIGRATE_BH_Migrate as _, bh_state(head));
    }
    let mut bh = head;
    loop {
        unlock_buffer(bh);
        bh = bh_next(bh);
        if bh == head {
            break;
        }
    }
    rc
}

#[cfg(CONFIG_BUFFER_HEAD)]
#[no_mangle]
pub unsafe extern "C" fn buffer_migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
) -> c_int {
    __buffer_migrate_folio(mapping, dst, src, mode, false)
}
#[cfg(CONFIG_BUFFER_HEAD)]
#[no_mangle]
pub unsafe extern "C" fn buffer_migrate_folio_norefs(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
) -> c_int {
    __buffer_migrate_folio(mapping, dst, src, mode, true)
}

#[no_mangle]
pub unsafe extern "C" fn filemap_migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
) -> c_int {
    __migrate_folio(mapping, dst, src, folio_get_private(src), mode)
}

unsafe fn fallback_migrate_folio(
    mapping: *mut address_space,
    dst: *mut folio,
    src: *mut folio,
    mode: migrate_mode,
) -> c_int {
    warn_missing_migrate_folio(mapping);
    if folio_test_dirty(src) {
        return E_BUSY;
    }
    if !filemap_release_folio(src, RUST_MIGRATE_GFP_KERNEL as _) {
        return if mode == MIGRATE_SYNC {
            E_AGAIN
        } else {
            E_BUSY
        };
    }
    migrate_folio(mapping, dst, src, mode)
}

unsafe fn move_to_new_folio(dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    let mapping = folio_mapping(src);
    vm_diag!(bug_move_source_lock(!folio_test_locked(src), src));
    vm_diag!(bug_move_destination_lock(!folio_test_locked(dst), dst));
    let rc = if mapping.is_null() {
        migrate_folio(mapping, dst, src, mode)
    } else if mapping_inaccessible(mapping) {
        E_OPNOTSUPP
    } else if let Some(callback) = mapping_migrate_folio(mapping) {
        callback(mapping, dst, src, mode)
    } else {
        fallback_migrate_folio(mapping, dst, src, mode)
    };
    if rc == 0 {
        if !folio_test_anon(src) {
            folio_set_mapping_field(src, null_mut());
        }
        if !folio_is_zone_device(dst) {
            flush_dcache_folio(dst);
        }
    }
    rc
}
