// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Red Hat, Inc., Frederic Weisbecker <fweisbec@redhat.com>
 *
 * Selftests for a few posix timers interface.
 *
 * Kernel loop code stolen from Steven Rostedt <srostedt@redhat.com>
 */

#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::sync::atomic::{AtomicI32, Ordering};

use libc::{
    c_long, itimerval, itimerspec, pid_t, pthread_t, sigevent, siginfo_t, sigset_t, size_t, timespec, timer_t,
    CLOCK_MONOTONIC, CLOCK_PROCESS_CPUTIME_ID, CLOCK_REALTIME, CLOCK_THREAD_CPUTIME_ID, EINVAL, ITIMER_PROF,
    ITIMER_REAL, ITIMER_VIRTUAL, NSEC_PER_SEC, SA_SIGINFO, SIGALRM, SIGEV_NONE,
    SIGEV_SIGNAL, SIGEV_THREAD_ID, SIGIGN, SIGPROF, SIG_UNBLOCK, SIGUSR1, SIG_BLOCK,
    SIGVTALRM, SIG_ERR,
};

const DELAY: i64 = 2;

static DONE: AtomicI32 = AtomicI32::new(0);

// C: static volatile int done;

#[inline]
unsafe fn __fatal_error(test: *const c_char, name: *const c_char, what: *const c_char) {
    let mut buf = [0 as c_char; 64];
    let ret_str = {
        let r = strerror_r(*__errno_location(), buf.as_mut_ptr(), buf.len() as size_t);
        if r.is_null() {
            None
        } else {
            Some(CStr::from_ptr(r).to_string_lossy().into_owned())
        }
    };

    if !name.is_null() && CStr::from_ptr(name).to_bytes().len() > 0 {
        if let Some(msg) = ret_str {
            let test = CStr::from_ptr(test).to_string_lossy();
            let name = CStr::from_ptr(name).to_string_lossy();
            let what = CStr::from_ptr(what).to_string_lossy();
            let fmt = CString::new(format!("{} {} {} {}\n", test, name, what, msg)).unwrap();
            ksft_exit_fail_msg(fmt.as_ptr());
        } else {
            let test = CStr::from_ptr(test).to_string_lossy();
            let name = CStr::from_ptr(name).to_string_lossy();
            let what = CStr::from_ptr(what).to_string_lossy();
            let fmt = CString::new(format!("{} {} {}\n", test, name, what)).unwrap();
            ksft_exit_fail_msg(fmt.as_ptr());
        }
    } else if let Some(msg) = ret_str {
        let test = CStr::from_ptr(test).to_string_lossy();
        let what = CStr::from_ptr(what).to_string_lossy();
        let fmt = CString::new(format!("{} {} {}\n", test, what, msg)).unwrap();
        ksft_exit_fail_msg(fmt.as_ptr());
    } else {
        let test = CStr::from_ptr(test).to_string_lossy();
        let what = CStr::from_ptr(what).to_string_lossy();
        let fmt = CString::new(format!("{} {}\n", test, what)).unwrap();
        ksft_exit_fail_msg(fmt.as_ptr());
    }
}

macro_rules! function_name {
    () => {
        "__unnamed"
    };
}

macro_rules! fatal_error {
    ($name:expr, $what:expr) => {{
        let test = CString::new(function_name!()).unwrap();
        let name_c = match $name {
            Some(v) => CString::new(v).unwrap(),
            None => CString::new("").unwrap(),
        };
        let what = CString::new($what).unwrap();
        unsafe {
            __fatal_error(
                test.as_ptr(),
                if $name.is_none() { std::ptr::null() } else { name_c.as_ptr() },
                what.as_ptr(),
            )
        }
    }};
}

// Busy loop in userspace to elapse ITIMER_VIRTUAL
fn user_loop() {
    while DONE.load(Ordering::SeqCst) == 0 {}
}

/*
 * Try to spend as much time as possible in kernelspace
 * to elapse ITIMER_PROF.
 */
fn kernel_loop() {
    let addr = unsafe { sbrk(std::ptr::null_mut()) };
    let mut err = 0;

    while DONE.load(Ordering::SeqCst) == 0 && err == 0 {
        unsafe {
            err = brk((addr as usize).wrapping_add(4096) as *mut c_void);
            err |= brk(addr);
        }
    }
}

/*
 * Sleep until ITIMER_REAL expiration.
 */
