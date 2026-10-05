// SPDX-License-Identifier: GPL-2.0
// Latched under scx_enable_mutex before rescue can execute. Rq locks protect
// runtime state. These are the original sub owner's private knob identities.
static mut scx_rescue_bw_1024: i32 = 0;
static mut scx_rescue_quantum_ns: i64 = 0;
static mut scx_rescue_sat_delta_ns: i64 = 0;
static mut scx_rescue_decay_halflife: c_ulong = 0;
static mut scx_rescue_overload_after: c_ulong = 0;

unsafe fn scx_rescue_slice_remaining(rq: *mut rq) -> i64 {
    // SAFETY: Caller holds rq lock with a live admitted rescuee. Runtime counters
    // use native u64 wrap semantics before interpreting the elapsed signed delta.
    unsafe {
        let served = (*(*rq).scx.rescue.curr).se.sum_exec_runtime
            .wrapping_sub((*rq).scx.rescue.exec_snap) as i64;
        (*rq).scx.rescue.slice.wrapping_sub(served).max(0)
    }
}

unsafe fn scx_rescue_decay_avg(pcpu: *mut scx_sched_pcpu) -> u64 {
    // SAFETY: pcpu belongs to the locked rq's CPU. Enabled rescue has a nonzero
    // latched halflife. Whole-halflife timestamp advancement matches sub.c.
    unsafe {
        let half = scx_rescue_decay_halflife;
        let n = lupos_scx_sub_jiffies64().wrapping_sub((*pcpu).rescue_avg_at) / half as u64;
        if n != 0 {
            (*pcpu).rescue_avg = if n < 64 { (*pcpu).rescue_avg >> n } else { 0 };
            (*pcpu).rescue_avg_at = (*pcpu).rescue_avg_at.wrapping_add(n.wrapping_mul(half as u64));
        }
        (*pcpu).rescue_avg
    }
}

/// Charge served rescue time, clamping tick overshoot and ending a served slice.
///
/// # Safety
/// rq is locked, its current task is its live rescuee, and rescue knobs are
/// latched for the active root. delta_exec is a scheduler execution-time delta.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_charge(rq: *mut rq, delta_exec: i64) {
    // SAFETY: Rq protection pins its current task, scheduler and per-CPU state.
    unsafe {
        lupos_scx_sub_assert_rescue_charge_rq(rq);
        let delta = delta_exec.min(scx_rescue_quantum_ns + LUPOS_SCX_SUB_TICK_NSEC as i64);
        (*rq).scx.rescue.budget = (*rq).scx.rescue.budget.wrapping_sub(delta);
        let sch = lupos_scx_sub_task_sched((*rq).curr);
        let pcpu = lupos_scx_sub_pcpu(sch, lupos_scx_sub_cpu(rq));
        (*pcpu).rescue_avg = scx_rescue_decay_avg(pcpu).wrapping_add(delta as u64);
        if scx_rescue_slice_remaining(rq) == 0 {
            scx_task_slice_ended(rq, (*rq).scx.rescue.curr);
        }
    }
}

/// End admission and discard surplus session credit when no waiter remains.
///
/// # Safety
/// Caller holds rq's lock, and its rescue state is initialized.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_end(rq: *mut rq) {
    // SAFETY: All rescue state and DSQ membership are rq-lock protected.
    unsafe {
        lupos_scx_sub_assert_rescue_end_rq(rq);
        (*rq).scx.rescue.curr = ptr::null_mut();
        if lupos_scx_sub_list_empty(ptr::addr_of!((*rq).scx.rescue.dsq.list)) {
            (*rq).scx.rescue.budget = (*rq).scx.rescue.budget.min(scx_rescue_quantum_ns);
        }
    }
}

/// Restore a preempted rescuee's unserved admitted slice when it can remain.
///
/// # Safety
/// Caller holds rq's lock and p is its current live admitted rescuee.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_keep(rq: *mut rq, p: *mut task_struct) -> bool {
    // SAFETY: Caller pins rescue.curr == p; bypass state uses the native helper.
    unsafe {
        let remaining = scx_rescue_slice_remaining(rq);
        lupos_scx_sub_assert_rescue_keep_rq(rq);
        if remaining == 0 || scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED == 0
            || lupos_scx_sub_bypassing(lupos_scx_sub_task_sched(p), lupos_scx_sub_cpu(rq))
        {
            return false;
        }
        lupos_scx_sub_set_slice(p, remaining as u64);
        true
    }
}

