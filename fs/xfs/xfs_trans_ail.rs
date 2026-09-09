// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_trans_ail.c. C headers and external kernel symbols are
 * intentionally left as external dependencies. */

#[cfg(feature = "debug")]
unsafe fn xfs_ail_check(ailp: *mut xfs_ail, lip: *mut xfs_log_item) {
    if list_empty(unsafe { &(*ailp).ail_head }) { return; }
    let in_ail = test_bit(XFS_LI_IN_AIL, unsafe { &(*lip).li_flags });
    let prev_lip = list_entry(unsafe { (*lip).li_ail.prev }, "xfs_log_item", "li_ail");
    let prev_lsn = if unsafe { &(*prev_lip).li_ail } != unsafe { &(*ailp).ail_head } { unsafe { (*prev_lip).li_lsn } } else { NULLCOMMITLSN };
    let next_lip = list_entry(unsafe { (*lip).li_ail.next }, "xfs_log_item", "li_ail");
    let next_lsn = if unsafe { &(*next_lip).li_ail } != unsafe { &(*ailp).ail_head } { unsafe { (*next_lip).li_lsn } } else { NULLCOMMITLSN };
    let lsn = unsafe { (*lip).li_lsn };
    if in_ail && (prev_lsn == NULLCOMMITLSN || XFS_LSN_CMP(prev_lsn, lsn) <= 0) && (next_lsn == NULLCOMMITLSN || XFS_LSN_CMP(next_lsn, lsn) >= 0) { return; }
    spin_unlock(unsafe { &mut (*ailp).ail_lock });
    ASSERT(in_ail);
    spin_lock(unsafe { &mut (*ailp).ail_lock });
}
#[cfg(not(feature = "debug"))]
unsafe fn xfs_ail_check(_a: *mut xfs_ail, _l: *mut xfs_log_item) {}

unsafe fn xfs_ail_max(ailp: *mut xfs_ail) -> *mut xfs_log_item {
    if list_empty(unsafe { &(*ailp).ail_head }) { core::ptr::null_mut() } else { list_entry(unsafe { (*ailp).ail_head.prev }, "xfs_log_item", "li_ail") }
}
unsafe fn xfs_ail_next(ailp: *mut xfs_ail, lip: *mut xfs_log_item) -> *mut xfs_log_item {
    if unsafe { (*lip).li_ail.next == &mut (*ailp).ail_head } { core::ptr::null_mut() } else { list_first_entry(unsafe { &(*lip).li_ail }, "xfs_log_item", "li_ail") }
}
unsafe fn __xfs_ail_min_lsn(ailp: *mut xfs_ail) -> xfs_lsn_t { let lip = xfs_ail_min(ailp); if !lip.is_null() { (*lip).li_lsn } else { 0 } }

pub unsafe fn xfs_ail_min_lsn(ailp: *mut xfs_ail) -> xfs_lsn_t {
    spin_lock(&mut (*ailp).ail_lock); let lsn = __xfs_ail_min_lsn(ailp); spin_unlock(&mut (*ailp).ail_lock); lsn
}
unsafe fn xfs_trans_ail_cursor_init(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor) { (*cur).item = core::ptr::null_mut(); list_add_tail(&mut (*cur).list, &mut (*ailp).ail_cursors); }
pub unsafe fn xfs_trans_ail_cursor_next(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor) -> *mut xfs_log_item { let mut lip = (*cur).item; if (lip as usize & 1) != 0 { lip = xfs_ail_min(ailp); } if !lip.is_null() { (*cur).item = xfs_ail_next(ailp, lip); } lip }
pub unsafe fn xfs_trans_ail_cursor_done(cur: *mut xfs_ail_cursor) { (*cur).item = core::ptr::null_mut(); list_del_init(&mut (*cur).list); }
unsafe fn xfs_trans_ail_cursor_clear(ailp: *mut xfs_ail, lip: *mut xfs_log_item) { let mut cur: *mut xfs_ail_cursor; list_for_each_entry!(cur, &mut (*ailp).ail_cursors, list, { if (*cur).item == lip { (*cur).item = ((lip as usize)|1) as *mut xfs_log_item; } }); }

