// SPDX-License-Identifier: GPL-2.0
// Both HPAGE_PMD_NR = (1 << HPAGE_PMD_ORDER) and fallback 512 are native int.
fn migration_batch_full(nr_pages: c_int) -> bool {
    nr_pages >= RUST_MIGRATE_NR_MAX_BATCHED_MIGRATION as c_int
}

const NR_MAX_MIGRATE_PAGES_RETRY: c_int = 10;
const NR_MAX_MIGRATE_ASYNC_RETRY: c_int = 3;
const NR_MAX_MIGRATE_SYNC_RETRY: c_int = NR_MAX_MIGRATE_PAGES_RETRY - NR_MAX_MIGRATE_ASYNC_RETRY;

// Private C source-local aggregate is now Rust-owned, never passed over FFI.
#[derive(Default)]
struct MigratePagesStats {
    nr_succeeded: c_int,
    nr_failed_pages: c_int,
    nr_thp_succeeded: c_int,
    nr_thp_failed: c_int,
    nr_thp_split: c_int,
    nr_split: c_int,
}
macro_rules! add_count {
    ($place:expr, $value:expr) => {
        $place = $place.wrapping_add($value as c_int)
    };
}

unsafe fn migrate_hugetlbs(
    from: *mut list_head,
    get_new_folio: NewFolio,
    put_new_folio: FreeFolio,
    private: c_ulong,
    mode: migrate_mode,
    reason: migrate_reason,
    stats: &mut MigratePagesStats,
    ret_folios: *mut list_head,
) -> c_int {
    let mut retry: c_int = 1;
    let mut nr_failed: c_int = 0;
    let mut nr_retry_pages: c_int = 0;
    let mut pass: c_int = 0;
    while pass < NR_MAX_MIGRATE_PAGES_RETRY && retry != 0 {
        retry = 0;
        nr_retry_pages = 0;
        let mut link = (*from).next;
        while link != from {
            let folio = folio_from_lru(link);
            link = (*link).next;
            if !folio_test_hugetlb(folio) {
                continue;
            }
            let nr_pages = folio_nr_pages(folio) as c_int;
            cond_resched();
            if !hugepage_migration_supported(folio_hstate(folio)) {
                add_count!(nr_failed, 1);
                add_count!(stats.nr_failed_pages, nr_pages);
                list_move_tail(folio_lru(folio), ret_folios);
                continue;
            }
            let rc = unmap_and_move_hugetlb_folio(
                get_new_folio,
                put_new_folio,
                private,
                folio,
                (pass > 2) as c_int,
                mode,
                reason,
                ret_folios,
            );
            match rc {
                E_NOMEM => {
                    add_count!(stats.nr_failed_pages, nr_pages.wrapping_add(nr_retry_pages));
                    return E_NOMEM;
                }
                E_AGAIN => {
                    add_count!(retry, 1);
                    add_count!(nr_retry_pages, nr_pages);
                }
                0 => add_count!(stats.nr_succeeded, nr_pages),
                _ => {
                    add_count!(nr_failed, 1);
                    add_count!(stats.nr_failed_pages, nr_pages);
                }
            }
        }
        pass = pass.wrapping_add(1);
    }
    add_count!(nr_failed, retry);
    add_count!(stats.nr_failed_pages, nr_retry_pages);
    nr_failed
}

unsafe fn migrate_folios_move(
    src_folios: *mut list_head,
    dst_folios: *mut list_head,
    put_new_folio: FreeFolio,
    private: c_ulong,
    mode: migrate_mode,
    reason: migrate_reason,
    ret_folios: *mut list_head,
    stats: &mut MigratePagesStats,
    retry: &mut c_int,
    thp_retry: &mut c_int,
    nr_failed: &mut c_int,
    nr_retry_pages: &mut c_int,
) {
    // Walk list links, avoiding C's synthetic container_of(head) sentinel.
    let mut source_link = (*src_folios).next;
    let mut dst_link = (*dst_folios).next;
    while source_link != src_folios {
        let folio = folio_from_lru(source_link);
        let dst = folio_from_lru(dst_link);
        source_link = (*source_link).next;
        dst_link = (*dst_link).next;
        let is_thp = folio_test_large(folio) && folio_test_pmd_mappable(folio);
        let nr_pages = folio_nr_pages(folio) as c_int;
        cond_resched();
        let rc = migrate_folio_move(put_new_folio, private, folio, dst, mode, reason, ret_folios);
        match rc {
            E_AGAIN => {
                add_count!(*retry, 1);
                add_count!(*thp_retry, is_thp);
                add_count!(*nr_retry_pages, nr_pages);
            }
            0 => {
                add_count!(stats.nr_succeeded, nr_pages);
                add_count!(stats.nr_thp_succeeded, is_thp);
            }
            _ => {
                add_count!(*nr_failed, 1);
                add_count!(stats.nr_thp_failed, is_thp);
                add_count!(stats.nr_failed_pages, nr_pages);
            }
        }
    }
}

