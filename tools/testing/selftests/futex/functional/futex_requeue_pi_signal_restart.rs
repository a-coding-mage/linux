// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 *   Copyright (C) International Business Machines  Corp., 2006-2008
 *
 * DESCRIPTION
 *      This test exercises the futex_wait_requeue_pi() signal handling both
 *      before and after the requeue. The first should be restarted by the
 *      kernel. The latter should return EWOULDBLOCK to the waiter.
 *
 * AUTHORS
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2008-May-5: Initial version by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

// C dependencies: errno.h, getopt.h, limits.h, pthread.h, signal.h, stdio.h,
// stdlib.h, string.h, atomic.h, futextest.h, kselftest_harness.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type futex_t = c_uint;
type pthread_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const DELAY_US: c_uint = 100;
const FUTEX_INITIALIZER: futex_t = 0;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const PTHREAD_EXPLICIT_SCHED: c_int = 2;
const SCHED_FIFO: c_int = 1;
const SIGUSR1: c_int = 10;
const EWOULDBLOCK: c_int = 11;

#[repr(C)]
pub struct atomic_t {
    pub val: c_int,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
}

#[repr(C)]
pub struct pthread_attr_t {
    _private: [usize; 7],
}

#[repr(C)]
pub struct sigset_t {
    _private: [usize; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: sighandler_t,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

static mut f1: futex_t = FUTEX_INITIALIZER;
static mut f2: futex_t = FUTEX_INITIALIZER;
static mut requeued: atomic_t = atomic_t { val: 0 };

static mut waiter_ret: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setinheritsched(attr: *mut pthread_attr_t, inheritsched: c_int) -> c_int;
    fn pthread_attr_setschedpolicy(attr: *mut pthread_attr_t, policy: c_int) -> c_int;
    fn pthread_attr_setschedparam(attr: *mut pthread_attr_t, param: *const sched_param) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn usleep(usec: c_uint) -> c_int;

    fn futex_wait_requeue_pi(
        uaddr: *mut futex_t,
        val: c_uint,
        uaddr2: *mut futex_t,
        timeout: *mut c_void,
        opflags: c_int,
    ) -> c_int;
    fn futex_unlock_pi(uaddr: *mut futex_t, opflags: c_int) -> c_int;
    fn futex_lock_pi(uaddr: *mut futex_t, detect: c_int, timeout: c_int, opflags: c_int) -> c_int;
    fn futex_cmp_requeue_pi(
        uaddr: *mut futex_t,
        val: c_uint,
        uaddr2: *mut futex_t,
        nr_wake: c_int,
        nr_requeue: c_int,
        opflags: c_int,
    ) -> c_int;

    fn atomic_set(atom: *mut atomic_t, val: c_int);

    fn TH_LOG(format: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! EXPECT_TRUE {
    ($expr:expr) => {
        assert!($expr != 0)
    };
}

pub unsafe extern "C" fn create_rt_thread(
    _metadata: *mut __test_metadata,
    pth: *mut pthread_t,
    func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
    policy: c_int,
    prio: c_int,
) -> c_int {
    let mut schedp: sched_param = mem::zeroed();
    let mut attr: pthread_attr_t = mem::zeroed();
    let mut ret: c_int;

    pthread_attr_init(&mut attr);
    memset(
        &mut schedp as *mut sched_param as *mut c_void,
        0,
        mem::size_of::<sched_param>(),
    );

    ret = pthread_attr_setinheritsched(&mut attr, PTHREAD_EXPLICIT_SCHED);
    ASSERT_EQ!(ret, 0);
    TH_LOG(cstr!("pthread_attr_setinheritsched failed"));

    ret = pthread_attr_setschedpolicy(&mut attr, policy);
    ASSERT_EQ!(ret, 0);
    TH_LOG(cstr!("pthread_attr_setschedpolicy failed"));

    schedp.sched_priority = prio;
    ret = pthread_attr_setschedparam(&mut attr, &schedp);
    ASSERT_EQ!(ret, 0);
    TH_LOG(cstr!("pthread_attr_setschedparam failed"));

    ret = pthread_create(pth, &attr, func, arg);
    ASSERT_EQ!(ret, 0);
    TH_LOG(cstr!("pthread_create failed"));

    0
}

pub unsafe extern "C" fn handle_signal(signo: c_int) {
    let _ = signo;
    printf(
        cstr!("INFO: signal received %s requeue\n"),
        if requeued.val != 0 {
            cstr!("after")
        } else {
            cstr!("prior to")
        },
    );
}

pub unsafe extern "C" fn waiterfn(arg: *mut c_void) -> *mut c_void {
    let _metadata: *mut __test_metadata = arg as *mut __test_metadata;
    let mut old_val: c_uint;
    let mut res: c_int;

    TH_LOG(cstr!("Waiter running"));
    TH_LOG(cstr!("Calling FUTEX_LOCK_PI on f2=%x @ %p"), f2, &raw mut f2);
    old_val = f1;
    res = futex_wait_requeue_pi(
        &raw mut f1,
        old_val,
        &raw mut f2,
        ptr::null_mut(),
        FUTEX_PRIVATE_FLAG,
    );
    if requeued.val == 0 || errno != EWOULDBLOCK {
        EXPECT_TRUE!(0);
        TH_LOG(
            cstr!("unexpected return from futex_wait_requeue_pi: %d (%s)"),
            res,
            strerror(errno),
        );
        TH_LOG(cstr!("w2:futex: %x"), f2);
        if res == 0 {
            futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
        }
    }

    pthread_exit(ptr::null_mut());
}

pub unsafe extern "C" fn futex_requeue_pi_signal_restart(
    _metadata: *mut __test_metadata,
) {
    let mut old_val: c_uint;
    let mut sa: sigaction = mem::zeroed();
    let mut waiter: pthread_t = mem::zeroed();
    let mut res: c_int;

    sa.sa_handler = Some(handle_signal);
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0;
    ASSERT_EQ!(sigaction(SIGUSR1, &sa, ptr::null_mut()), 0);
    TH_LOG(cstr!("sigaction failed"));

    TH_LOG(cstr!("m1:f2: %x"), f2);
    TH_LOG(cstr!("Creating waiter"));
    create_rt_thread(
        _metadata,
        &mut waiter,
        Some(waiterfn),
        _metadata as *mut c_void,
        SCHED_FIFO,
        1,
    );

    TH_LOG(cstr!("Calling FUTEX_LOCK_PI on f2=%x @ %p"), f2, &raw mut f2);
    TH_LOG(cstr!("m2:f2: %x"), f2);
    futex_lock_pi(&raw mut f2, 0, 0, FUTEX_PRIVATE_FLAG);
    TH_LOG(cstr!("m3:f2: %x"), f2);

    loop {
        /*
         * signal the waiter before requeue, waiter should automatically
         * restart futex_wait_requeue_pi() in the kernel. Wait for the
         * waiter to block on f1 again.
         */
        TH_LOG(cstr!("Issuing SIGUSR1 to waiter"));
        pthread_kill(waiter, SIGUSR1);
        usleep(DELAY_US);

        TH_LOG(cstr!("Requeueing waiter via FUTEX_CMP_REQUEUE_PI"));
        old_val = f1;
        res = futex_cmp_requeue_pi(&raw mut f1, old_val, &raw mut f2, 1, 0, FUTEX_PRIVATE_FLAG);
        /*
         * If res is non-zero, we either requeued the waiter or hit an
         * error, break out and handle it. If it is zero, then the
         * signal may have hit before the waiter was blocked on f1.
         * Try again.
         */
        if res > 0 {
            atomic_set(&raw mut requeued, 1);
            break;
        } else if res < 0 {
            ASSERT_GE!(res, 0);
            TH_LOG(
                cstr!("FUTEX_CMP_REQUEUE_PI failed: %s"),
                strerror(errno),
            );
        }
    }
    TH_LOG(cstr!("m4:f2: %x"), f2);

    /*
     * Signal the waiter after requeue, waiter should return from
     * futex_wait_requeue_pi() with EWOULDBLOCK. Join the thread here so the
     * futex_unlock_pi() can't happen before the signal wakeup is detected
     * in the kernel.
     */
    TH_LOG(cstr!("Issuing SIGUSR1 to waiter"));
    pthread_kill(waiter, SIGUSR1);
    TH_LOG(cstr!("Waiting for waiter to return"));
    pthread_join(waiter, ptr::null_mut());

    TH_LOG(cstr!("Calling FUTEX_UNLOCK_PI on mutex=%x @ %p"), f2, &raw mut f2);
    futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
    TH_LOG(cstr!("m5:f2: %x"), f2);
}

// TEST_HARNESS_MAIN
