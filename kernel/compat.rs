// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/kernel/compat.c
 *
 *  Kernel compatibililty routines for e.g. 32 bit syscall support
 *  on 64 bit kernels.
 *
 *  Copyright (C) 2002-2003 Stephen Rothwell, IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(__ARCH_WANT_SYS_SIGPROCMASK)]
#[inline]
unsafe fn compat_sig_setmask(blocked: *mut sigset_t, set: compat_sigset_word) {
    core::ptr::copy_nonoverlapping(
        &set as *const compat_sigset_word as *const u8,
        (*blocked).sig.as_mut_ptr() as *mut u8,
        core::mem::size_of::<compat_sigset_word>(),
    );
}

#[cfg(__ARCH_WANT_SYS_SIGPROCMASK)]
unsafe fn compat_sys_sigprocmask(
    how: i32,
    nset: *mut compat_old_sigset_t,
    oset: *mut compat_old_sigset_t,
) -> i32 {
    let mut old_set: old_sigset_t;
    let mut new_set: old_sigset_t;
    let mut new_blocked: sigset_t;

    old_set = (*current).blocked.sig[0];

    if !nset.is_null() {
        if get_user(&mut new_set, nset) != 0 {
            return -EFAULT;
        }
        new_set &= !(sigmask(SIGKILL) | sigmask(SIGSTOP));

        new_blocked = (*current).blocked;

        match how {
            SIG_BLOCK => sigaddsetmask(&mut new_blocked, new_set),
            SIG_UNBLOCK => sigdelsetmask(&mut new_blocked, new_set),
            SIG_SETMASK => compat_sig_setmask(&mut new_blocked, new_set),
            _ => return -EINVAL,
        }

        set_current_blocked(&new_blocked);
    }

    if !oset.is_null() {
        if put_user(old_set, oset) != 0 {
            return -EFAULT;
        }
    }

    0
}

