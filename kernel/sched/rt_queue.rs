// SPDX-License-Identifier: GPL-2.0
// rt.c runnable accounting, hierarchical queues, statistics and class dispatch.
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn need_pull_rt_task(rq: *mut b::rq, prev: *mut b::task_struct) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_rq_online(rq) && (*rq).rt.highest_prio.curr > (*prev).prio
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn rt_overloaded(rq: *mut b::rq) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_atomic_read(addr_of!((*(*rq).rd).rto_count))
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn rt_set_overload(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_rq_online(rq) { return; }
        b::rust_rt_cpumask_set_cpu((*rq).cpu as _, b::rust_rt_rd_rto_mask((*rq).rd));
        b::rust_rt_smp_wmb();
        b::rust_rt_atomic_inc(addr_of_mut!((*(*rq).rd).rto_count));
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn rt_clear_overload(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_rq_online(rq) { return; }
        b::rust_rt_atomic_dec(addr_of_mut!((*(*rq).rd).rto_count));
        b::rust_rt_cpumask_clear_cpu((*rq).cpu as _, b::rust_rt_rd_rto_mask((*rq).rd));
    }
}
/// Checks the native pushable-list head for an RT runqueue.
///
/// # Safety
/// The initialized runqueue must remain live. This is only the native optimistic
/// list-head snapshot; callers must acquire the rq lock before following task entries
/// or acting on a selected task.
unsafe fn has_pushable_tasks(rq: *mut b::rq) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        !b::rust_rt_plist_head_empty(addr_of!((*rq).rt.pushable_tasks))
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn rt_queue_push_tasks(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !has_pushable_tasks(rq) { return; }
        b::rust_rt_queue_balance_callback(rq, b::rust_rt_push_head((*rq).cpu), Some(push_rt_tasks));
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn rt_queue_pull_task(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_queue_balance_callback(rq, b::rust_rt_pull_head((*rq).cpu), Some(pull_rt_task));
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn enqueue_pushable_task(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::plist_del(addr_of_mut!((*p).pushable_tasks), addr_of_mut!((*rq).rt.pushable_tasks));
        b::rust_rt_plist_node_init(addr_of_mut!((*p).pushable_tasks), (*p).prio);
        b::plist_add(addr_of_mut!((*p).pushable_tasks), addr_of_mut!((*rq).rt.pushable_tasks));
        if (*p).prio < (*rq).rt.highest_prio.next { (*rq).rt.highest_prio.next = (*p).prio; }
        if !(*rq).rt.overloaded { rt_set_overload(rq); (*rq).rt.overloaded = true; }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dequeue_pushable_task(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::plist_del(addr_of_mut!((*p).pushable_tasks), addr_of_mut!((*rq).rt.pushable_tasks));
        if has_pushable_tasks(rq) {
            let first = b::rust_rt_plist_first_task(addr_of_mut!((*rq).rt.pushable_tasks));
            (*rq).rt.highest_prio.next = (*first).prio;
        } else {
            (*rq).rt.highest_prio.next = b::RUST_RT_MAX_RT_PRIO as c_int - 1;
            if (*rq).rt.overloaded { rt_clear_overload(rq); (*rq).rt.overloaded = false; }
        }
    }
}
/// Compares an RT task's effective clamps with CPU capacity.
///
/// # Safety
/// When utilization clamping is enabled, the task must remain live and the CPU must be
/// a valid native CPU index. The caller must preserve the scheduler's lifetime and
/// synchronization protocol for the effective clamp values.
unsafe extern "C" fn rt_task_fits_capacity(_p: *mut b::task_struct, _cpu: c_int) -> bool {
    #[cfg(CONFIG_UCLAMP_TASK)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_sched_asym_cpucap_active() { return true; }
        let min_cap = b::rust_rt_uclamp_eff_value(_p, b::UCLAMP_MIN) as c_uint;
        let max_cap = b::rust_rt_uclamp_eff_value(_p, b::UCLAMP_MAX) as c_uint;
        let cpu_cap = b::rust_rt_arch_scale_cpu_capacity(_cpu) as c_uint;
        cpu_cap >= min(min_cap, max_cap)
    }
    #[cfg(not(CONFIG_UCLAMP_TASK))] { true }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn rust_rt_update_curr_rt(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        update_curr_rt(rq);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_curr_rt(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let donor = (*rq).donor;
        if (*donor).sched_class != addr_of!(b::rt_sched_class) { return; }
        let delta_exec = b::update_curr_common(rq);
        if delta_exec <= 0 { return; }
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let mut rt_se = addr_of_mut!((*donor).rt);
            if !b::rust_rt_bandwidth_enabled() { return; }
            while !rt_se.is_null() {
                let rr = rt_rq_of_se(rt_se);
                if sched_rt_runtime(rr) != b::RUST_RT_RUNTIME_INF as u64 {
                    b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
                    (*rr).rt_time = (*rr).rt_time.wrapping_add(delta_exec as u64);
                    let exceeded = sched_rt_runtime_exceeded(rr);
                    if exceeded { b::resched_curr(rq); }
                    b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
                    if exceeded { do_start_rt_bandwidth(sched_rt_bandwidth(rr)); }
                }
                rt_se = rt_parent(rt_se);
            }
        }
    }
}
/// Updates runnable accounting for a top-level RT queue.
///
/// # Safety
/// The queue must be the actual rt field of its live owning rq, with the rq lock held
/// and interrupts in the native scheduler state. Runnable counts and list state must
/// satisfy the native enqueue/dequeue accounting protocol.
unsafe fn dequeue_top_rt_rq(rr: *mut b::rt_rq, count: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_rq(rr);
        b::rust_rt_bug_1015(addr_of_mut!((*rq).rt) != rr);
        if (*rr).rt_queued == 0 { return; }
        b::rust_rt_bug_1020((*rq).nr_running == 0);
        b::rust_rt_sub_nr_running(rq, count);
        (*rr).rt_queued = 0;
    }
}
/// Updates runnable accounting for a top-level RT queue.
///
/// # Safety
/// The queue must be the actual rt field of its live owning rq, with the rq lock held
/// and interrupts in the native scheduler state. Runnable counts and list state must
/// satisfy the native enqueue/dequeue accounting protocol.
unsafe fn enqueue_top_rt_rq(rr: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_rq(rr);
        b::rust_rt_bug_1032(addr_of_mut!((*rq).rt) != rr);
        if (*rr).rt_queued != 0 { return; }
        if rt_rq_throttled(rr) { return; }
        if (*rr).rt_nr_running != 0 {
            b::rust_rt_add_nr_running(rq, (*rr).rt_nr_running);
            (*rr).rt_queued = 1;
        }
        b::rust_rt_cpufreq_update_util(rq, 0);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn inc_rt_prio_smp(rr: *mut b::rt_rq, prio: c_int, prev_prio: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_rq(rr);
        #[cfg(CONFIG_RT_GROUP_SCHED)] if addr_of_mut!((*rq).rt) != rr { return; }
        if b::rust_rt_rq_online(rq) && prio < prev_prio { b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, prio); }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dec_rt_prio_smp(rr: *mut b::rt_rq, _prio: c_int, prev_prio: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_rq(rr);
        #[cfg(CONFIG_RT_GROUP_SCHED)] if addr_of_mut!((*rq).rt) != rr { return; }
        if b::rust_rt_rq_online(rq) && (*rr).highest_prio.curr != prev_prio {
            b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, (*rr).highest_prio.curr);
        }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn inc_rt_prio(rr: *mut b::rt_rq, prio: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let prev_prio = (*rr).highest_prio.curr;
        if prio < prev_prio { (*rr).highest_prio.curr = prio; }
        inc_rt_prio_smp(rr, prio, prev_prio);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dec_rt_prio(rr: *mut b::rt_rq, prio: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let prev_prio = (*rr).highest_prio.curr;
        if (*rr).rt_nr_running != 0 {
            b::rust_rt_warn_1097(prio < prev_prio);
            if prio == prev_prio { (*rr).highest_prio.curr = b::rust_rt_sched_find_first_bit(addr_of!((*rr).active.bitmap).cast::<c_ulong>()) as c_int; }
        } else { (*rr).highest_prio.curr = b::RUST_RT_MAX_RT_PRIO as c_int - 1; }
        dec_rt_prio_smp(rr, prio, prev_prio);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn inc_rt_group(_rt_se: *mut b::sched_rt_entity, _rr: *mut b::rt_rq) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if rt_se_boosted(_rt_se) { (*_rr).rt_nr_boosted = (*_rr).rt_nr_boosted.wrapping_add(1); }
        start_rt_bandwidth(addr_of_mut!((*(*_rr).tg).rt_bandwidth));
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dec_rt_group(_rt_se: *mut b::sched_rt_entity, _rr: *mut b::rt_rq) {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if rt_se_boosted(_rt_se) { (*_rr).rt_nr_boosted = (*_rr).rt_nr_boosted.wrapping_sub(1); }
        b::rust_rt_warn_1134((*_rr).rt_nr_running == 0 && (*_rr).rt_nr_boosted != 0);
    }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_se_nr_running(rt_se: *mut b::sched_rt_entity) -> c_uint {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let group_rq = group_rt_rq(rt_se);
        if !group_rq.is_null() { (*group_rq).rt_nr_running } else { 1 }
    }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_se_rr_nr_running(rt_se: *mut b::sched_rt_entity) -> c_uint {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let group_rq = group_rt_rq(rt_se);
        if !group_rq.is_null() { return (*group_rq).rr_nr_running; }
        ((*rt_task_of(rt_se)).policy == b::RUST_RT_SCHED_RR as c_uint) as c_uint
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn inc_rt_tasks(rt_se: *mut b::sched_rt_entity, rr: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let prio = rt_se_prio(rt_se);
        b::rust_rt_warn_1179(!b::rust_rt_rt_prio(prio));
        (*rr).rt_nr_running = (*rr).rt_nr_running.wrapping_add(rt_se_nr_running(rt_se));
        (*rr).rr_nr_running = (*rr).rr_nr_running.wrapping_add(rt_se_rr_nr_running(rt_se));
        inc_rt_prio(rr, prio);
        inc_rt_group(rt_se, rr);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dec_rt_tasks(rt_se: *mut b::sched_rt_entity, rr: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_warn_1190(!b::rust_rt_rt_prio(rt_se_prio(rt_se)));
        b::rust_rt_warn_1191((*rr).rt_nr_running == 0);
        (*rr).rt_nr_running = (*rr).rt_nr_running.wrapping_sub(rt_se_nr_running(rt_se));
        (*rr).rr_nr_running = (*rr).rr_nr_running.wrapping_sub(rt_se_rr_nr_running(rt_se));
        dec_rt_prio(rr, rt_se_prio(rt_se));
        dec_rt_group(rt_se, rr);
    }
}
fn move_entity(flags: c_uint) -> bool {
    (flags & (b::RUST_RT_DEQUEUE_SAVE as c_uint | b::RUST_RT_DEQUEUE_MOVE as c_uint)) != b::RUST_RT_DEQUEUE_SAVE as c_uint
}
/// Removes an RT entity from its priority list.
///
/// # Safety
/// The caller must hold the owning rq lock. The entity must be linked in the supplied
/// live priority array, its current priority must index that array, and its run_list
/// must be a real entity node rather than a sentinel.
unsafe fn __delist_rt_entity(rt_se: *mut b::sched_rt_entity, array: *mut b::rt_prio_array) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_list_del_init(addr_of_mut!((*rt_se).run_list));
        if b::rust_rt_list_empty(addr_of!((*array).queue).cast::<b::list_head>().add(rt_se_prio(rt_se) as usize)) {
            b::rust_rt_clear_bit(rt_se_prio(rt_se), addr_of_mut!((*array).bitmap).cast::<c_ulong>());
        }
        (*rt_se).on_list = 0;
    }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn __schedstats_from_rt_se(rt_se: *mut b::sched_rt_entity) -> *mut b::sched_statistics {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !rt_entity_is_task(rt_se) { return null_mut(); }
        addr_of_mut!((*rt_task_of(rt_se)).stats)
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_stats_wait_start_rt(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_schedstat_enabled() { return; }
        let p = if rt_entity_is_task(rt_se) { rt_task_of(rt_se) } else { null_mut() };
        let stats = __schedstats_from_rt_se(rt_se);
        if stats.is_null() { return; }
        b::rust_rt_update_stats_wait_start(rq_of_rt_rq(rr), p, stats);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_stats_enqueue_sleeper_rt(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_schedstat_enabled() { return; }
        let p = if rt_entity_is_task(rt_se) { rt_task_of(rt_se) } else { null_mut() };
        let stats = __schedstats_from_rt_se(rt_se);
        if stats.is_null() { return; }
        b::rust_rt_update_stats_enqueue_sleeper(rq_of_rt_rq(rr), p, stats);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_stats_enqueue_rt(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity, flags: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_schedstat_enabled() { return; }
        if flags & b::RUST_RT_ENQUEUE_WAKEUP as c_int != 0 { update_stats_enqueue_sleeper_rt(rr, rt_se); }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_stats_wait_end_rt(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_schedstat_enabled() { return; }
        let p = if rt_entity_is_task(rt_se) { rt_task_of(rt_se) } else { null_mut() };
        let stats = __schedstats_from_rt_se(rt_se);
        if stats.is_null() { return; }
        b::rust_rt_update_stats_wait_end(rq_of_rt_rq(rr), p, stats);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn update_stats_dequeue_rt(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity, flags: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut p = null_mut();
        let rq = rq_of_rt_rq(rr);
        if !b::rust_rt_schedstat_enabled() { return; }
        if rt_entity_is_task(rt_se) {
            p = rt_task_of(rt_se);
            if p != (*rq).curr { update_stats_wait_end_rt(rr, rt_se); }
        }
        if flags & b::RUST_RT_DEQUEUE_SLEEP as c_int != 0 && !p.is_null() {
            let state = b::rust_rt_read_task_state(p);
            if state & b::RUST_RT_TASK_INTERRUPTIBLE as c_uint != 0 {
                b::rust_rt_schedstat_sleep_start(p, b::rust_rt_rq_clock(rq_of_rt_rq(rr)));
            }
            if state & b::RUST_RT_TASK_UNINTERRUPTIBLE as c_uint != 0 {
                b::rust_rt_schedstat_block_start(p, b::rust_rt_rq_clock(rq_of_rt_rq(rr)));
            }
        }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn __enqueue_rt_entity(rt_se: *mut b::sched_rt_entity, flags: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rr = rt_rq_of_se(rt_se);
        let array = addr_of_mut!((*rr).active);
        let group_rq = group_rt_rq(rt_se);
        let queue = addr_of_mut!((*array).queue).cast::<b::list_head>().add(rt_se_prio(rt_se) as usize);
        if !group_rq.is_null() && (rt_rq_throttled(group_rq) || (*group_rq).rt_nr_running == 0) {
            if (*rt_se).on_list != 0 { __delist_rt_entity(rt_se, array); }
            return;
        }
        if move_entity(flags) {
            b::rust_rt_warn_1351((*rt_se).on_list != 0);
            if flags & b::RUST_RT_ENQUEUE_HEAD as c_uint != 0 { b::rust_rt_list_add(addr_of_mut!((*rt_se).run_list), queue); }
            else { b::rust_rt_list_add_tail(addr_of_mut!((*rt_se).run_list), queue); }
            b::rust_rt_set_bit(rt_se_prio(rt_se), addr_of_mut!((*array).bitmap).cast::<c_ulong>());
            (*rt_se).on_list = 1;
        }
        (*rt_se).on_rq = 1;
        inc_rt_tasks(rt_se, rr);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn __dequeue_rt_entity(rt_se: *mut b::sched_rt_entity, flags: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rr = rt_rq_of_se(rt_se);
        let array = addr_of_mut!((*rr).active);
        if move_entity(flags) {
            b::rust_rt_warn_1371((*rt_se).on_list == 0);
            __delist_rt_entity(rt_se, array);
        }
        (*rt_se).on_rq = 0;
        dec_rt_tasks(rt_se, rr);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dequeue_rt_stack(mut rt_se: *mut b::sched_rt_entity, flags: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut back = null_mut();
        while !rt_se.is_null() {
            (*rt_se).back = back;
            back = rt_se;
            rt_se = rt_parent(rt_se);
        }
        let rt_nr_running = (*rt_rq_of_se(back)).rt_nr_running;
        rt_se = back;
        while !rt_se.is_null() {
            if on_rt_rq(rt_se) { __dequeue_rt_entity(rt_se, flags); }
            rt_se = (*rt_se).back;
        }
        dequeue_top_rt_rq(rt_rq_of_se(back), rt_nr_running);
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn enqueue_rt_entity(mut rt_se: *mut b::sched_rt_entity, flags: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_se(rt_se);
        update_stats_enqueue_rt(rt_rq_of_se(rt_se), rt_se, flags as c_int);
        dequeue_rt_stack(rt_se, flags);
        while !rt_se.is_null() { __enqueue_rt_entity(rt_se, flags); rt_se = rt_parent(rt_se); }
        enqueue_top_rt_rq(addr_of_mut!((*rq).rt));
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn dequeue_rt_entity(mut rt_se: *mut b::sched_rt_entity, flags: c_uint) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rq = rq_of_rt_se(rt_se);
        update_stats_dequeue_rt(rt_rq_of_se(rt_se), rt_se, flags as c_int);
        dequeue_rt_stack(rt_se, flags);
        while !rt_se.is_null() {
            let rr = group_rt_rq(rt_se);
            if !rr.is_null() && (*rr).rt_nr_running != 0 { __enqueue_rt_entity(rt_se, flags); }
            rt_se = rt_parent(rt_se);
        }
        enqueue_top_rt_rq(addr_of_mut!((*rq).rt));
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn enqueue_task_rt(rq: *mut b::rq, p: *mut b::task_struct, flags: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_se = addr_of_mut!((*p).rt);
        if flags & b::RUST_RT_ENQUEUE_WAKEUP as c_int != 0 { (*rt_se).timeout = 0; }
        b::rust_rt_check_schedstat_required();
        update_stats_wait_start_rt(rt_rq_of_se(rt_se), rt_se);
        enqueue_rt_entity(rt_se, flags as c_uint);
        if b::rust_rt_task_is_blocked(p) { return; }
        if !b::rust_rt_task_current(rq, p) && (*p).nr_cpus_allowed > 1 { enqueue_pushable_task(rq, p); }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn dequeue_task_rt(rq: *mut b::rq, p: *mut b::task_struct, flags: c_int) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_se = addr_of_mut!((*p).rt);
        update_curr_rt(rq);
        dequeue_rt_entity(rt_se, flags as c_uint);
        dequeue_pushable_task(rq, p);
        true
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn requeue_rt_entity(rr: *mut b::rt_rq, rt_se: *mut b::sched_rt_entity, head: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if on_rt_rq(rt_se) {
            let queue = addr_of_mut!((*rr).active.queue).cast::<b::list_head>().add(rt_se_prio(rt_se) as usize);
            if head != 0 { b::rust_rt_list_move(addr_of_mut!((*rt_se).run_list), queue); }
            else { b::rust_rt_list_move_tail(addr_of_mut!((*rt_se).run_list), queue); }
        }
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn requeue_task_rt(_rq: *mut b::rq, p: *mut b::task_struct, head: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut rt_se = addr_of_mut!((*p).rt);
        while !rt_se.is_null() { requeue_rt_entity(rt_rq_of_se(rt_se), rt_se, head); rt_se = rt_parent(rt_se); }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn yield_task_rt(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        requeue_task_rt(rq, (*rq).donor, 0);
    }
}
#[no_mangle]
/// Selects a candidate CPU under the native RT placement protocol.
///
/// # Safety
/// The task must remain live and its affinity and scheduler state must be protected as
/// required by native wakeup or migration selection. CPU IDs must identify initialized
/// runqueues and the current CPU must remain pinned while the per-CPU temporary mask
/// is used. RCU protects topology lifetime; optimistic remote-field reads still
/// require native/Rust memory-model qualification before admission.
pub unsafe extern "C" fn select_task_rq_rt(p: *mut b::task_struct, mut cpu: c_int, flags: c_int) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if flags & (b::RUST_RT_WF_TTWU as c_int | b::RUST_RT_WF_FORK as c_int) == 0 { return cpu; }
        let rq = b::rust_rt_cpu_rq(cpu);
        b::rust_rt_rcu_read_lock();
        let curr = b::rust_rt_read_rq_curr(rq);
        let donor = b::rust_rt_read_rq_donor(rq);
        let test = !curr.is_null() && b::rust_rt_rt_task(donor) && ((*curr).nr_cpus_allowed < 2 || (*donor).prio <= (*p).prio);
        if test || !rt_task_fits_capacity(p, cpu) {
            let target = find_lowest_rq(p);
            if !(!test && target != -1 && !rt_task_fits_capacity(p, target)) {
                if target != -1 && (*p).prio < (*b::rust_rt_cpu_rq(target)).rt.highest_prio.curr { cpu = target; }
            }
        }
        b::rust_rt_rcu_read_unlock();
        cpu
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn check_preempt_equal_prio(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if (*(*rq).curr).nr_cpus_allowed == 1 || b::cpupri_find(addr_of_mut!((*(*rq).rd).cpupri), (*rq).donor, null_mut()) == 0 { return; }
        if (*p).nr_cpus_allowed != 1 && b::cpupri_find(addr_of_mut!((*(*rq).rd).cpupri), p, null_mut()) != 0 { return; }
        requeue_task_rt(rq, p, 1);
        b::resched_curr(rq);
    }
}
#[no_mangle]
/// Balances RT work while preserving the scheduler rq pin.
///
/// # Safety
/// The rq lock must be held with the matching initialized rq_flags pin, preemption and
/// interrupts disabled as in the scheduler pick path. The runqueue and current task
/// must remain live while native balancing may drop and reacquire rq locks.
pub unsafe extern "C" fn balance_rt(rq: *mut b::rq, rf: *mut b::rq_flags) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let p = (*rq).donor;
        if !on_rt_rq(addr_of_mut!((*p).rt)) && need_pull_rt_task(rq, p) {
            b::rust_rt_rq_unpin_lock(rq, rf);
            pull_rt_task(rq);
            b::rust_rt_rq_repin_lock(rq, rf);
        }
        (b::rust_rt_sched_stop_runnable(rq) || b::rust_rt_sched_dl_runnable(rq) || b::rust_rt_sched_rt_runnable(rq)) as c_int
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn wakeup_preempt_rt(rq: *mut b::rq, p: *mut b::task_struct, _flags: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let donor = (*rq).donor;
        if (*p).sched_class != addr_of!(b::rt_sched_class) || (*donor).sched_class != addr_of!(b::rt_sched_class) { return; }
        if (*p).prio < (*donor).prio { b::resched_curr(rq); return; }
        if (*p).prio == (*donor).prio && !b::rust_rt_test_tsk_need_resched((*rq).curr) { check_preempt_equal_prio(rq, p); }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn set_next_task_rt(rq: *mut b::rq, p: *mut b::task_struct, first: bool) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_se = addr_of_mut!((*p).rt);
        let rr = addr_of_mut!((*rq).rt);
        (*p).se.exec_start = b::rust_rt_rq_clock_task(rq);
        if on_rt_rq(addr_of_mut!((*p).rt)) { update_stats_wait_end_rt(rr, rt_se); }
        dequeue_pushable_task(rq, p);
        if !first { return; }
        if (*(*rq).donor).sched_class != addr_of!(b::rt_sched_class) { b::rust_rt_update_rt_rq_load_avg(b::rust_rt_rq_clock_pelt(rq), rq, 0); }
        rt_queue_push_tasks(rq);
    }
}
/// Selects the first entity from a runnable RT priority array.
///
/// # Safety
/// The RT queue must be live and protected by its owning rq lock. Its bitmap must
/// describe initialized priority lists with the native delimiter, and non-empty
/// entries must be actual sched_rt_entity nodes.
unsafe fn pick_next_rt_entity(rr: *mut b::rt_rq) -> *mut b::sched_rt_entity {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let array = addr_of_mut!((*rr).active);
        let idx = b::rust_rt_sched_find_first_bit(addr_of!((*array).bitmap).cast::<c_ulong>()) as c_int;
        b::rust_rt_bug_1691(idx >= b::RUST_RT_MAX_RT_PRIO as c_int);
        let queue = addr_of_mut!((*array).queue).cast::<b::list_head>().add(idx as usize);
        if b::rust_rt_warn_1694(b::rust_rt_list_empty(queue)) { return null_mut(); }
        container_of!((*queue).next, b::sched_rt_entity, run_list)
    }
}
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn _pick_next_task_rt(rq: *mut b::rq) -> *mut b::task_struct {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut rr = addr_of_mut!((*rq).rt);
        loop {
            let rt_se = pick_next_rt_entity(rr);
            if rt_se.is_null() { return null_mut(); }
            rr = group_rt_rq(rt_se);
            if rr.is_null() { return rt_task_of(rt_se); }
        }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn pick_task_rt(rq: *mut b::rq, _rf: *mut b::rq_flags) -> *mut b::task_struct {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_sched_rt_runnable(rq) { return null_mut(); }
        _pick_next_task_rt(rq)
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn put_prev_task_rt(rq: *mut b::rq, p: *mut b::task_struct, _next: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_se = addr_of_mut!((*p).rt);
        let rr = addr_of_mut!((*rq).rt);
        if on_rt_rq(addr_of_mut!((*p).rt)) { update_stats_wait_start_rt(rr, rt_se); }
        update_curr_rt(rq);
        b::rust_rt_update_rt_rq_load_avg(b::rust_rt_rq_clock_pelt(rq), rq, 1);
        if b::rust_rt_task_is_blocked(p) { return; }
        if on_rt_rq(addr_of_mut!((*p).rt)) && (*p).nr_cpus_allowed > 1 { enqueue_pushable_task(rq, p); }
    }
}
