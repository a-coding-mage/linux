// SPDX-License-Identifier: GPL-2.0
// Dependencies: linux/init_task.h, linux/export.h, linux/mqueue.h,
// linux/sched.h, linux/sched/sysctl.h, linux/sched/rt.h, linux/sched/task.h,
// linux/sched/ext.h, linux/sched/exec_state.h, linux/user_namespace.h,
// linux/init.h, linux/fs.h, linux/mm.h, linux/audit.h, linux/numa.h,
// linux/scs.h, linux/plist.h, linux/uaccess.h.
//
// The C objects use designated initializers, which zero every field they do
// not name.  Each Rust initializer starts from an all-zero value and assigns
// exactly the fields the C initializer names, in the same order and under
// the same configuration conditions.

use core::mem::zeroed;

static mut init_signals: signal_struct = {
    // SAFETY: signal_struct is plain C data; C zero-fills unnamed fields.
    let mut s: signal_struct = unsafe { zeroed() };
    // SAFETY (all `&raw mut` below): only addresses of statics are taken.
    s.nr_threads = 1;
    s.thread_head = LIST_HEAD_INIT!(init_task.thread_node);
    s.wait_chldexit = __WAIT_QUEUE_HEAD_INITIALIZER!(init_signals.wait_chldexit);
    s.shared_pending.list = LIST_HEAD_INIT!(init_signals.shared_pending.list);
    s.multiprocess = HLIST_HEAD_INIT!();
    s.rlim = INIT_RLIMITS!();
    #[cfg(CONFIG_CGROUPS)]
    {
        s.cgroup_threadgroup_rwsem = __RWSEM_INITIALIZER!(init_signals.cgroup_threadgroup_rwsem);
    }
    s.cred_guard_mutex = __MUTEX_INITIALIZER!(init_signals.cred_guard_mutex);
    s.exec_update_lock = __RWSEM_INITIALIZER!(init_signals.exec_update_lock);
    #[cfg(CONFIG_POSIX_TIMERS)]
    {
        s.posix_timers = HLIST_HEAD_INIT!();
        s.ignored_posix_timers = HLIST_HEAD_INIT!();
        s.cputimer.cputime_atomic = INIT_CPUTIME_ATOMIC!();
        // INIT_CPU_TIMERS(init_signals)
        s.posix_cputimers.bases[0].nextevt = u64::MAX;
        s.posix_cputimers.bases[1].nextevt = u64::MAX;
        s.posix_cputimers.bases[2].nextevt = u64::MAX;
    }
    s.pids[PIDTYPE_PID as usize] = unsafe { &raw mut init_struct_pid };
    s.pids[PIDTYPE_TGID as usize] = unsafe { &raw mut init_struct_pid };
    s.pids[PIDTYPE_PGID as usize] = unsafe { &raw mut init_struct_pid };
    s.pids[PIDTYPE_SID as usize] = unsafe { &raw mut init_struct_pid };
    // INIT_PREV_CPUTIME(init_signals)
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    {
        s.prev_cputime.lock = __RAW_SPIN_LOCK_UNLOCKED!(init_signals.prev_cputime.lock);
    }
    s
};

static mut init_sighand: sighand_struct = {
    // SAFETY: sighand_struct is plain C data; C zero-fills unnamed fields.
    let mut s: sighand_struct = unsafe { zeroed() };
    s.count = REFCOUNT_INIT!(1);
    // { { { .sa_handler = SIG_DFL } } }: only action[0] is named; SIG_DFL is 0.
    s.action[0].sa.sa_handler = SIG_DFL;
    s.siglock = __SPIN_LOCK_UNLOCKED!(init_sighand.siglock);
    s.signalfd_wqh = __WAIT_QUEUE_HEAD_INITIALIZER!(init_sighand.signalfd_wqh);
    s
};

/* init to 2 - one for init_task, one to ensure it is never freed */
#[no_mangle]
pub static mut init_task_exec_state: task_exec_state = {
    // SAFETY: task_exec_state is plain C data; C zero-fills unnamed fields.
    let mut s: task_exec_state = unsafe { zeroed() };
    s.count = REFCOUNT_INIT!(2);
    s.dumpable = TASK_DUMPABLE_OWNER as _;
    s.user_ns = unsafe { &raw mut init_user_ns };
    s
};

