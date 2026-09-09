// SPDX-License-Identifier: GPL-2.0+
/*
 * RCU segmented callback lists, function definitions
 *
 * Copyright IBM Corporation, 2017
 *
 * Authors: Paul E. McKenney <paulmck@linux.ibm.com>
 */

// C dependencies: linux/cpu.h, linux/interrupt.h, linux/kernel.h,
// linux/types.h, rcu.h, and rcu_segcblist.h.

pub unsafe fn rcu_cblist_init(rclp: *mut rcu_cblist) {
    (*rclp).head = core::ptr::null_mut();
    (*rclp).tail = &mut (*rclp).head;
    (*rclp).len = 0;
}

pub unsafe fn rcu_cblist_enqueue(rclp: *mut rcu_cblist, rhp: *mut rcu_head) {
    *(*rclp).tail = rhp;
    (*rclp).tail = &mut (*rhp).next;
    WRITE_ONCE((*rclp).len, (*rclp).len + 1);
}

pub unsafe fn rcu_cblist_flush_enqueue(drclp: *mut rcu_cblist, srclp: *mut rcu_cblist, rhp: *mut rcu_head) {
    (*drclp).head = (*srclp).head;
    if !(*drclp).head.is_null() { (*drclp).tail = (*srclp).tail; }
    else { (*drclp).tail = &mut (*drclp).head; }
    (*drclp).len = (*srclp).len;
    if rhp.is_null() { rcu_cblist_init(srclp); }
    else {
        (*rhp).next = core::ptr::null_mut();
        (*srclp).head = rhp;
        (*srclp).tail = &mut (*rhp).next;
        WRITE_ONCE((*srclp).len, 1);
    }
}

pub unsafe fn rcu_cblist_dequeue(rclp: *mut rcu_cblist) -> *mut rcu_head {
    let rhp = (*rclp).head;
    if rhp.is_null() { return core::ptr::null_mut(); }
    (*rclp).len -= 1;
    (*rclp).head = (*rhp).next;
    if (*rclp).head.is_null() { (*rclp).tail = &mut (*rclp).head; }
    rhp
}

unsafe fn rcu_segcblist_set_len(rsclp: *mut rcu_segcblist, v: i64) {
    // CONFIG_RCU_NOCB_CPU selects atomic_long_set in the C build.
    WRITE_ONCE((*rsclp).len, v);
}

pub unsafe fn rcu_segcblist_get_seglen(rsclp: *mut rcu_segcblist, seg: i32) -> i64 {
    READ_ONCE((*rsclp).seglen[seg as usize])
}

pub unsafe fn rcu_segcblist_n_segment_cbs(rsclp: *mut rcu_segcblist) -> i64 {
    let mut len = 0;
    let mut i = RCU_DONE_TAIL;
    while i < RCU_CBLIST_NSEGS { len += rcu_segcblist_get_seglen(rsclp, i); i += 1; }
    len
}

unsafe fn rcu_segcblist_set_seglen(rsclp: *mut rcu_segcblist, seg: i32, v: i64) {
    WRITE_ONCE((*rsclp).seglen[seg as usize], v);
}
unsafe fn rcu_segcblist_add_seglen(rsclp: *mut rcu_segcblist, seg: i32, v: i64) {
    WRITE_ONCE((*rsclp).seglen[seg as usize], (*rsclp).seglen[seg as usize] + v);
}
unsafe fn rcu_segcblist_move_seglen(rsclp: *mut rcu_segcblist, from: i32, to: i32) {
    if from == to { return; }
    let len = rcu_segcblist_get_seglen(rsclp, from);
    if len == 0 { return; }
    rcu_segcblist_add_seglen(rsclp, to, len);
    rcu_segcblist_set_seglen(rsclp, from, 0);
}
unsafe fn rcu_segcblist_inc_seglen(rsclp: *mut rcu_segcblist, seg: i32) { rcu_segcblist_add_seglen(rsclp, seg, 1); }

