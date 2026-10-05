// SPDX-License-Identifier: GPL-2.0
// Newly reconstructed owner behavior from the unchanged deadline.c oracle.
// Scheduler entry points retain the native runqueue locking preconditions.
unsafe fn inc_dl_deadline(dl_rq: *mut b::dl_rq, deadline: u64) {
    // SAFETY: The dl runqueue and its domain indices are live under the owning rq lock.
    unsafe {
        let rq = rq_of_dl_rq(dl_rq);
        if (*dl_rq).earliest_dl.curr == 0
            || b::rust_dl_time_before(deadline, (*dl_rq).earliest_dl.curr) {
            if (*dl_rq).earliest_dl.curr == 0 {
                b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, b::CPUPRI_HIGHER as c_int);
            }
            (*dl_rq).earliest_dl.curr = deadline;
            b::cpudl_set(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, deadline);
        }
    }
}
unsafe fn dec_dl_deadline(dl_rq: *mut b::dl_rq, _deadline: u64) {
    // SAFETY: The owning rq lock protects the already-updated tree and runnable count.
    unsafe {
        let rq = rq_of_dl_rq(dl_rq);
        if (*dl_rq).dl_nr_running == 0 {
            (*dl_rq).earliest_dl.curr = 0;
            (*dl_rq).earliest_dl.next = 0;
            b::cpudl_clear(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, (*rq).online != 0);
            b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, (*rq).rt.highest_prio.curr);
        } else {
            let left = b::rust_dl_rb_first_cached(addr_of!((*dl_rq).root));
            let entry = b::rust_dl_entity_from_node(left);
            (*dl_rq).earliest_dl.curr = (*entry).deadline;
            b::cpudl_set(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, (*entry).deadline);
        }
    }
}
unsafe fn inc_dl_tasks(se: *mut b::sched_dl_entity, dl_rq: *mut b::dl_rq) {
    // SAFETY: The live entity has just been inserted under its owning rq lock.
    unsafe {
        (*dl_rq).dl_nr_running = (*dl_rq).dl_nr_running.wrapping_add(1);
        if !b::rust_dl_server(se) { b::rust_dl_add_nr_running(rq_of_dl_rq(dl_rq), 1); }
        inc_dl_deadline(dl_rq, (*se).deadline);
    }
}
unsafe fn dec_dl_tasks(se: *mut b::sched_dl_entity, dl_rq: *mut b::dl_rq) {
    // SAFETY: The live entity has been removed under its owning rq lock and was counted runnable.
    unsafe {
        b::rust_dl_warn_dec_tasks((*dl_rq).dl_nr_running == 0);
        (*dl_rq).dl_nr_running = (*dl_rq).dl_nr_running.wrapping_sub(1);
        if !b::rust_dl_server(se) { b::rust_dl_sub_nr_running(rq_of_dl_rq(dl_rq), 1); }
        dec_dl_deadline(dl_rq, (*se).deadline);
    }
}
unsafe extern "C" fn __dl_less(a: *mut b::rb_node, other: *const b::rb_node) -> bool {
    // SAFETY: Both nodes belong to live deadline entities; rb_add_cached holds the rq lock.
    unsafe {
        b::rust_dl_time_before((*b::rust_dl_entity_from_node(a)).deadline,
            (*b::rust_dl_entity_from_node(other.cast_mut())).deadline)
    }
}
unsafe fn __schedstats_from_dl_se(se: *mut b::sched_dl_entity) -> *mut b::sched_statistics {
    // SAFETY: The live entity is rq-locked; task conversion occurs only after excluding servers.
    unsafe {
        if !b::rust_dl_schedstat_enabled() || b::rust_dl_server(se) { return core::ptr::null_mut(); }
        addr_of_mut!((*b::rust_dl_task_of(se)).stats)
    }
}
unsafe fn update_stats_wait_start_dl(dl_rq: *mut b::dl_rq, se: *mut b::sched_dl_entity) {
    // SAFETY: The entity and dl runqueue are live and rq-locked for wait-time accounting.
    unsafe {
        let stats = __schedstats_from_dl_se(se);
        if !stats.is_null() { b::rust_dl_stats_wait_start(rq_of_dl_rq(dl_rq), b::rust_dl_task_of(se), stats); }
    }
}
unsafe fn update_stats_wait_end_dl(dl_rq: *mut b::dl_rq, se: *mut b::sched_dl_entity) {
    // SAFETY: The entity and dl runqueue are live and rq-locked for wait-time accounting.
    unsafe {
        let stats = __schedstats_from_dl_se(se);
        if !stats.is_null() { b::rust_dl_stats_wait_end(rq_of_dl_rq(dl_rq), b::rust_dl_task_of(se), stats); }
    }
}
unsafe fn update_stats_enqueue_sleeper_dl(dl_rq: *mut b::dl_rq, se: *mut b::sched_dl_entity) {
    // SAFETY: The entity and dl runqueue are live and rq-locked while sleep statistics are updated.
    unsafe {
        let stats = __schedstats_from_dl_se(se);
        if !stats.is_null() { b::rust_dl_stats_enqueue_sleeper(rq_of_dl_rq(dl_rq), b::rust_dl_task_of(se), stats); }
    }
}
unsafe fn update_stats_enqueue_dl(dl_rq: *mut b::dl_rq, se: *mut b::sched_dl_entity, flags: c_int) {
    // SAFETY: The enqueue caller supplies the locked runqueue and its live entity.
    unsafe {
        if !b::rust_dl_schedstat_enabled() { return; }
        if flags & b::ENQUEUE_WAKEUP as c_int != 0 { update_stats_enqueue_sleeper_dl(dl_rq, se); }
    }
}
unsafe fn update_stats_dequeue_dl(dl_rq: *mut b::dl_rq, se: *mut b::sched_dl_entity, flags: c_int) {
    // SAFETY: This task-only accounting path has a live task entity and its owning rq lock.
    unsafe {
        let p = b::rust_dl_task_of(se);
        let rq = rq_of_dl_rq(dl_rq);
        if !b::rust_dl_schedstat_enabled() { return; }
        if p != b::rust_dl_rq_curr(rq) { update_stats_wait_end_dl(dl_rq, se); }
        if flags & b::DEQUEUE_SLEEP as c_int != 0 {
            let state = b::rust_dl_task_state(p);
            if state & b::TASK_INTERRUPTIBLE as c_uint != 0 {
                b::rust_dl_stats_sleep_start(p, b::rust_dl_rq_clock(rq_of_dl_rq(dl_rq)));
            }
            if state & b::TASK_UNINTERRUPTIBLE as c_uint != 0 {
                b::rust_dl_stats_block_start(p, b::rust_dl_rq_clock(rq_of_dl_rq(dl_rq)));
            }
        }
    }
}
unsafe fn __enqueue_dl_entity(se: *mut b::sched_dl_entity) {
    // SAFETY: The caller holds the entity's rq lock and supplies a detached live rb node.
    unsafe {
        let dl_rq = dl_rq_of_se(se);
        b::rust_dl_warn_enqueue_node(!b::rust_dl_rb_empty_node(addr_of!((*se).rb_node)));
        b::rust_dl_rb_add_cached(addr_of_mut!((*se).rb_node), addr_of_mut!((*dl_rq).root), Some(__dl_less));
        inc_dl_tasks(se, dl_rq);
    }
}
unsafe fn __dequeue_dl_entity(se: *mut b::sched_dl_entity) {
    // SAFETY: The live entity's rq lock serializes its rb node and runnable accounting.
    unsafe {
        let dl_rq = dl_rq_of_se(se);
        if b::rust_dl_rb_empty_node(addr_of!((*se).rb_node)) { return; }
        b::rust_dl_rb_erase_cached(addr_of_mut!((*se).rb_node), addr_of_mut!((*dl_rq).root));
        b::rust_dl_rb_clear_node(addr_of_mut!((*se).rb_node));
        dec_dl_tasks(se, dl_rq);
    }
}
unsafe fn enqueue_dl_entity(se: *mut b::sched_dl_entity, flags: c_int) {
    // SAFETY: The entity's rq is locked; its timers, PI source and bandwidth reservations are live.
    unsafe {
        b::rust_dl_warn_enqueue_on_rq(on_dl_rq(se) != 0);
        update_stats_enqueue_dl(dl_rq_of_se(se), se, flags);
        if (*se).dl_throttled() == 0 && !b::rust_dl_is_implicit(se) { dl_check_constrained_dl(se); }
        if flags & (b::ENQUEUE_RESTORE | b::ENQUEUE_MIGRATING) as c_int != 0 {
            let dl_rq = dl_rq_of_se(se);
            add_rq_bw(se, dl_rq);
            add_running_bw(se, dl_rq);
        }
        if (*se).dl_defer() == 0 && (*se).dl_throttled() != 0
            && flags & b::ENQUEUE_REPLENISH as c_int == 0 {
            if flags & b::ENQUEUE_WAKEUP as c_int != 0 { task_contending(se, flags); }
            return;
        }
        if flags & b::ENQUEUE_WAKEUP as c_int != 0 {
            task_contending(se, flags);
            update_dl_entity(se);
        } else if flags & b::ENQUEUE_REPLENISH as c_int != 0 {
            replenish_dl_entity(se);
        } else if flags & b::ENQUEUE_MOVE as c_int != 0 && !is_dl_boosted(se)
            && b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq_of_dl_se(se))) {
            setup_new_dl_entity(se);
        }
        if (*se).dl_throttled() != 0 && start_dl_timer(se) != 0 { return; }
        if (*se).dl_throttled() != 0 {
            b::hrtimer_try_to_cancel(addr_of_mut!((*se).dl_timer));
            (*se).set_dl_defer_armed(0);
            (*se).set_dl_throttled(0);
        }
        __enqueue_dl_entity(se);
    }
}
unsafe fn dequeue_dl_entity(se: *mut b::sched_dl_entity, flags: c_int) {
    // SAFETY: The entity's rq lock serializes tree removal and inactive-timer ownership changes.
    unsafe {
        __dequeue_dl_entity(se);
        if flags & (b::DEQUEUE_SAVE | b::DEQUEUE_MIGRATING) as c_int != 0 {
            let dl_rq = dl_rq_of_se(se);
            sub_running_bw(se, dl_rq);
            sub_rq_bw(se, dl_rq);
        }
        if flags & b::DEQUEUE_SLEEP as c_int != 0 { task_non_contending(se, true); }
    }
}
#[export_name = "rust_dl_enqueue_task_dl"]
unsafe extern "C" fn enqueue_task_dl(rq: *mut b::rq, p: *mut b::task_struct, mut flags: c_int) {
    // SAFETY: The scheduler enqueue contract holds rq and keeps the task/PI/timer state live.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        let dl_rq = addr_of_mut!((*rq).dl);
        if is_dl_boosted(se) {
            if (*se).dl_throttled() != 0 {
                cancel_replenish_timer(se);
                (*se).set_dl_throttled(0);
            }
        } else if !b::rust_dl_prio((*p).normal_prio) {
            (*se).set_dl_throttled(0);
            if flags & b::ENQUEUE_REPLENISH as c_int == 0 { b::rust_dl_print_missing_replenish(p); }
            return;
        }
        b::rust_dl_check_schedstat_required();
        update_stats_wait_start_dl(dl_rq, se);
        if b::rust_dl_task_on_rq_migrating(p) { flags |= b::ENQUEUE_MIGRATING as c_int; }
        enqueue_dl_entity(se, flags);
        if b::rust_dl_server(se) || b::rust_dl_task_is_blocked(p) || (*dl_rq).curr == se { return; }
        if !b::rust_dl_task_current(rq, p) && (*se).dl_throttled() == 0 && (*p).nr_cpus_allowed > 1 {
            enqueue_pushable_dl_task(rq, p);
        }
    }
}
#[export_name = "rust_dl_dequeue_task_dl"]
unsafe extern "C" fn dequeue_task_dl(rq: *mut b::rq, p: *mut b::task_struct, mut flags: c_int) -> bool {
    // SAFETY: The scheduler dequeue contract holds rq and keeps the task and its timers live.
    unsafe {
        update_curr_dl(rq);
        if b::rust_dl_task_on_rq_migrating(p) { flags |= b::DEQUEUE_MIGRATING as c_int; }
        dequeue_dl_entity(addr_of_mut!((*p).dl), flags);
        if (*p).dl.dl_throttled() == 0 && !b::rust_dl_server(addr_of_mut!((*p).dl)) {
            dequeue_pushable_dl_task(rq, p);
        }
        true
    }
}
#[export_name = "rust_dl_yield_task_dl"]
unsafe extern "C" fn yield_task_dl(rq: *mut b::rq) {
    // SAFETY: The scheduler yield contract holds rq and keeps its donor live.
    unsafe {
        (*b::rust_dl_rq_donor(rq)).dl.set_dl_yielded(1);
        b::update_rq_clock(rq);
        update_curr_dl(rq);
        b::rust_dl_rq_clock_skip_update(rq);
    }
}
#[export_name = "rust_dl_wakeup_preempt_dl"]
unsafe extern "C" fn wakeup_preempt_dl(rq: *mut b::rq, p: *mut b::task_struct, _flags: c_int) {
    // SAFETY: The wakeup-preemption caller holds rq; both waking task and donor are live.
    unsafe {
        let donor = b::rust_dl_rq_donor(rq);
        if (*p).sched_class != addr_of!(b::dl_sched_class)
            || (*donor).sched_class != addr_of!(b::dl_sched_class) { return; }
        if b::rust_dl_entity_preempt(addr_of!((*p).dl), addr_of!((*donor).dl)) {
            b::resched_curr(rq);
            return;
        }
        if (*p).dl.deadline == (*b::rust_dl_rq_donor(rq)).dl.deadline
            && !b::rust_dl_test_tsk_need_resched(b::rust_dl_rq_curr(rq)) { check_preempt_equal_dl(rq, p); }
    }
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe fn start_hrtick_dl(rq: *mut b::rq, se: *mut b::sched_dl_entity) {
    // SAFETY: The locked rq and entity remain live while their runtime sets the native hrtick.
    unsafe { b::hrtick_start(rq, (*se).runtime as u64); }
}
#[cfg(not(CONFIG_SCHED_HRTICK))]
unsafe fn start_hrtick_dl(_rq: *mut b::rq, _se: *mut b::sched_dl_entity) {}
#[export_name = "rust_dl_set_next_task_dl"]
unsafe extern "C" fn set_next_task_dl(rq: *mut b::rq, p: *mut b::task_struct, first: bool) {
    // SAFETY: The scheduler holds rq and installs this live task as its next deadline entity.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        let dl_rq = addr_of_mut!((*rq).dl);
        (*p).se.exec_start = b::rust_dl_rq_clock_task(rq);
        if on_dl_rq(se) != 0 { update_stats_wait_end_dl(dl_rq, se); }
        dequeue_pushable_dl_task(rq, p);
        b::rust_dl_warn_set_next_curr(!(*dl_rq).curr.is_null());
        (*dl_rq).curr = se;
        if !first { return; }
        if (*b::rust_dl_rq_donor(rq)).sched_class != addr_of!(b::dl_sched_class) {
            b::rust_dl_update_rq_load_avg(b::rust_dl_rq_clock_pelt(rq), rq, 0);
        }
        deadline_queue_push_tasks(rq);
        if b::rust_dl_hrtick_enabled(rq) { start_hrtick_dl(rq, se); }
    }
}
unsafe fn pick_next_dl_entity(dl_rq: *mut b::dl_rq) -> *mut b::sched_dl_entity {
    // SAFETY: The dl runqueue tree is live and locked, so its cached first node remains valid.
    unsafe {
        let left = b::rust_dl_rb_first_cached(addr_of!((*dl_rq).root));
        if left.is_null() { core::ptr::null_mut() } else { b::rust_dl_entity_from_node(left) }
    }
}
unsafe fn __pick_task_dl(rq: *mut b::rq, rf: *mut b::rq_flags) -> *mut b::task_struct {
    // SAFETY: The scheduler holds rq and its pin context; initialized servers supply a valid pick callback.
    unsafe {
        loop {
            if !b::rust_dl_runnable(rq) { return core::ptr::null_mut(); }
            let se = pick_next_dl_entity(addr_of_mut!((*rq).dl));
            b::rust_dl_warn_pick_null(se.is_null());
            if b::rust_dl_server(se) {
                // The server initializer supplies a non-null native callback.
                let p = ((*se).server_pick_task.unwrap_unchecked())(se, rf);
                if p.is_null() { dl_server_stop(se); continue; }
                (*rq).dl_server = se;
                return p;
            }
            return b::rust_dl_task_of(se);
        }
    }
}
#[export_name = "rust_dl_pick_task_dl"]
unsafe extern "C" fn pick_task_dl(rq: *mut b::rq, rf: *mut b::rq_flags) -> *mut b::task_struct {
    // SAFETY: The scheduler pick contract supplies the locked rq and live pin context.
    unsafe { __pick_task_dl(rq, rf) }
}
#[export_name = "rust_dl_put_prev_task_dl"]
unsafe extern "C" fn put_prev_task_dl(rq: *mut b::rq, p: *mut b::task_struct, _next: *mut b::task_struct) {
    // SAFETY: The scheduler holds rq and keeps the previous task live through current-state removal.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        let dl_rq = addr_of_mut!((*rq).dl);
        if on_dl_rq(se) != 0 { update_stats_wait_start_dl(dl_rq, se); }
        update_curr_dl(rq);
        b::rust_dl_update_rq_load_avg(b::rust_dl_rq_clock_pelt(rq), rq, 1);
        b::rust_dl_warn_put_prev_curr((*dl_rq).curr != se);
        (*dl_rq).curr = core::ptr::null_mut();
        if b::rust_dl_task_is_blocked(p) { return; }
        if on_dl_rq(se) != 0 && (*p).nr_cpus_allowed > 1 { enqueue_pushable_dl_task(rq, p); }
    }
}
#[export_name = "rust_dl_task_tick_dl"]
unsafe extern "C" fn task_tick_dl(rq: *mut b::rq, p: *mut b::task_struct, queued: c_int) {
    // SAFETY: The scheduler supplies the locked target rq and its live task, including remote ticks.
    unsafe {
        update_curr_dl(rq);
        b::rust_dl_update_rq_load_avg(b::rust_dl_rq_clock_pelt(rq), rq, 1);
        if b::rust_dl_hrtick_enabled(rq) && queued != 0 && (*p).dl.runtime > 0
            && is_leftmost(addr_of_mut!((*p).dl), addr_of_mut!((*rq).dl)) != 0 {
            start_hrtick_dl(rq, addr_of_mut!((*p).dl));
        }
    }
}
#[export_name = "rust_dl_task_fork_dl"]
unsafe extern "C" fn task_fork_dl(_p: *mut b::task_struct) {
    // The original callback is intentionally empty: sched_fork rejects DL fork.
}
#[export_name = "rust_dl_switched_from_dl"]
unsafe extern "C" fn switched_from_dl(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The policy-change caller locks rq and preserves the task and reservations through class change.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        if b::rust_dl_task_on_rq_queued(p) && (*se).dl_runtime != 0 { task_non_contending(se, false); }
        b::rust_dl_dec_tasks_cs(p);
        if !b::rust_dl_task_on_rq_queued(p) {
            if (*se).dl_non_contending() != 0 { sub_running_bw(se, addr_of_mut!((*rq).dl)); }
            sub_rq_bw(se, addr_of_mut!((*rq).dl));
        }
        if (*se).dl_non_contending() != 0 { (*se).set_dl_non_contending(0); }
        if !b::rust_dl_task_on_rq_queued(p) || (*rq).dl.dl_nr_running != 0 { return; }
        deadline_queue_pull_task(rq);
    }
}
#[export_name = "rust_dl_switched_to_dl"]
unsafe extern "C" fn switched_to_dl(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The policy-change caller locks rq and preserves the task and reservations through class change.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        cancel_inactive_timer(se);
        b::rust_dl_inc_tasks_cs(p);
        if !b::rust_dl_task_on_rq_queued(p) { add_rq_bw(se, addr_of_mut!((*rq).dl)); return; }
        if b::rust_dl_rq_donor(rq) != p {
            if (*p).nr_cpus_allowed > 1 && (*rq).dl.overloaded { deadline_queue_push_tasks(rq); }
            if b::rust_dl_task(b::rust_dl_rq_donor(rq)) { wakeup_preempt_dl(rq, p, 0); }
            else { b::resched_curr(rq); }
        } else { b::rust_dl_update_rq_load_avg(b::rust_dl_rq_clock_pelt(rq), rq, 0); }
    }
}
#[export_name = "rust_dl_get_prio_dl"]
unsafe extern "C" fn get_prio_dl(rq: *mut b::rq, p: *mut b::task_struct) -> u64 {
    // SAFETY: The priority caller holds rq and keeps the queried task and donor live.
    unsafe {
        if b::rust_dl_task_current_donor(rq, p) { update_curr_dl(rq); }
        (*p).dl.deadline
    }
}
#[export_name = "rust_dl_prio_changed_dl"]
unsafe extern "C" fn prio_changed_dl(rq: *mut b::rq, p: *mut b::task_struct, old_deadline: u64) {
    // SAFETY: The priority-change caller holds rq and keeps the affected task and donor live.
    unsafe {
        if !b::rust_dl_task_on_rq_queued(p) || (*p).dl.deadline == old_deadline { return; }
        if b::rust_dl_time_before(old_deadline, (*p).dl.deadline) { deadline_queue_pull_task(rq); }
        if b::rust_dl_task_current_donor(rq, p) {
            if b::rust_dl_time_before((*rq).dl.earliest_dl.curr, (*p).dl.deadline) { b::resched_curr(rq); }
        } else if !b::rust_dl_task(b::rust_dl_rq_curr(rq))
            || b::rust_dl_time_before((*p).dl.deadline, (*b::rust_dl_rq_curr(rq)).dl.deadline) { b::resched_curr(rq); }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[export_name = "rust_dl_task_is_throttled_dl"]
unsafe extern "C" fn task_is_throttled_dl(p: *mut b::task_struct, _cpu: c_int) -> c_int {
    // SAFETY: The core-scheduling caller supplies a live task under scheduler synchronization.
    unsafe { (*p).dl.dl_throttled() as c_int }
}