unsafe fn migrate_folios_undo(
    src_folios: *mut list_head,
    dst_folios: *mut list_head,
    put_new_folio: FreeFolio,
    private: c_ulong,
    ret_folios: *mut list_head,
) {
    let mut source_link = (*src_folios).next;
    let mut dst_link = (*dst_folios).next;
    while source_link != src_folios {
        let folio = folio_from_lru(source_link);
        let dst = folio_from_lru(dst_link);
        source_link = (*source_link).next;
        dst_link = (*dst_link).next;
        let mut old_folio_state = 0;
        let mut anon_vma = null_mut();
        __migrate_folio_extract(dst, &mut old_folio_state, &mut anon_vma);
        migrate_folio_undo_src(
            folio,
            old_folio_state & FOLIO_WAS_MAPPED,
            anon_vma,
            true,
            ret_folios,
        );
        list_del(folio_lru(dst));
        migrate_folio_undo_dst(dst, true, put_new_folio, private);
    }
}

unsafe fn migrate_pages_batch(
    from: *mut list_head,
    get_new_folio: NewFolio,
    put_new_folio: FreeFolio,
    private: c_ulong,
    mode: migrate_mode,
    reason: migrate_reason,
    ret_folios: *mut list_head,
    split_folios: *mut list_head,
    stats: &mut MigratePagesStats,
    nr_pass: c_int,
) -> c_int {
    let mut retry: c_int = 1;
    let mut thp_retry: c_int = 1;
    let mut nr_failed: c_int = 0;
    let mut nr_retry_pages: c_int = 0;
    let mut rc_saved: c_int = 0;
    let mut unmap_folios: list_head = zeroed();
    let mut dst_folios: list_head = zeroed();
    init_list_head(&mut unmap_folios);
    init_list_head(&mut dst_folios);
    let nosplit = reason == MR_NUMA_MISPLACED;
    vm_diag!(warn_sync_batch(
        mode != MIGRATE_ASYNC && !list_empty(from) && !list_is_singular(from)
    ));
    let normally_exhausted = 'unmap_phase: {
        let mut pass: c_int = 0;
        while pass < nr_pass && retry != 0 {
            retry = 0;
            thp_retry = 0;
            nr_retry_pages = 0;
            let mut link = (*from).next;
            while link != from {
                let folio = folio_from_lru(link);
                link = (*link).next;
                let is_large = folio_test_large(folio);
                let is_thp = folio_test_pmd_mappable(folio);
                let nr_pages = folio_nr_pages(folio) as c_int;
                cond_resched_tasks_rcu_qs();
                if nr_pages > 2
                    && !list_empty(folio_deferred_list(folio))
                    && folio_test_partially_mapped(folio)
                {
                    if try_split_folio(folio, split_folios, mode) == 0 {
                        add_count!(nr_failed, 1);
                        add_count!(stats.nr_thp_failed, is_thp);
                        add_count!(stats.nr_thp_split, is_thp);
                        add_count!(stats.nr_split, 1);
                        continue;
                    }
                }
                if !thp_migration_supported() && is_thp {
                    add_count!(nr_failed, 1);
                    add_count!(stats.nr_thp_failed, 1);
                    if try_split_folio(folio, split_folios, mode) == 0 {
                        add_count!(stats.nr_thp_split, 1);
                        add_count!(stats.nr_split, 1);
                        continue;
                    }
                    add_count!(stats.nr_failed_pages, nr_pages);
                    list_move_tail(folio_lru(folio), ret_folios);
                    continue;
                }
                if !page_has_movable_ops(folio_page(folio, 0)) && folio_ref_count(folio) == 1 {
                    folio_clear_active(folio);
                    folio_clear_unevictable(folio);
                    list_del(folio_lru(folio));
                    migrate_folio_done(folio, reason);
                    add_count!(stats.nr_succeeded, nr_pages);
                    add_count!(stats.nr_thp_succeeded, is_thp);
                    continue;
                }
                let mut dst = null_mut();
                let rc = migrate_folio_unmap(
                    get_new_folio,
                    put_new_folio,
                    private,
                    folio,
                    &mut dst,
                    mode,
                    ret_folios,
                );
                match rc {
                    E_NOMEM => {
                        add_count!(nr_failed, 1);
                        add_count!(stats.nr_thp_failed, is_thp);
                        if is_large && !nosplit {
                            let ret = try_split_folio(folio, split_folios, mode);
                            if ret == 0 {
                                add_count!(stats.nr_thp_split, is_thp);
                                add_count!(stats.nr_split, 1);
                                continue;
                            } else if reason == MR_LONGTERM_PIN && ret == E_AGAIN {
                                add_count!(retry, 1);
                                add_count!(thp_retry, is_thp);
                                add_count!(nr_retry_pages, nr_pages);
                                nr_failed = nr_failed.wrapping_sub(1);
                                stats.nr_thp_failed =
                                    stats.nr_thp_failed.wrapping_sub(is_thp as c_int);
                                continue;
                            }
                        }
                        add_count!(stats.nr_failed_pages, nr_pages.wrapping_add(nr_retry_pages));
                        add_count!(stats.nr_thp_failed, thp_retry);
                        rc_saved = rc;
                        break 'unmap_phase false;
                    }
                    E_AGAIN => {
                        add_count!(retry, 1);
                        add_count!(thp_retry, is_thp);
                        add_count!(nr_retry_pages, nr_pages);
                    }
                    0 => {
                        list_move_tail(folio_lru(folio), &mut unmap_folios);
                        list_add_tail(folio_lru(dst), &mut dst_folios);
                    }
                    _ => {
                        add_count!(nr_failed, 1);
                        add_count!(stats.nr_thp_failed, is_thp);
                        add_count!(stats.nr_failed_pages, nr_pages);
                    }
                }
            }
            pass = pass.wrapping_add(1);
        }
        true
    };
    if normally_exhausted {
        add_count!(nr_failed, retry);
        add_count!(stats.nr_thp_failed, thp_retry);
        add_count!(stats.nr_failed_pages, nr_retry_pages);
    }
    // The original ENOMEM/out shortcut skips the flush when nothing was unmapped.
    if normally_exhausted || !list_empty(&unmap_folios) {
        try_to_unmap_flush();
        retry = 1;
        let mut pass: c_int = 0;
        while pass < nr_pass && retry != 0 {
            retry = 0;
            thp_retry = 0;
            nr_retry_pages = 0;
            migrate_folios_move(
                &mut unmap_folios,
                &mut dst_folios,
                put_new_folio,
                private,
                mode,
                reason,
                ret_folios,
                stats,
                &mut retry,
                &mut thp_retry,
                &mut nr_failed,
                &mut nr_retry_pages,
            );
            pass = pass.wrapping_add(1);
        }
        add_count!(nr_failed, retry);
        add_count!(stats.nr_thp_failed, thp_retry);
        add_count!(stats.nr_failed_pages, nr_retry_pages);
    }
    let rc = if rc_saved != 0 { rc_saved } else { nr_failed };
    migrate_folios_undo(
        &mut unmap_folios,
        &mut dst_folios,
        put_new_folio,
        private,
        ret_folios,
    );
    rc
}

