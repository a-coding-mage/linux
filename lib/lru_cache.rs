// SPDX-License-Identifier: GPL-2.0-or-later
/*
   lru_cache.c

   This file is part of DRBD by Philipp Reisner and Lars Ellenberg.

   Copyright (C) 2003-2008, LINBIT Information Technologies GmbH.
   Copyright (C) 2003-2008, Philipp Reisner <philipp.reisner@linbit.com>.
   Copyright (C) 2003-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
 */

// Linux kernel dependencies are supplied by the surrounding crate.

/* this is developers aid only. It catches concurrent access. */

pub unsafe fn lc_try_lock(lc: *mut lru_cache) -> i32 {
    let mut val: c_ulong;
    loop {
        val = cmpxchg(&mut (*lc).flags, 0, LC_LOCKED);
        if val != LC_PARANOIA { break; }
    }
    (val == 0) as i32
}

pub unsafe fn lc_create(name: *const c_char, cache: *mut kmem_cache,
    max_pending_changes: c_uint, e_count: c_uint, e_size: usize,
    e_off: usize) -> *mut lru_cache {
    let cache_obj_size = kmem_cache_size(cache);
    WARN_ON(cache_obj_size < e_size);
    if cache_obj_size < e_size || e_count > LC_MAX_ACTIVE { return core::ptr::null_mut(); }

    let mut slot = kzalloc_objs::<hlist_head>(e_count);
    if slot.is_null() { return core::ptr::null_mut(); }
    let mut element = kzalloc_objs::<*mut lc_element>(e_count);
    if element.is_null() { kfree(slot); return core::ptr::null_mut(); }
    let lc = kzalloc_obj::<lru_cache>();
    if lc.is_null() { kfree(element); kfree(slot); return core::ptr::null_mut(); }

    INIT_LIST_HEAD(&mut (*lc).in_use);
    INIT_LIST_HEAD(&mut (*lc).lru);
    INIT_LIST_HEAD(&mut (*lc).free);
    INIT_LIST_HEAD(&mut (*lc).to_be_changed);
    (*lc).name = name;
    (*lc).element_size = e_size;
    (*lc).element_off = e_off;
    (*lc).nr_elements = e_count;
    (*lc).max_pending_changes = max_pending_changes;
    (*lc).lc_cache = cache;
    (*lc).lc_element = element;
    (*lc).lc_slot = slot;

    let mut i = 0;
    while i < e_count {
        let p = kmem_cache_alloc(cache, GFP_KERNEL);
        if p.is_null() { break; }
        core::ptr::write_bytes(p, 0, (*lc).element_size);
        let e = (p as *mut u8).add(e_off) as *mut lc_element;
        (*e).lc_index = i;
        (*e).lc_number = LC_FREE;
        (*e).lc_new_number = LC_FREE;
        list_add(&mut (*e).list, &mut (*lc).free);
        *element.add(i as usize) = e;
        i += 1;
    }
    if i == e_count { return lc; }
    while i != 0 {
        i -= 1;
        let p = *element.add(i as usize);
        kmem_cache_free(cache, (p as *mut u8).sub(e_off));
    }
    kfree(lc);
    kfree(element);
    kfree(slot);
    core::ptr::null_mut()
}

unsafe fn lc_free_by_index(lc: *mut lru_cache, i: c_uint) {
    let mut p = *(*lc).lc_element.add(i as usize) as *mut u8;
    WARN_ON(p.is_null());
    if !p.is_null() { p = p.sub((*lc).element_off); kmem_cache_free((*lc).lc_cache, p); }
}

pub unsafe fn lc_destroy(lc: *mut lru_cache) {
    if lc.is_null() { return; }
    for i in 0..(*lc).nr_elements { lc_free_by_index(lc, i); }
    kfree((*lc).lc_element); kfree((*lc).lc_slot); kfree(lc);
}