pub unsafe fn rcu_segcblist_add_len(rsclp: *mut rcu_segcblist, v: i64) {
    smp_mb();
    WRITE_ONCE((*rsclp).len, (*rsclp).len + v);
    smp_mb();
}
pub unsafe fn rcu_segcblist_inc_len(rsclp: *mut rcu_segcblist) { rcu_segcblist_add_len(rsclp, 1); }

pub unsafe fn rcu_segcblist_init(rsclp: *mut rcu_segcblist) {
    let mut i = 0;
    while i < RCU_CBLIST_NSEGS {
        (*rsclp).tails[i as usize] = &mut (*rsclp).head;
        rcu_segcblist_set_seglen(rsclp, i, 0); i += 1;
    }
    rcu_segcblist_set_len(rsclp, 0);
    rcu_segcblist_set_flags(rsclp, SEGCBLIST_ENABLED);
}

pub unsafe fn rcu_segcblist_disable(rsclp: *mut rcu_segcblist) {
    WARN_ON_ONCE(!rcu_segcblist_empty(rsclp));
    WARN_ON_ONCE(rcu_segcblist_n_cbs(rsclp));
    rcu_segcblist_clear_flags(rsclp, SEGCBLIST_ENABLED);
}
pub unsafe fn rcu_segcblist_ready_cbs(rsclp: *mut rcu_segcblist) -> bool {
    rcu_segcblist_is_enabled(rsclp) && &mut (*rsclp).head != READ_ONCE((*rsclp).tails[RCU_DONE_TAIL as usize])
}
pub unsafe fn rcu_segcblist_pend_cbs(rsclp: *mut rcu_segcblist) -> bool {
    rcu_segcblist_is_enabled(rsclp) && !rcu_segcblist_restempty(rsclp, RCU_DONE_TAIL)
}
pub unsafe fn rcu_segcblist_first_cb(rsclp: *mut rcu_segcblist) -> *mut rcu_head {
    if rcu_segcblist_is_enabled(rsclp) { (*rsclp).head } else { core::ptr::null_mut() }
}
pub unsafe fn rcu_segcblist_first_pend_cb(rsclp: *mut rcu_segcblist) -> *mut rcu_head {
    if rcu_segcblist_is_enabled(rsclp) { *(*rsclp).tails[RCU_DONE_TAIL as usize] } else { core::ptr::null_mut() }
}
pub unsafe fn rcu_segcblist_nextgp(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool {
    if !rcu_segcblist_pend_cbs(rsclp) { return false; } *gsp = (*rsclp).gp_seq[RCU_WAIT_TAIL as usize]; true
}

pub unsafe fn rcu_segcblist_enqueue(rsclp: *mut rcu_segcblist, rhp: *mut rcu_head) {
    rcu_segcblist_inc_len(rsclp); rcu_segcblist_inc_seglen(rsclp, RCU_NEXT_TAIL);
    (*rhp).next = core::ptr::null_mut();
    WRITE_ONCE(*(*rsclp).tails[RCU_NEXT_TAIL as usize], rhp);
    WRITE_ONCE((*rsclp).tails[RCU_NEXT_TAIL as usize], &mut (*rhp).next);
}

pub unsafe fn rcu_segcblist_entrain(rsclp: *mut rcu_segcblist, rhp: *mut rcu_head) -> bool {
    if rcu_segcblist_n_cbs(rsclp) == 0 { return false; }
    rcu_segcblist_inc_len(rsclp); smp_mb(); (*rhp).next = core::ptr::null_mut();
    let mut i = RCU_NEXT_TAIL;
    while i > RCU_DONE_TAIL { if !rcu_segcblist_segempty(rsclp, i) { break; } i -= 1; }
    rcu_segcblist_inc_seglen(rsclp, i); WRITE_ONCE(*(*rsclp).tails[i as usize], rhp);
    while i <= RCU_NEXT_TAIL { WRITE_ONCE((*rsclp).tails[i as usize], &mut (*rhp).next); i += 1; }
    true
}

pub unsafe fn rcu_segcblist_extract_done_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist) {
    if !rcu_segcblist_ready_cbs(rsclp) { return; }
    (*rclp).len = rcu_segcblist_get_seglen(rsclp, RCU_DONE_TAIL);
    *(*rclp).tail = (*rsclp).head; WRITE_ONCE((*rsclp).head, *(*rsclp).tails[RCU_DONE_TAIL as usize]);
    WRITE_ONCE(*(*rsclp).tails[RCU_DONE_TAIL as usize], core::ptr::null_mut());
    (*rclp).tail = (*rsclp).tails[RCU_DONE_TAIL as usize];
    let mut i = RCU_CBLIST_NSEGS - 1; while i >= RCU_DONE_TAIL { if (*rsclp).tails[i as usize] == (*rsclp).tails[RCU_DONE_TAIL as usize] { WRITE_ONCE((*rsclp).tails[i as usize], &mut (*rsclp).head); } i -= 1; }
    rcu_segcblist_set_seglen(rsclp, RCU_DONE_TAIL, 0);
}