unsafe fn migrate_pages_sync(
    from: *mut list_head,
    get_new_folio: NewFolio,
    put_new_folio: FreeFolio,
    private: c_ulong,
    mode: migrate_mode,
    reason: migrate_reason,
    ret_folios: *mut list_head,
    split_folios: *mut list_head,
    stats: &mut MigratePagesStats,
) -> c_int {
    let mut folios: list_head = zeroed();
    init_list_head(&mut folios);
    let mut astats = MigratePagesStats::default();
    let rc = migrate_pages_batch(
        from,
        get_new_folio,
        put_new_folio,
        private,
        MIGRATE_ASYNC,
        reason,
        &mut folios,
        split_folios,
        &mut astats,
        NR_MAX_MIGRATE_ASYNC_RETRY,
    );
    add_count!(stats.nr_succeeded, astats.nr_succeeded);
    add_count!(stats.nr_thp_succeeded, astats.nr_thp_succeeded);
    add_count!(stats.nr_thp_split, astats.nr_thp_split);
    add_count!(stats.nr_split, astats.nr_split);
    if rc < 0 {
        add_count!(stats.nr_failed_pages, astats.nr_failed_pages);
        add_count!(stats.nr_thp_failed, astats.nr_thp_failed);
        list_splice_tail(&mut folios, ret_folios);
        return rc;
    }
    add_count!(stats.nr_thp_failed, astats.nr_thp_split);
    let mut nr_failed = astats.nr_split;
    list_splice_tail_init(&mut folios, from);
    while !list_empty(from) {
        list_move((*from).next, &mut folios);
        let rc = migrate_pages_batch(
            &mut folios,
            get_new_folio,
            put_new_folio,
            private,
            mode,
            reason,
            ret_folios,
            split_folios,
            stats,
            NR_MAX_MIGRATE_SYNC_RETRY,
        );
        list_splice_tail_init(&mut folios, ret_folios);
        if rc < 0 {
            return rc;
        }
        add_count!(nr_failed, rc);
    }
    nr_failed
}