fn idle_loop() {
    unsafe { pause() };
}

extern "C" fn sig_handler(_nr: c_int) {
    DONE.store(1, Ordering::SeqCst);
}

#[inline]
fn calcdiff_ns(t1: timespec, t2: timespec) -> i64 {
    let mut diff: i64 = NSEC_PER_SEC as i64 * ((t1.tv_sec as i64) - (t2.tv_sec as i64));
    diff += (t1.tv_nsec as i64) - (t2.tv_nsec as i64);
    diff
}

/*
 * Check the expected timer expiration matches the GTOD elapsed delta since
 * we armed the timer. Keep a 0.5 sec error margin due to various jitter.
 */
fn check_diff(start: timespec, end: timespec) -> c_int {
    let diff = calcdiff_ns(end, start);

    if (diff - DELAY * NSEC_PER_SEC as i64).abs() > (NSEC_PER_SEC as i64 / 2) {
        let msg = CString::new(format!("Diff too high: {} ns..", diff)).unwrap();
        unsafe {
            printf(msg.as_ptr());
        }
        return -1;
    }

    0
}

fn check_itimer(which: c_int, name: &str) {
    let mut start: timespec = unsafe { std::mem::zeroed() };
    let mut end: timespec = unsafe { std::mem::zeroed() };
    let mut val: itimerval = unsafe { std::mem::zeroed() };
    val.it_value.tv_sec = DELAY as libc::time_t;
    let mut clock_id = CLOCK_REALTIME;

    DONE.store(0, Ordering::SeqCst);

    unsafe {
        if which == ITIMER_VIRTUAL {
            signal(SIGVTALRM, Some(sig_handler));
        } else if which == ITIMER_PROF {
            clock_id = CLOCK_THREAD_CPUTIME_ID;
            signal(SIGPROF, Some(sig_handler));
        } else if which == ITIMER_REAL {
            signal(SIGALRM, Some(sig_handler));
        }

        if clock_gettime(clock_id, &mut start) != 0 {
            fatal_error!(Some(name), "clock_gettime()");
        }

        if setitimer(which, &val, std::ptr::null_mut()) < 0 {
            fatal_error!(Some(name), "setitimer()");
        }

        if which == ITIMER_VIRTUAL {
            user_loop();
        } else if which == ITIMER_PROF {
            kernel_loop();
        } else if which == ITIMER_REAL {
            idle_loop();
        }

        if clock_gettime(clock_id, &mut end) != 0 {
            fatal_error!(Some(name), "clock_gettime()");
        }
    }

    let pass = check_diff(start, end) == 0;
    let name = CString::new(format!("{}\n", name)).unwrap();
    unsafe {
        ksft_test_result(pass as c_int, name.as_ptr());
    }
}

fn check_timer_create(which: c_int) {
    let name = unsafe { CStr::from_ptr(clock_name(which)).to_string_lossy().into_owned() };
    let mut start: timespec = unsafe { std::mem::zeroed() };
    let mut end: timespec = unsafe { std::mem::zeroed() };
    let mut val: itimerspec = unsafe { std::mem::zeroed() };
    val.it_value.tv_sec = DELAY as libc::time_t;
    let clock_id = CLOCK_REALTIME;
    let mut id: timer_t = unsafe { std::mem::zeroed() };

    DONE.store(0, Ordering::SeqCst);

    unsafe {
        if timer_create(which, std::ptr::null_mut(), &mut id) < 0 {
            fatal_error!(Some(&name), "timer_create()");
        }

        if signal(SIGALRM, Some(sig_handler)) == SIG_ERR {
            fatal_error!(Some(&name), "signal()");
        }

        if clock_gettime(clock_id, &mut start) != 0 {
            fatal_error!(Some(&name), "clock_gettime()");
        }

        if timer_settime(id, 0, &val, std::ptr::null_mut()) < 0 {
            fatal_error!(Some(&name), "timer_settime()");
        }

        user_loop();

        if clock_gettime(clock_id, &mut end) != 0 {
            fatal_error!(Some(&name), "clock_gettime()");
        }
    }

    let name_msg = CString::new(format!("timer_create() per {}\n", name)).unwrap();
    unsafe {
        ksft_test_result((check_diff(start, end) == 0) as c_int, name_msg.as_ptr());
    }
}

