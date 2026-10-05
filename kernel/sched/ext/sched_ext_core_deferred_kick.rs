// SPDX-License-Identifier: GPL-2.0
// F07 continuation of retained ext.rs against ext.c at pinned
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. No old C owner fallback.
// Configured native headers own types/constants. Native leaves stay explicit
// unqualified C runtime. No admission, build or runtime result is claimed.
compile_error!("SOURCE ONLY HOLD: sched_ext deferred/kick ownership, ABI and protection qualification incomplete");

use super::*;
use core::ptr;

/// Exact static deferred_bal_cb_workfn adapter target (995-998).
///
/// # Safety
/// Native balance-callback infrastructure supplies a live locked, unpinned rq;
/// run_deferred may drop/reacquire its lock while migrating a deferred task.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_bal_body(rq: *mut rq) {
    unsafe { run_deferred(rq); }
}

/// Exact static deferred_irq_workfn adapter target (1000-1008).
///
/// # Safety
/// irq_work is the live embedded rq.scx.deferred_irq_work. Native IRQ-work
/// lifetime/cancellation and execution context stabilize the containing rq.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_irq_body(irq_work: *mut irq_work) {
    unsafe {
        let rq = lupos_scx_core_deferred_irq_rq(irq_work);
        lupos_scx_core_raw_spin_rq_lock(rq);
        run_deferred(rq);
        scx_rq_lock_drop(rq);
        lupos_scx_core_raw_spin_rq_unlock(rq);
    }
}

/// Queue on the owning CPU, preserving remote-IPI delivery (1018-1033).
///
/// # Safety
/// rq and its embedded work remain live through asynchronous execution; the
/// caller has the original deferred-state lifetime and CPU validity protocol.
unsafe fn schedule_deferred(rq: *mut rq) {
    unsafe { lupos_scx_core_deferred_schedule(rq); }
}

/// Prefer existing wakeup/dispatch hooks before IRQ work (1042-1080).
///
/// # Safety
/// rq is live and locked. Never insert a balance callback here: dispatch can
/// drop its rq lock before it is safe for foreign rq-pin paths to observe it.
pub(crate) unsafe fn schedule_deferred_locked(rq: *mut rq) {
    unsafe {
        lupos_scx_core_deferred_assert_schedule(rq);
        if (*rq).scx.flags & SCX_RQ_IN_WAKEUP as u32 != 0 {
            return;
        }
        if (*rq).scx.flags & SCX_RQ_BAL_CB_PENDING as u32 != 0 {
            return;
        }
        if (*rq).scx.flags & SCX_RQ_IN_DISPATCH as u32 != 0 {
            (*rq).scx.flags |= SCX_RQ_BAL_CB_PENDING as u32;
            return;
        }
        schedule_deferred(rq);
    }
}

/// Publish deferred reenqueue work with the paired barriers (1082-1157).
///
/// # Safety
/// sch/dsq and their per-CPU allocations remain live under the original caller
/// RCU/scheduler protection; this CPU is pinned where this_rq is used. locked_rq
/// is NULL or the rq actually locked by the caller. Native guard leaves retain
/// the pinned counted-interrupt protocol; list lookahead remains lockless.
/// DSQ lifetime does not exclude destroy_dsq's id invalidation. The original
/// plain ID reads remain native and distinct from the later READ_ONCE consumer.
#[no_mangle]
pub unsafe extern "C" fn schedule_dsq_reenq(
    sch: *mut scx_sched, dsq: *mut scx_dispatch_q,
    reenq_flags: u64, locked_rq: *mut rq,
) {
    unsafe {
        if lupos_scx_core_deferred_unlikely_bypass(sch) {
            return;
        }
        let rq;
        if scx_shared_dsq_id_read(dsq) == SCX_DSQ_LOCAL as u64 {
            rq = lupos_scx_core_deferred_local_rq(dsq);
            if lupos_scx_core_deferred_unlikely_base_missing(sch, rq) {
                lupos_scx_core_deferred_event_reenq_denied(sch);
                return;
            }
            let drl = lupos_scx_core_deferred_local(sch, rq);
            // Paired with process_deferred_reenq_locals after pop/unlock.
            lupos_scx_core_deferred_mb();
            if lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*drl).node))
                || lupos_scx_core_deferred_flags_read_once(ptr::addr_of_mut!((*drl).flags))
                    & reenq_flags != reenq_flags
            {
                lupos_scx_core_deferred_add_local_guard(rq, drl, reenq_flags);
            }
        } else if scx_shared_dsq_id_read(dsq) & SCX_DSQ_FLAG_BUILTIN as u64 == 0 {
            rq = lupos_scx_core_deferred_this_rq();
            let dru = lupos_scx_core_deferred_user(dsq, rq);
            // Paired with process_deferred_reenq_users after pop/unlock.
            lupos_scx_core_deferred_mb();
            if lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*dru).node))
                || lupos_scx_core_deferred_flags_read_once(ptr::addr_of_mut!((*dru).flags))
                    & reenq_flags != reenq_flags
            {
                lupos_scx_core_deferred_add_user_guard(rq, dru, reenq_flags);
            }
        } else {
            lupos_scx_core_deferred_error_dsq(sch, dsq);
            return;
        }
        if rq == locked_rq {
            schedule_deferred_locked(rq);
        } else {
            schedule_deferred(rq);
        }
    }
}

