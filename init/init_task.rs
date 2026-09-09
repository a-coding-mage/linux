// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied externally by the surrounding translation unit.

static mut init_signals: signal_struct = signal_struct {
    nr_threads: 1,
    thread_head: LIST_HEAD_INIT!(init_task.thread_node),
    wait_chldexit: __WAIT_QUEUE_HEAD_INITIALIZER!(init_signals.wait_chldexit),
    shared_pending: shared_pending_struct {
        list: LIST_HEAD_INIT!(init_signals.shared_pending.list),
        signal: [[0; _]; _],
    },
    multiprocess: HLIST_HEAD_INIT!(),
    rlim: INIT_RLIMITS,
    #[cfg(feature = "CONFIG_CGROUPS")]
    cgroup_threadgroup_rwsem: __RWSEM_INITIALIZER!(init_signals.cgroup_threadgroup_rwsem),
    cred_guard_mutex: __MUTEX_INITIALIZER!(init_signals.cred_guard_mutex),
    exec_update_lock: __RWSEM_INITIALIZER!(init_signals.exec_update_lock),
    #[cfg(feature = "CONFIG_POSIX_TIMERS")]
    posix_timers: HLIST_HEAD_INIT!(),
    #[cfg(feature = "CONFIG_POSIX_TIMERS")]
    ignored_posix_timers: HLIST_HEAD_INIT!(),
    #[cfg(feature = "CONFIG_POSIX_TIMERS")]
    cputimer: cputimer_struct { cputime_atomic: INIT_CPUTIME_ATOMIC },
    INIT_CPU_TIMERS!(init_signals)
    pids: [
        [PIDTYPE_PID]: &raw mut init_struct_pid,
        [PIDTYPE_TGID]: &raw mut init_struct_pid,
        [PIDTYPE_PGID]: &raw mut init_struct_pid,
        [PIDTYPE_SID]: &raw mut init_struct_pid,
    ],
    INIT_PREV_CPUTIME!(init_signals)
};

static mut init_sighand: sighand_struct = sighand_struct {
    count: REFCOUNT_INIT!(1),
    action: [[[SigAction { sa_handler: SIG_DFL }; _]; _]; _],
    siglock: __SPIN_LOCK_UNLOCKED!(init_sighand.siglock),
    signalfd_wqh: __WAIT_QUEUE_HEAD_INITIALIZER!(init_sighand.signalfd_wqh),
};

/* init to 2 - one for init_task, one to ensure it is never freed */
#[no_mangle]
pub static mut init_task_exec_state: task_exec_state = task_exec_state {
    count: REFCOUNT_INIT!(2),
    dumpable: TASK_DUMPABLE_OWNER,
    user_ns: &raw mut init_user_ns,
};

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
pub static mut init_shadow_call_stack: [u64; SCS_SIZE / core::mem::size_of::<u64>()] = {
    let mut value = [0; SCS_SIZE / core::mem::size_of::<u64>()];
    value[(SCS_SIZE / core::mem::size_of::<u64>()) - 1] = SCS_END_MAGIC;
    value
};

/* init to 2 - one for init_task, one to ensure it is never freed */
static mut init_groups: group_info = group_info { usage: REFCOUNT_INIT!(2) };

/*
 * The initial credentials for the initial task
 */
static mut init_cred: cred = cred {
    usage: ATOMIC_INIT!(4),
    uid: GLOBAL_ROOT_UID,
    gid: GLOBAL_ROOT_GID,
    suid: GLOBAL_ROOT_UID,
    sgid: GLOBAL_ROOT_GID,
    euid: GLOBAL_ROOT_UID,
    egid: GLOBAL_ROOT_GID,
    fsuid: GLOBAL_ROOT_UID,
    fsgid: GLOBAL_ROOT_GID,
    securebits: SECUREBITS_DEFAULT,
    cap_inheritable: CAP_EMPTY_SET,
    cap_permitted: CAP_FULL_SET,
    cap_effective: CAP_FULL_SET,
    cap_bset: CAP_FULL_SET,
    user: INIT_USER,
    user_ns: &raw mut init_user_ns,
    group_info: &raw mut init_groups,
    ucounts: &raw mut init_ucounts,
};

/*
 * Set up the first task table, touch at your own risk!. Base=0,
 * limit=0x1fffff (=2MB)
 */
#[repr(C, align( L1_CACHE_BYTES ))]
pub struct AlignedInitTask(pub task_struct);

