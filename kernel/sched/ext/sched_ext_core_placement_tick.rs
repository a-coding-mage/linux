// SPDX-License-Identifier: GPL-2.0
// F05 addition to the retained ext.rs, pinned ext.c:3530-3807,4639-4674,
// 6257-6275 at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native leaves remain explicit unqualified C runtime. No idle algorithm,
// storage replica, fabricated layout or enabled-feature fallback is supplied.
compile_error!("SOURCE ONLY HOLD: sched_ext placement/tick ABI, race and protection qualification incomplete");

use super::*;
use core::ptr;

/// Original select_task_rq_scx, including its pi-lock/direct-dispatch protocol.
///
/// # Safety
/// p is a live scheduler task under the native select-task-rq calling protocol:
/// p's pi_lock pins its scheduling state, and this CPU stays pinned across the
/// per-CPU direct-dispatch slot and SCX task envelope. This does not assert
/// exclusivity over BPF-writable fields or strengthen supported native races;
/// their Rust memory-model qualification remains blocked by the source hold.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_select_task_rq_body(
    p: *mut task_struct, prev_cpu: c_int, wake_flags: c_int,
) -> c_int {
    // SAFETY: Task scheduler lookup intentionally precedes the WF_EXEC test.
    // Every original callback/CPU conversion and once-only diagnostic remains
    // a native leaf with its original native task argument envelope.
    unsafe {
        let sch = lupos_scx_core_pt_task_sched(p);
        if lupos_scx_core_pt_unlikely_exec(wake_flags) {
            return prev_cpu;
        }
        let bypassing = lupos_scx_core_bypassing(sch, lupos_scx_core_pt_task_cpu(p));
        if lupos_scx_core_pt_likely_has_select(sch) && !bypassing {
            let ddsp_taskp = lupos_scx_core_this_direct_dispatch_task();
            lupos_scx_core_pt_warn_direct_dispatch(ddsp_taskp);
            *ddsp_taskp = p;
            (*lupos_scx_core_pt_this_rq()).scx.in_select_cpu = true;
            let selected = lupos_scx_core_pt_call_select(sch, p, prev_cpu, wake_flags);
            let cpu = lupos_scx_core_pt_cpu_ret(sch, selected);
            (*lupos_scx_core_pt_this_rq()).scx.in_select_cpu = false;
            (*p).scx.selected_cpu = cpu;
            *ddsp_taskp = ptr::null_mut();
            if scx_cpu_valid(sch, cpu, b"from ops.select_cpu()\0".as_ptr().cast()) {
                return cpu;
            }
            return prev_cpu;
        }
        if bypassing {
            lupos_scx_core_pt_event_bypass(sch);
            (*p).scx.selected_cpu = prev_cpu;
            return prev_cpu;
        }
        let mut cpu = lupos_scx_core_pt_select_default(p, prev_cpu, wake_flags);
        if cpu >= 0 {
            lupos_scx_core_pt_event_refill(sch);
            (*p).scx.ddsp_slice = lupos_scx_core_pt_slice_dfl_read_once(sch);
            (*p).scx.ddsp_enq_flags = SCX_ENQ_SLICE_DFL as u64;
            (*p).scx.ddsp_dsq_id = SCX_DSQ_LOCAL as u64;
        } else {
            cpu = prev_cpu;
        }
        (*p).scx.selected_cpu = cpu;
        cpu
    }
}

/// Original task_woken_scx; p is deliberately unused.
///
/// # Safety
/// rq has the native task-woken rq-lock/IRQ context required by run_deferred.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_task_woken_body(
    rq: *mut rq, _p: *mut task_struct,
) {
    // SAFETY: F07 owns and must supply the deferred-work algorithm.
    unsafe { run_deferred(rq); }
}

