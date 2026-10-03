// SPDX-License-Identifier: GPL-2.0-only
// kernel/fork.c:1828-2647. Publication transaction and exact reverse unwind.
#[no_mangle]
pub unsafe extern "C" fn rust_fork_set_tid_address(tidptr: *mut c_int) -> c_long {
    let t = current_task();
    (*t).clear_child_tid = tidptr;
    rust_fork_task_pid_vnr(t) as c_long
}
unsafe fn rt_mutex_init_task(p: *mut task_struct) {
    rust_fork_pi_lock_init(p);
    #[cfg(CONFIG_RT_MUTEXES)]
    {
        (*p).pi_waiters = zeroed();
        (*p).pi_top_task = null_mut();
        (*p).pi_blocked_on = null_mut();
    }
}
unsafe fn init_task_pid_links(t: *mut task_struct) {
    for kind in PIDTYPE_PID..PIDTYPE_MAX {
        init_hlist(addr_of_mut!((*t).pid_links[kind as usize]));
    }
}
unsafe fn init_task_pid(t: *mut task_struct, kind: pid_type, pid: *mut pid) {
    if kind == PIDTYPE_PID {
        (*t).thread_pid = pid;
    } else {
        (*(*t).signal).pids[kind as usize] = pid;
    }
}
unsafe fn rcu_copy_process(p: *mut task_struct) {
    #[cfg(CONFIG_PREEMPT_RCU)]
    {
        (*p).rcu_read_lock_nesting = 0;
        (*p).rcu_read_unlock_special.s = 0;
        (*p).rcu_blocked_node = null_mut();
        init_list(addr_of_mut!((*p).rcu_node_entry));
    }
    #[cfg(CONFIG_TASKS_RCU)]
    {
        (*p).rcu_tasks_holdout = false;
        init_list(addr_of_mut!((*p).rcu_tasks_holdout_list));
        (*p).rcu_tasks_idle_cpu = -1;
        init_list(addr_of_mut!((*p).rcu_tasks_exit_list));
    }
    #[cfg(CONFIG_TASKS_TRACE_RCU)]
    {
        (*p).trc_reader_nesting = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pidfd_prepare(
    pid: *mut pid,
    flags: c_uint,
    ret_file: *mut *mut file,
) -> c_int {
    if flags & PIDFD_STALE == 0 {
        rust_fork_spin_lock_irq(addr_of_mut!((*pid).wait_pidfd.lock));
        let error = if !rust_fork_pid_has_task(pid, PIDTYPE_PID) {
            -(ESRCH as c_int)
        } else if flags & PIDFD_THREAD == 0 && !rust_fork_pid_has_task(pid, PIDTYPE_TGID) {
            -(ENOENT as c_int)
        } else {
            0
        };
        rust_fork_spin_unlock_irq(addr_of_mut!((*pid).wait_pidfd.lock));
        if error != 0 {
            return error;
        }
    }
    let fd = get_unused_fd_flags(O_CLOEXEC);
    if fd < 0 {
        return fd;
    }
    let f = pidfs_alloc_file(pid, flags | O_RDWR);
    if is_err(f) {
        put_unused_fd(fd as c_uint);
        return ptr_err(f);
    }
    *ret_file = f;
    fd
}
unsafe extern "C" fn __delayed_free_task(rhp: *mut rcu_head) {
    free_task(rhp.byte_sub(offset_of!(task_struct, rcu)).cast());
}
unsafe fn delayed_free_task(tsk: *mut task_struct) {
    #[cfg(CONFIG_MEMCG)]
    call_rcu(addr_of_mut!((*tsk).rcu), Some(__delayed_free_task));
    #[cfg(not(CONFIG_MEMCG))]
    free_task(tsk);
}
unsafe fn copy_oom_score_adj(flags: u64, tsk: *mut task_struct) {
    if (*tsk).mm.is_null()
        || (flags & (CLONE_VM | CLONE_THREAD | CLONE_VFORK) as u64) != CLONE_VM as u64
    {
        return;
    }
    rust_fork_oom_adj_lock();
    rust_fork_mm_set_multiprocess((*tsk).mm);
    (*(*tsk).signal).oom_score_adj = (*(*current_task()).signal).oom_score_adj;
    (*(*tsk).signal).oom_score_adj_min = (*(*current_task()).signal).oom_score_adj_min;
    rust_fork_oom_adj_unlock();
}
#[cfg_attr(not(CONFIG_RV), allow(unused_variables))]
unsafe fn rv_task_fork(p: *mut task_struct) {
    #[cfg(CONFIG_RV)]
    core::ptr::write_bytes(addr_of_mut!((*p).rv), 0, 1);
}
fn need_futex_hash_allocate_default(flags: u64) -> bool {
    (flags & (CLONE_VM | CLONE_VFORK) as u64) == CLONE_VM as u64
}
#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
enum ForkUnwind {
    Free,
    Count,
    Delayacct,
    Policy,
    Sched,
    Perf,
    Audit,
    Security,
    Semundo,
    Files,
    Fs,
    Sighand,
    Signal,
    Mm,
    Namespaces,
    Io,
    Thread,
    Pid,
    Pidfd,
    Cgroup,
    Core,
}

#[no_mangle]
pub unsafe extern "C" fn copy_process(
    mut pid: *mut pid,
    trace: c_int,
    node: c_int,
    args: *mut kernel_clone_args,
) -> *mut task_struct {
    use ForkUnwind::*;
    let current = current_task();
    let flags = (*args).flags;
    let nsp = (*current).nsproxy;
    if flags & (CLONE_NEWNS | CLONE_FS) as u64 == (CLONE_NEWNS | CLONE_FS) as u64
        || flags & (CLONE_NEWUSER | CLONE_FS) as u64 == (CLONE_NEWUSER | CLONE_FS) as u64
        || (flags & CLONE_THREAD as u64 != 0 && flags & CLONE_SIGHAND as u64 == 0)
        || (flags & CLONE_SIGHAND as u64 != 0 && flags & CLONE_VM as u64 == 0)
        || (flags & CLONE_PARENT as u64 != 0 && (*(*current).signal).flags & SIGNAL_UNKILLABLE != 0)
    {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_THREAD as u64 != 0
        && (flags & (CLONE_NEWUSER | CLONE_NEWPID) as u64 != 0
            || task_active_pid_ns(current) != (*nsp).pid_ns_for_children)
    {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_PIDFD as u64 != 0 && flags & CLONE_DETACHED as u64 != 0 {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_AUTOREAP as u64 != 0
        && (flags & (CLONE_THREAD | CLONE_PARENT) as u64 != 0 || (*args).exit_signal != 0)
    {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_PARENT as u64 != 0 && (*(*current).signal).autoreap() != 0 {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_NNP as u64 != 0 && flags & CLONE_THREAD as u64 != 0 {
        return err_ptr(-(EINVAL as c_int));
    }
    if flags & CLONE_PIDFD_AUTOKILL as u64 != 0 {
        if flags & CLONE_PIDFD as u64 == 0
            || flags & CLONE_AUTOREAP as u64 == 0
            || flags & CLONE_THREAD as u64 != 0
        {
            return err_ptr(-(EINVAL as c_int));
        }
        if flags & CLONE_NNP as u64 == 0
            && !ns_capable(rust_fork_current_user_ns(), CAP_SYS_ADMIN as c_int)
        {
            return err_ptr(-(EPERM as c_int));
        }
    }
    let mut delayed: multiprocess_signals = zeroed();
    rust_fork_sigemptyset(addr_of_mut!(delayed.signal));
    init_hlist(addr_of_mut!(delayed.node));
    rust_fork_spin_lock_irq(addr_of_mut!((*(*current).sighand).siglock));
    if flags & CLONE_THREAD as u64 == 0 {
        rust_fork_hlist_add_head(
            addr_of_mut!(delayed.node),
            addr_of_mut!((*(*current).signal).multiprocess),
        );
    }
    recalc_sigpending();
    rust_fork_spin_unlock_irq(addr_of_mut!((*(*current).sighand).siglock));
    let mut p: *mut task_struct = null_mut();
    let mut pidfile: *mut file = null_mut();
    let mut pidfd = -1;
    let (error, unwind) = 'build: {
        if rust_fork_task_sigpending(current) {
            break 'build (-(ERESTARTNOINTR as c_int), None);
        }
        p = dup_task_struct(current, node);
        if p.is_null() {
            break 'build (-(ENOMEM as c_int), None);
        }
        macro_rules! attempt {
            ($call:expr, $stage:ident) => {{
                let e = $call;
                if e != 0 {
                    break 'build (e, Some($stage));
                }
            }};
        }
        attempt!(copy_exec_state(flags, p), Free);
        (*p).flags &= !PF_KTHREAD;
        if (*args).kthread() != 0 {
            (*p).flags |= PF_KTHREAD;
        }
        if (*args).user_worker() != 0 {
            (*p).flags |= PF_USER_WORKER;
            rust_fork_siginitsetinv(
                addr_of_mut!((*p).blocked),
                (1u64 << (SIGKILL - 1)) as c_ulong | (1u64 << (SIGSTOP - 1)) as c_ulong,
            );
        }
        if (*args).io_thread() != 0 {
            (*p).flags |= PF_IO_WORKER;
        }
        if !(*args).name.is_null() {
            rust_fork_strscpy_pad((*p).comm.as_mut_ptr(), (*args).name, (*p).comm.len());
        }
        (*p).set_child_tid = if flags & CLONE_CHILD_SETTID as u64 != 0 {
            (*args).child_tid
        } else {
            null_mut()
        };
        (*p).clear_child_tid = if flags & CLONE_CHILD_CLEARTID as u64 != 0 {
            (*args).child_tid
        } else {
            null_mut()
        };
        rust_fork_ftrace_graph_init_task(p);
        rt_mutex_init_task(p);
        rust_fork_blocked_lock_init(p);
        rust_fork_assert_irqs_enabled();
        #[cfg(CONFIG_PROVE_LOCKING)]
        rust_fork_debug_locks_warn(!rust_fork_softirqs_enabled(p));
        let retval = copy_creds(p, flags);
        if retval < 0 {
            break 'build (retval, Some(Free));
        }
        if rust_fork_is_rlimit_overlimit(
            rust_fork_task_ucounts(p),
            UCOUNT_RLIMIT_NPROC,
            rust_fork_rlimit(RLIMIT_NPROC),
        ) {
            if (*(*p).real_cred).user != addr_of_mut!(root_user)
                && !capable(CAP_SYS_RESOURCE as c_int)
                && !capable(CAP_SYS_ADMIN as c_int)
            {
                break 'build (-(EAGAIN as c_int), Some(Count));
            }
        }
        (*current).flags &= !PF_NPROC_EXCEEDED;
        if rust_fork_read_nr_threads(addr_of!(nr_threads))
            >= rust_fork_read_nr_threads(addr_of!(max_threads))
        {
            break 'build (-(EAGAIN as c_int), Some(Count));
        }
        rust_fork_delayacct_tsk_init(p);
        (*p).flags &= !(PF_SUPERPRIV | PF_WQ_WORKER | PF_IDLE | PF_NO_SETAFFINITY);
        (*p).flags |= PF_FORKNOEXEC;
        init_list(addr_of_mut!((*p).children));
        init_list(addr_of_mut!((*p).sibling));
        rcu_copy_process(p);
        (*p).vfork_done = null_mut();
        rust_fork_alloc_lock_init(p);
        rust_fork_init_sigpending(addr_of_mut!((*p).pending));
        (*p).utime = 0;
        (*p).stime = 0;
        (*p).gtime = 0;
        #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
        {
            (*p).utimescaled = 0;
            (*p).stimescaled = 0;
        }
        rust_fork_prev_cputime_init(addr_of_mut!((*p).prev_cputime));
        #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
        {
            rust_fork_vtime_seqcount_init(p);
            (*p).vtime.starttime = 0;
            (*p).vtime.state = VTIME_INACTIVE;
        }
        #[cfg(CONFIG_IO_URING)]
        {
            (*p).io_uring = null_mut();
            attempt!(rust_fork_io_uring_fork(p), Delayacct);
        }
        (*p).default_timer_slack_ns = (*current).timer_slack_ns;
        #[cfg(CONFIG_PSI)]
        {
            (*p).psi_flags = 0;
        }
        rust_fork_task_io_accounting_init(addr_of_mut!((*p).ioac));
        rust_fork_acct_clear_integrals(p);
        fork_posix_cputimers_init(addr_of_mut!((*p).posix_cputimers));
        rust_fork_tick_dep_init_task(p);
        (*p).io_context = null_mut();
        rust_fork_audit_set_context(p, null_mut());
        rust_fork_cgroup_fork(p);
        if (*args).kthread() != 0 && !set_kthread_struct(p) {
            break 'build (-(EAGAIN as c_int), Some(Delayacct));
        }
        #[cfg(CONFIG_NUMA)]
        {
            (*p).mempolicy = rust_fork_mpol_dup((*p).mempolicy);
            if is_err((*p).mempolicy) {
                let e = ptr_err((*p).mempolicy);
                (*p).mempolicy = null_mut();
                break 'build (e, Some(Delayacct));
            }
        }
        #[cfg(CONFIG_CPUSETS)]
        {
            (*p).cpuset_mem_spread_rotor = NUMA_NO_NODE;
            rust_fork_mems_allowed_seq_init(p);
        }
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        {
            core::ptr::write_bytes(addr_of_mut!((*p).irqtrace), 0, 1);
            (*p).irqtrace.hardirq_disable_ip = rust_fork_this_ip();
            (*p).irqtrace.softirq_enable_ip = rust_fork_this_ip();
            (*p).softirqs_enabled = 1;
            (*p).softirq_context = 0;
        }
        (*p).pagefault_disabled = 0;
        rust_fork_lockdep_init_task(p);
        (*p).blocked_on = null_mut();
        (*p).blocked_donor = null_mut();
        #[cfg(CONFIG_BCACHE)]
        {
            (*p).sequential_io = 0;
            (*p).sequential_io_avg = 0;
        }
        rust_fork_unwind_task_init(p);
        attempt!(sched_fork(flags, p), Policy);
        attempt!(rust_fork_perf_event_init_task(p, flags), Sched);
        attempt!(rust_fork_audit_alloc(p), Perf);
        rust_fork_shm_init_task(p);
        attempt!(rust_fork_security_task_alloc(p, flags as c_ulong), Audit);
        attempt!(rust_fork_copy_semundo(flags, p), Security);
        attempt!(copy_files(flags, p, (*args).no_files() as c_int), Semundo);
        attempt!(copy_fs(flags, p, (*args).umh() != 0), Files);
        attempt!(copy_sighand(flags, p), Fs);
        attempt!(copy_signal(flags, p), Sighand);
        attempt!(copy_mm(flags, p), Signal);
        attempt!(copy_namespaces(flags, p), Mm);
        attempt!(rust_fork_copy_io(flags, p), Namespaces);
        attempt!(copy_thread(p, args), Io);
        rust_fork_stackleak_task_init(p);
        if pid != addr_of_mut!(init_struct_pid) {
            pid = alloc_pid(
                (*(*p).nsproxy).pid_ns_for_children,
                (*args).set_tid,
                (*args).set_tid_size,
            );
            if is_err(pid) {
                break 'build (ptr_err(pid), Some(Thread));
            }
        }
        if flags & CLONE_PIDFD as u64 != 0 {
            let mut fdflags = PIDFD_STALE;
            if flags & CLONE_THREAD as u64 != 0 {
                fdflags |= PIDFD_THREAD;
            }
            if flags & CLONE_PIDFD_AUTOKILL as u64 != 0 {
                fdflags |= PIDFD_AUTOKILL;
            }
            pidfd = pidfd_prepare(pid, fdflags, &mut pidfile);
            if pidfd < 0 {
                break 'build (pidfd, Some(Pid));
            }
            attempt!(rust_fork_put_user_int(pidfd, (*args).pidfd), Pidfd);
        }
        #[cfg(CONFIG_BLOCK)]
        {
            (*p).plug = null_mut();
            (*p).flags &= !PF_BLOCK_TS;
        }
        rust_fork_futex_init_task(p);
        if flags & (CLONE_VM | CLONE_VFORK) as u64 == CLONE_VM as u64 {
            rust_fork_sas_ss_reset(p);
        }
        rust_fork_user_disable_single_step(p);
        rust_fork_clear_syscall_trace(p);
        rust_fork_clear_syscall_emu(p);
        rust_fork_clear_tsk_latency_tracing(p);
        (*p).pid = rust_fork_pid_nr(pid);
        if flags & CLONE_THREAD as u64 != 0 {
            (*p).group_leader = (*current).group_leader;
            (*p).tgid = (*current).tgid;
        } else {
            (*p).group_leader = p;
            (*p).tgid = (*p).pid;
        }
        (*p).nr_dirtied = 0;
        (*p).nr_dirtied_pause = 128 >> (RUST_FORK_PAGE_SHIFT - 10);
        (*p).dirty_paused_when = 0;
        (*p).pdeath_signal = 0;
        (*p).task_works = null_mut();
        rust_fork_clear_posix_cputimers_work(p);
        #[cfg(CONFIG_KRETPROBES)]
        {
            (*p).kretprobe_instances.first = null_mut();
        }
        #[cfg(CONFIG_RETHOOK)]
        {
            (*p).rethooks.first = null_mut();
        }
        attempt!(rust_fork_cgroup_can_fork(p, args), Pidfd);
        attempt!(sched_cgroup_fork(p, args), Cgroup);
        if need_futex_hash_allocate_default(flags) {
            attempt!(rust_fork_futex_hash_allocate_default(), Cgroup);
        }
        (*p).start_time = rust_fork_ktime_get_ns();
        (*p).start_boottime = rust_fork_ktime_get_boottime_ns();
        rust_fork_tasklist_write_lock_irq();
        if flags & (CLONE_PARENT | CLONE_THREAD) as u64 != 0 {
            (*p).real_parent = (*current).real_parent;
            (*p).parent_exec_id = (*current).parent_exec_id;
            (*p).exit_signal = if flags & CLONE_THREAD as u64 != 0 {
                -1
            } else {
                (*(*current).group_leader).exit_signal
            };
        } else {
            (*p).real_parent = current;
            (*p).parent_exec_id = (*current).self_exec_id;
            (*p).exit_signal = (*args).exit_signal;
        }
        rust_fork_klp_copy_process(p);
        rust_fork_sched_core_fork(p);
        rust_fork_spin_lock(addr_of_mut!((*(*current).sighand).siglock));
        rv_task_fork(p);
        fork_rseq_fork(p, flags);
        if (*rust_fork_ns_of_pid(pid)).pid_allocated & PIDNS_ADDING == 0 {
            break 'build (-(ENOMEM as c_int), Some(Core));
        }
        if rust_fork_fatal_signal_pending(current) {
            break 'build (-(EINTR as c_int), Some(Core));
        }
        // Commit: the original has no failure exits after this point.
        copy_seccomp(p);
        if flags & CLONE_NNP as u64 != 0 {
            rust_fork_task_set_no_new_privs(p);
        }
        init_task_pid_links(p);
        if (*p).pid != 0 {
            fork_ptrace_init_task(p, flags & CLONE_PTRACE as u64 != 0 || trace != 0);
            init_task_pid(p, PIDTYPE_PID, pid);
            if rust_fork_thread_group_leader(p) {
                init_task_pid(p, PIDTYPE_TGID, pid);
                init_task_pid(p, PIDTYPE_PGID, rust_fork_task_pgrp(current));
                init_task_pid(p, PIDTYPE_SID, rust_fork_task_session(current));
                if rust_fork_is_child_reaper(pid) {
                    let ns = rust_fork_ns_of_pid(pid);
                    rust_fork_set_child_reaper(ns, p);
                    (*(*p).signal).flags |= SIGNAL_UNKILLABLE;
                }
                (*(*p).signal).shared_pending.signal = delayed.signal;
                (*(*p).signal).tty = rust_fork_tty_kref_get((*(*current).signal).tty);
                let parent_sig = (*(*p).real_parent).signal;
                (*(*p).signal).set_has_child_subreaper(
                    ((*parent_sig).has_child_subreaper() != 0
                        || (*parent_sig).is_child_subreaper() != 0) as c_uint,
                );
                if flags & CLONE_AUTOREAP as u64 != 0 {
                    (*(*p).signal).set_autoreap(1);
                }
                rust_fork_list_add_tail(
                    addr_of_mut!((*p).sibling),
                    addr_of_mut!((*(*p).real_parent).children),
                );
                rust_fork_list_add_tail_rcu(
                    addr_of_mut!((*p).tasks),
                    addr_of_mut!((*addr_of_mut!(init_task)).tasks),
                );
                attach_pid(p, PIDTYPE_TGID);
                attach_pid(p, PIDTYPE_PGID);
                attach_pid(p, PIDTYPE_SID);
                rust_fork_process_count_inc();
            } else {
                (*(*current).signal).nr_threads += 1;
                (*(*current).signal).quick_threads += 1;
                rust_fork_atomic_inc(addr_of_mut!((*(*current).signal).live));
                rust_fork_refcount_inc(addr_of_mut!((*(*current).signal).sigcnt));
                task_join_group_stop(p);
                rust_fork_list_add_tail_rcu(
                    addr_of_mut!((*p).thread_node),
                    addr_of_mut!((*(*p).signal).thread_head),
                );
            }
            attach_pid(p, PIDTYPE_PID);
            nr_threads += 1;
        }
        total_forks = total_forks.wrapping_add(1);
        rust_fork_hlist_del_init(addr_of_mut!(delayed.node));
        rust_fork_spin_unlock(addr_of_mut!((*(*current).sighand).siglock));
        rust_fork_syscall_tracepoint_update(p);
        rust_fork_tasklist_write_unlock_irq();
        if !pidfile.is_null() {
            fd_install(pidfd as c_uint, pidfile);
        }
        rust_fork_proc_fork_connector(p);
        rust_fork_cgroup_post_fork(p, args);
        sched_post_fork(p);
        rust_fork_perf_event_fork(p);
        rust_fork_trace_task_newtask(p, flags);
        rust_fork_uprobe_copy_process(p, flags);
        rust_fork_user_events_fork(p, flags);
        copy_oom_score_adj(flags, p);
        return p;
    };
    if let Some(stage) = unwind {
        if stage >= Core {
            rust_fork_sched_core_free(p);
            rust_fork_spin_unlock(addr_of_mut!((*(*current).sighand).siglock));
            rust_fork_tasklist_write_unlock_irq();
        }
        if stage >= Cgroup {
            rust_fork_cgroup_cancel_fork(p, args);
        }
        if stage >= Pidfd && flags & CLONE_PIDFD as u64 != 0 {
            fput(pidfile);
            put_unused_fd(pidfd as c_uint);
        }
        if stage >= Pid && pid != addr_of_mut!(init_struct_pid) {
            free_pid(pid);
        }
        if stage >= Thread {
            rust_fork_exit_thread(p);
        }
        if stage >= Io && !(*p).io_context.is_null() {
            rust_fork_exit_io_context(p);
        }
        if stage >= Namespaces {
            rust_fork_exit_nsproxy_namespaces(p);
        }
        if stage >= Mm && !(*p).mm.is_null() {
            mm_clear_owner((*p).mm, p);
            mmput((*p).mm);
        }
        if stage >= Signal && flags & CLONE_THREAD as u64 == 0 {
            free_signal_struct((*p).signal);
        }
        if stage >= Sighand {
            __cleanup_sighand((*p).sighand);
        }
        if stage >= Fs {
            exit_fs(p);
        }
        if stage >= Files {
            exit_files(p);
        }
        if stage >= Semundo {
            rust_fork_exit_sem(p);
        }
        if stage >= Security {
            rust_fork_security_task_free(p);
        }
        if stage >= Audit {
            rust_fork_audit_free(p);
        }
        if stage >= Perf {
            rust_fork_perf_event_free_task(p);
        }
        if stage >= Sched {
            sched_cancel_fork(p);
        }
        if stage >= Policy {
            rust_fork_lockdep_free_task(p);
            #[cfg(CONFIG_NUMA)]
            rust_fork_mpol_put((*p).mempolicy);
        }
        if stage >= Delayacct {
            rust_fork_io_uring_free(p);
            rust_fork_delayacct_tsk_free(p);
        }
        if stage >= Count {
            dec_rlimit_ucounts(rust_fork_task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
            exit_cred_namespaces(p);
            exit_creds(p);
        }
        write_volatile(addr_of_mut!((*p).__state), TASK_DEAD);
        exit_task_stack_account(p);
        put_task_stack(p);
        delayed_free_task(p);
    }
    rust_fork_spin_lock_irq(addr_of_mut!((*(*current).sighand).siglock));
    rust_fork_hlist_del_init(addr_of_mut!(delayed.node));
    rust_fork_spin_unlock_irq(addr_of_mut!((*(*current).sighand).siglock));
    err_ptr(error)
}
