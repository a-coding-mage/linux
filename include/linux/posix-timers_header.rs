/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub const unsafe fn make_process_cpuclock(pid: core::ffi::c_uint, clock: clockid_t) -> clockid_t {
    ((!pid) << 3) | clock
}

#[inline]
pub const unsafe fn make_thread_cpuclock(tid: core::ffi::c_uint, clock: clockid_t) -> clockid_t {
    make_process_cpuclock(tid, clock | CPUCLOCK_PERTHREAD_MASK)
}

#[inline]
pub const unsafe fn fd_to_clockid(fd: core::ffi::c_int) -> clockid_t {
    make_process_cpuclock(fd as core::ffi::c_uint, CLOCKFD)
}

#[inline]
pub const unsafe fn clockid_to_fd(clk: clockid_t) -> core::ffi::c_int {
    (!(clk >> 3)) as core::ffi::c_int
}

#[inline]
pub const fn clockid_is_aux_clock(id: clockid_t) -> bool {
    // CONFIG_POSIX_AUX_CLOCKS is a build-time configuration condition.
    cfg!(feature = "CONFIG_POSIX_AUX_CLOCKS") && id >= CLOCK_AUX && id <= CLOCK_AUX_LAST
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[repr(C)]
pub struct cpu_timer {
    pub node: timerqueue_node,
    pub head: *mut timerqueue_head,
    pub pid: *mut pid,
    pub elist: list_head,
    pub firing: bool,
    pub nanosleep: bool,
    pub handling: *mut task_struct,
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn cpu_timer_enqueue(head: *mut timerqueue_head, ctmr: *mut cpu_timer) -> bool {
    (*ctmr).head = head;
    timerqueue_add(head, &mut (*ctmr).node)
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn cpu_timer_queued(ctmr: *const cpu_timer) -> bool {
    !(*ctmr).head.is_null()
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn cpu_timer_dequeue(ctmr: *mut cpu_timer) -> bool {
    if cpu_timer_queued(ctmr) {
        timerqueue_del((*ctmr).head, &mut (*ctmr).node);
        (*ctmr).head = core::ptr::null_mut();
        true
    } else {
        false
    }
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn cpu_timer_getexpires(ctmr: *const cpu_timer) -> u64 { (*ctmr).node.expires }

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn cpu_timer_setexpires(ctmr: *mut cpu_timer, exp: u64) { (*ctmr).node.expires = exp; }

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posix_cputimers_init(pct: *mut posix_cputimers) {
    core::ptr::write_bytes(pct, 0, 1);
    (*pct).bases[0].nextevt = u64::MAX;
    (*pct).bases[1].nextevt = u64::MAX;
    (*pct).bases[2].nextevt = u64::MAX;
}

extern "C" {
    pub fn posix_cputimers_group_init(pct: *mut posix_cputimers, cpu_limit: u64);
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posix_cputimers_rt_watchdog(pct: *mut posix_cputimers, runtime: u64) {
    (*pct).bases[CPUCLOCK_SCHED as usize].nextevt = runtime;
}

extern "C" {
    pub fn posixtimer_rearm_itimer(p: *mut task_struct);
    pub fn posixtimer_init_sigqueue(q: *mut sigqueue) -> bool;
    pub fn posixtimer_send_sigqueue(tmr: *mut k_itimer);
    pub fn posixtimer_deliver_signal(info: *mut kernel_siginfo, timer_sigq: *mut sigqueue) -> bool;
    pub fn posixtimer_free_timer(timer: *mut k_itimer);
    pub fn posixtimer_create_prctl(ctrl: core::ffi::c_ulong) -> core::ffi::c_long;
}

// INIT_CPU_TIMERBASE, INIT_CPU_TIMERBASES, and INIT_CPU_TIMERS are C static
// initializer macros. Their designated-initializer intent is preserved here:
// each CPU timer base starts with nextevt = u64::MAX.

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
extern "C" {
    pub fn clear_posix_cputimers_work(p: *mut task_struct);
    pub fn posix_cputimers_init_work();
}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[repr(C)]
pub struct cpu_timer {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posix_cputimers_init(_pct: *mut posix_cputimers) {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posix_cputimers_group_init(_pct: *mut posix_cputimers, _cpu_limit: u64) {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_rearm_itimer(_p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_deliver_signal(_info: *mut kernel_siginfo, _timer_sigq: *mut sigqueue) -> bool { false }

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_free_timer(_timer: *mut k_itimer) {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_create_prctl(_ctrl: core::ffi::c_ulong) -> core::ffi::c_long { -EINVAL }

#[cfg(not(feature = "CONFIG_POSIX_CPU_TIMERS_TASK_WORK"))]
#[inline]
pub unsafe fn clear_posix_cputimers_work(_p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_POSIX_CPU_TIMERS_TASK_WORK"))]
#[inline]
pub unsafe fn posix_cputimers_init_work() {}

#[repr(C)]
pub struct k_itimer {
    pub t_hash: hlist_node,
    pub list: hlist_node,
    pub it_id: timer_t,
    pub it_clock: clockid_t,
    pub it_sigev_notify: core::ffi::c_int,
    pub it_pid_type: pid_type,
    pub it_signal: *mut signal_struct,
    pub kclock: *const k_clock,
    pub it_lock: spinlock_t,
    pub it_status: core::ffi::c_int,
    pub it_sig_periodic: bool,
    pub it_overrun: i64,
    pub it_overrun_last: i64,
    pub it_signal_seq: core::ffi::c_uint,
    pub it_sigqueue_seq: core::ffi::c_uint,
    pub it_interval: ktime_t,
    pub ignored_list: hlist_node,
    pub it_pid: *mut pid,
    pub sigq: sigqueue,
    pub rcuref: rcuref_t,
    pub it: k_itimer_it,
    pub rcu: rcu_head,
}

#[repr(C)]
pub union k_itimer_it {
    pub real: k_itimer_real,
    pub cpu: cpu_timer,
    pub alarm: k_itimer_alarm,
}

#[repr(C)] pub struct k_itimer_real { pub timer: hrtimer }
#[repr(C)] pub struct k_itimer_alarm { pub alarmtimer: alarm }

extern "C" {
    pub fn run_posix_cpu_timers();
    pub fn posix_cpu_timers_exit(task: *mut task_struct);
    pub fn posix_cpu_timers_exit_group(task: *mut task_struct);
    pub fn set_process_cpu_timer(task: *mut task_struct, clock_idx: core::ffi::c_uint, newval: *mut u64, oldval: *mut u64);
    pub fn update_rlimit_cpu(task: *mut task_struct, rlim_new: core::ffi::c_ulong) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posixtimer_putref(tmr: *mut k_itimer) {
    if rcuref_put(&mut (*tmr).rcuref) { posixtimer_free_timer(tmr); }
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posixtimer_sigqueue_getref(q: *mut sigqueue) {
    let tmr = container_of!(q, k_itimer, sigq);
    WARN_ON_ONCE!(!rcuref_get(&mut (*tmr).rcuref));
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posixtimer_sigqueue_putref(q: *mut sigqueue) {
    let tmr = container_of!(q, k_itimer, sigq);
    posixtimer_putref(tmr);
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[inline]
pub unsafe fn posixtimer_valid(timer: *const k_itimer) -> bool {
    let val = (*timer).it_signal as usize;
    (val & 0x1) == 0
}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_sigqueue_getref(_q: *mut sigqueue) {}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[inline]
pub unsafe fn posixtimer_sigqueue_putref(_q: *mut sigqueue) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