/// Native counted IRQ guard continuation; never called without that guard.
///
/// # Safety
/// rq/drl live and rq.scx.deferred_reenq_lock is held by the native envelope.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_add_local_body(
    rq: *mut rq, drl: *mut scx_deferred_reenq_local, reenq_flags: u64,
) {
    unsafe {
        if lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*drl).node)) {
            lupos_scx_core_deferred_list_move_tail(
                ptr::addr_of_mut!((*drl).node), ptr::addr_of_mut!((*rq).scx.deferred_reenq_locals),
            );
        }
        lupos_scx_core_deferred_flags_write_once(
            ptr::addr_of_mut!((*drl).flags), (*drl).flags | reenq_flags,
        );
    }
}

/// Native counted IRQ guard continuation for a user DSQ request.
///
/// # Safety
/// rq/dru live and rq.scx.deferred_reenq_lock is held by the native envelope.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_add_user_body(
    rq: *mut rq, dru: *mut scx_deferred_reenq_user, reenq_flags: u64,
) {
    unsafe {
        if lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*dru).node)) {
            lupos_scx_core_deferred_list_move_tail(
                ptr::addr_of_mut!((*dru).node), ptr::addr_of_mut!((*rq).scx.deferred_reenq_users),
            );
        }
        lupos_scx_core_deferred_flags_write_once(
            ptr::addr_of_mut!((*dru).flags), (*dru).flags | reenq_flags,
        );
    }
}

/// Pop each head anew because remote dispatch can unlock rq (4305-4334).
///
/// # Safety
/// rq is locked and unpinned; deferred task/scheduler lifetime is retained by
/// the direct-dispatch custody protocol. No cached successor survives dispatch.
unsafe fn process_ddsp_deferred_locals(rq: *mut rq) {
    unsafe {
        lupos_scx_core_deferred_assert_ddsp(rq);
        loop {
            let p = lupos_scx_core_deferred_first_task(ptr::addr_of_mut!((*rq).scx.ddsp_deferred_locals));
            if p.is_null() { break; }
            let sch = lupos_scx_core_deferred_task_sched(p);
            let dsq_id = (*p).scx.ddsp_dsq_id;
            let enq_flags = (*p).scx.ddsp_enq_flags;
            let slice = (*p).scx.ddsp_slice;
            let vtime = (*p).scx.ddsp_vtime;
            lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*p).scx.dsq_list.node));
            clear_direct_dispatch(p);
            let dsq = find_dsq_for_dispatch(sch, rq, dsq_id, lupos_scx_core_deferred_task_cpu(p));
            if !lupos_scx_core_deferred_warn_ddsp(dsq) {
                dispatch_to_local_dsq(sch, rq, dsq, p, slice, vtime, enq_flags);
            }
        }
    }
}

