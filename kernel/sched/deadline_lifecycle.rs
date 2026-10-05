// SPDX-License-Identifier: GPL-2.0
// Newly reconstructed lifecycle owners from the unchanged deadline.c oracle.
// Source-only checkpoint. Generated ABI, compilation and runtime remain unproved.

unsafe fn __push_dl_task(rq: *mut b::rq, rf: *mut b::rq_flags) {
    // SAFETY: The caller owns the rq lock and its live pin context.
    unsafe {
        if has_pushable_dl_tasks(rq) != 0 {
            b::rust_dl_lifecycle_rq_unpin_lock(rq, rf);
            push_dl_task(rq);
            b::rust_dl_lifecycle_rq_repin_lock(rq, rf);
        }
    }
}

// Original one-millisecond forwarding threshold, derived from the native header.
const DL_SERVER_MIN_RES: u64 = b::NSEC_PER_MSEC as u64;

unsafe fn dl_server_timer(
    timer: *mut b::hrtimer,
    dl_se: *mut b::sched_dl_entity,
) -> b::hrtimer_restart {
    // SAFETY: The hrtimer callback owns a live server; rq_lock serializes state.
    unsafe {
        let rq = rq_of_dl_se(dl_se);
        let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        b::rust_dl_lifecycle_rq_lock(rq, rf);
        let result = 'unlock: {
            if (*dl_se).dl_throttled() == 0 || (*dl_se).dl_runtime == 0 {
                break 'unlock b::HRTIMER_NORESTART;
            }
            b::rust_dl_lifecycle_sched_clock_tick();
            b::rust_dl_lifecycle_update_rq_clock(rq);
            // Every installed scheduler class supplies this callback.
            ((*(*b::rust_dl_lifecycle_rq_donor(rq)).sched_class).update_curr.unwrap_unchecked())(rq);
            if (*dl_se).dl_defer_idle() != 0 {
                dl_server_stop(dl_se);
                break 'unlock b::HRTIMER_NORESTART;
            }
            if (*dl_se).dl_defer_armed() != 0 {
                let threshold = (*dl_se).deadline
                    .wrapping_sub((*dl_se).runtime as u64)
                    .wrapping_sub(DL_SERVER_MIN_RES);
                if b::rust_dl_time_before(b::rust_dl_rq_clock((*dl_se).rq), threshold) {
                    let fw = (*dl_se).deadline
                        .wrapping_sub(b::rust_dl_rq_clock((*dl_se).rq))
                        .wrapping_sub((*dl_se).runtime as u64);
                    b::rust_dl_lifecycle_hrtimer_forward_now(timer, b::rust_dl_ns_to_ktime(fw));
                    break 'unlock b::HRTIMER_RESTART;
                }
                (*dl_se).set_dl_defer_running(1);
            }
            enqueue_dl_entity(dl_se, b::ENQUEUE_REPLENISH as c_int);
            let curr = b::rust_dl_lifecycle_rq_curr((*dl_se).rq);
            if !b::rust_dl_task(curr)
                || b::rust_dl_lifecycle_entity_preempt(dl_se, addr_of!((*curr).dl))
            {
                b::rust_dl_lifecycle_resched_curr(rq);
            }
            __push_dl_task(rq, rf);
            b::HRTIMER_NORESTART
        };
        b::rust_dl_lifecycle_rq_unlock(rq, rf);
        result
    }
}

unsafe extern "C" fn dl_task_timer(timer: *mut b::hrtimer) -> b::hrtimer_restart {
    // SAFETY: Timer ownership carries the task reference until the final put.
    unsafe {
        let dl_se = b::rust_dl_lifecycle_entity_from_dl_timer(timer);
        if b::rust_dl_server(dl_se) {
            return dl_server_timer(timer, dl_se);
        }
        let p = b::rust_dl_task_of(dl_se);
        let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        let mut rq = b::rust_dl_lifecycle_task_rq_lock(p, rf);
        'unlock: {
            if !b::rust_dl_task(p) || is_dl_boosted(dl_se) || (*dl_se).dl_throttled() == 0 {
                break 'unlock;
            }
            b::rust_dl_lifecycle_sched_clock_tick();
            b::rust_dl_lifecycle_update_rq_clock(rq);
            if !b::rust_dl_task_on_rq_queued(p) {
                replenish_dl_entity(dl_se);
                break 'unlock;
            }
            if b::rust_dl_lifecycle_unlikely_timer_offline((*rq).online == 0) {
                b::rust_dl_lifecycle_lockdep_unpin_rq(rq, rf);
                rq = dl_task_offline_migration(rq, p);
                b::rust_dl_lifecycle_lockdep_pin_rq(rq, rf);
                b::rust_dl_lifecycle_update_rq_clock(rq);
            }
            enqueue_task_dl(rq, p, b::ENQUEUE_REPLENISH as c_int);
            if b::rust_dl_task(b::rust_dl_lifecycle_rq_donor(rq)) {
                wakeup_preempt_dl(rq, p, 0);
            } else {
                b::rust_dl_lifecycle_resched_curr(rq);
            }
            __push_dl_task(rq, rf);
        }
        b::rust_dl_lifecycle_task_rq_unlock(rq, p, rf);
        // No task, entity, timer or rq dereference is permitted after this put.
        b::rust_dl_put_task_struct(p);
        b::HRTIMER_NORESTART
    }
}

