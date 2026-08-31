// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2025 Sebastian Andrzej Siewior <bigeasy@linutronix.de>
 */

// C dependencies: errno.h, pthread.h, stdio.h, stdlib.h, string.h, unistd.h,
// linux/prctl.h, sys/prctl.h, and "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const MAX_THREADS: usize = 64;

type pthread_t = libc::pthread_t;
type pthread_barrier_t = libc::pthread_barrier_t;
type pthread_mutex_t = libc::pthread_mutex_t;
type pthread_mutexattr_t = libc::pthread_mutexattr_t;
type timespec = libc::timespec;

static mut barrier_main: pthread_barrier_t = unsafe { core::mem::zeroed() };
static mut global_lock: pthread_mutex_t = unsafe { core::mem::zeroed() };
static mut threads: [pthread_t; MAX_THREADS] = unsafe { core::mem::zeroed() };
static mut counter: c_int = 0;

// If PR_FUTEX_HASH is not provided by linux/prctl.h, the C source defines it.
const PR_FUTEX_HASH: c_int = 78;
const PR_FUTEX_HASH_SET_SLOTS: c_int = 1;
const PR_FUTEX_HASH_GET_SLOTS: c_int = 2;

const SEC_IN_NSEC: libc::time_t = 1000000000;
const MSEC_IN_NSEC: libc::c_long = 1000000;

static test_msg_auto_create: *const c_char =
    b"Automatic hash bucket init on thread creation.\n\0".as_ptr() as *const c_char;
static test_msg_auto_inc: *const c_char =
    b"Automatic increase with more than 16 CPUs\n\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const libc::pthread_barrierattr_t,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    fn pthread_mutexattr_setprotocol(attr: *mut pthread_mutexattr_t, protocol: c_int) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_timedlock(mutex: *mut pthread_mutex_t, abstime: *const timespec) -> c_int;
    fn clock_gettime(clk_id: libc::clockid_t, tp: *mut timespec) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn futex_hash_slots_set(slots: c_uint) -> c_int {
    prctl(
        PR_FUTEX_HASH,
        PR_FUTEX_HASH_SET_SLOTS,
        slots,
        0 as c_int,
    )
}

unsafe fn futex_hash_slots_get() -> c_int {
    prctl(PR_FUTEX_HASH, PR_FUTEX_HASH_GET_SLOTS)
}

unsafe fn futex_hash_slots_set_verify(_metadata: *mut __test_metadata, slots: c_int) {
    let mut ret: c_int;

    ret = futex_hash_slots_set(slots as c_uint);
    ASSERT_EQ!(ret, 0);
    TH_LOG!(
        "Failed to set slots to %d: %s",
        slots,
        strerror(errno())
    );

    ret = futex_hash_slots_get();
    if !ASSERT_EQ!(ret, slots) {
        TH_LOG!(
            "Set %d slots but PR_FUTEX_HASH_GET_SLOTS returns: %d, %s",
            slots,
            ret,
            strerror(errno())
        );
    }
}

unsafe fn futex_hash_slots_set_must_fail(_metadata: *mut __test_metadata, slots: c_int) {
    let ret: c_int;

    ret = futex_hash_slots_set(slots as c_uint);
    EXPECT_LT!(ret, 0);
    TH_LOG!(
        "futex_hash_slots_set(%d) should fail but succeeded",
        slots
    );
}

