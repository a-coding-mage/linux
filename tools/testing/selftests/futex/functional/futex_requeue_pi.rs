// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 *   Copyright (c) International Business Machines  Corp., 2006-2008
 *
 * DESCRIPTION
 *      This test excercises the futex syscall op codes needed for requeuing
 *      priority inheritance aware POSIX condition variables and mutexes.
 *
 * AUTHORS
 *      Sripathi Kodi <sripathik@in.ibm.com>
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2008-Jan-13: Initial version by Sripathi Kodi <sripathik@in.ibm.com>
 *      2009-Nov-6: futex test adaptation by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

// C dependencies: errno.h, limits.h, pthread.h, stdio.h, stdlib.h, signal.h,
// string.h, atomic.h, futextest.h, kselftest_harness.h.
// C defined _GNU_SOURCE before including those headers.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

const MAX_WAKE_ITERS: c_int = 1000;
const THREAD_MAX: usize = 10;
const SIGNAL_PERIOD_US: c_uint = 100;

const INT_MAX: c_int = c_int::MAX;
const ETIMEDOUT: c_int = 110;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const PTHREAD_EXPLICIT_SCHED: c_int = 2;
const SCHED_FIFO: c_int = 1;
const CLOCK_MONOTONIC: c_int = 1;

#[repr(C)]
pub struct atomic_t {
    pub val: c_int,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
}

// Opaque C library / harness types supplied by external headers.
#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_attr_t {
    _private: [u8; 0],
}

type time_t = c_long;
type pthread_t = usize;
type futex_t = u32;

const ATOMIC_INITIALIZER: atomic_t = atomic_t { val: 0 };
const FUTEX_INITIALIZER: futex_t = 0;

static mut waiters_blocked: atomic_t = ATOMIC_INITIALIZER;
static mut waiters_woken: atomic_t = ATOMIC_INITIALIZER;

static mut f1: futex_t = FUTEX_INITIALIZER;
static mut f2: futex_t = FUTEX_INITIALIZER;
static mut wake_complete: futex_t = FUTEX_INITIALIZER;

#[repr(C)]
pub struct thread_arg {
    pub _metadata: *mut __test_metadata,
    pub id: c_long,
    pub timeout: *mut timespec,
    pub lock: c_int,
    pub ret: c_int,
}

const THREAD_ARG_INITIALIZER: thread_arg = thread_arg {
    _metadata: ptr::null_mut(),
    id: 0,
    timeout: ptr::null_mut(),
    lock: 0,
    ret: 0,
};

// kselftest fixture declarations from the C source:
// FIXTURE(args) {};
// FIXTURE_SETUP(args) {};
// FIXTURE_TEARDOWN(args) {};

#[repr(C)]
pub struct args_variant {
    pub timeout_ns: c_long,
    pub broadcast: bool,
    pub owner: bool,
    pub locked: bool,
}

// For a given timeout value, the C macro FIXTURE_VARIANT_ADD_TIMEOUT(timeout)
// creates test inputs with all possible combinations of valid arguments:
//   t_timeout
//   t_timeout_broadcast
//   t_timeout_broadcast_locked
//   t_timeout_broadcast_owner
//   t_timeout_locked
//   t_timeout_owner
// The C file instantiates it for 0, 5000, 500000, and 2000000000.
const ARGS_VARIANTS: [args_variant; 24] = [
    args_variant { timeout_ns: 0, broadcast: false, owner: false, locked: false },
    args_variant { timeout_ns: 0, broadcast: true, owner: false, locked: false },
    args_variant { timeout_ns: 0, broadcast: true, owner: false, locked: true },
    args_variant { timeout_ns: 0, broadcast: true, owner: true, locked: false },
    args_variant { timeout_ns: 0, broadcast: false, owner: false, locked: true },
    args_variant { timeout_ns: 0, broadcast: false, owner: true, locked: false },
    args_variant { timeout_ns: 5000, broadcast: false, owner: false, locked: false },
    args_variant { timeout_ns: 5000, broadcast: true, owner: false, locked: false },
    args_variant { timeout_ns: 5000, broadcast: true, owner: false, locked: true },
    args_variant { timeout_ns: 5000, broadcast: true, owner: true, locked: false },
    args_variant { timeout_ns: 5000, broadcast: false, owner: false, locked: true },
    args_variant { timeout_ns: 5000, broadcast: false, owner: true, locked: false },
    args_variant { timeout_ns: 500000, broadcast: false, owner: false, locked: false },
    args_variant { timeout_ns: 500000, broadcast: true, owner: false, locked: false },
    args_variant { timeout_ns: 500000, broadcast: true, owner: false, locked: true },
    args_variant { timeout_ns: 500000, broadcast: true, owner: true, locked: false },
    args_variant { timeout_ns: 500000, broadcast: false, owner: false, locked: true },
    args_variant { timeout_ns: 500000, broadcast: false, owner: true, locked: false },
    args_variant { timeout_ns: 2000000000, broadcast: false, owner: false, locked: false },
    args_variant { timeout_ns: 2000000000, broadcast: true, owner: false, locked: false },
    args_variant { timeout_ns: 2000000000, broadcast: true, owner: false, locked: true },
    args_variant { timeout_ns: 2000000000, broadcast: true, owner: true, locked: false },
    args_variant { timeout_ns: 2000000000, broadcast: false, owner: false, locked: true },
    args_variant { timeout_ns: 2000000000, broadcast: false, owner: true, locked: false },
];

