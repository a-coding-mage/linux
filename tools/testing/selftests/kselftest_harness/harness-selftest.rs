// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C dependencies:
// #include <stdio.h>
// #include <sys/resource.h>
// #include <sys/prctl.h>
// #include "kselftest_harness.h"
//
// Avoid any inconsistencies:
// #define TH_LOG_STREAM stdout

type pid_t = c_int;
type rlim_t = c_ulong;

const SIGUSR1: c_int = 10;
const PR_SET_DUMPABLE: c_int = 4;
const RLIMIT_CORE: c_int = 4;

const KSFT_PASS: c_int = 0;
const KSFT_XPASS: c_int = 2;
const KSFT_FAIL: c_int = 1;
const KSFT_XFAIL: c_int = 3;
const KSFT_SKIP: c_int = 4;

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __fixture_variant {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fixture {
    pub testpid: pid_t,
}

#[repr(C)]
pub struct fixture_parent {
    pub testpid: pid_t,
}

#[repr(C)]
pub struct fixture_setup_failure {
    pub testpid: pid_t,
}

unsafe extern "C" {
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn exit(status: c_int) -> !;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;

    fn ksft_test_result_pass(msg: *const c_char);
    fn ksft_test_result_xpass(msg: *const c_char);
    fn ksft_test_result_fail(msg: *const c_char);
    fn ksft_test_result_xfail(msg: *const c_char);
    fn ksft_test_result_skip(msg: *const c_char);
}

type c_uint = u32;

macro_rules! TH_LOG {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        th_log(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr $(,)?) => {
        assert_eq_external(($left) as _, ($right) as _)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr $(,)?) => {
        expect_eq_external(($left) as _, ($right) as _)
    };
}

unsafe extern "C" {
    fn th_log(fmt: *const c_char, ...);
    fn assert_eq_external(left: c_int, right: c_int);
    fn expect_eq_external(left: c_int, right: c_int);
}

unsafe fn test_helper(_metadata: *mut __test_metadata) {
    unsafe {
        ASSERT_EQ!(0, 0);
    }
}

// TEST(standalone_pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn standalone_pass(_metadata: *mut __test_metadata) {
    unsafe {
        TH_LOG!("before");
        ASSERT_EQ!(0, 0);
        EXPECT_EQ!(0, 0);
        test_helper(_metadata);
        TH_LOG!("after");
    }
}

// TEST(standalone_fail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn standalone_fail(_metadata: *mut __test_metadata) {
    unsafe {
        TH_LOG!("before");
        EXPECT_EQ!(0, 0);
        EXPECT_EQ!(0, 1);
        ASSERT_EQ!(0, 1);
        TH_LOG!("after");
    }
}

// TEST_SIGNAL(signal_pass, SIGUSR1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_pass(_metadata: *mut __test_metadata) {
    unsafe {
        TH_LOG!("before");
        ASSERT_EQ!(0, 0);
        TH_LOG!("after");
        kill(getpid(), SIGUSR1);
    }
}

// TEST_SIGNAL(signal_fail, SIGUSR1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_fail(_metadata: *mut __test_metadata) {
    unsafe {
        TH_LOG!("before");
        ASSERT_EQ!(0, 1);
        TH_LOG!("after");
        kill(getpid(), SIGUSR1);
    }
}

// FIXTURE_SETUP(fixture)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_setup(_metadata: *mut __test_metadata, self_: *mut fixture) {
    unsafe {
        TH_LOG!("setup");
        (*self_).testpid = getpid();
    }
}

// FIXTURE_TEARDOWN(fixture)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_teardown(_metadata: *mut __test_metadata, self_: *mut fixture) {
    unsafe {
        TH_LOG!("teardown same-process=%d", ((*self_).testpid == getpid()) as c_int);
    }
}

// TEST_F(fixture, pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_pass(
    _metadata: *mut __test_metadata,
    self_: *mut fixture,
    variant: *const __fixture_variant,
) {
    unsafe {
        let _ = self_;
        let _ = variant;
        TH_LOG!("before");
        ASSERT_EQ!(0, 0);
        test_helper(_metadata);
        standalone_pass(_metadata);
        TH_LOG!("after");
    }
}

