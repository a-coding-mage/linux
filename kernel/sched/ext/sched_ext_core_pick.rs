// SPDX-License-Identifier: GPL-2.0
// F04 continuation, pinned ext.c:2918-3528 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves are unqualified
// runtime boundaries. Original algorithms have no old-C fallback here.
compile_error!("SOURCE ONLY HOLD: sched_ext pick/dispatch ABI, recursion, stack and protection qualification incomplete");

use super::*;
use super::inlines_header::scx_dispatch_sched;
use core::ptr;

// F00's shared field boundary keeps task flags/slice/class accesses native and
// plain. It adds no protection; mixed native/BPF access qualification stays open.

/// Dispatch once, retaining the previous runnable task and follow-up reenqueues.
///
/// # Safety
/// rq is locked, the caller pins its CPU/scheduler context and prev, and the
/// live root cannot be unpublished. Dispatch may drop/reacquire rq's lock;
/// no Rust reference or snapshot of mutable queue state spans that window.
unsafe fn dispatch_one(rq: *mut rq, prev: *mut task_struct) -> scx_dsp_verdict {
    // SAFETY: Every post-dispatch decision reloads task/queue state. The in-
    // dispatch flag encloses caps sync, cpu_acquire and all dispatch decisions.
    unsafe {
        let sch = lupos_scx_core_pick_root_protected_live();
        let cpu = lupos_scx_core_pick_cpu_of(rq);
        lupos_scx_core_pick_assert_dispatch_rq(rq);
        (*rq).scx.flags |= SCX_RQ_IN_DISPATCH as u32;
        lupos_scx_core_pick_process_sync_ecaps(rq, prev);
        if lupos_scx_core_task_ops_flags(sch) & SCX_OPS_HAS_CPU_PREEMPT as u64 != 0
            && lupos_scx_core_pick_unlikely_cpu_released(rq)
        {
            if lupos_scx_core_pick_has_cpu_acquire(sch) {
                lupos_scx_core_pick_call_cpu_acquire(sch, rq, cpu);
            }
            (*rq).scx.cpu_released = false;
        }

        // This labeled expression preserves all jumps to C's has_tasks label,
        // including which paths execute the extra-IMMED reenq check.
        let verdict = 'has_tasks: {
            if scx_shared_class_read(prev) == lupos_scx_core_ext_class() {
                update_curr_scx(rq);
                if scx_shared_flags_read(ptr::addr_of!((*prev).scx)) & SCX_TASK_QUEUED as u32 != 0
                    && scx_shared_slice_read(ptr::addr_of!((*prev).scx)) != 0
                    && !lupos_scx_core_bypassing(sch, cpu)
                {
                    break 'has_tasks SCX_DSP_PREV;
                }
            }
            if (*rq).scx.local_dsq.nr != 0 {
                break 'has_tasks SCX_DSP_LOCAL;
            }
            let verdict = scx_dispatch_sched(sch, rq, prev, false);
            if verdict != SCX_DSP_NONE {
                break 'has_tasks verdict;
            }
            if scx_shared_flags_read(ptr::addr_of!((*prev).scx)) & SCX_TASK_QUEUED as u32 != 0
                && (lupos_scx_core_task_ops_flags(sch) & SCX_OPS_ENQ_LAST as u64 == 0
                    || lupos_scx_core_bypassing(sch, cpu))
                && lupos_scx_core_pick_can_stay(rq, prev)
            {
                lupos_scx_core_pick_event_keep_last(sch);
                break 'has_tasks SCX_DSP_PREV;
            }
            (*rq).scx.flags &= !(SCX_RQ_IN_DISPATCH as u32);
            return SCX_DSP_NONE;
        };
        if lupos_scx_core_pick_unlikely_extra_immed(rq) {
            lupos_scx_core_pick_schedule_reenq(rq);
        }
        (*rq).scx.flags &= !(SCX_RQ_IN_DISPATCH as u32);
        verdict
    }
}

