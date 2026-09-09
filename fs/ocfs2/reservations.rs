// SPDX-License-Identifier: GPL-2.0-only
/*
 * reservations.c
 *
 * Allocation reservations implementation
 *
 * Some code borrowed from fs/ext3/balloc.c and is:
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 * The rest is copyright (C) 2010 Novell.  All rights reserved.
 */

// Linux and OCFS2 dependencies are supplied by other translation units.
// CONFIG_OCFS2_DEBUG_FS enables OCFS2_CHECK_RESERVATIONS.

static mut resv_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK_INIT;

pub unsafe fn ocfs2_dir_resv_allowed(osb: *mut ocfs2_super) -> c_int {
    ((*osb).osb_resv_level != 0 && (*osb).osb_dir_resv_level != 0) as c_int
}

unsafe fn ocfs2_resv_window_bits(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation) -> c_uint {
    let osb = (*resmap).m_osb;
    if (*resv).r_flags & OCFS2_RESV_FLAG_DIR == 0 {
        4u32 << (*osb).osb_resv_level
    } else {
        4u32 << (*osb).osb_dir_resv_level
    }
}

unsafe fn ocfs2_resv_end(resv: *mut ocfs2_alloc_reservation) -> c_uint {
    if (*resv).r_len != 0 { (*resv).r_start + (*resv).r_len - 1 } else { (*resv).r_start }
}

unsafe fn ocfs2_resv_empty(resv: *mut ocfs2_alloc_reservation) -> c_int { ((*resv).r_len == 0) as c_int }

unsafe fn ocfs2_resmap_disabled(resmap: *mut ocfs2_reservation_map) -> c_int {
    ((*(*resmap).m_osb).osb_resv_level == 0) as c_int
}

unsafe fn ocfs2_dump_resv(resmap: *mut ocfs2_reservation_map) {
    let osb = (*resmap).m_osb;
    let mut node = rb_first(&(*resmap).m_reservations);
    let mut i = 0;
    mlog(ML_NOTICE, "Dumping resmap for device %s. Bitmap length: %u\n", (*osb).dev_str, (*resmap).m_bitmap_len);
    while !node.is_null() {
        let resv = rb_entry(node, struct_ocfs2_alloc_reservation, r_node);
        mlog(ML_NOTICE, "start: %u\tend: %u\tlen: %u\tlast_start: %u\tlast_len: %u\n", (*resv).r_start, ocfs2_resv_end(resv), (*resv).r_len, (*resv).r_last_start, (*resv).r_last_len);
        node = rb_next(node); i += 1;
    }
    mlog(ML_NOTICE, "%d reservations found. LRU follows\n", i);
    i = 0;
    list_for_each_entry!(resv, &(*resmap).m_lru, r_lru) {
        mlog(ML_NOTICE, "LRU(%d) start: %u\tend: %u\tlen: %u\tlast_start: %u\tlast_len: %u\n", i, (*resv).r_start, ocfs2_resv_end(resv), (*resv).r_len, (*resv).r_last_start, (*resv).r_last_len);
        i += 1;
    }
}

#[cfg(OCFS2_CHECK_RESERVATIONS)]
unsafe fn ocfs2_validate_resmap_bits(resmap: *mut ocfs2_reservation_map, i: c_int, resv: *mut ocfs2_alloc_reservation) -> c_int {
    let mut start = (*resv).r_start;
    let end = ocfs2_resv_end(resv);
    while start <= end {
        if ocfs2_test_bit(start, (*resmap).m_disk_bitmap) != 0 {
            mlog(ML_ERROR, "reservation %d covers an allocated area starting at bit %u!\n", i, start);
            return 1;
        }
        start += 1;
    }
    0
}

