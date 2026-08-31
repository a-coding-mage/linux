/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012 The Chromium OS Authors. All rights reserved.
 *
 * kselftest_harness.h: simple C unit test helper.
 *
 * See documentation in Documentation/dev-tools/kselftest.rst
 *
 * API inspired by code.google.com/p/googletest
 *
 * Rust source-level translation of testing/selftests/kselftest_harness.h.
 * C include directives, header guards, constructor attributes, token-pasting
 * declaration macros, and GNU statement-expression details are preserved as
 * comments or macro intent where Rust has no direct file-local equivalent.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub type pid_t = c_int;
pub type size_t = usize;

/* External dependencies supplied by the original C includes and kselftest.h. */
unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut ksft_debug_enabled: bool;

    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn abort() -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn islower(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn setpgrp() -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...) -> c_int;
    fn ksft_reset_state();
    fn ksft_get_fail_cnt() -> c_uint;
    fn ksft_get_error_cnt() -> c_uint;
    fn ksft_test_result_code(code: c_int, test_name: *const c_char, fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit(pass: bool) -> !;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

pub type c_short = i16;

pub const KSELFTEST_PRIO_TEST: c_int = 20000;
pub const KSELFTEST_PRIO_XFAIL: c_int = 20001;
pub const TEST_TIMEOUT_DEFAULT: c_int = 30;

pub const TH_LOG_ENABLED: c_int = 1;

/* Values supplied by Linux/kselftest headers in C builds. */
pub const KSFT_PASS: c_int = 0;
pub const KSFT_FAIL: c_int = 1;
pub const KSFT_SKIP: c_int = 4;
pub const KSFT_XFAIL: c_int = 2;
pub const KSFT_XPASS: c_int = 3;

pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const MAP_SHARED: c_int = 0x01;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const POLLIN: c_short = 0x001;
pub const WNOHANG: c_int = 1;
pub const EINTR: c_int = 4;
pub const SIGKILL: c_int = 9;
pub const SIGABRT: c_int = 6;
pub const __NR_pidfd_open: c_long = 434;

#[inline]
pub unsafe fn __kselftest_memset_safe(s: *mut c_void, c: c_int, n: size_t) {
    if n > 0 {
        unsafe {
            memset(s, c, n);
        }
    }
}

/*
 * TH_LOG(), __TH_LOG(), SKIP(), TEST(), TEST_SIGNAL(), FIXTURE(),
 * FIXTURE_SETUP(), FIXTURE_TEARDOWN(), FIXTURE_TEARDOWN_PARENT(),
 * FIXTURE_VARIANT(), FIXTURE_VARIANT_ADD(), TEST_F(), TEST_F_SIGNAL(),
 * TEST_F_TIMEOUT(), TEST_HARNESS_MAIN, ASSERT_*(), EXPECT_*(), ARRAY_SIZE(),
 * OPTIONAL_HANDLER(), and XFAIL_ADD() are preprocessor interfaces in C.
 * Rust cannot reproduce C token-pasting declarations and constructor
 * registration as ordinary items without caller-side procedural support, so the
 * following macro_rules definitions preserve the direct expression-level
 * interfaces where possible and document the registration macros' intent.
 */

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ::core::mem::size_of_val(&$a) / ::core::mem::size_of_val(&$a[0])
    };
}

#[macro_export]
macro_rules! OPTIONAL_HANDLER {
    ($_assert:expr, $_metadata:expr) => {
        while unsafe { (*$_metadata).trigger } != 0 {
            unsafe {
                (*$_metadata).trigger = $crate::__bail($_assert, $_metadata);
            }
        }
    };
}

