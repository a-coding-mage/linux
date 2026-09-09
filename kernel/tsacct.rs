// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * tsacct.c - System accounting over taskstats interface
 *
 * Copyright (C) Jay Lan,\t<jlan@sgi.com>
 */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */

/*
 * fill in basic accounting fields
 */
pub unsafe fn bacct_add_tsk(
    user_ns: *mut user_namespace,
    pid_ns: *mut pid_namespace,
    stats: *mut taskstats,
    tsk: *mut task_struct,
) {
    let tcred: *const cred;
    let mut utime: u64 = 0;
    let mut stime: u64 = 0;
    let mut utimescaled: u64 = 0;
    let mut stimescaled: u64 = 0;
    let mut now_ns: u64;
    let mut delta: u64;
    let btime: time64_t;

    // BUILD_BUG_ON(TS_COMM_LEN < TASK_COMM_LEN);

    /* calculate task elapsed time in nsec */
    now_ns = ktime_get_ns();
    /* store whole group time first */
    delta = now_ns.wrapping_sub((*(*tsk).group_leader).start_time);
    /* Convert to micro seconds */
    delta /= NSEC_PER_USEC;
    (*stats).ac_tgetime = delta;
    delta = now_ns.wrapping_sub((*tsk).start_time);
    delta /= NSEC_PER_USEC;
    (*stats).ac_etime = delta;
    /* Convert to seconds for btime (note y2106 limit) */
    btime = ktime_get_real_seconds() - delta / USEC_PER_SEC;
    (*stats).ac_btime = clamp_t::<time64_t>(btime, 0, U32_MAX);
    (*stats).ac_btime64 = btime;

    if ((*tsk).flags & PF_EXITING) != 0 {
        (*stats).ac_exitcode = (*tsk).exit_code;
    }
    if thread_group_leader(tsk) && ((*tsk).flags & PF_FORKNOEXEC) != 0 {
        (*stats).ac_flag |= AFORK;
    }
    if ((*tsk).flags & PF_SUPERPRIV) != 0 {
        (*stats).ac_flag |= ASU;
    }
    if ((*tsk).flags & PF_DUMPCORE) != 0 {
        (*stats).ac_flag |= ACORE;
    }
    if ((*tsk).flags & PF_SIGNALED) != 0 {
        (*stats).ac_flag |= AXSIG;
    }
    (*stats).ac_nice = task_nice(tsk);
    (*stats).ac_sched = (*tsk).policy;
    (*stats).ac_pid = task_pid_nr_ns(tsk, pid_ns);
    (*stats).ac_tgid = task_tgid_nr_ns(tsk, pid_ns);
    (*stats).ac_ppid = task_ppid_nr_ns(tsk, pid_ns);
    rcu_read_lock();
    tcred = __task_cred(tsk);
    (*stats).ac_uid = from_kuid_munged(user_ns, (*tcred).uid);
    (*stats).ac_gid = from_kgid_munged(user_ns, (*tcred).gid);
    rcu_read_unlock();

    task_cputime(tsk, &mut utime, &mut stime);
    (*stats).ac_utime = utime / NSEC_PER_USEC;
    (*stats).ac_stime = stime / NSEC_PER_USEC;

    task_cputime_scaled(tsk, &mut utimescaled, &mut stimescaled);
    (*stats).ac_utimescaled = utimescaled / NSEC_PER_USEC;
    (*stats).ac_stimescaled = stimescaled / NSEC_PER_USEC;

    (*stats).ac_minflt = (*tsk).min_flt;
    (*stats).ac_majflt = (*tsk).maj_flt;

    strscpy_pad(&mut (*stats).ac_comm, &(*tsk).comm);
}

#[cfg(CONFIG_TASK_XACCT)]
const KB: u64 = 1024;
#[cfg(CONFIG_TASK_XACCT)]
const MB: u64 = 1024 * KB;
#[cfg(CONFIG_TASK_XACCT)]
const KB_MASK: u64 = !(KB - 1);

/*
 * fill in extended accounting fields
 */
