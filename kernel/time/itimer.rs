// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992 Darren Senn
 */

/* These are all the functions necessary to implement itimers */

unsafe fn itimer_get_remtime(timer: *mut hrtimer) -> timespec64 {
    let mut rem: ktime_t = __hrtimer_get_remaining(timer, true);
    if hrtimer_active(timer) {
        if rem <= 0 { rem = NSEC_PER_USEC; }
    } else { rem = 0; }
    ktime_to_timespec64(rem)
}

unsafe fn get_cpu_itimer(tsk: *mut task_struct, clock_id: c_uint, value: *mut itimerspec64) {
    let mut val: u64;
    let mut interval: u64;
    let it: *mut cpu_itimer = &mut (*(*(*tsk).signal).it.as_mut_ptr().add(clock_id as usize));
    spin_lock_irq((*(*tsk).sighand).siglock);
    val = (*it).expires;
    interval = (*it).incr;
    if val != 0 {
        let mut samples: [u64; CPUCLOCK_MAX as usize] = [0; CPUCLOCK_MAX as usize];
        thread_group_sample_cputime(tsk, samples.as_mut_ptr());
        let t = samples[clock_id as usize];
        if val < t { val = TICK_NSEC; } else { val -= t; }
    }
    spin_unlock_irq((*(*tsk).sighand).siglock);
    (*value).it_value = ns_to_timespec64(val);
    (*value).it_interval = ns_to_timespec64(interval);
}

unsafe fn do_getitimer(which: c_int, value: *mut itimerspec64) -> c_int {
    let tsk = current;
    match which {
        ITIMER_REAL => {
            spin_lock_irq((*(*tsk).sighand).siglock);
            (*value).it_value = itimer_get_remtime(&mut (*(*tsk).signal).real_timer);
            (*value).it_interval = ktime_to_timespec64((*(*tsk).signal).it_real_incr);
            spin_unlock_irq((*(*tsk).sighand).siglock);
        }
        ITIMER_VIRTUAL => get_cpu_itimer(tsk, CPUCLOCK_VIRT, value),
        ITIMER_PROF => get_cpu_itimer(tsk, CPUCLOCK_PROF, value),
        _ => return -EINVAL,
    }
    0
}