/// Notify affinity changes using the effective cpus_ptr, after common handling.
///
/// # Safety
/// p/ac satisfy the scheduler affinity callback's original lock/lifetime
/// contract. The common affinity operation and effective mask remain live for
/// this synchronous callback; it may differ from the configured cpus_mask.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_set_cpus_allowed_body(
    p: *mut task_struct, ac: *mut affinity_context,
) {
    // SAFETY: Retain the initial scheduler snapshot, post-common death check,
    // then the native F00 task/cmask envelope with effective mask const cast.
    unsafe {
        let sch = lupos_scx_core_pt_task_sched(p);
        lupos_scx_core_pt_set_allowed_common(p, ac);
        if task_dead_and_done(p) {
            return;
        }
        if lupos_scx_core_pt_has_set_cpumask(sch) {
            scx_call_op_set_cpumask(
                sch, lupos_scx_core_pt_task_rq(p), p, (*p).cpus_ptr as *mut cpumask,
            );
        }
    }
}

/// Hotplug dispatch stays valid even before the global enabled key is set.
///
/// # Safety
/// Caller holds the native CPU-hotplug serialization which excludes root
/// updates and CID table retirement. NULL CID tables after enable failure are
/// supported. No added RCU lock or false table-non-NULL promise is required.
unsafe fn handle_hotplug(rq: *mut rq, online: bool) {
    // SAFETY: Native checked lookups keep original lockdep conditions. The
    // enabled key only gates idle topology updates, never hotplug callbacks.
    unsafe {
        let sch = lupos_scx_core_pt_root_protected();
        let cpu = lupos_scx_core_pt_cpu_of(rq);
        let mut cpu_or_cid = cpu;
        lupos_scx_core_hotplug_seq_inc();
        if sch.is_null() {
            return;
        }
        if lupos_scx_core_pt_enabled() {
            lupos_scx_core_pt_update_topology(sch);
        }
        if online {
            lupos_scx_core_pt_online_ecaps(rq);
        } else {
            lupos_scx_core_pt_offline_ecaps(rq);
        }
        if lupos_scx_core_is_cid_type() {
            let tbl = lupos_scx_core_pt_hotplug_cid_table();
            if !tbl.is_null() {
                cpu_or_cid = *tbl.add(cpu as usize) as i32;
            }
        }
        if online && lupos_scx_core_pt_has_cpu_online(sch) {
            lupos_scx_core_pt_call_cpu_online(sch, cpu_or_cid);
        } else if !online && lupos_scx_core_pt_has_cpu_offline(sch) {
            lupos_scx_core_pt_call_cpu_offline(sch, cpu_or_cid);
        } else {
            lupos_scx_core_pt_exit_hotplug(sch, cpu, online);
        }
    }
}

/// # Safety
/// rq satisfies handle_hotplug's original CPU-hotplug lock/lifetime protocol.
#[no_mangle]
pub unsafe extern "C" fn scx_rq_activate(rq: *mut rq) {
    // SAFETY: Preserve the activation hook's true argument.
    unsafe { handle_hotplug(rq, true); }
}

/// # Safety
/// rq satisfies handle_hotplug's original CPU-hotplug lock/lifetime protocol.
#[no_mangle]
pub unsafe extern "C" fn scx_rq_deactivate(rq: *mut rq) {
    // SAFETY: Preserve the deactivation hook's false argument.
    unsafe { handle_hotplug(rq, false); }
}

/// # Safety
/// rq is live in the native rq-online callback's scheduler lock context.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_rq_online_body(rq: *mut rq) {
    // SAFETY: Original plain rq flag update; no additional callback is implied.
    unsafe { (*rq).scx.flags |= SCX_RQ_ONLINE as u32; }
}

/// # Safety
/// rq is live in the native rq-offline callback's scheduler lock context.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_rq_offline_body(rq: *mut rq) {
    // SAFETY: Rescue flush is ordered after the original online-flag clear.
    unsafe {
        (*rq).scx.flags &= !(SCX_RQ_ONLINE as u32);
        lupos_scx_core_pt_rescue_flush(rq);
    }
}