#[cfg(CONFIG_SHADOW_CALL_STACK)]
#[no_mangle]
pub static mut init_shadow_call_stack: [kernel::ffi::c_ulong; SCS_SIZE / core::mem::size_of::<kernel::ffi::c_long>()] = {
    let mut v = [0; SCS_SIZE / core::mem::size_of::<kernel::ffi::c_long>()];
    v[(SCS_SIZE / core::mem::size_of::<kernel::ffi::c_long>()) - 1] = SCS_END_MAGIC;
    v
};

/* init to 2 - one for init_task, one to ensure it is never freed */
static mut init_groups: group_info = {
    // SAFETY: group_info is plain C data; C zero-fills unnamed fields.
    let mut g: group_info = unsafe { zeroed() };
    g.usage = REFCOUNT_INIT!(2);
    g
};

/*
 * The initial credentials for the initial task
 */
static mut init_cred: cred = {
    // SAFETY: cred is plain C data; C zero-fills unnamed fields.
    let mut c: cred = unsafe { zeroed() };
    c.usage = ATOMIC_INIT!(4);
    c.uid = GLOBAL_ROOT_UID;
    c.gid = GLOBAL_ROOT_GID;
    c.suid = GLOBAL_ROOT_UID;
    c.sgid = GLOBAL_ROOT_GID;
    c.euid = GLOBAL_ROOT_UID;
    c.egid = GLOBAL_ROOT_GID;
    c.fsuid = GLOBAL_ROOT_UID;
    c.fsgid = GLOBAL_ROOT_GID;
    c.securebits = SECUREBITS_DEFAULT;
    c.cap_inheritable = CAP_EMPTY_SET;
    c.cap_permitted = CAP_FULL_SET;
    c.cap_effective = CAP_FULL_SET;
    c.cap_bset = CAP_FULL_SET;
    c.user = INIT_USER!();
    c.user_ns = unsafe { &raw mut init_user_ns };
    c.group_info = unsafe { &raw mut init_groups };
    c.ucounts = unsafe { &raw mut init_ucounts };
    c
};

/// `INIT_TASK_COMM`, copied into the fixed-size `comm` array.
const fn init_task_comm() -> [kernel::ffi::c_char; TASK_COMM_LEN as usize] {
    let name = b"swapper";
    let mut comm = [0 as kernel::ffi::c_char; TASK_COMM_LEN as usize];
    let mut i = 0;
    while i < name.len() {
        comm[i] = name[i] as kernel::ffi::c_char;
        i += 1;
    }
    comm
}

/*
 * Set up the first task table, touch at your own risk!. Base=0,
 * limit=0x1fffff (=2MB)
 */
/// `__aligned(L1_CACHE_BYTES)`: the C object is cache-line aligned.
#[repr(C, align(64))]
pub struct __init_task_aligned(pub task_struct);
const _: () = assert!(core::mem::align_of::<__init_task_aligned>() >= L1_CACHE_BYTES as usize);

