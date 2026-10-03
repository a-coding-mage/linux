// SPDX-License-Identifier: GPL-2.0-only
unsafe fn eligible_pid(wo: *mut wait_opts, p: *mut task_struct) -> bool {
    (*wo).wo_type == PIDTYPE_MAX || task_pid_type(p, (*wo).wo_type) == (*wo).wo_pid
}
unsafe fn eligible_child(wo: *mut wait_opts, ptrace: bool, p: *mut task_struct) -> bool {
    eligible_pid(wo, p) && (ptrace || wait_flags(wo, __WALL) || (((*p).exit_signal != SIGCHLD as c_int) == wait_flags(wo, __WCLONE)))
}
unsafe fn wait_task_zombie(wo: *mut wait_opts, p: *mut task_struct) -> c_int {
    let pid = b::__task_pid_nr_ns(p, PIDTYPE_PID, null_mut());
    let uid = task_wait_uid(p);
    if !wait_flags(wo, WEXITED) { return 0; }
    let status;
    if wait_flags(wo, WNOWAIT) {
        status = task_exit_status(p);
        get_task_struct(p);
        b::rust_exit_tasklist_read_unlock();
        b::rust_exit_sched_annotate_sleep();
        if !(*wo).wo_rusage.is_null() { b::getrusage(p, RUSAGE_BOTH as _, (*wo).wo_rusage); }
        put_task_struct(p);
    } else {
        let mut state = if ptrace_reparented(p) && thread_group_leader(p) { EXIT_TRACE } else { EXIT_DEAD } as c_int;
        if b::rust_exit_cmpxchg_exit_state(p, EXIT_ZOMBIE as _, state) != EXIT_ZOMBIE as c_int { return 0; }
        b::rust_exit_tasklist_read_unlock();
        b::rust_exit_sched_annotate_sleep();
        if state == EXIT_DEAD as c_int && thread_group_leader(p) {
            let sig = (*p).signal;
            let psig = (*current_task()).signal;
            let (mut tgutime, mut tgstime) = (0, 0);
            b::thread_group_cputime_adjusted(p, &mut tgutime, &mut tgstime);
            b::rust_exit_seq_lock_irq(addr_of_mut!((*psig).stats_lock));
            (*psig).cutime = (*psig).cutime.wrapping_add(tgutime).wrapping_add((*sig).cutime);
            (*psig).cstime = (*psig).cstime.wrapping_add(tgstime).wrapping_add((*sig).cstime);
            (*psig).cgtime = (*psig).cgtime.wrapping_add(task_gtime(p)).wrapping_add((*sig).gtime).wrapping_add((*sig).cgtime);
            (*psig).cmin_flt = (*psig).cmin_flt.wrapping_add((*p).min_flt).wrapping_add((*sig).min_flt).wrapping_add((*sig).cmin_flt);
            (*psig).cmaj_flt = (*psig).cmaj_flt.wrapping_add((*p).maj_flt).wrapping_add((*sig).maj_flt).wrapping_add((*sig).cmaj_flt);
            (*psig).cnvcsw = (*psig).cnvcsw.wrapping_add((*p).nvcsw).wrapping_add((*sig).nvcsw).wrapping_add((*sig).cnvcsw);
            (*psig).cnivcsw = (*psig).cnivcsw.wrapping_add((*p).nivcsw).wrapping_add((*sig).nivcsw).wrapping_add((*sig).cnivcsw);
            (*psig).cinblock = (*psig).cinblock.wrapping_add(task_io_get_inblock(p)).wrapping_add((*sig).inblock).wrapping_add((*sig).cinblock);
            (*psig).coublock = (*psig).coublock.wrapping_add(task_io_get_oublock(p)).wrapping_add((*sig).oublock).wrapping_add((*sig).coublock);
            (*psig).cmaxrss = core::cmp::max((*psig).cmaxrss, core::cmp::max((*sig).maxrss, (*sig).cmaxrss));
            task_io_accounting_add(addr_of_mut!((*psig).ioac), addr_of_mut!((*p).ioac));
            task_io_accounting_add(addr_of_mut!((*psig).ioac), addr_of_mut!((*sig).ioac));
            b::rust_exit_seq_unlock_irq(addr_of_mut!((*psig).stats_lock));
        }
        if !(*wo).wo_rusage.is_null() { b::getrusage(p, RUSAGE_BOTH as _, (*wo).wo_rusage); }
        status = task_exit_status(p);
        (*wo).wo_stat = status;
        if state == EXIT_TRACE as c_int {
            b::rust_exit_tasklist_write_lock_irq();
            ptrace_unlink(p);
            state = EXIT_ZOMBIE as c_int;
            if b::do_notify_parent(p, (*p).exit_signal) { state = EXIT_DEAD as c_int; }
            (*p).exit_state = state;
            b::rust_exit_tasklist_write_unlock_irq();
        }
        if state == EXIT_DEAD as c_int { release_task(p); }
    }
    let infop = (*wo).wo_info;
    if !infop.is_null() {
        if status & 0x7f == 0 {
            (*infop).cause = CLD_EXITED as c_int;
            (*infop).status = status >> 8;
        } else {
            (*infop).cause = if status & 0x80 != 0 { CLD_DUMPED } else { CLD_KILLED } as c_int;
            (*infop).status = status & 0x7f;
        }
        (*infop).pid = pid;
        (*infop).uid = uid;
    }
    pid
}
unsafe fn task_stopped_code(p: *mut task_struct, ptrace: bool) -> *mut c_int {
    if ptrace {
        if read_volatile(addr_of!((*p).jobctl)) & JOBCTL_TRACED as c_ulong != 0 && (*p).jobctl & JOBCTL_LISTENING as c_ulong == 0 {
            return addr_of_mut!((*p).exit_code);
        }
    } else if (*(*p).signal).flags & SIGNAL_STOP_STOPPED != 0 {
        return addr_of_mut!((*(*p).signal).group_exit_code);
    }
    null_mut()
}
unsafe fn wait_task_stopped(wo: *mut wait_opts, ptrace: bool, p: *mut task_struct) -> c_int {
    if !ptrace && !wait_flags(wo, WUNTRACED) { return 0; }
    if task_stopped_code(p, ptrace).is_null() { return 0; }
    let mut exit_code = 0;
    let mut uid = 0;
    b::rust_exit_spin_lock_irq(addr_of_mut!((*(*p).sighand).siglock));
    let p_code = task_stopped_code(p, ptrace);
    if !p_code.is_null() {
        exit_code = *p_code;
        if exit_code != 0 {
            if !wait_flags(wo, WNOWAIT) { *p_code = 0; }
            uid = task_wait_uid(p);
        }
    }
    b::rust_exit_spin_unlock_irq(addr_of_mut!((*(*p).sighand).siglock));
    if exit_code == 0 { return 0; }
    get_task_struct(p);
    let pid = b::__task_pid_nr_ns(p, PIDTYPE_PID, null_mut());
    let why = if ptrace { CLD_TRAPPED } else { CLD_STOPPED } as c_int;
    b::rust_exit_tasklist_read_unlock();
    b::rust_exit_sched_annotate_sleep();
    if !(*wo).wo_rusage.is_null() { b::getrusage(p, RUSAGE_BOTH as _, (*wo).wo_rusage); }
    put_task_struct(p);
    if !wait_flags(wo, WNOWAIT) { (*wo).wo_stat = exit_code.wrapping_shl(8) | 0x7f; }
    let infop = (*wo).wo_info;
    if !infop.is_null() {
        (*infop).cause = why; (*infop).status = exit_code; (*infop).pid = pid; (*infop).uid = uid;
    }
    pid
}
unsafe fn wait_task_continued(wo: *mut wait_opts, p: *mut task_struct) -> c_int {
    if !wait_flags(wo, WCONTINUED) { return 0; }
    if (*(*p).signal).flags & SIGNAL_STOP_CONTINUED == 0 { return 0; }
    b::rust_exit_spin_lock_irq(addr_of_mut!((*(*p).sighand).siglock));
    if (*(*p).signal).flags & SIGNAL_STOP_CONTINUED == 0 {
        b::rust_exit_spin_unlock_irq(addr_of_mut!((*(*p).sighand).siglock));
        return 0;
    }
    if !wait_flags(wo, WNOWAIT) { (*(*p).signal).flags &= !SIGNAL_STOP_CONTINUED; }
    let uid = task_wait_uid(p);
    b::rust_exit_spin_unlock_irq(addr_of_mut!((*(*p).sighand).siglock));
    let pid = b::__task_pid_nr_ns(p, PIDTYPE_PID, null_mut());
    get_task_struct(p);
    b::rust_exit_tasklist_read_unlock();
    b::rust_exit_sched_annotate_sleep();
    if !(*wo).wo_rusage.is_null() { b::getrusage(p, RUSAGE_BOTH as _, (*wo).wo_rusage); }
    put_task_struct(p);
    let infop = (*wo).wo_info;
    if infop.is_null() { (*wo).wo_stat = 0xffff; }
    else {
        (*infop).cause = CLD_CONTINUED as c_int; (*infop).pid = pid; (*infop).uid = uid; (*infop).status = SIGCONT as c_int;
    }
    pid
}
unsafe fn wait_consider_task(wo: *mut wait_opts, mut ptrace: bool, p: *mut task_struct) -> c_int {
    let exit_state = read_volatile(addr_of!((*p).exit_state));
    if exit_state == EXIT_DEAD as c_int || !eligible_child(wo, ptrace, p) { return 0; }
    if exit_state == EXIT_TRACE as c_int {
        if !ptrace { (*wo).notask_error = 0; }
        return 0;
    }
    if !ptrace && (*p).ptrace != 0 && !ptrace_reparented(p) { ptrace = true; }
    if exit_state == EXIT_ZOMBIE as c_int {
        if !(thread_group_leader(p) && !thread_group_empty(p)) && (ptrace || (*p).ptrace == 0) {
            return wait_task_zombie(wo, p);
        }
        if !ptrace || wait_flags(wo, WCONTINUED | WEXITED) { (*wo).notask_error = 0; }
    } else { (*wo).notask_error = 0; }
    let ret = wait_task_stopped(wo, ptrace, p);
    if ret != 0 { return ret; }
    wait_task_continued(wo, p)
}
unsafe fn do_wait_thread(wo: *mut wait_opts, tsk: *mut task_struct) -> c_int {
    let head = addr_of_mut!((*tsk).children);
    let mut node = (*head).next;
    while node != head {
        let ret = wait_consider_task(wo, false, task_from_sibling(node));
        if ret != 0 { return ret; }
        node = (*node).next;
    }
    0
}
unsafe fn ptrace_do_wait(wo: *mut wait_opts, tsk: *mut task_struct) -> c_int {
    let head = addr_of_mut!((*tsk).ptraced);
    let mut node = (*head).next;
    while node != head {
        let ret = wait_consider_task(wo, true, task_from_ptrace(node));
        if ret != 0 { return ret; }
        node = (*node).next;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn pid_child_should_wake(wo: *mut wait_opts, p: *mut task_struct) -> bool {
    eligible_pid(wo, p) && (!wait_flags(wo, __WNOTHREAD) || (*wo).child_wait.private == (*p).parent.cast())
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_child_wait_callback(wait: *mut wait_queue_entry_t, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int {
    let wo = wait.cast::<u8>().sub(offset_of!(wait_opts, child_wait)).cast::<wait_opts>();
    if pid_child_should_wake(wo, key.cast()) { b::default_wake_function(wait, mode, sync, key) } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn __wake_up_parent(p: *mut task_struct, parent: *mut task_struct) {
    b::__wake_up_sync_key(addr_of_mut!((*(*parent).signal).wait_chldexit), TASK_INTERRUPTIBLE as _, p.cast());
}
unsafe fn is_effectively_child(wo: *mut wait_opts, ptrace: bool, target: *mut task_struct) -> bool {
    let parent = if ptrace { (*target).parent } else { (*target).real_parent };
    let current = current_task();
    current == parent || (!wait_flags(wo, __WNOTHREAD) && same_thread_group(current, parent))
}
unsafe fn do_wait_pid(wo: *mut wait_opts) -> c_int {
    let target = b::pid_task((*wo).wo_pid, PIDTYPE_TGID);
    if !target.is_null() && is_effectively_child(wo, false, target) {
        let ret = wait_consider_task(wo, false, target);
        if ret != 0 { return ret; }
    }
    let target = b::pid_task((*wo).wo_pid, PIDTYPE_PID);
    if !target.is_null() && (*target).ptrace != 0 && is_effectively_child(wo, true, target) {
        let ret = wait_consider_task(wo, true, target);
        if ret != 0 { return ret; }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn __do_wait(wo: *mut wait_opts) -> c_long {
    (*wo).notask_error = -(ECHILD as c_int);
    if !((*wo).wo_type.0 < PIDTYPE_MAX.0 && ((*wo).wo_pid.is_null() || !pid_has_task((*wo).wo_pid, (*wo).wo_type))) {
        b::rust_exit_tasklist_read_lock();
        if (*wo).wo_type == PIDTYPE_PID {
            let ret = do_wait_pid(wo);
            if ret != 0 { return ret as c_long; }
        } else {
            let current = current_task();
            let mut tsk = current;
            loop {
                let ret = do_wait_thread(wo, tsk);
                if ret != 0 { return ret as c_long; }
                let ret = ptrace_do_wait(wo, tsk);
                if ret != 0 { return ret as c_long; }
                if wait_flags(wo, __WNOTHREAD) { break; }
                tsk = next_thread(tsk);
                if tsk == current { break; }
            }
        }
        b::rust_exit_tasklist_read_unlock();
    }
    let ret = (*wo).notask_error as c_long;
    if ret == 0 && !wait_flags(wo, WNOHANG) { -(ERESTARTSYS as c_long) } else { ret }
}
unsafe fn do_wait(wo: *mut wait_opts) -> c_long {
    b::rust_exit_trace_wait((*wo).wo_pid);
    let current = current_task();
    (*wo).child_wait.flags = 0;
    (*wo).child_wait.private = current.cast();
    (*wo).child_wait.func = Some(b::rust_exit_child_wait_callback_native);
    b::add_wait_queue(addr_of_mut!((*(*current).signal).wait_chldexit), addr_of_mut!((*wo).child_wait));
    let retval = loop {
        b::rust_exit_set_state(TASK_INTERRUPTIBLE as _);
        let ret = __do_wait(wo) as c_int;
        if ret != -(ERESTARTSYS as c_int) || signal_pending(current) { break ret; }
        b::schedule();
    };
    b::rust_exit_set_state_unbarriered(TASK_RUNNING as _);
    b::remove_wait_queue(addr_of_mut!((*(*current).signal).wait_chldexit), addr_of_mut!((*wo).child_wait));
    retval as c_long
}
#[no_mangle]
pub unsafe extern "C" fn kernel_waitid_prepare(wo: *mut wait_opts, which: c_int, upid: pid_t, infop: *mut waitid_info, options: c_int, ru: *mut rusage) -> c_int {
    let valid = WNOHANG | WNOWAIT | WEXITED | WSTOPPED | WCONTINUED | __WNOTHREAD | __WCLONE | __WALL;
    if (options as c_uint) & !valid != 0 || (options as c_uint) & (WEXITED | WSTOPPED | WCONTINUED) == 0 { return -(EINVAL as c_int); }
    let mut f_flags = 0;
    let mut pid = null_mut();
    let kind;
    if which == P_ALL as c_int { kind = PIDTYPE_MAX; }
    else if which == P_PID as c_int {
        kind = PIDTYPE_PID;
        if upid <= 0 { return -(EINVAL as c_int); }
        pid = b::find_get_pid(upid);
    } else if which == P_PGID as c_int {
        kind = PIDTYPE_PGID;
        if upid < 0 { return -(EINVAL as c_int); }
        pid = if upid != 0 { b::find_get_pid(upid) } else { b::get_task_pid(current_task(), PIDTYPE_PGID) };
    } else if which == P_PIDFD as c_int {
        kind = PIDTYPE_PID;
        if upid < 0 { return -(EINVAL as c_int); }
        pid = b::pidfd_get_pid(upid as _, &mut f_flags);
        if is_err(pid) { return pid as c_long as c_int; }
    } else { return -(EINVAL as c_int); }
    (*wo).wo_type = kind; (*wo).wo_pid = pid; (*wo).wo_flags = options;
    (*wo).wo_info = infop; (*wo).wo_rusage = ru;
    if f_flags & O_NONBLOCK != 0 { (*wo).wo_flags |= WNOHANG as c_int; }
    0
}
unsafe fn kernel_waitid(which: c_int, upid: pid_t, infop: *mut waitid_info, options: c_int, ru: *mut rusage) -> c_long {
    let mut wo: wait_opts = zeroed();
    let ret = kernel_waitid_prepare(&mut wo, which, upid, infop, options, ru);
    if ret != 0 { return ret as c_long; }
    let mut ret = do_wait(&mut wo);
    if ret == 0 && (options as c_uint) & WNOHANG == 0 && wait_flags(&mut wo, WNOHANG) { ret = -(EAGAIN as c_long); }
    b::put_pid(wo.wo_pid);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_sys_waitid(which: c_int, upid: pid_t, infop: *mut siginfo, options: c_int, ru: *mut rusage) -> c_long {
    let mut r: rusage = zeroed();
    let mut info: waitid_info = zeroed();
    let mut err = kernel_waitid(which, upid, &mut info, options, if ru.is_null() { null_mut() } else { &mut r });
    let mut signo = 0;
    if err > 0 {
        signo = SIGCHLD as c_int;
        err = 0;
        if !ru.is_null() && b::rust_exit_copy_to_user(ru.cast(), addr_of!(r).cast(), size_of::<rusage>() as _) != 0 { return -(EFAULT as c_long); }
    }
    if infop.is_null() { return err; }
    write_wait_siginfo(infop.cast(), false, signo, &info, err)
}
#[no_mangle]
pub unsafe extern "C" fn kernel_wait4(upid: pid_t, stat_addr: *mut c_int, options: c_int, ru: *mut rusage) -> c_long {
    let valid = WNOHANG | WUNTRACED | WCONTINUED | __WNOTHREAD | __WCLONE | __WALL;
    if (options as c_uint) & !valid != 0 { return -(EINVAL as c_long); }
    if upid == c_int::MIN { return -(ESRCH as c_long); }
    let kind;
    let pid;
    if upid == -1 { kind = PIDTYPE_MAX; pid = null_mut(); }
    else if upid < 0 { kind = PIDTYPE_PGID; pid = b::find_get_pid(-upid); }
    else if upid == 0 { kind = PIDTYPE_PGID; pid = b::get_task_pid(current_task(), PIDTYPE_PGID); }
    else { kind = PIDTYPE_PID; pid = b::find_get_pid(upid); }
    let mut wo: wait_opts = zeroed();
    wo.wo_type = kind; wo.wo_pid = pid; wo.wo_flags = options | WEXITED as c_int; wo.wo_rusage = ru;
    let mut ret = do_wait(&mut wo);
    b::put_pid(pid);
    if ret > 0 && !stat_addr.is_null() && b::rust_exit_put_user_int(wo.wo_stat, stat_addr) != 0 { ret = -(EFAULT as c_long); }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn kernel_wait(pid: pid_t, stat: *mut c_int) -> c_int {
    let mut wo: wait_opts = zeroed();
    wo.wo_type = PIDTYPE_PID; wo.wo_pid = b::find_get_pid(pid); wo.wo_flags = WEXITED as c_int;
    let ret = do_wait(&mut wo) as c_int;
    if ret > 0 && wo.wo_stat != 0 { *stat = wo.wo_stat; }
    b::put_pid(wo.wo_pid);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn rust_exit_sys_wait4(upid: pid_t, stat_addr: *mut c_int, options: c_int, ru: *mut rusage) -> c_long {
    let mut r: rusage = zeroed();
    let err = kernel_wait4(upid, stat_addr, options, if ru.is_null() { null_mut() } else { &mut r });
    if err > 0 && !ru.is_null() && b::rust_exit_copy_to_user(ru.cast(), addr_of!(r).cast(), size_of::<rusage>() as _) != 0 { return -(EFAULT as c_long); }
    err
}
// Native syscall metadata is guarded by __ARCH_WANT_SYS_WAITPID. Keeping the
// harmless Rust target present avoids inventing a Rust cfg for an arch macro.
#[no_mangle]
pub unsafe extern "C" fn rust_exit_sys_waitpid(pid: pid_t, stat_addr: *mut c_int, options: c_int) -> c_long {
    kernel_wait4(pid, stat_addr, options, null_mut())
}
#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn rust_exit_compat_wait4(pid: compat_pid_t, stat_addr: *mut compat_uint_t, options: c_int, ru: *mut compat_rusage) -> c_long {
    let mut r: rusage = zeroed();
    let err = kernel_wait4(pid, stat_addr.cast(), options, if ru.is_null() { null_mut() } else { &mut r });
    if err > 0 && !ru.is_null() && b::put_compat_rusage(&mut r, ru) != 0 { return -(EFAULT as c_long); }
    err
}
#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn rust_exit_compat_waitid(which: c_int, pid: compat_pid_t, infop: *mut compat_siginfo, options: c_int, uru: *mut compat_rusage) -> c_long {
    let mut ru: rusage = zeroed();
    let mut info: waitid_info = zeroed();
    let mut err = kernel_waitid(which, pid, &mut info, options, if uru.is_null() { null_mut() } else { &mut ru });
    let mut signo = 0;
    if err > 0 {
        signo = SIGCHLD as c_int;
        err = 0;
        if !uru.is_null() {
            err = if b::rust_exit_compat_64bit_time() {
                b::rust_exit_copy_to_user(uru.cast(), addr_of!(ru).cast(), size_of::<rusage>() as _) as c_long
            } else { b::put_compat_rusage(&mut ru, uru) as c_long };
            if err != 0 { return -(EFAULT as c_long); }
        }
    }
    if infop.is_null() { return err; }
    write_wait_siginfo(infop.cast(), true, signo, &info, err)
}
// Linux's six unsafe_put_user operations stay ordered and do not zero padding.
// The primitive owns one fixed native exception-table transaction; Rust chooses
// each address and value, the selected ABI extent, and syscall error policy.
unsafe fn write_wait_siginfo(infop: *mut c_void, compat: bool, signo: c_int, info: &waitid_info, err: c_long) -> c_long {
    let size = if compat { RUST_EXIT_COMPAT_SIGINFO_SIZE } else { size_of::<siginfo>() as _ };
    let offsets = if compat { [RUST_EXIT_COMPAT_SI_SIGNO, RUST_EXIT_COMPAT_SI_ERRNO, RUST_EXIT_COMPAT_SI_CODE, RUST_EXIT_COMPAT_SI_PID, RUST_EXIT_COMPAT_SI_UID, RUST_EXIT_COMPAT_SI_STATUS] }
        else { [RUST_EXIT_SI_SIGNO, RUST_EXIT_SI_ERRNO, RUST_EXIT_SI_CODE, RUST_EXIT_SI_PID, RUST_EXIT_SI_UID, RUST_EXIT_SI_STATUS] };
    let values = [signo as c_uint, 0, info.cause as c_uint, info.pid as c_uint, info.uid as c_uint, info.status as c_uint];
    let fields = offsets.map(|offset| infop.cast::<u8>().wrapping_add(offset as usize).cast::<c_uint>());
    if b::rust_exit_user_write_six(infop, size as _, fields.as_ptr(), values.as_ptr()) != 0 { -(EFAULT as c_long) } else { err }
}