/// First-position, protected/rescue and cap-revoke reenqueue selection (4359-4386).
///
/// # Safety
/// rq owns p under its rq lock. reenq_flags/reason are live exclusive scalar
/// outputs; flags carry scan position, including a skipped non-IMMED head.
unsafe fn local_task_should_reenq(
    rq: *mut rq, p: *mut task_struct, reenq_flags: *mut u64, reason: *mut u32,
) -> bool {
    unsafe {
        let first = *reenq_flags & SCX_REENQ_TSR_NOT_FIRST as u64 == 0;
        *reenq_flags |= SCX_REENQ_TSR_NOT_FIRST as u64;
        if lupos_scx_core_deferred_unlikely_protected(rq, p) {
            return false;
        }
        *reason = SCX_TASK_REENQ_KFUNC as u32;
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_IMMED as u32 != 0
            && (!first || *reenq_flags & SCX_REENQ_TSR_RQ_OPEN as u64 == 0)
        {
            lupos_scx_core_deferred_event_immed(p);
            *reason = SCX_TASK_REENQ_IMMED as u32;
            return true;
        }
        if *reenq_flags & SCX_REENQ_CAP_REVOKE as u64 != 0
            && lupos_scx_core_deferred_revoke(rq, p)
        {
            *reason = SCX_TASK_REENQ_CAP as u32;
            return true;
        }
        *reenq_flags & SCX_REENQ_ANY as u64 != 0
    }
}

/// Keep a self-referential native LIST_HEAD at its stable native address.
///
/// # Safety
/// sch/rq are live and rq is locked, unpinned. The synchronous continuation
/// detaches all temporary task nodes before its native stack list disappears.
pub(crate) unsafe fn reenq_local(sch: *mut scx_sched, rq: *mut rq, reenq_flags: u64) -> u32 {
    unsafe { lupos_scx_core_deferred_with_tasks(sch, rq, reenq_flags) }
}

/// Two-pass local DSQ reenqueue algorithm, then evict capless curr (4388-4464).
///
/// # Safety
/// Called only by with_tasks with an empty stable native LIST_HEAD and locked
/// rq. Same lifetimes as reenq_local. Neither task pointers nor node references
/// escape; each safe-walk successor is saved before dequeuing the current task.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_reenq_local_body(
    sch: *mut scx_sched, rq: *mut rq, mut reenq_flags: u64, tasks: *mut list_head,
) -> u32 {
    unsafe {
        let mut nr_enqueued: u32 = 0;
        lupos_scx_core_deferred_assert_reenq_local(rq);
        if lupos_scx_core_deferred_warn_tsr(reenq_flags) {
            reenq_flags &= !(__SCX_REENQ_TSR_MASK as u64);
        }
        if rq_is_open(rq, 0) {
            reenq_flags |= SCX_REENQ_TSR_RQ_OPEN as u64;
        }
        let head = ptr::addr_of_mut!((*rq).scx.local_dsq.list);
        let mut p = lupos_scx_core_deferred_first_safe_task(head);
        while !p.is_null() {
            let n = lupos_scx_core_deferred_next_task(p, head);
            let task_sch = lupos_scx_core_deferred_task_sched(p);
            let mut reason: u32 = 0;
            if (*p).migration_pending.is_null()
                && scx_is_descendant(task_sch, sch)
                && local_task_should_reenq(rq, p, &mut reenq_flags, &mut reason)
            {
                scx_dispatch_dequeue(rq, p);
                if lupos_scx_core_deferred_warn_local_reason(p) {
                    scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_REENQ_REASON_MASK as u32));
                }
                scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), reason);
                lupos_scx_core_deferred_list_add_tail(ptr::addr_of_mut!((*p).scx.dsq_list.node), tasks);
            }
            p = n;
        }
        p = lupos_scx_core_deferred_first_safe_task(tasks);
        while !p.is_null() {
            let n = lupos_scx_core_deferred_next_task(p, tasks);
            lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*p).scx.dsq_list.node));
            scx_do_enqueue_task(rq, p, SCX_ENQ_REENQ as u64, -1);
            scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_REENQ_REASON_MASK as u32));
            nr_enqueued = nr_enqueued.wrapping_add(1);
            p = n;
        }
        // rq->curr is a native plain read at each original expression. Do not
        // substitute donor, READ_ONCE, or an early common task snapshot.
        if reenq_flags & SCX_REENQ_CAP_REVOKE as u64 != 0
            && scx_shared_class_read(lupos_scx_core_rq_curr(rq)) == lupos_scx_core_ext_class()
            && lupos_scx_core_deferred_revoke(rq, lupos_scx_core_rq_curr(rq))
        {
            scx_set_task_slice(lupos_scx_core_rq_curr(rq), 0);
            lupos_scx_core_deferred_resched(rq);
        }
        nr_enqueued
    }
}

