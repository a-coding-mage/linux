// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2012-2014 Cisco Systems
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike{addtoit,linux.intel}.com)
 */

// Declarations supplied by the included system and UML headers.
extern "C" {
    fn clock_gettime(clock_id: libc::c_int, tp: *mut libc::timespec) -> libc::c_int;
    fn timer_create(clock_id: libc::c_int, evp: *mut libc::sigevent,
                    timerid: *mut libc::timer_t) -> libc::c_int;
    fn timer_settime(timerid: libc::timer_t, flags: libc::c_int,
                     new_value: *const libc::itimerspec,
                     old_value: *mut libc::itimerspec) -> libc::c_int;
    fn gettid() -> libc::pid_t;
    fn sigemptyset(set: *mut libc::sigset_t) -> libc::c_int;
    fn sigaddset(set: *mut libc::sigset_t, signum: libc::c_int) -> libc::c_int;
    fn signalfd(fd: libc::c_int, mask: *const libc::sigset_t, flags: libc::c_int) -> libc::c_int;
    fn sigprocmask(how: libc::c_int, set: *const libc::sigset_t,
                   oldset: *mut libc::sigset_t) -> libc::c_int;
    fn memset(s: *mut libc::c_void, c: libc::c_int, n: libc::size_t) -> *mut libc::c_void;
    fn panic(fmt: *const libc::c_char, ...);
    fn uml_curr_cpu() -> libc::c_int;
    fn uml_need_resched() -> libc::c_int;
    fn timer_alarm_pending() -> libc::c_int;
    fn os_poll(timeout: libc::c_int, fds: *mut libc::c_int);
}

const UM_NSEC_PER_SEC: libc::c_longlong = 1_000_000_000;

static mut EVENT_HIGH_RES_TIMER: [libc::timer_t; CONFIG_NR_CPUS] = [0 as libc::timer_t; CONFIG_NR_CPUS];

#[inline]
unsafe fn timespec_to_ns(ts: *const libc::timespec) -> libc::c_longlong {
    ((*ts).tv_sec as libc::c_longlong) * UM_NSEC_PER_SEC + (*ts).tv_nsec as libc::c_longlong
}

pub unsafe fn os_persistent_clock_emulation() -> libc::c_longlong {
    let mut realtime_tp: libc::timespec = core::mem::zeroed();
    clock_gettime(libc::CLOCK_REALTIME, &mut realtime_tp);
    timespec_to_ns(&realtime_tp)
}

/** os_timer_create() - create an new posix (interval) timer */
pub unsafe fn os_timer_create() -> libc::c_int {
    let cpu = uml_curr_cpu() as usize;
    let t = &mut EVENT_HIGH_RES_TIMER[cpu] as *mut libc::timer_t;
    let mut sev: libc::sigevent = core::mem::zeroed();
    sev.sigev_notify = libc::SIGEV_THREAD_ID;
    sev.sigev_signo = libc::SIGALRM;
    sev.sigev_value.sival_ptr = t as *mut libc::c_void;
    // sigev_notify_thread_id is the Linux _sigev_un._tid member.
    sev._sigev_un._tid = gettid();

    if timer_create(libc::CLOCK_MONOTONIC, &mut sev, t) == -1 {
        return -1;
    }
    0
}

pub unsafe fn os_timer_set_interval(cpu: libc::c_int, nsecs: libc::c_ulonglong) -> libc::c_int {
    let mut its: libc::itimerspec = core::mem::zeroed();
    its.it_value.tv_sec = (nsecs / UM_NSEC_PER_SEC as libc::c_ulonglong) as libc::time_t;
    its.it_value.tv_nsec = (nsecs % UM_NSEC_PER_SEC as libc::c_ulonglong) as libc::c_long;
    its.it_interval.tv_sec = its.it_value.tv_sec;
    its.it_interval.tv_nsec = its.it_value.tv_nsec;
    if timer_settime(EVENT_HIGH_RES_TIMER[cpu as usize], 0, &its, core::ptr::null_mut()) == -1 {
        return -libc::__errno_location().read();
    }
    0
}

pub unsafe fn os_timer_one_shot(cpu: libc::c_int, nsecs: libc::c_ulonglong) -> libc::c_int {
    let mut its: libc::itimerspec = core::mem::zeroed();
    its.it_value.tv_sec = (nsecs / UM_NSEC_PER_SEC as libc::c_ulonglong) as libc::time_t;
    its.it_value.tv_nsec = (nsecs % UM_NSEC_PER_SEC as libc::c_ulonglong) as libc::c_long;
    its.it_interval.tv_sec = 0;
    its.it_interval.tv_nsec = 0; // we cheat here
    timer_settime(EVENT_HIGH_RES_TIMER[cpu as usize], 0, &its, core::ptr::null_mut());
    0
}

/** os_timer_disable() - disable the posix (interval) timer */
pub unsafe fn os_timer_disable(cpu: libc::c_int) {
    let mut its: libc::itimerspec = core::mem::zeroed();
    memset(&mut its as *mut _ as *mut libc::c_void, 0, core::mem::size_of::<libc::itimerspec>());
    timer_settime(EVENT_HIGH_RES_TIMER[cpu as usize], 0, &its, core::ptr::null_mut());
}

pub unsafe fn os_nsecs() -> libc::c_longlong {
    let mut ts: libc::timespec = core::mem::zeroed();
    clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    timespec_to_ns(&ts)
}

static mut WAKE_SIGNALS: libc::c_int = 0;

pub unsafe fn os_idle_prepare() {
    let mut set: libc::sigset_t = core::mem::zeroed();
    sigemptyset(&mut set);
    sigaddset(&mut set, libc::SIGALRM);
    sigaddset(&mut set, IPI_SIGNAL);
    /*
     * We need to use signalfd rather than sigsuspend in idle sleep
     * because the IPI signal is a real-time signal that carries data,
     * and unlike handling SIGALRM, we cannot simply flag it in
     * signals_pending.
     */
    WAKE_SIGNALS = signalfd(-1, &set, libc::SFD_CLOEXEC);
    if WAKE_SIGNALS < 0 {
        panic(b"Failed to create signal FD, errno = %d\0".as_ptr() as *const libc::c_char,
              *__errno_location());
    }
}

/** os_idle_sleep() - sleep until interrupted */
pub unsafe fn os_idle_sleep() {
    let mut set: libc::sigset_t = core::mem::zeroed();
    sigemptyset(&mut set);
    sigaddset(&mut set, libc::SIGALRM);
    sigprocmask(libc::SIG_BLOCK, &set, core::ptr::null_mut());
    if uml_need_resched() == 0 && timer_alarm_pending() == 0 {
        os_poll(1, &mut WAKE_SIGNALS);
    }
    sigprocmask(libc::SIG_UNBLOCK, &set, core::ptr::null_mut());
}

// Build-time symbols supplied by the surrounding UML translation.
extern "C" {
    static CONFIG_NR_CPUS: usize;
    static IPI_SIGNAL: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
