// SPDX-License-Identifier: GPL-2.0
// F03 source continuation of retained ext.rs, pinned ext.c:2376-2916 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves are unqualified C
// runtime. No old C owner algorithm, substitute layout or fake success path.
compile_error!("SOURCE ONLY HOLD: sched_ext consume ABI, ownership and protection qualification incomplete");

use super::*;
use core::ptr;

/// Insert an already-local task into its capability-resolved rq queue (2376-2398).
///
/// # Safety
/// sch, p, src_dsq and dst_rq remain live. Caller holds dst_rq's lock and,
/// unless src_dsq is rq-owned, its DSQ lock; p is on dst_rq and already unlinked.
/// The original task-custody protocol must serialize flags/list mutation.
#[no_mangle]
pub unsafe extern "C" fn scx_move_local_task_to_local_dsq(
    sch: *mut scx_sched, p: *mut task_struct, mut enq_flags: u64,
    src_dsq: *mut scx_dispatch_q, dst_rq: *mut rq,
) {
    // SAFETY: Resolution may change flags and select REJECT/RESCUE. Resolve
    // before assertions, as in C, then preserve list/accounting/custody order.
    unsafe {
        let dst_dsq = lupos_scx_core_consume_resolve_local(sch, dst_rq, p, &mut enq_flags);
        if !dsq_is_rq_owned(src_dsq) {
            lupos_scx_core_consume_assert_local_src(src_dsq);
        }
        lupos_scx_core_consume_assert_local_dst(dst_rq);
        lupos_scx_core_consume_warn_local_holding(p);
        if enq_flags & (SCX_ENQ_HEAD as u64 | SCX_ENQ_PREEMPT as u64) != 0 {
            dsq_insert_head(dst_dsq, p);
        } else {
            lupos_scx_core_consume_list_add_tail(
                ptr::addr_of_mut!((*p).scx.dsq_list.node),
                ptr::addr_of_mut!((*dst_dsq).list),
            );
        }
        dsq_inc_nr(dst_dsq, p, enq_flags);
        (*p).scx.dsq = dst_dsq;
        rq_owned_post_enq(sch, dst_rq, dst_dsq, p, enq_flags);
    }
}

/// Migrate with sticky-CPU and placing-scheduler state intact (2410-2440).
///
/// # Safety
/// p/sch/rqs stay live; src_rq is locked and p belongs to it. Caller has the
/// original migration custody, validated destination and pinned CPU context.
/// This consumes src_rq's lock and returns with dst_rq locked, not both locks.
unsafe fn move_remote_task_to_local_dsq(
    sch: *mut scx_sched, p: *mut task_struct, enq_flags: u64,
    src_rq: *mut rq, dst_rq: *mut rq,
) {
    // SAFETY: Sticky CPU precedes deactivate; changing task CPU precedes lock
    // switching. The original placing scheduler and full-width flags are
    // stashed only across activate_task, whose enqueue callback reads them.
    unsafe {
        lupos_scx_core_consume_assert_remote_src(src_rq);
        (*p).scx.sticky_cpu = lupos_scx_core_consume_cpu_of(dst_rq);
        deactivate_task(src_rq, p, 0);
        lupos_scx_core_consume_set_task_cpu(p, dst_rq);
        switch_rq_lock(src_rq, dst_rq);
        lupos_scx_core_consume_warn_remote_affinity(dst_rq, p);
        lupos_scx_core_consume_warn_remote_stash(dst_rq);
        (*dst_rq).scx.remote_activate_enq_flags = enq_flags;
        (*dst_rq).scx.remote_activate_sch = sch;
        activate_task(dst_rq, p, 0);
        (*dst_rq).scx.remote_activate_enq_flags = 0;
        (*dst_rq).scx.remote_activate_sch = ptr::null_mut();
    }
}