/// Prepare the selected task and its tick dependency (ext.c:3004-3063).
///
/// # Safety
/// The scheduler invokes this with rq locked and p live in the selected class.
/// Owner/callback lifetime and native task-argument guards must remain valid.
#[export_name = "lupos_scx_core_pick_set_next_body"]
pub unsafe extern "C" fn set_next_task_scx(rq: *mut rq, p: *mut task_struct, _first: bool) {
    // SAFETY: Dequeue precedes running; runnable clearing precedes OOB apply;
    // all tick/load operations remain after the callback's possible slice edit.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 != 0 {
            ops_dequeue(rq, p, SCX_DEQ_CORE_SCHED_EXEC as u64);
            scx_dispatch_dequeue(rq, p);
        }
        (*p).se.exec_start = lupos_scx_core_pick_clock_task(rq);
        if lupos_scx_core_pick_has_running(sch)
            && scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 != 0
        {
            lupos_scx_core_pick_call_running(sch, rq, p);
        }
        clr_task_runnable(p, true);
        apply_task_slice_oob(rq, p);
        if scx_shared_slice_read(ptr::addr_of!((*p).scx)) == SCX_SLICE_INF as u64 {
            if (*rq).scx.flags & SCX_RQ_CAN_STOP_TICK as u32 == 0 {
                (*rq).scx.flags |= SCX_RQ_CAN_STOP_TICK as u32;
                lupos_scx_core_pick_update_tick_dependency(rq);
                lupos_scx_core_pick_update_other_load_avgs(rq);
            }
        } else {
            if (*rq).scx.flags & SCX_RQ_CAN_STOP_TICK as u32 != 0 {
                (*rq).scx.flags &= !(SCX_RQ_CAN_STOP_TICK as u32);
                lupos_scx_core_pick_update_other_load_avgs(rq);
            }
            if lupos_scx_core_pick_nohz_full(rq) {
                lupos_scx_core_pick_tick_dep_set(rq);
            }
        }
    }
}

/// Class identity comparison only; class points at a live native class record.
unsafe fn preempt_reason_from_class(class: *const sched_class) -> scx_cpu_preempt_reason {
    // SAFETY: Native static class addresses and enum values are authoritative.
    unsafe {
        if class == lupos_scx_core_stop_class() {
            return SCX_CPU_PREEMPT_STOP;
        }
        if class == lupos_scx_core_pick_dl_class() {
            return SCX_CPU_PREEMPT_DL;
        }
        if class == lupos_scx_core_pick_rt_class() {
            return SCX_CPU_PREEMPT_RT;
        }
        SCX_CPU_PREEMPT_UNKNOWN
    }
}

/// Emit one CPU-release notification when switching to a higher class.
///
/// # Safety
/// rq is locked, root remains live, and next is pinned by scheduler selection.
/// HAS_CPU_PREEMPT must gate access to the cpu-only callback tail.
unsafe fn switch_class(rq: *mut rq, next: *mut task_struct) {
    // SAFETY: Test the ops flag before reading cpu_release; preserve the local
    // next_class snapshot and evaluate cpu_of(rq) inside the native callback.
    unsafe {
        let sch = lupos_scx_core_pick_root_protected_live();
        let next_class = scx_shared_class_read(next);
        if lupos_scx_core_task_ops_flags(sch) & SCX_OPS_HAS_CPU_PREEMPT as u64 == 0 {
            return;
        }
        if lupos_scx_core_class_above(lupos_scx_core_ext_class(), next_class) {
            return;
        }
        if !(*rq).scx.cpu_released {
            if lupos_scx_core_pick_has_cpu_release(sch) {
                let reason = preempt_reason_from_class(next_class);
                lupos_scx_core_pick_call_cpu_release(sch, rq, next, reason);
            }
            (*rq).scx.cpu_released = true;
        }
    }
}