/// Remove one scheduler request under the exact native non-IRQ scoped guard.
///
/// # Safety
/// Native envelope holds deferred_reenq_lock, rq is held, flags is writable;
/// scheduler/per-CPU allocation stays live. NULL means the list was empty.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_pop_local_body(
    rq: *mut rq, flags: *mut u64,
) -> *mut scx_sched {
    unsafe {
        let drl = lupos_scx_core_deferred_first_local(rq);
        if drl.is_null() { return ptr::null_mut(); }
        let sch = lupos_scx_core_deferred_local_sched(drl);
        *flags = (*drl).flags;
        lupos_scx_core_deferred_flags_write_once(ptr::addr_of_mut!((*drl).flags), 0);
        lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*drl).node));
        sch
    }
}

/// Drain local requests, including bounded enqueue recursion (4466-4503).
///
/// # Safety
/// rq is live, locked and unpinned; all scheduler requests hold the original
/// scheduler/per-CPU lifetime protocol. Barrier runs only after guard release.
unsafe fn process_deferred_reenq_locals(rq: *mut rq) {
    unsafe {
        lupos_scx_core_deferred_assert_pop_locals(rq);
        loop {
            let mut reenq_flags: u64 = 0;
            let sch = lupos_scx_core_deferred_pop_local_guard(rq, &mut reenq_flags);
            if sch.is_null() { return; }
            lupos_scx_core_deferred_mb();
            reenq_local(sch, rq, reenq_flags);
        }
    }
}

/// Native original user-DSQ selector (4505-4509); p intentionally unused.
///
/// # Safety
/// reason is writable. p's lifetime follows the surrounding locked cursor walk.
unsafe fn user_task_should_reenq(
    _p: *mut task_struct, reenq_flags: u64, reason: *mut u32,
) -> bool {
    unsafe {
        *reason = SCX_TASK_REENQ_KFUNC as u32;
        reenq_flags & SCX_REENQ_ANY as u64 != 0
    }
}

/// Keep INIT_DSQ_LIST_CURSOR's self-link/READ_ONCE(seq) native (4511-4585).
///
/// # Safety
/// rq is locked and unpinned, dsq/scheduler stay alive while their locks drop.
/// Caller retains the original deferred-DSQ RCU lifetime; invalidity was checked.
unsafe fn reenq_user(rq: *mut rq, dsq: *mut scx_dispatch_q, reenq_flags: u64) {
    unsafe {
        // The original scheduler snapshot precedes cursor's READ_ONCE(seq).
        let sch = (*dsq).sched;
        lupos_scx_core_deferred_with_cursor(rq, dsq, reenq_flags, sch);
    }
}

