/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Dynamic queue limits (dql) - Definitions
 *
 * Copyright (c) 2011, Tom Herbert <therbert@google.com>
 *
 * This header file contains the definitions for dynamic queue limits (dql).
 * dql would be used in conjunction with a producer/consumer type queue
 * (possibly a HW queue). Such a queue would have these general properties:
 *
 *   1) Objects are queued up to some limit specified as number of objects.
 *   2) Periodically a completion process executes which retires consumed
 *      objects.
 *   3) Starvation occurs when limit has been reached, all queued data has
 *      actually been consumed, but completion processing has not yet run
 *      so queuing new data is blocked.
 *   4) Minimizing the amount of queued data is desirable.
 *
 * The goal of dql is to calculate the limit as the minimum number of objects
 * needed to prevent starvation.
 *
 * The primary functions of dql are:
 *    dql_queued - called when objects are enqueued to record number of objects
 *    dql_avail - returns how many objects are available to be queued based
 *      on the object limit and how many objects are already enqueued
 *    dql_completed - called at completion time to indicate how many objects
 *      were retired from the queue
 */

use core::ffi::{c_int, c_uint, c_ulong};

pub const DQL_HIST_LEN: usize = 4;

#[repr(C)]
pub struct dql {
    /* Fields accessed in enqueue path (dql_queued) */
    pub num_queued: c_uint,
    pub adj_limit: c_uint,
    pub last_obj_cnt: c_uint,

    /* Stall threshold (in jiffies), defined by user */
    pub stall_thrs: u16,

    pub history_head: c_ulong,
    /* stall entries, a bit per entry */
    pub history: [c_ulong; DQL_HIST_LEN],

    /* Fields accessed only by completion path (dql_completed) */
    pub limit: c_uint, /* ____cacheline_aligned_in_smp */
    pub num_completed: c_uint,

    pub prev_ovlimit: c_uint,
    pub prev_num_queued: c_uint,
    pub prev_last_obj_cnt: c_uint,

    pub lowest_slack: c_uint,
    pub slack_start_time: c_ulong,

    /* Configuration */
    pub max_limit: c_uint,
    pub min_limit: c_uint,
    pub slack_hold_time: c_uint,

    /* Longest stall detected, reported to user */
    pub stall_max: u16,
    pub last_reap: c_ulong,
    pub stall_cnt: c_ulong,
}

/* Set some static maximums */
pub const DQL_MAX_OBJECT: c_uint = c_uint::MAX / 16;
pub const DQL_MAX_LIMIT: c_uint = (c_uint::MAX / 2) - DQL_MAX_OBJECT;

/* The following symbols are supplied by the surrounding kernel environment. */
unsafe extern "C" {
    pub static mut jiffies: c_ulong;
    pub static BITS_PER_LONG: c_ulong;
    pub fn unlikely(value: bool) -> bool;
    pub fn WARN_ON_ONCE(condition: bool) -> bool;
    pub fn smp_wmb();
}

#[inline]
unsafe fn dql_hist_ent(dql: *mut dql, idx: c_ulong) -> *mut c_ulong {
    (*dql).history.as_mut_ptr().add((idx as usize) % DQL_HIST_LEN)
}

#[inline]
unsafe fn read_once<T: Copy>(ptr: *const T) -> T {
    core::ptr::read_volatile(ptr)
}

#[inline]
unsafe fn write_once<T>(ptr: *mut T, value: T) {
    core::ptr::write_volatile(ptr, value);
}

/* Populate the bitmap to be processed later in dql_check_stall() */
#[inline]
pub unsafe fn dql_queue_stall(dql: *mut dql) {
    let mut map: c_ulong;
    let now: c_ulong = jiffies;
    let now_hi: c_ulong = now / BITS_PER_LONG;
    let mut i: c_ulong = 0;

    /* The following code sets a bit in the ring buffer, where each bit tracks
     * time the packet was queued. The dql->history buffer tracks
     * DQL_HIST_LEN * BITS_PER_LONG time (jiffies) slots.
     */
    if unlikely(now_hi != (*dql).history_head) {
        /* About to reuse slots, clear them */
        while i < DQL_HIST_LEN as c_ulong {
            /* Multiplication masks high bits */
            if now_hi * BITS_PER_LONG == ((*dql).history_head + i) * BITS_PER_LONG {
                break;
            }
            write_once(dql_hist_ent(dql, (*dql).history_head + i + 1), 0);
            i += 1;
        }
        /* pairs with smp_rmb() in dql_check_stall() */
        smp_wmb();
        write_once(&mut (*dql).history_head, now_hi);
    }

    /* __set_bit() does not guarantee WRITE_ONCE() semantics */
    map = read_once(dql_hist_ent(dql, now_hi));

    /* Populate the history with an entry (bit) per queued */
    let bit = 1 as c_ulong << (now % BITS_PER_LONG);
    if map & bit == 0 {
        write_once(dql_hist_ent(dql, now_hi), map | bit);
    }
}

/*
 * Record number of objects queued. Assumes that caller has already checked
 * availability in the queue with dql_avail.
 */
#[inline]
pub unsafe fn dql_queued(dql: *mut dql, count: c_uint) {
    if WARN_ON_ONCE(count > DQL_MAX_OBJECT) {
        return;
    }

    write_once(&mut (*dql).last_obj_cnt, count);

    /* We want to force a write first, so that cpu do not attempt to get cache
     * line containing last_obj_cnt, num_queued, adj_limit in Shared state,
     * but directly does a Request For Ownership. It is only a hint.
     */
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    (*dql).num_queued = (*dql).num_queued.wrapping_add(count);

    /* Only populate stall information if the threshold is set */
    if read_once(&(*dql).stall_thrs) != 0 {
        dql_queue_stall(dql);
    }
}

/* Returns how many objects can be queued, < 0 indicates over limit. */
#[inline]
pub unsafe fn dql_avail(dql: *const dql) -> c_int {
    read_once(&(*dql).adj_limit) as c_int - read_once(&(*dql).num_queued) as c_int
}

/* Record number of completed objects and recalculate the limit. */
unsafe extern "C" {
    pub fn dql_completed(dql: *mut dql, count: c_uint);
    /* Reset dql state */
    pub fn dql_reset(dql: *mut dql);
    /* Initialize dql state */
    pub fn dql_init(dql: *mut dql, hold_time: c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
