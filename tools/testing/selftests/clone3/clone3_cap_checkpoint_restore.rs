// SPDX-License-Identifier: GPL-2.0

/*
 * Based on Christian Brauner's clone3() example.
 * These tests are assuming to be running in the host's
 * PID namespace.
 */

/* capabilities related code based on selftests/bpf/test_verifier.c */

/* Translated from C implementation source. Original includes:
 * errno.h, linux/types.h, linux/sched.h, stdio.h, stdlib.h, stdbool.h,
 * sys/capability.h, sys/prctl.h, sys/syscall.h, sys/types.h, sys/un.h,
 * sys/wait.h, unistd.h, sched.h, kselftest_harness.h,
 * clone3_selftests.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type pid_t = i32;
type size_t = usize;
type c_int = i32;
type c_uint = u32;
type c_char = i8;
type c_void = core::ffi::c_void;
type cap_value_t = c_int;
type cap_t = *mut c_void;

const SIGCHLD: c_int = 17;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const EPERM: c_int = 1;
const PR_SET_KEEPCAPS: c_int = 8;
const CAP_SETUID: cap_value_t = 7;
const CAP_SETGID: cap_value_t = 6;
const CAP_CHECKPOINT_RESTORE: cap_value_t = 40;
const CAP_EFFECTIVE: c_int = 0;
const CAP_PERMITTED: c_int = 1;
const CAP_SET: c_int = 1;

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct __clone_args {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

extern "C" {
    static mut errno: c_int;

    fn fflush(stream: *mut c_void) -> c_int;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
    fn _exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn getpid() -> pid_t;
    fn getuid() -> c_uint;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn fork() -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn setgid(gid: c_uint) -> c_int;
    fn setuid(uid: c_uint) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn cap_get_proc() -> cap_t;
    fn cap_clear(cap_p: cap_t) -> c_int;
    fn cap_set_flag(
        cap_p: cap_t,
        flag: c_int,
        ncap: c_int,
        caps: *const cap_value_t,
        value: c_int,
    ) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(cap_p: cap_t) -> c_int;

    fn ptr_to_u64(ptr: *const pid_t) -> u64;
    fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t;
    fn test_clone3_supported();

    fn TH_LOG(fmt: *const c_char, ...);
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn child_exit(ret: c_int) -> ! {
    fflush(stdout);
    fflush(stderr);
    _exit(ret);
}

unsafe fn call_clone3_set_tid(
    _metadata: *mut __test_metadata,
    set_tid: *mut pid_t,
    set_tid_size: size_t,
) -> c_int {
    let mut status: c_int = 0;
    let mut pid: pid_t = -1;

    let mut args = __clone_args {
        flags: 0,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: ptr_to_u64(set_tid as *const pid_t),
        set_tid_size: set_tid_size as u64,
        cgroup: 0,
    };

    pid = sys_clone3(&mut args, core::mem::size_of_val(&args));
    if pid < 0 {
        TH_LOG(
            b"%s - Failed to create new process\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return -errno;
    }

    if pid == 0 {
        TH_LOG(
            b"I am the child, my PID is %d (expected %d)\0".as_ptr() as *const c_char,
            getpid(),
            *set_tid.add(0),
        );

        if *set_tid.add(0) != getpid() {
            child_exit(EXIT_FAILURE);
        }
        child_exit(EXIT_SUCCESS);
    }

    TH_LOG(
        b"I am the parent (%d). My child's pid is %d\0".as_ptr() as *const c_char,
        getpid(),
        pid,
    );

    if waitpid(pid, &mut status, 0) < 0 {
        TH_LOG(
            b"Child returned %s\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return -errno;
    }

    if !WIFEXITED(status) {
        return -1;
    }

    WEXITSTATUS(status)
}

unsafe fn test_clone3_set_tid(
    _metadata: *mut __test_metadata,
    set_tid: *mut pid_t,
    set_tid_size: size_t,
) -> c_int {
    let ret: c_int;

    TH_LOG(
        b"[%d] Trying clone3() with CLONE_SET_TID to %d\0".as_ptr() as *const c_char,
        getpid(),
        *set_tid.add(0),
    );
    ret = call_clone3_set_tid(_metadata, set_tid, set_tid_size);
    TH_LOG(
        b"[%d] clone3() with CLONE_SET_TID %d says:%d\0".as_ptr() as *const c_char,
        getpid(),
        *set_tid.add(0),
        ret,
    );
    ret
}

unsafe fn set_capability() -> c_int {
    let cap_values: [cap_value_t; 3] = [CAP_SETUID, CAP_SETGID, CAP_CHECKPOINT_RESTORE];
    let mut ret: c_int = -1;
    let caps: cap_t;

    caps = cap_get_proc();
    if caps.is_null() {
        perror(b"cap_get_proc\0".as_ptr() as *const c_char);
        return -1;
    }

    /* Drop all capabilities */
    if cap_clear(caps) != 0 {
        perror(b"cap_clear\0".as_ptr() as *const c_char);
        if cap_free(caps) != 0 {
            perror(b"cap_free\0".as_ptr() as *const c_char);
        }
        return ret;
    }

    cap_set_flag(caps, CAP_EFFECTIVE, 3, cap_values.as_ptr(), CAP_SET);
    cap_set_flag(caps, CAP_PERMITTED, 3, cap_values.as_ptr(), CAP_SET);

    if cap_set_proc(caps) != 0 {
        perror(b"cap_set_proc\0".as_ptr() as *const c_char);
        if cap_free(caps) != 0 {
            perror(b"cap_free\0".as_ptr() as *const c_char);
        }
        return ret;
    }
    ret = 0;

    if cap_free(caps) != 0 {
        perror(b"cap_free\0".as_ptr() as *const c_char);
    }
    ret
}