pub unsafe fn rcu_segcblist_extract_pend_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist) {
    if !rcu_segcblist_pend_cbs(rsclp) { return; }
    (*rclp).len = 0; *(*rclp).tail = *(*rsclp).tails[RCU_DONE_TAIL as usize]; (*rclp).tail = (*rsclp).tails[RCU_NEXT_TAIL as usize];
    WRITE_ONCE(*(*rsclp).tails[RCU_DONE_TAIL as usize], core::ptr::null_mut());
    let mut i = RCU_DONE_TAIL + 1; while i < RCU_CBLIST_NSEGS { (*rclp).len += rcu_segcblist_get_seglen(rsclp, i); WRITE_ONCE((*rsclp).tails[i as usize], (*rsclp).tails[RCU_DONE_TAIL as usize]); rcu_segcblist_set_seglen(rsclp, i, 0); i += 1; }
}

pub unsafe fn rcu_segcblist_insert_count(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist) { rcu_segcblist_add_len(rsclp, (*rclp).len); }
pub unsafe fn rcu_segcblist_insert_done_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist) {
    if (*rclp).head.is_null() { return; } rcu_segcblist_add_seglen(rsclp, RCU_DONE_TAIL, (*rclp).len); *(*rclp).tail = (*rsclp).head; WRITE_ONCE((*rsclp).head, (*rclp).head);
    let mut i = RCU_DONE_TAIL; while i < RCU_CBLIST_NSEGS { if &mut (*rsclp).head == (*rsclp).tails[i as usize] { WRITE_ONCE((*rsclp).tails[i as usize], (*rclp).tail); } else { break; } i += 1; }
    (*rclp).head = core::ptr::null_mut(); (*rclp).tail = &mut (*rclp).head;
}
pub unsafe fn rcu_segcblist_insert_pend_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist) {
    if (*rclp).head.is_null() { return; } rcu_segcblist_add_seglen(rsclp, RCU_NEXT_TAIL, (*rclp).len); WRITE_ONCE(*(*rsclp).tails[RCU_NEXT_TAIL as usize], (*rclp).head); WRITE_ONCE((*rsclp).tails[RCU_NEXT_TAIL as usize], (*rclp).tail);
}

unsafe fn rcu_segcblist_advance_compact(rsclp: *mut rcu_segcblist, mut i: i32) {
    let mut j = RCU_WAIT_TAIL;
    while j < i { WRITE_ONCE((*rsclp).tails[j as usize], (*rsclp).tails[RCU_DONE_TAIL as usize]); j += 1; }
    j = RCU_WAIT_TAIL;
    while i < RCU_NEXT_TAIL {
        if (*rsclp).tails[j as usize] == (*rsclp).tails[RCU_NEXT_TAIL as usize] { break; }
        WRITE_ONCE((*rsclp).tails[j as usize], (*rsclp).tails[i as usize]);
        rcu_segcblist_move_seglen(rsclp, i, j); (*rsclp).gp_seq[j as usize] = (*rsclp).gp_seq[i as usize]; i += 1; j += 1;
    }
}

