// SPDX-License-Identifier: GPL-2.0-or-later

/* Kernel dependencies supplied by the surrounding futex implementation. */

/*
 * Support for robust futexes: the kernel cleans up held futexes at
 * thread exit time.
 *
 * Implementation: user-space maintains a per-thread list of locks it
 * is holding. Upon do_exit(), the kernel carefully walks this list,
 * and marks all locks that are owned by this thread with the FUTEX_OWNER_DIED bit,
 * and wakes up a waiter (if any). The list is always manipulated with the lock
 * held, so the list is private and per-thread. Userspace also maintains a
 * per-thread 'list_op_pending' field, to allow the kernel to clean up if the
 * thread dies after acquiring the lock, but just before it could have added
 * itself to the list. There can only be one such pending lock.
 */

pub unsafe fn set_robust_list(head: *mut robust_list_head, len: usize) -> c_long {
    if unlikely(len != core::mem::size_of::<robust_list_head>()) { return -EINVAL; }
    (*current).futex.robust_list = head;
    0
}

#[inline]
unsafe fn futex_task_robust_list(p: *mut task_struct, compat: bool) -> *mut core::ffi::c_void {
    #[cfg(CONFIG_COMPAT)]
    if compat { return (*p).futex.compat_robust_list as *mut core::ffi::c_void; }
    (*p).futex.robust_list as *mut core::ffi::c_void
}

unsafe fn futex_get_robust_list_common(pid: i32, compat: bool) -> *mut core::ffi::c_void {
    let mut p = current;
    let head: *mut core::ffi::c_void;
    let mut ret: c_int;
    unsafe { scoped_guard_rcu(|| {
        if pid != 0 {
            p = find_task_by_vpid(pid);
            if p.is_null() { return Err((-ESRCH) as c_long); }
        }
        get_task_struct(p);
        Ok(())
    }).unwrap_or_else(|e| return e as _); }
    ret = down_read_killable(&(*(*p).signal).exec_update_lock);
    if ret != 0 { put_task_struct(p); return ERR_PTR(ret as _); }
    ret = -EPERM;
    if !ptrace_may_access(p, PTRACE_MODE_READ_REALCREDS) {
        up_read(&(*(*p).signal).exec_update_lock); put_task_struct(p); return ERR_PTR(ret as _);
    }
    head = futex_task_robust_list(p, compat);
    up_read(&(*(*p).signal).exec_update_lock);
    put_task_struct(p);
    head
}

pub unsafe fn get_robust_list(pid: i32, head_ptr: *mut *mut robust_list_head,
                              len_ptr: *mut usize) -> c_long {
    let head = futex_get_robust_list_common(pid, false);
    if IS_ERR(head) { return PTR_ERR(head); }
    if put_user(core::mem::size_of::<robust_list_head>(), len_ptr) != 0 { return -EFAULT; }
    put_user(head as *mut robust_list_head, head_ptr)
}

pub unsafe fn do_futex(uaddr: *mut u32, op: c_int, val: u32, timeout: *mut ktime_t,
                       uaddr2: *mut u32, val2: u32, mut val3: u32) -> c_long {
    let mut flags = futex_to_flags(op);
    let cmd = op & FUTEX_CMD_MASK;
    if flags & FLAGS_CLOCKRT != 0 && cmd != FUTEX_WAIT_BITSET && cmd != FUTEX_WAIT_REQUEUE_PI && cmd != FUTEX_LOCK_PI2 { return -ENOSYS; }
    if flags & FLAGS_ROBUST_UNLOCK != 0 && cmd != FUTEX_WAKE && cmd != FUTEX_WAKE_BITSET && cmd != FUTEX_UNLOCK_PI { return -ENOSYS; }
    match cmd {
        FUTEX_WAIT => { val3 = FUTEX_BITSET_MATCH_ANY; futex_wait(uaddr, flags, val, timeout, val3) }
        FUTEX_WAIT_BITSET => futex_wait(uaddr, flags, val, timeout, val3),
        FUTEX_WAKE => { val3 = FUTEX_BITSET_MATCH_ANY; futex_wake(uaddr, flags, uaddr2, val, val3) }
        FUTEX_WAKE_BITSET => futex_wake(uaddr, flags, uaddr2, val, val3),
        FUTEX_REQUEUE => futex_requeue(uaddr, flags, uaddr2, flags, val, val2, core::ptr::null_mut(), 0),
        FUTEX_CMP_REQUEUE => futex_requeue(uaddr, flags, uaddr2, flags, val, val2, &mut val3, 0),
        FUTEX_WAKE_OP => futex_wake_op(uaddr, flags, uaddr2, val, val2, val3),
        FUTEX_LOCK_PI => { flags |= FLAGS_CLOCKRT; futex_lock_pi(uaddr, flags, timeout, 0) }
        FUTEX_LOCK_PI2 => futex_lock_pi(uaddr, flags, timeout, 0),
        FUTEX_UNLOCK_PI => futex_unlock_pi(uaddr, flags, uaddr2),
        FUTEX_TRYLOCK_PI => futex_lock_pi(uaddr, flags, core::ptr::null_mut(), 1),
        FUTEX_WAIT_REQUEUE_PI => futex_wait_requeue_pi(uaddr, flags, val, timeout, FUTEX_BITSET_MATCH_ANY, uaddr2),
        FUTEX_CMP_REQUEUE_PI => futex_requeue(uaddr, flags, uaddr2, flags, val, val2, &mut val3, 1),
        _ => -ENOSYS,
    }
}

