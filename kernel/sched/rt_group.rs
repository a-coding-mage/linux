// SPDX-License-Identifier: GPL-2.0
// rt.c: bandwidth/group ownership and all CONFIG_RT_GROUP_SCHED alternatives.
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Runs the RT bandwidth timer callback.
///
/// # Safety
/// The timer must be the initialized rt_period_timer field of a live rt_bandwidth,
/// retained by the native timer subsystem for this hard-timer callback. Its owning
/// task group and CPU runqueues must remain available under the native timer lifetime
/// protocol.
unsafe extern "C" fn sched_rt_period_timer(timer: *mut b::hrtimer) -> b::hrtimer_restart {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_b = container_of!(timer, b::rt_bandwidth, rt_period_timer);
        let mut idle = 0;
        b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
        loop {
            // C assigns the u64 overrun to int before testing and passing it on.
            let overrun = b::rust_rt_hrtimer_forward_now(timer, (*rt_b).rt_period) as c_int;
            if overrun == 0 { break; }
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
            idle = do_sched_rt_period_timer(rt_b, overrun);
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
        }
        if idle != 0 { (*rt_b).rt_period_active = 0; }
        b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
        if idle != 0 { b::HRTIMER_NORESTART } else { b::HRTIMER_RESTART }
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Initializes a native RT bandwidth object and timer.
///
/// # Safety
/// The bandwidth storage must be writable, aligned and unpublished or otherwise
/// exclusively owned. No timer callback may use it while its lock and timer are
/// initialized.
pub unsafe extern "C" fn init_rt_bandwidth(rt_b: *mut b::rt_bandwidth, period: u64, runtime: u64) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        (*rt_b).rt_period = b::rust_rt_ns_to_ktime(period);
        (*rt_b).rt_runtime = runtime;
        b::rust_rt_init_bandwidth_runtime_lock(rt_b);
        b::rust_rt_hrtimer_setup(addr_of_mut!((*rt_b).rt_period_timer), Some(sched_rt_period_timer), b::RUST_RT_CLOCK_MONOTONIC as _, b::HRTIMER_MODE_REL_HARD);
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Starts an initialized RT bandwidth timer when required.
///
/// # Safety
/// The initialized bandwidth and its owning task group must remain live. The caller
/// must have the native scheduler interrupt/locking context required by raw
/// rt_runtime_lock acquisition and timer activation.
unsafe fn do_start_rt_bandwidth(rt_b: *mut b::rt_bandwidth) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
        if (*rt_b).rt_period_active == 0 {
            (*rt_b).rt_period_active = 1;
            b::rust_rt_hrtimer_forward_now(addr_of_mut!((*rt_b).rt_period_timer), b::rust_rt_ns_to_ktime(0));
            b::rust_rt_hrtimer_start_expires(addr_of_mut!((*rt_b).rt_period_timer), b::HRTIMER_MODE_ABS_PINNED_HARD);
        }
        b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Starts an initialized RT bandwidth timer when required.
///
/// # Safety
/// The initialized bandwidth and its owning task group must remain live. The caller
/// must have the native scheduler interrupt/locking context required by raw
/// rt_runtime_lock acquisition and timer activation.
unsafe fn start_rt_bandwidth(rt_b: *mut b::rt_bandwidth) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_bandwidth_enabled() || (*rt_b).rt_runtime == b::RUST_RT_RUNTIME_INF as u64 { return; }
        do_start_rt_bandwidth(rt_b);
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Cancels an RT bandwidth timer before its owner is freed.
///
/// # Safety
/// The bandwidth and initialized timer must remain live until native cancellation
/// completes. The caller must prevent new timer starts and use a context in which
/// hrtimer_cancel is permitted.
unsafe fn destroy_rt_bandwidth(rt_b: *mut b::rt_bandwidth) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::hrtimer_cancel(addr_of_mut!((*rt_b).rt_period_timer));
    }
}
#[no_mangle]
/// Unregisters the bandwidth timer of an RT task group.
///
/// # Safety
/// When RT group scheduling is enabled, the task group must remain live with its
/// initialization state intact. Teardown must prevent new timer starts, and the caller
/// must permit synchronous native timer cancellation.
pub unsafe extern "C" fn unregister_rt_sched_group(_tg: *mut b::task_group) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_group_sched_enabled() { return; }
        if !(*_tg).rt_se.is_null() { destroy_rt_bandwidth(addr_of_mut!((*_tg).rt_bandwidth)); }
    }
}
#[no_mangle]
/// Frees the RT allocations owned by a task group.
///
/// # Safety
/// When RT group scheduling is enabled, the group must own the allocations or null
/// entries recorded by allocation, including partial failure. Timer and scheduler
/// users must already be quiesced, and no other caller may free or use those
/// allocations.
pub unsafe extern "C" fn free_rt_sched_group(_tg: *mut b::task_group) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_group_sched_enabled() { return; }
        each_cpu(b::rust_rt_cpu_possible_mask(), |i| {
            if !(*_tg).rt_rq.is_null() { b::kfree((*(*_tg).rt_rq.add(i as usize)).cast()); }
            if !(*_tg).rt_se.is_null() { b::kfree((*(*_tg).rt_se.add(i as usize)).cast()); }
        });
        b::kfree((*_tg).rt_rq.cast());
        b::kfree((*_tg).rt_se.cast());
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Connects an RT task-group entity to its per-CPU runqueue.
///
/// # Safety
/// The group arrays must contain the CPU index, and rt_rq must be writable initialized
/// native storage. A non-null entity must be exclusively writable; a non-null parent
/// must be a live initialized parent entity for this CPU. None of these links may be
/// concurrently consumed during setup.
pub unsafe extern "C" fn init_tg_rt_entry(tg: *mut b::task_group, rt_rq: *mut b::rt_rq,
    rt_se: *mut b::sched_rt_entity, cpu: c_int, parent: *mut b::sched_rt_entity) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = b::rust_rt_cpu_rq(cpu);
        (*rt_rq).highest_prio.curr = b::RUST_RT_MAX_RT_PRIO as c_int - 1;
        (*rt_rq).rt_nr_boosted = 0;
        (*rt_rq).rq = rq;
        (*rt_rq).tg = tg;
        *(*tg).rt_rq.add(cpu as usize) = rt_rq;
        *(*tg).rt_se.add(cpu as usize) = rt_se;
        if rt_se.is_null() { return; }
        (*rt_se).rt_rq = if parent.is_null() { addr_of_mut!((*rq).rt) } else { (*parent).my_q };
        (*rt_se).my_q = rt_rq;
        (*rt_se).parent = parent;
        b::rust_rt_init_list_head(addr_of_mut!((*rt_se).run_list));
    }
}
#[no_mangle]
/// Allocates the per-CPU RT state for a task group.
///
/// # Safety
/// When RT group scheduling is enabled, the task group must be exclusively initialized
/// native storage and the parent's per-CPU entities must remain live. The caller must
/// be in a GFP_KERNEL allocation context and retain responsibility for
/// partial-allocation cleanup on failure.
pub unsafe extern "C" fn alloc_rt_sched_group(_tg: *mut b::task_group, _parent: *mut b::task_group) -> c_int {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_group_sched_enabled() { return 1; }
        (*_tg).rt_rq = b::rust_rt_alloc_rq_array(b::rust_rt_nr_cpu_ids());
        if (*_tg).rt_rq.is_null() { return 0; }
        (*_tg).rt_se = b::rust_rt_alloc_se_array(b::rust_rt_nr_cpu_ids());
        if (*_tg).rt_se.is_null() { return 0; }
        init_rt_bandwidth(addr_of_mut!((*_tg).rt_bandwidth), b::rust_rt_ktime_to_ns(b::rust_rt_global_period() as b::ktime_t) as u64, 0);
        let mask = b::rust_rt_cpu_possible_mask();
        let mut i = b::rust_rt_cpumask_first(mask) as c_int;
        while (i as c_uint) < b::rust_rt_nr_cpu_ids() {
            let rt_rq = b::rust_rt_alloc_rq_node(b::rust_rt_cpu_to_node(i));
            if rt_rq.is_null() { return 0; }
            let rt_se = b::rust_rt_alloc_se_node(b::rust_rt_cpu_to_node(i));
            if rt_se.is_null() { b::kfree(rt_rq.cast()); return 0; }
            init_rt_rq(rt_rq);
            (*rt_rq).rt_runtime = (*_tg).rt_bandwidth.rt_runtime;
            init_tg_rt_entry(_tg, rt_rq, rt_se, i, *(*_parent).rt_se.add(i as usize));
            i = b::rust_rt_cpumask_next(i, mask) as c_int;
        }
    }
    1
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn sched_rt_runtime(rt_rq: *mut b::rt_rq) -> u64 {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        (*rt_rq).rt_runtime
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn sched_rt_period(rt_rq: *mut b::rt_rq) -> u64 {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_ktime_to_ns((*(*rt_rq).tg).rt_bandwidth.rt_period) as u64
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Traverses live RT task-group runqueues.
///
/// # Safety
/// The runqueue or task group must be live, with initialized list links and per-CPU
/// arrays. The caller must hold RCU read protection or the native equivalent that
/// prevents task-group reclamation, and satisfy the callback's additional locking
/// requirements.
unsafe fn next_task_group(tg: *mut b::task_group) -> *mut b::task_group {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_group_sched_enabled() {
            b::rust_rt_warn_496(tg != addr_of_mut!(b::root_task_group));
            return null_mut();
        }
        let head = addr_of_mut!(b::task_groups);
        let mut node = b::rust_rt_list_next_rcu(addr_of_mut!((*tg).list));
        while node != head {
            let next = container_of!(node, b::task_group, list);
            if !b::rust_rt_task_group_is_autogroup(next) { return next; }
            node = b::rust_rt_list_next_rcu(node);
        }
        null_mut()
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Updates RT group visibility in its owning runqueue.
///
/// # Safety
/// The RT queue, task group and hierarchy must remain live, with the owning rq lock
/// held and the native scheduler interrupt state preserved. The group entity and list
/// membership must satisfy the enqueue/dequeue protocol.
unsafe fn sched_rt_rq_enqueue(rt_rq: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let donor = b::rust_rt_rq_donor(rq_of_rt_rq(rt_rq));
            let rq = rq_of_rt_rq(rt_rq);
            let cpu = b::rust_rt_cpu_of(rq);
            let rt_se = *(*(*rt_rq).tg).rt_se.add(cpu as usize);
            if (*rt_rq).rt_nr_running != 0 {
                if rt_se.is_null() { enqueue_top_rt_rq(rt_rq); }
                else if !on_rt_rq(rt_se) { enqueue_rt_entity(rt_se, 0); }
                if (*rt_rq).highest_prio.curr < (*donor).prio { b::resched_curr(rq); }
            }
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] {
            let rq = rq_of_rt_rq(rt_rq);
            if (*rt_rq).rt_nr_running == 0 { return; }
            enqueue_top_rt_rq(rt_rq);
            b::resched_curr(rq);
        }
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Updates RT group visibility in its owning runqueue.
///
/// # Safety
/// The RT queue, task group and hierarchy must remain live, with the owning rq lock
/// held and the native scheduler interrupt state preserved. The group entity and list
/// membership must satisfy the enqueue/dequeue protocol.
unsafe fn sched_rt_rq_dequeue(rt_rq: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let cpu = b::rust_rt_cpu_of(rq_of_rt_rq(rt_rq));
            let rt_se = *(*(*rt_rq).tg).rt_se.add(cpu as usize);
            if rt_se.is_null() {
                dequeue_top_rt_rq(rt_rq, (*rt_rq).rt_nr_running);
                b::rust_rt_cpufreq_update_util(rq_of_rt_rq(rt_rq), 0);
            } else if on_rt_rq(rt_se) { dequeue_rt_entity(rt_se, 0); }
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { dequeue_top_rt_rq(rt_rq, (*rt_rq).rt_nr_running); }
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_se_boosted(rt_se: *mut b::sched_rt_entity) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rr = group_rt_rq(rt_se);
        if !rr.is_null() { return (*rr).rt_nr_boosted != 0; }
        let p = rt_task_of(rt_se);
        (*p).prio != (*p).normal_prio
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn sched_rt_bandwidth(rt_rq: *mut b::rt_rq) -> *mut b::rt_bandwidth {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        addr_of_mut!((*(*rt_rq).tg).rt_bandwidth)
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Checks whether an RT runqueue should account bandwidth.
///
/// # Safety
/// The runqueue and its initialized group bandwidth must remain live. The caller must
/// provide the native synchronization required to read rt_time and rt_runtime and
/// inspect the timer state.
pub unsafe extern "C" fn sched_rt_bandwidth_account(rt_rq: *mut b::rt_rq) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_b = sched_rt_bandwidth(rt_rq);
        b::rust_rt_hrtimer_active(addr_of_mut!((*rt_b).rt_period_timer)) || (*rt_rq).rt_time < (*rt_b).rt_runtime
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Borrows available RT runtime from other CPU queues.
///
/// # Safety
/// The target RT queue, bandwidth and root-domain span must remain live. The caller
/// must retain the owning rq protection and call without holding the target
/// rt_runtime_lock, as required by the native nested bandwidth-lock protocol.
unsafe fn do_balance_runtime(rt_rq: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_b = sched_rt_bandwidth(rt_rq);
        let rd = (*rq_of_rt_rq(rt_rq)).rd;
        let span = b::rust_rt_rd_span(rd);
        let weight = b::rust_rt_cpumask_weight(span);
        b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
        let rt_period = b::rust_rt_ktime_to_ns((*rt_b).rt_period) as u64;
        let mut i = b::rust_rt_cpumask_first(span) as c_int;
        while (i as c_uint) < b::rust_rt_nr_cpu_ids() {
            let iter = sched_rt_period_rt_rq(rt_b, i);
            if iter == rt_rq {
                i = b::rust_rt_cpumask_next(i, span) as c_int;
                continue;
            }
            b::rust_rt_raw_spin_lock(addr_of_mut!((*iter).rt_runtime_lock));
            if (*iter).rt_runtime != b::RUST_RT_RUNTIME_INF as u64 {
                let mut diff = (*iter).rt_runtime.wrapping_sub((*iter).rt_time) as i64;
                if diff > 0 {
                    diff = b::rust_rt_div_u64(diff as u64, weight) as i64;
                    if (*rt_rq).rt_runtime.wrapping_add(diff as u64) > rt_period {
                        diff = rt_period.wrapping_sub((*rt_rq).rt_runtime) as i64;
                    }
                    (*iter).rt_runtime = (*iter).rt_runtime.wrapping_sub(diff as u64);
                    (*rt_rq).rt_runtime = (*rt_rq).rt_runtime.wrapping_add(diff as u64);
                    if (*rt_rq).rt_runtime == rt_period {
                        b::rust_rt_raw_spin_unlock(addr_of_mut!((*iter).rt_runtime_lock));
                        break;
                    }
                }
            }
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*iter).rt_runtime_lock));
            i = b::rust_rt_cpumask_next(i, span) as c_int;
        }
        b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
    }
}
/// Updates per-CPU RT bandwidth during runqueue hotplug.
///
/// # Safety
/// The runqueue and root domain must remain live under the native hotplug/runqueue
/// locking protocol. Task-group traversal requires RCU or equivalent lifetime
/// protection, and bandwidth objects must have initialized runtime locks.
unsafe fn __disable_runtime(_rq: *mut b::rq) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rd = (*_rq).rd;
        if b::scheduler_running == 0 { return; }
        each_rt_rq(_rq, |rr| {
            let rt_b = sched_rt_bandwidth(rr);
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
            if (*rr).rt_runtime != b::RUST_RT_RUNTIME_INF as u64 && (*rr).rt_runtime != (*rt_b).rt_runtime {
                b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
                let mut want = (*rt_b).rt_runtime.wrapping_sub((*rr).rt_runtime) as i64;
                let span = b::rust_rt_rd_span(rd);
                let mut i = b::rust_rt_cpumask_first(span) as c_int;
                while (i as c_uint) < b::rust_rt_nr_cpu_ids() {
                    let iter = sched_rt_period_rt_rq(rt_b, i);
                    if iter == rr || (*iter).rt_runtime == b::RUST_RT_RUNTIME_INF as u64 {
                        i = b::rust_rt_cpumask_next(i, span) as c_int;
                        continue;
                    }
                    b::rust_rt_raw_spin_lock(addr_of_mut!((*iter).rt_runtime_lock));
                    if want > 0 {
                        let diff = min((*iter).rt_runtime as i64, want);
                        (*iter).rt_runtime = (*iter).rt_runtime.wrapping_sub(diff as u64);
                        want = want.wrapping_sub(diff);
                    } else {
                        (*iter).rt_runtime = (*iter).rt_runtime.wrapping_sub(want as u64);
                        want = want.wrapping_sub(want);
                    }
                    b::rust_rt_raw_spin_unlock(addr_of_mut!((*iter).rt_runtime_lock));
                    if want == 0 { break; }
                    i = b::rust_rt_cpumask_next(i, span) as c_int;
                }
                b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
                b::rust_rt_warn_726(want != 0);
            }
            (*rr).rt_runtime = b::RUST_RT_RUNTIME_INF as u64;
            (*rr).rt_throttled = 0;
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
            sched_rt_rq_enqueue(rr);
        });
    }
}
/// Updates per-CPU RT bandwidth during runqueue hotplug.
///
/// # Safety
/// The runqueue and root domain must remain live under the native hotplug/runqueue
/// locking protocol. Task-group traversal requires RCU or equivalent lifetime
/// protection, and bandwidth objects must have initialized runtime locks.
unsafe fn __enable_runtime(_rq: *mut b::rq) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if b::scheduler_running == 0 { return; }
        each_rt_rq(_rq, |rr| {
            let rt_b = sched_rt_bandwidth(rr);
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_b).rt_runtime_lock));
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
            (*rr).rt_runtime = (*rt_b).rt_runtime;
            (*rr).rt_time = 0;
            (*rr).rt_throttled = 0;
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_b).rt_runtime_lock));
        });
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Balances and checks the RT runtime limit.
///
/// # Safety
/// The RT queue and its hierarchy must remain live, and the caller must hold both its
/// owning rq protection and rt_runtime_lock in native order. The function may
/// temporarily drop and reacquire rt_runtime_lock while preserving that protocol.
unsafe fn balance_runtime(rt_rq: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_feat_RT_RUNTIME_SHARE() { return; }
        if (*rt_rq).rt_time > (*rt_rq).rt_runtime {
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rt_rq).rt_runtime_lock));
            do_balance_runtime(rt_rq);
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rt_rq).rt_runtime_lock));
        }
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Replenishes RT runtime across the bandwidth span.
///
/// # Safety
/// The bandwidth must be retained by its active native timer callback, with its owning
/// group and per-CPU runqueues alive. The caller must release the bandwidth lock
/// before entry; this function takes each queue's native runtime and rq locks in the
/// original order.
unsafe fn do_sched_rt_period_timer(rt_b: *mut b::rt_bandwidth, overrun: c_int) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut idle = 1;
        let mut throttled = false;
        let mut span = sched_rt_period_mask();
        if rt_b == addr_of_mut!(b::root_task_group.rt_bandwidth) { span = b::rust_rt_cpu_online_mask(); }
        each_cpu(span, |i| {
            let mut enqueue = false;
            let rr = sched_rt_period_rt_rq(rt_b, i);
            let rq = rq_of_rt_rq(rr);
            let mut rf = MaybeUninit::<b::rq_flags>::uninit();
            b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
            if !b::rust_rt_feat_RT_RUNTIME_SHARE() && (*rr).rt_runtime != b::RUST_RT_RUNTIME_INF as u64 {
                (*rr).rt_runtime = (*rt_b).rt_runtime;
            }
            let skip = (*rr).rt_time == 0 && (*rr).rt_nr_running == 0;
            b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
            if skip { return; }
            b::rust_rt_rq_lock(rq, rf.as_mut_ptr());
            b::update_rq_clock(rq);
            if (*rr).rt_time != 0 {
                b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
                if (*rr).rt_throttled != 0 { balance_runtime(rr); }
                let runtime = (*rr).rt_runtime;
                (*rr).rt_time = (*rr).rt_time.wrapping_sub(min((*rr).rt_time, (overrun as u64).wrapping_mul(runtime)));
                if (*rr).rt_throttled != 0 && (*rr).rt_time < runtime {
                    (*rr).rt_throttled = 0;
                    enqueue = true;
                    if (*rr).rt_nr_running != 0 && b::rust_rt_rq_curr(rq) == (*rq).idle { b::rust_rt_rq_clock_cancel_skipupdate(rq); }
                }
                if (*rr).rt_time != 0 || (*rr).rt_nr_running != 0 { idle = 0; }
                b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
            } else if (*rr).rt_nr_running != 0 {
                idle = 0;
                if !rt_rq_throttled(rr) { enqueue = true; }
            }
            if (*rr).rt_throttled != 0 { throttled = true; }
            if enqueue { sched_rt_rq_enqueue(rr); }
            b::rust_rt_rq_unlock(rq, rf.as_mut_ptr());
        });
        if !throttled && (!b::rust_rt_bandwidth_enabled() || (*rt_b).rt_runtime == b::RUST_RT_RUNTIME_INF as u64) { return 1; }
        idle
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Balances and checks the RT runtime limit.
///
/// # Safety
/// The RT queue and its hierarchy must remain live, and the caller must hold both its
/// owning rq protection and rt_runtime_lock in native order. The function may
/// temporarily drop and reacquire rt_runtime_lock while preserving that protocol.
unsafe fn sched_rt_runtime_exceeded(rr: *mut b::rt_rq) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut runtime = sched_rt_runtime(rr);
        if (*rr).rt_throttled != 0 { return rt_rq_throttled(rr); }
        if runtime >= sched_rt_period(rr) { return false; }
        balance_runtime(rr);
        runtime = sched_rt_runtime(rr);
        if runtime == b::RUST_RT_RUNTIME_INF as u64 { return false; }
        if (*rr).rt_time > runtime {
            let rt_b = sched_rt_bandwidth(rr);
            if (*rt_b).rt_runtime != 0 {
                (*rr).rt_throttled = 1;
                b::rust_rt_print_throttling_once();
            } else { (*rr).rt_time = 0; }
            if rt_rq_throttled(rr) { sched_rt_rq_dequeue(rr); return true; }
        }
        false
    }
}