unsafe fn scx_rescue_accrue(rq: *mut rq) {
    // SAFETY: Rq lock serializes the bucket and native clock access; latched
    // saturation threshold avoids overflowing the multiplication on long gaps.
    unsafe {
        let active = !(*rq).scx.rescue.curr.is_null()
            || !lupos_scx_sub_list_empty(ptr::addr_of!((*rq).scx.rescue.dsq.list));
        let cap = if active { 3 * scx_rescue_quantum_ns } else { scx_rescue_quantum_ns };
        lupos_scx_sub_assert_rescue_accrue_rq(rq);
        let now = __scx_bpf_now(rq);
        let delta = now.wrapping_sub((*rq).scx.rescue.clock) as i64;
        (*rq).scx.rescue.clock = now;
        (*rq).scx.rescue.budget = if delta >= scx_rescue_sat_delta_ns {
            cap
        } else {
            cap.min((*rq).scx.rescue.budget.wrapping_add(
                delta.wrapping_mul(scx_rescue_bw_1024 as i64) >> LUPOS_SCX_SUB_CAPACITY_SHIFT))
        };
    }
}

unsafe fn scx_rescue_next_slice(rq: *mut rq) -> i64 {
    // SAFETY: Rq lock pins queue depth and positive latched quantum.
    unsafe {
        let minimum = (SCX_RESCUE_MIN_SLICE_US as i64 * LUPOS_SCX_SUB_NSEC_PER_USEC as i64)
            .max(LUPOS_SCX_SUB_TICK_NSEC as i64);
        let depth = (*rq).scx.rescue.dsq.nr.max(1);
        (scx_rescue_quantum_ns / depth as i64).max(minimum).min(scx_rescue_quantum_ns)
    }
}

unsafe fn scx_rescue_timer_arm(rq: *mut rq) {
    // SAFETY: Caller holds rq lock; timer is initialized, pinned to rq's CPU,
    // and positive rescue bandwidth bounds funding-delay division.
    unsafe {
        let timer = ptr::addr_of_mut!((*rq).scx.rescue.timer);
        let mut delay = scx_rescue_quantum_ns / 4;
        if lupos_scx_sub_timer_pending(timer) {
            return;
        }
        if (*rq).scx.rescue.curr.is_null() && (*rq).scx.rescue.budget < scx_rescue_quantum_ns {
            let deficit = scx_rescue_quantum_ns.wrapping_sub((*rq).scx.rescue.budget);
            delay = delay.max((deficit << LUPOS_SCX_SUB_CAPACITY_SHIFT) / scx_rescue_bw_1024 as i64);
        }
        (*timer).expires = lupos_scx_sub_jiffies()
            .wrapping_add(lupos_scx_sub_nsecs_jiffies(delay as u64)).wrapping_add(1);
        lupos_scx_sub_add_timer(timer, lupos_scx_sub_cpu(rq));
    }
}

unsafe fn scx_rescue_admit(rq: *mut rq, p: *mut task_struct, slice: i64) {
    // SAFETY: Rq lock held, p is pinned and off every DSQ, no other admission.
    // Native WARN_ON_ONCE retains the oracle's diagnostic before overwriting.
    unsafe {
        lupos_scx_sub_assert_rescue_admit_rq(rq);
        lupos_scx_sub_warn_rescue_current(rq);
        (*rq).scx.rescue.curr = p;
        (*rq).scx.rescue.slice = slice;
        (*rq).scx.rescue.exec_snap = (*p).se.sum_exec_runtime;
        lupos_scx_sub_set_slice(p, slice as u64);
        scx_rescue_timer_arm(rq);
    }
}