/* TEST(clone3_cap_checkpoint_restore) */
unsafe fn clone3_cap_checkpoint_restore(_metadata: *mut __test_metadata) {
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut set_tid: [pid_t; 1] = [0; 1];

    test_clone3_supported();

    if getuid() != 0 {
        /* EXPECT_EQ(getuid(), 0) SKIP(return, "Skipping all tests as non-root"); */
        TH_LOG(
            b"Skipping all tests as non-root\0".as_ptr() as *const c_char,
        );
        return;
    }

    memset(
        set_tid.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&set_tid),
    );

    /* Find the current active PID */
    pid = fork();
    if pid == 0 {
        TH_LOG(b"Child has PID %d\0".as_ptr() as *const c_char, getpid());
        child_exit(EXIT_SUCCESS);
    }
    if waitpid(pid, &mut status, 0) <= 0 {
        /* ASSERT_GT(waitpid(pid, &status, 0), 0) TH_LOG("Waiting for child %d failed", pid); */
        TH_LOG(
            b"Waiting for child %d failed\0".as_ptr() as *const c_char,
            pid,
        );
        return;
    }

    /* After the child has finished, its PID should be free. */
    set_tid[0] = pid;

    if set_capability() != 0 {
        /* ASSERT_EQ(set_capability(), 0) TH_LOG("Could not set CAP_CHECKPOINT_RESTORE"); */
        TH_LOG(
            b"Could not set CAP_CHECKPOINT_RESTORE\0".as_ptr() as *const c_char,
        );
        return;
    }

    if prctl(PR_SET_KEEPCAPS, 1 as c_int, 0 as c_int, 0 as c_int, 0 as c_int) != 0 {
        /* ASSERT_EQ(prctl(PR_SET_KEEPCAPS, 1, 0, 0, 0), 0); */
        return;
    }

    if setgid(65534) != 0 {
        /* EXPECT_EQ(setgid(65534), 0) TH_LOG("Failed to setgid(65534)"); */
        TH_LOG(
            b"Failed to setgid(65534)\0".as_ptr() as *const c_char,
        );
    }
    if setuid(65534) != 0 {
        /* ASSERT_EQ(setuid(65534), 0); */
        return;
    }

    set_tid[0] = pid;
    /* This would fail without CAP_CHECKPOINT_RESTORE */
    if test_clone3_set_tid(_metadata, set_tid.as_mut_ptr(), 1) != -EPERM {
        /* ASSERT_EQ(test_clone3_set_tid(_metadata, set_tid, 1), -EPERM); */
        return;
    }
    if set_capability() != 0 {
        /* ASSERT_EQ(set_capability(), 0) TH_LOG("Could not set CAP_CHECKPOINT_RESTORE"); */
        TH_LOG(
            b"Could not set CAP_CHECKPOINT_RESTORE\0".as_ptr() as *const c_char,
        );
        return;
    }
    /* This should work as we have CAP_CHECKPOINT_RESTORE as non-root */
    if test_clone3_set_tid(_metadata, set_tid.as_mut_ptr(), 1) != 0 {
        /* ASSERT_EQ(test_clone3_set_tid(_metadata, set_tid, 1), 0); */
        return;
    }
}

/* TEST_HARNESS_MAIN */