#[cfg(CONFIG_TASK_XACCT)]
pub unsafe fn xacct_add_tsk(stats: *mut taskstats, p: *mut task_struct) {
    let mm: *mut mm_struct;

    /* convert pages-nsec/1024 to Mbyte-usec, see __acct_update_integrals */
    (*stats).coremem = (*p).acct_rss_mem1 * PAGE_SIZE;
    (*stats).coremem /= 1000 * KB;
    (*stats).virtmem = (*p).acct_vm_mem1 * PAGE_SIZE;
    (*stats).virtmem /= 1000 * KB;
    mm = get_task_mm(p);
    if !mm.is_null() {
        /* adjust to KB unit */
        (*stats).hiwater_rss = get_mm_hiwater_rss(mm) * PAGE_SIZE / KB;
        (*stats).hiwater_vm = get_mm_hiwater_vm(mm) * PAGE_SIZE / KB;
        mmput(mm);
    }
    (*stats).read_char = (*p).ioac.rchar & KB_MASK;
    (*stats).write_char = (*p).ioac.wchar & KB_MASK;
    (*stats).read_syscalls = (*p).ioac.syscr & KB_MASK;
    (*stats).write_syscalls = (*p).ioac.syscw & KB_MASK;
    #[cfg(CONFIG_TASK_IO_ACCOUNTING)]
    {
        (*stats).read_bytes = (*p).ioac.read_bytes & KB_MASK;
        (*stats).write_bytes = (*p).ioac.write_bytes & KB_MASK;
        (*stats).cancelled_write_bytes = (*p).ioac.cancelled_write_bytes & KB_MASK;
    }
    #[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
    {
        (*stats).read_bytes = 0;
        (*stats).write_bytes = 0;
        (*stats).cancelled_write_bytes = 0;
    }
}

#[cfg(CONFIG_TASK_XACCT)]
unsafe fn __acct_update_integrals(tsk: *mut task_struct, utime: u64, stime: u64) {
    let time: u64;
    let delta: u64;

    if (*tsk).mm.is_null() || ((*tsk).flags & PF_KTHREAD) != 0 {
        return;
    }

    time = stime + utime;
    delta = time.wrapping_sub((*tsk).acct_timexpd);

    if delta < TICK_NSEC {
        return;
    }

    (*tsk).acct_timexpd = time;
    /*
     * Divide by 1024 to avoid overflow, and to avoid division.
     * The final unit reported to userspace is Mbyte-usecs,
     * the rest of the math is done in xacct_add_tsk.
     */
    (*tsk).acct_rss_mem1 += delta * get_mm_rss((*tsk).mm) >> 10;
    (*tsk).acct_vm_mem1 += delta * READ_ONCE((*(*tsk).mm).total_vm) >> 10;
}

/**
 * acct_update_integrals - update mm integral fields in task_struct
 * @tsk: task_struct for accounting
 */
#[cfg(CONFIG_TASK_XACCT)]
pub unsafe fn acct_update_integrals(tsk: *mut task_struct) {
    let mut utime: u64 = 0;
    let mut stime: u64 = 0;
    let mut flags: unsigned_long = 0;

    local_irq_save(&mut flags);
    task_cputime(tsk, &mut utime, &mut stime);
    __acct_update_integrals(tsk, utime, stime);
    local_irq_restore(flags);
}

/**
 * acct_account_cputime - update mm integral after cputime update
 * @tsk: task_struct for accounting
 */
#[cfg(CONFIG_TASK_XACCT)]
pub unsafe fn acct_account_cputime(tsk: *mut task_struct) {
    __acct_update_integrals(tsk, (*tsk).utime, (*tsk).stime);
}

/**
 * acct_clear_integrals - clear the mm integral fields in task_struct
 * @tsk: task_struct whose accounting fields are cleared
 */
#[cfg(CONFIG_TASK_XACCT)]
pub unsafe fn acct_clear_integrals(tsk: *mut task_struct) {
    (*tsk).acct_timexpd = 0;
    (*tsk).acct_rss_mem1 = 0;
    (*tsk).acct_vm_mem1 = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