#[cfg(OCFS2_CHECK_RESERVATIONS)]
unsafe fn ocfs2_check_resmap(resmap: *mut ocfs2_reservation_map) {
    let mut off = 0; let mut i = 0; let mut node = rb_first(&(*resmap).m_reservations);
    while !node.is_null() {
        let resv = rb_entry(node, struct_ocfs2_alloc_reservation, r_node);
        if i > 0 && (*resv).r_start <= off { mlog(ML_ERROR, "reservation %d has bad start off!\n", i); goto_bad!(resmap); }
        if (*resv).r_len == 0 { mlog(ML_ERROR, "reservation %d has no length!\n", i); goto_bad!(resmap); }
        if (*resv).r_start > ocfs2_resv_end(resv) { mlog(ML_ERROR, "reservation %d has invalid range!\n", i); goto_bad!(resmap); }
        if ocfs2_resv_end(resv) >= (*resmap).m_bitmap_len { mlog(ML_ERROR, "reservation %d extends past bitmap!\n", i); goto_bad!(resmap); }
        if ocfs2_validate_resmap_bits(resmap, i, resv) != 0 { goto_bad!(resmap); }
        off = ocfs2_resv_end(resv); node = rb_next(node); i += 1;
    }
    return;
bad: ocfs2_dump_resv(resmap); BUG();
}

#[cfg(not(OCFS2_CHECK_RESERVATIONS))]
unsafe fn ocfs2_check_resmap(_resmap: *mut ocfs2_reservation_map) {}

pub unsafe fn ocfs2_resv_init_once(resv: *mut ocfs2_alloc_reservation) { memset(resv as *mut c_void, 0, core::mem::size_of::<ocfs2_alloc_reservation>()); INIT_LIST_HEAD!(&mut (*resv).r_lru); }

pub unsafe fn ocfs2_resv_set_type(resv: *mut ocfs2_alloc_reservation, flags: c_uint) { BUG_ON(flags & !OCFS2_RESV_TYPES); (*resv).r_flags |= flags; }

pub unsafe fn ocfs2_resmap_init(osb: *mut ocfs2_super, resmap: *mut ocfs2_reservation_map) { memset(resmap as *mut c_void, 0, core::mem::size_of::<ocfs2_reservation_map>()); (*resmap).m_osb = osb; (*resmap).m_reservations = RB_ROOT; INIT_LIST_HEAD!(&mut (*resmap).m_lru); }

unsafe fn ocfs2_resv_mark_lru(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation) { assert_spin_locked(&resv_lock); if !list_empty(&(*resv).r_lru) { list_del_init(&mut (*resv).r_lru); } list_add_tail(&mut (*resv).r_lru, &mut (*resmap).m_lru); }
unsafe fn __ocfs2_resv_trunc(resv: *mut ocfs2_alloc_reservation) { (*resv).r_len = 0; (*resv).r_start = 0; }
unsafe fn ocfs2_resv_remove(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation) { if (*resv).r_flags & OCFS2_RESV_FLAG_INUSE != 0 { list_del_init(&mut (*resv).r_lru); rb_erase(&mut (*resv).r_node, &mut (*resmap).m_reservations); (*resv).r_flags &= !OCFS2_RESV_FLAG_INUSE; } }
unsafe fn __ocfs2_resv_discard(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation) { assert_spin_locked(&resv_lock); __ocfs2_resv_trunc(resv); (*resv).r_last_len = 0; (*resv).r_last_start = 0; ocfs2_resv_remove(resmap, resv); }

pub unsafe fn ocfs2_resv_discard(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation) { if !resv.is_null() { spin_lock(&mut resv_lock); __ocfs2_resv_discard(resmap, resv); spin_unlock(&mut resv_lock); } }

unsafe fn ocfs2_resmap_clear_all_resv(resmap: *mut ocfs2_reservation_map) { assert_spin_locked(&resv_lock); loop { let node = rb_last(&(*resmap).m_reservations); if node.is_null() { break; } let resv = rb_entry(node, struct_ocfs2_alloc_reservation, r_node); __ocfs2_resv_discard(resmap, resv); } }

pub unsafe fn ocfs2_resmap_restart(resmap: *mut ocfs2_reservation_map, clen: c_uint, disk_bitmap: *mut c_char) { if ocfs2_resmap_disabled(resmap) != 0 { return; } spin_lock(&mut resv_lock); ocfs2_resmap_clear_all_resv(resmap); (*resmap).m_bitmap_len = clen; (*resmap).m_disk_bitmap = disk_bitmap; spin_unlock(&mut resv_lock); }
pub unsafe fn ocfs2_resmap_uninit(_resmap: *mut ocfs2_reservation_map) {}