/// Stop the previous task, retain a rescue or reenqueue, then switch class.
///
/// # Safety
/// rq is locked and p belongs to it; next is NULL or the lifetime-pinned next
/// task. All original task custody, callback, rescue and scheduler lifetimes
/// must survive synchronous owner calls; stopping can modify p's slice.
#[export_name = "lupos_scx_core_pick_put_prev_body"]
pub unsafe extern "C" fn put_prev_task_scx(
    rq: *mut rq, p: *mut task_struct, next: *mut task_struct,
) {
    // SAFETY: Kick release precedes accounting. Rescue checks remain at their
    // three distinct positions; no early snapshot crosses ops.stopping().
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        let mut rescue_keep = false;
        lupos_scx_core_pick_kick_sync_advance(rq);
        update_curr_scx(rq);
        if scx_shared_slice_read(ptr::addr_of!((*p).scx)) == 0 {
            if lupos_scx_core_pick_unlikely_rescue_no_slice(p, rq) {
                rescue_keep = lupos_scx_core_pick_rescue_keep(rq, p);
            }
            if !rescue_keep {
                scx_task_slice_ended(rq, p);
            }
        }
        if lupos_scx_core_pick_has_stopping(sch)
            && scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 != 0
        {
            lupos_scx_core_pick_call_stopping(sch, rq, p);
        }
        'switch_class: {
            if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 == 0 {
                break 'switch_class;
            }
            set_task_runnable(rq, p);
            if (scx_shared_slice_read(ptr::addr_of!((*p).scx)) != 0
                || lupos_scx_core_pick_unlikely_rescue_keep_local(p, rq))
                && !lupos_scx_core_bypassing(sch, lupos_scx_core_pick_cpu_of(rq))
            {
                if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_IMMED as u32 != 0 {
                    scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_REENQ_PREEMPTED as u32);
                    scx_do_enqueue_task(rq, p, SCX_ENQ_REENQ as u64, -1);
                    scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_REENQ_REASON_MASK as u32));
                } else {
                    let mut enq_flags: u64 = 0;
                    if lupos_scx_core_pick_unlikely_rescue_enq_flags(p, rq) {
                        enq_flags |= SCX_ENQ_IGNORE_CAPS as u64;
                        if !rescue_keep {
                            enq_flags |= SCX_ENQ_HEAD as u64;
                        }
                    } else {
                        enq_flags |= SCX_ENQ_HEAD as u64;
                    }
                    scx_dispatch_enqueue(
                        sch, rq, ptr::addr_of_mut!((*rq).scx.local_dsq), p, 0, 0, enq_flags,
                    );
                }
                break 'switch_class;
            }
            if !next.is_null()
                && lupos_scx_core_class_above(lupos_scx_core_ext_class(), scx_shared_class_read(next))
                && lupos_scx_core_pick_can_stay(rq, p)
            {
                lupos_scx_core_pick_warn_enq_last(rq, sch);
                scx_do_enqueue_task(rq, p, SCX_ENQ_LAST as u64, -1);
            } else {
                scx_do_enqueue_task(rq, p, 0, -1);
            }
        }
        if !next.is_null() && scx_shared_class_read(next) != lupos_scx_core_ext_class() {
            switch_class(rq, next);
        }
    }
}

/// Wait for kick acknowledgements without holding rq or disabling IRQ delivery.
///
/// # Safety
/// Native balance-callback context supplies locked rq and disabled IRQs. The
/// current CPU and the RCU-sched kick snapshot allocation stay pinned through
/// explicit unlock_irq/lock_irq windows. This is the original lifetime contract,
/// not a newly acquired RCU guard. Foreign invocations must return before access.
#[export_name = "lupos_scx_core_pick_kick_sync_wait_body"]
pub unsafe extern "C" fn kick_sync_wait_bal_cb(rq: *mut rq) {
    // SAFETY: Acquire checks pair with target release updates. Polling uses
    // READ_ONCE separately; local release progress prevents cyclic waits. A
    // fresh full cpumask scan follows every pass that actually dropped rq.
    // The IRQ-work-written ksyncs element remains a plain native operand.
    unsafe {
        if lupos_scx_core_pick_unlikely_foreign_wait(rq) {
            return;
        }
        let ksyncs = lupos_scx_core_this_kick_syncs();
        loop {
            let mut waited = false;
            let mut cpu: i32 = 0;
            // Native leaf retains find_next_bit, small_cpumask_bits, assignment
            // and limit comparison from this tree's exact for_each_cpu macro.
            while lupos_scx_core_pick_next_sync_cpu(rq, &mut cpu) {
                if cpu == lupos_scx_core_pick_cpu_of(rq)
                    || lupos_scx_core_pick_kick_sync_acquire_advanced(cpu, ksyncs)
                {
                    lupos_scx_core_pick_clear_sync_cpu(rq, cpu);
                } else {
                    scx_rq_lock_drop(rq);
                    lupos_scx_core_pick_unlock_irq(rq);
                    while lupos_scx_core_pick_kick_sync_read_once_pending(cpu, ksyncs) {
                        lupos_scx_core_pick_kick_sync_advance(rq);
                        lupos_scx_core_cpu_relax();
                    }
                    lupos_scx_core_pick_lock_irq(rq);
                    waited = true;
                }
                cpu += 1;
            }
            if !waited {
                break;
            }
        }
    }
}

