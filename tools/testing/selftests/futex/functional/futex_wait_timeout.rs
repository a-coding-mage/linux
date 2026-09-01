// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 *   Copyright © International Business Machines  Corp., 2009
 *
 * DESCRIPTION
 *      Block on a futex and wait for timeout.
 *
 * AUTHOR
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2009-Nov-6: Initial version by Darren Hart <dvhart@linux.intel.com>
 *      2021-Apr-26: More test cases by André Almeida <andrealmeid@collabora.com>
 *
 *****************************************************************************/

// C dependencies: pthread.h, futextest.h, futex2test.h, kselftest_harness.h

type c_int = i32;
type c_long = i64;
type c_uint = u32;
type c_void = core::ffi::c_void;
type uintptr_t = usize;
type futex_t = u32;
type pthread_t = usize;

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: c_long,
}

#[repr(C)]
struct pthread_barrier_t {
    _private: [u8; 0],
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct futex_waitv {
    uaddr: uintptr_t,
    val: u64,
    flags: u32,
    __reserved: u32,
}

const FUTEX_INITIALIZER: futex_t = 0;
const FUTEX_CLOCK_REALTIME: c_int = 256;
const FUTEX_32: u32 = 2;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const ETIMEDOUT: c_int = 110;
const ENOSYS: c_int = 38;

static mut timeout_ns: c_long = 100000; /* 100us default timeout */
static mut futex_pi: futex_t = 0;
static mut barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };

unsafe extern "C" {
    static mut errno: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;

    fn futex_lock_pi(uaddr: *mut futex_t, timeout: *const timespec, detect: c_int, opflags: c_int)
        -> c_int;
    fn futex_wait(uaddr: *mut futex_t, val: futex_t, timeout: *const timespec, opflags: c_int)
        -> c_int;
    fn futex_wait_bitset(
        uaddr: *mut futex_t,
        val: futex_t,
        timeout: *const timespec,
        bitset: c_uint,
        opflags: c_int,
    ) -> c_int;
    fn futex_wait_requeue_pi(
        uaddr: *mut futex_t,
        val: futex_t,
        uaddr2: *mut futex_t,
        timeout: *const timespec,
        opflags: c_int,
    ) -> c_int;
    fn futex_waitv(
        waiters: *mut futex_waitv,
        nr_futexes: c_uint,
        flags: c_uint,
        timeout: *const timespec,
        clockid: c_int,
    ) -> c_int;
    fn is_futex_waitv_supported() -> bool;
}

/*
 * Get a PI lock and hold it forever, so the main thread lock_pi will block
 * and we can test the timeout
 */
unsafe extern "C" fn get_pi_lock(arg: *mut c_void) -> *mut c_void {
    let _metadata: *mut __test_metadata = arg as *mut __test_metadata;
    let mut ret: c_int;
    let mut lock: futex_t = 0;

    ret = futex_lock_pi(&raw mut futex_pi, core::ptr::null(), 0, 0);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("futex_lock_pi failed");

    pthread_barrier_wait(&raw mut barrier);

    /* Blocks forever */
    ret = futex_wait(&mut lock, 0, core::ptr::null(), 0);
    ASSERT_TRUE!(0);
    TH_LOG!("futex_wait returned unexpectedly: %d", ret);

    core::ptr::null_mut()
}

macro_rules! TEST_TIMEOUT {
    ($_res:expr, $_test_name:expr, $_err:expr) => {{
        if ($_res) < 0 && unsafe { errno } == ENOSYS && ($_err) != ENOSYS {
            SKIP!(return, "%s is not supported (ENOSYS)", $_test_name);
        }
        EXPECT_EQ!(($_res), -1);
        TH_LOG!(
            "%s returned unexpected result: %d",
            $_test_name,
            ($_res)
        );
        if ($_res) == -1 {
            EXPECT_EQ!(unsafe { errno }, ($_err));
            TH_LOG!(
                "%s returned unexpected errno: %d (expected %d)",
                $_test_name,
                unsafe { errno },
                ($_err)
            );
        }
    }};
}

macro_rules! GET_ABS_TIMEOUT {
    ($_clockid:expr, $_to:expr, $_timeout_ns:expr) => {{
        ASSERT_EQ!(unsafe { clock_gettime(($_clockid), ($_to)) }, 0);
        TH_LOG!("clock_gettime failed");
        unsafe {
            (*($_to)).tv_nsec += ($_timeout_ns);
            if (*($_to)).tv_nsec >= 1000000000 {
                (*($_to)).tv_sec += 1;
                (*($_to)).tv_nsec -= 1000000000;
            }
        }
    }};
}