static mut CTD_THREAD: pthread_t = 0 as pthread_t;
static CTD_COUNT: AtomicI32 = AtomicI32::new(0);
static CTD_FAILED: AtomicI32 = AtomicI32::new(0);

extern "C" fn ctd_sighandler(_sig: c_int) {
    unsafe {
        if pthread_self() != CTD_THREAD {
            CTD_FAILED.store(1, Ordering::SeqCst);
        }
    }
    CTD_COUNT.fetch_sub(1, Ordering::SeqCst);
}

extern "C" fn ctd_thread_func(_arg: *mut c_void) -> *mut c_void {
    let mut val: itimerspec = unsafe { std::mem::zeroed() };
    val.it_value.tv_sec = 0;
    val.it_value.tv_nsec = 1000 * 1000;
    val.it_interval.tv_sec = 0;
    val.it_interval.tv_nsec = 1000 * 1000;
    let mut id: timer_t = unsafe { std::mem::zeroed() };

    /* 1/10 seconds to ensure the leader sleeps */
    unsafe { usleep(10000) };

    CTD_COUNT.store(100, Ordering::SeqCst);
    unsafe {
        if timer_create(CLOCK_PROCESS_CPUTIME_ID, std::ptr::null_mut(), &mut id) != 0 {
            fatal_error!(None, "timer_create()");
        }
        if timer_settime(id, 0, &val, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "timer_settime()");
        }
    }

    while CTD_COUNT.load(Ordering::SeqCst) > 0 && CTD_FAILED.load(Ordering::SeqCst) == 0 {}

    unsafe {
        if timer_delete(id) != 0 {
            fatal_error!(None, "timer_delete()");
        }
    }

    std::ptr::null_mut()
}

/*
 * Test that only the running thread receives the timer signal.
 */
fn check_timer_distribution() {
    unsafe {
        if signal(SIGALRM, Some(ctd_sighandler)) == SIG_ERR {
            fatal_error!(None, "signal()");
        }

        if pthread_create(&mut CTD_THREAD, std::ptr::null_mut(), Some(ctd_thread_func), std::ptr::null_mut()) != 0 {
            fatal_error!(None, "pthread_create()");
        }

        if pthread_join(CTD_THREAD, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "pthread_join()");
        }

        if CTD_FAILED.load(Ordering::SeqCst) == 0 {
            let msg = CString::new("check signal distribution\n").unwrap();
            ksft_test_result_pass(msg.as_ptr());
        } else if ksft_min_kernel_version(6, 3) != 0 {
            let msg = CString::new("check signal distribution\n").unwrap();
            ksft_test_result_fail(msg.as_ptr());
        } else {
            let msg = CString::new("check signal distribution (old kernel)\n").unwrap();
            ksft_test_result_skip(msg.as_ptr());
        }
    }
}

#[repr(C)]
struct tmrsig {
    signals: c_int,
    overruns: c_int,
}

extern "C" fn siginfo_handler(_sig: c_int, si: *mut siginfo_t, _uc: *mut c_void) {
    unsafe {
        if !si.is_null() {
            let tsig = (*(si as *mut c_void) as *mut tmrsig).cast::<tmrsig>();
            if !tsig.is_null() {
                (*tsig).signals += 1;
                (*tsig).overruns += (*si).si_overrun;
            }
        }
    }
}

extern "C" fn ignore_thread(arg: *mut c_void) -> *mut c_void {
    let tid = arg as *mut u32;
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_BLOCK)");
        }

        if !tid.is_null() {
            *tid = gettid() as u32;
        }

        sleep(100);

        if sigprocmask(SIG_UNBLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_UNBLOCK)");
        }
    }

    std::ptr::null_mut()
}