unsafe fn init_dl_task_timer(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller exclusively initializes the live entity's timer.
    unsafe {
        b::rust_dl_lifecycle_hrtimer_setup(
            addr_of_mut!((*dl_se).dl_timer), Some(dl_task_timer),
            b::CLOCK_MONOTONIC as _, b::HRTIMER_MODE_REL_HARD,
        );
    }
}

unsafe fn update_curr_dl_se(rq: *mut b::rq, dl_se: *mut b::sched_dl_entity, delta_exec: i64) {
    // SAFETY: The caller holds rq and provides its live current task/server entity.
    unsafe {
        let idle = b::rust_dl_lifecycle_idle_rq(rq);
        if b::rust_dl_lifecycle_unlikely_delta_nonpositive(delta_exec <= 0) {
            if !b::rust_dl_lifecycle_unlikely_delta_yielded((*dl_se).dl_yielded() != 0) {
                return;
            }
            // A yielded entity goes straight to throttling, even for nonpositive time.
        } else {
            if b::rust_dl_server(dl_se) && (*dl_se).dl_throttled() != 0
                && (*dl_se).dl_defer() == 0
            {
                return;
            }
            if b::rust_dl_entity_is_special(dl_se) {
                return;
            }
            let scaled_delta_exec = if !b::rust_dl_server(dl_se) {
                dl_scaled_delta_exec(rq, dl_se, delta_exec)
            } else {
                delta_exec
            };
            (*dl_se).runtime = (*dl_se).runtime.wrapping_sub(scaled_delta_exec);
            if (*dl_se).dl_defer_idle() != 0 && !idle {
                (*dl_se).set_dl_defer_idle(0);
            }
            if (*dl_se).dl_defer() != 0 && (*dl_se).dl_throttled() != 0
                && dl_runtime_exceeded(dl_se) != 0
            {
                b::rust_dl_warn_lifecycle_defer_nonserver(!b::rust_dl_server(dl_se));
                if (*dl_se).dl_defer_idle() != 0 && idle {
                    (*dl_se).runtime = 0;
                    return;
                }
                (*dl_se).set_dl_defer_running(0);
                b::hrtimer_try_to_cancel(addr_of_mut!((*dl_se).dl_timer));
                replenish_dl_new_period(dl_se, (*dl_se).rq);
                if idle {
                    (*dl_se).set_dl_defer_idle(1);
                }
                b::rust_dl_warn_lifecycle_defer_timer_start(start_dl_timer(dl_se) == 0);
                return;
            }
        }

        if dl_runtime_exceeded(dl_se) != 0 || (*dl_se).dl_yielded() != 0 {
            b::rust_dl_trace_throttle(dl_se, b::rust_dl_cpu_of(rq), dl_get_type(dl_se, rq));
            (*dl_se).set_dl_throttled(1);
            if dl_runtime_exceeded(dl_se) != 0
                && (*dl_se).flags & b::SCHED_FLAG_DL_OVERRUN as c_uint != 0
            {
                (*dl_se).set_dl_overrun(1);
            }
            dequeue_dl_entity(dl_se, 0);
            if !b::rust_dl_server(dl_se) {
                update_stats_dequeue_dl(addr_of_mut!((*rq).dl), dl_se, 0);
                dequeue_pushable_dl_task(rq, b::rust_dl_task_of(dl_se));
            }
            if b::rust_dl_lifecycle_unlikely_throttle_timer(is_dl_boosted(dl_se) || start_dl_timer(dl_se) == 0) {
                if b::rust_dl_server(dl_se) {
                    if (*dl_se).dl_defer() != 0 {
                        replenish_dl_new_period(dl_se, rq);
                        start_dl_timer(dl_se);
                    } else {
                        enqueue_dl_entity(dl_se, b::ENQUEUE_REPLENISH as c_int);
                    }
                } else {
                    enqueue_task_dl(rq, b::rust_dl_task_of(dl_se), b::ENQUEUE_REPLENISH as c_int);
                }
            }
            if is_leftmost(dl_se, addr_of_mut!((*rq).dl)) == 0 {
                b::rust_dl_lifecycle_resched_curr(rq);
            }
        } else {
            b::rust_dl_lifecycle_trace_update(dl_se, b::rust_dl_cpu_of(rq), dl_get_type(dl_se, rq));
        }
        if (*dl_se).dl_server() != 0 {
            return;
        }
        #[cfg(CONFIG_RT_GROUP_SCHED)]
        if b::rust_dl_lifecycle_rt_bandwidth_enabled() != 0 {
            let rt_rq = addr_of_mut!((*rq).rt);
            b::rust_dl_raw_spin_lock(addr_of_mut!((*rt_rq).rt_runtime_lock));
            if b::rust_dl_lifecycle_sched_rt_bandwidth_account(rt_rq) {
                (*rt_rq).rt_time = (*rt_rq).rt_time.wrapping_add(delta_exec as u64);
            }
            b::rust_dl_raw_spin_unlock(addr_of_mut!((*rt_rq).rt_runtime_lock));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_update_idle(dl_se: *mut b::sched_dl_entity, delta_exec: i64) {
    // SAFETY: The caller holds the server rq lock while accounting idle time.
    unsafe {
        if (*dl_se).dl_server_active() != 0 && (*dl_se).dl_runtime != 0 && (*dl_se).dl_defer() != 0 {
            update_curr_dl_se((*dl_se).rq, dl_se, delta_exec);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_update(dl_se: *mut b::sched_dl_entity, delta_exec: i64) {
    // SAFETY: The caller holds the server rq lock while accounting execution.
    unsafe {
        if (*dl_se).dl_server_active() != 0 && (*dl_se).dl_runtime != 0 {
            update_curr_dl_se((*dl_se).rq, dl_se, delta_exec);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_start(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller holds the server rq lock; the donor has a scheduler class.
    unsafe {
        let rq = (*dl_se).rq;
        (*dl_se).set_dl_defer_idle(0);
        if !b::rust_dl_server(dl_se) || (*dl_se).dl_server_active() != 0
            || (*dl_se).dl_runtime == 0 || (*dl_se).dl_bw_attached() == 0
        {
            return;
        }
        ((*(*b::rust_dl_lifecycle_rq_donor(rq)).sched_class).update_curr.unwrap_unchecked())(rq);
        if b::rust_dl_warn_lifecycle_start_offline(!b::rust_dl_lifecycle_cpu_online(b::rust_dl_cpu_of(rq))) {
            return;
        }
        b::rust_dl_lifecycle_trace_server_start(dl_se, b::rust_dl_cpu_of(rq), dl_get_type(dl_se, rq));
        (*dl_se).set_dl_server_active(1);
        enqueue_dl_entity(dl_se, b::ENQUEUE_WAKEUP as c_int);
        if !b::rust_dl_task(b::rust_dl_lifecycle_rq_curr((*dl_se).rq))
            || b::rust_dl_lifecycle_entity_preempt(dl_se, addr_of!((*b::rust_dl_lifecycle_rq_curr(rq)).dl))
        {
            b::rust_dl_lifecycle_resched_curr((*dl_se).rq);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_stop(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller holds the server rq lock throughout dequeue/cancellation.
    unsafe {
        if !b::rust_dl_server(dl_se) || !b::rust_dl_lifecycle_server_active(dl_se) {
            return;
        }
        b::rust_dl_lifecycle_trace_server_stop(
            dl_se, b::rust_dl_cpu_of((*dl_se).rq), dl_get_type(dl_se, (*dl_se).rq),
        );
        dequeue_dl_entity(dl_se, b::DEQUEUE_SLEEP as c_int);
        b::hrtimer_try_to_cancel(addr_of_mut!((*dl_se).dl_timer));
        (*dl_se).set_dl_defer_armed(0);
        (*dl_se).set_dl_throttled(0);
        (*dl_se).set_dl_defer_idle(0);
        (*dl_se).set_dl_server_active(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_init(
    dl_se: *mut b::sched_dl_entity, rq: *mut b::rq, pick_task: b::dl_server_pick_f,
) {
    // SAFETY: The caller exclusively initializes the entity and its callback.
    unsafe {
        (*dl_se).rq = rq;
        (*dl_se).server_pick_task = pick_task;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sched_init_dl_servers() {
    // SAFETY: Scheduler initialization supplies stable online CPUs and runqueues.
    unsafe {
        let online = b::rust_dl_online_mask();
        let mut cpu = b::rust_dl_cpumask_first(online);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            let runtime = 50u64 * b::NSEC_PER_MSEC as u64;
            let period = 1000u64 * b::NSEC_PER_MSEC as u64;
            let rq = b::rust_dl_cpu_rq(cpu as c_int);
            let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
            let rf = rf.as_mut_ptr();
            b::rust_dl_lifecycle_rq_lock_irq(rq, rf);
            b::rust_dl_lifecycle_update_rq_clock(rq);
            let dl_se = addr_of_mut!((*rq).fair_server);
            b::rust_dl_warn_lifecycle_init_fair_server(b::rust_dl_server(dl_se));
            dl_server_apply_params(dl_se, runtime, period, true);
            (*dl_se).set_dl_server(1);
            (*dl_se).set_dl_defer(1);
            setup_new_dl_entity(dl_se);
            #[cfg(CONFIG_SCHED_CLASS_EXT)]
            {
                let dl_se = addr_of_mut!((*rq).ext_server);
                b::rust_dl_warn_lifecycle_init_ext_server(b::rust_dl_server(dl_se));
                dl_server_apply_params(dl_se, runtime, period, true);
                (*dl_se).set_dl_server(1);
                (*dl_se).set_dl_defer(1);
                setup_new_dl_entity(dl_se);
                dl_server_detach_bw(dl_se);
            }
            b::rust_dl_lifecycle_rq_unlock_irq(rq, rf);
            cpu = b::rust_dl_cpumask_next(cpu as c_int, online);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __dl_server_attach_root(dl_se: *mut b::sched_dl_entity, rq: *mut b::rq) {
    // SAFETY: Root-domain rebuilding supplies the rq/RCU exclusion of the C owner.
    unsafe {
        let new_bw = (*dl_se).dl_bw;
        let cpu = b::rust_dl_cpu_of(rq);
        if (*dl_se).dl_bw_attached() == 0 {
            return;
        }
        let dl_b = dl_bw_of(b::rust_dl_cpu_of(rq));
        b::rust_dl_raw_spin_lock(addr_of_mut!((*dl_b).lock));
        if dl_bw_cpus(cpu) != 0 {
            __dl_add(dl_b, new_bw, dl_bw_cpus(cpu));
        }
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*dl_b).lock));
    }
}

unsafe fn __dl_server_attach_bw_locked(
    dl_se: *mut b::sched_dl_entity, dl_b: *mut b::dl_bw, cpus: c_int,
) -> c_int {
    // SAFETY: The caller holds both rq and the root-domain bandwidth lock.
    unsafe {
        let rq = (*dl_se).rq;
        if b::rust_dl_cpu_active(b::rust_dl_cpu_of(rq)) {
            let cap = dl_bw_capacity(b::rust_dl_cpu_of(rq));
            if __dl_overflow(dl_b, cap, 0, (*dl_se).dl_bw) {
                return -(b::EBUSY as c_int);
            }
            __dl_add(dl_b, (*dl_se).dl_bw, cpus);
        }
        __add_rq_bw((*dl_se).dl_bw, addr_of_mut!((*rq).dl));
        (*dl_se).set_dl_bw_attached(1);
        0
    }
}

unsafe fn __dl_server_detach_bw_locked(
    dl_se: *mut b::sched_dl_entity, dl_b: *mut b::dl_bw, cpus: c_int,
) {
    // SAFETY: The caller holds both rq and the root-domain bandwidth lock.
    unsafe {
        let rq = (*dl_se).rq;
        if (*dl_se).dl_server_active() != 0 {
            dl_server_stop(dl_se);
        }
        dl_rq_change_utilization(rq, dl_se, 0);
        if b::rust_dl_cpu_active(b::rust_dl_cpu_of(rq)) {
            __dl_sub(dl_b, (*dl_se).dl_bw, cpus);
        }
        (*dl_se).set_dl_bw_attached(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_attach_bw(dl_se: *mut b::sched_dl_entity) -> c_int {
    // SAFETY: The caller holds rq, keeping its root domain stable across locking.
    unsafe {
        let rq = (*dl_se).rq;
        let cpu = b::rust_dl_cpu_of(rq);
        if (*dl_se).dl_bw_attached() != 0 {
            return 0;
        }
        let lock = addr_of_mut!((*dl_bw_of(cpu)).lock);
        b::rust_dl_raw_spin_lock(lock);
        let dl_b = dl_bw_of(cpu);
        let cpus = dl_bw_cpus(cpu);
        let ret = __dl_server_attach_bw_locked(dl_se, dl_b, cpus);
        b::rust_dl_raw_spin_unlock(lock);
        if ret != 0 {
            return ret;
        }
        if b::rust_dl_lifecycle_cpu_online(cpu) {
            dl_server_start(dl_se);
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_detach_bw(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller holds rq, keeping its root domain stable across locking.
    unsafe {
        let cpu = b::rust_dl_cpu_of((*dl_se).rq);
        if (*dl_se).dl_bw_attached() == 0 {
            return;
        }
        let dl_b = dl_bw_of(cpu);
        b::rust_dl_raw_spin_lock(addr_of_mut!((*dl_b).lock));
        let cpus = dl_bw_cpus(cpu);
        __dl_server_detach_bw_locked(dl_se, dl_b, cpus);
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*dl_b).lock));
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_server_swap_bw(
    detach_se: *mut b::sched_dl_entity, attach_se: *mut b::sched_dl_entity,
) -> c_int {
    // SAFETY: Both entities belong to the locked rq, as required by the C owner.
    unsafe {
        let rq = (*detach_se).rq;
        let cpu = b::rust_dl_cpu_of(rq);
        b::rust_dl_warn_lifecycle_swap_rq((*attach_se).rq != rq);
        let lock = addr_of_mut!((*dl_bw_of(cpu)).lock);
        b::rust_dl_raw_spin_lock(lock);
        let dl_b = dl_bw_of(cpu);
        let cpus = dl_bw_cpus(cpu);
        if (*detach_se).dl_bw_attached() != 0 {
            __dl_server_detach_bw_locked(detach_se, dl_b, cpus);
        }
        let ret = if (*attach_se).dl_bw_attached() != 0 {
            0
        } else {
            __dl_server_attach_bw_locked(attach_se, dl_b, cpus)
        };
        b::rust_dl_raw_spin_unlock(lock);
        if ret != 0 {
            return ret;
        }
        if b::rust_dl_lifecycle_cpu_online(cpu) {
            dl_server_start(attach_se);
        }
        0
    }
}

#[export_name = "rust_dl_update_curr_dl"]
unsafe extern "C" fn update_curr_dl(rq: *mut b::rq) {
    // SAFETY: The scheduler invokes this with rq locked and a live donor.
    unsafe {
        let donor = b::rust_dl_lifecycle_rq_donor(rq);
        let dl_se = addr_of_mut!((*donor).dl);
        if !b::rust_dl_task(donor) || on_dl_rq(dl_se) == 0 {
            return;
        }
        let delta_exec = b::rust_dl_lifecycle_update_curr_common(rq);
        update_curr_dl_se(rq, dl_se, delta_exec);
    }
}

unsafe extern "C" fn inactive_task_timer(timer: *mut b::hrtimer) -> b::hrtimer_restart {
    // SAFETY: A task timer carries a task reference; server timers have rq lifetime.
    unsafe {
        let dl_se = b::rust_dl_lifecycle_entity_from_inactive_timer(timer);
        let mut p = core::ptr::null_mut();
        let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        let rq;
        if !b::rust_dl_server(dl_se) {
            p = b::rust_dl_task_of(dl_se);
            rq = b::rust_dl_lifecycle_task_rq_lock(p, rf);
        } else {
            rq = (*dl_se).rq;
            b::rust_dl_lifecycle_rq_lock(rq, rf);
        }
        b::rust_dl_lifecycle_sched_clock_tick();
        b::rust_dl_lifecycle_update_rq_clock(rq);
        'unlock: {
            if !b::rust_dl_server(dl_se)
                && (!b::rust_dl_task(p) || b::rust_dl_task_state(p) == b::TASK_DEAD as c_uint)
            {
                let dl_b = dl_bw_of(b::rust_dl_task_cpu(p));
                if b::rust_dl_task_state(p) == b::TASK_DEAD as c_uint
                    && (*dl_se).dl_non_contending() != 0
                {
                    let task_dl = addr_of_mut!((*p).dl);
                    sub_running_bw(task_dl, dl_rq_of_se(task_dl));
                    sub_rq_bw(task_dl, dl_rq_of_se(task_dl));
                    (*dl_se).set_dl_non_contending(0);
                }
                b::rust_dl_raw_spin_lock(addr_of_mut!((*dl_b).lock));
                __dl_sub(dl_b, (*p).dl.dl_bw, dl_bw_cpus(b::rust_dl_task_cpu(p)));
                b::rust_dl_raw_spin_unlock(addr_of_mut!((*dl_b).lock));
                __dl_clear_params(dl_se);
                break 'unlock;
            }
            if (*dl_se).dl_non_contending() == 0 {
                break 'unlock;
            }
            sub_running_bw(dl_se, addr_of_mut!((*rq).dl));
            (*dl_se).set_dl_non_contending(0);
        }
        if !b::rust_dl_server(dl_se) {
            b::rust_dl_lifecycle_task_rq_unlock(rq, p, rf);
            b::rust_dl_put_task_struct(p);
        } else {
            b::rust_dl_lifecycle_rq_unlock(rq, rf);
        }
        b::HRTIMER_NORESTART
    }
}

unsafe fn init_dl_inactive_task_timer(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller exclusively initializes the live entity's timer.
    unsafe {
        b::rust_dl_lifecycle_hrtimer_setup(
            addr_of_mut!((*dl_se).inactive_timer), Some(inactive_task_timer),
            b::CLOCK_MONOTONIC as _, b::HRTIMER_MODE_REL_HARD,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn __getparam_dl(p: *mut b::task_struct, attr: *mut b::sched_attr, flags: c_uint) {
    // SAFETY: The caller supplies a live task and writable attributes; lock dynamic reads.
    unsafe {
        let dl_se = addr_of_mut!((*p).dl);
        let rq = b::rust_dl_task_rq(p);
        (*attr).sched_priority = (*p).rt_priority;
        if flags & b::SCHED_GETATTR_FLAG_DL_DYNAMIC as c_uint != 0 {
            b::rust_dl_lifecycle_raw_spin_lock_irq(addr_of_mut!((*rq).__lock));
            b::rust_dl_lifecycle_update_rq_clock(rq);
            if b::rust_dl_lifecycle_task_current(rq, p) != 0 {
                update_curr_dl(rq);
            }
            (*attr).sched_runtime = (*dl_se).runtime as u64;
            let adj_deadline = (*dl_se).deadline.wrapping_sub(b::rust_dl_rq_clock(rq))
                .wrapping_add(b::rust_dl_lifecycle_ktime_get_ns());
            (*attr).sched_deadline = adj_deadline;
            b::rust_dl_lifecycle_raw_spin_unlock_irq(addr_of_mut!((*rq).__lock));
        } else {
            (*attr).sched_runtime = (*dl_se).dl_runtime;
            (*attr).sched_deadline = (*dl_se).dl_deadline;
        }
        (*attr).sched_period = (*dl_se).dl_period;
        (*attr).sched_flags &= !(b::SCHED_DL_FLAGS as u64);
        (*attr).sched_flags |= (*dl_se).flags as u64;
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_dl_entity(dl_se: *mut b::sched_dl_entity) {
    // SAFETY: The caller exclusively initializes the entity before publishing it.
    unsafe {
        b::rust_dl_lifecycle_rb_clear_node(addr_of_mut!((*dl_se).rb_node));
        init_dl_task_timer(dl_se);
        init_dl_inactive_task_timer(dl_se);
        __dl_clear_params(dl_se);
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_dl_stats(m: *mut b::seq_file, cpu: c_int) {
    // SAFETY: The debug caller supplies a valid CPU and either a live seq_file
    // or null for console output, as accepted by native print_dl_rq/SEQ_printf.
    unsafe {
        b::rust_dl_lifecycle_print_dl_rq(m, cpu, addr_of_mut!((*b::rust_dl_cpu_rq(cpu)).dl));
    }
}