/// User-DSQ cursor walk with exact lock handoff/recheck and batch release.
///
/// # Safety
/// Native wrapper supplies live stable cursor initialized for dsq with flags 0.
/// Original rq is locked; dsq lifetime spans all unlocked windows. This body
/// removes the cursor and restores original rq lock on every normal exit.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_reenq_user_body(
    rq: *mut rq, dsq: *mut scx_dispatch_q, reenq_flags: u64,
    sch: *mut scx_sched, cursor: *mut scx_dsq_list_node,
) {
    unsafe {
        let mut locked_rq = rq;
        let mut nr_enqueued: i32 = 0;
        lupos_scx_core_deferred_assert_reenq_user(rq);
        lupos_scx_core_deferred_dsq_lock(dsq);
        while lupos_scx_core_deferred_likely_not_bypass(sch) {
            let p = nldsq_cursor_next_task(cursor, dsq);
            if p.is_null() { break; }
            let mut reason: u32 = 0;
            if !user_task_should_reenq(p, reenq_flags, &mut reason) { continue; }
            let task_rq = lupos_scx_core_task_rq(p);
            if locked_rq != task_rq {
                if !locked_rq.is_null() {
                    scx_rq_lock_drop(locked_rq);
                    lupos_scx_core_raw_spin_rq_unlock(locked_rq);
                }
                if lupos_scx_core_deferred_unlikely_trylock_failed(task_rq) {
                    lupos_scx_core_deferred_dsq_unlock(dsq);
                    lupos_scx_core_raw_spin_rq_lock(task_rq);
                    lupos_scx_core_deferred_dsq_lock(dsq);
                }
                locked_rq = task_rq;
                if nldsq_cursor_lost_task(cursor, task_rq, dsq, p) { continue; }
            }
            dispatch_dequeue_locked(p, dsq);
            lupos_scx_core_deferred_dsq_unlock(dsq);
            if lupos_scx_core_deferred_warn_user_reason(p) {
                scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_REENQ_REASON_MASK as u32));
            }
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), reason);
            scx_do_enqueue_task(task_rq, p, SCX_ENQ_REENQ as u64, -1);
            scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_REENQ_REASON_MASK as u32));
            nr_enqueued = nr_enqueued.wrapping_add(1);
            if nr_enqueued % SCX_TASK_ITER_BATCH as i32 == 0 {
                scx_rq_lock_drop(locked_rq);
                lupos_scx_core_raw_spin_rq_unlock(locked_rq);
                locked_rq = ptr::null_mut();
                lupos_scx_core_cpu_relax();
            }
            lupos_scx_core_deferred_dsq_lock(dsq);
        }
        lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*cursor).node));
        lupos_scx_core_deferred_dsq_unlock(dsq);
        if locked_rq != rq {
            if !locked_rq.is_null() {
                scx_rq_lock_drop(locked_rq);
                lupos_scx_core_raw_spin_rq_unlock(locked_rq);
            }
            lupos_scx_core_raw_spin_rq_lock(rq);
        }
    }
}

/// Remove one user request under the original non-IRQ scoped guard.
///
/// # Safety
/// Native envelope holds deferred_reenq_lock; flags is writable. DSQ/per-CPU
/// lifetime remains pinned after return despite possible concurrent destroy.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_pop_user_body(
    rq: *mut rq, flags: *mut u64,
) -> *mut scx_dispatch_q {
    unsafe {
        let dru = lupos_scx_core_deferred_first_user(rq);
        if dru.is_null() { return ptr::null_mut(); }
        let dsq = lupos_scx_core_deferred_user_dsq(dru);
        *flags = (*dru).flags;
        lupos_scx_core_deferred_flags_write_once(ptr::addr_of_mut!((*dru).flags), 0);
        lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*dru).node));
        dsq
    }
}

/// Drain user requests, rechecking destruction after pop/barrier (4587-4624).
///
/// # Safety
/// rq is live, locked and unpinned; deferred DSQ lifetime outlasts the lockless
/// post-pop READ_ONCE(id), even if destroy_dsq has invalidated the queue.
unsafe fn process_deferred_reenq_users(rq: *mut rq) {
    unsafe {
        lupos_scx_core_deferred_assert_pop_users(rq);
        loop {
            let mut reenq_flags: u64 = 0;
            let dsq = lupos_scx_core_deferred_pop_user_guard(rq, &mut reenq_flags);
            if dsq.is_null() { return; }
            lupos_scx_core_deferred_mb();
            let dsq_id = lupos_scx_core_deferred_dsq_id_read_once(dsq);
            if lupos_scx_core_deferred_unlikely_invalid(dsq_id) { continue; }
            lupos_scx_core_deferred_bug_builtin(dsq_id);
            reenq_user(rq, dsq, reenq_flags);
        }
    }
}

