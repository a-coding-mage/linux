// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/signalfd.c
 *
 *  Copyright (C) 2003  Linus Torvalds
 *
 *  Mon Mar 5, 2007: Davide Libenzi <davidel@xmailserver.org>
 *      Changed ->read() to return a siginfo strcture instead of signal number.
 *      Fixed locking in ->poll().
 *      Added sighand-detach notification.
 *      Added fd re-use in sys_signalfd() syscall.
 *      Now using anonymous inode source.
 *      Thanks to Oleg Nesterov for useful code review and suggestions.
 *      More comments and suggestions from Arnd Bergmann.
 *  Sat May 19, 2007: Davi E. M. Arnaut <davi@haxent.com.br>
 *      Retrieve multiple signals with one read() call
 *  Sun Jul 15, 2007: Davide Libenzi <davidel@xmailserver.org>
 *      Attach to the sighand only during read() and poll().
 */

// Linux kernel dependencies are supplied by other translation units.

pub unsafe fn signalfd_cleanup(sighand: *mut sighand_struct) {
    wake_up_pollfree(unsafe { &mut (*sighand).signalfd_wqh });
}

#[repr(C)]
pub struct signalfd_ctx {
    pub sigmask: sigset_t,
}

unsafe fn signalfd_release(_inode: *mut inode, file: *mut file) -> c_int {
    kfree(unsafe { (*file).private_data });
    0
}

unsafe fn signalfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let ctx = (*file).private_data as *mut signalfd_ctx;
    let mut events: __poll_t = 0;

    poll_wait(file, &mut (*current).sighand.signalfd_wqh, wait);
    spin_lock_irq(&mut (*current).sighand.siglock);
    if next_signal(&mut (*current).pending, &mut (*ctx).sigmask) != 0
        || next_signal(&mut (*current).signal.shared_pending, &mut (*ctx).sigmask) != 0
    {
        events |= EPOLLIN;
    }
    spin_unlock_irq(&mut (*current).sighand.siglock);
    events
}

/* Copied from copy_siginfo_to_user() in kernel/signal.c */
unsafe fn signalfd_copyinfo(to: *mut iov_iter, kinfo: *const kernel_siginfo_t) -> c_int {
    let mut new: signalfd_siginfo = core::mem::zeroed();
    // BUILD_BUG_ON(sizeof(struct signalfd_siginfo) != 128);
    new.ssi_signo = (*kinfo).si_signo;
    new.ssi_errno = (*kinfo).si_errno;
    new.ssi_code = (*kinfo).si_code;
    match siginfo_layout((*kinfo).si_signo, (*kinfo).si_code) {
        SIL_KILL => { new.ssi_pid = (*kinfo).si_pid; new.ssi_uid = (*kinfo).si_uid; }
        SIL_TIMER => { new.ssi_tid = (*kinfo).si_tid; new.ssi_overrun = (*kinfo).si_overrun; new.ssi_ptr = (*kinfo).si_ptr as c_long; new.ssi_int = (*kinfo).si_int; }
        SIL_POLL => { new.ssi_band = (*kinfo).si_band; new.ssi_fd = (*kinfo).si_fd; }
        SIL_FAULT_BNDERR | SIL_FAULT_PKUERR | SIL_FAULT_PERF_EVENT | SIL_FAULT => { new.ssi_addr = (*kinfo).si_addr as c_long; }
        SIL_FAULT_TRAPNO => { new.ssi_addr = (*kinfo).si_addr as c_long; new.ssi_trapno = (*kinfo).si_trapno; }
        SIL_FAULT_MCEERR => { new.ssi_addr = (*kinfo).si_addr as c_long; new.ssi_addr_lsb = (*kinfo).si_addr_lsb as c_short; }
        SIL_CHLD => { new.ssi_pid = (*kinfo).si_pid; new.ssi_uid = (*kinfo).si_uid; new.ssi_status = (*kinfo).si_status; new.ssi_utime = (*kinfo).si_utime; new.ssi_stime = (*kinfo).si_stime; }
        SIL_RT => { new.ssi_pid = (*kinfo).si_pid; new.ssi_uid = (*kinfo).si_uid; new.ssi_ptr = (*kinfo).si_ptr as c_long; new.ssi_int = (*kinfo).si_int; }
        SIL_SYS => { new.ssi_call_addr = (*kinfo).si_call_addr as c_long; new.ssi_syscall = (*kinfo).si_syscall; new.ssi_arch = (*kinfo).si_arch; }
        _ => {}
    }
    if !copy_to_iter_full(&new as *const _ as *const c_void, core::mem::size_of::<signalfd_siginfo>(), to) { return -EFAULT; }
    core::mem::size_of::<signalfd_siginfo>() as c_int
}

unsafe fn signalfd_dequeue(ctx: *mut signalfd_ctx, info: *mut kernel_siginfo_t, nonblock: bool) -> ssize_t {
    let mut typ: pid_type = core::mem::zeroed();
    spin_lock_irq(&mut (*current).sighand.siglock);
    let mut ret = dequeue_signal(&(*ctx).sigmask, info, &mut typ);
    if ret == 0 && nonblock { spin_unlock_irq(&mut (*current).sighand.siglock); return -EAGAIN; }
    if ret != 0 { spin_unlock_irq(&mut (*current).sighand.siglock); return ret; }
    let mut wait = wait_queue_entry::new(current);
    add_wait_queue(&mut (*current).sighand.signalfd_wqh, &mut wait);
    loop {
        set_current_state(TASK_INTERRUPTIBLE);
        ret = dequeue_signal(&(*ctx).sigmask, info, &mut typ);
        if ret != 0 || signal_pending(current) { if ret == 0 { ret = -ERESTARTSYS; } break; }
        spin_unlock_irq(&mut (*current).sighand.siglock); schedule(); spin_lock_irq(&mut (*current).sighand.siglock);
    }
    spin_unlock_irq(&mut (*current).sighand.siglock);
    remove_wait_queue(&mut (*current).sighand.signalfd_wqh, &mut wait);
    __set_current_state(TASK_RUNNING);
    ret
}

