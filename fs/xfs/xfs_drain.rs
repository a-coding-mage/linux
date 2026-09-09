// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * Use a static key here to reduce the overhead of xfs_defer_drain_rele.  If
 * the compiler supports jump labels, the static branch will be replaced by a
 * nop sled when there are no xfs_defer_drain_wait callers.  Online fsck is
 * currently the only caller, so this is a reasonable tradeoff.
 *
 * Note: Patching the kernel code requires taking the cpu hotplug lock.  Other
 * parts of the kernel allocate memory with that lock held, which means that
 * XFS callers cannot hold any locks that might be used by memory reclaim or
 * writeback when calling the static_branch_{inc,dec} functions.
 */
static mut XFS_DEFER_DRAIN_WAITER_GATE: StaticKeyFalse = StaticKeyFalse::new();

pub unsafe fn xfs_defer_drain_wait_disable() {
    static_branch_dec(&mut XFS_DEFER_DRAIN_WAITER_GATE);
}

pub unsafe fn xfs_defer_drain_wait_enable() {
    static_branch_inc(&mut XFS_DEFER_DRAIN_WAITER_GATE);
}

pub unsafe fn xfs_defer_drain_init(dr: *mut xfs_defer_drain) {
    atomic_set(&mut (*dr).dr_count, 0);
    init_waitqueue_head(&mut (*dr).dr_waiters);
}

pub unsafe fn xfs_defer_drain_free(dr: *mut xfs_defer_drain) {
    ASSERT(atomic_read(&(*dr).dr_count) == 0);
}

/* Increase the pending intent count. */
unsafe fn xfs_defer_drain_grab(dr: *mut xfs_defer_drain) {
    atomic_inc(&mut (*dr).dr_count);
}

unsafe fn has_waiters(wq_head: *mut wait_queue_head) -> bool {
    /*
     * This memory barrier is paired with the one in set_current_state on
     * the waiting side.
     */
    smp_mb__after_atomic();
    waitqueue_active(wq_head)
}

/* Decrease the pending intent count, and wake any waiters, if appropriate. */
unsafe fn xfs_defer_drain_rele(dr: *mut xfs_defer_drain) {
    if atomic_dec_and_test(&mut (*dr).dr_count)
        && static_branch_unlikely(&XFS_DEFER_DRAIN_WAITER_GATE)
        && has_waiters(&mut (*dr).dr_waiters)
    {
        wake_up(&mut (*dr).dr_waiters);
    }
}

/* Are there intents pending? */
unsafe fn xfs_defer_drain_busy(dr: *mut xfs_defer_drain) -> bool {
    atomic_read(&(*dr).dr_count) > 0
}

/*
 * Wait for the pending intent count for a drain to hit zero.
 *
 * Callers must not hold any locks that would prevent intents from being
 * finished.
 */
unsafe fn xfs_defer_drain_wait(dr: *mut xfs_defer_drain) -> i32 {
    wait_event_killable(&mut (*dr).dr_waiters, !xfs_defer_drain_busy(dr))
}

/*
 * Get a passive reference to the group that contains a fsbno and declare an
 * intent to update its metadata.
 *
 * Other threads that need exclusive access can decide to back off if they see
 * declared intentions.
 */
pub unsafe fn xfs_group_intent_get(
    mp: *mut xfs_mount,
    fsbno: xfs_fsblock_t,
    type_: xfs_group_type,
) -> *mut xfs_group {
    let xg = xfs_group_get_by_fsb(mp, fsbno, type_);
    if xg.is_null() {
        return core::ptr::null_mut();
    }
    trace_xfs_group_intent_hold(xg, __return_address());
    xfs_defer_drain_grab(&mut (*xg).xg_intents_drain);
    xg
}

/*
 * Release our intent to update this groups metadata, and then release our
 * passive ref to it.
 */
pub unsafe fn xfs_group_intent_put(xg: *mut xfs_group) {
    trace_xfs_group_intent_rele(xg, __return_address());
    xfs_defer_drain_rele(&mut (*xg).xg_intents_drain);
    xfs_group_put(xg);
}

/*
 * Wait for the intent update count for this AG to hit zero.
 * Callers must not hold any AG header buffers.
 */
pub unsafe fn xfs_group_intent_drain(xg: *mut xfs_group) -> i32 {
    trace_xfs_group_wait_intents(xg, __return_address());
    xfs_defer_drain_wait(&mut (*xg).xg_intents_drain)
}

/*
 * Has anyone declared an intent to update this group?
 */
pub unsafe fn xfs_group_intent_busy(xg: *mut xfs_group) -> bool {
    xfs_defer_drain_busy(&mut (*xg).xg_intents_drain)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