#[no_mangle]
pub unsafe extern "C" fn migrate_pages(
    from: *mut list_head,
    get_new_folio: NewFolio,
    put_new_folio: FreeFolio,
    private: c_ulong,
    mode: migrate_mode,
    reason: migrate_reason,
    ret_succeeded: *mut c_uint,
) -> c_int {
    let mut folios: list_head = zeroed();
    let mut ret_folios: list_head = zeroed();
    let mut split_folios: list_head = zeroed();
    init_list_head(&mut folios);
    init_list_head(&mut ret_folios);
    init_list_head(&mut split_folios);
    let mut stats = MigratePagesStats::default();
    trace_mm_migrate_pages_start(mode, reason);
    let mut rc_gather = migrate_hugetlbs(
        from,
        get_new_folio,
        put_new_folio,
        private,
        mode,
        reason,
        &mut stats,
        &mut ret_folios,
    );
    if rc_gather >= 0 {
        loop {
            let mut nr_pages: c_int = 0;
            let mut link = (*from).next;
            while link != from {
                let folio = folio_from_lru(link);
                link = (*link).next;
                if folio_test_hugetlb(folio) {
                    list_move_tail(folio_lru(folio), &mut ret_folios);
                    continue;
                }
                // C compound assignment converts the ulong sum back to int.
                nr_pages = (nr_pages as c_ulong).wrapping_add(folio_nr_pages(folio)) as c_int;
                if migration_batch_full(nr_pages) {
                    break;
                }
            }
            if migration_batch_full(nr_pages) {
                list_cut_before(&mut folios, from, link);
            } else {
                list_splice_init(from, &mut folios);
            }
            let rc = if mode == MIGRATE_ASYNC {
                migrate_pages_batch(
                    &mut folios,
                    get_new_folio,
                    put_new_folio,
                    private,
                    mode,
                    reason,
                    &mut ret_folios,
                    &mut split_folios,
                    &mut stats,
                    NR_MAX_MIGRATE_PAGES_RETRY,
                )
            } else {
                migrate_pages_sync(
                    &mut folios,
                    get_new_folio,
                    put_new_folio,
                    private,
                    mode,
                    reason,
                    &mut ret_folios,
                    &mut split_folios,
                    &mut stats,
                )
            };
            list_splice_tail_init(&mut folios, &mut ret_folios);
            if rc < 0 {
                rc_gather = rc;
                list_splice_tail(&mut split_folios, &mut ret_folios);
                break;
            }
            if !list_empty(&split_folios) {
                migrate_pages_batch(
                    &mut split_folios,
                    get_new_folio,
                    put_new_folio,
                    private,
                    MIGRATE_ASYNC,
                    reason,
                    &mut ret_folios,
                    null_mut(),
                    &mut stats,
                    1,
                );
                list_splice_tail_init(&mut split_folios, &mut ret_folios);
            }
            add_count!(rc_gather, rc);
            if list_empty(from) {
                break;
            }
        }
    }
    list_splice(&mut ret_folios, from);
    if list_empty(from) {
        rc_gather = 0;
    }
    count_vm_events(PGMIGRATE_SUCCESS, stats.nr_succeeded as _);
    count_vm_events(PGMIGRATE_FAIL, stats.nr_failed_pages as _);
    count_vm_events(THP_MIGRATION_SUCCESS, stats.nr_thp_succeeded as _);
    count_vm_events(THP_MIGRATION_FAIL, stats.nr_thp_failed as _);
    count_vm_events(THP_MIGRATION_SPLIT, stats.nr_thp_split as _);
    trace_mm_migrate_pages(
        stats.nr_succeeded as _,
        stats.nr_failed_pages as _,
        stats.nr_thp_succeeded as _,
        stats.nr_thp_failed as _,
        stats.nr_thp_split as _,
        stats.nr_split as _,
        mode,
        reason,
    );
    if !ret_succeeded.is_null() {
        *ret_succeeded = stats.nr_succeeded as c_uint;
    }
    rc_gather
}
