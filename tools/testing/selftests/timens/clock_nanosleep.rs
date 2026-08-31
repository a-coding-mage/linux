// SPDX-License-Identifier: GPL-2.0
// _GNU_SOURCE
// C dependencies: sched.h, sys/timerfd.h, sys/syscall.h, time.h, unistd.h,
// stdlib.h, stdio.h, stdint.h, pthread.h, signal.h, string.h, log.h, timens.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr;

#[repr(C)]
struct thread_args {
    now: *mut libc::timespec,
    rem: *mut libc::timespec,
    lock: *mut libc::pthread_mutex_t,
    clockid: c_int,
    abs: c_int,
}

unsafe extern "C" {
    fn clock_nanosleep(
        clockid: libc::clockid_t,
        flags: c_int,
        request: *const libc::timespec,
        remain: *mut libc::timespec,
    ) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;

    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn nscheck();
    fn ksft_set_plan(plan: c_int);
    fn check_supported_timers();
    fn unshare_timens() -> c_int;
    fn _settime(clockid: c_int, offset: libc::time_t) -> c_int;
    fn check_skip(clockid: c_int) -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

extern "C" fn test_sig(sig: c_int) {
    unsafe {
        if sig == libc::SIGUSR2 {
            pthread_exit(ptr::null_mut());
        }
    }
}

extern "C" fn call_nanosleep(_args: *mut c_void) -> *mut c_void {
    unsafe {
        let args = _args as *mut thread_args;

        clock_nanosleep(
            (*args).clockid,
            if (*args).abs != 0 { libc::TIMER_ABSTIME } else { 0 },
            (*args).now,
            (*args).rem,
        );
        libc::pthread_mutex_unlock((*args).lock);
        ptr::null_mut()
    }
}

unsafe fn run_test(clockid: c_int, abs: c_int) -> c_int {
    let mut now: libc::timespec = mem::zeroed();
    let mut rem = MaybeUninit::<libc::timespec>::uninit();
    let mut args = thread_args {
        now: &mut now,
        rem: rem.as_mut_ptr(),
        lock: ptr::null_mut(),
        clockid,
        abs: 0,
    };
    let mut start = MaybeUninit::<libc::timespec>::uninit();
    let mut lock = MaybeUninit::<libc::pthread_mutex_t>::uninit();
    let mut thread = MaybeUninit::<libc::pthread_t>::uninit();
    let mut ret: c_int;

    libc::signal(libc::SIGUSR1, test_sig as libc::sighandler_t);
    libc::signal(libc::SIGUSR2, test_sig as libc::sighandler_t);

    libc::pthread_mutex_init(lock.as_mut_ptr(), ptr::null());
    libc::pthread_mutex_lock(lock.as_mut_ptr());

    if libc::clock_gettime(clockid, start.as_mut_ptr()) == -1 {
        if *libc::__errno_location() == libc::EINVAL && check_skip(clockid) != 0 {
            return 0;
        }
        return pr_perror(b"clock_gettime\0".as_ptr() as *const c_char);
    }

    let start = start.assume_init();

    if abs != 0 {
        now.tv_sec = start.tv_sec;
        now.tv_nsec = start.tv_nsec;
    }

    now.tv_sec += 3600;
    args.abs = abs;
    args.lock = lock.as_mut_ptr();
    ret = libc::pthread_create(
        thread.as_mut_ptr(),
        ptr::null(),
        call_nanosleep,
        &mut args as *mut thread_args as *mut c_void,
    );
    if ret != 0 {
        pr_err(
            b"Unable to create a thread: %s\0".as_ptr() as *const c_char,
            libc::strerror(ret),
        );
        return 1;
    }

    /* Wait when the thread will call clock_nanosleep(). */
    let mut ok: c_int = 0;
    for j in 0..8 {
        /* The maximum timeout is about 5 seconds. */
        libc::usleep((10000 << j) as libc::useconds_t);

        /* Try to interrupt clock_nanosleep(). */
        libc::pthread_kill(thread.assume_init(), libc::SIGUSR1);

        libc::usleep((10000 << j) as libc::useconds_t);
        /* Check whether clock_nanosleep() has been interrupted or not. */
        if libc::pthread_mutex_trylock(lock.as_mut_ptr()) == 0 {
            /**/
            ok = 1;
            break;
        }
    }
    if ok == 0 {
        libc::pthread_kill(thread.assume_init(), libc::SIGUSR2);
    }
    libc::pthread_join(thread.assume_init(), ptr::null_mut());
    libc::pthread_mutex_destroy(lock.as_mut_ptr());

    if ok == 0 {
        ksft_test_result_pass(
            b"clockid: %d abs:%d timeout\n\0".as_ptr() as *const c_char,
            clockid,
            abs,
        );
        return 1;
    }

    let rem = rem.assume_init();
    if rem.tv_sec < 3300 || rem.tv_sec > 3900 {
        pr_fail(
            b"clockid: %d abs: %d remain: %ld\n\0".as_ptr() as *const c_char,
            clockid,
            abs,
            rem.tv_sec,
        );
        return 1;
    }
    ksft_test_result_pass(
        b"clockid: %d abs:%d\n\0".as_ptr() as *const c_char,
        clockid,
        abs,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let nsfd: c_int;

    ksft_print_header();

    nscheck();

    ksft_set_plan(4);

    check_supported_timers();

    if unshare_timens() != 0 {
        return 1;
    }

    if _settime(libc::CLOCK_MONOTONIC, 7 * 24 * 3600) != 0 {
        return 1;
    }
    if _settime(libc::CLOCK_BOOTTIME, 9 * 24 * 3600) != 0 {
        return 1;
    }

    nsfd = libc::open(
        b"/proc/self/ns/time_for_children\0".as_ptr() as *const c_char,
        libc::O_RDONLY,
    );
    if nsfd < 0 {
        return pr_perror(b"Unable to open timens_for_children\0".as_ptr() as *const c_char);
    }

    if libc::setns(nsfd, libc::CLONE_NEWTIME) != 0 {
        return pr_perror(b"Unable to set timens\0".as_ptr() as *const c_char);
    }

    ret = 0;
    ret |= run_test(libc::CLOCK_MONOTONIC, 0);
    ret |= run_test(libc::CLOCK_MONOTONIC, 1);
    ret |= run_test(libc::CLOCK_BOOTTIME_ALARM, 0);
    ret |= run_test(libc::CLOCK_BOOTTIME_ALARM, 1);

    if ret != 0 {
        ksft_exit_fail();
    }
    ksft_exit_pass();
    ret
}
