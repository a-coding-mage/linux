// SPDX-License-Identifier: GPL-2.0
const FOLIO_WAS_MAPPED: c_int = 1 << 0;
const FOLIO_WAS_MLOCKED: c_int = 1 << 1;
const FOLIO_OLD_STATES: c_int = FOLIO_WAS_MAPPED | FOLIO_WAS_MLOCKED;

unsafe fn __migrate_folio_record(dst: *mut folio, old_folio_state: c_int, anon_vma: *mut anon_vma) {
    folio_set_migrate_info(dst, anon_vma as c_ulong | old_folio_state as c_ulong);
}
unsafe fn __migrate_folio_extract(
    dst: *mut folio,
    old_folio_state: *mut c_int,
    anon_vmap: *mut *mut anon_vma,
) {
    let info = folio_migrate_info(dst);
    *anon_vmap = (info & !(FOLIO_OLD_STATES as c_ulong)) as *mut anon_vma;
    *old_folio_state = (info & FOLIO_OLD_STATES as c_ulong) as c_int;
    folio_set_migrate_info(dst, 0);
}
unsafe fn migrate_folio_undo_src(
    src: *mut folio,
    was_mapped: c_int,
    anon_vma: *mut anon_vma,
    locked: bool,
    ret: *mut list_head,
) {
    if was_mapped != 0 {
        remove_migration_ptes(src, src, 0);
    }
    if !anon_vma.is_null() {
        put_anon_vma(anon_vma);
    }
    if locked {
        folio_unlock(src);
    }
    if !ret.is_null() {
        list_move_tail(folio_lru(src), ret);
    }
}
unsafe fn migrate_folio_undo_dst(
    dst: *mut folio,
    locked: bool,
    put_new_folio: FreeFolio,
    private: c_ulong,
) {
    if locked {
        folio_unlock(dst);
    }
    if let Some(put) = put_new_folio {
        put(dst, private);
    } else {
        folio_put(dst);
    }
}
unsafe fn migrate_folio_done(src: *mut folio, reason: migrate_reason) {
    if !page_has_movable_ops(folio_page(src, 0)) && reason != MR_DEMOTION {
        mod_node_page_state(
            folio_pgdat(src),
            isolated_stat(src),
            (folio_nr_pages(src) as c_long).wrapping_neg(),
        );
    }
    if reason != MR_MEMORY_FAILURE {
        folio_put(src);
    }
}

unsafe fn migrate_folio_unmap(
    get_new_folio: NewFolio,
    put_new_folio: FreeFolio,
    private: c_ulong,
    src: *mut folio,
    dstp: *mut *mut folio,
    mode: migrate_mode,
    mut ret: *mut list_head,
) -> c_int {
    let dst = get_new_folio.unwrap()(src, private);
    if dst.is_null() {
        return E_NOMEM;
    }
    *dstp = dst;
    folio_set_migrate_info(dst, 0);
    let mut old_folio_state = 0;
    let mut anon_vma = null_mut();
    let mut locked = false;
    let mut dst_locked = false;
    let rc = 'work: {
        if !folio_trylock(src) {
            if mode == MIGRATE_ASYNC
                || current_flags() & RUST_MIGRATE_PF_MEMALLOC as c_uint != 0
                || (mode == MIGRATE_SYNC_LIGHT && !folio_test_uptodate(src))
            {
                break 'work E_AGAIN;
            }
            folio_lock(src);
        }
        locked = true;
        if folio_test_mlocked(src) {
            old_folio_state |= FOLIO_WAS_MLOCKED;
        }
        if folio_test_writeback(src) {
            if mode != MIGRATE_SYNC {
                break 'work E_BUSY;
            }
            folio_wait_writeback(src);
        }
        if folio_test_anon(src) && !folio_test_ksm(src) {
            anon_vma = folio_get_anon_vma(src);
        }
        if !folio_trylock(dst) {
            break 'work E_AGAIN;
        }
        dst_locked = true;
        if page_has_movable_ops(folio_page(src, 0)) {
            __migrate_folio_record(dst, old_folio_state, anon_vma);
            return 0;
        }
        if folio_mapping_field(src).is_null() {
            if folio_test_private(src) {
                try_to_free_buffers(src);
                break 'work E_AGAIN;
            }
        } else if folio_mapped(src) {
            vm_diag!(bug_unmap_anon_vma(
                folio_test_anon(src) && !folio_test_ksm(src) && anon_vma.is_null(),
                src
            ));
            try_to_migrate(
                src,
                if mode == MIGRATE_ASYNC {
                    TTU_BATCH_FLUSH
                } else {
                    0
                },
            );
            old_folio_state |= FOLIO_WAS_MAPPED;
        }
        if !folio_mapped(src) {
            __migrate_folio_record(dst, old_folio_state, anon_vma);
            return 0;
        }
        E_AGAIN
    };
    if rc == E_AGAIN {
        ret = null_mut();
    }
    migrate_folio_undo_src(
        src,
        old_folio_state & FOLIO_WAS_MAPPED,
        anon_vma,
        locked,
        ret,
    );
    migrate_folio_undo_dst(dst, dst_locked, put_new_folio, private);
    rc
}