// TEST_F(fixture, fail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_fail(
    _metadata: *mut __test_metadata,
    self_: *mut fixture,
    variant: *const __fixture_variant,
) {
    unsafe {
        TH_LOG!("before");
        ASSERT_EQ!(0, 1);
        fixture_pass(_metadata, self_, variant);
        TH_LOG!("after");
    }
}

// TEST_F_TIMEOUT(fixture, timeout, 1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_timeout(
    _metadata: *mut __test_metadata,
    self_: *mut fixture,
    variant: *const __fixture_variant,
) {
    unsafe {
        let _ = _metadata;
        let _ = self_;
        let _ = variant;
        TH_LOG!("before");
        sleep(2);
        TH_LOG!("after");
    }
}

// FIXTURE_SETUP(fixture_parent)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_parent_setup(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_parent,
) {
    unsafe {
        TH_LOG!("setup");
        (*self_).testpid = getpid();
    }
}

// FIXTURE_TEARDOWN_PARENT(fixture_parent)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_parent_teardown_parent(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_parent,
) {
    unsafe {
        TH_LOG!("teardown same-process=%d", ((*self_).testpid == getpid()) as c_int);
    }
}

// TEST_F(fixture_parent, pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_parent_pass(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_parent,
    variant: *const __fixture_variant,
) {
    unsafe {
        let _ = _metadata;
        let _ = self_;
        let _ = variant;
        TH_LOG!("before");
        ASSERT_EQ!(0, 0);
        TH_LOG!("after");
    }
}

// FIXTURE_SETUP(fixture_setup_failure)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_setup_failure_setup(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_setup_failure,
) {
    unsafe {
        let _ = _metadata;
        TH_LOG!("setup");
        (*self_).testpid = getpid();
        ASSERT_EQ!(0, 1);
    }
}

// FIXTURE_TEARDOWN(fixture_setup_failure)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_setup_failure_teardown(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_setup_failure,
) {
    unsafe {
        let _ = _metadata;
        TH_LOG!("teardown same-process=%d", ((*self_).testpid == getpid()) as c_int);
    }
}

// TEST_F(fixture_setup_failure, pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixture_setup_failure_pass(
    _metadata: *mut __test_metadata,
    self_: *mut fixture_setup_failure,
    variant: *const __fixture_variant,
) {
    unsafe {
        let _ = _metadata;
        let _ = self_;
        let _ = variant;
        TH_LOG!("before");
        ASSERT_EQ!(0, 0);
        TH_LOG!("after");
    }
}

// TEST(exit_pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_pass(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        exit(KSFT_PASS);
    }
}

// TEST(exit_xpass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_xpass(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        exit(KSFT_XPASS);
    }
}

// TEST(exit_fail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_fail(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        exit(KSFT_FAIL);
    }
}

// TEST(exit_xfail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_xfail(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        exit(KSFT_XFAIL);
    }
}

// TEST(exit_skip)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_skip(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        exit(KSFT_SKIP);
    }
}

// TEST(test_result_pass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_result_pass(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        ksft_test_result_pass(c"".as_ptr());
    }
}

// TEST(test_result_xpass)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_result_xpass(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        ksft_test_result_xpass(c"".as_ptr());
    }
}

// TEST(test_result_fail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_result_fail(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        ksft_test_result_fail(c"".as_ptr());
    }
}

// TEST(test_result_xfail)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_result_xfail(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        ksft_test_result_xfail(c"".as_ptr());
    }
}

// TEST(test_result_skip)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_result_skip(_metadata: *mut __test_metadata) {
    unsafe {
        let _ = _metadata;
        ksft_test_result_skip(c"".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        /*
         * The harness uses abort() to signal assertion failures, which triggers coredumps.
         * This may be useful to debug real failures but not for this selftest, disable them.
         */
        let rlimit = rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };

        prctl(PR_SET_DUMPABLE, 0, 0, 0, 0);
        setrlimit(RLIMIT_CORE, &rlimit);

        test_harness_run(argc, argv)
    }
}
