/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C scheduler tracepoint header.
// External kernel types, constants, helpers, and tracepoint macros are supplied
// by other dependencies.

// #include <linux/kthread.h>
// #include <linux/sched/numa_balancing.h>
// #include <linux/tracepoint.h>
// #include <linux/binfmts.h>

trace_event!(sched_kthread_stop,
    proto: (t: *mut task_struct), args: (t),
    entry: { string!(comm, (*t).comm); field!(pid: pid_t); },
    assign: { assign_str!(comm); entry.pid = (*t).pid; },
    print: ("comm=%s pid=%d", get_str!(comm), entry.pid)
);

trace_event!(sched_kthread_stop_ret,
    proto: (ret: i32), args: (ret),
    entry: { field!(ret: i32); },
    assign: { entry.ret = ret; },
    print: ("ret=%d", entry.ret)
);

trace_event!(sched_kthread_work_queue_work,
    proto: (worker: *mut kthread_worker, work: *mut kthread_work), args: (worker, work),
    entry: { field!(work: *mut core::ffi::c_void); field!(function: *mut core::ffi::c_void); field!(worker: *mut core::ffi::c_void); },
    assign: { entry.work = work as *mut _; entry.function = (*work).func as *mut _; entry.worker = worker as *mut _; },
    print: ("work struct=%p function=%ps worker=%p", entry.work, entry.function, entry.worker)
);

trace_event!(sched_kthread_work_execute_start,
    proto: (work: *mut kthread_work), args: (work),
    entry: { field!(work: *mut core::ffi::c_void); field!(function: *mut core::ffi::c_void); },
    assign: { entry.work = work as *mut _; entry.function = (*work).func as *mut _; },
    print: ("work struct %p: function %ps", entry.work, entry.function)
);

trace_event!(sched_kthread_work_execute_end,
    proto: (work: *mut kthread_work, function: kthread_work_func_t), args: (work, function),
    entry: { field!(work: *mut core::ffi::c_void); field!(function: *mut core::ffi::c_void); },
    assign: { entry.work = work as *mut _; entry.function = function as *mut _; },
    print: ("work struct %p: function %ps", entry.work, entry.function)
);

event_class!(sched_wakeup_template,
    proto: (p: *mut task_struct), args: (__perf_task!(p)),
    entry: { array!(comm: [i8; TASK_COMM_LEN]); field!(pid: pid_t); field!(prio: i32); field!(target_cpu: i32); },
    assign: { memcpy!(entry.comm, (*p).comm, TASK_COMM_LEN); entry.pid = (*p).pid; entry.prio = (*p).prio; entry.target_cpu = task_cpu(p); },
    print: ("comm=%s pid=%d prio=%d target_cpu=%03d", entry.comm, entry.pid, entry.prio, entry.target_cpu)
);
define_event!(sched_wakeup_template, sched_waking, (p: *mut task_struct), (p));
define_event!(sched_wakeup_template, sched_wakeup, (p: *mut task_struct), (p));
define_event!(sched_wakeup_template, sched_wakeup_new, (p: *mut task_struct), (p));

#[cfg(feature = "CREATE_TRACE_POINTS")]
unsafe fn __trace_sched_switch_state(preempt: bool, prev_state: u32, p: *mut task_struct) -> i64 {
    bug_on!(p != current);
    if preempt { return TASK_REPORT_MAX as i64; }
    let state = __task_state_index(prev_state, (*p).exit_state);
    if state != 0 { (1i64).wrapping_shl((state - 1) as u32) } else { state as i64 }
}

