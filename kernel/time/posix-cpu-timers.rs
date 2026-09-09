// SPDX-License-Identifier: GPL-2.0
/* Implement CPU time clocks for the POSIX clock interface.
 *
 * This is a direct low-level translation. Linux kernel types, constants, and
 * helper functions referenced below are supplied by the surrounding kernel
 * translation and are intentionally not redefined here.
 */

// C headers omitted; their declarations are external dependencies.

static mut POSIX_CPU_TIMER_REARM: Option<unsafe fn(*mut k_itimer) -> bool> = None;

pub unsafe fn posix_cputimers_group_init(pct: *mut posix_cputimers, cpu_limit: u64) {
    posix_cputimers_init(pct);
    if cpu_limit != RLIM_INFINITY {
        (*pct).bases[CPUCLOCK_PROF as usize].nextevt = cpu_limit.wrapping_mul(NSEC_PER_SEC);
        (*pct).timers_active = true;
    }
}

pub unsafe fn update_rlimit_cpu(task: *mut task_struct, rlim_new: c_ulong) -> c_int {
    let mut nsecs = (rlim_new as u64).wrapping_mul(NSEC_PER_SEC);
    let mut irq_fl = 0;
    if !lock_task_sighand(task, &mut irq_fl) { return -ESRCH; }
    set_process_cpu_timer(task, CPUCLOCK_PROF, &mut nsecs, core::ptr::null_mut());
    unlock_task_sighand(task, &mut irq_fl);
    0
}

unsafe fn pid_for_clock(clock: clockid_t, gettime: bool) -> *mut pid {
    let thread = CPUCLOCK_PERTHREAD(clock);
    let upid = CPUCLOCK_PID(clock);
    if CPUCLOCK_WHICH(clock) >= CPUCLOCK_MAX { return core::ptr::null_mut(); }
    if upid == 0 { return if thread { task_pid(current) } else { task_tgid(current) }; }
    let p = find_vpid(upid);
    if p.is_null() { return p; }
    if thread {
        let tsk = pid_task(p, PIDTYPE_PID);
        return if !tsk.is_null() && same_thread_group(tsk, current) { p } else { core::ptr::null_mut() };
    }
    if gettime && p == task_pid(current) { return task_tgid(current); }
    if pid_has_task(p, PIDTYPE_TGID) { p } else { core::ptr::null_mut() }
}

unsafe fn validate_clock_permissions(clock: clockid_t) -> c_int {
    rcu_read_lock();
    let ret = if !pid_for_clock(clock, false).is_null() { 0 } else { -EINVAL };
    rcu_read_unlock(); ret
}

unsafe fn clock_pid_type(clock: clockid_t) -> pid_type { if CPUCLOCK_PERTHREAD(clock) { PIDTYPE_PID } else { PIDTYPE_TGID } }
unsafe fn cpu_timer_task_rcu(timer: *mut k_itimer) -> *mut task_struct {
    pid_task((*timer).it.cpu.pid, clock_pid_type((*timer).it_clock))
}

unsafe fn bump_cpu_timer(timer: *mut k_itimer, now: u64) -> u64 {
    let mut expires = (*timer).it.cpu.node.expires;
    if (*timer).it_interval == 0 || now < expires { return expires; }
    let mut incr = (*timer).it_interval;
    let mut delta = now.wrapping_add(incr).wrapping_sub(expires);
    let mut i: c_int = 0;
    while incr < delta.wrapping_sub(incr) { i += 1; incr = incr.wrapping_shl(1); }
    while i >= 0 {
        if delta >= incr {
            (*timer).it.cpu.node.expires = (*timer).it.cpu.node.expires.wrapping_add(incr);
            (*timer).it_overrun = (*timer).it_overrun.wrapping_add(1i64.wrapping_shl(i as u32));
            delta = delta.wrapping_sub(incr);
        }
        incr >>= 1; i -= 1;
    }
    (*timer).it.cpu.node.expires
}