unsafe extern "C" fn thread_return_fn(_arg: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

unsafe extern "C" fn thread_lock_fn(_arg: *mut c_void) -> *mut c_void {
    pthread_barrier_wait(&mut barrier_main);

    pthread_mutex_lock(&mut global_lock);
    counter += 1;
    usleep(20);
    pthread_mutex_unlock(&mut global_lock);
    ptr::null_mut()
}

unsafe fn create_max_threads(
    _metadata: *mut __test_metadata,
    thread_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
) {
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < MAX_THREADS as c_int {
        ret = pthread_create(
            &mut threads[i as usize],
            ptr::null(),
            thread_fn,
            ptr::null_mut(),
        );
        ASSERT_EQ!(ret, 0);
        TH_LOG!("pthread_create failed: %s", strerror(errno()));
        i += 1;
    }
}

unsafe fn join_max_threads(_metadata: *mut __test_metadata) {
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < MAX_THREADS as c_int {
        ret = pthread_join(threads[i as usize], ptr::null_mut());
        ASSERT_EQ!(ret, 0);
        TH_LOG!(
            "pthread_join failed for thread %d: %s",
            i,
            strerror(errno())
        );
        i += 1;
    }
}

unsafe fn futex_dummy_op(_metadata: *mut __test_metadata) {
    let mut lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;
    let mut timeout: timespec = core::mem::zeroed();
    let mut ret: c_int;

    pthread_mutex_lock(&mut lock);
    clock_gettime(libc::CLOCK_REALTIME, &mut timeout);
    timeout.tv_nsec += 100 * MSEC_IN_NSEC;
    if timeout.tv_nsec >= SEC_IN_NSEC {
        timeout.tv_nsec -= SEC_IN_NSEC;
        timeout.tv_sec += 1;
    }
    ret = pthread_mutex_timedlock(&mut lock, &timeout);
    ASSERT_NE!(ret, 0);
    TH_LOG!("Successfully locked an already locked mutex");

    ASSERT_EQ!(ret, libc::ETIMEDOUT);
    TH_LOG!("pthread_mutex_timedlock() did not timeout: %d", ret);
}

// TEST(priv_hash)
unsafe fn priv_hash(_metadata: *mut __test_metadata) {
    let mut futex_slots1: c_int;
    let mut futex_slotsn: c_int = 0;
    let mut online_cpus: c_int;
    let mut mutex_attr_pi: pthread_mutexattr_t = core::mem::zeroed();
    let mut ret: c_int;
    let mut retry: c_int = 20;

    ret = pthread_mutexattr_init(&mut mutex_attr_pi);
    ret |= pthread_mutexattr_setprotocol(&mut mutex_attr_pi, libc::PTHREAD_PRIO_INHERIT);
    ret |= pthread_mutex_init(&mut global_lock, &mutex_attr_pi);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("Failed to initialize pthread mutex");

    /* First thread, expect to be 0, not yet initialized */
    ret = futex_hash_slots_get();
    if ret < 0 && errno() == libc::EINVAL {
        SKIP!(return, "PR_FUTEX_HASH not supported by kernel");
    }

    ASSERT_EQ!(ret, 0);
    TH_LOG!(
        "futex_hash_slots_get() failed: %d, %s",
        ret,
        strerror(errno())
    );

    ret = pthread_create(
        &mut threads[0],
        ptr::null(),
        thread_return_fn,
        ptr::null_mut(),
    );
    ASSERT_EQ!(ret, 0);
    TH_LOG!(
        "pthread_create() failed: %d, %s",
        ret,
        strerror(errno())
    );

    ret = pthread_join(threads[0], ptr::null_mut());
    ASSERT_EQ!(ret, 0);
    TH_LOG!(
        "pthread_join() failed: %d, %s",
        ret,
        strerror(errno())
    );

    /* First thread, has to initialize private hash */
    futex_slots1 = futex_hash_slots_get();
    EXPECT_GT!(futex_slots1, 0);
    TH_LOG!(
        "Current hash buckets: %d. %s",
        futex_slots1,
        test_msg_auto_create
    );

    online_cpus = sysconf(libc::_SC_NPROCESSORS_ONLN) as c_int;
    ret = pthread_barrier_init(&mut barrier_main, ptr::null(), (MAX_THREADS + 1) as c_uint);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_barrier_init failed: %s", strerror(errno()));

    ret = pthread_mutex_lock(&mut global_lock);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_mutex_lock failed: %s", strerror(errno()));

    counter = 0;
    create_max_threads(_metadata, thread_lock_fn);
    pthread_barrier_wait(&mut barrier_main);

    /*
     * The current default size of hash buckets is 16. The auto increase
     * works only if more than 16 CPUs are available.
     */
    TH_LOG!("Online CPUs: %d", online_cpus);
    if online_cpus > 16 {
        loop {
            futex_slotsn = futex_hash_slots_get();
            if futex_slotsn < 0 || futex_slots1 == futex_slotsn {
                retry -= 1;
                /*
                 * Auto scaling on thread creation can be slightly delayed
                 * because it waits for a RCU grace period twice. The new
                 * private hash is assigned upon the first futex operation
                 * after grace period.
                 * To cover all this for testing purposes the function
                 * below will acquire a lock and acquire it again with a
                 * 100ms timeout which must timeout. This ensures we
                 * sleep for 100ms and issue a futex operation.
                 */
                if retry > 0 {
                    futex_dummy_op(_metadata);
                    continue;
                }
                if !EXPECT_NE!(futex_slots1, futex_slotsn) {
                    TH_LOG!(
                        "Expected increase of hash buckets but got: %d -> %d. %s",
                        futex_slots1,
                        futex_slotsn,
                        test_msg_auto_inc
                    );
                }
            }
            break;
        }
    } else {
        SKIP!(
            return,
            "Automatic increase with more than 16 CPUs (only %d online)",
            online_cpus
        );
    }
    ret = pthread_mutex_unlock(&mut global_lock);

    /* Once the user changes it, it has to be what is set */
    futex_hash_slots_set_verify(_metadata, 2);
    futex_hash_slots_set_verify(_metadata, 4);
    futex_hash_slots_set_verify(_metadata, 8);
    futex_hash_slots_set_verify(_metadata, 32);
    futex_hash_slots_set_verify(_metadata, 16);

    ret = futex_hash_slots_set(15);
    EXPECT_LT!(ret, 0);
    TH_LOG!("Use 15 slots should fail but succeeded");

    futex_hash_slots_set_verify(_metadata, 2);
    join_max_threads(_metadata);

    EXPECT_EQ!(counter, MAX_THREADS as c_int);
    TH_LOG!(
        "Created and waited for %d of %d threads",
        counter,
        MAX_THREADS as c_int
    );

    counter = 0;
    /* Once the user set something, auto resize must be disabled */
    ret = pthread_barrier_init(&mut barrier_main, ptr::null(), MAX_THREADS as c_uint);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_barrier_init failed: %s", strerror(errno()));

    create_max_threads(_metadata, thread_lock_fn);
    join_max_threads(_metadata);

    ret = futex_hash_slots_get();
    EXPECT_EQ!(ret, 2);
    TH_LOG!("No more auto-resize after manual setting, got %d", ret);

    futex_hash_slots_set_must_fail(_metadata, 1 << 29);
    futex_hash_slots_set_verify(_metadata, 4);

    /*
     * Once the global hash has been requested, then this requested can not
     * be undone.
     */
    ret = futex_hash_slots_set(0);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("Global hash request failed: %s", strerror(errno()));

    futex_hash_slots_set_must_fail(_metadata, 4);
    futex_hash_slots_set_must_fail(_metadata, 8);
    futex_hash_slots_set_must_fail(_metadata, 8);
    futex_hash_slots_set_must_fail(_metadata, 0);
    futex_hash_slots_set_must_fail(_metadata, 6);

    ret = pthread_barrier_init(&mut barrier_main, ptr::null(), MAX_THREADS as c_uint);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("pthread_barrier_init failed: %s", strerror(errno()));

    create_max_threads(_metadata, thread_lock_fn);
    join_max_threads(_metadata);

    ret = futex_hash_slots_get();
    EXPECT_EQ!(ret, 0);
    TH_LOG!("Continue to use global hash failed");
}

// TEST_HARNESS_MAIN