unsafe fn scx_rescue_try_admit(rq: *mut rq, p: *mut task_struct) -> bool {
    // SAFETY: Rq lock held and p is a pinned stranded task not yet on a DSQ.
    unsafe {
        scx_rescue_accrue(rq);
        if (*rq).scx.rescue.curr.is_null()
            && lupos_scx_sub_list_empty(ptr::addr_of!((*rq).scx.rescue.dsq.list))
            && (*rq).scx.rescue.budget >= scx_rescue_quantum_ns
        {
            scx_rescue_admit(rq, p, scx_rescue_quantum_ns);
            return true;
        }
        scx_rescue_timer_arm(rq);
        false
    }
}

unsafe fn scx_rescue_check_overload(rq: *mut rq) {
    // SAFETY: Rq lock and the timer's native scheduler-RCU context pin task and
    // scheduler list entries; per-CPU averages are owned by this rq's CPU.
    unsafe {
        lupos_scx_sub_assert_rescue_overload_rq(rq);
        let p = lupos_scx_sub_dsq_first(ptr::addr_of!((*rq).scx.rescue.dsq));
        if p.is_null() || lupos_scx_sub_time_before(lupos_scx_sub_jiffies(),
            (*p).scx.rescue_at.wrapping_add(scx_rescue_overload_after))
        {
            return;
        }
        if lupos_scx_sub_time_before64(lupos_scx_sub_jiffies64(),
            (*rq).scx.rescue.kill_at.wrapping_add(scx_rescue_overload_after as u64))
        {
            return;
        }
        let cpu = lupos_scx_sub_cpu(rq);
        let mut victim = ptr::null_mut();
        let mut max_avg = 0;
        let mut pos = lupos_scx_sub_all_next(ptr::null_mut());
        while !pos.is_null() {
            let avg = scx_rescue_decay_avg(lupos_scx_sub_pcpu(pos, cpu));
            if (*pos).level != 0 && avg > max_avg && lupos_scx_sub_exit_kind(pos) == SCX_EXIT_NONE as c_int {
                max_avg = avg;
                victim = pos;
            }
            pos = lupos_scx_sub_all_next(pos);
        }
        if !victim.is_null() {
            (*rq).scx.rescue.kill_at = lupos_scx_sub_jiffies64();
            lupos_scx_sub_exit_rescue(victim, cpu, max_avg, p,
                lupos_scx_sub_jiffies().wrapping_sub((*p).scx.rescue_at));
        }
    }
}

/// Run the rescue timer body under the native timer callback's rq lock.
///
/// # Safety
/// Only the native pinned timer adapter calls this with the containing live rq
/// locked irqsave. The rq's DSQs, timer and admitted tasks must be initialized.
#[export_name = "lupos_scx_sub_rescue_timer_locked"]
pub unsafe extern "C" fn scx_rescue_timerfn(rq: *mut rq) {
    // SAFETY: Native adapter keeps rq_lock_irqsave held across this synchronous
    // callback. DSQ transfers delegate only to the separate ext dispatch owner.
    unsafe {
        let mut p = (*rq).scx.rescue.curr;
        if p.is_null() && lupos_scx_sub_list_empty(ptr::addr_of!((*rq).scx.rescue.dsq.list)) {
            return;
        }
        scx_rescue_accrue(rq);
        scx_rescue_check_overload(rq);
        if p.is_null() {
            let slice = scx_rescue_next_slice(rq);
            if (*rq).scx.rescue.budget >= scx_rescue_quantum_ns {
                let dsq = ptr::addr_of_mut!((*rq).scx.rescue.dsq);
                p = lupos_scx_sub_dsq_first(dsq);
                scx_task_unlink_from_dsq(p, dsq);
                scx_rescue_admit(rq, p, slice);
                scx_move_local_task_to_local_dsq(lupos_scx_sub_task_sched(p), p,
                    SCX_ENQ_IGNORE_CAPS as u64, dsq, rq);
                if lupos_scx_sub_ext_above_current(rq) {
                    lupos_scx_sub_resched(rq);
                }
            }
        } else if !(*p).scx.dsq.is_null() && (*rq).scx.rescue.budget > 2 * scx_rescue_quantum_ns {
            lupos_scx_sub_set_slice(p, scx_rescue_slice_remaining(rq) as u64);
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_PROTECTED);
            let dsq = ptr::addr_of_mut!((*rq).scx.local_dsq);
            scx_task_unlink_from_dsq(p, dsq);
            scx_move_local_task_to_local_dsq(lupos_scx_sub_task_sched(p), p,
                (SCX_ENQ_HEAD | SCX_ENQ_PREEMPT | SCX_ENQ_IGNORE_CAPS) as u64, dsq, rq);
        }
        scx_rescue_timer_arm(rq);
    }
}

