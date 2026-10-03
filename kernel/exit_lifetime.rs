// SPDX-License-Identifier: GPL-2.0-only
unsafe fn __unhash_process(post: &mut [*mut pid; PIDTYPE_MAX.0 as usize], p: *mut task_struct, group_dead: bool) {
    let pid = task_pid(p);
    b::nr_threads = b::nr_threads.wrapping_sub(1);
    b::detach_pid(post.as_mut_ptr(), p, PIDTYPE_PID);
    b::rust_exit_wake_all(addr_of_mut!((*pid).wait_pidfd));
    if group_dead {
        b::detach_pid(post.as_mut_ptr(), p, PIDTYPE_TGID);
        b::detach_pid(post.as_mut_ptr(), p, PIDTYPE_PGID);
        b::detach_pid(post.as_mut_ptr(), p, PIDTYPE_SID);
        list_del_rcu(addr_of_mut!((*p).tasks));
        list_del_init(addr_of_mut!((*p).sibling));
        b::rust_exit_process_count_dec();
    }
    list_del_rcu(addr_of_mut!((*p).thread_node));
}
unsafe fn __exit_signal(post: &mut [*mut pid; PIDTYPE_MAX.0 as usize], tsk: *mut task_struct) {
    let sig = (*tsk).signal;
    let group_dead = thread_group_leader(tsk);
    let sighand = b::rust_exit_sighand_load(tsk);
    let mut tty = null_mut();
    b::rust_exit_spin_lock(addr_of_mut!((*sighand).siglock));
    #[cfg(CONFIG_POSIX_TIMERS)] {
        b::posix_cpu_timers_exit(tsk);
        if group_dead { b::posix_cpu_timers_exit_group(tsk); }
    }
    if group_dead {
        tty = (*sig).tty;
        (*sig).tty = null_mut();
    } else {
        if (*sig).notify_count > 0 {
            (*sig).notify_count -= 1;
            if (*sig).notify_count == 0 { b::wake_up_process((*sig).group_exec_task); }
        }
        if tsk == (*sig).curr_target { (*sig).curr_target = next_thread(tsk); }
    }
    let (mut utime, mut stime) = (0, 0);
    task_cputime(tsk, &mut utime, &mut stime);
    b::rust_exit_seq_lock(addr_of_mut!((*sig).stats_lock));
    (*sig).utime = (*sig).utime.wrapping_add(utime);
    (*sig).stime = (*sig).stime.wrapping_add(stime);
    (*sig).gtime = (*sig).gtime.wrapping_add(task_gtime(tsk));
    (*sig).min_flt = (*sig).min_flt.wrapping_add((*tsk).min_flt);
    (*sig).maj_flt = (*sig).maj_flt.wrapping_add((*tsk).maj_flt);
    (*sig).nvcsw = (*sig).nvcsw.wrapping_add((*tsk).nvcsw);
    (*sig).nivcsw = (*sig).nivcsw.wrapping_add((*tsk).nivcsw);
    (*sig).inblock = (*sig).inblock.wrapping_add(task_io_get_inblock(tsk));
    (*sig).oublock = (*sig).oublock.wrapping_add(task_io_get_oublock(tsk));
    task_io_accounting_add(addr_of_mut!((*sig).ioac), addr_of_mut!((*tsk).ioac));
    (*sig).sum_sched_runtime = (*sig).sum_sched_runtime.wrapping_add((*tsk).se.sum_exec_runtime);
    (*sig).nr_threads -= 1;
    __unhash_process(post, tsk, group_dead);
    b::rust_exit_seq_unlock(addr_of_mut!((*sig).stats_lock));
    // Release pairs with lock_task_sighand's acquire-after-control-dependency.
    b::rust_exit_sighand_clear_release(tsk);
    b::rust_exit_spin_unlock(addr_of_mut!((*sighand).siglock));
    b::__cleanup_sighand(sighand);
    #[cfg(CONFIG_TTY)] if group_dead { b::tty_kref_put(tty); }
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_delayed_put_task_struct(rhp: *mut callback_head) {
    let tsk = rhp.cast::<u8>().sub(offset_of!(task_struct, rcu)).cast::<task_struct>();
    #[cfg(all(CONFIG_KPROBES, CONFIG_KRETPROBES, not(CONFIG_KRETPROBE_ON_RETHOOK)))] b::kprobe_flush_task(tsk);
    #[cfg(CONFIG_RETHOOK)] b::rethook_flush_task(tsk);
    #[cfg(CONFIG_PERF_EVENTS)] b::perf_event_delayed_put(tsk);
    b::rust_exit_trace_free(tsk);
    put_task_struct(tsk);
}
#[no_mangle]
pub unsafe extern "C" fn put_task_struct_rcu_user(task: *mut task_struct) {
    if b::rust_exit_ref_dec(addr_of_mut!((*task).rcu_users)) {
        b::call_rcu(addr_of_mut!((*task).rcu), Some(b::rust_exit_delayed_put_callback));
    }
}
// Original weak default is intentionally empty; architectures may override it.
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn release_thread(_dead_task: *mut task_struct) {}
#[no_mangle]
pub unsafe extern "C" fn release_task(mut p: *mut task_struct) {
    loop {
        let mut post = [null_mut(); PIDTYPE_MAX.0 as usize];
        b::dec_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
        b::pidfs_exit(p);
        #[cfg(CONFIG_CGROUPS)] b::cgroup_task_release(p);
        let thread_pid = task_pid(p);
        b::rust_exit_tasklist_write_lock_irq();
        ptrace_release_task(p);
        __exit_signal(&mut post, p);
        let leader = (*p).group_leader;
        let mut zap_leader = false;
        if leader != p && thread_group_empty(leader) && (*leader).exit_state == EXIT_ZOMBIE as c_int {
            if (*(*leader).signal).flags & SIGNAL_GROUP_EXIT != 0 {
                (*leader).exit_code = (*(*leader).signal).group_exit_code;
            }
            zap_leader = b::do_notify_parent(leader, (*leader).exit_signal);
            if zap_leader { (*leader).exit_state = EXIT_DEAD as c_int; }
        }
        b::rust_exit_tasklist_write_unlock_irq();
        #[cfg(CONFIG_PROC_FS)] b::proc_flush_pid(thread_pid);
        b::exit_cred_namespaces(p);
        b::add_device_randomness(addr_of!((*p).se.sum_exec_runtime).cast(), size_of_val_raw(addr_of!((*p).se.sum_exec_runtime)) as _);
        b::free_pids(post.as_mut_ptr());
        b::release_thread(p);
        b::flush_sigqueue(addr_of_mut!((*p).pending));
        if thread_group_leader(p) { b::flush_sigqueue(addr_of_mut!((*(*p).signal).shared_pending)); }
        put_task_struct_rcu_user(p);
        if !zap_leader { break; }
        p = leader;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rcuwait_wake_up(w: *mut rcuwait) -> c_int {
    b::rust_exit_rcu_lock();
    // Full barrier is the original B paired with set_current_state's A.
    b::rust_exit_mb();
    let task = b::rust_exit_rcuwait_task_load(w);
    let ret = if task.is_null() { 0 } else { b::wake_up_process(task) };
    b::rust_exit_rcu_unlock();
    ret
}
unsafe fn will_become_orphaned_pgrp(pgrp: *mut pid, ignored_task: *mut task_struct) -> bool {
    if pgrp.is_null() { return true; }
    let mut node = pid_first(pgrp, PIDTYPE_PGID);
    while !node.is_null() {
        let p = task_from_pid_link(node, PIDTYPE_PGID);
        if p != ignored_task && !((*p).exit_state != 0 && thread_group_empty(p)) && !is_global_init((*p).real_parent) {
            if task_pgrp((*p).real_parent) != pgrp && task_session((*p).real_parent) == task_session(p) { return false; }
        }
        node = b::rust_exit_hlist_next(node);
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn is_current_pgrp_orphaned() -> c_int {
    b::rust_exit_tasklist_read_lock();
    let retval = will_become_orphaned_pgrp(task_pgrp(current_task()), null_mut());
    b::rust_exit_tasklist_read_unlock();
    retval as c_int
}
unsafe fn has_stopped_jobs(pgrp: *mut pid) -> bool {
    if pgrp.is_null() { return false; }
    let mut node = pid_first(pgrp, PIDTYPE_PGID);
    while !node.is_null() {
        let p = task_from_pid_link(node, PIDTYPE_PGID);
        if (*(*p).signal).flags & SIGNAL_STOP_STOPPED != 0 { return true; }
        node = b::rust_exit_hlist_next(node);
    }
    false
}
unsafe fn kill_orphaned_pgrp(tsk: *mut task_struct, mut parent: *mut task_struct) {
    let pgrp = task_pgrp(tsk);
    let ignored_task = if parent.is_null() { parent = (*tsk).real_parent; tsk } else { null_mut() };
    if task_pgrp(parent) != pgrp && task_session(parent) == task_session(tsk)
        && will_become_orphaned_pgrp(pgrp, ignored_task) && has_stopped_jobs(pgrp) {
        b::__kill_pgrp_info(SIGHUP as _, RUST_EXIT_SEND_SIG_PRIV as *mut kernel_siginfo, pgrp);
        b::__kill_pgrp_info(SIGCONT as _, RUST_EXIT_SEND_SIG_PRIV as *mut kernel_siginfo, pgrp);
    }
}