pub unsafe fn lc_reset(lc: *mut lru_cache) {
    INIT_LIST_HEAD(&mut (*lc).in_use); INIT_LIST_HEAD(&mut (*lc).lru);
    INIT_LIST_HEAD(&mut (*lc).free); INIT_LIST_HEAD(&mut (*lc).to_be_changed);
    (*lc).used = 0; (*lc).hits = 0; (*lc).misses = 0; (*lc).starving = 0;
    (*lc).locked = 0; (*lc).changed = 0; (*lc).pending_changes = 0; (*lc).flags = 0;
    core::ptr::write_bytes((*lc).lc_slot, 0, core::mem::size_of::<hlist_head>() * (*lc).nr_elements as usize);
    for i in 0..(*lc).nr_elements {
        let e = *(*lc).lc_element.add(i as usize);
        core::ptr::write_bytes((e as *mut u8).sub((*lc).element_off), 0, (*lc).element_size);
        (*e).lc_index = i; (*e).lc_number = LC_FREE; (*e).lc_new_number = LC_FREE;
        list_add(&mut (*e).list, &mut (*lc).free);
    }
}

unsafe fn lc_hash_slot(lc: *mut lru_cache, enr: c_uint) -> *mut hlist_head {
    (*lc).lc_slot.add((enr % (*lc).nr_elements) as usize)
}

unsafe fn __lc_find(lc: *mut lru_cache, enr: c_uint, include_changing: bool) -> *mut lc_element {
    BUG_ON(lc.is_null()); BUG_ON((*lc).nr_elements == 0);
    let mut e = hlist_first_entry(lc_hash_slot(lc, enr));
    while !e.is_null() {
        if (*e).lc_new_number == enr {
            if (*e).lc_new_number == (*e).lc_number || include_changing { return e; }
            break;
        }
        e = hlist_next_entry(e);
    }
    core::ptr::null_mut()
}

pub unsafe fn lc_find(lc: *mut lru_cache, enr: c_uint) -> *mut lc_element { __lc_find(lc, enr, false) }
pub unsafe fn lc_is_used(lc: *mut lru_cache, enr: c_uint) -> bool { let e = __lc_find(lc, enr, true); !e.is_null() && (*e).refcnt != 0 }

pub unsafe fn lc_del(lc: *mut lru_cache, e: *mut lc_element) {
    PARANOIA_ENTRY!(lc); PARANOIA_LC_ELEMENT!(lc, e); BUG_ON((*e).refcnt != 0);
    (*e).lc_number = LC_FREE; (*e).lc_new_number = LC_FREE;
    hlist_del_init(&mut (*e).collision); list_move(&mut (*e).list, &mut (*lc).free); RETURN!();
}

unsafe fn lc_prepare_for_change(lc: *mut lru_cache, new_number: c_uint) -> *mut lc_element {
    let n = if !list_empty(&(*lc).free) { (*lc).free.next } else if !list_empty(&(*lc).lru) { (*lc).lru.prev } else { return core::ptr::null_mut() };
    let e = list_entry(n); PARANOIA_LC_ELEMENT!(lc, e); (*e).lc_new_number = new_number;
    if !hlist_unhashed(&(*e).collision) { __hlist_del(&mut (*e).collision); }
    hlist_add_head(&mut (*e).collision, lc_hash_slot(lc, new_number)); list_move(&mut (*e).list, &mut (*lc).to_be_changed); e
}

unsafe fn lc_unused_element_available(lc: *mut lru_cache) -> bool { !list_empty(&(*lc).free) || !list_empty(&(*lc).lru) }

const LC_GET_MAY_CHANGE: c_uint = 1;
const LC_GET_MAY_USE_UNCOMMITTED: c_uint = 2;

unsafe fn __lc_get(lc: *mut lru_cache, enr: c_uint, flags: c_uint) -> *mut lc_element {
    PARANOIA_ENTRY!(lc);
    if test_bit(__LC_STARVING, &(*lc).flags) { (*lc).starving += 1; RETURN!(core::ptr::null_mut()); }
    let e = __lc_find(lc, enr, true);
    if !e.is_null() {
        if (*e).lc_new_number != (*e).lc_number {
            if flags & LC_GET_MAY_USE_UNCOMMITTED == 0 { RETURN!(core::ptr::null_mut()); }
            (*e).refcnt += 1; (*lc).hits += 1; RETURN!(e);
        }
        (*lc).hits += 1; if (*e).refcnt == 0 { (*lc).used += 1; } (*e).refcnt += 1;
        list_move(&mut (*e).list, &mut (*lc).in_use); RETURN!(e);
    }
    (*lc).misses += 1; if flags & LC_GET_MAY_CHANGE == 0 { RETURN!(core::ptr::null_mut()); }
    test_and_set_bit(__LC_DIRTY, &mut (*lc).flags);
    if test_bit(__LC_LOCKED, &(*lc).flags) { (*lc).locked += 1; RETURN!(core::ptr::null_mut()); }
    if !lc_unused_element_available(lc) { set_bit(__LC_STARVING, &mut (*lc).flags); RETURN!(core::ptr::null_mut()); }
    if (*lc).pending_changes >= (*lc).max_pending_changes { RETURN!(core::ptr::null_mut()); }
    let e = lc_prepare_for_change(lc, enr); BUG_ON(e.is_null()); clear_bit(__LC_STARVING, &mut (*lc).flags);
    (*e).refcnt += 1; BUG_ON((*e).refcnt != 1); (*lc).used += 1; (*lc).pending_changes += 1; RETURN!(e);
}