/// Ordered deferred direct dispatch, local/user reenqueue, reject drain (4626-4637).
///
/// # Safety
/// rq is locked but unpinned, with original scheduler/DSQ/task lifetime and
/// IRQ context. Every child algorithm restores that rq's lock before returning.
pub(crate) unsafe fn run_deferred(rq: *mut rq) {
    unsafe {
        process_ddsp_deferred_locals(rq);
        if !lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*rq).scx.deferred_reenq_locals)) {
            process_deferred_reenq_locals(rq);
        }
        if !lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*rq).scx.deferred_reenq_users)) {
            process_deferred_reenq_users(rq);
        }
        lupos_scx_core_deferred_reject(rq);
    }
}

/// Drain pending kick IRQs before unpublishing each array (6241-6255).
///
/// # Safety
/// Called only in original serialized disable/allocation-failure context,
/// where native irq_work_sync and RCU retirement are legal. Possible CPU
/// topology and F00 storage stay initialized; consumers keep original RCU modes.
pub(crate) unsafe fn free_kick_syncs() {
    unsafe {
        let mut cpu: i32 = 0;
        while lupos_scx_core_deferred_next_possible(&mut cpu) {
            let ksyncs = lupos_scx_core_kick_syncs_slot(cpu);
            lupos_scx_core_deferred_sync_kick(cpu);
            let to_free = lupos_scx_core_deferred_replace_syncs(ksyncs);
            if !to_free.is_null() {
                lupos_scx_core_deferred_free_syncs(to_free);
            }
            cpu += 1;
        }
    }
}

/// Lazy nr_cpu_ids-sized per-CPU arrays with full partial-allocation unwind.
///
/// # Safety
/// Original enable serialization and sleepable GFP_KERNEL context apply.
/// F00 slots and possible topology are initialized; disable/free is serialized.
pub(crate) unsafe fn alloc_kick_syncs() -> c_int {
    unsafe {
        let mut cpu: i32 = 0;
        while lupos_scx_core_deferred_next_possible(&mut cpu) {
            let ksyncs = lupos_scx_core_kick_syncs_slot(cpu);
            lupos_scx_core_deferred_warn_syncs(ksyncs);
            let new_ksyncs = lupos_scx_core_deferred_alloc_syncs(cpu);
            if new_ksyncs.is_null() {
                free_kick_syncs();
                return -(ENOMEM as c_int);
            }
            lupos_scx_core_deferred_assign_syncs(ksyncs, new_ksyncs);
            cpu += 1;
        }
        0
    }
}

/// Skip only if a full scheduling cycle is guaranteed (8428-8443).
///
/// # Safety
/// rq is live and locked. Native plain rq->curr preserves proxy-exec layout.
unsafe fn can_skip_idle_kick(rq: *mut rq) -> bool {
    unsafe {
        lupos_scx_core_deferred_assert_idle(rq);
        !lupos_scx_core_deferred_curr_idle(rq)
            && (*rq).scx.flags & SCX_RQ_IN_DISPATCH as u32 == 0
    }
}

