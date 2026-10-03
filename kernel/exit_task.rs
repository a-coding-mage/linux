// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_DEBUG_STACK_USAGE)]
#[no_mangle]
pub unsafe extern "C" fn stack_not_used(p: *mut task_struct) -> c_ulong {
    let end = end_of_stack(p);
    let mut n = end;
    loop {
        #[cfg(CONFIG_STACK_GROWSUP)] { n = n.sub(1); }
        #[cfg(not(CONFIG_STACK_GROWSUP))] { n = n.add(1); }
        if *n != 0 { break; }
    }
    #[cfg(CONFIG_STACK_GROWSUP)] { (end as c_ulong).wrapping_sub(n as c_ulong) }
    #[cfg(not(CONFIG_STACK_GROWSUP))] { (n as c_ulong).wrapping_sub(end as c_ulong) }
}
#[cfg(CONFIG_DEBUG_STACK_USAGE)]
unsafe fn kstack_histogram(used_stack: c_ulong) {
    #[cfg(CONFIG_VM_EVENT_COUNTERS)] {
        // KSTACK events are consecutive. The C declaration contains exactly
        // the buckets permitted by THREAD_SIZE, then KSTACK_REST if needed.
        // Select only a bucket that exists in that configured declaration.
        let mut threshold: c_ulong = 1024;
        let mut bucket = 0;
        while used_stack > threshold && threshold < RUST_EXIT_THREAD_SIZE as c_ulong && threshold < 65536 {
            threshold *= 2;
            bucket += 1;
        }
        if used_stack <= threshold {
            b::rust_exit_count_vm_event(RUST_EXIT_KSTACK_FIRST + bucket);
        } else if RUST_EXIT_THREAD_SIZE > 65536 {
            b::rust_exit_count_vm_event(RUST_EXIT_KSTACK_FIRST + 7);
        }
    }
}
unsafe fn check_stack_usage() {
    #[cfg(CONFIG_DEBUG_STACK_USAGE)] {
        let free = stack_not_used(current_task());
        kstack_histogram((RUST_EXIT_THREAD_SIZE as c_ulong).wrapping_sub(free));
        if free >= lowest_to_date as c_ulong { return; }
        b::rust_exit_spin_lock(addr_of_mut!(low_water_lock));
        if free < lowest_to_date as c_ulong {
            b::rust_exit_log_stack(current_task(), free);
            lowest_to_date = free as c_int;
        }
        b::rust_exit_spin_unlock(addr_of_mut!(low_water_lock));
    }
}
unsafe fn synchronize_group_exit(tsk: *mut task_struct, code: c_long) {
    let sighand = (*tsk).sighand;
    let signal = (*tsk).signal;
    b::rust_exit_spin_lock_irq(addr_of_mut!((*sighand).siglock));
    (*signal).quick_threads -= 1;
    if (*signal).quick_threads == 0 && (*signal).flags & SIGNAL_GROUP_EXIT == 0 {
        (*signal).flags = SIGNAL_GROUP_EXIT;
        (*signal).group_exit_code = code as c_int;
        (*signal).group_stop_count = 0;
    }
    (*tsk).flags |= PF_POSTCOREDUMP;
    let core_state = (*signal).core_state;
    b::rust_exit_spin_unlock_irq(addr_of_mut!((*sighand).siglock));
    if !core_state.is_null() { coredump_task_exit(tsk, core_state); }
}
#[no_mangle]
pub unsafe extern "C" fn do_exit(code: c_long) -> ! {
    let tsk = current_task();
    b::rust_exit_warn_irqs();
    b::rust_exit_warn_plug(!(*tsk).plug.is_null());
    if (*tsk).flags & PF_KTHREAD != 0 {
        let kthread: *mut kthread = (*tsk).worker_private.cast();
        if !kthread.is_null() { b::kthread_do_exit(kthread, code); }
    }
    #[cfg(CONFIG_KCOV)] b::kcov_task_exit(tsk);
    #[cfg(CONFIG_KMSAN)] b::kmsan_task_exit(tsk);
    synchronize_group_exit(tsk, code);
    // ptrace_event(PTRACE_EVENT_EXIT): only its enabled event branch applies.
    if (*tsk).ptrace & ((RUST_EXIT_PT_EVENT_FLAG_BASE as c_uint) << PTRACE_EVENT_EXIT) != 0 {
        b::ptrace_notify(((PTRACE_EVENT_EXIT << 8) | SIGTRAP) as _, code as c_ulong);
    }
    #[cfg(CONFIG_USER_EVENTS)] {
        if !(*tsk).user_event_mm.is_null() { b::user_event_mm_remove(tsk); }
    }
    #[cfg(CONFIG_IO_URING)] {
        if !(*tsk).io_uring.is_null() { b::__io_uring_cancel(false); }
    }
    #[cfg(CONFIG_SCHED_MM_CID)] b::sched_mm_cid_exit(tsk);
    b::exit_signals(tsk);
    #[cfg(CONFIG_SECCOMP_FILTER)] b::seccomp_filter_release(tsk);
    #[cfg(CONFIG_TASK_XACCT)] b::acct_update_integrals(tsk);
    let group_dead = b::rust_exit_atomic_dec(addr_of_mut!((*(*tsk).signal).live));
    if group_dead {
        if is_global_init(tsk) {
            let group_code = (*(*tsk).signal).group_exit_code;
            b::panic(c"Attempted to kill init! exitcode=0x%08x\n".as_ptr().cast(), if group_code != 0 { group_code } else { code as c_int });
        }
        #[cfg(CONFIG_POSIX_TIMERS)] {
            b::hrtimer_cancel(addr_of_mut!((*(*tsk).signal).real_timer));
            b::exit_itimers(tsk);
        }
        if !(*tsk).mm.is_null() { setmax_mm_hiwater_rss(addr_of_mut!((*(*tsk).signal).maxrss), (*tsk).mm); }
    }
    #[cfg(CONFIG_BSD_PROCESS_ACCT)] b::acct_collect(code, group_dead as c_int);
    #[cfg(CONFIG_AUDIT)] { if group_dead { b::tty_audit_exit(); } }
    #[cfg(CONFIG_AUDITSYSCALL)] { if !(*tsk).audit_context.is_null() { b::__audit_free(tsk); } }
    (*tsk).exit_code = code as c_int;
    #[cfg(CONFIG_TASKSTATS)] b::taskstats_exit(tsk, group_dead as c_int);
    b::rust_exit_trace_exit(tsk, group_dead);
    #[cfg(CONFIG_PERF_EVENTS)] b::perf_event_exit_task(tsk);
    #[cfg(CONFIG_UNWIND_USER)] b::unwind_deferred_task_exit(tsk);
    exit_mm();
    #[cfg(CONFIG_BSD_PROCESS_ACCT)] { if group_dead { b::acct_process(); } }
    #[cfg(CONFIG_SYSVIPC)] { b::exit_sem(tsk); b::exit_shm(tsk); }
    b::exit_files(tsk);
    b::exit_fs(tsk);
    #[cfg(CONFIG_TTY)] if group_dead { b::disassociate_ctty(1); }
    b::exit_nsproxy_namespaces(tsk);
    b::task_work_run();
    #[cfg(CONFIG_HAVE_EXIT_THREAD)] b::exit_thread(tsk);
    #[cfg(CONFIG_SCHED_AUTOGROUP)] b::sched_autogroup_exit_task(tsk);
    #[cfg(CONFIG_CGROUPS)] b::cgroup_task_exit(tsk);
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)] b::flush_ptrace_hw_breakpoint(tsk);
    #[cfg(CONFIG_TASKS_RCU_GENERIC)] b::exit_tasks_rcu_start();
    exit_notify(tsk, group_dead);
    #[cfg(CONFIG_PROC_EVENTS)] b::proc_exit_connector(tsk);
    #[cfg(CONFIG_NUMA)] b::mpol_put_task_policy(tsk);
    #[cfg(CONFIG_FUTEX)] {
        if !(*tsk).futex.pi_state_cache.is_null() { b::kfree((*tsk).futex.pi_state_cache.cast()); }
    }
    #[cfg(CONFIG_LOCKDEP)] b::debug_check_no_locks_held();
    #[cfg(CONFIG_BLOCK)] if !(*tsk).io_context.is_null() { b::exit_io_context(tsk); }
    if !(*tsk).splice_pipe.is_null() { b::free_pipe_info((*tsk).splice_pipe); }
    if !(*tsk).task_frag.page.is_null() { put_page((*tsk).task_frag.page); }
    b::exit_task_stack_account(tsk);
    check_stack_usage();
    preempt_disable();
    if (*tsk).nr_dirtied != 0 { b::rust_exit_dirty_leaks_add((*tsk).nr_dirtied as _); }
    #[cfg(CONFIG_TREE_RCU)] b::exit_rcu();
    #[cfg(CONFIG_TASKS_RCU_GENERIC)] b::exit_tasks_rcu_finish();
    // lockdep_free_task is unconditionally empty in this source revision.
    b::do_task_dead()
}
#[no_mangle]
pub unsafe extern "C" fn make_task_dead(signr: c_int) -> ! {
    let tsk = current_task();
    if b::rust_exit_in_interrupt() { b::panic(c"Aiee, killing interrupt handler!".as_ptr().cast()); }
    if (*tsk).pid == 0 { b::panic(c"Attempted to kill the idle task!".as_ptr().cast()); }
    if b::rust_exit_irqs_disabled() {
        b::rust_exit_log_irq(tsk);
        local_irq_enable();
    }
    if b::rust_exit_in_atomic() {
        b::rust_exit_log_preempt(tsk);
        b::rust_exit_preempt_reset();
    }
    let limit = read_volatile(addr_of!(oops_limit));
    // Preserve C's signed-to-unsigned conversion before the comparison.
    if b::rust_exit_atomic_inc_return(addr_of_mut!(oops_count)) as c_uint >= limit && limit != 0 {
        b::panic(c"Oopsed too often (kernel.oops_limit is %d)".as_ptr().cast(), limit);
    }
    if (*tsk).flags & PF_EXITING != 0 {
        b::rust_exit_log_recursive();
        #[cfg(CONFIG_FUTEX)] b::futex_exit_recursive(tsk);
        (*tsk).exit_state = EXIT_DEAD as c_int;
        b::rust_exit_ref_inc(addr_of_mut!((*tsk).rcu_users));
        preempt_disable();
        b::do_task_dead();
    }
    do_exit(signr as c_long)
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_sys_exit(error_code: c_int) -> ! {
    do_exit(((error_code & 0xff) << 8) as c_long)
}
#[no_mangle]
pub unsafe extern "C" fn do_group_exit(mut exit_code: c_int) -> ! {
    let tsk = current_task();
    let sig = (*tsk).signal;
    if (*sig).flags & SIGNAL_GROUP_EXIT != 0 { exit_code = (*sig).group_exit_code; }
    else if !(*sig).group_exec_task.is_null() { exit_code = 0; }
    else {
        let sighand = (*tsk).sighand;
        b::rust_exit_spin_lock_irq(addr_of_mut!((*sighand).siglock));
        if (*sig).flags & SIGNAL_GROUP_EXIT != 0 { exit_code = (*sig).group_exit_code; }
        else if !(*sig).group_exec_task.is_null() { exit_code = 0; }
        else {
            (*sig).group_exit_code = exit_code;
            (*sig).flags = SIGNAL_GROUP_EXIT;
            b::zap_other_threads(tsk);
        }
        b::rust_exit_spin_unlock_irq(addr_of_mut!((*sighand).siglock));
    }
    do_exit(exit_code as c_long)
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_sys_exit_group(error_code: c_int) -> ! {
    do_group_exit((error_code & 0xff) << 8)
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_abort() -> ! {
    b::rust_exit_bug()
}