/// Check BPF migration eligibility, before affinity and online checks (2462-2517).
///
/// # Safety
/// Objects and CPU topology remain live; p and rq belong to different CPUs.
/// With enforce, p's rq lock is held. Without enforce the caller's locked DSQ
/// stabilizes the task and follows the native permitted migration-state reads;
/// that concurrent native/Rust access contract is not yet qualified.
pub(crate) unsafe fn task_can_run_on_remote_rq(
    sch: *mut scx_sched, p: *mut task_struct, rq: *mut rq, enforce: bool,
) -> bool {
    // SAFETY: Checking migration-disabled first preserves the put_prev versus
    // migrate_disable_switch race handling. Errors/events occur only on the
    // enforce path, under p's rq lock; offline alone never emits scx_error.
    unsafe {
        let cpu = lupos_scx_core_consume_cpu_of(rq);
        if enforce {
            lupos_scx_core_consume_assert_enforce_task_rq(p);
        }
        lupos_scx_core_consume_warn_same_cpu(p, cpu);
        if lupos_scx_core_consume_unlikely_migration_disabled(p) {
            if enforce {
                lupos_scx_core_consume_error_migration_disabled(sch, p, cpu);
            }
            return false;
        }
        if !lupos_scx_core_consume_task_allowed(p, cpu) {
            if enforce {
                lupos_scx_core_consume_error_not_allowed(sch, p, cpu);
            }
            return false;
        }
        if !scx_rq_online(rq) {
            if enforce {
                lupos_scx_core_consume_event_offline(sch);
            }
            return false;
        }
        true
    }
}

/// Transfer from DSQ protection to the source rq and recheck custody (2549-2569).
///
/// # Safety
/// Caller holds dsq and locked_rq locks; p remains lifetime-pinned by the
/// surrounding dispatch/iterator protocol through the unlock/recheck window.
/// src_rq is p's rq observed under dsq lock. Execution cannot preempt/recurse.
/// Returns with dsq unlocked and src_rq locked, including on lost custody.
/// No Rust reference into p/dsq survives the lock switch.
pub(crate) unsafe fn unlink_dsq_and_switch_rq_lock(
    p: *mut task_struct, dsq: *mut scx_dispatch_q,
    locked_rq: *mut rq, src_rq: *mut rq,
) -> bool {
    // SAFETY: holding_cpu is published before releasing the DSQ lock; only
    // after acquiring src_rq is it rechecked. Failed holding comparison must
    // short-circuit the rq consistency diagnostic and all later task mutation.
    unsafe {
        let cpu = lupos_scx_core_consume_raw_cpu();
        lupos_scx_core_consume_assert_unlink_dsq(dsq);
        lupos_scx_core_consume_assert_unlink_rq(locked_rq);
        lupos_scx_core_consume_warn_unlink_holding(p);
        scx_task_unlink_from_dsq(p, dsq);
        (*p).scx.holding_cpu = cpu;
        lupos_scx_core_consume_unlock(dsq);
        switch_rq_lock(locked_rq, src_rq);
        lupos_scx_core_consume_likely_unlink_held(p, cpu)
            && !lupos_scx_core_consume_warn_unlink_rq_changed(src_rq, p)
    }
}

/// Consume remotely or restore the original rq lock after losing (2571-2582).
///
/// # Safety
/// Same lifetime, CPU pinning and entry locks as unlink_dsq_and_switch_rq_lock;
/// destination eligibility was checked while dsq was locked. Both outcomes
/// leave this_rq locked and dsq unlocked, and only success migrates the task.
unsafe fn consume_remote_task(
    sch: *mut scx_sched, this_rq: *mut rq, p: *mut task_struct,
    enq_flags: u64, dsq: *mut scx_dispatch_q, src_rq: *mut rq,
) -> bool {
    // SAFETY: On loss, do not retry using stale p or attempt a second unlink.
    unsafe {
        if unlink_dsq_and_switch_rq_lock(p, dsq, this_rq, src_rq) {
            move_remote_task_to_local_dsq(sch, p, enq_flags, src_rq, this_rq);
            true
        } else {
            switch_rq_lock(src_rq, this_rq);
            false
        }
    }
}