pub unsafe fn lc_get(lc: *mut lru_cache, enr: c_uint) -> *mut lc_element { __lc_get(lc, enr, LC_GET_MAY_CHANGE) }
pub unsafe fn lc_get_cumulative(lc: *mut lru_cache, enr: c_uint) -> *mut lc_element { __lc_get(lc, enr, LC_GET_MAY_CHANGE | LC_GET_MAY_USE_UNCOMMITTED) }
pub unsafe fn lc_try_get(lc: *mut lru_cache, enr: c_uint) -> *mut lc_element { __lc_get(lc, enr, 0) }

pub unsafe fn lc_committed(lc: *mut lru_cache) {
    PARANOIA_ENTRY!(lc); list_for_each_entry_safe!((*lc).to_be_changed, e, tmp, {
        (*lc).changed += 1; (*e).lc_number = (*e).lc_new_number; list_move(&mut (*e).list, &mut (*lc).in_use);
    }); (*lc).pending_changes = 0; RETURN!();
}

pub unsafe fn lc_put(lc: *mut lru_cache, e: *mut lc_element) -> c_uint {
    PARANOIA_ENTRY!(lc); PARANOIA_LC_ELEMENT!(lc, e); BUG_ON((*e).refcnt == 0); BUG_ON((*e).lc_number != (*e).lc_new_number);
    (*e).refcnt -= 1; if (*e).refcnt == 0 { list_move(&mut (*e).list, &mut (*lc).lru); (*lc).used -= 1; clear_bit_unlock(__LC_STARVING, &mut (*lc).flags); }
    let r = (*e).refcnt; RETURN!(r)
}

pub unsafe fn lc_element_by_index(lc: *mut lru_cache, i: c_uint) -> *mut lc_element {
    BUG_ON(i >= (*lc).nr_elements); let e = *(*lc).lc_element.add(i as usize); BUG_ON(e.is_null()); BUG_ON((*e).lc_index != i); e
}

// The seq_file reporting functions retain their C formatting and callback interfaces.
pub unsafe fn lc_seq_printf_stats(seq: *mut seq_file, lc: *mut lru_cache) {
    seq_printf(seq, "\t%s: used:%u/%u hits:%lu misses:%lu starving:%lu locked:%lu changed:%lu\n", (*lc).name, (*lc).used, (*lc).nr_elements, (*lc).hits, (*lc).misses, (*lc).starving, (*lc).locked, (*lc).changed);
}

pub unsafe fn lc_seq_dump_details(seq: *mut seq_file, lc: *mut lru_cache, utext: *mut c_char,
    detail: Option<unsafe extern "C" fn(*mut seq_file, *mut lc_element)>) {
    seq_printf(seq, "\tnn: lc_number (new nr) refcnt %s\n ", utext);
    for i in 0..(*lc).nr_elements {
        let e = lc_element_by_index(lc, i);
        if (*e).lc_number != (*e).lc_new_number { seq_printf(seq, "\t%5d: %6d %8d %6d ", i, (*e).lc_number, (*e).lc_new_number, (*e).refcnt); }
        else { seq_printf(seq, "\t%5d: %6d %-8s %6d ", i, (*e).lc_number, "-\"-", (*e).refcnt); }
        if let Some(f) = detail { f(seq, e); } seq_putc(seq, b'\n' as c_int);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
