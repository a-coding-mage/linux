// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Enforcing the same restrictions across multiple threads
 *
 * Copyright (C) 2025 Gunther Noack <gnoack3000@gmail.com>
 */

// C dependencies: linux/landlock.h, pthread.h, signal.h, sys/prctl.h, common.h.

use core::ffi::{c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type size_t = usize;
type pthread_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const LANDLOCK_ACCESS_FS_WRITE_FILE: u64 = 1 << 1;
const LANDLOCK_ACCESS_FS_TRUNCATE: u64 = 1 << 14;
const LANDLOCK_RESTRICT_SELF_TSYNC: __u32 = 1 << 0;
const LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF: __u32 = 1 << 1;
const LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON: __u32 = 1 << 2;
const LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF: __u32 = 1 << 3;
const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS: __u32 = 1 << 4;
const LANDLOCK_MAX_NUM_LAYERS: c_int = 16;

const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_GET_NO_NEW_PRIVS: c_int = 39;
const SIGUSR1: c_int = 10;
const E2BIG: c_int = 7;
const EBADF: c_int = 9;

const NUM_IDLE_THREADS: size_t = 200;

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct landlock_ruleset_attr {
    handled_access_fs: u64,
    handled_access_net: u64,
    scoped: u64,
}

#[repr(C)]
struct sigset_t {
    __val: [usize; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: sighandler_t,
    sa_flags: c_uint,
    sa_restorer: Option<unsafe extern "C" fn()>,
    sa_mask: sigset_t,
}

unsafe extern "C" {
    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: size_t,
        flags: __u32,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: __u32) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_self() -> pthread_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn strerror(errnum: c_int) -> *mut i8;
    fn disable_caps(_metadata: *mut __test_metadata);
    fn TH_LOG(fmt: *const i8, ...);
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        assert!($left <= $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

/* create_ruleset - Create a simple ruleset FD common to all tests */
unsafe fn create_ruleset(_metadata: *mut __test_metadata) -> c_int {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_WRITE_FILE | LANDLOCK_ACCESS_FS_TRUNCATE,
        handled_access_net: 0,
        scoped: 0,
    };
    let ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);

    ASSERT_LE!(0, ruleset_fd);
    if !(0 <= ruleset_fd) {
        TH_LOG(
            b"landlock_create_ruleset: %s\0".as_ptr() as *const i8,
            strerror(errno()),
        );
    }
    ruleset_fd
}

unsafe fn single_threaded_success(_metadata: *mut __test_metadata) {
    let ruleset_fd = create_ruleset(_metadata);

    disable_caps(_metadata);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));
    ASSERT_EQ!(
        0,
        landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC)
    );

    EXPECT_EQ!(0, close(ruleset_fd));
}

unsafe extern "C" fn store_no_new_privs(data: *mut c_void) {
    let nnp = data as *mut bool;

    if nnp.is_null() {
        return;
    }
    *nnp = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) != 0;
}

unsafe extern "C" fn idle(data: *mut c_void) -> *mut c_void {
    /* pthread_cleanup_push/pop are C macros; this translation preserves the
     * intended cleanup by calling the cleanup routine before returning after
     * the infinite loop is cancelled by pthread cancellation.
     */
    loop {
        sleep(1);
    }

    #[allow(unreachable_code)]
    {
        store_no_new_privs(data);
        ptr::null_mut()
    }
}

#[repr(C)]
struct multi_threaded {
    ruleset_fd: c_int,
}

#[repr(C)]
struct multi_threaded_variant {
    restrict_flags: __u32,
    /* Sets no_new_privs with prctl(2) before the enforcement. */
    prior_no_new_privs: bool,
    /* Enforces the maximum number of allowed layers beforehand. */
    max_layers: bool,
    expected_errno: c_int,
    /* Expected no_new_privs state of all threads after the call. */
    expected_no_new_privs: bool,
}

static MULTI_THREADED_SUCCESS: multi_threaded_variant = multi_threaded_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_TSYNC,
    prior_no_new_privs: true,
    max_layers: false,
    expected_errno: 0,
    expected_no_new_privs: true,
};