/// Locked continuation for check_rq_for_timeouts; native owns rq_flags/IRQ pair.
///
/// # Safety
/// Only call inside lupos_scx_core_pt_check_timeouts_locked's rq lock lifetime.
/// The original runnable-list and scheduler/DSQ lifetime rules apply. A DSQ
/// READ_ONCE does not establish extra exclusion from supported DSQ races.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_check_timeouts_locked_body(rq: *mut rq) -> bool {
    // SAFETY: Never dereference an empty-list sentinel. Root-null and stalled
    // early returns still pass through the native rq_unlock_irqrestore.
    unsafe {
        let root = lupos_scx_core_pt_timeout_root_bh();
        if root.is_null() {
            return false;
        }
        let head = ptr::addr_of_mut!((*rq).scx.runnable_list);
        let mut node = (*head).next;
        while node != head {
            let p = lupos_scx_core_pt_runnable_task(node);
            let mut sch = lupos_scx_core_pt_task_sched(p);
            let last_runnable = (*p).scx.runnable_at;
            if lupos_scx_core_pt_unlikely_task_timeout(sch, last_runnable) {
                let dsq = lupos_scx_core_pt_task_dsq_read_once(p);
                let dur_ms = lupos_scx_core_pt_duration_ms(last_runnable);
                if !dsq.is_null() && !(*dsq).sched.is_null() && scx_shared_dsq_id_read(dsq) != SCX_DSQ_LOCAL as u64 {
                    sch = (*dsq).sched;
                }
                lupos_scx_core_pt_exit_task_stall(sch, rq, p, dur_ms);
                return true;
            }
            node = (*node).next;
        }
        false
    }
}

/// # Safety
/// rq is a live native CPU rq in the watchdog worker's process context.
unsafe fn check_rq_for_timeouts(rq: *mut rq) -> bool {
    // SAFETY: Native keeps original explicit rq_lock_irqsave/rq_unlock_irqrestore.
    unsafe { lupos_scx_core_pt_check_timeouts_locked(rq) }
}

/// Original watchdog work, preserving live online-mask scans and one-pass break.
///
/// # Safety
/// work is the original delayed_work's embedded work_struct, invoked by its
/// native workqueue callback. No extra hotplug read lock is claimed or added.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_watchdog_work_body(work: *mut work_struct) {
    // SAFETY: Native CPU iterator condition preserves the configured UP path,
    // live bitmap sampling, int CPU cursor and small_cpumask_bits domain.
    unsafe {
        lupos_scx_core_watchdog_timestamp_write_once(lupos_scx_core_pt_jiffies());
        let mut cpu: c_int = 0;
        while lupos_scx_core_pt_online_cpu_condition(&mut cpu) {
            if check_rq_for_timeouts(lupos_scx_core_pt_cpu_rq(cpu)) {
                break;
            }
            lupos_scx_core_pt_cond_resched();
            cpu += 1;
        }
        let intv = lupos_scx_core_watchdog_interval_read_once();
        if intv < lupos_scx_core_pt_ulong_max() {
            lupos_scx_core_pt_queue_watchdog(work, intv);
        }
    }
}

/// Watchdog liveness and other-class load update in the native scheduler tick.
///
/// # Safety
/// rq is live with the caller's original tick IRQ/BH/RCU protection. The root
/// check must retain its native rcu_dereference_bh diagnostic; concurrent
/// watchdog writes remain native READ_ONCE/WRITE_ONCE accesses.
#[no_mangle]
pub unsafe extern "C" fn scx_tick(rq: *mut rq) {
    // SAFETY: Load-average updates occur even after requesting a stall exit;
    // they are skipped only for disabled or NULL-root entry states.
    unsafe {
        if !lupos_scx_core_pt_enabled() {
            return;
        }
        let root = lupos_scx_core_pt_tick_root_bh();
        if root.is_null() {
            return;
        }
        let last_check = lupos_scx_core_watchdog_timestamp_read_once();
        if lupos_scx_core_pt_unlikely_watchdog_timeout(root, last_check) {
            let dur_ms = lupos_scx_core_pt_duration_ms(last_check);
            lupos_scx_core_pt_exit_watchdog_stall(root, dur_ms);
        }
        lupos_scx_core_pt_update_other_load_avgs(rq);
    }
}