#[no_mangle]
pub static mut init_task: task_struct = task_struct {
    #[cfg(feature = "CONFIG_THREAD_INFO_IN_TASK")]
    thread_info: INIT_THREAD_INFO!(init_task),
    #[cfg(feature = "CONFIG_THREAD_INFO_IN_TASK")]
    stack_refcount: REFCOUNT_INIT!(1),
    __state: 0,
    stack: init_stack,
    usage: REFCOUNT_INIT!(2),
    flags: PF_KTHREAD,
    prio: MAX_PRIO - 20,
    static_prio: MAX_PRIO - 20,
    normal_prio: MAX_PRIO - 20,
    policy: SCHED_NORMAL,
    cpus_ptr: &raw mut init_task.cpus_mask,
    user_cpus_ptr: core::ptr::null_mut(),
    cpus_mask: CPU_MASK_ALL,
    max_allowed_capacity: SCHED_CAPACITY_SCALE,
    nr_cpus_allowed: NR_CPUS,
    mm: core::ptr::null_mut(),
    active_mm: &raw mut init_mm,
    exec_state: &raw mut init_task_exec_state,
    restart_block: restart_block { fn_: do_no_restart_syscall },
    se: sched_entity { group_node: LIST_HEAD_INIT!(init_task.se.group_node) },
    rt: sched_rt_entity {
        run_list: LIST_HEAD_INIT!(init_task.rt.run_list),
        time_slice: RR_TIMESLICE,
    },
    tasks: LIST_HEAD_INIT!(init_task.tasks),
    #[cfg(feature = "CONFIG_SMP")]
    pushable_tasks: PLIST_NODE_INIT!(init_task.pushable_tasks, MAX_PRIO),
    #[cfg(feature = "CONFIG_CGROUP_SCHED")]
    sched_task_group: &raw mut root_task_group,
    #[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
    scx: sched_ext_entity {
        dsq_list: dsq_list { node: LIST_HEAD_INIT!(init_task.scx.dsq_list.node) },
        sticky_cpu: -1,
        holding_cpu: -1,
        runnable_cpu: -1,
        runnable_node: LIST_HEAD_INIT!(init_task.scx.runnable_node),
        runnable_at: INITIAL_JIFFIES,
        ddsp_dsq_id: SCX_DSQ_INVALID,
        slice: SCX_SLICE_DFL,
    },
    ptraced: LIST_HEAD_INIT!(init_task.ptraced),
    ptrace_entry: LIST_HEAD_INIT!(init_task.ptrace_entry),
    real_parent: &raw mut init_task,
    parent: &raw mut init_task,
    children: LIST_HEAD_INIT!(init_task.children),
    sibling: LIST_HEAD_INIT!(init_task.sibling),
    group_leader: &raw mut init_task,
    real_cred: RCU_POINTER_INITIALIZER!(real_cred, &raw mut init_cred),
    cred: RCU_POINTER_INITIALIZER!(cred, &raw mut init_cred),
    comm: INIT_TASK_COMM,
    thread: INIT_THREAD,
    real_fs: &raw mut init_fs,
    fs: &raw mut init_fs,
    files: &raw mut init_files,
    #[cfg(feature = "CONFIG_IO_URING")]
    io_uring: core::ptr::null_mut(),
    signal: &raw mut init_signals,
    sighand: &raw mut init_sighand,
    nsproxy: &raw mut init_nsproxy,
    pending: pending_struct {
        list: LIST_HEAD_INIT!(init_task.pending.list),
        signal: [[0; _]; _],
    },
    blocked: [[0; _]; _],
    alloc_lock: __SPIN_LOCK_UNLOCKED!(init_task.alloc_lock),
    journal_info: core::ptr::null_mut(),
    INIT_CPU_TIMERS!(init_task)
    pi_lock: __RAW_SPIN_LOCK_UNLOCKED!(init_task.pi_lock),
    blocked_lock: __RAW_SPIN_LOCK_UNLOCKED!(init_task.blocked_lock),
    timer_slack_ns: 50000, /* 50 usec default slack */
    thread_pid: &raw mut init_struct_pid,
    thread_node: LIST_HEAD_INIT!(init_signals.thread_head),
    #[cfg(feature = "CONFIG_AUDIT")]
    loginuid: INVALID_UID,
    #[cfg(feature = "CONFIG_AUDIT")]
    sessionid: AUDIT_SID_UNSET,
    #[cfg(feature = "CONFIG_PERF_EVENTS")]
    perf_event_mutex: __MUTEX_INITIALIZER!(init_task.perf_event_mutex),
    #[cfg(feature = "CONFIG_PERF_EVENTS")]
    perf_event_list: LIST_HEAD_INIT!(init_task.perf_event_list),
    #[cfg(feature = "CONFIG_PREEMPT_RCU")]
    rcu_read_lock_nesting: 0,
    #[cfg(feature = "CONFIG_PREEMPT_RCU")]
    rcu_read_unlock_special: rcu_read_unlock_special { s: 0 },
    #[cfg(feature = "CONFIG_PREEMPT_RCU")]
    rcu_node_entry: LIST_HEAD_INIT!(init_task.rcu_node_entry),
    #[cfg(feature = "CONFIG_PREEMPT_RCU")]
    rcu_blocked_node: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_TASKS_RCU")]
    rcu_tasks_holdout: false,
    #[cfg(feature = "CONFIG_TASKS_RCU")]
    rcu_tasks_holdout_list: LIST_HEAD_INIT!(init_task.rcu_tasks_holdout_list),
    #[cfg(feature = "CONFIG_TASKS_RCU")]
    rcu_tasks_idle_cpu: -1,
    #[cfg(feature = "CONFIG_TASKS_RCU")]
    rcu_tasks_exit_list: LIST_HEAD_INIT!(init_task.rcu_tasks_exit_list),
    #[cfg(feature = "CONFIG_TASKS_TRACE_RCU")]
    trc_reader_nesting: 0,
    #[cfg(feature = "CONFIG_CPUSETS")]
    mems_allowed_seq: SEQCNT_SPINLOCK_ZERO!(init_task.mems_allowed_seq, &raw mut init_task.alloc_lock),
    blocked_donor: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_RT_MUTEXES")]
    pi_waiters: RB_ROOT_CACHED,
    #[cfg(feature = "CONFIG_RT_MUTEXES")]
    pi_top_task: core::ptr::null_mut(),
    INIT_PREV_CPUTIME!(init_task)
    #[cfg(feature = "CONFIG_VIRT_CPU_ACCOUNTING_GEN")]
    vtime: vtime_struct { seqcount: SEQCNT_ZERO!(init_task.vtime_seqcount), starttime: 0, state: VTIME_SYS },
    #[cfg(feature = "CONFIG_NUMA_BALANCING")]
    numa_preferred_nid: NUMA_NO_NODE,
    #[cfg(feature = "CONFIG_NUMA_BALANCING")]
    numa_group: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_NUMA_BALANCING")]
    numa_faults: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_SCHED_CACHE")]
    preferred_llc: -1,
    #[cfg(feature = "CONFIG_SCHED_CACHE")]
    pref_llc_queued: 0,
    #[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
    kasan_depth: 1,
    #[cfg(feature = "CONFIG_KCSAN")]
    kcsan_ctx: kcsan_ctx { scoped_accesses: (LIST_POISON1, core::ptr::null_mut()) },
    #[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
    softirqs_enabled: 1,
    #[cfg(feature = "CONFIG_LOCKDEP")]
    lockdep_depth: 0, /* no locks held yet */
    #[cfg(feature = "CONFIG_LOCKDEP")]
    curr_chain_key: INITIAL_CHAIN_KEY,
    #[cfg(feature = "CONFIG_LOCKDEP")]
    lockdep_recursion: 0,
    #[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
    ret_stack: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
    tracing_graph_pause: ATOMIC_INIT!(0),
    #[cfg(all(feature = "CONFIG_TRACING", feature = "CONFIG_PREEMPTION"))]
    trace_recursion: 0,
    #[cfg(feature = "CONFIG_LIVEPATCH")]
    patch_state: KLP_TRANSITION_IDLE,
    #[cfg(feature = "CONFIG_SECURITY")]
    security: core::ptr::null_mut(),
    #[cfg(feature = "CONFIG_SECCOMP_FILTER")]
    seccomp: seccomp_struct { filter_count: ATOMIC_INIT!(0) },
    #[cfg(feature = "CONFIG_SCHED_MM_CID")]
    mm_cid: mm_cid_struct { cid: MM_CID_UNSET },
};

EXPORT_SYMBOL!(init_task);

/*
 * Initial thread structure. Alignment of this is handled by a special
 * linker map entry.
 */
#[cfg(not(feature = "CONFIG_THREAD_INFO_IN_TASK"))]
pub static mut init_thread_info: thread_info = INIT_THREAD_INFO!(init_task);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
