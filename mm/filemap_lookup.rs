// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:1811-2440.
#[no_mangle]
pub unsafe extern "C" fn page_cache_next_miss(
    mapping: *mut address_space,
    index: Pgoff,
    mut max_scan: c_ulong,
) -> Pgoff {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), index);
    while max_scan != 0 {
        max_scan = max_scan.wrapping_sub(1);
        let entry = xas_next(&mut xas);
        if entry.is_null() || xa_is_value(entry) {
            return xas.xa_index;
        }
        if xas.xa_index == 0 {
            return 0;
        }
    }
    xas.xa_index.wrapping_add(1)
}
#[no_mangle]
pub unsafe extern "C" fn page_cache_prev_miss(
    mapping: *mut address_space,
    index: Pgoff,
    mut max_scan: c_ulong,
) -> Pgoff {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), index);
    while max_scan != 0 {
        max_scan = max_scan.wrapping_sub(1);
        let entry = xas_prev(&mut xas);
        if entry.is_null() || xa_is_value(entry) {
            return xas.xa_index;
        }
        if xas.xa_index == c_ulong::MAX {
            return c_ulong::MAX;
        }
    }
    xas.xa_index.wrapping_sub(1)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_get_entry(
    mapping: *mut address_space,
    index: Pgoff,
) -> *mut c_void {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), index);
    rcu_read_lock();
    let entry = loop {
        xas_reset(&mut xas);
        let folio = xas_load(&mut xas) as *mut folio;
        if xas_retry(&mut xas, folio.cast()) {
            continue;
        }
        if folio.is_null() || xa_is_value(folio.cast()) {
            break folio;
        }
        if !folio_try_get(folio) {
            continue;
        }
        if folio.cast() != xas_reload(&mut xas) {
            folio_put(folio);
            continue;
        }
        break folio;
    };
    rcu_read_unlock();
    entry.cast()
}
#[no_mangle]
pub unsafe extern "C" fn __filemap_get_folio_mpol(
    mapping: *mut address_space,
    mut index: Pgoff,
    mut fgp_flags: fgf_t,
    mut gfp: gfp_t,
    policy: *mut mempolicy,
) -> *mut folio {
    loop {
        let mut folio = filemap_get_entry(mapping, index) as *mut folio;
        if xa_is_value(folio.cast()) {
            folio = null_mut();
        }
        if !folio.is_null() {
            if fgp_flags & RUST_FILEMAP_FGP_LOCK != 0 {
                if fgp_flags & RUST_FILEMAP_FGP_NOWAIT != 0 {
                    if !folio_trylock(folio) {
                        folio_put(folio);
                        return ERR_PTR(-(EAGAIN as c_long)).cast();
                    }
                } else {
                    folio_lock(folio);
                }
                if folio_mapping_field(folio) != mapping {
                    folio_unlock(folio);
                    folio_put(folio);
                    continue;
                }
                vm_bug_folio!((!folio_contains(folio, index)) as c_int, folio);
            }
            if fgp_flags & RUST_FILEMAP_FGP_ACCESSED != 0 {
                folio_mark_accessed(folio);
            } else if fgp_flags & RUST_FILEMAP_FGP_WRITE != 0 && folio_test_idle(folio) {
                folio_clear_idle(folio);
            }
            if fgp_flags & RUST_FILEMAP_FGP_STABLE != 0 {
                folio_wait_stable(folio);
            }
        }
        if folio.is_null() && fgp_flags & RUST_FILEMAP_FGP_CREAT != 0 {
            let min_order = mapping_min_folio_order(mapping);
            let mut order = core::cmp::max(min_order, FGF_GET_ORDER(fgp_flags));
            index = mapping_align_index(mapping, index);
            if fgp_flags & RUST_FILEMAP_FGP_WRITE != 0 && mapping_can_writeback(mapping) {
                gfp |= RUST_FILEMAP___GFP_WRITE;
            }
            if fgp_flags & RUST_FILEMAP_FGP_NOFS != 0 {
                gfp &= !RUST_FILEMAP___GFP_FS;
            }
            if fgp_flags & RUST_FILEMAP_FGP_NOWAIT != 0 {
                gfp &= !RUST_FILEMAP_GFP_KERNEL;
                gfp |= RUST_FILEMAP_GFP_NOWAIT;
            }
            if rust_filemap_warn_get_unlocked(
                fgp_flags & (RUST_FILEMAP_FGP_LOCK | RUST_FILEMAP_FGP_FOR_MMAP) == 0,
            ) {
                fgp_flags |= RUST_FILEMAP_FGP_LOCK;
            }
            order = core::cmp::min(order, mapping_max_folio_order(mapping));
            if index & (1 as c_ulong).wrapping_shl(order).wrapping_sub(1) != 0 {
                order = __ffs(index) as c_uint;
            }
            let mut err;
            loop {
                let mut alloc_gfp = gfp;
                err = -(ENOMEM as c_int);
                if order > min_order {
                    alloc_gfp |= RUST_FILEMAP___GFP_NORETRY | RUST_FILEMAP___GFP_NOWARN;
                }
                folio = filemap_alloc_folio_c2018(alloc_gfp, order, policy);
                if !folio.is_null() {
                    if fgp_flags & RUST_FILEMAP_FGP_ACCESSED != 0 {
                        __folio_set_referenced(folio);
                    }
                    if fgp_flags & RUST_FILEMAP_FGP_DONTCACHE != 0 {
                        __folio_set_dropbehind(folio);
                    }
                    err = filemap_add_folio(mapping, folio, index, gfp);
                    if err == 0 {
                        break;
                    }
                    folio_put(folio);
                    folio = null_mut();
                }
                let previous = order;
                order = order.wrapping_sub(1);
                if previous <= min_order {
                    break;
                }
            }
            if err == -(EEXIST as c_int) {
                continue;
            }
            if err != 0 {
                if fgp_flags & RUST_FILEMAP_FGP_NOWAIT != 0 && err == -(ENOMEM as c_int) {
                    err = -(EAGAIN as c_int);
                }
                return ERR_PTR(err as c_long).cast();
            }
            if !folio.is_null() && fgp_flags & RUST_FILEMAP_FGP_FOR_MMAP != 0 {
                folio_unlock(folio);
            }
        }
        if folio.is_null() {
            return ERR_PTR(-(ENOENT as c_long)).cast();
        }
        if fgp_flags & RUST_FILEMAP_FGP_DONTCACHE == 0 && folio_test_clear_dropbehind(folio) {
            if folio_test_dirty(folio) && mapping_can_writeback(mapping) {
                let inode = mapping_host(mapping);
                let mut cookie: wb_lock_cookie = zeroed();
                let nr = folio_nr_pages(folio) as c_long;
                let wb = unlocked_inode_to_wb_begin(inode, &mut cookie);
                wb_stat_mod(wb, WB_DONTCACHE_DIRTY, nr.wrapping_neg());
                unlocked_inode_to_wb_end(inode, &mut cookie);
            }
        }
        return folio;
    }
}
unsafe fn find_get_entry(xas: *mut xa_state, max: Pgoff, mark: xa_mark_t) -> *mut folio {
    loop {
        let folio = (if mark == RUST_FILEMAP_XA_PRESENT {
            xas_find(xas, max)
        } else {
            xas_find_marked(xas, max, mark)
        }) as *mut folio;
        if xas_retry(xas, folio.cast()) {
            continue;
        }
        if folio.is_null() || xa_is_value(folio.cast()) {
            return folio;
        }
        if folio_try_get(folio) {
            if folio.cast() == xas_reload(xas) {
                return folio;
            }
            folio_put(folio);
        }
        xas_reset(xas);
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_get_entries(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    fbatch: *mut folio_batch,
    indices: *mut Pgoff,
) -> c_uint {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), *start);
    rcu_read_lock();
    loop {
        let folio = find_get_entry(&mut xas, end, RUST_FILEMAP_XA_PRESENT);
        if folio.is_null() {
            break;
        }
        *indices.add((*fbatch).nr as usize) = xas.xa_index;
        if folio_batch_add(fbatch, folio) == 0 {
            break;
        }
    }
    if folio_batch_count(fbatch) != 0 {
        let idx = folio_batch_count(fbatch) as usize - 1;
        let folio = (*fbatch).folios[idx];
        let nr = if !xa_is_value(folio.cast()) {
            folio_nr_pages(folio)
        } else {
            (1 as c_int)
                .wrapping_shl(xa_get_order(mapping_i_pages(mapping), *indices.add(idx)) as c_uint)
                as c_ulong
        };
        *start = (*indices.add(idx)).wrapping_add(nr) & !nr.wrapping_sub(1);
    }
    rcu_read_unlock();
    folio_batch_count(fbatch)
}
#[no_mangle]
pub unsafe extern "C" fn find_lock_entries(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    fbatch: *mut folio_batch,
    indices: *mut Pgoff,
) -> c_uint {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), *start);
    rcu_read_lock();
    loop {
        let folio = find_get_entry(&mut xas, end, RUST_FILEMAP_XA_PRESENT);
        if folio.is_null() {
            break;
        }
        let (base, nr);
        if !xa_is_value(folio.cast()) {
            nr = folio_nr_pages(folio);
            base = folio_index(folio);
            if base < *start || base.wrapping_add(nr).wrapping_sub(1) > end || !folio_trylock(folio)
            {
                folio_put(folio);
                continue;
            }
            if folio_mapping_field(folio) != mapping || folio_test_writeback(folio) {
                folio_unlock(folio);
                folio_put(folio);
                continue;
            }
            vm_bug_folio!((!folio_contains(folio, xas.xa_index)) as c_int, folio);
        } else {
            nr = (1 as c_int).wrapping_shl(xas_get_order(&mut xas) as c_uint) as c_ulong;
            base = xas.xa_index & !nr.wrapping_sub(1);
            if base < *start {
                continue;
            }
            if base.wrapping_add(nr).wrapping_sub(1) > end {
                break;
            }
        }
        *start = base.wrapping_add(nr);
        *indices.add((*fbatch).nr as usize) = xas.xa_index;
        if folio_batch_add(fbatch, folio) == 0 {
            break;
        }
    }
    rcu_read_unlock();
    folio_batch_count(fbatch)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    fbatch: *mut folio_batch,
) -> c_uint {
    filemap_get_folios_tag(mapping, start, end, RUST_FILEMAP_XA_PRESENT, fbatch)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_contig(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    fbatch: *mut folio_batch,
) -> c_uint {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), *start);
    if *start > end {
        return 0;
    }
    rcu_read_lock();
    let mut folio = xas_load(&mut xas) as *mut folio;
    while !folio.is_null() {
        'entry: {
            if xas_retry(&mut xas, folio.cast()) {
                break 'entry;
            }
            if xa_is_value(folio.cast()) || xa_is_sibling(folio.cast()) {
                break;
            }
            if folio_try_get(folio) {
                if folio.cast() == xas_reload(&mut xas) {
                    if folio_batch_add(fbatch, folio) == 0 {
                        break;
                    }
                    xas_advance(&mut xas, folio_next_index(folio).wrapping_sub(1));
                    if xas.xa_index >= end {
                        break;
                    }
                    break 'entry;
                }
                folio_put(folio);
            }
            xas_reset(&mut xas);
        }
        folio = xas_next(&mut xas) as *mut folio;
    }
    rcu_read_unlock();
    let nr = folio_batch_count(fbatch);
    if nr != 0 {
        *start = folio_next_index((*fbatch).folios[nr as usize - 1]);
    }
    nr
}
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_tag(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    tag: xa_mark_t,
    fbatch: *mut folio_batch,
) -> c_uint {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), *start);
    rcu_read_lock();
    loop {
        let folio = find_get_entry(&mut xas, end, tag);
        if folio.is_null() {
            *start = end.saturating_add(1);
            break;
        }
        if xa_is_value(folio.cast()) {
            continue;
        }
        if folio_batch_add(fbatch, folio) == 0 {
            *start = folio_next_index(folio);
            break;
        }
    }
    rcu_read_unlock();
    folio_batch_count(fbatch)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_dirty(
    mapping: *mut address_space,
    start: *mut Pgoff,
    end: Pgoff,
    fbatch: *mut folio_batch,
) -> c_uint {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), *start);
    rcu_read_lock();
    loop {
        let folio = find_get_entry(&mut xas, end, RUST_FILEMAP_XA_PRESENT);
        if folio.is_null() {
            *start = end.saturating_add(1);
            break;
        }
        if xa_is_value(folio.cast()) {
            continue;
        }
        if folio_trylock(folio) {
            let clean = !folio_test_dirty(folio) && !folio_test_writeback(folio);
            folio_unlock(folio);
            if clean {
                folio_put(folio);
                continue;
            }
        }
        if folio_batch_add(fbatch, folio) == 0 {
            *start = folio_next_index(folio);
            break;
        }
    }
    rcu_read_unlock();
    folio_batch_count(fbatch)
}