// The remaining syscall and file-operation glue is a direct declaration-level translation;
// kernel-provided types and helpers are intentionally left external.
unsafe fn signalfd_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let ctx = (*file).private_data as *mut signalfd_ctx;
    let mut count = iov_iter_count(to) / core::mem::size_of::<signalfd_siginfo>();
    if count == 0 { return -EINVAL; }
    let mut total: ssize_t = 0;
    let mut ret;
    let mut info: kernel_siginfo_t = core::mem::zeroed();
    let mut nonblock = ((*file).f_flags & O_NONBLOCK) != 0 || ((*iocb).ki_flags & IOCB_NOWAIT) != 0;
    loop { ret = signalfd_dequeue(ctx, &mut info, nonblock); if ret <= 0 { break; } ret = signalfd_copyinfo(to, &info); if ret < 0 { break; } total += ret; nonblock = true; count -= 1; if count == 0 { break; } }
    if total != 0 { total } else { ret }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn signalfd_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let ctx = (*f).private_data as *mut signalfd_ctx;
    let mut sigmask = (*ctx).sigmask;
    signotset(&mut sigmask);
    render_sigset_t(m, "sigmask:\t", &sigmask);
}

#[repr(C)]
static signalfd_fops: file_operations = file_operations {
    #[cfg(CONFIG_PROC_FS)]
    show_fdinfo: Some(signalfd_show_fdinfo),
    release: Some(signalfd_release),
    poll: Some(signalfd_poll),
    read_iter: Some(signalfd_read_iter),
    llseek: Some(noop_llseek),
};

unsafe fn do_signalfd4(ufd: c_int, mask: *mut sigset_t, flags: c_int) -> c_int {
    // BUILD_BUG_ON(SFD_CLOEXEC != O_CLOEXEC);
    // BUILD_BUG_ON(SFD_NONBLOCK != O_NONBLOCK);
    if flags & !(SFD_CLOEXEC | SFD_NONBLOCK) != 0 { return -EINVAL; }
    sigdelsetmask(mask, sigmask(SIGKILL) | sigmask(SIGSTOP));
    signotset(mask);
    if ufd == -1 {
        let ctx = kmalloc::<signalfd_ctx>();
        if ctx.is_null() { return -ENOMEM; }
        (*ctx).sigmask = *mask;
        let fd = fd_add(flags & O_CLOEXEC, anon_inode_getfile_fmode(
            b"[signalfd]\0".as_ptr() as *const c_char, &signalfd_fops, ctx as *mut c_void,
            O_RDWR | (flags & O_NONBLOCK), FMODE_NOWAIT));
        if fd >= 0 { retain_and_null_ptr(ctx); }
        fd
    } else {
        let f = fd_get(ufd);
        if fd_empty(f) { return -EBADF; }
        let ctx = fd_file(f).private_data as *mut signalfd_ctx;
        if (*fd_file(f)).f_op != &signalfd_fops { return -EINVAL; }
        spin_lock_irq(&mut (*current).sighand.siglock);
        (*ctx).sigmask = *mask;
        spin_unlock_irq(&mut (*current).sighand.siglock);
        wake_up(&mut (*current).sighand.signalfd_wqh);
        ufd
    }
}

pub unsafe fn sys_signalfd4(ufd: c_int, user_mask: *mut sigset_t, sizemask: size_t, flags: c_int) -> c_int {
    if sizemask != core::mem::size_of::<sigset_t>() { return -EINVAL; }
    let mut mask: sigset_t = core::mem::zeroed();
    if copy_from_user(&mut mask, user_mask, core::mem::size_of::<sigset_t>()) != 0 { return -EFAULT; }
    do_signalfd4(ufd, &mut mask, flags)
}

pub unsafe fn sys_signalfd(ufd: c_int, user_mask: *mut sigset_t, sizemask: size_t) -> c_int {
    sys_signalfd4(ufd, user_mask, sizemask, 0)
}

#[cfg(CONFIG_COMPAT)]
unsafe fn do_compat_signalfd4(ufd: c_int, user_mask: *const compat_sigset_t, sigsetsize: compat_size_t, flags: c_int) -> c_long {
    if sigsetsize != core::mem::size_of::<compat_sigset_t>() { return -EINVAL as c_long; }
    let mut mask: sigset_t = core::mem::zeroed();
    if get_compat_sigset(&mut mask, user_mask) != 0 { return -EFAULT as c_long; }
    do_signalfd4(ufd, &mut mask, flags) as c_long
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_sys_signalfd4(ufd: c_int, user_mask: *const compat_sigset_t, sigsetsize: compat_size_t, flags: c_int) -> c_long { do_compat_signalfd4(ufd, user_mask, sigsetsize, flags) }

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_sys_signalfd(ufd: c_int, user_mask: *const compat_sigset_t, sigsetsize: compat_size_t) -> c_long { do_compat_signalfd4(ufd, user_mask, sigsetsize, 0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