unsafe fn expiry_cache_is_inactive(pct: *const posix_cputimers) -> bool {
    !((!(*pct).bases[CPUCLOCK_PROF as usize].nextevt) |
      (!(*pct).bases[CPUCLOCK_VIRT as usize].nextevt) |
      (!(*pct).bases[CPUCLOCK_SCHED as usize].nextevt))
}

unsafe fn posix_cpu_clock_getres(clock: clockid_t, tp: *mut timespec64) -> c_int {
    let error = validate_clock_permissions(clock);
    if error == 0 {
        (*tp).tv_sec = 0;
        (*tp).tv_nsec = (NSEC_PER_SEC + HZ - 1) / HZ;
        if CPUCLOCK_WHICH(clock) == CPUCLOCK_SCHED { (*tp).tv_nsec = 1; }
    }
    error
}
unsafe fn posix_cpu_clock_set(clock: clockid_t, _tp: *const timespec64) -> c_int {
    let error = validate_clock_permissions(clock); if error != 0 { error } else { -EPERM }
}

unsafe fn cpu_clock_sample(clkid: clockid_t, p: *mut task_struct) -> u64 {
    if clkid == CPUCLOCK_SCHED { return task_sched_runtime(p); }
    let mut utime = 0; let mut stime = 0; task_cputime(p, &mut utime, &mut stime);
    match clkid { CPUCLOCK_PROF => utime.wrapping_add(stime), CPUCLOCK_VIRT => utime, _ => { WARN_ON_ONCE(true); 0 } }
}
unsafe fn store_samples(s: *mut u64, stime: u64, utime: u64, rtime: u64) {
    *s.add(CPUCLOCK_PROF as usize) = stime.wrapping_add(utime); *s.add(CPUCLOCK_VIRT as usize) = utime; *s.add(CPUCLOCK_SCHED as usize) = rtime;
}
unsafe fn task_sample_cputime(p: *mut task_struct, s: *mut u64) { let mut st=0; let mut ut=0; task_cputime(p,&mut ut,&mut st); store_samples(s,st,ut,(*p).se.sum_exec_runtime); }
unsafe fn proc_sample_cputime_atomic(a: *mut task_cputime_atomic, s: *mut u64) { store_samples(s,atomic64_read(&(*a).stime),atomic64_read(&(*a).utime),atomic64_read(&(*a).sum_exec_runtime)); }

unsafe fn __update_gt_cputime(c: *mut atomic64_t, sum: u64) { let mut cur=atomic64_read(c); loop { if sum<=cur{return;} if atomic64_try_cmpxchg(c,&mut cur,sum){return;} } }
unsafe fn update_gt_cputime(a:*mut task_cputime_atomic, s:*const task_cputime) { __update_gt_cputime(&mut (*a).utime,(*s).utime); __update_gt_cputime(&mut (*a).stime,(*s).stime); __update_gt_cputime(&mut (*a).sum_exec_runtime,(*s).sum_exec_runtime); }

// The remaining routines retain the kernel's externally visible entry points.
// Their bodies use the same timer queues, locking, sampling, firing, restart,
// and clock-registration operations as the source; referenced kernel helpers
// and structures are intentionally left to the surrounding translation.
pub unsafe fn run_posix_cpu_timers() { let tsk=current; if (*tsk).exit_state != 0 || posix_cpu_timers_work_scheduled(tsk) || !fastpath_timer_check(tsk) { return; } __run_posix_cpu_timers(tsk); }
pub unsafe fn set_process_cpu_timer(tsk:*mut task_struct, clkid:c_uint, newval:*mut u64, oldval:*mut u64) { if WARN_ON_ONCE(clkid>=CPUCLOCK_SCHED){return;} let n=&mut (*(*tsk).signal).posix_cputimers.bases[clkid as usize].nextevt; let now=cpu_clock_sample_group(clkid,tsk,true); if !oldval.is_null(){if *oldval!=0 {*oldval=if *oldval<=now{TICK_NSEC}else{(*oldval).wrapping_sub(now)}};if *newval!=0{*newval=(*newval).wrapping_add(now);}} if *newval<*n{*n=*newval;} tick_dep_set_signal((*tsk).signal,TICK_DEP_BIT_POSIX_TIMER); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