/// Move between DSQs from source-rq custody to destination-rq custody (2600-2650).
///
/// # Safety
/// sch/p/queues stay live; caller holds p's rq and src_dsq locks. src_dsq is
/// nonlocal; dst_dsq is any native DSQ. For a remote local destination this
/// may migrate p and transfer the rq lock. Returns with src_dsq unlocked and
/// only the returned rq locked. No holding_cpu transfer is added to this path.
pub(crate) unsafe fn move_task_between_dsqs(
    sch: *mut scx_sched, p: *mut task_struct, mut enq_flags: u64,
    src_dsq: *mut scx_dispatch_q, mut dst_dsq: *mut scx_dispatch_q,
) -> *mut rq {
    // SAFETY: Preserve fallback before choosing local/nonlocal movement, and
    // let deactivate_task's existing dequeue perform the remote-path unlink.
    unsafe {
        let src_rq = lupos_scx_core_task_rq(p);
        let mut dst_rq;
        lupos_scx_core_consume_bug_local_source(src_dsq);
        lupos_scx_core_consume_assert_move_dsq(src_dsq);
        lupos_scx_core_consume_assert_move_rq(src_rq);
        if (*dst_dsq).id == SCX_DSQ_LOCAL as u64 {
            dst_rq = lupos_scx_core_consume_local_rq(dst_dsq);
            if src_rq != dst_rq
                && lupos_scx_core_consume_unlikely_move_disallowed(
                    !task_can_run_on_remote_rq(sch, p, dst_rq, true),
                )
            {
                dst_dsq = find_global_dsq(sch, lupos_scx_core_consume_task_cpu(p));
                dst_rq = src_rq;
                enq_flags |= SCX_ENQ_GDSQ_FALLBACK as u64;
            }
        } else {
            dst_rq = src_rq;
        }
        if (*dst_dsq).id == SCX_DSQ_LOCAL as u64 {
            if src_rq == dst_rq {
                scx_task_unlink_from_dsq(p, src_dsq);
                scx_move_local_task_to_local_dsq(sch, p, enq_flags, src_dsq, dst_rq);
                lupos_scx_core_consume_unlock(src_dsq);
            } else {
                lupos_scx_core_consume_unlock(src_dsq);
                move_remote_task_to_local_dsq(sch, p, enq_flags, src_rq, dst_rq);
            }
        } else {
            dispatch_dequeue_locked(p, src_dsq);
            lupos_scx_core_consume_unlock(src_dsq);
            scx_dispatch_enqueue(sch, dst_rq, dst_dsq, p, 0, 0, enq_flags);
        }
        dst_rq
    }
}

/// Consume a suitable task, restarting only after a lost remote race (2652-2697).
///
/// # Safety
/// rq is locked and current execution cannot preempt or recurse; sch and the
/// nonlocal DSQ remain pinned through lock dropping. Tasks obtained under its
/// lock retain the original scheduler lifetime protection for custody rechecks.
/// The initial lockless list_empty is the native advisory read, not a promise
/// of visibility. The caller must provide visibility if success is required.
#[no_mangle]
pub unsafe extern "C" fn scx_consume_dispatch_q(
    sch: *mut scx_sched, rq: *mut rq, dsq: *mut scx_dispatch_q, enq_flags: u64,
) -> bool {
    // SAFETY: The retry label is outside the DSQ lock. A losing remote consume
    // already unlocks dsq and restores rq, so retry must neither unlock again
    // nor reuse its task/list position. Aborting bypass DSQs remains permitted.
    unsafe {
        'retry: loop {
            if lupos_scx_core_consume_list_empty(dsq) {
                return false;
            }
            lupos_scx_core_consume_lock(dsq);
            let mut p = nldsq_next_task(dsq, ptr::null_mut(), false);
            while !p.is_null() {
                let task_rq = lupos_scx_core_task_rq(p);
                if lupos_scx_core_consume_unlikely_aborting(sch)
                    && (*dsq).id != SCX_DSQ_BYPASS as u64
                {
                    break;
                }
                if rq == task_rq {
                    scx_task_unlink_from_dsq(p, dsq);
                    scx_move_local_task_to_local_dsq(sch, p, enq_flags, dsq, rq);
                    lupos_scx_core_consume_unlock(dsq);
                    return true;
                }
                if task_can_run_on_remote_rq(sch, p, rq, false) {
                    if lupos_scx_core_consume_likely_consumed(consume_remote_task(
                        sch, rq, p, enq_flags, dsq, task_rq,
                    )) {
                        return true;
                    }
                    continue 'retry;
                }
                p = nldsq_next_task(dsq, p, false);
            }
            lupos_scx_core_consume_unlock(dsq);
            return false;
        }
    }
}

