/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/sched/signal.h. Included C dependencies are supplied elsewhere.

#[repr(C)]
pub struct sighand_struct { pub siglock: spinlock_t, pub count: refcount_t, pub signalfd_wqh: wait_queue_head_t, pub action: [k_sigaction; _NSIG as usize] }
#[repr(C)]
pub struct pacct_struct { pub ac_flag: i32, pub ac_exitcode: libc::c_long, pub ac_mem: libc::c_ulong, pub ac_utime: u64, pub ac_stime: u64, pub ac_minflt: libc::c_ulong, pub ac_majflt: libc::c_ulong }
#[repr(C)]
pub struct cpu_itimer { pub expires: u64, pub incr: u64 }
#[repr(C)]
pub struct task_cputime_atomic { pub utime: atomic64_t, pub stime: atomic64_t, pub sum_exec_runtime: atomic64_t }
#[macro_export]
macro_rules! INIT_CPUTIME_ATOMIC { () => { task_cputime_atomic { utime: ATOMIC64_INIT!(0), stime: ATOMIC64_INIT!(0), sum_exec_runtime: ATOMIC64_INIT!(0) } } }
#[repr(C)]
pub struct thread_group_cputimer { pub cputime_atomic: task_cputime_atomic }
#[repr(C)]
pub struct multiprocess_signals { pub signal: sigset_t, pub node: hlist_node }
#[repr(C)]
pub struct core_thread { pub task: *mut task_struct, pub next: *mut core_thread }
#[repr(C)]
pub struct core_state { pub nr_threads: atomic_t, pub dumper: core_thread, pub startup: completion }

#[repr(C)]
pub struct signal_struct {
    pub sigcnt: refcount_t, pub live: atomic_t, pub nr_threads: i32, pub quick_threads: i32,
    pub thread_head: list_head, pub wait_chldexit: wait_queue_head_t, pub curr_target: *mut task_struct,
    pub shared_pending: sigpending, pub multiprocess: hlist_head, pub group_exit_code: i32,
    pub notify_count: i32, pub group_exec_task: *mut task_struct, pub group_stop_count: i32,
    pub flags: u32, pub core_state: *mut core_state,
    pub is_child_subreaper: bool, pub has_child_subreaper: bool, pub autoreap: bool,
    #[cfg(CONFIG_POSIX_TIMERS)] pub timer_create_restore_ids: bool,
    #[cfg(CONFIG_POSIX_TIMERS)] pub next_posix_timer_id: atomic_t,
    #[cfg(CONFIG_POSIX_TIMERS)] pub posix_timers: hlist_head,
    #[cfg(CONFIG_POSIX_TIMERS)] pub ignored_posix_timers: hlist_head,
    #[cfg(CONFIG_POSIX_TIMERS)] pub real_timer: hrtimer,
    #[cfg(CONFIG_POSIX_TIMERS)] pub it_real_incr: ktime_t,
    #[cfg(CONFIG_POSIX_TIMERS)] pub it: [cpu_itimer; 2],
    #[cfg(CONFIG_POSIX_TIMERS)] pub cputimer: thread_group_cputimer,
    pub posix_cputimers: posix_cputimers, pub pids: [*mut pid; PIDTYPE_MAX as usize],
    #[cfg(CONFIG_NO_HZ_FULL)] pub tick_dep_mask: atomic_t,
    pub tty_old_pgrp: *mut pid, pub leader: i32, pub tty: *mut tty_struct,
    #[cfg(CONFIG_SCHED_AUTOGROUP)] pub autogroup: *mut autogroup,
    pub stats_lock: seqlock_t, pub utime: u64, pub stime: u64, pub cutime: u64, pub cstime: u64,
    pub gtime: u64, pub cgtime: u64, pub prev_cputime: prev_cputime,
    pub nvcsw: libc::c_ulong, pub nivcsw: libc::c_ulong, pub cnvcsw: libc::c_ulong, pub cnivcsw: libc::c_ulong,
    pub min_flt: libc::c_ulong, pub maj_flt: libc::c_ulong, pub cmin_flt: libc::c_ulong, pub cmaj_flt: libc::c_ulong,
    pub inblock: libc::c_ulong, pub oublock: libc::c_ulong, pub cinblock: libc::c_ulong, pub coublock: libc::c_ulong,
    pub maxrss: libc::c_ulong, pub cmaxrss: libc::c_ulong, pub ioac: task_io_accounting,
    pub sum_sched_runtime: libc::c_ulonglong, pub rlim: [rlimit; RLIM_NLIMITS as usize],
    #[cfg(CONFIG_BSD_PROCESS_ACCT)] pub pacct: pacct_struct,
    #[cfg(CONFIG_TASKSTATS)] pub stats: *mut taskstats,
    #[cfg(CONFIG_AUDIT)] pub audit_tty: u32,
    #[cfg(CONFIG_AUDIT)] pub tty_audit_buf: *mut tty_audit_buf,
    #[cfg(CONFIG_CGROUPS)] pub cgroup_threadgroup_rwsem: rw_semaphore,
    pub oom_flag_origin: bool, pub oom_score_adj: i16, pub oom_score_adj_min: i16, pub oom_mm: *mut mm_struct,
    pub cred_guard_mutex: mutex, pub exec_update_lock: rw_semaphore,
}