/// Original task_tick_scx, with update/accounting before bypass/tick decisions.
///
/// # Safety
/// rq/curr satisfy the native class-tick rq-lock and lifetime protocol; queued
/// is deliberately unused. A rejected protected-slice write is not forced.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_task_tick_body(
    rq: *mut rq, curr: *mut task_struct, _queued: c_int,
) {
    // SAFETY: Only the native macro leaf invokes the BPF tick; the final slice
    // read occurs after the callback and does not substitute callback state.
    unsafe {
        let sch = lupos_scx_core_pt_task_sched(curr);
        update_curr_scx(rq);
        if lupos_scx_core_bypassing(sch, lupos_scx_core_pt_cpu_of(rq)) {
            scx_set_task_slice(curr, 0);
        } else if lupos_scx_core_pt_has_tick(sch) {
            lupos_scx_core_pt_call_tick(sch, rq, curr);
        }
        if scx_shared_slice_read(ptr::addr_of!((*curr).scx)) == 0 {
            lupos_scx_core_pt_resched_curr(rq);
        }
    }
}

/// # Safety
/// rq is live under the original nohz tick-dependency rq-lock context. Its
/// curr may still be a dequeued EXT task, a supported state checked below.
#[cfg(CONFIG_NO_HZ_FULL)]
#[no_mangle]
pub unsafe extern "C" fn scx_can_stop_tick(rq: *mut rq) -> bool {
    // SAFETY: rq->curr is a plain native load, not donor/READ_ONCE. Scheduler
    // lookup precedes even the class check as in the original source.
    unsafe {
        let p = lupos_scx_core_pt_rq_curr_plain(rq);
        let sch = lupos_scx_core_pt_task_sched(p);
        if !lupos_scx_core_pt_is_ext_task(p) {
            return true;
        }
        if (*rq).scx.nr_running == 0 {
            return true;
        }
        if lupos_scx_core_bypassing(sch, lupos_scx_core_pt_cpu_of(rq)) {
            return false;
        }
        if lupos_scx_core_pt_unlikely_rescuee(p, rq) {
            return false;
        }
        (*rq).scx.flags & SCX_RQ_CAN_STOP_TICK as u32 != 0
    }
}

/// Compute max(min(interval, timeout / 2), 1) for each live scheduler.
///
/// # Safety
/// Only the native refresh_interval_rcu envelope invokes this continuation;
/// its explicit RCU lifetime covers the entire traversal. Concurrent RCU list
/// changes remain allowed; no scheduler list snapshot or exclusion is assumed.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_pt_refresh_interval_rcu_body() -> c_ulong {
    // SAFETY: Native entry/next preserve list_entry_rcu READ_ONCE and initial
    // list lockdep metadata. Sentinel checks precede every object dereference.
    // watchdog_timeout is deliberately plain here, not READ_ONCE as in ticks.
    unsafe {
        let mut intv = lupos_scx_core_pt_ulong_max();
        let head = lupos_scx_core_sched_all();
        let mut sch = lupos_scx_core_pt_sched_first_rcu(head);
        while !lupos_scx_core_pt_sched_is_head(sch, head) {
            intv = core::cmp::max(core::cmp::min(intv, (*sch).watchdog_timeout / 2), 1);
            sch = lupos_scx_core_pt_sched_next_rcu(sch);
        }
        intv
    }
}

/// Recompute the shared timer after scheduler link/unlink, including sync cancel.
///
/// # Safety
/// Caller has the original link/unlink process context permitting synchronous
/// delayed-work cancellation. Shared stores retain native WRITE_ONCE behavior;
/// concurrent work/tick readers remain supported rather than excluded.
pub(crate) unsafe fn refresh_watchdog() {
    // SAFETY: RCU is released before timestamp/interval publication and before
    // either workqueue operation. Empty-list ULONG_MAX selects synchronous cancel.
    unsafe {
        let intv = lupos_scx_core_pt_refresh_interval_rcu();
        lupos_scx_core_watchdog_timestamp_write_once(lupos_scx_core_pt_jiffies());
        lupos_scx_core_watchdog_interval_write_once(intv);
        if intv < lupos_scx_core_pt_ulong_max() {
            lupos_scx_core_pt_mod_watchdog(intv);
        } else {
            lupos_scx_core_pt_cancel_watchdog();
        }
    }
}