#[export_name = "init_task"]
pub static mut init_task_storage: __init_task_aligned = __init_task_aligned({
    // SAFETY: task_struct is plain C data; C zero-fills unnamed fields.
    let mut t: task_struct = unsafe { zeroed() };
    #[cfg(CONFIG_THREAD_INFO_IN_TASK)]
    {
        t.thread_info = INIT_THREAD_INFO!(init_task);
        t.stack_refcount = REFCOUNT_INIT!(1);
    }
    t.__state = 0;
    t.stack = unsafe { &raw mut init_stack }.cast();
    t.usage = REFCOUNT_INIT!(2);
    t.flags = PF_KTHREAD as _;
    t.prio = (MAX_PRIO - 20) as _;
    t.static_prio = (MAX_PRIO - 20) as _;
    t.normal_prio = (MAX_PRIO - 20) as _;
    t.policy = SCHED_NORMAL as _;
    t.cpus_ptr = unsafe { &raw mut init_task.cpus_mask };
    t.user_cpus_ptr = core::ptr::null_mut();
    t.cpus_mask = CPU_MASK_ALL!();
    t.max_allowed_capacity = SCHED_CAPACITY_SCALE as _;
    t.nr_cpus_allowed = NR_CPUS as _;
    t.mm = core::ptr::null_mut();
    t.active_mm = unsafe { &raw mut init_mm };
    t.exec_state = unsafe { &raw mut init_task_exec_state };
    t.restart_block.fn_ = Some(do_no_restart_syscall);
    t.se.group_node = LIST_HEAD_INIT!(init_task.se.group_node);
    t.rt.run_list = LIST_HEAD_INIT!(init_task.rt.run_list);
    t.rt.time_slice = RR_TIMESLICE as _;
    t.tasks = LIST_HEAD_INIT!(init_task.tasks);
    #[cfg(CONFIG_SMP)]
    {
        t.pushable_tasks = PLIST_NODE_INIT!(init_task.pushable_tasks, MAX_PRIO);
    }
    #[cfg(CONFIG_CGROUP_SCHED)]
    {
        t.sched_task_group = unsafe { &raw mut root_task_group };
    }
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    {
        t.scx.dsq_list.node = LIST_HEAD_INIT!(init_task.scx.dsq_list.node);
        t.scx.sticky_cpu = -1;
        t.scx.holding_cpu = -1;
        t.scx.runnable_cpu = -1;
        t.scx.runnable_node = LIST_HEAD_INIT!(init_task.scx.runnable_node);
        t.scx.runnable_at = INITIAL_JIFFIES;
        t.scx.ddsp_dsq_id = SCX_DSQ_INVALID as _;
        t.scx.slice = SCX_SLICE_DFL as _;
    }
    t.ptraced = LIST_HEAD_INIT!(init_task.ptraced);
    t.ptrace_entry = LIST_HEAD_INIT!(init_task.ptrace_entry);
    t.real_parent = unsafe { &raw mut init_task };
    t.parent = unsafe { &raw mut init_task };
    t.children = LIST_HEAD_INIT!(init_task.children);
    t.sibling = LIST_HEAD_INIT!(init_task.sibling);
    t.group_leader = unsafe { &raw mut init_task };
    // RCU_POINTER_INITIALIZER(real_cred / cred, &init_cred)
    t.real_cred = unsafe { &raw const init_cred };
    t.cred = unsafe { &raw const init_cred };
    t.comm = init_task_comm();
    t.thread = INIT_THREAD!();
    t.real_fs = unsafe { &raw mut init_fs };
    t.fs = unsafe { &raw mut init_fs };
    t.files = unsafe { &raw mut init_files };
    #[cfg(CONFIG_IO_URING)]
    {
        t.io_uring = core::ptr::null_mut();
    }
    t.signal = unsafe { &raw mut init_signals };
    t.sighand = unsafe { &raw mut init_sighand };
    t.nsproxy = unsafe { &raw mut init_nsproxy };
    t.pending.list = LIST_HEAD_INIT!(init_task.pending.list);
    t.alloc_lock = __SPIN_LOCK_UNLOCKED!(init_task.alloc_lock);
    t.journal_info = core::ptr::null_mut();
    // INIT_CPU_TIMERS(init_task)
    #[cfg(CONFIG_POSIX_TIMERS)]
    {
        t.posix_cputimers.bases[0].nextevt = u64::MAX;
        t.posix_cputimers.bases[1].nextevt = u64::MAX;
        t.posix_cputimers.bases[2].nextevt = u64::MAX;
    }
    t.pi_lock = __RAW_SPIN_LOCK_UNLOCKED!(init_task.pi_lock);
    t.blocked_lock = __RAW_SPIN_LOCK_UNLOCKED!(init_task.blocked_lock);
    t.timer_slack_ns = 50000; /* 50 usec default slack */
    t.thread_pid = unsafe { &raw mut init_struct_pid };
    t.thread_node = LIST_HEAD_INIT!(init_signals.thread_head);
    #[cfg(CONFIG_AUDIT)]
    {
        t.loginuid = INVALID_UID;
        t.sessionid = AUDIT_SID_UNSET as _;
    }
    #[cfg(CONFIG_PERF_EVENTS)]
    {
        t.perf_event_mutex = __MUTEX_INITIALIZER!(init_task.perf_event_mutex);
        t.perf_event_list = LIST_HEAD_INIT!(init_task.perf_event_list);
    }
    #[cfg(CONFIG_PREEMPT_RCU)]
    {
        t.rcu_read_lock_nesting = 0;
        t.rcu_read_unlock_special.s = 0;
        t.rcu_node_entry = LIST_HEAD_INIT!(init_task.rcu_node_entry);
        t.rcu_blocked_node = core::ptr::null_mut();
    }
    #[cfg(CONFIG_TASKS_RCU)]
    {
        t.rcu_tasks_holdout = false;
        t.rcu_tasks_holdout_list = LIST_HEAD_INIT!(init_task.rcu_tasks_holdout_list);
        t.rcu_tasks_idle_cpu = -1;
        t.rcu_tasks_exit_list = LIST_HEAD_INIT!(init_task.rcu_tasks_exit_list);
    }
    #[cfg(CONFIG_TASKS_TRACE_RCU)]
    {
        t.trc_reader_nesting = 0;
    }
    #[cfg(CONFIG_CPUSETS)]
    {
        t.mems_allowed_seq = SEQCNT_SPINLOCK_ZERO!(init_task.mems_allowed_seq, &raw mut init_task.alloc_lock);
    }
    t.blocked_donor = core::ptr::null_mut();
    #[cfg(CONFIG_RT_MUTEXES)]
    {
        t.pi_waiters = RB_ROOT_CACHED!();
        t.pi_top_task = core::ptr::null_mut();
    }
    // INIT_PREV_CPUTIME(init_task)
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    {
        t.prev_cputime.lock = __RAW_SPIN_LOCK_UNLOCKED!(init_task.prev_cputime.lock);
    }
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
    {
        t.vtime.seqcount = SEQCNT_ZERO!(init_task.vtime_seqcount);
        t.vtime.starttime = 0;
        t.vtime.state = VTIME_SYS as _;
    }
    #[cfg(CONFIG_NUMA_BALANCING)]
    {
        t.numa_preferred_nid = NUMA_NO_NODE;
        t.numa_group = core::ptr::null_mut();
        t.numa_faults = core::ptr::null_mut();
    }
    #[cfg(CONFIG_SCHED_CACHE)]
    {
        t.preferred_llc = -1;
        t.pref_llc_queued = 0;
    }
    #[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
    {
        t.kasan_depth = 1;
    }
    #[cfg(CONFIG_KCSAN)]
    {
        t.kcsan_ctx.scoped_accesses.next = LIST_POISON1 as _;
        t.kcsan_ctx.scoped_accesses.prev = core::ptr::null_mut();
    }
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    {
        t.softirqs_enabled = 1;
    }
    #[cfg(CONFIG_LOCKDEP)]
    {
        t.lockdep_depth = 0; /* no locks held yet */
        t.curr_chain_key = INITIAL_CHAIN_KEY;
        t.lockdep_recursion = 0;
    }
    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    {
        t.ret_stack = core::ptr::null_mut();
        t.tracing_graph_pause = ATOMIC_INIT!(0);
    }
    #[cfg(all(CONFIG_TRACING, CONFIG_PREEMPTION))]
    {
        t.trace_recursion = 0;
    }
    #[cfg(CONFIG_LIVEPATCH)]
    {
        t.patch_state = KLP_TRANSITION_IDLE as _;
    }
    #[cfg(CONFIG_SECURITY)]
    {
        t.security = core::ptr::null_mut();
    }
    #[cfg(CONFIG_SECCOMP_FILTER)]
    {
        t.seccomp.filter_count = ATOMIC_INIT!(0);
    }
    #[cfg(CONFIG_SCHED_MM_CID)]
    {
        t.mm_cid.cid = MM_CID_UNSET as _;
    }
    t
});
// EXPORT_SYMBOL(init_task);

extern "C" {
    /// `init_task` itself, as the rest of the kernel names it.
    #[link_name = "init_task"]
    pub static mut init_task: task_struct;
}

/*
 * Initial thread structure. Alignment of this is handled by a special
 * linker map entry.
 */
#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
#[no_mangle]
#[link_section = ".data..init_thread_info"]
pub static mut init_thread_info: thread_info = INIT_THREAD_INFO!(init_task);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