/// Consume this rq's scheduler-global NUMA queue (2699-2704).
///
/// # Safety
/// sch's native pnode array and topology remain allocated and pinned. The
/// selected DSQ and caller's locked rq meet scx_consume_dispatch_q's contract.
#[no_mangle]
pub unsafe extern "C" fn scx_consume_global_dsq(sch: *mut scx_sched, rq: *mut rq) -> bool {
    // SAFETY: cpu_to_node uses the actual configured topology, not an assumed
    // flat array mapping or current CPU in place of the argument rq's CPU.
    unsafe {
        let node = lupos_scx_core_cpu_to_node(lupos_scx_core_consume_cpu_of(rq));
        let pnode = *(*sch).pnode.add(node as usize);
        scx_consume_dispatch_q(sch, rq, ptr::addr_of_mut!((*pnode).global_dsq), 0)
    }
}

/// Finish a local verdict with release/recheck and balanced rq transfer (2723-2800).
///
/// # Safety
/// rq is locked, current CPU pinned, sch/queues/p lifetime-protected. Caller
/// owns p exclusively through SCX_OPSS_DISPATCHING or the exact equivalent
/// native contract. A dequeue may race after the release; no Rust references
/// to task state escape across that point or across rq lock switches.
pub(crate) unsafe fn dispatch_to_local_dsq(
    sch: *mut scx_sched, rq: *mut rq, dst_dsq: *mut scx_dispatch_q,
    p: *mut task_struct, slice: u64, vtime: u64, enq_flags: u64,
) {
    // SAFETY: Holding CPU is stored before release of DISPATCHING. No task
    // mutation follows a lost holding check. All branches restore original rq;
    // only successful remote migration changes locked_rq to the destination.
    unsafe {
        let src_rq = lupos_scx_core_task_rq(p);
        let dst_rq = lupos_scx_core_consume_local_rq(dst_dsq);
        let mut locked_rq = rq;
        if rq == src_rq && rq == dst_rq {
            scx_dispatch_enqueue(sch, rq, dst_dsq, p, slice, vtime,
                enq_flags | SCX_ENQ_APPLY_SLICE as u64 | SCX_ENQ_CLEAR_OPSS as u64);
            return;
        }
        (*p).scx.holding_cpu = lupos_scx_core_consume_raw_cpu();
        lupos_scx_core_consume_opss_set_release(p, SCX_OPSS_NONE as c_ulong);
        if locked_rq != src_rq {
            switch_rq_lock(locked_rq, src_rq);
            locked_rq = src_rq;
        }
        if lupos_scx_core_consume_likely_dispatch_held(p)
            && !lupos_scx_core_consume_warn_dispatch_rq_changed(src_rq, p)
        {
            let mut fallback = false;
            if src_rq == dst_rq {
                (*p).scx.holding_cpu = -1;
                scx_dispatch_enqueue(sch, dst_rq, ptr::addr_of_mut!((*dst_rq).scx.local_dsq),
                    p, slice, vtime, enq_flags | SCX_ENQ_APPLY_SLICE as u64);
            } else if lupos_scx_core_consume_unlikely_dispatch_disallowed(
                !task_can_run_on_remote_rq(sch, p, dst_rq, true),
            ) {
                (*p).scx.holding_cpu = -1;
                fallback = true;
                scx_dispatch_enqueue(sch, src_rq,
                    find_global_dsq(sch, lupos_scx_core_consume_task_cpu(p)),
                    p, slice, vtime,
                    enq_flags | SCX_ENQ_APPLY_SLICE as u64 | SCX_ENQ_GDSQ_FALLBACK as u64);
            } else {
                apply_slice_vtime(p, slice, vtime, enq_flags);
                move_remote_task_to_local_dsq(sch, p, enq_flags, src_rq, dst_rq);
                locked_rq = dst_rq;
            }
            if !fallback && lupos_scx_core_class_above(
                (*p).sched_class, (*lupos_scx_core_rq_curr(dst_rq)).sched_class,
            ) {
                resched_curr(dst_rq);
            }
        }
        if locked_rq != rq {
            switch_rq_lock(locked_rq, rq);
        }
    }
}