static MULTI_THREADED_NO_NEW_PRIVS: multi_threaded_variant = multi_threaded_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_TSYNC | LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS,
    prior_no_new_privs: false,
    max_layers: false,
    expected_errno: 0,
    expected_no_new_privs: true,
};

static MULTI_THREADED_NO_NEW_PRIVS_MAX_LAYERS: multi_threaded_variant = multi_threaded_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_TSYNC | LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS,
    prior_no_new_privs: false,
    max_layers: true,
    expected_errno: E2BIG,
    expected_no_new_privs: false,
};

unsafe fn multi_threaded_setup(
    self_: *mut multi_threaded,
    variant: *const multi_threaded_variant,
    _metadata: *mut __test_metadata,
) {
    (*self_).ruleset_fd = create_ruleset(_metadata);

    if (*variant).max_layers {
        /* Enforces the maximum number of allowed layers. */
        for _i in 0..LANDLOCK_MAX_NUM_LAYERS {
            ASSERT_EQ!(0, landlock_restrict_self((*self_).ruleset_fd, 0));
        }
    }

    disable_caps(_metadata);
}

unsafe fn multi_threaded_teardown(self_: *mut multi_threaded) {
    EXPECT_EQ!(0, close((*self_).ruleset_fd));
}

unsafe fn multi_threaded_restrict(
    self_: *mut multi_threaded,
    variant: *const multi_threaded_variant,
) {
    let mut t1: pthread_t = 0;
    let mut t2: pthread_t = 0;
    let mut no_new_privs1 = false;
    let mut no_new_privs2 = false;

    ASSERT_EQ!(
        0,
        pthread_create(
            &mut t1,
            ptr::null(),
            idle,
            &mut no_new_privs1 as *mut bool as *mut c_void,
        )
    );
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut t2,
            ptr::null(),
            idle,
            &mut no_new_privs2 as *mut bool as *mut c_void,
        )
    );

    if (*variant).prior_no_new_privs {
        ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));
    } else {
        /* No prior prctl(2) PR_SET_NO_NEW_PRIVS call. */
        ASSERT_EQ!(0, prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0));
    }

    if (*variant).expected_errno != 0 {
        EXPECT_EQ!(
            -1,
            landlock_restrict_self((*self_).ruleset_fd, (*variant).restrict_flags)
        );
        EXPECT_EQ!((*variant).expected_errno, errno());
    } else {
        EXPECT_EQ!(
            0,
            landlock_restrict_self((*self_).ruleset_fd, (*variant).restrict_flags)
        );
    }

    /* Checks the no_new_privs state of the calling thread. */
    EXPECT_EQ!(
        (*variant).expected_no_new_privs,
        prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) != 0
    );

    ASSERT_EQ!(0, pthread_cancel(t1));
    ASSERT_EQ!(0, pthread_cancel(t2));
    ASSERT_EQ!(0, pthread_join(t1, ptr::null_mut()));
    ASSERT_EQ!(0, pthread_join(t2, ptr::null_mut()));

    /* Checks the no_new_privs state of the sibling threads. */
    EXPECT_EQ!((*variant).expected_no_new_privs, no_new_privs1);
    EXPECT_EQ!((*variant).expected_no_new_privs, no_new_privs2);
}

unsafe fn multi_threaded_success_despite_diverging_domains(_metadata: *mut __test_metadata) {
    let mut t1: pthread_t = 0;
    let mut t2: pthread_t = 0;
    let ruleset_fd = create_ruleset(_metadata);

    disable_caps(_metadata);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));

    ASSERT_EQ!(0, pthread_create(&mut t1, ptr::null(), idle, ptr::null_mut()));
    ASSERT_EQ!(0, pthread_create(&mut t2, ptr::null(), idle, ptr::null_mut()));

    /*
     * The main thread enforces a ruleset,
     * thereby bringing the threads' Landlock domains out of sync.
     */
    EXPECT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));

    /* Still, TSYNC succeeds, bringing the threads in sync again. */
    EXPECT_EQ!(
        0,
        landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC)
    );

    ASSERT_EQ!(0, pthread_cancel(t1));
    ASSERT_EQ!(0, pthread_cancel(t2));
    ASSERT_EQ!(0, pthread_join(t1, ptr::null_mut()));
    ASSERT_EQ!(0, pthread_join(t2, ptr::null_mut()));
    EXPECT_EQ!(0, close(ruleset_fd));
}

