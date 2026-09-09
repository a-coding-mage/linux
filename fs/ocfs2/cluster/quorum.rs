// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) 2005 Oracle.  All rights reserved.
 */

/* Translated from quorum.c. Kernel-provided declarations and macros are
 * intentionally referenced as external dependencies. */

#[repr(C)]
struct O2quoState {
    qs_lock: spinlock_t,
    qs_work: work_struct,
    qs_pending: ::core::ffi::c_int,
    qs_heartbeating: ::core::ffi::c_int,
    qs_hb_bm: [::core::ffi::c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    qs_connected: ::core::ffi::c_int,
    qs_conn_bm: [::core::ffi::c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    qs_holds: ::core::ffi::c_int,
    qs_hold_bm: [::core::ffi::c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
}

static mut o2quo_state: O2quoState = unsafe { ::core::mem::zeroed() };

unsafe fn o2quo_fence_self() {
    o2hb_stop_all_regions();

    match (*o2nm_single_cluster).cl_fence_method {
        O2NM_FENCE_PANIC => panic!("*** ocfs2 is very sorry to be fencing this system by panicking ***\n"),
        method => {
            WARN_ON(method >= O2NM_FENCE_METHODS);
            printk(KERN_ERR, "*** ocfs2 is very sorry to be fencing this system by restarting ***\n");
            emergency_restart();
        }
    }
}

pub unsafe fn o2quo_disk_timeout() {
    o2quo_fence_self();
}

unsafe fn o2quo_make_decision(_work: *mut work_struct) {
    let mut quorum: ::core::ffi::c_int;
    let mut lowest_hb: ::core::ffi::c_int;
    let mut lowest_reachable: ::core::ffi::c_int = 0;
    let mut fence: ::core::ffi::c_int = 0;
    let qs = &mut o2quo_state;

    spin_lock_bh(&mut qs.qs_lock);
    lowest_hb = find_first_bit(qs.qs_hb_bm.as_ptr(), O2NM_MAX_NODES);
    if lowest_hb != O2NM_MAX_NODES {
        lowest_reachable = test_bit(lowest_hb, qs.qs_conn_bm.as_ptr()) as ::core::ffi::c_int;
    }
    mlog(0, "heartbeating: %d, connected: %d, lowest: %d (%sreachable)\n",
         qs.qs_heartbeating, qs.qs_connected, lowest_hb,
         if lowest_reachable != 0 { "" } else { "un" });

    if !test_bit(o2nm_this_node(), qs.qs_hb_bm.as_ptr()) || qs.qs_heartbeating == 1 {
        spin_unlock_bh(&mut qs.qs_lock);
        return;
    }

    if qs.qs_heartbeating & 1 != 0 {
        quorum = (qs.qs_heartbeating + 1) / 2;
        if qs.qs_connected < quorum { fence = 1; }
    } else {
        quorum = qs.qs_heartbeating / 2;
        if qs.qs_connected < quorum || (qs.qs_connected == quorum && lowest_reachable == 0) {
            fence = 1;
        }
    }

    if fence != 0 {
        spin_unlock_bh(&mut qs.qs_lock);
        o2quo_fence_self();
    } else {
        mlog(ML_NOTICE, "not fencing this node, heartbeating: %d, connected: %d, lowest: %d (%sreachable)\n",
             qs.qs_heartbeating, qs.qs_connected, lowest_hb,
             if lowest_reachable != 0 { "" } else { "un" });
        spin_unlock_bh(&mut qs.qs_lock);
    }
}

unsafe fn o2quo_set_hold(qs: &mut O2quoState, node: u8) {
    assert_spin_locked(&qs.qs_lock);
    if !test_and_set_bit(node, qs.qs_hold_bm.as_mut_ptr()) {
        qs.qs_holds += 1;
        mlog_bug_on_msg(qs.qs_holds == O2NM_MAX_NODES, "node %u\n", node);
        mlog(0, "node %u, %d total\n", node, qs.qs_holds);
    }
}

unsafe fn o2quo_clear_hold(qs: &mut O2quoState, node: u8) {
    assert_spin_locked(&qs.qs_lock);
    if test_and_clear_bit(node, qs.qs_hold_bm.as_mut_ptr()) {
        mlog(0, "node %u, %d total\n", node, qs.qs_holds - 1);
        qs.qs_holds -= 1;
        if qs.qs_holds == 0 && qs.qs_pending != 0 {
            qs.qs_pending = 0;
            schedule_work(&mut qs.qs_work);
        }
        mlog_bug_on_msg(qs.qs_holds < 0, "node %u, holds %d\n", node, qs.qs_holds);
    }
}

pub unsafe fn o2quo_hb_up(node: u8) {
    let qs = &mut o2quo_state; spin_lock_bh(&mut qs.qs_lock);
    qs.qs_heartbeating += 1; mlog_bug_on_msg(qs.qs_heartbeating == O2NM_MAX_NODES, "node %u\n", node);
    mlog_bug_on_msg(test_bit(node, qs.qs_hb_bm.as_ptr()), "node %u\n", node); set_bit(node, qs.qs_hb_bm.as_mut_ptr());
    mlog(0, "node %u, %d total\n", node, qs.qs_heartbeating);
    if !test_bit(node, qs.qs_conn_bm.as_ptr()) { o2quo_set_hold(qs, node); } else { o2quo_clear_hold(qs, node); } spin_unlock_bh(&mut qs.qs_lock);
}

pub unsafe fn o2quo_hb_down(node: u8) { let qs = &mut o2quo_state; spin_lock_bh(&mut qs.qs_lock); qs.qs_heartbeating -= 1; clear_bit(node, qs.qs_hb_bm.as_mut_ptr()); o2quo_clear_hold(qs, node); spin_unlock_bh(&mut qs.qs_lock); }
pub unsafe fn o2quo_hb_still_up(node: u8) { let qs = &mut o2quo_state; spin_lock_bh(&mut qs.qs_lock); mlog(0, "node %u\n", node); qs.qs_pending = 1; o2quo_clear_hold(qs, node); spin_unlock_bh(&mut qs.qs_lock); }
pub unsafe fn o2quo_conn_up(node: u8) { let qs = &mut o2quo_state; spin_lock_bh(&mut qs.qs_lock); qs.qs_connected += 1; set_bit(node, qs.qs_conn_bm.as_mut_ptr()); if !test_bit(node, qs.qs_hb_bm.as_ptr()) { o2quo_set_hold(qs, node); } else { o2quo_clear_hold(qs, node); } spin_unlock_bh(&mut qs.qs_lock); }
pub unsafe fn o2quo_conn_err(node: u8) { let qs = &mut o2quo_state; spin_lock_bh(&mut qs.qs_lock); if test_bit(node, qs.qs_conn_bm.as_ptr()) { qs.qs_connected -= 1; clear_bit(node, qs.qs_conn_bm.as_mut_ptr()); if test_bit(node, qs.qs_hb_bm.as_ptr()) { o2quo_set_hold(qs, node); } } spin_unlock_bh(&mut qs.qs_lock); }
pub unsafe fn o2quo_init() { let qs = &mut o2quo_state; spin_lock_init(&mut qs.qs_lock); INIT_WORK(&mut qs.qs_work, o2quo_make_decision); }
pub unsafe fn o2quo_exit() { flush_work(&mut o2quo_state.qs_work); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