unsafe fn migrate_folio_move(
    put_new_folio: FreeFolio,
    private: c_ulong,
    src: *mut folio,
    dst: *mut folio,
    mode: migrate_mode,
    reason: migrate_reason,
    ret: *mut list_head,
) -> c_int {
    let mut old_folio_state = 0;
    let mut anon_vma = null_mut();
    __migrate_folio_extract(dst, &mut old_folio_state, &mut anon_vma);
    let prev = (*folio_lru(dst)).prev;
    list_del(folio_lru(dst));
    let rc = if page_has_movable_ops(folio_page(src, 0)) {
        migrate_movable_ops_page(folio_page(dst, 0), folio_page(src, 0), mode)
    } else {
        let src_deferred_split = folio_order(src) > 1 && !deferred_list_empty_data_race(src);
        let src_partially_mapped = src_deferred_split && folio_test_partially_mapped(src);
        let rc = move_to_new_folio(dst, src, mode);
        if rc == 0 {
            if src_deferred_split {
                deferred_split_folio(dst, src_partially_mapped);
            }
            folio_add_lru(dst);
            if old_folio_state & FOLIO_WAS_MLOCKED != 0 {
                lru_add_drain();
            }
            if old_folio_state & FOLIO_WAS_MAPPED != 0 {
                remove_migration_ptes(src, dst, 0);
            }
        }
        rc
    };
    if rc != 0 {
        if rc == E_AGAIN {
            list_add(folio_lru(dst), prev);
            __migrate_folio_record(dst, old_folio_state, anon_vma);
            return rc;
        }
        migrate_folio_undo_src(src, old_folio_state & FOLIO_WAS_MAPPED, anon_vma, true, ret);
        migrate_folio_undo_dst(dst, true, put_new_folio, private);
        return rc;
    }
    folio_unlock(dst);
    folio_set_owner_migrate_reason(dst, reason as _);
    folio_put(dst);
    list_del(folio_lru(src));
    if !anon_vma.is_null() {
        put_anon_vma(anon_vma);
    }
    folio_unlock(src);
    migrate_folio_done(src, reason);
    rc
}