unsafe fn put_compat_rusage(r: *const rusage, ru: *mut compat_rusage) -> i32 {
    let mut r32: compat_rusage = core::mem::zeroed();
    r32.ru_utime.tv_sec = (*r).ru_utime.tv_sec;
    r32.ru_utime.tv_usec = (*r).ru_utime.tv_usec;
    r32.ru_stime.tv_sec = (*r).ru_stime.tv_sec;
    r32.ru_stime.tv_usec = (*r).ru_stime.tv_usec;
    r32.ru_maxrss = (*r).ru_maxrss;
    r32.ru_ixrss = (*r).ru_ixrss;
    r32.ru_idrss = (*r).ru_idrss;
    r32.ru_isrss = (*r).ru_isrss;
    r32.ru_minflt = (*r).ru_minflt;
    r32.ru_majflt = (*r).ru_majflt;
    r32.ru_nswap = (*r).ru_nswap;
    r32.ru_inblock = (*r).ru_inblock;
    r32.ru_oublock = (*r).ru_oublock;
    r32.ru_msgsnd = (*r).ru_msgsnd;
    r32.ru_msgrcv = (*r).ru_msgrcv;
    r32.ru_nsignals = (*r).ru_nsignals;
    r32.ru_nvcsw = (*r).ru_nvcsw;
    r32.ru_nivcsw = (*r).ru_nivcsw;
    if copy_to_user(ru, &r32, core::mem::size_of::<compat_rusage>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn compat_get_user_cpu_mask(
    user_mask_ptr: *mut compat_ulong_t,
    mut len: u32,
    new_mask: *mut cpumask,
) -> i32 {
    if len < cpumask_size() {
        core::ptr::write_bytes(new_mask as *mut u8, 0, cpumask_size() as usize);
    } else if len > cpumask_size() {
        len = cpumask_size();
    }

    let k = cpumask_bits(new_mask);
    compat_get_bitmap(k, user_mask_ptr, len.wrapping_mul(8) as usize)
}

unsafe fn compat_sys_sched_setaffinity(
    pid: compat_pid_t,
    len: u32,
    user_mask_ptr: *mut compat_ulong_t,
) -> i32 {
    let mut new_mask: cpumask_var_t = core::mem::zeroed();
    if !alloc_cpumask_var(&mut new_mask, GFP_KERNEL) {
        return -ENOMEM;
    }

    let retval = compat_get_user_cpu_mask(user_mask_ptr, len, new_mask);
    let retval = if retval != 0 {
        retval
    } else {
        sched_setaffinity(pid, new_mask)
    };
    free_cpumask_var(new_mask);
    retval
}

unsafe fn compat_sys_sched_getaffinity(
    pid: compat_pid_t,
    len: u32,
    user_mask_ptr: *mut compat_ulong_t,
) -> i32 {
    if len.wrapping_mul(BITS_PER_BYTE) < nr_cpu_ids {
        return -EINVAL;
    }
    if len & (core::mem::size_of::<compat_ulong_t>() as u32 - 1) != 0 {
        return -EINVAL;
    }

    let mut mask: cpumask_var_t = core::mem::zeroed();
    if !zalloc_cpumask_var(&mut mask, GFP_KERNEL) {
        return -ENOMEM;
    }

    let mut ret = sched_getaffinity(pid, mask);
    if ret == 0 {
        let retlen = core::cmp::min(len, cpumask_size());
        if compat_put_bitmap(user_mask_ptr, cpumask_bits(mask), retlen as usize * 8) != 0 {
            ret = -EFAULT;
        } else {
            ret = retlen as i32;
        }
    }
    free_cpumask_var(mask);
    ret
}

/*
 * We currently only need the following fields from the sigevent
 * structure: sigev_value, sigev_signo, sig_notify and (sometimes
 * sigev_notify_thread_id).  The others are handled in user mode.
 * We also assume that copying sigev_value.sival_int is sufficient
 * to keep all the bits of sigev_value.sival_ptr intact.
 */
unsafe fn get_compat_sigevent(
    event: *mut sigevent,
    u_event: *const compat_sigevent,
) -> i32 {
    core::ptr::write_bytes(event as *mut u8, 0, core::mem::size_of::<sigevent>());
    if !access_ok(u_event, core::mem::size_of::<compat_sigevent>())
        || __get_user(&mut (*event).sigev_value.sival_int, &(*u_event).sigev_value.sival_int) != 0
        || __get_user(&mut (*event).sigev_signo, &(*u_event).sigev_signo) != 0
        || __get_user(&mut (*event).sigev_notify, &(*u_event).sigev_notify) != 0
        || __get_user(
            &mut (*event).sigev_notify_thread_id,
            &(*u_event).sigev_notify_thread_id,
        ) != 0
    {
        -EFAULT
    } else {
        0
    }
}

unsafe fn compat_get_bitmap(
    mut mask: *mut c_ulong,
    mut umask: *const compat_ulong_t,
    mut bitmap_size: usize,
) -> i64 {
    bitmap_size = ALIGN(bitmap_size, BITS_PER_COMPAT_LONG);
    let mut nr_compat_longs = BITS_TO_COMPAT_LONGS(bitmap_size);

    if !user_read_access_begin(umask, bitmap_size / 8) {
        return -EFAULT as i64;
    }

    while nr_compat_longs > 1 {
        let mut l1: compat_ulong_t = 0;
        let mut l2: compat_ulong_t = 0;
        if unsafe_get_user(&mut l1, umask) != 0 { user_read_access_end(); return -EFAULT as i64; }
        umask = umask.add(1);
        if unsafe_get_user(&mut l2, umask) != 0 { user_read_access_end(); return -EFAULT as i64; }
        umask = umask.add(1);
        *mask = ((l2 as c_ulong) << BITS_PER_COMPAT_LONG) | l1 as c_ulong;
        mask = mask.add(1);
        nr_compat_longs -= 2;
    }
    if nr_compat_longs != 0 {
        if unsafe_get_user(mask, umask) != 0 { user_read_access_end(); return -EFAULT as i64; }
    }
    user_read_access_end();
    0
}

unsafe fn compat_put_bitmap(
    mut umask: *mut compat_ulong_t,
    mut mask: *mut c_ulong,
    mut bitmap_size: usize,
) -> i64 {
    bitmap_size = ALIGN(bitmap_size, BITS_PER_COMPAT_LONG);
    let mut nr_compat_longs = BITS_TO_COMPAT_LONGS(bitmap_size);

    if !user_write_access_begin(umask, bitmap_size / 8) {
        return -EFAULT as i64;
    }
    while nr_compat_longs > 1 {
        let m = *mask;
        mask = mask.add(1);
        if unsafe_put_user(m as compat_ulong_t, umask) != 0 { user_write_access_end(); return -EFAULT as i64; }
        umask = umask.add(1);
        if unsafe_put_user((m >> BITS_PER_COMPAT_LONG) as compat_ulong_t, umask) != 0 { user_write_access_end(); return -EFAULT as i64; }
        umask = umask.add(1);
        nr_compat_longs -= 2;
    }
    if nr_compat_longs != 0 {
        if unsafe_put_user(*mask as compat_ulong_t, umask) != 0 { user_write_access_end(); return -EFAULT as i64; }
    }
    user_write_access_end();
    0
}

unsafe fn get_compat_sigset(set: *mut sigset_t, compat: *const compat_sigset_t) -> i32 {
    // On big-endian targets, compat words are combined into native words.
    #[cfg(__BIG_ENDIAN)]
    {
        let mut v: compat_sigset_t = core::mem::zeroed();
        if copy_from_user(&mut v, compat, core::mem::size_of::<compat_sigset_t>()) != 0 {
            return -EFAULT;
        }
        match _NSIG_WORDS {
            4 => { (*set).sig[3] = v.sig[6] | ((v.sig[7] as c_long) << 32); }
            3 => { (*set).sig[2] = v.sig[4] | ((v.sig[5] as c_long) << 32); }
            2 => { (*set).sig[1] = v.sig[2] | ((v.sig[3] as c_long) << 32); }
            1 => { (*set).sig[0] = v.sig[0] | ((v.sig[1] as c_long) << 32); }
            _ => {}
        }
    }
    #[cfg(not(__BIG_ENDIAN))]
    {
        if copy_from_user(set, compat, core::mem::size_of::<compat_sigset_t>()) != 0 {
            return -EFAULT;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