pub const SIGNAL_STOP_STOPPED: u32 = 0x00000001;
pub const SIGNAL_STOP_CONTINUED: u32 = 0x00000002;
pub const SIGNAL_GROUP_EXIT: u32 = 0x00000004;
pub const SIGNAL_CLD_STOPPED: u32 = 0x00000010;
pub const SIGNAL_CLD_CONTINUED: u32 = 0x00000020;
pub const SIGNAL_CLD_MASK: u32 = SIGNAL_CLD_STOPPED | SIGNAL_CLD_CONTINUED;
pub const SIGNAL_UNKILLABLE: u32 = 0x00000040;
pub const SIGNAL_STOP_MASK: u32 = SIGNAL_CLD_MASK | SIGNAL_STOP_STOPPED | SIGNAL_STOP_CONTINUED;

pub unsafe fn signal_set_stop_flags(sig: *mut signal_struct, flags: u32) { WARN_ON!((*sig).flags & SIGNAL_GROUP_EXIT != 0); (*sig).flags = ((*sig).flags & !SIGNAL_STOP_MASK) | flags; }

extern "C" {
    pub fn flush_signals(*mut task_struct); pub fn ignore_signals(*mut task_struct); pub fn flush_signal_handlers(*mut task_struct, i32);
    pub fn dequeue_signal(*mut sigset_t, *mut kernel_siginfo_t, *mut pid_type) -> i32;
    pub fn force_sig_fault_to_task(i32, i32, *mut libc::c_void, *mut task_struct) -> i32;
    pub fn force_sig_fault(i32, i32, *mut libc::c_void) -> i32;
    pub fn send_sig_fault(i32, i32, *mut libc::c_void, *mut task_struct) -> i32;
    pub fn force_sig_mceerr(i32, *mut libc::c_void, i16) -> i32; pub fn send_sig_mceerr(i32, *mut libc::c_void, i16, *mut task_struct) -> i32;
    pub fn force_sig_bnderr(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> i32;
    pub fn force_sig_pkuerr(*mut libc::c_void, u32) -> i32; pub fn send_sig_perf(*mut libc::c_void, u32, u64) -> i32;
    pub fn force_sig_ptrace_errno_trap(i32, *mut libc::c_void) -> i32; pub fn force_sig_fault_trapno(i32, i32, *mut libc::c_void, i32) -> i32;
    pub fn send_sig_fault_trapno(i32, i32, *mut libc::c_void, i32, *mut task_struct) -> i32; pub fn force_sig_seccomp(i32, i32, bool) -> i32;
    pub fn send_sig_info(i32, *mut kernel_siginfo, *mut task_struct) -> i32; pub fn force_sigsegv(i32); pub fn force_sig_info(*mut kernel_siginfo) -> i32;
    pub fn __kill_pgrp_info(i32, *mut kernel_siginfo, *mut pid) -> i32; pub fn kill_pid_info(i32, *mut kernel_siginfo, *mut pid) -> i32;
    pub fn kill_pid_usb_asyncio(i32, i32, sigval_t, *mut pid, *const cred) -> i32; pub fn kill_pgrp(*mut pid, i32, i32) -> i32; pub fn kill_pid(*mut pid, i32, i32) -> i32;
    pub fn do_notify_parent(*mut task_struct, i32) -> bool; pub fn __wake_up_parent(*mut task_struct, *mut task_struct); pub fn force_sig(i32); pub fn force_fatal_sig(i32); pub fn force_exit_sig(i32);
    pub fn send_sig(i32, *mut task_struct, i32) -> i32; pub fn zap_other_threads(*mut task_struct) -> i32; pub fn do_sigaction(i32, *mut k_sigaction, *mut k_sigaction) -> i32;
    pub fn recalc_sigpending(); pub fn calculate_sigpending(); pub fn signal_wake_up_state(*mut task_struct, u32); pub fn task_join_group_stop(*mut task_struct);
    pub fn set_user_sigmask(*const sigset_t, usize) -> i32; pub fn __cleanup_sighand(*mut sighand_struct); pub fn flush_itimer_signals();
    pub fn current_is_single_threaded() -> bool; pub fn walk_process_tree(*mut task_struct, proc_visitor, *mut libc::c_void);
    pub fn lock_task_sighand(*mut task_struct, *mut libc::c_ulong) -> *mut sighand_struct;
}

pub unsafe fn kernel_dequeue_signal() -> i32 { let task = current; let mut info = core::mem::zeroed(); let mut ty = core::mem::zeroed(); spin_lock_irq(&mut (*(*task).sighand).siglock); let r = dequeue_signal(&mut (*task).blocked, &mut info, &mut ty); spin_unlock_irq(&mut (*(*task).sighand).siglock); r }
pub unsafe fn kernel_signal_stop() { spin_lock_irq(&mut (*(*current).sighand).siglock); if (*current).jobctl & JOBCTL_STOP_DEQUEUED != 0 { (*current).jobctl |= JOBCTL_STOPPED; set_special_state(TASK_STOPPED); } spin_unlock_irq(&mut (*(*current).sighand).siglock); schedule(); }
pub unsafe fn clear_notify_signal() { clear_thread_flag(TIF_NOTIFY_SIGNAL); smp_mb__after_atomic(); }
pub unsafe fn __set_notify_signal(task: *mut task_struct) -> bool { !test_and_set_tsk_thread_flag(task, TIF_NOTIFY_SIGNAL) && !wake_up_state(task, TASK_INTERRUPTIBLE) }
pub unsafe fn set_notify_signal(task: *mut task_struct) { if __set_notify_signal(task) { kick_process(task); } }
pub unsafe fn restart_syscall() -> i32 { set_tsk_thread_flag(current, TIF_SIGPENDING); -ERESTARTNOINTR }
pub unsafe fn task_sigpending(p: *mut task_struct) -> i32 { unlikely(test_tsk_thread_flag(p, TIF_SIGPENDING)) as i32 }
pub unsafe fn signal_pending(p: *mut task_struct) -> i32 { if unlikely(test_tsk_thread_flag(p, TIF_NOTIFY_SIGNAL)) { 1 } else { task_sigpending(p) } }
pub unsafe fn __fatal_signal_pending(p: *mut task_struct) -> i32 { unlikely(sigismember(&mut (*p).pending.signal, SIGKILL)) as i32 }
pub unsafe fn fatal_signal_pending(p: *mut task_struct) -> i32 { (task_sigpending(p) != 0 && __fatal_signal_pending(p) != 0) as i32 }
pub unsafe fn signal_pending_state(state: u32, p: *mut task_struct) -> i32 { if state & (TASK_INTERRUPTIBLE | TASK_WAKEKILL) == 0 || signal_pending(p) == 0 { 0 } else { ((state & TASK_INTERRUPTIBLE != 0) || __fatal_signal_pending(p) != 0) as i32 } }
pub unsafe fn fault_signal_pending(fault_flags: vm_fault_t, regs: *mut pt_regs) -> bool { unlikely(fault_flags & VM_FAULT_RETRY != 0 && (fatal_signal_pending(current) != 0 || (user_mode(regs) && signal_pending(current) != 0))) }
pub unsafe fn signal_wake_up(t: *mut task_struct, fatal: bool) { let mut state = 0; if fatal && (*t).jobctl & JOBCTL_PTRACE_FROZEN == 0 { (*t).jobctl &= !(JOBCTL_STOPPED | JOBCTL_TRACED); state = TASK_WAKEKILL | __TASK_TRACED; } signal_wake_up_state(t, state); }
pub unsafe fn ptrace_signal_wake_up(t: *mut task_struct, resume: bool) { let mut state = 0; if resume { (*t).jobctl &= !JOBCTL_TRACED; state = __TASK_TRACED; } signal_wake_up_state(t, state); }
pub unsafe fn set_restore_sigmask() { set_thread_flag(TIF_RESTORE_SIGMASK); }
pub unsafe fn clear_tsk_restore_sigmask(task: *mut task_struct) { clear_tsk_thread_flag(task, TIF_RESTORE_SIGMASK); }
pub unsafe fn clear_restore_sigmask() { clear_thread_flag(TIF_RESTORE_SIGMASK); }
pub unsafe fn test_restore_sigmask() -> bool { test_thread_flag(TIF_RESTORE_SIGMASK) }
pub unsafe fn test_tsk_restore_sigmask(task: *mut task_struct) -> bool { test_tsk_thread_flag(task, TIF_RESTORE_SIGMASK) }
pub unsafe fn test_and_clear_restore_sigmask() -> bool { test_and_clear_thread_flag(TIF_RESTORE_SIGMASK) }
pub unsafe fn restore_saved_sigmask_unless(interrupted: bool) { if interrupted { WARN_ON!(signal_pending(current) == 0); } else { restore_saved_sigmask(); } }
pub unsafe fn restore_saved_sigmask() { if test_and_clear_restore_sigmask() { __set_current_blocked(&(*current).saved_sigmask); } }
pub unsafe fn sigmask_to_save() -> *mut sigset_t { if unlikely(test_restore_sigmask()) { &mut (*current).saved_sigmask } else { &mut (*current).blocked } }
pub unsafe fn kill_cad_pid(sig: i32, priv_: i32) -> i32 { kill_pid(cad_pid, sig, priv_) }
pub const SEND_SIG_NOINFO: *mut kernel_siginfo = core::ptr::null_mut(); pub const SEND_SIG_PRIV: *mut kernel_siginfo = 1 as *mut kernel_siginfo;

pub unsafe fn sas_ss_reset(p: *mut task_struct) { (*p).sas_ss_sp = 0; (*p).sas_ss_size = 0; (*p).sas_ss_flags = SS_DISABLE; }
pub unsafe fn __on_sig_stack(sp: libc::c_ulong) -> i32 { #[cfg(CONFIG_STACK_GROWSUP)] { return (sp >= (*current).sas_ss_sp && sp - (*current).sas_ss_sp < (*current).sas_ss_size) as i32; } #[cfg(not(CONFIG_STACK_GROWSUP))] { (sp > (*current).sas_ss_sp && sp - (*current).sas_ss_sp <= (*current).sas_ss_size) as i32 } }
pub unsafe fn on_sig_stack(sp: libc::c_ulong) -> i32 { if (*current).sas_ss_flags & SS_AUTODISARM != 0 { 0 } else { __on_sig_stack(sp) } }
pub unsafe fn sas_ss_flags(sp: libc::c_ulong) -> u32 { if (*current).sas_ss_size == 0 { SS_DISABLE } else if on_sig_stack(sp) != 0 { SS_ONSTACK } else { 0 } }
pub unsafe fn sigsp(sp: libc::c_ulong, ksig: *mut ksignal) -> libc::c_ulong { if unlikely((*ksig).ka.sa.sa_flags & SA_ONSTACK != 0) && sas_ss_flags(sp) == 0 { #[cfg(CONFIG_STACK_GROWSUP)] { return (*current).sas_ss_sp; } #[cfg(not(CONFIG_STACK_GROWSUP))] { return (*current).sas_ss_sp + (*current).sas_ss_size; } } sp }
pub unsafe fn task_pid_type(task: *mut task_struct, ty: pid_type) -> *mut pid { if ty == PIDTYPE_PID { task_pid(task) } else { (*(*task).signal).pids[ty as usize] } }
pub unsafe fn task_tgid(task: *mut task_struct) -> *mut pid { (*(*task).signal).pids[PIDTYPE_TGID as usize] }
pub unsafe fn task_pgrp(task: *mut task_struct) -> *mut pid { (*(*task).signal).pids[PIDTYPE_PGID as usize] }
pub unsafe fn task_session(task: *mut task_struct) -> *mut pid { (*(*task).signal).pids[PIDTYPE_SID as usize] }
pub unsafe fn get_nr_threads(task: *mut task_struct) -> i32 { (*(*task).signal).nr_threads }
pub unsafe fn thread_group_leader(p: *mut task_struct) -> bool { (*p).exit_signal >= 0 }
pub unsafe fn same_thread_group(p1: *mut task_struct, p2: *mut task_struct) -> bool { (*p1).signal == (*p2).signal }
pub unsafe fn __next_thread(p: *mut task_struct) -> *mut task_struct { list_next_or_null_rcu!(&(*(*p).signal).thread_head, &(*p).thread_node, task_struct, thread_node) }
pub unsafe fn next_thread(p: *mut task_struct) -> *mut task_struct { let n = __next_thread(p); if n.is_null() { (*p).group_leader } else { n } }
pub unsafe fn thread_group_empty(p: *mut task_struct) -> i32 { (thread_group_leader(p) && list_is_last(&(*p).thread_node, &(*(*p).signal).thread_head)) as i32 }
pub type proc_visitor = unsafe extern "C" fn(*mut task_struct, *mut libc::c_void) -> i32;
pub unsafe fn unlock_task_sighand(task: *mut task_struct, flags: *mut libc::c_ulong) { spin_unlock_irqrestore(&mut (*(*task).sighand).siglock, *flags); }
pub unsafe fn task_rlimit(task: *const task_struct, limit: u32) -> libc::c_ulong { READ_ONCE!((*(*task).signal).rlim[limit as usize].rlim_cur) }
pub unsafe fn task_rlimit_max(task: *const task_struct, limit: u32) -> libc::c_ulong { READ_ONCE!((*(*task).signal).rlim[limit as usize].rlim_max) }
pub unsafe fn rlimit(limit: u32) -> libc::c_ulong { task_rlimit(current, limit) }
pub unsafe fn rlimit_max(limit: u32) -> libc::c_ulong { task_rlimit_max(current, limit) }
pub unsafe fn lockdep_assert_task_sighand_held(_task: *mut task_struct) { }

#[macro_export] macro_rules! tasklist_empty { () => { list_empty(&init_task.tasks) } }
#[macro_export] macro_rules! next_task { ($p:expr) => { list_entry_rcu!((*$p).tasks.next, task_struct, tasks) } }
#[macro_export] macro_rules! while_each_thread { ($g:expr, $t:expr) => { while { $t = next_thread($t); $t != $g } } }
#[macro_export] macro_rules! delay_group_leader { ($p:expr) => { thread_group_leader($p) && !thread_group_empty($p) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
