// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source. Original includes:
// <test_progs.h>, "test_deny_namespace.skel.h", <sched.h>, "cap_helpers.h",
// <stdio.h>. External test, libc, capability, and skeleton symbols are
// declared here and supplied by the surrounding build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type pid_t = i32;
type __u32 = u32;
type __u64 = u64;

const CAP_SYS_ADMIN: i32 = 21;
const CLONE_NEWUSER: i32 = 0x10000000;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const EINTR: i32 = 4;
const EPERM: i32 = 1;

#[repr(C)]
struct test_deny_namespace {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: i32;

    fn waitpid(pid: pid_t, status: *mut i32, options: i32) -> pid_t;
    fn fork() -> pid_t;
    fn unshare(flags: i32) -> i32;
    fn _exit(status: i32) -> !;

    fn cap_enable_effective(mask: __u32, old_caps: *mut __u64);
    fn cap_disable_effective(mask: __u32, old_caps: *mut __u64);

    fn test__start_subtest(name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(ret: i32, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_EQ(
        actual: i32,
        expected: i32,
        name: *const core::ffi::c_char,
    ) -> bool;
    fn ASSERT_OK_PTR(
        ptr: *mut test_deny_namespace,
        name: *const core::ffi::c_char,
    ) -> bool;

    fn test_deny_namespace__open_and_load() -> *mut test_deny_namespace;
    fn test_deny_namespace__attach(skel: *mut test_deny_namespace) -> i32;
    fn test_deny_namespace__detach(skel: *mut test_deny_namespace);
    fn test_deny_namespace__destroy(skel: *mut test_deny_namespace);
}

fn WIFEXITED(status: i32) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: i32) -> i32 {
    (status & 0xff00) >> 8
}

unsafe fn wait_for_pid(pid: pid_t) -> i32 {
    let mut status: i32 = 0;
    let mut ret: i32;

    loop {
        ret = waitpid(pid, &mut status, 0);
        if ret == -1 {
            if errno == EINTR {
                continue;
            }

            return -1;
        }

        break;
    }

    if !WIFEXITED(status) {
        return -1;
    }

    WEXITSTATUS(status)
}

/* negative return value -> some internal error
 * positive return value -> userns creation failed
 * 0                     -> userns creation succeeded
 */
unsafe fn create_user_ns() -> i32 {
    let mut pid: pid_t;

    pid = fork();
    if pid < 0 {
        return -1;
    }

    if pid == 0 {
        if unshare(CLONE_NEWUSER) != 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    wait_for_pid(pid)
}

unsafe fn test_userns_create_bpf() {
    let cap_mask: __u32 = (1u64 << CAP_SYS_ADMIN) as __u32;
    let mut old_caps: __u64 = 0;

    cap_enable_effective(cap_mask, &mut old_caps);

    ASSERT_OK(create_user_ns(), c"priv new user ns".as_ptr());

    cap_disable_effective(cap_mask, &mut old_caps);

    ASSERT_EQ(create_user_ns(), EPERM, c"unpriv new user ns".as_ptr());

    if (cap_mask as __u64 & old_caps) != 0 {
        cap_enable_effective(cap_mask, core::ptr::null_mut());
    }
}

unsafe fn test_unpriv_userns_create_no_bpf() {
    let cap_mask: __u32 = (1u64 << CAP_SYS_ADMIN) as __u32;
    let mut old_caps: __u64 = 0;

    cap_disable_effective(cap_mask, &mut old_caps);

    ASSERT_OK(create_user_ns(), c"no-bpf unpriv new user ns".as_ptr());

    if (cap_mask as __u64 & old_caps) != 0 {
        cap_enable_effective(cap_mask, core::ptr::null_mut());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_deny_namespace() {
    let mut skel: *mut test_deny_namespace = core::ptr::null_mut();
    let mut err: i32;

    if test__start_subtest(c"unpriv_userns_create_no_bpf".as_ptr()) {
        test_unpriv_userns_create_no_bpf();
    }

    skel = test_deny_namespace__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel load".as_ptr()) {
        return test_deny_namespace__destroy(skel);
    }

    err = test_deny_namespace__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        return test_deny_namespace__destroy(skel);
    }

    if test__start_subtest(c"userns_create_bpf".as_ptr()) {
        test_userns_create_bpf();
    }

    test_deny_namespace__detach(skel);

    test_deny_namespace__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