// The remaining implementation follows the C source literally; required
// kernel rbtree/list/bitmap and tracing symbols are external dependencies.
pub unsafe fn ocfs2_resmap_resv_bits(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation, cstart: *mut c_int, clen: *mut c_int) -> c_int { if resv.is_null() || ocfs2_resmap_disabled(resmap) != 0 { return -ENOSPC; } spin_lock(&mut resv_lock); if ocfs2_resv_empty(resv) != 0 { let mut wanted = ocfs2_resv_window_bits(resmap, resv); if (*resv).r_flags & OCFS2_RESV_FLAG_TMP != 0 || wanted < *clen as c_uint { wanted = *clen as c_uint; } ocfs2_resv_find_window(resmap, resv, wanted); } BUG_ON(ocfs2_resv_empty(resv)); *cstart = (*resv).r_start as c_int; *clen = (*resv).r_len as c_int; spin_unlock(&mut resv_lock); 0 }

// Helper routines below preserve the source-level API and behavior.
unsafe fn ocfs2_resv_insert(resmap: *mut ocfs2_reservation_map, new: *mut ocfs2_alloc_reservation) {
    let mut parent = core::ptr::null_mut(); let mut p = &mut (*resmap).m_reservations.rb_node as *mut *mut rb_node;
    assert_spin_locked(&resv_lock);
    while !(*p).is_null() { parent = *p; let tmp = rb_entry(parent, struct_ocfs2_alloc_reservation, r_node); if (*new).r_start < (*tmp).r_start { p = &mut (*parent).rb_left; BUG_ON(ocfs2_resv_end(new) >= (*tmp).r_start); } else if (*new).r_start > ocfs2_resv_end(tmp) { p = &mut (*parent).rb_right; } else { mlog(ML_ERROR, "Duplicate reservation window!\n"); BUG(); } }
    rb_link_node(&mut (*new).r_node, parent, p); rb_insert_color(&mut (*new).r_node, &mut (*resmap).m_reservations); (*new).r_flags |= OCFS2_RESV_FLAG_INUSE; ocfs2_resv_mark_lru(resmap, new); ocfs2_check_resmap(resmap);
}

unsafe fn ocfs2_find_resv_lhs(resmap: *mut ocfs2_reservation_map, goal: c_uint) -> *mut ocfs2_alloc_reservation {
    let mut prev = core::ptr::null_mut(); let mut node = rb_first(&(*resmap).m_reservations); assert_spin_locked(&resv_lock);
    while !node.is_null() { let resv = rb_entry(node, struct_ocfs2_alloc_reservation, r_node); if (*resv).r_start <= goal && ocfs2_resv_end(resv) >= goal { return resv; } if (*resv).r_start > goal { return prev; } prev = resv; node = rb_next(node); } prev
}

unsafe fn ocfs2_resmap_find_free_bits(resmap: *mut ocfs2_reservation_map, wanted: c_uint, search_start: c_uint, search_len: c_uint, rstart: *mut c_uint, rlen: *mut c_uint) -> c_int {
    let mut best_start = 0; let mut best_len = 0; let mut found = 0; let mut start = search_start;
    let bitmap = (*resmap).m_disk_bitmap as *mut c_void;
    while { let offset = ocfs2_find_next_zero_bit(bitmap, (*resmap).m_bitmap_len, start); if offset >= (*resmap).m_bitmap_len || offset >= search_start + search_len { false } else { if offset == start { found += 1; start += 1; } else { found = 1; start = offset + 1; } if found > best_len { best_len = found; best_start = start - found; } found < wanted } } {}
    if best_len == 0 { return 0; } if best_len >= wanted { best_len = wanted; } *rlen = best_len; *rstart = best_start; *rlen as c_int
}