#[inline(always)] unsafe fn futex_cmd_has_timeout(cmd: u32) -> bool {
    matches!(cmd, FUTEX_WAIT | FUTEX_LOCK_PI | FUTEX_LOCK_PI2 | FUTEX_WAIT_BITSET | FUTEX_WAIT_REQUEUE_PI)
}

#[inline(always)] unsafe fn futex_init_timeout(cmd: u32, op: u32, ts: *mut timespec64, t: *mut ktime_t) -> c_int {
    if !timespec64_valid(ts) { return -EINVAL; }
    *t = timespec64_to_ktime(*ts);
    if cmd == FUTEX_WAIT { *t = ktime_add_safe(ktime_get(), *t); }
    else if cmd != FUTEX_LOCK_PI && op & FUTEX_CLOCK_REALTIME == 0 { *t = timens_ktime_to_host(CLOCK_MONOTONIC, *t); }
    0
}

pub unsafe fn futex(uaddr: *mut u32, op: c_int, val: u32, utime: *const kernel_timespec,
                    uaddr2: *mut u32, val3: u32) -> c_long {
    let cmd = op & FUTEX_CMD_MASK; let mut t = ktime_t::default(); let mut ts = timespec64::default();
    let mut tp = core::ptr::null_mut();
    if !utime.is_null() && futex_cmd_has_timeout(cmd as u32) {
        if should_fail_futex(!(op & FUTEX_PRIVATE_FLAG != 0)) { return -EFAULT; }
        if get_timespec64(&mut ts, utime) != 0 { return -EFAULT; }
        let ret = futex_init_timeout(cmd as u32, op as u32, &mut ts, &mut t); if ret != 0 { return ret as _; } tp = &mut t;
    }
    do_futex(uaddr, op, val, tp, uaddr2, utime as usize as u32, val3)
}

pub unsafe fn futex_parse_waitv(futexv: *mut futex_vector, uwaitv: *mut futex_waitv,
                                nr_futexes: u32, wake: futex_wake_fn, wake_data: *mut core::ffi::c_void) -> c_int {
    for i in 0..nr_futexes {
        let mut aux = core::mem::zeroed::<futex_waitv>();
        if copy_from_user(&mut aux, uwaitv.add(i as usize), core::mem::size_of::<futex_waitv>()) != 0 { return -EFAULT; }
        if aux.flags & !FUTEX2_VALID_MASK != 0 || aux.__reserved != 0 { return -EINVAL; }
        let flags = futex2_to_flags(aux.flags); if !futex_flags_valid(flags) || !futex_validate_input(flags, aux.val) { return -EINVAL; }
        (*futexv.add(i as usize)).w.flags = flags; (*futexv.add(i as usize)).w.val = aux.val; (*futexv.add(i as usize)).w.uaddr = aux.uaddr;
        (*futexv.add(i as usize)).q = futex_q_init; (*futexv.add(i as usize)).q.wake = wake; (*futexv.add(i as usize)).q.wake_data = wake_data;
    }
    0
}

/* The remaining futex2 syscall wrappers retain the kernel's external helper calls. */
pub unsafe fn futex_wake(uaddr: *mut core::ffi::c_void, mask: usize, nr: c_int, mut flags: u32) -> c_long {
    if flags & !FUTEX2_VALID_MASK != 0 { return -EINVAL; } flags = futex2_to_flags(flags);
    if !futex_flags_valid(flags) || !futex_validate_input(flags, mask as u64) { return -EINVAL; }
    crate::futex_wake(uaddr, FLAGS_STRICT | flags, core::ptr::null_mut(), nr, mask as u32)
}

unsafe fn futex2_setup_timeout(timeout: *mut kernel_timespec, clockid: clockid_t,
                               to: *mut hrtimer_sleeper) -> c_int {
    if timeout.is_null() { return 0; }
    let (mut flag_clkid, mut flag_init) = (0, 0); let mut ts = timespec64::default(); let mut time = ktime_t::default();
    if clockid == CLOCK_REALTIME { flag_clkid = FLAGS_CLOCKRT; flag_init = FUTEX_CLOCK_REALTIME; }
    if clockid != CLOCK_REALTIME && clockid != CLOCK_MONOTONIC { return -EINVAL; }
    if get_timespec64(&mut ts, timeout) != 0 { return -EFAULT; }
    let ret = futex_init_timeout(FUTEX_WAIT_BITSET, flag_init, &mut ts, &mut time); if ret != 0 { return ret; }
    futex_setup_timer(&mut time, to, flag_clkid, 0); 0
}