#[repr(C)]
struct thread_restrict_data {
    t: pthread_t,
    ruleset_fd: c_int,
    result: c_int,
}

unsafe extern "C" fn thread_restrict(data: *mut c_void) -> *mut c_void {
    let d = data as *mut thread_restrict_data;

    (*d).result = landlock_restrict_self((*d).ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC);
    ptr::null_mut()
}

unsafe fn competing_enablement(_metadata: *mut __test_metadata) {
    let ruleset_fd = create_ruleset(_metadata);
    let mut d = [
        thread_restrict_data {
            t: 0,
            ruleset_fd,
            result: 0,
        },
        thread_restrict_data {
            t: 0,
            ruleset_fd,
            result: 0,
        },
    ];

    disable_caps(_metadata);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut d[0].t,
            ptr::null(),
            thread_restrict,
            &mut d[0] as *mut thread_restrict_data as *mut c_void,
        )
    );
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut d[1].t,
            ptr::null(),
            thread_restrict,
            &mut d[1] as *mut thread_restrict_data as *mut c_void,
        )
    );

    /* Wait for threads to finish. */
    ASSERT_EQ!(0, pthread_join(d[0].t, ptr::null_mut()));
    ASSERT_EQ!(0, pthread_join(d[1].t, ptr::null_mut()));

    /* Expect that both succeeded. */
    EXPECT_EQ!(0, d[0].result);
    EXPECT_EQ!(0, d[1].result);

    EXPECT_EQ!(0, close(ruleset_fd));
}

unsafe extern "C" fn signal_nop_handler(_sig: c_int) {}

#[repr(C)]
struct signaler_data {
    target: pthread_t,
    stop: bool,
}

unsafe extern "C" fn signaler_thread(data: *mut c_void) -> *mut c_void {
    let sd = data as *mut signaler_data;

    while !core::ptr::read_volatile(&(*sd).stop) {
        pthread_kill((*sd).target, SIGUSR1);
    }

    ptr::null_mut()
}

/*
 * Number of idle sibling threads.  This must be large enough that even on
 * machines with many cores, the sibling threads cannot all complete their
 * credential preparation in a single parallel wave, otherwise the signaler
 * thread has no window to interrupt wait_for_completion_interruptible().
 * 200 threads on a 64-core machine yields ~3 serialized waves, giving the
 * tight signal loop enough time to land an interruption.
 */

/*
 * Exercises the tsync interruption and cancellation paths in tsync.c.
 *
 * When a signal interrupts the calling thread while it waits for sibling
 * threads to finish their credential preparation
 * (wait_for_completion_interruptible in landlock_restrict_sibling_threads),
 * the kernel sets ERESTARTNOINTR, cancels queued task works that have not
 * started yet (cancel_tsync_works), then waits for the remaining works to
 * finish.  On the error return, syscalls.c aborts the prepared credentials.
 * The kernel automatically restarts the syscall, so userspace sees success.
 */