/// Flush pending rescue tasks when the CPU actually becomes inactive.
///
/// # Safety
/// Caller holds rq's lock during the CPU-offline path. Tasks and DSQs are live;
/// configured hotplug ordering protects the native cpu_active observation.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_flush(rq: *mut rq) {
    // SAFETY: Current and queued rescuees remain pinned while their DSQ links
    // are removed. Snapshotting next before transfer matches safe iteration.
    unsafe {
        lupos_scx_sub_assert_rescue_flush_rq(rq);
        if lupos_scx_sub_cpu_active(lupos_scx_sub_cpu(rq)) {
            return;
        }
        if !(*rq).scx.rescue.curr.is_null() {
            scx_task_slice_ended(rq, (*rq).scx.rescue.curr);
        }
        let dsq = ptr::addr_of_mut!((*rq).scx.rescue.dsq);
        let mut p = lupos_scx_sub_dsq_first(dsq);
        while !p.is_null() {
            let next = lupos_scx_sub_dsq_next(dsq, p);
            scx_task_unlink_from_dsq(p, dsq);
            scx_move_local_task_to_local_dsq(lupos_scx_sub_task_sched(p), p,
                SCX_ENQ_IGNORE_CAPS as u64, dsq, rq);
            p = next;
        }
        lupos_scx_sub_timer_delete(ptr::addr_of_mut!((*rq).scx.rescue.timer));
    }
}

/// Append the original rescue status line to the scheduler dump.
///
/// # Safety
/// s is a writable native seq_buf under dump serialization; rq and its current
/// rescuee are stable for the dump's existing locking/lifetime protocol.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_dump(s: *mut seq_buf, rq: *mut rq) {
    // SAFETY: Native formatting preserves the exact varargs types and format.
    unsafe { lupos_scx_sub_dump_rescue(s, rq) };
}

unsafe fn scx_rescue_check_timeout(sch: *mut scx_sched) {
    // SAFETY: sch and immutable watchdog settings are pinned by enable mutex.
    unsafe {
        if scx_rescue_bw_1024 != 0 && (*sch).watchdog_timeout <= scx_rescue_overload_after {
            lupos_scx_sub_warn_timeout(sch, scx_rescue_overload_after);
        }
    }
}

/// Latch the root's validated rescue knobs before any rescue can run.
///
/// # Safety
/// Caller holds scx_enable_mutex with rescue execution quiescent. Native root
/// setup has already range-validated the user-provided bandwidth and quantum.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_set_knobs(sch: *mut scx_sched) {
    // SAFETY: The native ops accessor uses the real anonymous-union member;
    // validated knobs keep all denominators positive and all shifts bounded.
    unsafe {
        let ops = lupos_scx_sub_ops(sch);
        let bw = if (*ops).rescue_bandwidth_ppt != 0 { (*ops).rescue_bandwidth_ppt } else { SCX_RESCUE_DFL_BW_PPT };
        let quantum = if (*ops).rescue_quantum_us != 0 { (*ops).rescue_quantum_us } else { SCX_RESCUE_DFL_QUANTUM_US };
        if (*ops).rescue_bandwidth_ppt == SCX_RESCUE_DISABLE {
            scx_rescue_bw_1024 = 0;
            return;
        }
        scx_rescue_bw_1024 = (bw as i32 * LUPOS_SCX_SUB_CAPACITY_SCALE as i32) / 1000;
        scx_rescue_quantum_ns = (quantum as i64 * LUPOS_SCX_SUB_NSEC_PER_USEC as i64)
            .max(LUPOS_SCX_SUB_TICK_NSEC as i64);
        scx_rescue_sat_delta_ns = ((4 * scx_rescue_quantum_ns + LUPOS_SCX_SUB_TICK_NSEC as i64)
            << LUPOS_SCX_SUB_CAPACITY_SHIFT) / scx_rescue_bw_1024 as i64;
        let period = (scx_rescue_quantum_ns << LUPOS_SCX_SUB_CAPACITY_SHIFT) / scx_rescue_bw_1024 as i64;
        scx_rescue_overload_after = lupos_scx_sub_nsecs_jiffies(SCX_RESCUE_OVERLOAD_MULT as u64 * period as u64)
            .max(lupos_scx_sub_msecs_jiffies(SCX_RESCUE_MIN_OVERLOAD_MS))
            .min(lupos_scx_sub_msecs_jiffies(SCX_RESCUE_MAX_OVERLOAD_MS));
        scx_rescue_decay_halflife = scx_rescue_overload_after / 4;
        if lupos_scx_sub_nsecs_jiffies(period as u64) > scx_rescue_overload_after / 2 {
            lupos_scx_sub_warn_funding(sch, period, scx_rescue_overload_after);
        }
        scx_rescue_check_timeout(sch);
    }
}