pub unsafe fn xfs_trans_ail_cursor_first(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, lsn: xfs_lsn_t) -> *mut xfs_log_item {
    xfs_trans_ail_cursor_init(ailp, cur); let mut lip: *mut xfs_log_item = core::ptr::null_mut();
    if lsn == 0 { lip = xfs_ail_min(ailp); } else { list_for_each_entry!(lip, &mut (*ailp).ail_head, li_ail, { if XFS_LSN_CMP((*lip).li_lsn, lsn) >= 0 { break; } }); }
    if !lip.is_null() { (*cur).item = xfs_ail_next(ailp, lip); } lip
}
unsafe fn __xfs_trans_ail_cursor_last(ailp: *mut xfs_ail, lsn: xfs_lsn_t) -> *mut xfs_log_item { let mut lip: *mut xfs_log_item = core::ptr::null_mut(); list_for_each_entry_reverse!(lip, &mut (*ailp).ail_head, li_ail, { if XFS_LSN_CMP((*lip).li_lsn, lsn) <= 0 { return lip; } }); core::ptr::null_mut() }
pub unsafe fn xfs_trans_ail_cursor_last(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, lsn: xfs_lsn_t) -> *mut xfs_log_item { xfs_trans_ail_cursor_init(ailp, cur); (*cur).item = __xfs_trans_ail_cursor_last(ailp, lsn); (*cur).item }

unsafe fn xfs_ail_splice(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, list: *mut list_head, lsn: xfs_lsn_t) { ASSERT(!list_empty(&*list)); let mut lip = if !cur.is_null() { (*cur).item } else { core::ptr::null_mut() }; if lip.is_null() || lip as usize & 1 != 0 { lip = __xfs_trans_ail_cursor_last(ailp, lsn); } if !cur.is_null() { (*cur).item = list_entry((*list).prev, "xfs_log_item", "li_ail"); } if !lip.is_null() { list_splice(list, &mut (*lip).li_ail); } else { list_splice(list, &mut (*ailp).ail_head); } }
unsafe fn xfs_ail_delete(ailp: *mut xfs_ail, lip: *mut xfs_log_item) { xfs_ail_check(ailp, lip); list_del(&mut (*lip).li_ail); xfs_trans_ail_cursor_clear(ailp, lip); }

unsafe fn xfsaild_resubmit_item(lip: *mut xfs_log_item, buffer_list: *mut list_head) -> i32 { let bp = (*lip).li_buf; if !xfs_buf_trylock(bp) { return XFS_ITEM_LOCKED; } if !xfs_buf_delwri_queue(bp, buffer_list) { xfs_buf_unlock(bp); return XFS_ITEM_FLUSHING; } list_for_each_entry!(lip, &mut (*bp).b_li_list, li_bio_list, { clear_bit(XFS_LI_FAILED, &mut (*lip).li_flags); }); xfs_buf_unlock(bp); XFS_ITEM_SUCCESS }
unsafe fn xfsaild_push_item(ailp: *mut xfs_ail, lip: *mut xfs_log_item) -> u32 { if XFS_TEST_ERROR((*(*ailp).ail_log).l_mp, XFS_ERRTAG_LOG_ITEM_PIN) { return XFS_ITEM_PINNED; } if (*(*lip).li_ops).iop_push.is_none() { return XFS_ITEM_PINNED; } if test_bit(XFS_LI_FAILED, &(*lip).li_flags) { xfsaild_resubmit_item(lip, &mut (*ailp).ail_buf_list) as u32 } else { ((*(*lip).li_ops).iop_push.unwrap())(lip, &mut (*ailp).ail_buf_list) } }