fn check_sig_ign(thread: c_int) {
    let mut tsig = tmrsig { signals: 0, overruns: 0 };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut tid: u32 = 0;
    let mut sa: libc::sigaction = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut pthread: pthread_t = 0 as pthread_t;
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        if thread != 0 {
            if pthread_create(&mut pthread, std::ptr::null_mut(), Some(ignore_thread), &mut tid as *mut _ as *mut c_void) != 0 {
                fatal_error!(None, "pthread_create()");
            }
            sleep(1);
        }

        sa.sa_flags = SA_SIGINFO;
        sa.sa_sigaction = Some(siginfo_handler);
        sigemptyset(&mut sa.sa_mask);
        if sigaction(SIGUSR1, &sa, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigaction()");
        }

        // Block the signal
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_BLOCK)");
        }

        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_SIGNAL;
        sev.sigev_signo = SIGUSR1;
        sev.sigev_value.sival_ptr = &mut tsig as *mut _ as *mut c_void;
        if thread != 0 {
            sev.sigev_notify = SIGEV_THREAD_ID;
            // GNU-specific union field; retain intent with direct access on compatible libc layouts.
            sev._sigev_un._tid = tid as c_int;
        }

        if timer_create(CLOCK_MONOTONIC, &mut sev, &mut timerid) != 0 {
            fatal_error!(None, "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        timer_settime(timerid, 0, &its, std::ptr::null_mut());

        sleep(1);

        // Set the signal to be ignored
        if signal(SIGUSR1, SIG_IGN) == SIG_ERR {
            fatal_error!(None, "signal(SIG_IGN)");
        }

        sleep(1);

        if thread != 0 {
            // Stop the thread first. No signal should be delivered to it
            if pthread_cancel(pthread) != 0 {
                fatal_error!(None, "pthread_cancel()");
            }
            if pthread_join(pthread, std::ptr::null_mut()) != 0 {
                fatal_error!(None, "pthread_join()");
            }
        }

        // Restore the handler
        if sigaction(SIGUSR1, &sa, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigaction()");
        }

        sleep(1);

        // Unblock it, which should deliver the signal in the !thread case
        if sigprocmask(SIG_UNBLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_UNBLOCK)");
        }

        if timer_delete(timerid) != 0 {
            fatal_error!(None, "timer_delete()");
        }
    }

    unsafe {
        if thread == 0 {
            let msg = CString::new("check_sig_ign SIGEV_SIGNAL\n").unwrap();
            ksft_test_result((tsig.signals == 1 && tsig.overruns == 29) as c_int, msg.as_ptr());
        } else {
            let msg = CString::new("check_sig_ign SIGEV_THREAD_ID\n").unwrap();
            ksft_test_result((tsig.signals == 0 && tsig.overruns == 0) as c_int, msg.as_ptr());
        }
    }
}

fn check_rearm() {
    let mut tsig = tmrsig { signals: 0, overruns: 0 };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut sa: libc::sigaction = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        sa.sa_flags = SA_SIGINFO;
        sa.sa_sigaction = Some(siginfo_handler);
        sigemptyset(&mut sa.sa_mask);
        if sigaction(SIGUSR1, &sa, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigaction()");
        }

        // Block the signal
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_BLOCK)");
        }

        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_SIGNAL;
        sev.sigev_signo = SIGUSR1;
        sev.sigev_value.sival_ptr = &mut tsig as *mut _ as *mut c_void;
        if timer_create(CLOCK_MONOTONIC, &mut sev, &mut timerid) != 0 {
            fatal_error!(None, "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        if timer_settime(timerid, 0, &its, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "timer_settime()");
        }

        sleep(1);

        // Reprogram the timer to single shot
        its.it_value.tv_sec = 10;
        its.it_value.tv_nsec = 0;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 0;
        if timer_settime(timerid, 0, &its, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "timer_settime()");
        }

        // Unblock it, which should not deliver a signal
        if sigprocmask(SIG_UNBLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_UNBLOCK)");
        }

        if timer_delete(timerid) != 0 {
            fatal_error!(None, "timer_delete()");
        }
    }

    unsafe {
        let msg = CString::new("check_rearm\n").unwrap();
        ksft_test_result((tsig.signals == 0) as c_int, msg.as_ptr());
    }
}

fn check_delete() {
    let mut tsig = tmrsig { signals: 0, overruns: 0 };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut sa: libc::sigaction = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        sa.sa_flags = SA_SIGINFO;
        sa.sa_sigaction = Some(siginfo_handler);
        sigemptyset(&mut sa.sa_mask);
        if sigaction(SIGUSR1, &sa, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigaction()");
        }

        // Block the signal
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_BLOCK)");
        }

        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_SIGNAL;
        sev.sigev_signo = SIGUSR1;
        sev.sigev_value.sival_ptr = &mut tsig as *mut _ as *mut c_void;
        if timer_create(CLOCK_MONOTONIC, &mut sev, &mut timerid) != 0 {
            fatal_error!(None, "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        if timer_settime(timerid, 0, &its, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "timer_settime()");
        }

        sleep(1);

        if timer_delete(timerid) != 0 {
            fatal_error!(None, "timer_delete()");
        }

        // Unblock it, which should not deliver a signal
        if sigprocmask(SIG_UNBLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(None, "sigprocmask(SIG_UNBLOCK)");
        }
    }

    unsafe {
        let msg = CString::new("check_delete\n").unwrap();
        ksft_test_result((tsig.signals == 0) as c_int, msg.as_ptr());
    }
}