unsafe fn unmap_and_move_hugetlb_folio(
    get_new_folio: NewFolio,
    mut put_new_folio: FreeFolio,
    private: c_ulong,
    src: *mut folio,
    force: c_int,
    mode: migrate_mode,
    reason: migrate_reason,
    ret: *mut list_head,
) -> c_int {
    if folio_ref_count(src) == 1 {
        folio_putback_hugetlb(src);
        return 0;
    }
    let dst = get_new_folio.unwrap()(src, private);
    if dst.is_null() {
        return E_NOMEM;
    }
    let rc = 'out: {
        if !folio_trylock(src) {
            if force == 0 || mode != MIGRATE_SYNC {
                break 'out E_AGAIN;
            }
            folio_lock(src);
        }
        let rc = 'source_locked: {
            if !hugetlb_folio_subpool(src).is_null() && folio_mapping(src).is_null() {
                break 'source_locked E_BUSY;
            }
            let anon_vma = if folio_test_anon(src) {
                folio_get_anon_vma(src)
            } else {
                null_mut()
            };
            let rc = 'anon_held: {
                if !folio_trylock(dst) {
                    break 'anon_held E_AGAIN;
                }
                let rc = 'destination_locked: {
                    let mut ttu: ttu_flags = 0;
                    let mut mapping = null_mut();
                    let mut was_mapped = false;
                    if folio_mapped(src) {
                        if !folio_test_anon(src) {
                            mapping = hugetlb_folio_mapping_lock_write(src);
                            if mapping.is_null() {
                                break 'destination_locked E_AGAIN;
                            }
                            ttu = TTU_RMAP_LOCKED;
                        }
                        try_to_migrate(src, ttu);
                        was_mapped = true;
                    }
                    let rc = if !folio_mapped(src) {
                        move_to_new_folio(dst, src, mode)
                    } else {
                        E_AGAIN
                    };
                    if was_mapped {
                        remove_migration_ptes(src, if rc == 0 { dst } else { src }, ttu);
                    }
                    if ttu & TTU_RMAP_LOCKED != 0 {
                        i_mmap_unlock_write(mapping);
                    }
                    rc
                };
                folio_unlock(dst);
                rc
            };
            if !anon_vma.is_null() {
                put_anon_vma(anon_vma);
            }
            if rc == 0 {
                move_hugetlb_state(src, dst, reason);
                put_new_folio = None;
            }
            rc
        };
        folio_unlock(src);
        rc
    };
    if rc == 0 {
        folio_putback_hugetlb(src);
    } else if rc != E_AGAIN {
        list_move_tail(folio_lru(src), ret);
    }
    if let Some(put) = put_new_folio {
        put(dst, private);
    } else {
        folio_put(dst);
    }
    rc
}

unsafe fn try_split_folio(
    folio: *mut folio,
    split_folios: *mut list_head,
    mode: migrate_mode,
) -> c_int {
    if mode == MIGRATE_ASYNC {
        if !folio_trylock(folio) {
            return E_AGAIN;
        }
    } else {
        folio_lock(folio);
    }
    let rc = split_folio_to_list(folio, split_folios);
    folio_unlock(folio);
    if rc == 0 {
        list_move_tail(folio_lru(folio), split_folios);
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn alloc_migration_target(src: *mut folio, private: c_ulong) -> *mut folio {
    let mtc = private as *mut migration_target_control;
    let mut gfp_mask = (*mtc).gfp_mask;
    let mut nid = (*mtc).nid;
    let mut order = 0;
    if nid == RUST_MIGRATE_NUMA_NO_NODE as c_int {
        nid = folio_nid(src);
    }
    if folio_test_hugetlb(src) {
        let h = folio_hstate(src);
        gfp_mask = htlb_modify_alloc_mask(h, gfp_mask);
        return alloc_hugetlb_folio_nodemask(
            h,
            nid,
            (*mtc).nmask,
            gfp_mask,
            htlb_allow_alloc_fallback((*mtc).reason),
        );
    }
    if folio_test_large(src) {
        gfp_mask &= !(RUST_MIGRATE___GFP_RECLAIM as gfp_t);
        gfp_mask |= RUST_MIGRATE_GFP_TRANSHUGE as gfp_t;
        order = folio_order(src);
    }
    let zidx = folio_zonenum(src);
    if is_highmem_idx(zidx) || zidx == ZONE_MOVABLE {
        gfp_mask |= RUST_MIGRATE___GFP_HIGHMEM as gfp_t;
    }
    __folio_alloc(gfp_mask, order, nid, (*mtc).nmask)
}