/// rq's local list is initialized and protected by the caller's rq lock.
unsafe fn first_local_task(rq: *mut rq) -> *mut task_struct {
    // SAFETY: Native list_first_entry_or_null owns the embedded-member offset.
    unsafe { lupos_scx_core_pick_first_local(rq) }
}

/// rq is locked with live pin cookie rf; prev is pinned across dispatch.
unsafe fn dispatch_pick(rq: *mut rq, rf: *mut rq_flags, prev: *mut task_struct) -> scx_dsp_verdict {
    // SAFETY: Ordinary pick repins before queueing follow-up callbacks, unlike
    // the core-pick path. Kick waits remain deferred until balance-callback time.
    unsafe {
        lupos_scx_core_pick_unpin(rq, rf);
        let verdict = dispatch_one(rq, prev);
        lupos_scx_core_pick_repin(rq, rf);
        maybe_queue_balance_callback(rq);
        if lupos_scx_core_pick_unlikely_sync_pending(rq) {
            (*rq).scx.kick_sync_pending = false;
            lupos_scx_core_pick_queue_kick_sync(rq);
        }
        verdict
    }
}

/// rq and sibling selection are locked; rf/prev remain valid across dispatch.
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn dispatch_core_pick(
    rq: *mut rq, rf: *mut rq_flags, prev: *mut task_struct,
) -> scx_dsp_verdict {
    // SAFETY: In-flight dispatch is rejected before unpinning. Foreign deferred
    // work is run directly; after repinning a lock-drop sequence change forces
    // the scheduler to restart atomic sibling selection against current state.
    unsafe {
        let seq = (*rq).scx.lock_drop_seq;
        if (*rq).scx.flags & SCX_RQ_IN_DISPATCH as u32 != 0 {
            return SCX_DSP_NONE;
        }
        lupos_scx_core_pick_unpin(rq, rf);
        let verdict = dispatch_one(rq, prev);
        if lupos_scx_core_pick_cpu_of(rq) == lupos_scx_core_pick_processor_id() {
            maybe_queue_balance_callback(rq);
            if lupos_scx_core_pick_unlikely_core_sync_pending(rq) {
                (*rq).scx.kick_sync_pending = false;
                lupos_scx_core_pick_queue_kick_sync(rq);
            }
        } else if lupos_scx_core_pick_unlikely_foreign_balance(rq) {
            (*rq).scx.flags &= !(SCX_RQ_BAL_CB_PENDING as u32);
            run_deferred(rq);
        }
        lupos_scx_core_pick_repin(rq, rf);
        if (*rq).scx.lock_drop_seq != seq {
            return SCX_DSP_RETRY;
        }
        verdict
    }
}

/// Original ext.c:3350-3354 disabled-CONFIG_SCHED_CORE body, not a new stub.
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn dispatch_core_pick(
    _rq: *mut rq, _rf: *mut rq_flags, _prev: *mut task_struct,
) -> scx_dsp_verdict {
    SCX_DSP_NONE
}

/// Pick against post-dispatch state, honoring class changes unless forced.
///
/// # Safety
/// rq is locked, rf describes its live pin state, current is lifetime-pinned,
/// and the enabled class/root lifetime spans the complete scheduling pass.
unsafe fn do_pick_task_scx(rq: *mut rq, rf: *mut rq_flags, force_scx: bool) -> *mut task_struct {
    // SAFETY: curr is an original plain native union read. All mutable queue,
    // previous-task and higher-class checks occur after dispatch has returned.
    unsafe {
        let prev = lupos_scx_core_rq_curr(rq);
        lupos_scx_core_pick_kick_sync_advance(rq);
        lupos_scx_core_pick_modified_begin(rq);
        let verdict = if lupos_scx_core_sched_core_enabled(rq) {
            dispatch_core_pick(rq, rf, prev)
        } else {
            dispatch_pick(rq, rf, prev)
        };
        if verdict == SCX_DSP_RETRY {
            return lupos_scx_core_pick_retry_task();
        }
        if !force_scx && lupos_scx_core_pick_modified_above(rq) {
            return lupos_scx_core_pick_retry_task();
        }
        let p;
        if verdict == SCX_DSP_PREV {
            p = prev;
            if scx_shared_slice_read(ptr::addr_of!((*p).scx)) == 0 {
                scx_task_slice_ended(rq, p);
                refill_task_slice_dfl(lupos_scx_core_task_sched(p), p);
            }
        } else {
            p = first_local_task(rq);
            if p.is_null() {
                return ptr::null_mut();
            }
            if lupos_scx_core_pick_unlikely_zero_slice(p) && lupos_scx_core_pick_can_stay(rq, p) {
                let sch = lupos_scx_core_task_sched(p);
                if !lupos_scx_core_bypassing(sch, lupos_scx_core_pick_cpu_of(rq))
                    && !lupos_scx_core_pick_warned_zero_slice(sch)
                {
                    lupos_scx_core_pick_print_zero_slice(p);
                    lupos_scx_core_pick_mark_warned_zero_slice(sch);
                }
                refill_task_slice_dfl(sch, p);
            }
        }
        p
    }
}