#[macro_export]
macro_rules! ASSERT_EQ {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), ==, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_NE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), !=, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_LT {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), <, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_LE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), <=, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_GT {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), >, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_GE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), >=, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_TRUE {
    ($seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!(0, "0", $seen, stringify!($seen), !=, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! ASSERT_FALSE {
    ($seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!(0, "0", $seen, stringify!($seen), ==, 1, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_EQ {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), ==, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_NE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), !=, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_LT {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), <, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_LE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), <=, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_GT {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), >, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_GE {
    ($expected:expr, $seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!($expected, stringify!($expected), $seen, stringify!($seen), >=, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_TRUE {
    ($seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!(0, "0", $seen, stringify!($seen), !=, 0, $_metadata)
    };
}
#[macro_export]
macro_rules! EXPECT_FALSE {
    ($seen:expr, $_metadata:expr) => {
        $crate::__EXPECT!(0, "0", $seen, stringify!($seen), ==, 0, $_metadata)
    };
}

#[macro_export]
macro_rules! __EXPECT {
    ($_expected:expr, $_expected_str:expr, $_seen:expr, $_seen_str:expr, $op:tt, $_assert:expr, $_metadata:expr) => {{
        let __exp = $_expected;
        let __seen = $_seen;
        if !(__exp $op __seen) {
            unsafe {
                (*$_metadata).exit_code = $crate::KSFT_FAIL;
                (*$_metadata).trigger = 1;
            }
        }
        $crate::OPTIONAL_HANDLER!($_assert, $_metadata);
    }};
}

#[macro_export]
macro_rules! TEST {
    ($test_name:ident $body:block) => {
        /* C version declares test_name, wrapper_test_name, static metadata,
         * constructor registration, then the test body. */
        unsafe fn $test_name(_metadata: *mut $crate::__test_metadata) $body
    };
}

#[macro_export]
macro_rules! TEST_SIGNAL {
    ($test_name:ident, $signal:expr, $body:block) => {
        unsafe fn $test_name(_metadata: *mut $crate::__test_metadata) $body
    };
}

#[macro_export]
macro_rules! TEST_HARNESS_MAIN {
    () => {
        fn main() {
            let args: ::std::vec::Vec<::std::ffi::CString> = ::std::env::args()
                .map(|a| ::std::ffi::CString::new(a).unwrap())
                .collect();
            let mut argv: ::std::vec::Vec<*mut ::core::ffi::c_char> =
                args.iter().map(|a| a.as_ptr() as *mut ::core::ffi::c_char).collect();
            unsafe {
                ::std::process::exit($crate::test_harness_run(argv.len() as ::core::ffi::c_int, argv.as_mut_ptr()));
            }
        }
    };
}

#[repr(C)]
pub struct __test_results {
    pub reason: [c_char; 1024],
}

#[repr(C)]
pub struct __fixture_metadata {
    pub name: *const c_char,
    pub tests: *mut __test_metadata,
    pub variant: *mut __fixture_variant_metadata,
    pub prev: *mut __fixture_metadata,
    pub next: *mut __fixture_metadata,
}

#[repr(C)]
pub struct __test_xfail {
    pub fixture: *mut __fixture_metadata,
    pub variant: *mut __fixture_variant_metadata,
    pub test: *mut __test_metadata,
    pub prev: *mut __test_xfail,
    pub next: *mut __test_xfail,
}

#[repr(C)]
pub struct __fixture_variant_metadata {
    pub name: *const c_char,
    pub data: *const c_void,
    pub xfails: *mut __test_xfail,
    pub prev: *mut __fixture_variant_metadata,
    pub next: *mut __fixture_variant_metadata,
}

pub type __test_fn = Option<unsafe extern "C" fn(*mut __test_metadata, *mut __fixture_variant_metadata)>;
pub type __teardown_fn = Option<unsafe extern "C" fn(bool, *mut __test_metadata, *mut c_void, *const c_void)>;

#[repr(C)]
pub struct __test_metadata {
    pub name: *const c_char,
    pub fn_: __test_fn,
    pub pid: pid_t,
    pub fixture: *mut __fixture_metadata,
    pub teardown_fn: __teardown_fn,
    pub termsig: c_int,
    pub exit_code: c_int,
    pub trigger: c_int,
    pub timeout: c_int,
    pub aborted: bool,
    pub no_teardown: *mut bool,
    pub self_: *mut c_void,
    pub variant: *const c_void,
    pub results: *mut __test_results,
    pub prev: *mut __test_metadata,
    pub next: *mut __test_metadata,
}

static GLOBAL_NAME: &[u8] = b"global\0";

pub static mut _fixture_global: __fixture_metadata = __fixture_metadata {
    name: GLOBAL_NAME.as_ptr() as *const c_char,
    tests: ptr::null_mut(),
    variant: ptr::null_mut(),
    prev: ptr::null_mut(),
    next: ptr::null_mut(),
};

pub static mut __fixture_list: *mut __fixture_metadata = unsafe { &raw mut _fixture_global };
pub static mut __constructor_order_forward: bool = false;

#[inline]
pub unsafe fn __list_append_fixture(head: *mut *mut __fixture_metadata, item: *mut __fixture_metadata) {
    unsafe {
        if (*head).is_null() {
            *head = item;
            (*item).next = ptr::null_mut();
            (*item).prev = item;
            return;
        }
        if __constructor_order_forward {
            (*item).next = ptr::null_mut();
            (*item).prev = (**head).prev;
            (*(*item).prev).next = item;
            (**head).prev = item;
        } else {
            (*item).next = *head;
            (*(*item).next).prev = item;
            (*item).prev = item;
            *head = item;
        }
    }
}

#[inline]
pub unsafe fn __list_append_variant(head: *mut *mut __fixture_variant_metadata, item: *mut __fixture_variant_metadata) {
    unsafe {
        if (*head).is_null() {
            *head = item;
            (*item).next = ptr::null_mut();
            (*item).prev = item;
            return;
        }
        if __constructor_order_forward {
            (*item).next = ptr::null_mut();
            (*item).prev = (**head).prev;
            (*(*item).prev).next = item;
            (**head).prev = item;
        } else {
            (*item).next = *head;
            (*(*item).next).prev = item;
            (*item).prev = item;
            *head = item;
        }
    }
}

#[inline]
pub unsafe fn __list_append_test(head: *mut *mut __test_metadata, item: *mut __test_metadata) {
    unsafe {
        if (*head).is_null() {
            *head = item;
            (*item).next = ptr::null_mut();
            (*item).prev = item;
            return;
        }
        if __constructor_order_forward {
            (*item).next = ptr::null_mut();
            (*item).prev = (**head).prev;
            (*(*item).prev).next = item;
            (**head).prev = item;
        } else {
            (*item).next = *head;
            (*(*item).next).prev = item;
            (*item).prev = item;
            *head = item;
        }
    }
}

#[inline]
pub unsafe fn __list_append_xfail(head: *mut *mut __test_xfail, item: *mut __test_xfail) {
    unsafe {
        if (*head).is_null() {
            *head = item;
            (*item).next = ptr::null_mut();
            (*item).prev = item;
            return;
        }
        if __constructor_order_forward {
            (*item).next = ptr::null_mut();
            (*item).prev = (**head).prev;
            (*(*item).prev).next = item;
            (**head).prev = item;
        } else {
            (*item).next = *head;
            (*(*item).next).prev = item;
            (*item).prev = item;
            *head = item;
        }
    }
}

#[inline]
pub unsafe fn __register_fixture(f: *mut __fixture_metadata) {
    unsafe {
        __list_append_fixture(&raw mut __fixture_list, f);
    }
}

#[inline]
pub unsafe fn __register_fixture_variant(f: *mut __fixture_metadata, variant: *mut __fixture_variant_metadata) {
    unsafe {
        __list_append_variant(&raw mut (*f).variant, variant);
    }
}

#[inline]
pub unsafe fn __test_passed(metadata: *mut __test_metadata) -> bool {
    unsafe { (*metadata).exit_code != KSFT_FAIL && (*metadata).exit_code <= KSFT_SKIP }
}

/*
 * Since constructors are called in reverse order, reverse the test list so
 * tests are run in source declaration order. Some toolchains do not do this
 * correctly, so __constructor_order_forward detects direction and adjusts list
 * building logic.
 */
#[inline]
pub unsafe fn __register_test(t: *mut __test_metadata) {
    unsafe {
        __list_append_test(&raw mut (*(*t).fixture).tests, t);
    }
}

#[inline]
pub unsafe fn __register_xfail(xf: *mut __test_xfail) {
    unsafe {
        __list_append_xfail(&raw mut (*(*xf).variant).xfails, xf);
    }
}

#[inline]
pub unsafe fn __bail(for_realz: c_int, t: *mut __test_metadata) -> c_int {
    unsafe {
        if for_realz != 0 {
            if let Some(teardown_fn) = (*t).teardown_fn {
                teardown_fn(false, t, (*t).self_, (*t).variant);
            }
            abort();
        }
        0
    }
}

#[inline]
unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

#[inline]
unsafe fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) >= 2
}

#[inline]
unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

pub unsafe fn __wait_for_test(t: *mut __test_metadata) {
    unsafe {
        let mut status: c_int = KSFT_FAIL << 8;
        let mut poll_child: pollfd = pollfd { fd: 0, events: 0, revents: 0 };
        let mut timed_out = false;

        let childfd = syscall(__NR_pidfd_open, (*t).pid, 0) as c_int;
        if childfd == -1 {
            (*t).exit_code = KSFT_FAIL;
            fprintf(stderr, cstr(b"# %s: unable to open pidfd\n\0"), (*t).name);
            return;
        }

        poll_child.fd = childfd;
        poll_child.events = POLLIN;
        let ret = poll(&mut poll_child, 1, (*t).timeout * 1000);
        close(childfd);
        if ret == -1 {
            (*t).exit_code = KSFT_FAIL;
            fprintf(stderr, cstr(b"# %s: unable to wait on child pidfd\n\0"), (*t).name);
            return;
        } else if ret == 0 {
            timed_out = true;
            kill(-(*t).pid, SIGKILL);
        }

        let child = waitpid((*t).pid, &mut status, WNOHANG);
        if child == -1 && errno != EINTR {
            (*t).exit_code = KSFT_FAIL;
            fprintf(stderr, cstr(b"# %s: Failed to wait for PID %d (errno: %d)\n\0"), (*t).name, (*t).pid, errno);
            return;
        }

        if timed_out {
            (*t).exit_code = KSFT_FAIL;
            fprintf(stderr, cstr(b"# %s: Test terminated by timeout\n\0"), (*t).name);
        } else if WIFEXITED(status) {
            if WEXITSTATUS(status) == KSFT_SKIP ||
               WEXITSTATUS(status) == KSFT_XPASS ||
               WEXITSTATUS(status) == KSFT_XFAIL {
                (*t).exit_code = WEXITSTATUS(status);
            } else if (*t).termsig != -1 {
                (*t).exit_code = KSFT_FAIL;
                fprintf(stderr, cstr(b"# %s: Test exited normally instead of by signal (code: %d)\n\0"),
                        (*t).name, WEXITSTATUS(status));
            } else {
                match WEXITSTATUS(status) {
                    KSFT_PASS => (*t).exit_code = KSFT_PASS,
                    _ => {
                        (*t).exit_code = KSFT_FAIL;
                        fprintf(stderr, cstr(b"# %s: Test failed\n\0"), (*t).name);
                    }
                }
            }
        } else if WIFSIGNALED(status) {
            (*t).exit_code = KSFT_FAIL;
            if WTERMSIG(status) == SIGABRT {
                fprintf(stderr, cstr(b"# %s: Test terminated by assertion\n\0"), (*t).name);
            } else if WTERMSIG(status) == (*t).termsig {
                (*t).exit_code = KSFT_PASS;
            } else {
                fprintf(stderr, cstr(b"# %s: Test terminated unexpectedly by signal %d\n\0"),
                        (*t).name, WTERMSIG(status));
            }
        } else {
            (*t).exit_code = KSFT_FAIL;
            fprintf(stderr, cstr(b"# %s: Test ended in some other way [%u]\n\0"), (*t).name, status as c_uint);
        }
    }
}

pub unsafe fn test_harness_list_tests() {
    unsafe {
        let mut f = __fixture_list;

        while !f.is_null() {
            let mut v = (*f).variant;
            let mut t = (*f).tests;

            if f == __fixture_list {
                fprintf(stderr, cstr(b"%-20s %-25s %s\n\0"), cstr(b"# FIXTURE\0"), cstr(b"VARIANT\0"), cstr(b"TEST\0"));
            } else {
                fprintf(stderr, cstr(b"--------------------------------------------------------------------------------\n\0"));
            }

            loop {
                fprintf(stderr, cstr(b"%-20s %-25s %s\n\0"),
                        if t == (*f).tests { (*f).name } else { cstr(b"\0") },
                        if !v.is_null() { (*v).name } else { cstr(b"\0") },
                        if !t.is_null() { (*t).name } else { cstr(b"\0") });

                v = if !v.is_null() { (*v).next } else { ptr::null_mut() };
                t = if !t.is_null() { (*t).next } else { ptr::null_mut() };
                if v.is_null() && t.is_null() {
                    break;
                }
            }

            f = (*f).next;
        }
    }
}

pub unsafe fn test_harness_argv_check(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        loop {
            let opt = getopt(argc, argv, cstr(b"dhlF:f:V:v:t:T:r:\0"));
            if opt == -1 {
                break;
            }
            match opt as u8 as char {
                'f' | 'F' | 'v' | 'V' | 't' | 'T' | 'r' => {}
                'l' => {
                    test_harness_list_tests();
                    return KSFT_SKIP;
                }
                'd' => {
                    ksft_debug_enabled = true;
                }
                'h' | _ => {
                    fprintf(stderr,
                            cstr(b"Usage: %s [-h|-l|-d] [-t|-T|-v|-V|-f|-F|-r name]\n\t-h       print help\n\t-l       list all tests\n\t-d       enable debug prints\n\n\t-t name  include test\n\t-T name  exclude test\n\t-v name  include variant\n\t-V name  exclude variant\n\t-f name  include fixture\n\t-F name  exclude fixture\n\t-r name  run specified test\n\nTest filter options can be specified multiple times. The filtering stops\nat the first match. For example to include all tests from variant 'bla'\nbut not test 'foo' specify '-T foo -v bla'.\n\0"),
                            *argv);
                    return if opt as u8 as char == 'h' { KSFT_SKIP } else { KSFT_FAIL };
                }
            }
        }
        KSFT_PASS
    }
}

pub unsafe fn test_enabled(
    argc: c_int,
    argv: *mut *mut c_char,
    f: *mut __fixture_metadata,
    v: *mut __fixture_variant_metadata,
    t: *mut __test_metadata,
) -> bool {
    unsafe {
        let mut flen: c_uint = 0;
        let mut vlen: c_uint = 0;
        let mut tlen: c_uint = 0;
        let mut has_positive = false;

        optind = 1;
        loop {
            let opt = getopt(argc, argv, cstr(b"dF:f:V:v:t:T:r:\0"));
            if opt == -1 {
                break;
            }
            if opt != 'd' as c_int {
                has_positive |= islower(opt) != 0;
            }

            match tolower(opt) as u8 as char {
                't' => {
                    if strcmp((*t).name, optarg) == 0 {
                        return islower(opt) != 0;
                    }
                }
                'f' => {
                    if strcmp((*f).name, optarg) == 0 {
                        return islower(opt) != 0;
                    }
                }
                'v' => {
                    if strcmp((*v).name, optarg) == 0 {
                        return islower(opt) != 0;
                    }
                }
                'r' => {
                    if tlen == 0 {
                        flen = strlen((*f).name) as c_uint;
                        vlen = strlen((*v).name) as c_uint;
                        tlen = strlen((*t).name) as c_uint;
                    }
                    let expected_len = flen + 1 + vlen + if vlen != 0 { 1 } else { 0 } + tlen;
                    if strlen(optarg) as c_uint == expected_len
                        && strncmp((*f).name, optarg.add(0), flen as size_t) == 0
                        && strncmp((*v).name, optarg.add((flen + 1) as usize), vlen as size_t) == 0
                        && strncmp((*t).name, optarg.add((flen + 1 + vlen + if vlen != 0 { 1 } else { 0 }) as usize), tlen as size_t) == 0
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }

        /*
         * If there are no positive tests then we assume user just wants
         * exclusions and everything else is a pass.
         */
        !has_positive
    }
}

pub unsafe fn __run_test(f: *mut __fixture_metadata, variant: *mut __fixture_variant_metadata, t: *mut __test_metadata) {
    unsafe {
        let mut xfail: *mut __test_xfail;
        let mut test_name: [c_char; 1024] = [0; 1024];
        let diagnostic: *const c_char;

        (*t).exit_code = KSFT_PASS;
        (*t).trigger = 0;
        (*t).aborted = false;
        (*t).no_teardown = ptr::null_mut();
        memset((*t).results.cast::<c_void>(), 0, size_of::<__test_results>());

        snprintf(test_name.as_mut_ptr(), test_name.len(), cstr(b"%s%s%s.%s\0"),
                 (*f).name,
                 if *(*variant).name != 0 { cstr(b".\0") } else { cstr(b"\0") },
                 (*variant).name,
                 (*t).name);

        ksft_print_msg(cstr(b" RUN           %s ...\n\0"), test_name.as_ptr());

        fflush(stdout);
        fflush(stderr);

        let child = fork();
        if child < 0 {
            ksft_print_msg(cstr(b"ERROR SPAWNING TEST CHILD\n\0"));
            (*t).exit_code = KSFT_FAIL;
        } else if child == 0 {
            setpgrp();
            ksft_reset_state();

            if let Some(test_fn) = (*t).fn_ {
                test_fn(t, variant);
            }

            if __test_passed(t) && (ksft_get_fail_cnt() != 0 || ksft_get_error_cnt() != 0) {
                ksft_print_msg(cstr(b"Illegal usage of low-level ksft APIs in harness test\n\0"));
                (*t).exit_code = KSFT_FAIL;
            }
            _exit((*t).exit_code);
        } else {
            (*t).pid = child;
            __wait_for_test(t);
        }
        ksft_print_msg(cstr(b"         %4s  %s\n\0"),
                       if __test_passed(t) { cstr(b"OK\0") } else { cstr(b"FAIL\0") },
                       test_name.as_ptr());

        xfail = (*variant).xfails;
        while !xfail.is_null() {
            if (*xfail).test == t {
                break;
            }
            xfail = (*xfail).next;
        }
        if !xfail.is_null() {
            (*t).exit_code = if __test_passed(t) { KSFT_XPASS } else { KSFT_XFAIL };
        }

        if (*(*t).results).reason[0] != 0 {
            diagnostic = (*(*t).results).reason.as_ptr();
        } else if (*t).exit_code == KSFT_PASS || (*t).exit_code == KSFT_FAIL {
            diagnostic = ptr::null();
        } else {
            diagnostic = cstr(b"unknown\0");
        }

        ksft_test_result_code((*t).exit_code, test_name.as_ptr(),
                              if !diagnostic.is_null() { cstr(b"%s\0") } else { ptr::null() },
                              diagnostic);
    }
}

pub unsafe fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut no_variant = __fixture_variant_metadata {
            name: cstr(b"\0"),
            data: ptr::null(),
            xfails: ptr::null_mut(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        };
        let mut ret: c_int;
        let mut case_count: c_uint = 0;
        let mut test_count: c_uint = 0;
        let mut count: c_uint = 0;
        let mut pass_count: c_uint = 0;

        ret = test_harness_argv_check(argc, argv);
        if ret != KSFT_PASS {
            return ret;
        }

        let mut f = __fixture_list;
        while !f.is_null() {
            let mut v = if !(*f).variant.is_null() { (*f).variant } else { &mut no_variant };
            while !v.is_null() {
                let old_tests = test_count;
                let mut t = (*f).tests;
                while !t.is_null() {
                    if test_enabled(argc, argv, f, v, t) {
                        test_count += 1;
                    }
                    t = (*t).next;
                }
                if old_tests != test_count {
                    case_count += 1;
                }
                v = (*v).next;
            }
            f = (*f).next;
        }

        let results = mmap(ptr::null_mut(), size_of::<__test_results>(), PROT_READ | PROT_WRITE,
                           MAP_SHARED | MAP_ANONYMOUS, -1, 0) as *mut __test_results;

        ksft_print_header();
        ksft_set_plan(test_count);
        ksft_print_msg(cstr(b"Starting %u tests from %u test cases.\n\0"), test_count, case_count);

        f = __fixture_list;
        while !f.is_null() {
            let mut v = if !(*f).variant.is_null() { (*f).variant } else { &mut no_variant };
            while !v.is_null() {
                let mut t = (*f).tests;
                while !t.is_null() {
                    if !test_enabled(argc, argv, f, v, t) {
                        t = (*t).next;
                        continue;
                    }
                    count += 1;
                    (*t).results = results;
                    __run_test(f, v, t);
                    (*t).results = ptr::null_mut();
                    if __test_passed(t) {
                        pass_count += 1;
                    } else {
                        ret = 1;
                    }
                    t = (*t).next;
                }
                v = (*v).next;
            }
            f = (*f).next;
        }
        munmap(results.cast::<c_void>(), size_of::<__test_results>());

        ksft_print_msg(cstr(b"%s: %u / %u tests passed.\n\0"),
                       if ret != 0 { cstr(b"FAILED\0") } else { cstr(b"PASSED\0") },
                       pass_count, count);
        ksft_exit(ret == 0);
    }
}

/*
 * C constructor(KSELFTEST_PRIO_TEST):
 * static void __constructor_order_first(void)
 * {
 *     __constructor_order_forward = true;
 * }
 */
pub unsafe fn __constructor_order_first() {
    unsafe {
        __constructor_order_forward = true;
    }
}