#[inline] unsafe fn futex2_destroy_timeout(to: *mut hrtimer_sleeper) {
    hrtimer_cancel(&mut (*to).timer); destroy_hrtimer_on_stack(&mut (*to).timer);
}

pub unsafe fn futex_waitv(waiters: *mut futex_waitv, nr_futexes: u32, flags: u32,
                          timeout: *mut kernel_timespec, clockid: clockid_t) -> c_long {
    let mut to = core::mem::zeroed::<hrtimer_sleeper>();
    if flags != 0 || nr_futexes == 0 || nr_futexes > FUTEX_WAITV_MAX || waiters.is_null() { return -EINVAL; }
    if !timeout.is_null() { let ret = futex2_setup_timeout(timeout, clockid, &mut to); if ret != 0 { return ret as _; } }
    let futexv = kzalloc_futex_vector(nr_futexes); if futexv.is_null() { if !timeout.is_null() { futex2_destroy_timeout(&mut to); } return -ENOMEM; }
    let mut ret = futex_parse_waitv(futexv, waiters, nr_futexes, futex_wake_mark, core::ptr::null_mut());
    if ret == 0 { ret = futex_wait_multiple(futexv, nr_futexes, if timeout.is_null() { core::ptr::null_mut() } else { &mut to }); }
    kfree(futexv); if !timeout.is_null() { futex2_destroy_timeout(&mut to); } ret as _
}

pub unsafe fn futex_wait(uaddr: *mut core::ffi::c_void, val: usize, mask: usize, mut flags: u32,
                         timeout: *mut kernel_timespec, clockid: clockid_t) -> c_long {
    let mut to = core::mem::zeroed::<hrtimer_sleeper>();
    if flags & !FUTEX2_VALID_MASK != 0 { return -EINVAL; } flags = futex2_to_flags(flags);
    if !futex_flags_valid(flags) || !futex_validate_input(flags, val as u64) || !futex_validate_input(flags, mask as u64) { return -EINVAL; }
    if !timeout.is_null() { let ret = futex2_setup_timeout(timeout, clockid, &mut to); if ret != 0 { return ret as _; } }
    let ret = __futex_wait(uaddr, flags, val as u32, if timeout.is_null() { core::ptr::null_mut() } else { &mut to }, mask as u32);
    if !timeout.is_null() { futex2_destroy_timeout(&mut to); } ret
}

pub unsafe fn futex_requeue(waiters: *mut futex_waitv, flags: u32, nr_wake: c_int, nr_requeue: c_int) -> c_long {
    if flags != 0 || waiters.is_null() { return -EINVAL; }
    let mut futexes = [core::mem::zeroed::<futex_vector>(); 2];
    let ret = futex_parse_waitv(futexes.as_mut_ptr(), waiters, 2, futex_wake_mark, core::ptr::null_mut()); if ret != 0 { return ret as _; }
    if futexes[0].w.flags != futexes[1].w.flags { return -EINVAL; }
    let mut cmpval = futexes[0].w.val;
    crate::futex_requeue(u64_to_user_ptr(futexes[0].w.uaddr), futexes[0].w.flags,
                         u64_to_user_ptr(futexes[1].w.uaddr), futexes[1].w.flags,
                         nr_wake, nr_requeue, &mut cmpval, 0)
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_set_robust_list(head: *mut compat_robust_list_head, len: compat_size_t) -> c_long {
    if unlikely(len != core::mem::size_of::<compat_robust_list_head>()) { return -EINVAL; }
    (*current).futex.compat_robust_list = head; 0
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_get_robust_list(pid: c_int, head_ptr: *mut compat_uptr_t, len_ptr: *mut compat_size_t) -> c_long {
    let head = futex_get_robust_list_common(pid, true); if IS_ERR(head) { return PTR_ERR(head); }
    if put_user(core::mem::size_of::<compat_robust_list_head>(), len_ptr) != 0 { return -EFAULT; }
    put_user(ptr_to_compat(head as *mut compat_robust_list_head), head_ptr)
}

#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn futex_time32(uaddr: *mut u32, op: c_int, val: u32, utime: *const old_timespec32,
                           uaddr2: *mut u32, val3: u32) -> c_long {
    let cmd = op & FUTEX_CMD_MASK; let mut t = ktime_t::default(); let mut ts = timespec64::default(); let mut tp = core::ptr::null_mut();
    if !utime.is_null() && futex_cmd_has_timeout(cmd as u32) {
        if get_old_timespec32(&mut ts, utime) != 0 { return -EFAULT; }
        let ret = futex_init_timeout(cmd as u32, op as u32, &mut ts, &mut t); if ret != 0 { return ret as _; } tp = &mut t;
    }
    do_futex(uaddr, op, val, tp, uaddr2, utime as usize as u32, val3)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