/// Native scheduler-class entry; the scheduler supplies do_pick's locked context.
#[export_name = "lupos_scx_core_pick_task_body"]
pub unsafe extern "C" fn pick_task_scx(rq: *mut rq, rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The native callback preserves the exact rq/rf arguments.
    unsafe { do_pick_task_scx(rq, rf, false) }
}

/// Native dl-server callback with a live initialized entity and locked rq/rf.
#[export_name = "lupos_scx_core_pick_server_task_body"]
pub unsafe extern "C" fn ext_server_pick_task(
    dl_se: *mut sched_dl_entity, rf: *mut rq_flags,
) -> *mut task_struct {
    // SAFETY: Enabled check precedes rq use; force_scx deliberately ignores
    // higher-class modifications but never a core-sibling retry verdict.
    unsafe {
        if !lupos_scx_core_task_enabled() {
            return ptr::null_mut();
        }
        do_pick_task_scx((*dl_se).rq, rf, true)
    }
}

/// Initialize an rq's deadline server before it is used by scheduler selection.
///
/// # Safety
/// rq is live and its ext_server has exclusive initialization ownership.
#[no_mangle]
pub unsafe extern "C" fn ext_server_init(rq: *mut rq) {
    // SAFETY: The entity is initialized before installing its exact native
    // callback adapter; no callback pointer cast or alternate layout is used.
    unsafe {
        let dl_se = ptr::addr_of_mut!((*rq).ext_server);
        init_dl_entity(dl_se);
        lupos_scx_core_pick_dl_server_init(dl_se, rq);
    }
}

/// Core-sched comparison using the nearest common callback-capable ancestor.
///
/// # Safety
/// Both tasks, their scheduler owners and complete ancestor arrays are pinned
/// under the original core scheduler locks; their task/rq state is protected.
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn scx_prio_less(
    a: *const task_struct, b: *const task_struct, _in_fi: bool,
) -> bool {
    // SAFETY: The native callback keeps task_rq(a) evaluation inside its rq-
    // tracking envelope and intentionally swaps b/a without early rq snapshot.
    unsafe {
        let sch_a = lupos_scx_core_task_sched(a);
        let sch_b = lupos_scx_core_task_sched(b);
        let mut sch = ptr::null_mut();
        if sch_a == sch_b {
            if lupos_scx_core_pick_has_core_sched_before(sch_a) {
                sch = sch_a;
            }
        } else {
            let mut level = core::cmp::min((*sch_a).level, (*sch_b).level);
            while level >= 0 {
                let anc = lupos_scx_core_ancestor_at(sch_a, level);
                if anc == lupos_scx_core_ancestor_at(sch_b, level)
                    && lupos_scx_core_pick_has_core_sched_before(anc)
                {
                    sch = anc;
                    break;
                }
                level -= 1;
            }
        }
        if !sch.is_null()
            && !lupos_scx_core_bypassing(sch, lupos_scx_core_pick_task_cpu(a))
            && !lupos_scx_core_bypassing(sch, lupos_scx_core_pick_task_cpu(b))
        {
            return lupos_scx_core_pick_call_core_sched_before(sch, a, b);
        }
        let a_running = (*a).on_cpu != 0;
        let b_running = (*b).on_cpu != 0;
        if a_running != b_running {
            return a_running;
        }
        lupos_scx_core_time_after((*a).scx.runnable_at, (*b).scx.runnable_at)
    }
}

// Assisted-by: LLM