fn check_sigev_none(which: c_int) {
    let name = unsafe { CStr::from_ptr(clock_name(which)).to_string_lossy().into_owned() };
    let mut start: timespec = unsafe { std::mem::zeroed() };
    let mut now: timespec = unsafe { std::mem::zeroed() };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };

    unsafe {
        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_NONE;

        if timer_create(which, &mut sev, &mut timerid) != 0 {
            fatal_error!(Some(&name), "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        timer_settime(timerid, 0, &its, std::ptr::null_mut());

        if clock_gettime(which, &mut start) != 0 {
            fatal_error!(Some(&name), "clock_gettime()");
        }

        loop {
            if clock_gettime(which, &mut now) != 0 {
                fatal_error!(Some(&name), "clock_gettime()");
            }
            if calcdiff_ns(now, start) >= NSEC_PER_SEC as i64 {
                break;
            }
        }

        if timer_gettime(timerid, &mut its) != 0 {
            fatal_error!(Some(&name), "timer_gettime()");
        }

        if timer_delete(timerid) != 0 {
            fatal_error!(Some(&name), "timer_delete()");
        }
    }

    let name = CString::new(format!("check_sigev_none {}\n", name)).unwrap();
    let pass = unsafe { (its.it_value.tv_sec != 0 || its.it_value.tv_nsec != 0) as c_int };
    unsafe { ksft_test_result(pass, name.as_ptr()) };
}

fn check_gettime(which: c_int) {
    let name = unsafe { CStr::from_ptr(clock_name(which)).to_string_lossy().into_owned() };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut prev: itimerspec = unsafe { std::mem::zeroed() };
    let mut start: timespec = unsafe { std::mem::zeroed() };
    let mut now: timespec = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };
    let mut wraps = 0;
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        // Block the signal
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "sigprocmask(SIG_BLOCK)");
        }

        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_SIGNAL;
        sev.sigev_signo = SIGUSR1;

        if timer_create(which, &mut sev, &mut timerid) != 0 {
            fatal_error!(Some(&name), "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        if timer_settime(timerid, 0, &its, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "timer_settime()");
        }

        if timer_gettime(timerid, &mut prev) != 0 {
            fatal_error!(Some(&name), "timer_gettime()");
        }

        if clock_gettime(which, &mut start) != 0 {
            fatal_error!(Some(&name), "clock_gettime()");
        }

        loop {
            if clock_gettime(which, &mut now) != 0 {
                fatal_error!(Some(&name), "clock_gettime()");
            }
            if timer_gettime(timerid, &mut its) != 0 {
                fatal_error!(Some(&name), "timer_gettime()");
            }
            if its.it_value.tv_nsec > prev.it_value.tv_nsec {
                wraps += 1;
            }
            prev = its;

            if calcdiff_ns(now, start) >= NSEC_PER_SEC as i64 {
                break;
            }
        }

        if timer_delete(timerid) != 0 {
            fatal_error!(Some(&name), "timer_delete()");
        }
    }

    let name = CString::new(format!("check_gettime {}\n", name)).unwrap();
    unsafe { ksft_test_result((wraps > 1) as c_int, name.as_ptr()) };
}