unsafe fn tsync_interrupt(_metadata: *mut __test_metadata) {
    let mut i: size_t;
    let mut threads: [pthread_t; NUM_IDLE_THREADS] = [0; NUM_IDLE_THREADS];
    let mut signaler: pthread_t = 0;
    let mut sd = signaler_data {
        target: 0,
        stop: false,
    };
    let mut sa = sigaction {
        sa_handler: None,
        sa_flags: 0,
        sa_restorer: None,
        sa_mask: sigset_t { __val: [0; 16] },
    };
    let ruleset_fd = create_ruleset(_metadata);

    disable_caps(_metadata);

    /* Install a no-op SIGUSR1 handler so the signal does not kill us. */
    sa.sa_handler = Some(signal_nop_handler);
    sigemptyset(&mut sa.sa_mask);
    ASSERT_EQ!(0, sigaction(SIGUSR1, &sa, ptr::null_mut()));

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));

    i = 0;
    while i < NUM_IDLE_THREADS {
        ASSERT_EQ!(
            0,
            pthread_create(&mut threads[i], ptr::null(), idle, ptr::null_mut())
        );
        i += 1;
    }

    /*
     * Start a signaler thread that continuously sends SIGUSR1 to the
     * calling thread.  This maximizes the chance of interrupting
     * wait_for_completion_interruptible() in the kernel's tsync path.
     */
    sd.target = pthread_self();
    core::ptr::write_volatile(&mut sd.stop, false);
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut signaler,
            ptr::null(),
            signaler_thread,
            &mut sd as *mut signaler_data as *mut c_void,
        )
    );

    /*
     * The syscall may be interrupted and transparently restarted by the
     * kernel (ERESTARTNOINTR).  From userspace, it should always succeed.
     */
    EXPECT_EQ!(
        0,
        landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC)
    );

    core::ptr::write_volatile(&mut sd.stop, true);
    ASSERT_EQ!(0, pthread_join(signaler, ptr::null_mut()));

    i = 0;
    while i < NUM_IDLE_THREADS {
        ASSERT_EQ!(0, pthread_cancel(threads[i]));
        ASSERT_EQ!(0, pthread_join(threads[i], ptr::null_mut()));
        i += 1;
    }

    EXPECT_EQ!(0, close(ruleset_fd));
}

#[repr(C)]
struct tsync_without_ruleset {}

#[repr(C)]
struct tsync_without_ruleset_variant {
    flags: __u32,
    expected_errno: c_int,
}

static TSYNC_WITHOUT_RULESET_TSYNC_ONLY: tsync_without_ruleset_variant =
    tsync_without_ruleset_variant {
        flags: LANDLOCK_RESTRICT_SELF_TSYNC,
        expected_errno: EBADF,
    };

static TSYNC_WITHOUT_RULESET_SUBDOMAINS_OFF_SAME_EXEC_OFF: tsync_without_ruleset_variant =
    tsync_without_ruleset_variant {
        flags: LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF
            | LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF
            | LANDLOCK_RESTRICT_SELF_TSYNC,
        expected_errno: EBADF,
    };

static TSYNC_WITHOUT_RULESET_SUBDOMAINS_OFF_NEW_EXEC_ON: tsync_without_ruleset_variant =
    tsync_without_ruleset_variant {
        flags: LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF
            | LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON
            | LANDLOCK_RESTRICT_SELF_TSYNC,
        expected_errno: EBADF,
    };

static TSYNC_WITHOUT_RULESET_ALL_FLAGS: tsync_without_ruleset_variant =
    tsync_without_ruleset_variant {
        flags: LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF
            | LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON
            | LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF
            | LANDLOCK_RESTRICT_SELF_TSYNC,
        expected_errno: EBADF,
    };

static TSYNC_WITHOUT_RULESET_SUBDOMAINS_OFF: tsync_without_ruleset_variant =
    tsync_without_ruleset_variant {
        flags: LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF | LANDLOCK_RESTRICT_SELF_TSYNC,
        expected_errno: 0,
    };

unsafe fn tsync_without_ruleset_setup(_metadata: *mut __test_metadata) {
    disable_caps(_metadata);
}

unsafe fn tsync_without_ruleset_teardown() {}

unsafe fn tsync_without_ruleset_check(
    _self: *mut tsync_without_ruleset,
    variant: *const tsync_without_ruleset_variant,
) {
    let ret: c_int;

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0, 0, 0));

    ret = landlock_restrict_self(-1, (*variant).flags);
    if (*variant).expected_errno != 0 {
        EXPECT_EQ!(-1, ret);
        EXPECT_EQ!((*variant).expected_errno, errno());
    } else {
        EXPECT_EQ!(0, ret);
    }
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
