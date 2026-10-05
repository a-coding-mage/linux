// SPDX-License-Identifier: GPL-2.0
// rt.c tick/watchdog, admission and sysctl transaction/rollback bodies.
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
unsafe fn watchdog(_rq: *mut b::rq, _p: *mut b::task_struct) {
    #[cfg(CONFIG_POSIX_TIMERS)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let soft = b::rust_rt_task_rlimit(_p, b::RUST_RT_RLIMIT_RTTIME as _);
        let hard = b::rust_rt_task_rlimit_max(_p, b::RUST_RT_RLIMIT_RTTIME as _);
        if soft != b::RUST_RT_RLIM_INFINITY as c_ulong {
            if (*_p).rt.watchdog_stamp != b::rust_rt_jiffies() {
                (*_p).rt.timeout = (*_p).rt.timeout.wrapping_add(1);
                (*_p).rt.watchdog_stamp = b::rust_rt_jiffies();
            }
            let divisor = b::RUST_RT_USEC_PER_SEC as c_ulong / b::RUST_RT_HZ as c_ulong;
            // DIV_ROUND_UP's unsigned-long arithmetic, including addition wrap.
            let next = min(soft, hard).wrapping_add(divisor).wrapping_sub(1) / divisor;
            if (*_p).rt.timeout > next {
                b::rust_rt_posix_cputimers_rt_watchdog(addr_of_mut!((*_p).posix_cputimers), (*_p).se.sum_exec_runtime);
            }
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
pub unsafe extern "C" fn task_tick_rt(rq: *mut b::rq, p: *mut b::task_struct, _queued: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut rt_se = addr_of_mut!((*p).rt);
        update_curr_rt(rq);
        b::rust_rt_update_rt_rq_load_avg(b::rust_rt_rq_clock_pelt(rq), rq, 1);
        watchdog(rq, p);
        if (*p).policy != b::RUST_RT_SCHED_RR as c_uint { return; }
        (*p).rt.time_slice = (*p).rt.time_slice.wrapping_sub(1);
        if (*p).rt.time_slice != 0 { return; }
        (*p).rt.time_slice = sched_rr_timeslice as c_uint;
        while !rt_se.is_null() {
            if (*rt_se).run_list.prev != (*rt_se).run_list.next {
                requeue_task_rt(rq, p, 0);
                b::resched_curr(rq);
                return;
            }
            rt_se = rt_parent(rt_se);
        }
    }
}
#[no_mangle]
/// Reads the configured interval for an RR task.
///
/// # Safety
/// The task must remain live with policy readable under its native scheduler protocol.
/// Access to the shared RR timeslice must obey the native sysctl/scheduler access
/// policy, whose Rust concurrency qualification remains an admission gate.
pub unsafe extern "C" fn get_rr_interval_rt(_rq: *mut b::rq, task: *mut b::task_struct) -> c_uint {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if (*task).policy == b::RUST_RT_SCHED_RR as c_uint { sched_rr_timeslice as c_uint } else { 0 }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
/// Checks the RT throttle state used by core scheduling.
///
/// # Safety
/// The task and selected CPU runqueue must remain live under the native
/// core-scheduling lock protocol. When RT groups are configured, task-group membership
/// and its per-CPU RT queue array must remain valid.
pub unsafe extern "C" fn task_is_throttled_rt(_p: *mut b::task_struct, cpu: c_int) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)]
        let rr = {
            let rr = *(*b::rust_rt_task_group(_p)).rt_rq.add(cpu as usize);
            b::rust_rt_warn_2593(!b::rust_rt_group_sched_enabled() && (*rr).tg != addr_of_mut!(b::root_task_group));
            rr
        };
        #[cfg(not(CONFIG_RT_GROUP_SCHED))]
        let rr = addr_of_mut!((*b::rust_rt_cpu_rq(cpu)).rt);
        rt_rq_throttled(rr) as c_int
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Checks whether a task group contains RT tasks.
///
/// # Safety
/// The task group and its cgroup subsystem state must remain live under the native
/// constraints-update protocol. The native css_task_iter lifetime rules must be
/// satisfied until iterator end.
unsafe fn tg_has_rt_tasks(tg: *mut b::task_group) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut ret = 0;
        if b::rust_rt_task_group_is_autogroup(tg) { return 0; }
        let mut it = MaybeUninit::<b::css_task_iter>::uninit();
        b::css_task_iter_start(addr_of_mut!((*tg).css), 0, it.as_mut_ptr());
        while ret == 0 {
            let task = b::css_task_iter_next(it.as_mut_ptr());
            if task.is_null() { break; }
            ret |= b::rust_rt_rt_task(task) as c_int;
        }
        b::css_task_iter_end(it.as_mut_ptr());
        ret
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Validates one group during RT admission traversal.
///
/// # Safety
/// The callback data must point to the live rt_schedulable_data object owned by the
/// synchronous traversal. RCU must protect the current group and child links, and the
/// native constraints mutex/protocol must serialize the bandwidth values being
/// validated.
unsafe extern "C" fn tg_rt_schedulable(tg: *mut b::task_group, data: *mut c_void) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let d = data.cast::<b::rt_schedulable_data>();
        let mut period = b::rust_rt_ktime_to_ns((*tg).rt_bandwidth.rt_period) as u64;
        let mut runtime = (*tg).rt_bandwidth.rt_runtime;
        if tg == (*d).tg { period = (*d).rt_period; runtime = (*d).rt_runtime; }
        if runtime > period && runtime != b::RUST_RT_RUNTIME_INF as u64 { return -(b::RUST_RT_EINVAL as c_int); }
        if b::rust_rt_bandwidth_enabled() && runtime == 0 && (*tg).rt_bandwidth.rt_runtime != 0 && tg_has_rt_tasks(tg) != 0 {
            return -(b::RUST_RT_EBUSY as c_int);
        }
        let total = b::to_ratio(period, runtime);
        if total > b::to_ratio(b::rust_rt_global_period() as u64, b::rust_rt_global_runtime() as u64) {
            return -(b::RUST_RT_EINVAL as c_int);
        }
        let mut sum = 0u64;
        let head = addr_of_mut!((*tg).children);
        let mut node = b::rust_rt_list_next_rcu(head);
        while node != head {
            let child = container_of!(node, b::task_group, siblings);
            period = b::rust_rt_ktime_to_ns((*child).rt_bandwidth.rt_period) as u64;
            runtime = (*child).rt_bandwidth.rt_runtime;
            if child == (*d).tg { period = (*d).rt_period; runtime = (*d).rt_runtime; }
            sum = sum.wrapping_add(b::to_ratio(period, runtime));
            node = b::rust_rt_list_next_rcu(node);
        }
        if sum > total { return -(b::RUST_RT_EINVAL as c_int); }
        0
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Validates an RT bandwidth proposal across the task-group tree.
///
/// # Safety
/// The caller must serialize bandwidth changes with the native constraints mutex. A
/// non-null proposal group must remain live; null is the native global-validation
/// sentinel. The synchronous native tree walker must not retain the stack-owned
/// callback data.
unsafe fn __rt_schedulable(tg: *mut b::task_group, period: u64, runtime: u64) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut data = b::rt_schedulable_data { tg, rt_period: period, rt_runtime: runtime };
        b::rust_rt_rcu_read_lock();
        let ret = b::rust_rt_walk_tg_tree(Some(tg_rt_schedulable), Some(b::tg_nop), addr_of_mut!(data).cast());
        b::rust_rt_rcu_read_unlock();
        ret
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Applies a validated RT task-group bandwidth update.
///
/// # Safety
/// The task group and its initialized per-CPU RT queues must remain live under the
/// cgroup update protocol. The caller must permit native mutex acquisition and supply
/// any outer serialization needed for the initial bandwidth reads and task-group
/// topology.
unsafe fn tg_set_rt_bandwidth(tg: *mut b::task_group, rt_period: u64, rt_runtime: u64) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if tg == addr_of_mut!(b::root_task_group) && rt_runtime == 0 { return -(b::RUST_RT_EINVAL as c_int); }
        if rt_period == 0 { return -(b::RUST_RT_EINVAL as c_int); }
        if rt_runtime != b::RUST_RT_RUNTIME_INF as u64 && rt_runtime > MAX_RT_RUNTIME { return -(b::RUST_RT_EINVAL as c_int); }
        b::rust_rt_mutex_lock(b::rust_rt_constraints_mutex());
        let err = __rt_schedulable(tg, rt_period, rt_runtime);
        if err == 0 {
            b::rust_rt_raw_spin_lock_irq(addr_of_mut!((*tg).rt_bandwidth.rt_runtime_lock));
            (*tg).rt_bandwidth.rt_period = b::rust_rt_ns_to_ktime(rt_period);
            (*tg).rt_bandwidth.rt_runtime = rt_runtime;
            each_cpu(b::rust_rt_cpu_possible_mask(), |i| {
                let rr = *(*tg).rt_rq.add(i as usize);
                b::rust_rt_raw_spin_lock(addr_of_mut!((*rr).rt_runtime_lock));
                (*rr).rt_runtime = rt_runtime;
                b::rust_rt_raw_spin_unlock(addr_of_mut!((*rr).rt_runtime_lock));
            });
            b::rust_rt_raw_spin_unlock_irq(addr_of_mut!((*tg).rt_bandwidth.rt_runtime_lock));
        }
        b::rust_rt_mutex_unlock(b::rust_rt_constraints_mutex());
        err
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Applies a validated RT task-group bandwidth update.
///
/// # Safety
/// The task group and its initialized per-CPU RT queues must remain live under the
/// cgroup update protocol. The caller must permit native mutex acquisition and supply
/// any outer serialization needed for the initial bandwidth reads and task-group
/// topology.
pub unsafe extern "C" fn sched_group_set_rt_runtime(tg: *mut b::task_group, rt_runtime_us: c_long) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rt_period = b::rust_rt_ktime_to_ns((*tg).rt_bandwidth.rt_period) as u64;
        let mut rt_runtime = (rt_runtime_us as u64).wrapping_mul(b::RUST_RT_NSEC_PER_USEC as u64);
        if rt_runtime_us < 0 { rt_runtime = b::RUST_RT_RUNTIME_INF as u64; }
        else if rt_runtime_us as u64 > b::RUST_RT_U64_MAX as u64 / b::RUST_RT_NSEC_PER_USEC as u64 { return -(b::RUST_RT_EINVAL as c_int); }
        tg_set_rt_bandwidth(tg, rt_period, rt_runtime)
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Reads the RT bandwidth policy for a live task group.
///
/// # Safety
/// The group, its bandwidth object and any supplied task must remain live. The caller
/// must preserve the native cgroup/scheduler synchronization for policy reads and task
/// attachment decisions.
pub unsafe extern "C" fn sched_group_rt_runtime(tg: *mut b::task_group) -> c_long {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if (*tg).rt_bandwidth.rt_runtime == b::RUST_RT_RUNTIME_INF as u64 { return -1; }
        b::rust_rt_div_u64((*tg).rt_bandwidth.rt_runtime, b::RUST_RT_NSEC_PER_USEC as u32) as c_long
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Applies a validated RT task-group bandwidth update.
///
/// # Safety
/// The task group and its initialized per-CPU RT queues must remain live under the
/// cgroup update protocol. The caller must permit native mutex acquisition and supply
/// any outer serialization needed for the initial bandwidth reads and task-group
/// topology.
pub unsafe extern "C" fn sched_group_set_rt_period(tg: *mut b::task_group, rt_period_us: u64) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if rt_period_us > b::RUST_RT_U64_MAX as u64 / b::RUST_RT_NSEC_PER_USEC as u64 { return -(b::RUST_RT_EINVAL as c_int); }
        let rt_period = rt_period_us.wrapping_mul(b::RUST_RT_NSEC_PER_USEC as u64);
        let rt_runtime = (*tg).rt_bandwidth.rt_runtime;
        tg_set_rt_bandwidth(tg, rt_period, rt_runtime)
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Reads the RT bandwidth policy for a live task group.
///
/// # Safety
/// The group, its bandwidth object and any supplied task must remain live. The caller
/// must preserve the native cgroup/scheduler synchronization for policy reads and task
/// attachment decisions.
pub unsafe extern "C" fn sched_group_rt_period(tg: *mut b::task_group) -> c_long {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_div_u64(b::rust_rt_ktime_to_ns((*tg).rt_bandwidth.rt_period) as u64, b::RUST_RT_NSEC_PER_USEC as u32) as c_long
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
#[no_mangle]
/// Reads the RT bandwidth policy for a live task group.
///
/// # Safety
/// The group, its bandwidth object and any supplied task must remain live. The caller
/// must preserve the native cgroup/scheduler synchronization for policy reads and task
/// attachment decisions.
pub unsafe extern "C" fn sched_rt_can_attach(tg: *mut b::task_group, tsk: *mut b::task_struct) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if b::rust_rt_group_sched_enabled() && b::rust_rt_rt_task(tsk) && (*tg).rt_bandwidth.rt_runtime == 0 { return 0; }
        1
    }
}
#[cfg(CONFIG_SYSCTL)]
/// Validates the shared RT bandwidth limits.
///
/// # Safety
/// The caller must serialize the sysctl values with the RT handler and
/// scheduler-domain mutexes. The native constraints mutex is acquired here when group
/// validation is enabled.
unsafe fn sched_rt_global_validate() -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if sysctl_sched_rt_runtime as u64 != b::RUST_RT_RUNTIME_INF as u64 &&
            (sysctl_sched_rt_runtime > sysctl_sched_rt_period ||
             (sysctl_sched_rt_runtime as u64).wrapping_mul(b::RUST_RT_NSEC_PER_USEC as u64) > MAX_RT_RUNTIME) {
            return -(b::RUST_RT_EINVAL as c_int);
        }
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            if !b::rust_rt_group_sched_enabled() { return 0; }
            b::rust_rt_mutex_lock(b::rust_rt_constraints_mutex());
            let ret = __rt_schedulable(null_mut(), 0, 0);
            b::rust_rt_mutex_unlock(b::rust_rt_constraints_mutex());
            return ret;
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { 0 }
    }
}
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
/// Handles a native RT scheduling sysctl transaction.
///
/// # Safety
/// The sysctl core must supply the live matching table, buffer, length and
/// file-position pointers according to proc_dointvec conventions. The caller must
/// permit native sleeping locks and preserve the sysctl table lifetime until the
/// transaction and rollback complete.
pub unsafe extern "C" fn sched_rt_handler(table: *const b::ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut b::loff_t) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_mutex_lock(b::rust_rt_handler_mutex());
        b::sched_domains_mutex_lock();
        let old_period = sysctl_sched_rt_period;
        let old_runtime = sysctl_sched_rt_runtime;
        let mut ret = b::proc_dointvec_minmax(table, write, buffer, lenp, ppos);
        if ret == 0 && write != 0 {
            ret = sched_rt_global_validate();
            if ret == 0 { ret = b::sched_dl_global_validate(); }
            if ret != 0 { sysctl_sched_rt_period = old_period; sysctl_sched_rt_runtime = old_runtime; }
            else { b::sched_dl_do_global(); }
        }
        b::sched_domains_mutex_unlock();
        b::rust_rt_mutex_unlock(b::rust_rt_handler_mutex());
        // Unconditional, including read requests and failed writes, exactly as C.
        b::rebuild_sched_domains();
        ret
    }
}
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
/// Handles a native RT scheduling sysctl transaction.
///
/// # Safety
/// The sysctl core must supply the live matching table, buffer, length and
/// file-position pointers according to proc_dointvec conventions. The caller must
/// permit native sleeping locks and preserve the sysctl table lifetime until the
/// transaction and rollback complete.
pub unsafe extern "C" fn sched_rr_handler(table: *const b::ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut b::loff_t) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_mutex_lock(b::rust_rt_rr_handler_mutex());
        let ret = b::proc_dointvec(table, write, buffer, lenp, ppos);
        if ret == 0 && write != 0 {
            sched_rr_timeslice = if rust_rt_sysctl_sched_rr_timeslice <= 0 { b::RUST_RT_RR_TIMESLICE as c_int }
                else { b::rust_rt_msecs_to_jiffies(rust_rt_sysctl_sched_rr_timeslice as c_uint) as c_int };
            if rust_rt_sysctl_sched_rr_timeslice <= 0 {
                rust_rt_sysctl_sched_rr_timeslice = b::rust_rt_jiffies_to_msecs(b::RUST_RT_RR_TIMESLICE as c_ulong) as c_int;
            }
        }
        b::rust_rt_mutex_unlock(b::rust_rt_rr_handler_mutex());
        ret
    }
}
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[link_section = ".init.text"]
/// Registers the native RT sysctl table during initialization.
///
/// # Safety
/// Invoke only during the native initialization phase, once, with the sysctl core
/// available and before this function's init text is discarded.
pub unsafe extern "C" fn sched_rt_sysctl_init() -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_register_sysctl_init();
        0
    }
}