// Remaining implementation follows the C source directly; kernel list, wait,
// locking, tracing, and external XFS definitions are supplied by dependencies.
pub unsafe fn xfs_trans_ail_insert(ailp: *mut xfs_ail, lip: *mut xfs_log_item, lsn: xfs_lsn_t) { spin_lock(&mut (*ailp).ail_lock); xfs_trans_ail_update_bulk(ailp, core::ptr::null_mut(), &mut lip, 1, lsn); }
pub unsafe fn xfs_trans_ail_delete(lip: *mut xfs_log_item, shutdown_type: i32) { let ailp = (*lip).li_ailp; let log = (*ailp).ail_log; spin_lock(&mut (*ailp).ail_lock); if !test_bit(XFS_LI_IN_AIL, &(*lip).li_flags) { spin_unlock(&mut (*ailp).ail_lock); if shutdown_type != 0 && !xlog_is_shutdown(log) { xfs_alert_tag((*log).l_mp, XFS_PTAG_AILDELETE, c"xfs_trans_ail_delete: attempting to delete a log item that is not in the AIL".as_ptr()); xlog_force_shutdown(log, shutdown_type); } return; } clear_bit(XFS_LI_FAILED, &mut (*lip).li_flags); let tail = xfs_ail_delete_one(ailp, lip); xfs_ail_update_finish(ailp, tail); }
pub unsafe fn xfs_trans_ail_init(mp: *mut xfs_mount) -> i32 { let ailp = kzalloc_obj::<xfs_ail>(GFP_KERNEL | __GFP_RETRY_MAYFAIL); if ailp.is_null() { return -12; } (*ailp).ail_log = (*mp).m_log; INIT_LIST_HEAD(&mut (*ailp).ail_head); INIT_LIST_HEAD(&mut (*ailp).ail_cursors); spin_lock_init(&mut (*ailp).ail_lock); INIT_LIST_HEAD(&mut (*ailp).ail_buf_list); init_waitqueue_head(&mut (*ailp).ail_empty); (*ailp).ail_task = kthread_run(xfsaild, ailp, c"xfsaild/%s".as_ptr(), (*mp).m_super.s_id); if IS_ERR((*ailp).ail_task) { kfree(ailp); return -12; } (*mp).m_ail = ailp; 0 }
pub unsafe fn xfs_trans_ail_destroy(mp: *mut xfs_mount) { let ailp = (*mp).m_ail; kthread_stop((*ailp).ail_task); kfree(ailp); }

pub unsafe fn xfs_ail_delete_one(ailp: *mut xfs_ail, lip: *mut xfs_log_item) -> xfs_lsn_t {
    let mlip = xfs_ail_min(ailp); let lsn = (*lip).li_lsn;
    xfs_ail_delete(ailp, lip); clear_bit(XFS_LI_IN_AIL, &mut (*lip).li_flags); (*lip).li_lsn = 0;
    if mlip == lip { lsn } else { 0 }
}
pub unsafe fn xfs_ail_update_finish(ailp: *mut xfs_ail, old_lsn: xfs_lsn_t) {
    let log = (*ailp).ail_log;
    if old_lsn == 0 || old_lsn == __xfs_ail_min_lsn(ailp) { spin_unlock(&mut (*ailp).ail_lock); return; }
    __xfs_ail_assign_tail_lsn(ailp); if list_empty(&(*ailp).ail_head) { wake_up_all(&mut (*ailp).ail_empty); }
    spin_unlock(&mut (*ailp).ail_lock); xfs_log_space_wake((*log).l_mp);
}
pub unsafe fn __xfs_ail_assign_tail_lsn(ailp: *mut xfs_ail) {
    let log = (*ailp).ail_log; if xlog_is_shutdown(log) { return; }
    let mut tail = __xfs_ail_min_lsn(ailp); if tail == 0 { tail = (*ailp).ail_head_lsn; }
    (*log).l_tail_space = xlog_lsn_sub(log, (*ailp).ail_head_lsn, tail); atomic64_set(&mut (*log).l_tail_lsn, tail);
}
pub unsafe fn xfs_trans_ail_update_bulk(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, items: *mut *mut xfs_log_item, n: i32, lsn: xfs_lsn_t) {
    let mut tmp = LIST_HEAD_INIT(); let mut mlip = xfs_ail_min(ailp); let mut tail = 0;
    for i in 0..n { let lip = *items.add(i as usize); if test_and_set_bit(XFS_LI_IN_AIL, &mut (*lip).li_flags) { if XFS_LSN_CMP(lsn, (*lip).li_lsn) <= 0 { continue; } if mlip == lip && tail == 0 { tail = (*lip).li_lsn; } xfs_ail_delete(ailp, lip); } (*lip).li_lsn = lsn; list_add_tail(&mut (*lip).li_ail, &mut tmp); }
    if !list_empty(&tmp) { xfs_ail_splice(ailp, cur, &mut tmp, lsn); } if mlip.is_null() { wake_up_process((*ailp).ail_task); tail = NULLCOMMITLSN; } xfs_ail_update_finish(ailp, tail);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