fn check_overrun(which: c_int) {
    let name = unsafe { CStr::from_ptr(clock_name(which)).to_string_lossy().into_owned() };
    let mut start: timespec = unsafe { std::mem::zeroed() };
    let mut now: timespec = unsafe { std::mem::zeroed() };
    let mut tsig = tmrsig { signals: 0, overruns: 0 };
    let mut its: itimerspec = unsafe { std::mem::zeroed() };
    let mut sa: libc::sigaction = unsafe { std::mem::zeroed() };
    let mut sev: sigevent = unsafe { std::mem::zeroed() };
    let mut timerid: timer_t = unsafe { std::mem::zeroed() };
    let mut set: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        sa.sa_flags = SA_SIGINFO;
        sa.sa_sigaction = Some(siginfo_handler);
        sigemptyset(&mut sa.sa_mask);
        if sigaction(SIGUSR1, &sa, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "sigaction()");
        }

        // Block the signal
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGUSR1);
        if sigprocmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "sigprocmask(SIG_BLOCK)");
        }

        memset(&mut sev as *mut _ as *mut c_void, 0, std::mem::size_of::<sigevent>());
        sev.sigev_notify = SIGEV_SIGNAL;
        sev.sigev_signo = SIGUSR1;
        sev.sigev_value.sival_ptr = &mut tsig as *mut _ as *mut c_void;
        if timer_create(which, &mut sev, &mut timerid) != 0 {
            fatal_error!(Some(&name), "timer_create()");
        }

        // Start the timer to expire in 100ms and 100ms intervals
        its.it_value.tv_sec = 0;
        its.it_value.tv_nsec = 100_000_000;
        its.it_interval.tv_sec = 0;
        its.it_interval.tv_nsec = 100_000_000;
        if timer_settime(timerid, 0, &its, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "timer_settime()");
        }

        if clock_gettime(which, &mut start) != 0 {
            fatal_error!(Some(&name), "clock_gettime()");
        }

        loop {
            if clock_gettime(which, &mut now) != 0 {
                fatal_error!(Some(&name), "clock_gettime()");
            }
            if calcdiff_ns(now, start) >= NSEC_PER_SEC as i64 {
                break;
            }
        }

        // Unblock it, which should deliver a signal
        if sigprocmask(SIG_UNBLOCK, &set, std::ptr::null_mut()) != 0 {
            fatal_error!(Some(&name), "sigprocmask(SIG_UNBLOCK)");
        }

        if timer_delete(timerid) != 0 {
            fatal_error!(Some(&name), "timer_delete()");
        }
    }

    let name = CString::new(format!("check_overrun {}\n", name)).unwrap();
    unsafe {
        ksft_test_result((tsig.signals == 1 && tsig.overruns == 9) as c_int, name.as_ptr());
    }
}

// #include <sys/syscall.h>

#[inline]
fn do_timer_create(id: *mut c_int) -> c_int {
    unsafe { syscall(libc::SYS_timer_create, libc::CLOCK_MONOTONIC, std::ptr::null::<c_void>(), id) as c_int }
}

#[inline]
fn do_timer_delete(id: c_int) -> c_int {
    unsafe { syscall(libc::SYS_timer_delete, id) as c_int }
}

const PR_TIMER_CREATE_RESTORE_IDS: c_int = 77;
const PR_TIMER_CREATE_RESTORE_IDS_OFF: c_int = 0;
const PR_TIMER_CREATE_RESTORE_IDS_ON: c_int = 1;
const PR_TIMER_CREATE_RESTORE_IDS_GET: c_int = 2;

fn check_timer_create_exact() {
    let mut id = 0 as c_int;

    unsafe {
        if prctl(PR_TIMER_CREATE_RESTORE_IDS, PR_TIMER_CREATE_RESTORE_IDS_ON, 0, 0, 0) != 0 {
            if *__errno_location() == EINVAL {
                let msg = CString::new("check timer create exact, not supported\n").unwrap();
                ksft_test_result_skip(msg.as_ptr());
                return;
            }
            let msg = CString::new(format!("check timer create exact, errno = {}\n", *__errno_location())).unwrap();
            ksft_test_result_skip(msg.as_ptr());
            return;
        }

        if prctl(PR_TIMER_CREATE_RESTORE_IDS, PR_TIMER_CREATE_RESTORE_IDS_GET, 0, 0, 0) != 1 {
            fatal_error!(None, "prctl(GET) failed\n");
        }

        id = 8;
        if do_timer_create(&mut id) < 0 {
            fatal_error!(None, "timer_create()");
        }

        if do_timer_delete(id) != 0 {
            fatal_error!(None, "timer_delete()");
        }

        if prctl(PR_TIMER_CREATE_RESTORE_IDS, PR_TIMER_CREATE_RESTORE_IDS_OFF, 0, 0, 0) != 0 {
            fatal_error!(None, "prctl(OFF)");
        }

        if prctl(PR_TIMER_CREATE_RESTORE_IDS, PR_TIMER_CREATE_RESTORE_IDS_GET, 0, 0, 0) != 0 {
            fatal_error!(None, "prctl(GET) failed\n");
        }

        if id != 8 {
            let msg = CString::new(format!("check timer create exact {} != 8\n", id)).unwrap();
            ksft_test_result_fail(msg.as_ptr());
            return;
        }

        // Validate that it went back to normal mode and allocates ID 9
        if do_timer_create(&mut id) < 0 {
            fatal_error!(None, "timer_create()");
        }

        if do_timer_delete(id) != 0 {
            fatal_error!(None, "timer_delete()");
        }

        if id == 9 {
            let msg = CString::new("check timer create exact\n").unwrap();
            ksft_test_result_pass(msg.as_ptr());
        } else {
            let msg = CString::new("check timer create exact. Disabling failed.\n").unwrap();
            ksft_test_result_fail(msg.as_ptr());
        }
    }
}