/// Claim one buffered verdict by qseq before touching dispatch-owned fields (2822-2887).
///
/// # Safety
/// rq is locked; caller pins scheduler, buffer entry, p allocation and current
/// CPU while allowing a concurrent enqueue/dequeue. ops_state atomics provide
/// the original custody synchronization, not task lifetime by themselves.
/// QUEUEING's owner must remain able to progress while this context busy-waits.
pub(crate) unsafe fn finish_dispatch(
    sch: *mut scx_sched, rq: *mut rq, p: *mut task_struct,
    qseq_at_dispatch: c_ulong, dsq_id: u64, slice: u64, vtime: u64, enq_flags: u64,
) {
    // SAFETY: Initial read is deliberately non-acquire. The successful native
    // try_cmpxchg grants DISPATCHING before task flags/CPU are inspected. A
    // failed exchange or QUEUEING wait retries with a fresh full-width word.
    unsafe {
        loop {
            let mut opss = lupos_scx_core_consume_opss_read(p);
            match opss & SCX_OPSS_STATE_MASK as c_ulong {
                state if state == SCX_OPSS_DISPATCHING as c_ulong
                    || state == SCX_OPSS_NONE as c_ulong => return,
                state if state == SCX_OPSS_QUEUED as c_ulong => {
                    if opss & SCX_OPSS_QSEQ_MASK as c_ulong != qseq_at_dispatch {
                        return;
                    }
                    if lupos_scx_core_consume_unlikely_not_owned(sch, p) {
                        lupos_scx_core_consume_event_not_owned(sch);
                        return;
                    }
                    if lupos_scx_core_consume_likely_claim(p, &mut opss) {
                        break;
                    }
                },
                state if state == SCX_OPSS_QUEUEING as c_ulong => {
                    wait_ops_state(p, opss);
                },
                // Original switch has no default arm. Do not manufacture a
                // new success/retry/error behavior for an impossible value.
                _ => break,
            }
        }
        lupos_scx_core_consume_bug_not_queued(p);
        let dsq = find_dsq_for_dispatch(sch, rq, dsq_id, lupos_scx_core_consume_task_cpu(p));
        if (*dsq).id == SCX_DSQ_LOCAL as u64 {
            dispatch_to_local_dsq(sch, rq, dsq, p, slice, vtime, enq_flags);
        } else {
            scx_dispatch_enqueue(sch, rq, dsq, p, slice, vtime,
                enq_flags | SCX_ENQ_APPLY_SLICE as u64 | SCX_ENQ_CLEAR_OPSS as u64);
        }
    }
}

/// Drain this CPU's dispatch buffer and account its full cursor (2889-2903).
///
/// # Safety
/// CPU is pinned; sch's pcpu allocation and flexible dispatch buffer stay
/// live. The cursor is within that allocation under original dispatch limits.
/// rq is locked, and each task retains finish_dispatch's lifetime protection.
#[no_mangle]
pub unsafe extern "C" fn scx_flush_dispatch_buf(sch: *mut scx_sched, rq: *mut rq) {
    // SAFETY: Obtain the per-CPU context once, but reread its cursor each loop
    // as C does. Do not clear it early, or count only successful verdicts.
    unsafe {
        let dspc = lupos_scx_core_consume_this_dsp_ctx(sch);
        let mut u = 0u32;
        while u < (*dspc).cursor {
            let ent = lupos_scx_core_consume_buf_entry(dspc, u);
            finish_dispatch(sch, rq, (*ent).task, (*ent).qseq, (*ent).dsq_id,
                (*ent).slice, (*ent).vtime, (*ent).enq_flags);
            u = u.wrapping_add(1);
        }
        (*dspc).nr_tasks = (*dspc).nr_tasks.wrapping_add((*dspc).cursor);
        (*dspc).cursor = 0;
    }
}

/// Queue deferred balance work before clearing its pending flag (2905-2916).
///
/// # Safety
/// rq is locked and live; its embedded callback and F07-owned exact native
/// callback entry retain their scheduler lifetime. Caller pins current CPU.
pub(crate) unsafe fn maybe_queue_balance_callback(rq: *mut rq) {
    // SAFETY: Native queue_balance_callback retains typed callback metadata;
    // an already-queued callback still permits the original pending-bit clear.
    unsafe {
        lupos_scx_core_consume_assert_balance_rq(rq);
        if (*rq).scx.flags & SCX_RQ_BAL_CB_PENDING as u32 == 0 {
            return;
        }
        lupos_scx_core_consume_queue_balance(rq);
        (*rq).scx.flags &= !(SCX_RQ_BAL_CB_PENDING as u32);
    }
}