// TEST(wait_bitset)
unsafe fn wait_bitset(_metadata: *mut __test_metadata) {
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut to: timespec = core::mem::zeroed();
    let mut res: c_int;

    /* initialize relative timeout */
    to.tv_sec = 0;
    to.tv_nsec = timeout_ns;

    res = futex_wait(&mut f1, f1, &to, 0);
    TEST_TIMEOUT!(res, "futex_wait relative", ETIMEDOUT);

    /* FUTEX_WAIT_BITSET with CLOCK_REALTIME */
    GET_ABS_TIMEOUT!(CLOCK_REALTIME, &mut to, timeout_ns);
    res = futex_wait_bitset(&mut f1, f1, &to, 1, FUTEX_CLOCK_REALTIME);
    TEST_TIMEOUT!(res, "futex_wait_bitset realtime", ETIMEDOUT);

    /* FUTEX_WAIT_BITSET with CLOCK_MONOTONIC */
    GET_ABS_TIMEOUT!(CLOCK_MONOTONIC, &mut to, timeout_ns);
    res = futex_wait_bitset(&mut f1, f1, &to, 1, 0);
    TEST_TIMEOUT!(res, "futex_wait_bitset monotonic", ETIMEDOUT);
}

// TEST(requeue_pi)
unsafe fn requeue_pi(_metadata: *mut __test_metadata) {
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut to: timespec = core::mem::zeroed();
    let mut res: c_int;

    /* FUTEX_WAIT_REQUEUE_PI with CLOCK_REALTIME */
    GET_ABS_TIMEOUT!(CLOCK_REALTIME, &mut to, timeout_ns);
    res = futex_wait_requeue_pi(&mut f1, f1, &raw mut futex_pi, &to, FUTEX_CLOCK_REALTIME);
    TEST_TIMEOUT!(res, "futex_wait_requeue_pi realtime", ETIMEDOUT);

    /* FUTEX_WAIT_REQUEUE_PI with CLOCK_MONOTONIC */
    GET_ABS_TIMEOUT!(CLOCK_MONOTONIC, &mut to, timeout_ns);
    res = futex_wait_requeue_pi(&mut f1, f1, &raw mut futex_pi, &to, 0);
    TEST_TIMEOUT!(res, "futex_wait_requeue_pi monotonic", ETIMEDOUT);
}

// TEST(lock_pi)
unsafe fn lock_pi(_metadata: *mut __test_metadata) {
    let mut to: timespec = core::mem::zeroed();
    let mut thread: pthread_t = 0;
    let mut res: c_int;

    /* Create a thread that will lock forever so any waiter will timeout */
    pthread_barrier_init(&raw mut barrier, core::ptr::null(), 2);
    ASSERT_EQ!(
        pthread_create(
            &mut thread,
            core::ptr::null(),
            get_pi_lock,
            _metadata as *mut c_void
        ),
        0
    );
    TH_LOG!("pthread_create failed");

    /* Wait until the other thread calls futex_lock_pi() */
    pthread_barrier_wait(&raw mut barrier);
    pthread_barrier_destroy(&raw mut barrier);

    /*
     * FUTEX_LOCK_PI with CLOCK_REALTIME
     * Due to historical reasons, FUTEX_LOCK_PI supports only realtime
     * clock, but requires the caller to not set CLOCK_REALTIME flag.
     *
     * If you call FUTEX_LOCK_PI with a monotonic clock, it'll be
     * interpreted as a realtime clock, and (unless you mess your machine's
     * time or your time machine) the monotonic clock value is always
     * smaller than realtime and the syscall will timeout immediately.
     */
    GET_ABS_TIMEOUT!(CLOCK_REALTIME, &mut to, timeout_ns);
    res = futex_lock_pi(&raw mut futex_pi, &to, 0, 0);
    TEST_TIMEOUT!(res, "futex_lock_pi realtime", ETIMEDOUT);

    /* Test operations that don't support FUTEX_CLOCK_REALTIME */
    res = futex_lock_pi(
        &raw mut futex_pi,
        core::ptr::null(),
        0,
        FUTEX_CLOCK_REALTIME,
    );
    TEST_TIMEOUT!(res, "futex_lock_pi invalid timeout flag", ENOSYS);
}

// TEST(waitv)
unsafe fn waitv(_metadata: *mut __test_metadata) {
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut waitv: futex_waitv = futex_waitv {
        uaddr: (&mut f1 as *mut futex_t) as uintptr_t,
        val: f1 as u64,
        flags: FUTEX_32,
        __reserved: 0,
    };
    let mut to: timespec = core::mem::zeroed();
    let mut res: c_int;

    if !is_futex_waitv_supported() {
        SKIP!(return, "futex_waitv syscall not supported");
    }

    /* futex_waitv with CLOCK_MONOTONIC */
    GET_ABS_TIMEOUT!(CLOCK_MONOTONIC, &mut to, timeout_ns);
    res = futex_waitv(&mut waitv, 1, 0, &to, CLOCK_MONOTONIC);
    TEST_TIMEOUT!(res, "futex_waitv monotonic", ETIMEDOUT);

    /* futex_waitv with CLOCK_REALTIME */
    GET_ABS_TIMEOUT!(CLOCK_REALTIME, &mut to, timeout_ns);
    res = futex_waitv(&mut waitv, 1, 0, &to, CLOCK_REALTIME);
    TEST_TIMEOUT!(res, "futex_waitv realtime", ETIMEDOUT);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