unsafe fn put_itimerval(o: *mut __kernel_old_itimerval, i: *const itimerspec64) -> c_int {
    let mut v: __kernel_old_itimerval = core::mem::zeroed();
    v.it_interval.tv_sec = (*i).it_interval.tv_sec;
    v.it_interval.tv_usec = (*i).it_interval.tv_nsec / NSEC_PER_USEC;
    v.it_value.tv_sec = (*i).it_value.tv_sec;
    v.it_value.tv_usec = (*i).it_value.tv_nsec / NSEC_PER_USEC;
    if copy_to_user(o as *mut c_void, &v as *const _ as *const c_void, core::mem::size_of::<__kernel_old_itimerval>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn getitimer(which: c_int, value: *mut __kernel_old_itimerval) -> c_int {
    let mut get_buffer: itimerspec64 = core::mem::zeroed();
    let mut error = do_getitimer(which, &mut get_buffer);
    if error == 0 && put_itimerval(value, &get_buffer) != 0 { error = -EFAULT; }
    error
}

#[cfg(any(feature = "CONFIG_COMPAT", feature = "CONFIG_ALPHA"))]
#[repr(C)]
struct old_itimerval32 { it_interval: old_timeval32, it_value: old_timeval32 }

#[cfg(any(feature = "CONFIG_COMPAT", feature = "CONFIG_ALPHA"))]
unsafe fn put_old_itimerval32(o: *mut old_itimerval32, i: *const itimerspec64) -> c_int {
    let mut v32: old_itimerval32 = core::mem::zeroed();
    v32.it_interval.tv_sec = (*i).it_interval.tv_sec;
    v32.it_interval.tv_usec = (*i).it_interval.tv_nsec / NSEC_PER_USEC;
    v32.it_value.tv_sec = (*i).it_value.tv_sec;
    v32.it_value.tv_usec = (*i).it_value.tv_nsec / NSEC_PER_USEC;
    if copy_to_user(o as *mut c_void, &v32 as *const _ as *const c_void, core::mem::size_of::<old_itimerval32>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn posixtimer_rearm_itimer(tsk: *mut task_struct) {
    let tmr = &mut (*(*tsk).signal).real_timer;
    if !hrtimer_is_queued(tmr) && (*(*tsk).signal).it_real_incr != 0 {
        hrtimer_forward_now(tmr, (*(*tsk).signal).it_real_incr);
        hrtimer_restart(tmr);
    }
}

pub unsafe fn it_real_fn(timer: *mut hrtimer) -> hrtimer_restart {
    let sig = container_of!(timer, signal_struct, real_timer);
    let leader_pid = (*sig).pids[PIDTYPE_TGID as usize];
    trace_itimer_expire(ITIMER_REAL, leader_pid, 0);
    kill_pid_info(SIGALRM, SEND_SIG_PRIV, leader_pid);
    HRTIMER_NORESTART
}

unsafe fn set_cpu_itimer(tsk: *mut task_struct, clock_id: c_uint, value: *const itimerspec64, ovalue: *mut itimerspec64) {
    let mut nval = timespec64_to_ns(&(*value).it_value);
    let ninterval = timespec64_to_ns(&(*value).it_interval);
    let it = &mut (*(*(*tsk).signal).it.as_mut_ptr().add(clock_id as usize));
    spin_lock_irq((*(*tsk).sighand).siglock);
    let oval = (*it).expires;
    let ointerval = (*it).incr;
    if oval != 0 || nval != 0 {
        if nval > 0 { nval += TICK_NSEC; }
        set_process_cpu_timer(tsk, clock_id, &mut nval, &mut (oval as u64));
    }
    (*it).expires = nval; (*it).incr = ninterval;
    trace_itimer_state(if clock_id == CPUCLOCK_VIRT { ITIMER_VIRTUAL } else { ITIMER_PROF }, value, nval);
    spin_unlock_irq((*(*tsk).sighand).siglock);
    if !ovalue.is_null() { (*ovalue).it_value = ns_to_timespec64(oval); (*ovalue).it_interval = ns_to_timespec64(ointerval); }
}

fn timeval_valid(t: *const timeval) -> bool { unsafe { (*t).tv_sec >= 0 && ((*t).tv_usec as c_ulong) < USEC_PER_SEC } }

unsafe fn do_setitimer(which: c_int, value: *const itimerspec64, ovalue: *mut itimerspec64) -> c_int {
    let tsk = current;
    match which {
        ITIMER_REAL => loop {
            let timer = &mut (*(*tsk).signal).real_timer;
            spin_lock_irq((*(*tsk).sighand).siglock);
            if !ovalue.is_null() {
                (*ovalue).it_value = itimer_get_remtime(timer);
                (*ovalue).it_interval = ktime_to_timespec64((*(*tsk).signal).it_real_incr);
            }
            if hrtimer_try_to_cancel(timer) < 0 {
                spin_unlock_irq((*(*tsk).sighand).siglock);
                hrtimer_cancel_wait_running(timer);
                continue;
            }
            let expires = timespec64_to_ktime(&(*value).it_value);
            if expires != 0 {
                (*(*tsk).signal).it_real_incr = timespec64_to_ktime(&(*value).it_interval);
                hrtimer_start(timer, expires, HRTIMER_MODE_REL);
            } else { (*(*tsk).signal).it_real_incr = 0; }
            trace_itimer_state(ITIMER_REAL, value, 0);
            spin_unlock_irq((*(*tsk).sighand).siglock);
            break;
        },
        ITIMER_VIRTUAL => set_cpu_itimer(tsk, CPUCLOCK_VIRT, value, ovalue),
        ITIMER_PROF => set_cpu_itimer(tsk, CPUCLOCK_PROF, value, ovalue),
        _ => return -EINVAL,
    }
    0
}

#[cfg(feature = "CONFIG_SECURITY_SELINUX")]
pub unsafe fn clear_itimer() {
    let v: itimerspec64 = core::mem::zeroed();
    for i in 0..3 { do_setitimer(i, &v, core::ptr::null_mut()); }
}

#[cfg(feature = "__ARCH_WANT_SYS_ALARM")]
unsafe fn alarm_setitimer(mut seconds: c_uint) -> c_uint {
    #[cfg(target_pointer_width = "32")]
    if seconds > INT_MAX as c_uint { seconds = INT_MAX as c_uint; }
    let mut it_new: itimerspec64 = core::mem::zeroed();
    let mut it_old: itimerspec64 = core::mem::zeroed();
    it_new.it_value.tv_sec = seconds as _;
    do_setitimer(ITIMER_REAL, &it_new, &mut it_old);
    if (it_old.it_value.tv_sec == 0 && it_old.it_value.tv_nsec != 0) || it_old.it_value.tv_nsec >= NSEC_PER_SEC / 2 { it_old.it_value.tv_sec += 1; }
    it_old.it_value.tv_sec as c_uint
}

unsafe fn get_itimerval(o: *mut itimerspec64, i: *const __kernel_old_itimerval) -> c_int {
    let mut v: __kernel_old_itimerval = core::mem::zeroed();
    if copy_from_user(&mut v as *mut _ as *mut c_void, i as *const c_void, core::mem::size_of::<__kernel_old_itimerval>()) != 0 { return -EFAULT; }
    if !timeval_valid(&v.it_value) || !timeval_valid(&v.it_interval) { return -EINVAL; }
    (*o).it_interval.tv_sec = v.it_interval.tv_sec; (*o).it_interval.tv_nsec = v.it_interval.tv_usec * NSEC_PER_USEC;
    (*o).it_value.tv_sec = v.it_value.tv_sec; (*o).it_value.tv_nsec = v.it_value.tv_usec * NSEC_PER_USEC;
    0
}

pub unsafe fn setitimer(which: c_int, value: *const __kernel_old_itimerval, ovalue: *mut __kernel_old_itimerval) -> c_int {
    let mut set_buffer: itimerspec64 = core::mem::zeroed();
    let mut get_buffer: itimerspec64 = core::mem::zeroed();
    if !value.is_null() { let error = get_itimerval(&mut set_buffer, value); if error != 0 { return error; } }
    let error = do_setitimer(which, &set_buffer, if ovalue.is_null() { core::ptr::null_mut() } else { &mut get_buffer });
    if error != 0 || ovalue.is_null() { return error; }
    if put_itimerval(ovalue, &get_buffer) != 0 { return -EFAULT; }
    0
}

#[cfg(any(feature = "CONFIG_COMPAT", feature = "CONFIG_ALPHA"))]
unsafe fn get_old_itimerval32(o: *mut itimerspec64, i: *const old_itimerval32) -> c_int {
    let mut v: old_itimerval32 = core::mem::zeroed();
    if copy_from_user(&mut v as *mut _ as *mut c_void, i as *const c_void, core::mem::size_of::<old_itimerval32>()) != 0 { return -EFAULT; }
    if !timeval_valid(&v.it_value) || !timeval_valid(&v.it_interval) { return -EINVAL; }
    (*o).it_interval.tv_sec = v.it_interval.tv_sec; (*o).it_interval.tv_nsec = v.it_interval.tv_usec * NSEC_PER_USEC;
    (*o).it_value.tv_sec = v.it_value.tv_sec; (*o).it_value.tv_nsec = v.it_value.tv_usec * NSEC_PER_USEC;
    0
}

#[cfg(any(feature = "CONFIG_COMPAT", feature = "CONFIG_ALPHA"))]
pub unsafe fn compat_getitimer(which: c_int, value: *mut old_itimerval32) -> c_int {
    let mut get_buffer: itimerspec64 = core::mem::zeroed();
    let mut error = do_getitimer(which, &mut get_buffer);
    if error == 0 && put_old_itimerval32(value, &get_buffer) != 0 { error = -EFAULT; }
    error
}

#[cfg(any(feature = "CONFIG_COMPAT", feature = "CONFIG_ALPHA"))]
pub unsafe fn compat_setitimer(which: c_int, value: *const old_itimerval32, ovalue: *mut old_itimerval32) -> c_int {
    let mut set_buffer: itimerspec64 = core::mem::zeroed();
    let mut get_buffer: itimerspec64 = core::mem::zeroed();
    if !value.is_null() { let error = get_old_itimerval32(&mut set_buffer, value); if error != 0 { return error; } }
    let error = do_setitimer(which, &set_buffer, if ovalue.is_null() { core::ptr::null_mut() } else { &mut get_buffer });
    if error != 0 || ovalue.is_null() { return error; }
    if put_old_itimerval32(ovalue, &get_buffer) != 0 { return -EFAULT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