/// Initialize the rq rescue DSQ and its pinned timer.
///
/// # Safety
/// rq is private scheduler-init storage, not yet running a rescue timer. Native
/// init_dsq failure is fatal exactly as in the C oracle.
#[no_mangle]
pub unsafe extern "C" fn scx_rescue_init(rq: *mut rq) {
    // SAFETY: Native BUG_ON and timer callback metadata are preserved by leaves.
    unsafe {
        let ret = scx_init_dsq(ptr::addr_of_mut!((*rq).scx.rescue.dsq), SCX_DSQ_RESCUE, ptr::null_mut());
        lupos_scx_sub_bug_init_dsq(ret);
        lupos_scx_sub_timer_setup(rq);
        (*rq).scx.rescue.kill_at = lupos_scx_sub_jiffies64();
    }
}

/// Resolve an insertion to local, rescue, or reject DSQ with cap checks.
///
/// # Safety
/// Caller holds rq's lock, pins sch and p, and supplies a uniquely writable
/// flags slot. CID tables and the scheduler hierarchy are live and protected.
#[no_mangle]
pub unsafe extern "C" fn scx_resolve_local_dsq(sch: *mut scx_sched, rq: *mut rq,
    p: *mut task_struct, enq_flags: *mut u64) -> *mut scx_dispatch_q
{
    // SAFETY: All field/flag mutation is under rq lock; native helpers preserve
    // remote-activation cap resolution, migration checks, and event accounting.
    unsafe {
        let local = ptr::addr_of_mut!((*rq).scx.local_dsq);
        if !lupos_scx_sub_has_subs() {
            return local;
        }
        let cpu = lupos_scx_sub_cpu(rq);
        let cid = lupos_scx_sub_cpu_cid(cpu);
        let asch = if (*rq).scx.remote_activate_sch.is_null() { sch } else { (*rq).scx.remote_activate_sch };
        let mut needed = lupos_scx_sub_caps_for_enq(*enq_flags);
        if *enq_flags & SCX_ENQ_PREEMPT as u64 != 0 {
            needed |= lupos_scx_sub_caps_for_preempt(asch, rq, *enq_flags);
        }
        let missing = lupos_scx_sub_missing_caps(asch, cpu, needed);
        if lupos_scx_sub_likely_resolve_caps(missing == 0) {
            return local;
        }
        if lupos_scx_sub_unlikely_forced_admit(!lupos_scx_sub_rq_online(rq)
            || lupos_scx_sub_migration_disabled(p) || !(*p).migration_pending.is_null())
        {
            lupos_scx_sub_event_forced(sch);
            *enq_flags &= !(SCX_ENQ_PREEMPT as u64);
            return local;
        }
        *enq_flags &= !((SCX_ENQ_IMMED | SCX_ENQ_PREEMPT | SCX_ENQ_HEAD |
            SCX_ENQ_APPLY_SLICE | SCX_ENQ_SLICE_DFL) as u64);
        scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !SCX_TASK_IMMED);
        if *enq_flags & SCX_ENQ_RESCUE as u64 != 0 && lupos_scx_sub_likely_rescue_enabled(scx_rescue_bw_1024 != 0) {
            lupos_scx_sub_event_rescue(sch);
            if scx_rescue_try_admit(rq, p) {
                return local;
            }
            (*p).scx.rescue_at = lupos_scx_sub_jiffies();
            return ptr::addr_of_mut!((*rq).scx.rescue.dsq);
        }
        (*p).scx.reenq_reason_caps = missing;
        (*p).scx.reenq_reason_cid = cid;
        ptr::addr_of_mut!((*rq).scx.reject_dsq)
    }
}