unsafe extern "C" {
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
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn usleep(usec: c_uint) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;

    fn atomic_inc(v: *mut atomic_t);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn futex_wait_requeue_pi(
        uaddr: *mut futex_t,
        val: futex_t,
        uaddr2: *mut futex_t,
        timeout: *mut timespec,
        opflags: c_int,
    ) -> c_int;
    fn futex_lock_pi(
        uaddr: *mut futex_t,
        timeout: *mut timespec,
        detect: c_int,
        opflags: c_int,
    ) -> c_int;
    fn futex_unlock_pi(uaddr: *mut futex_t, opflags: c_int) -> c_int;
    fn futex_cmp_requeue_pi(
        uaddr: *mut futex_t,
        val: futex_t,
        uaddr2: *mut futex_t,
        nr_wake: c_int,
        nr_requeue: c_int,
        opflags: c_int,
    ) -> c_int;
    fn futex_wake(uaddr: *mut futex_t, nr_wake: c_int, opflags: c_int) -> c_int;
    fn futex_wait(
        uaddr: *mut futex_t,
        val: futex_t,
        timeout: *mut timespec,
        opflags: c_int,
    ) -> c_int;
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {};
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

macro_rules! ASSERT_TRUE {
    ($cond:expr) => {
        let _ = $cond;
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

pub unsafe extern "C" fn create_rt_thread(
    _metadata: *mut __test_metadata,
    pth: *mut pthread_t,
    func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
    policy: c_int,
    prio: c_int,
) -> c_int {
    let mut ret: c_int;
    let mut schedp: sched_param = unsafe { mem::zeroed() };
    let mut attr: pthread_attr_t = unsafe { mem::zeroed() };

    unsafe {
        pthread_attr_init(&mut attr);
        ptr::write_bytes(
            &mut schedp as *mut sched_param as *mut u8,
            0,
            mem::size_of::<sched_param>(),
        );
    }

    ret = unsafe { pthread_attr_setinheritsched(&mut attr, PTHREAD_EXPLICIT_SCHED) };
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_attr_setinheritsched failed");

    ret = unsafe { pthread_attr_setschedpolicy(&mut attr, policy) };
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_attr_setschedpolicy failed");

    schedp.sched_priority = prio;
    ret = unsafe { pthread_attr_setschedparam(&mut attr, &schedp) };
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_attr_setschedparam failed");

    ret = unsafe { pthread_create(pth, &attr, func, arg) };
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_create failed");

    0
}

pub unsafe extern "C" fn waiterfn(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut thread_arg;
    let _metadata = unsafe { (*args)._metadata };
    let old_val: futex_t;

    TH_LOG!("Waiter %ld: running", unsafe { (*args).id });
    /* Each thread sleeps for a different amount of time
     * This is to avoid races, because we don't lock the
     * external mutex here
     */
    unsafe {
        usleep((1000 * (*args).id) as c_uint);
    }

    old_val = unsafe { f1 };
    unsafe {
        atomic_inc(&raw mut waiters_blocked);
    }
    TH_LOG!("Calling futex_wait_requeue_pi: %p (%u) -> %p", &raw mut f1, unsafe { f1 }, &raw mut f2);
    unsafe {
        (*args).ret = futex_wait_requeue_pi(
            &raw mut f1,
            old_val,
            &raw mut f2,
            (*args).timeout,
            FUTEX_PRIVATE_FLAG,
        );
    }

    TH_LOG!(
        "waiter %ld woke with %d %s",
        unsafe { (*args).id },
        unsafe { (*args).ret },
        if unsafe { (*args).ret } < 0 {
            unsafe { strerror(errno()) }
        } else {
            b"\0".as_ptr() as *const c_char as *mut c_char
        }
    );
    unsafe {
        atomic_inc(&raw mut waiters_woken);
    }
    if unsafe { (*args).ret } < 0 {
        if unsafe { !(*args).timeout.is_null() && errno() == ETIMEDOUT } {
            unsafe {
                (*args).ret = 0;
            }
        } else {
            ASSERT_EQ!(unsafe { (*args).ret }, 0);
            TH_LOG!("futex_wait_requeue_pi failed: %s", unsafe { strerror(errno()) });
        }
        unsafe {
            futex_lock_pi(&raw mut f2, ptr::null_mut(), 0, FUTEX_PRIVATE_FLAG);
        }
    }
    unsafe {
        futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
    }

    TH_LOG!("Waiter %ld: exiting with %d", unsafe { (*args).id }, unsafe { (*args).ret });
    unsafe {
        pthread_exit(&mut (*args).ret as *mut c_int as *mut c_void);
    }
}

pub unsafe extern "C" fn broadcast_wakerfn(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut thread_arg;
    let _metadata = unsafe { (*args)._metadata };
    let nr_requeue: c_int = INT_MAX;
    let mut task_count: c_int = 0;
    let mut old_val: futex_t;
    let nr_wake: c_int = 1;
    let mut i: c_int = 0;

    TH_LOG!("Waker: waiting for waiters to block");
    while unsafe { waiters_blocked.val } < THREAD_MAX as c_int {
        unsafe {
            usleep(1000);
        }
    }
    unsafe {
        usleep(1000);
    }

    TH_LOG!("Waker: Calling broadcast");
    if unsafe { (*args).lock } != 0 {
        TH_LOG!("Calling FUTEX_LOCK_PI on mutex=%x @ %p", unsafe { f2 }, &raw mut f2);
        unsafe {
            futex_lock_pi(&raw mut f2, ptr::null_mut(), 0, FUTEX_PRIVATE_FLAG);
        }
    }
    loop {
        old_val = unsafe { f1 };
        unsafe {
            (*args).ret = futex_cmp_requeue_pi(
                &raw mut f1,
                old_val,
                &raw mut f2,
                nr_wake,
                nr_requeue,
                FUTEX_PRIVATE_FLAG,
            );
        }
        if unsafe { (*args).ret } < 0 {
            ASSERT_GE!(unsafe { (*args).ret }, 0);
            TH_LOG!("FUTEX_CMP_REQUEUE_PI failed: %s", unsafe { strerror(errno()) });
            break;
        } else {
            i += 1;
            if i < MAX_WAKE_ITERS {
                task_count += unsafe { (*args).ret };
                if task_count < THREAD_MAX as c_int - unsafe { waiters_woken.val } {
                    continue;
                }
                break;
            } else {
                ASSERT_TRUE!(0);
                TH_LOG!(
                    "max broadcast iterations (%d) reached with %d/%d tasks woken or requeued",
                    MAX_WAKE_ITERS,
                    task_count,
                    THREAD_MAX
                );
                break;
            }
        }
    }

    unsafe {
        futex_wake(&raw mut wake_complete, 1, FUTEX_PRIVATE_FLAG);
    }

    if unsafe { (*args).lock } != 0 {
        unsafe {
            futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
        }
    }

    if unsafe { (*args).ret } > 0 {
        unsafe {
            (*args).ret = task_count;
        }
    }

    TH_LOG!("Waker: exiting with %d", unsafe { (*args).ret });
    unsafe {
        pthread_exit(&mut (*args).ret as *mut c_int as *mut c_void);
    }
}

pub unsafe extern "C" fn signal_wakerfn(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut thread_arg;
    let _metadata = unsafe { (*args)._metadata };
    let mut old_val: c_uint;
    let nr_requeue: c_int = 0;
    let mut task_count: c_int = 0;
    let nr_wake: c_int = 1;
    let mut i: c_int = 0;

    TH_LOG!("Waker: waiting for waiters to block");
    while unsafe { waiters_blocked.val } < THREAD_MAX as c_int {
        unsafe {
            usleep(1000);
        }
    }
    unsafe {
        usleep(1000);
    }

    while task_count < THREAD_MAX as c_int && unsafe { waiters_woken.val } < THREAD_MAX as c_int {
        TH_LOG!("task_count: %d, waiters_woken: %d", task_count, unsafe {
            waiters_woken.val
        });
        if unsafe { (*args).lock } != 0 {
            TH_LOG!("Calling FUTEX_LOCK_PI on mutex=%x @ %p", unsafe { f2 }, &raw mut f2);
            unsafe {
                futex_lock_pi(&raw mut f2, ptr::null_mut(), 0, FUTEX_PRIVATE_FLAG);
            }
        }
        TH_LOG!("Waker: Calling signal");
        /* cond_signal */
        old_val = unsafe { f1 };
        unsafe {
            (*args).ret = futex_cmp_requeue_pi(
                &raw mut f1,
                old_val,
                &raw mut f2,
                nr_wake,
                nr_requeue,
                FUTEX_PRIVATE_FLAG,
            );
        }
        if unsafe { (*args).ret } < 0 {
            unsafe {
                (*args).ret = -errno();
            }
        }
        TH_LOG!("futex: %x", unsafe { f2 });
        if unsafe { (*args).lock } != 0 {
            TH_LOG!("Calling FUTEX_UNLOCK_PI on mutex=%x @ %p", unsafe { f2 }, &raw mut f2);
            unsafe {
                futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
            }
        }
        TH_LOG!("futex: %x", unsafe { f2 });
        if unsafe { (*args).ret } < 0 {
            ASSERT_GE!(unsafe { (*args).ret }, 0);
            TH_LOG!("FUTEX_CMP_REQUEUE_PI failed: %s", unsafe {
                strerror(-(*args).ret)
            });
        }

        task_count += unsafe { (*args).ret };
        unsafe {
            usleep(SIGNAL_PERIOD_US);
        }
        i += 1;
        /* we have to loop at least THREAD_MAX times */
        if i > MAX_WAKE_ITERS + THREAD_MAX as c_int {
            ASSERT_TRUE!(0);
            TH_LOG!(
                "max signaling iterations (%d) reached, giving up on pending waiters.",
                MAX_WAKE_ITERS + THREAD_MAX as c_int
            );
        }
    }

    unsafe {
        futex_wake(&raw mut wake_complete, 1, FUTEX_PRIVATE_FLAG);
    }

    if unsafe { (*args).ret } >= 0 {
        unsafe {
            (*args).ret = task_count;
        }
    }

    TH_LOG!("Waker: exiting with %d", unsafe { (*args).ret });
    TH_LOG!("Waker: waiters_woken: %d", unsafe { waiters_woken.val });
    unsafe {
        pthread_exit(&mut (*args).ret as *mut c_int as *mut c_void);
    }
}

pub unsafe extern "C" fn third_party_blocker(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut thread_arg;
    let _metadata = unsafe { (*args)._metadata };
    let mut ret2: c_int = 0;

    unsafe {
        (*args).ret = futex_lock_pi(&raw mut f2, ptr::null_mut(), 0, FUTEX_PRIVATE_FLAG);
    }
    if unsafe { (*args).ret } == 0 {
        unsafe {
            (*args).ret = futex_wait(
                &raw mut wake_complete,
                wake_complete,
                ptr::null_mut(),
                FUTEX_PRIVATE_FLAG,
            );
            ret2 = futex_unlock_pi(&raw mut f2, FUTEX_PRIVATE_FLAG);
        }
    }

    if unsafe { (*args).ret } != 0 || ret2 != 0 {
        ASSERT_TRUE!(0);
        TH_LOG!("%s() futex error", "third_party_blocker\0".as_ptr());
    }

    unsafe {
        pthread_exit(&mut (*args).ret as *mut c_int as *mut c_void);
    }
}

pub unsafe fn futex_requeue_pi(_metadata: *mut __test_metadata, variant: *const args_variant) {
    let mut blocker_arg: thread_arg = THREAD_ARG_INITIALIZER;
    let mut waker_arg: thread_arg = THREAD_ARG_INITIALIZER;
    let mut waiter: [pthread_t; THREAD_MAX] = [0; THREAD_MAX];
    let mut waker: pthread_t = 0;
    let mut blocker: pthread_t = 0;
    let mut wakerfn: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void> = Some(signal_wakerfn);
    let third_party_owner: bool = unsafe { (*variant).owner };
    let timeout_ns: c_long = unsafe { (*variant).timeout_ns };
    let broadcast: bool = unsafe { (*variant).broadcast };
    let mut args: [thread_arg; THREAD_MAX] = [THREAD_ARG_INITIALIZER; THREAD_MAX];
    let mut ts: timespec = unsafe { mem::zeroed() };
    let mut tsp: *mut timespec = ptr::null_mut();
    let lock: bool = unsafe { (*variant).locked };
    let mut waiter_ret: *mut c_int;
    let mut i: usize;
    let mut ret: c_int = 0;

    TH_LOG!(
        "Arguments: broadcast=%d locked=%d owner=%d timeout=%ldns",
        broadcast as c_int,
        lock as c_int,
        third_party_owner as c_int,
        timeout_ns
    );

    if timeout_ns != 0 {
        let secs: time_t;

        TH_LOG!("timeout_ns = %ld", timeout_ns);
        ret = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut ts) };
        secs = (ts.tv_nsec + timeout_ns) / 1000000000;
        ts.tv_nsec = ((ts.tv_nsec as i64 + timeout_ns as i64) % 1000000000) as c_long;
        ts.tv_sec += secs;
        TH_LOG!("ts.tv_sec  = %ld", ts.tv_sec);
        TH_LOG!("ts.tv_nsec = %ld", ts.tv_nsec);
        tsp = &mut ts;
    }

    if broadcast {
        wakerfn = Some(broadcast_wakerfn);
    }

    if third_party_owner {
        blocker_arg._metadata = _metadata;
        unsafe {
            create_rt_thread(
                _metadata,
                &mut blocker,
                Some(third_party_blocker),
                &mut blocker_arg as *mut thread_arg as *mut c_void,
                SCHED_FIFO,
                1,
            );
        }
    }

    unsafe {
        atomic_set(&raw mut waiters_woken, 0);
    }
    i = 0;
    while i < THREAD_MAX {
        args[i]._metadata = _metadata;
        args[i].id = i as c_long;
        args[i].timeout = tsp;
        TH_LOG!("Starting thread %d", i);
        unsafe {
            create_rt_thread(
                _metadata,
                &mut waiter[i],
                Some(waiterfn),
                &mut args[i] as *mut thread_arg as *mut c_void,
                SCHED_FIFO,
                1,
            );
        }
        i += 1;
    }
    waker_arg._metadata = _metadata;
    waker_arg.lock = lock as c_int;
    unsafe {
        create_rt_thread(
            _metadata,
            &mut waker,
            wakerfn,
            &mut waker_arg as *mut thread_arg as *mut c_void,
            SCHED_FIFO,
            1,
        );
    }

    /* Wait for threads to finish */
    /* Store the first error or failure encountered in waiter_ret */
    waiter_ret = &mut args[0].ret;
    i = 0;
    while i < THREAD_MAX {
        unsafe {
            pthread_join(
                waiter[i],
                if *waiter_ret != 0 {
                    ptr::null_mut()
                } else {
                    &mut waiter_ret as *mut *mut c_int as *mut *mut c_void
                },
            );
        }
        i += 1;
    }

    if third_party_owner {
        unsafe {
            pthread_join(blocker, ptr::null_mut());
        }
    }
    unsafe {
        pthread_join(waker, ptr::null_mut());
    }

    if ret == 0 {
        if unsafe { *waiter_ret } != 0 {
            ret = unsafe { *waiter_ret };
        } else if waker_arg.ret < 0 {
            ret = waker_arg.ret;
        } else if blocker_arg.ret != 0 {
            ret = blocker_arg.ret;
        }
    }

    EXPECT_EQ!(ret, 0);
    TH_LOG!("Test failed with error code: %d", ret);
}

// TEST_F(args, futex_requeue_pi) and TEST_HARNESS_MAIN are kselftest harness
// entry points in C. Their Rust equivalents are expected to be supplied by the
// surrounding translated harness.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