pub unsafe fn rcu_segcblist_advance(rsclp: *mut rcu_segcblist) {
    WARN_ON_ONCE(!rcu_segcblist_is_enabled(rsclp)); if rcu_segcblist_restempty(rsclp, RCU_DONE_TAIL) { return; }
    let mut i = RCU_WAIT_TAIL;
    while i < RCU_NEXT_TAIL { if !poll_state_synchronize_rcu_full(&mut (*rsclp).gp_seq[i as usize]) { break; } WRITE_ONCE((*rsclp).tails[RCU_DONE_TAIL as usize], (*rsclp).tails[i as usize]); rcu_segcblist_move_seglen(rsclp, i, RCU_DONE_TAIL); i += 1; }
    if i == RCU_WAIT_TAIL { return; } rcu_segcblist_advance_compact(rsclp, i);
}

pub unsafe fn rcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool {
    WARN_ON_ONCE(!rcu_segcblist_is_enabled(rsclp)); if rcu_segcblist_restempty(rsclp, RCU_DONE_TAIL) { return false; }
    let mut i = RCU_NEXT_READY_TAIL;
    while i > RCU_DONE_TAIL { if !rcu_segcblist_segempty(rsclp, i) && ULONG_CMP_LT((*rsclp).gp_seq[i as usize].norm, (*gsp).norm) { break; } i -= 1; }
    if rcu_segcblist_restempty(rsclp, i) || { i += 1; i >= RCU_NEXT_TAIL } { return false; }
    let mut j = i + 1; while j <= RCU_NEXT_TAIL { rcu_segcblist_move_seglen(rsclp, j, i); j += 1; }
    while i < RCU_NEXT_TAIL { WRITE_ONCE((*rsclp).tails[i as usize], (*rsclp).tails[RCU_NEXT_TAIL as usize]); (*rsclp).gp_seq[i as usize] = *gsp; i += 1; }
    true
}

pub unsafe fn rcu_segcblist_merge(dst_rsclp: *mut rcu_segcblist, src_rsclp: *mut rcu_segcblist) {
    let mut donecbs: rcu_cblist = core::mem::zeroed(); let mut pendcbs: rcu_cblist = core::mem::zeroed();
    lockdep_assert_cpus_held(); rcu_cblist_init(&mut donecbs); rcu_cblist_init(&mut pendcbs);
    rcu_segcblist_extract_done_cbs(src_rsclp, &mut donecbs); rcu_segcblist_extract_pend_cbs(src_rsclp, &mut pendcbs); rcu_segcblist_set_len(src_rsclp, 0);
    rcu_segcblist_insert_count(dst_rsclp, &mut donecbs); rcu_segcblist_insert_count(dst_rsclp, &mut pendcbs); rcu_segcblist_insert_done_cbs(dst_rsclp, &mut donecbs); rcu_segcblist_insert_pend_cbs(dst_rsclp, &mut pendcbs); rcu_segcblist_init(src_rsclp);
}

pub unsafe fn srcu_segcblist_advance(rsclp: *mut rcu_segcblist, seq: u64) {
    WARN_ON_ONCE(!rcu_segcblist_is_enabled(rsclp)); if rcu_segcblist_restempty(rsclp, RCU_DONE_TAIL) { return; }
    let mut i = RCU_WAIT_TAIL;
    while i < RCU_NEXT_TAIL { if ULONG_CMP_LT(seq, (*rsclp).gp_seq[i as usize].norm) { break; } WRITE_ONCE((*rsclp).tails[RCU_DONE_TAIL as usize], (*rsclp).tails[i as usize]); rcu_segcblist_move_seglen(rsclp, i, RCU_DONE_TAIL); i += 1; }
    if i != RCU_WAIT_TAIL { rcu_segcblist_advance_compact(rsclp, i); }
}

pub unsafe fn srcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, seq: u64) -> bool {
    let mut gs: rcu_gp_seq = core::mem::zeroed(); gs.norm = seq; gs.exp = RCU_GET_STATE_NOT_TRACKED; rcu_segcblist_accelerate(rsclp, &mut gs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