/// Record cap loss requiring reenqueuing of a task on the local DSQ.
///
/// # Safety
/// Caller holds rq lock and pins the queued task p plus its scheduler/CID state.
#[no_mangle]
pub unsafe extern "C" fn scx_task_reenq_on_cap_revoke(rq: *mut rq, p: *mut task_struct) -> bool {
    // SAFETY: Native migration/rescue helpers preserve exemptions and locking.
    unsafe {
        if lupos_scx_sub_migration_disabled(p) || p == lupos_scx_sub_rescuee(rq) {
            return false;
        }
        let cpu = lupos_scx_sub_cpu(rq);
        let missing = lupos_scx_sub_missing_caps(lupos_scx_sub_task_sched(p), cpu,
            lupos_scx_sub_caps_for_task(p));
        if lupos_scx_sub_likely_reenq_caps(missing == 0) {
            return false;
        }
        (*p).scx.reenq_reason_caps = missing;
        (*p).scx.reenq_reason_cid = lupos_scx_sub_cpu_cid(cpu);
        true
    }
}

/// Drain rejected tasks without revisiting tasks rejected again this round.
///
/// # Safety
/// Caller holds rq's lock; reject DSQ and all queued tasks are initialized and
/// stable. The enqueue owner may re-reject tasks but may not free their storage.
#[no_mangle]
pub unsafe extern "C" fn scx_reenq_reject(rq: *mut rq) {
    // SAFETY: Native synchronous adapter supplies an initialized automatic list
    // whose address never escapes the callback below.
    unsafe {
        lupos_scx_sub_assert_reenq_reject_rq(rq);
        if !lupos_scx_sub_has_subs() || lupos_scx_sub_list_empty(ptr::addr_of!((*rq).scx.reject_dsq.list)) {
            return;
        }
        lupos_scx_sub_with_reject_list(rq);
    }
}

/// Native automatic-list continuation of the reject-drain owner.
///
/// # Safety
/// Only the native adapter calls this synchronously under rq lock. tasks is an
/// empty initialized native list, remains live throughout, and must end empty.
#[export_name = "lupos_scx_sub_reject_with_list"]
pub unsafe extern "C" fn scx_reenq_reject_list(rq: *mut rq, tasks: *mut list_head) {
    // SAFETY: Snapshot next before unlink; native list adapters use real task
    // member offsets. The private batch prevents same-round rejection loops.
    unsafe {
        let reject = ptr::addr_of_mut!((*rq).scx.reject_dsq);
        let mut p = lupos_scx_sub_dsq_first(reject);
        while !p.is_null() {
            let next = lupos_scx_sub_dsq_next(reject, p);
            if !lupos_scx_sub_warn_migration_pending(p) {
                scx_dispatch_dequeue(rq, p);
                if lupos_scx_sub_warn_reenq_flags(p) {
                    scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !SCX_TASK_REENQ_REASON_MASK);
                }
                scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_REENQ_CAP);
                lupos_scx_sub_task_list_add(p, tasks);
            }
            p = next;
        }
        p = lupos_scx_sub_task_list_first(tasks);
        while !p.is_null() {
            let next = lupos_scx_sub_task_list_next(tasks, p);
            lupos_scx_sub_task_list_del(p);
            scx_do_enqueue_task(rq, p, SCX_ENQ_REENQ as u64, -1);
            scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !SCX_TASK_REENQ_REASON_MASK);
            p = next;
        }
    }
}