/// Locked authoritative capability/preemption/wait processing (8445-8504).
///
/// # Safety
/// cpu valid; pcpu/scheduler/current rq and snapshots live in IRQ-work context.
/// ksyncs has nr_cpu_ids native unsigned-long elements. CPU mask custody is
/// local-IRQ serialized. Original explicit rq IRQ-save pair remains ordinary.
/// F04 polling can overlap the snapshot store with IRQs enabled; the exact
/// native plain assignment preserves that access, without claiming race safety.
unsafe fn kick_one_cpu(
    cpu: i32, pcpu: *mut scx_sched_pcpu, this_rq: *mut rq, ksyncs: *mut c_ulong,
) -> bool {
    unsafe {
        let rq = lupos_scx_core_deferred_cpu_rq(cpu);
        let mut should_wait = false;
        let flags = lupos_scx_core_deferred_lock_irqsave(rq);
        let cur_class = scx_shared_class_read(lupos_scx_core_rq_curr(rq));
        let kickable = (lupos_scx_core_deferred_online(cpu)
            || cpu == lupos_scx_core_deferred_cpu_of(this_rq))
            && !lupos_scx_core_class_above(cur_class, lupos_scx_core_ext_class());
        if kickable
            && lupos_scx_core_deferred_missing_caps((*pcpu).sch, cpu, SCX_CAP_BASE as u64) == 0
        {
            if lupos_scx_core_deferred_mask_test(cpu, lupos_scx_core_deferred_preempt_mask(pcpu)) {
                if cur_class == lupos_scx_core_ext_class() {
                    let caps = lupos_scx_core_deferred_preempt_caps(pcpu, rq);
                    if lupos_scx_core_deferred_unlikely_preempt_missing(pcpu, cpu, caps) {
                        lupos_scx_core_deferred_event_preempt_denied(pcpu);
                    } else if lupos_scx_core_deferred_unlikely_slice_denied(
                        !scx_set_task_slice(lupos_scx_core_rq_curr(rq), 0),
                    ) {
                        lupos_scx_core_deferred_event_slice_denied(pcpu);
                    }
                }
                lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_preempt_mask(pcpu));
            }
            if lupos_scx_core_deferred_mask_test(cpu, lupos_scx_core_deferred_wait_mask(pcpu)) {
                if cur_class == lupos_scx_core_ext_class() {
                    lupos_scx_core_deferred_mask_set(cpu, lupos_scx_core_deferred_sync_mask(this_rq));
                    lupos_scx_core_deferred_kick_sync_snapshot(cpu, ksyncs, rq);
                    should_wait = true;
                }
                lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_wait_mask(pcpu));
            }
            lupos_scx_core_deferred_resched(rq);
        } else {
            if kickable { lupos_scx_core_deferred_event_kick_denied(pcpu); }
            lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_preempt_mask(pcpu));
            lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_wait_mask(pcpu));
        }
        scx_rq_lock_drop(rq);
        lupos_scx_core_deferred_unlock_irqrestore(rq, flags);
        should_wait
    }
}

/// Idle kicks preserve online-or-self hotplug exception (8506-8525).
///
/// # Safety
/// Same valid CPU/per-CPU state and native IRQ-work lifetime as kick_one_cpu.
unsafe fn kick_one_cpu_if_idle(cpu: i32, pcpu: *mut scx_sched_pcpu, this_rq: *mut rq) {
    unsafe {
        let rq = lupos_scx_core_deferred_cpu_rq(cpu);
        let flags = lupos_scx_core_deferred_lock_irqsave(rq);
        if !can_skip_idle_kick(rq)
            && (lupos_scx_core_deferred_online(cpu) || cpu == lupos_scx_core_deferred_cpu_of(this_rq))
        {
            if lupos_scx_core_deferred_likely_idle_caps(pcpu, cpu) {
                lupos_scx_core_deferred_resched(rq);
            } else {
                lupos_scx_core_deferred_event_idle_denied(pcpu);
            }
        }
        scx_rq_lock_drop(rq);
        lupos_scx_core_deferred_unlock_irqrestore(rq, flags);
    }
}

/// Exact kick_cpus_irq_workfn adapter target (8527-8574).
///
/// # Safety
/// Native IRQ work executes on the queue's owner CPU and pins its rq, scheduler
/// kick list and pcpu nodes. Original RCU-bh context protects the array after
/// the raw per-CPU snapshot; free_kick_syncs drains this work before retirement.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_deferred_kick_irq_body(_irq_work: *mut irq_work) {
    unsafe {
        let this_rq = lupos_scx_core_deferred_this_rq();
        let ksyncs_pcpu = lupos_scx_core_this_kick_syncs_snapshot();
        let mut should_wait = false;
        if lupos_scx_core_deferred_unlikely_no_syncs(ksyncs_pcpu) { return; }
        let ksyncs = lupos_scx_core_deferred_syncs_bh(ksyncs_pcpu);
        let mut pcpu = lupos_scx_core_deferred_first_kick(this_rq);
        while !pcpu.is_null() {
            let tmp = lupos_scx_core_deferred_next_kick(pcpu, this_rq);
            lupos_scx_core_deferred_list_del_init(ptr::addr_of_mut!((*pcpu).to_kick_node));
            let mut cpu: i32 = 0;
            while lupos_scx_core_deferred_next_cpu(lupos_scx_core_deferred_kick_mask(pcpu), &mut cpu) {
                // |= evaluates every target even after a prior target requested WAIT.
                should_wait |= kick_one_cpu(cpu, pcpu, this_rq, ksyncs);
                lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_kick_mask(pcpu));
                lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_idle_mask(pcpu));
                cpu += 1;
            }
            cpu = 0;
            while lupos_scx_core_deferred_next_cpu(lupos_scx_core_deferred_idle_mask(pcpu), &mut cpu) {
                kick_one_cpu_if_idle(cpu, pcpu, this_rq);
                lupos_scx_core_deferred_mask_clear(cpu, lupos_scx_core_deferred_idle_mask(pcpu));
                cpu += 1;
            }
            pcpu = tmp;
        }
        // Never wait in hard IRQ; F04 owns kick_sync_wait_bal_cb.
        if should_wait {
            lupos_scx_core_raw_spin_rq_lock(this_rq);
            (*this_rq).scx.kick_sync_pending = true;
            lupos_scx_core_deferred_resched(this_rq);
            scx_rq_lock_drop(this_rq);
            lupos_scx_core_raw_spin_rq_unlock(this_rq);
        }
    }
}