unsafe fn __ocfs2_resv_find_window(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation, goal: c_uint, wanted: c_uint) {
    let mut best_start = 0; let mut best_len = 0; let mut cstart = 0; let mut clen = 0; let mut prev = ocfs2_find_resv_lhs(resmap, goal);
    if prev.is_null() { let next = rb_first(&(*resmap).m_reservations); let nr = rb_entry(next, struct_ocfs2_alloc_reservation, r_node); clen = ocfs2_resmap_find_free_bits(resmap, wanted, goal, (*nr).r_start-goal, &mut cstart, &mut clen) as c_uint; if clen != 0 { best_len=clen; best_start=cstart; if best_len==wanted { (*resv).r_start=best_start; (*resv).r_len=best_len; ocfs2_resv_insert(resmap,resv); return; } } prev=nr; }
    let mut node = &mut (*prev).r_node as *mut rb_node;
    loop { let next=rb_next(node); let (gap_start,gap_len) = if !next.is_null() { let nr=rb_entry(next,struct_ocfs2_alloc_reservation,r_node); (ocfs2_resv_end(prev)+1, (*nr).r_start-ocfs2_resv_end(prev)-1) } else { let gs=ocfs2_resv_end(prev)+1; (gs,(*resmap).m_bitmap_len-gs) }; if gap_len > best_len { clen=ocfs2_resmap_find_free_bits(resmap,wanted,gap_start,gap_len,&mut cstart,&mut clen) as c_uint; if clen==wanted {best_len=clen;best_start=cstart;break;} if clen>best_len {best_len=clen;best_start=cstart;} } if next.is_null(){break;} node=next; prev=rb_entry(node,struct_ocfs2_alloc_reservation,r_node); }
    if best_len != 0 { (*resv).r_start=best_start; (*resv).r_len=best_len; ocfs2_resv_insert(resmap,resv); }
}

unsafe fn ocfs2_resv_find_window(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation, wanted: c_uint) { let mut goal=0; BUG_ON(ocfs2_resv_empty(resv)); if (*resv).r_last_len != 0 { goal=(*resv).r_last_start+(*resv).r_last_len; if goal>=(*resmap).m_bitmap_len {goal=0;} } __ocfs2_resv_find_window(resmap,resv,goal,wanted); if ocfs2_resv_empty(resv)!=0 && goal!=0 {__ocfs2_resv_find_window(resmap,resv,0,wanted);} if ocfs2_resv_empty(resv)!=0 { let lru=list_first_entry(&(*resmap).m_lru,struct_ocfs2_alloc_reservation,r_lru); (*resv).r_start=(*lru).r_start;(*resv).r_len=(*lru).r_len;__ocfs2_resv_discard(resmap,lru);ocfs2_resv_insert(resmap,resv);} BUG_ON(ocfs2_resv_empty(resv)); }

pub unsafe fn ocfs2_resmap_claimed_bits(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation, cstart: u32, clen: u32) { if resmap.is_null() || ocfs2_resmap_disabled(resmap) != 0 || resv.is_null() { return; } BUG_ON(cstart != (*resv).r_start); spin_lock(&mut resv_lock); let cend = cstart + clen - 1; BUG_ON(cstart < (*resv).r_start); BUG_ON(cstart > ocfs2_resv_end(resv)); BUG_ON(cend > ocfs2_resv_end(resv)); ocfs2_adjust_resv_from_alloc(resmap, resv, cstart, cend); (*resv).r_last_start = cstart; (*resv).r_last_len = clen; if ocfs2_resv_empty(resv) == 0 { ocfs2_resv_mark_lru(resmap, resv); } ocfs2_check_resmap(resmap); spin_unlock(&mut resv_lock); }

unsafe fn ocfs2_adjust_resv_from_alloc(resmap: *mut ocfs2_reservation_map, resv: *mut ocfs2_alloc_reservation, start: u32, end: u32) { let old_end = ocfs2_resv_end(resv); BUG_ON(start != (*resv).r_start || old_end < end); if old_end == end { __ocfs2_resv_discard(resmap, resv); return; } let rhs = old_end - end; BUG_ON(rhs == 0); (*resv).r_start = end + 1; (*resv).r_len = old_end - (*resv).r_start + 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