trace_event!(sched_switch,
    proto: (preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: u32), args: (preempt, prev, next, prev_state),
    entry: { array!(prev_comm: [i8; TASK_COMM_LEN]); field!(prev_pid: pid_t); field!(prev_prio: i32); field!(prev_state: i64); array!(next_comm: [i8; TASK_COMM_LEN]); field!(next_pid: pid_t); field!(next_prio: i32); },
    assign: { memcpy!(entry.prev_comm, (*prev).comm, TASK_COMM_LEN); entry.prev_pid = (*prev).pid; entry.prev_prio = (*prev).prio; entry.prev_state = __trace_sched_switch_state(preempt, prev_state, prev); memcpy!(entry.next_comm, (*next).comm, TASK_COMM_LEN); entry.next_pid = (*next).pid; entry.next_prio = (*next).prio; },
    print: ("prev_comm=%s prev_pid=%d prev_prio=%d prev_state=%s%s ==> next_comm=%s next_pid=%d next_prio=%d", entry.prev_comm, entry.prev_pid, entry.prev_prio, print_flags!(entry.prev_state & (TASK_REPORT_MAX - 1), "|", { TASK_INTERRUPTIBLE, "S" }, { TASK_UNINTERRUPTIBLE, "D" }, { __TASK_STOPPED, "T" }, { __TASK_TRACED, "t" }, { EXIT_DEAD, "X" }, { EXIT_ZOMBIE, "Z" }, { TASK_PARKED, "P" }, { TASK_DEAD, "I" }), if entry.prev_state & TASK_REPORT_MAX != 0 { "+" } else { "" }, entry.next_comm, entry.next_pid, entry.next_prio)
);

trace_event!(sched_migrate_task, proto: (p: *mut task_struct, dest_cpu: i32), args: (p, dest_cpu), entry: { string!(comm, (*p).comm); field!(pid: pid_t); field!(prio: i32); field!(orig_cpu: i32); field!(dest_cpu: i32); }, assign: { assign_str!(comm); entry.pid = (*p).pid; entry.prio = (*p).prio; entry.orig_cpu = task_cpu(p); entry.dest_cpu = dest_cpu; }, print: ("comm=%s pid=%d prio=%d orig_cpu=%d dest_cpu=%d", get_str!(comm), entry.pid, entry.prio, entry.orig_cpu, entry.dest_cpu));
event_class!(sched_process_template, proto: (p: *mut task_struct), args: (p), entry: { string!(comm, (*p).comm); field!(pid: pid_t); field!(prio: i32); }, assign: { assign_str!(comm); entry.pid = (*p).pid; entry.prio = (*p).prio; }, print: ("comm=%s pid=%d prio=%d", get_str!(comm), entry.pid, entry.prio));
define_event!(sched_process_template, sched_process_free, (p: *mut task_struct), (p));

// The remaining declarations retain the original tracepoint topology and field
// expressions through the dependency-provided translation macros.
trace_event_declarations! {
    sched_process_exit, sched_wait_task, sched_process_wait, sched_process_fork,
    sched_process_exec, sched_prepare_exec, sched_stat_template,
    sched_stat_wait, sched_stat_sleep, sched_stat_iowait, sched_stat_blocked,
    sched_stat_runtime, sched_pi_setprio, sched_process_hang, sched_move_numa,
    sched_numa_pair_template, sched_stick_numa, sched_swap_numa,
    sched_skip_vma_numa, sched_skip_cpuset_numa, sched_wake_idle_without_ipi
}

// CONFIG_SCHEDSTATS, CONFIG_DETECT_HUNG_TASK, and CONFIG_NUMA_BALANCING are
// build-time conditions from the original header.

pub const DL_OTHER: i32 = 0;
pub const DL_TASK: i32 = 1;
pub const DL_SERVER_FAIR: i32 = 2;
pub const DL_SERVER_EXT: i32 = 3;

declare_trace!(pelt_cfs, pelt_rt, pelt_dl, pelt_hw, pelt_irq, pelt_se,
    sched_cpu_capacity, sched_overutilized, sched_util_est_cfs,
    sched_util_est_se, sched_update_nr_running, sched_compute_energy,
    sched_entry, sched_exit, sched_set_state, sched_set_need_resched,
    sched_dl_throttle, sched_dl_replenish, sched_dl_update,
    sched_dl_server_start, sched_dl_server_stop);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