/// Publish per-scheduler kick requests, excluding NMI and PM bypass (9515-9581).
///
/// # Safety
/// Caller validated cpu and pins sch/per-CPU state with original protection.
/// Ordinary local_irq_save/restore serializes the per-CPU list, including all
/// bypass/idle-skip returns. No rq lock is nested around IRQ-work processing.
#[no_mangle]
pub unsafe extern "C" fn scx_kick_cpu(sch: *mut scx_sched, cpu: i32, flags: u64) {
    unsafe {
        if lupos_scx_core_deferred_unlikely_nmi() {
            lupos_scx_core_deferred_error_nmi(sch);
            return;
        }
        let irq_flags = lupos_scx_core_deferred_irq_save();
        let this_rq = lupos_scx_core_deferred_this_rq();
        let pcpu = lupos_scx_core_deferred_this_pcpu(sch);
        if lupos_scx_core_bypassing(sch, lupos_scx_core_deferred_cpu_of(this_rq)) {
            lupos_scx_core_deferred_irq_restore(irq_flags);
            return;
        }
        if flags & SCX_KICK_IDLE as u64 != 0 {
            let target_rq = lupos_scx_core_deferred_cpu_rq(cpu);
            if lupos_scx_core_deferred_unlikely_idle_flags(flags) {
                lupos_scx_core_deferred_error_idle_flags(sch);
            }
            if lupos_scx_core_deferred_trylock(target_rq) {
                if can_skip_idle_kick(target_rq) {
                    scx_rq_lock_drop(target_rq);
                    lupos_scx_core_raw_spin_rq_unlock(target_rq);
                    lupos_scx_core_deferred_irq_restore(irq_flags);
                    return;
                }
                scx_rq_lock_drop(target_rq);
                lupos_scx_core_raw_spin_rq_unlock(target_rq);
            }
            lupos_scx_core_deferred_mask_set(cpu, lupos_scx_core_deferred_idle_mask(pcpu));
        } else {
            lupos_scx_core_deferred_mask_set(cpu, lupos_scx_core_deferred_kick_mask(pcpu));
            if flags & SCX_KICK_PREEMPT as u64 != 0 {
                lupos_scx_core_deferred_mask_set(cpu, lupos_scx_core_deferred_preempt_mask(pcpu));
            }
            if flags & SCX_KICK_WAIT as u64 != 0 {
                lupos_scx_core_deferred_mask_set(cpu, lupos_scx_core_deferred_wait_mask(pcpu));
            }
        }
        if lupos_scx_core_deferred_list_empty(ptr::addr_of_mut!((*pcpu).to_kick_node)) {
            lupos_scx_core_deferred_list_add_tail(
                ptr::addr_of_mut!((*pcpu).to_kick_node), ptr::addr_of_mut!((*this_rq).scx.sched_pcpus_to_kick),
            );
        }
        lupos_scx_core_deferred_queue_kick(this_rq);
        lupos_scx_core_deferred_irq_restore(irq_flags);
    }
}