fn main() {
    let run_sig_ign_tests = unsafe { ksft_min_kernel_version(6, 13) != 0 };

    unsafe {
        ksft_print_header();
        if run_sig_ign_tests {
            ksft_set_plan(19);
        } else {
            ksft_set_plan(10);
        }

        ksft_print_msg(c"Testing posix timers. False negative may happen on CPU execution \n".as_ptr());
        ksft_print_msg(c"based timers if other threads run on the CPU...\n".as_ptr());
    }

    check_timer_create_exact();

    check_itimer(ITIMER_VIRTUAL, "ITIMER_VIRTUAL");
    check_itimer(ITIMER_PROF, "ITIMER_PROF");
    check_itimer(ITIMER_REAL, "ITIMER_REAL");
    check_timer_create(CLOCK_THREAD_CPUTIME_ID);

    /*
     * It's unfortunately hard to reliably test a timer expiration
     * on parallel multithread cputime. We could arm it to expire
     * on DELAY * nr_threads, with nr_threads busy looping, then wait
     * the normal DELAY since the time is elapsing nr_threads faster.
     * But for that we need to ensure we have real physical free CPUs
     * to ensure true parallelism. So test only one thread until we
     * find a better solution.
     */
    check_timer_create(CLOCK_PROCESS_CPUTIME_ID);
    check_timer_distribution();

    if run_sig_ign_tests {
        check_sig_ign(0);
        check_sig_ign(1);
        check_rearm();
        check_delete();
        check_sigev_none(CLOCK_MONOTONIC);
        check_sigev_none(CLOCK_PROCESS_CPUTIME_ID);
        check_gettime(CLOCK_MONOTONIC);
        check_gettime(CLOCK_PROCESS_CPUTIME_ID);
        check_gettime(CLOCK_THREAD_CPUTIME_ID);
    } else {
        unsafe {
            ksft_print_msg(c"Skipping SIG_IGN tests on kernel < 6.13\n".as_ptr());
        }
    }

    check_overrun(CLOCK_MONOTONIC);
    check_overrun(CLOCK_PROCESS_CPUTIME_ID);
    check_overrun(CLOCK_THREAD_CPUTIME_ID);

    unsafe {
        ksft_finished();
    }
}

#[allow(dead_code)]
unsafe extern "C" {
    fn printf(format: *const c_char, ...);

    fn brk(end_data_segment: *mut c_void) -> c_int;
    fn sbrk(increment: *mut c_void) -> *mut c_void;

    fn syscall(num: c_long, ... ) -> c_long;

    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>) -> *mut c_void;
    fn pause() -> c_int;
    fn __errno_location() -> *mut c_int;
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const libc::sigaction, oldact: *mut libc::sigaction) -> c_int;

    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
    fn setitimer(which: c_int, new_val: *const itimerval, old_val: *mut itimerval) -> c_int;

    fn timer_create(clockid: c_int, sevp: *mut sigevent, timerid: *mut timer_t) -> c_int;
    fn timer_settime(timerid: timer_t, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
    fn timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> c_int;
    fn timer_delete(timerid: timer_t) -> c_int;

    fn pthread_self() -> pthread_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;

    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(useconds: c_uint) -> c_int;
    fn gettid() -> pid_t;

    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn prctl(option: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;

    fn clock_name(clockid: c_int) -> *const c_char;

    fn ksft_print_header();
    fn ksft_set_plan(tests: c_int);
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result(result: c_int, format: *const c_char, ...);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...);
    fn ksft_min_kernel_version(maj: c_int, min: c_int) -> c_int;
    fn ksft_finished();
}

#[macro_export]
macro_rules! c {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
