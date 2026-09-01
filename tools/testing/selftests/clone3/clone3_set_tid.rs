// SPDX-License-Identifier: GPL-2.0

/*
 * Based on Christian Brauner's clone3() example.
 * These tests are assuming to be running in the host's
 * PID namespace.
 */

// C dependencies removed from executable Rust:
// errno.h, linux/types.h, linux/sched.h, stdio.h, stdlib.h, stdbool.h,
// sys/syscall.h, sys/types.h, sys/un.h, sys/wait.h, unistd.h, sched.h,
// "kselftest.h", and "clone3_selftests.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type pid_t = c_int;
type uid_t = c_uint;
type size_t = usize;
type FILE = c_void;

const MAX_PID_NS_LEVEL: usize = 32;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const SIGCHLD: c_int = 17;
const CLONE_NEWPID: c_int = 0x20000000;
const EINVAL: c_int = 22;
const EEXIST: c_int = 17;

#[repr(C)]
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

#[repr(C)]
struct ksft_count {
    ksft_pass: c_int,
    ksft_fail: c_int,
    ksft_xfail: c_int,
    ksft_xskip: c_int,
}

extern "C" {
    static mut errno: c_int;
    static mut ksft_cnt: ksft_count;
    static mut ksft_plan: c_int;

    fn fflush(stream: *mut FILE) -> c_int;
    fn _exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn getpid() -> pid_t;
    fn getuid() -> uid_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fork() -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sscanf(str_: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);

    fn ptr_to_u64(ptr: *mut pid_t) -> u64;
    fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t;
    fn test_clone3_supported();

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_num() -> c_int;
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result(condition: bool, format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

static mut pipe_1: [c_int; 2] = [0; 2];
static mut pipe_2: [c_int; 2] = [0; 2];

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn child_exit(ret: c_int) -> ! {
    fflush(core::ptr::null_mut());
    fflush(core::ptr::null_mut());
    _exit(ret);
}

unsafe fn call_clone3_set_tid(
    set_tid: *mut pid_t,
    set_tid_size: size_t,
    flags: c_int,
    expected_pid: c_int,
    wait_for_it: bool,
) -> c_int {
    let mut status: c_int = 0;
    let mut pid: pid_t = -1;

    let mut args = __clone_args {
        flags: flags as u64,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: ptr_to_u64(set_tid),
        set_tid_size: set_tid_size as u64,
        cgroup: 0,
    };

    pid = sys_clone3(&mut args, core::mem::size_of_val(&args));
    if pid < 0 {
        ksft_print_msg(
            b"%s - Failed to create new process\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return -errno;
    }

    if pid == 0 {
        let mut ret: c_int;
        let mut tmp: c_char = 0;
        let mut exit_code: c_int = EXIT_SUCCESS;

        ksft_print_msg(
            b"I am the child, my PID is %d (expected %d)\n\0".as_ptr() as *const c_char,
            getpid(),
            *set_tid.add(0),
        );
        if wait_for_it {
            ksft_print_msg(
                b"[%d] Child is ready and waiting\n\0".as_ptr() as *const c_char,
                getpid(),
            );

            /* Signal the parent that the child is ready */
            close(pipe_1[0]);
            ret = write(pipe_1[1], &mut tmp as *mut c_char as *const c_void, 1) as c_int;
            if ret != 1 {
                ksft_print_msg(
                    b"Writing to pipe returned %d\0".as_ptr() as *const c_char,
                    ret,
                );
                exit_code = EXIT_FAILURE;
            }
            close(pipe_1[1]);
            close(pipe_2[1]);
            ret = read(pipe_2[0], &mut tmp as *mut c_char as *mut c_void, 1) as c_int;
            if ret != 1 {
                ksft_print_msg(
                    b"Reading from pipe returned %d\0".as_ptr() as *const c_char,
                    ret,
                );
                exit_code = EXIT_FAILURE;
            }
            close(pipe_2[0]);
        }

        if *set_tid.add(0) != getpid() {
            child_exit(EXIT_FAILURE);
        }
        child_exit(exit_code);
    }

    if expected_pid == 0 || expected_pid == pid {
        ksft_print_msg(
            b"I am the parent (%d). My child's pid is %d\n\0".as_ptr() as *const c_char,
            getpid(),
            pid,
        );
    } else {
        ksft_print_msg(
            b"Expected child pid %d does not match actual pid %d\n\0".as_ptr() as *const c_char,
            expected_pid,
            pid,
        );
        return -1;
    }

    if waitpid(pid, &mut status, 0) < 0 {
        ksft_print_msg(
            b"Child returned %s\n\0".as_ptr() as *const c_char,
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
    desc: *const c_char,
    set_tid: *mut pid_t,
    set_tid_size: size_t,
    flags: c_int,
    expected: c_int,
    expected_pid: c_int,
    wait_for_it: bool,
) {
    let ret: c_int;

    ksft_print_msg(
        b"[%d] Trying clone3() with CLONE_SET_TID to %d and 0x%x\n\0".as_ptr() as *const c_char,
        getpid(),
        *set_tid.add(0),
        flags,
    );
    ret = call_clone3_set_tid(set_tid, set_tid_size, flags, expected_pid, wait_for_it);
    ksft_print_msg(
        b"[%d] clone3() with CLONE_SET_TID %d says: %d - expected %d\n\0".as_ptr()
            as *const c_char,
        getpid(),
        *set_tid.add(0),
        ret,
        expected,
    );

    ksft_test_result(
        ret == expected,
        b"%s with %zu TIDs and flags 0x%x\n\0".as_ptr() as *const c_char,
        desc,
        set_tid_size,
        flags,
    );
}

pub unsafe fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut f: *mut FILE;
    let mut buf: c_char = 0;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut status: c_int = 0;
    let mut ret: c_int = -1;
    let mut len: size_t = 0;
    let mut pid_max: c_int = 0;
    let uid: uid_t = getuid();
    let mut proc_path: [c_char; 100] = [0; 100];
    let mut pid: pid_t;
    let mut ns1: pid_t = 0;
    let mut ns2: pid_t = 0;
    let mut ns3: pid_t = 0;
    let mut ns_pid: pid_t;
    let mut set_tid: [pid_t; MAX_PID_NS_LEVEL * 2] = [0; MAX_PID_NS_LEVEL * 2];

    ksft_print_header();
    ksft_set_plan(29);
    test_clone3_supported();

    if pipe(pipe_1.as_mut_ptr()) < 0 || pipe(pipe_2.as_mut_ptr()) < 0 {
        ksft_exit_fail_msg(b"pipe() failed\n\0".as_ptr() as *const c_char);
    }

    f = fopen(
        b"/proc/sys/kernel/pid_max\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if f.is_null() {
        ksft_exit_fail_msg(
            b"%s - Could not open /proc/sys/kernel/pid_max\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
    }
    fscanf(f, b"%d\0".as_ptr() as *const c_char, &mut pid_max);
    fclose(f);
    ksft_print_msg(
        b"/proc/sys/kernel/pid_max %d\n\0".as_ptr() as *const c_char,
        pid_max,
    );

    /* Try invalid settings */
    memset(
        set_tid.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&set_tid),
    );
    test_clone3_set_tid(
        b"invalid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL + 1,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 2,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 2 + 1,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 42,
        0,
        -EINVAL,
        0,
        false,
    );

    /*
     * This can actually work if this test running in a MAX_PID_NS_LEVEL - 1
     * nested PID namespace.
     */
    test_clone3_set_tid(
        b"invalid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL - 1,
        0,
        -EINVAL,
        0,
        false,
    );

    memset(
        set_tid.as_mut_ptr() as *mut c_void,
        0xff,
        core::mem::size_of_val(&set_tid),
    );
    test_clone3_set_tid(
        b"invalid size, TID all 1s\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL + 1,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, TID all 1s\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 2,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, TID all 1s\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 2 + 1,
        0,
        -EINVAL,
        0,
        false,
    );

    test_clone3_set_tid(
        b"invalid size, TID all 1s\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL * 42,
        0,
        -EINVAL,
        0,
        false,
    );

    /*
     * This can actually work if this test running in a MAX_PID_NS_LEVEL - 1
     * nested PID namespace.
     */
    test_clone3_set_tid(
        b"invalid size, TID all 1s\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        MAX_PID_NS_LEVEL - 1,
        0,
        -EINVAL,
        0,
        false,
    );

    memset(
        set_tid.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&set_tid),
    );
    /* Try with an invalid PID */
    set_tid[0] = 0;
    test_clone3_set_tid(
        b"valid size, 0 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        1,
        0,
        -EINVAL,
        0,
        false,
    );

    set_tid[0] = -1;
    test_clone3_set_tid(
        b"valid size, -1 TID\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        1,
        0,
        -EINVAL,
        0,
        false,
    );

    /* Claim that the set_tid array actually contains 2 elements. */
    test_clone3_set_tid(
        b"2 TIDs, -1 and 0\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        2,
        0,
        -EINVAL,
        0,
        false,
    );

    /* Try it in a new PID namespace */
    if uid == 0 {
        test_clone3_set_tid(
            b"valid size, -1 TID\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            CLONE_NEWPID,
            -EINVAL,
            0,
            false,
        );
    } else {
        ksft_test_result_skip(
            b"Clone3() with set_tid requires root\n\0".as_ptr() as *const c_char,
        );
    }

    /* Try with a valid PID (1) this should return -EEXIST. */
    set_tid[0] = 1;
    if uid == 0 {
        test_clone3_set_tid(
            b"duplicate PID 1\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            0,
            -EEXIST,
            0,
            false,
        );
    } else {
        ksft_test_result_skip(
            b"Clone3() with set_tid requires root\n\0".as_ptr() as *const c_char,
        );
    }

    /* Try it in a new PID namespace */
    if uid == 0 {
        test_clone3_set_tid(
            b"duplicate PID 1\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            CLONE_NEWPID,
            0,
            0,
            false,
        );
    } else {
        ksft_test_result_skip(
            b"Clone3() with set_tid requires root\n\0".as_ptr() as *const c_char,
        );
    }

    /* pid_max should fail everywhere */
    set_tid[0] = pid_max;
    test_clone3_set_tid(
        b"set TID to maximum\0".as_ptr() as *const c_char,
        set_tid.as_mut_ptr(),
        1,
        0,
        -EINVAL,
        0,
        false,
    );

    if uid == 0 {
        test_clone3_set_tid(
            b"set TID to maximum\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            CLONE_NEWPID,
            -EINVAL,
            0,
            false,
        );
    } else {
        ksft_test_result_skip(
            b"Clone3() with set_tid requires root\n\0".as_ptr() as *const c_char,
        );
    }

    if uid != 0 {
        /*
         * All remaining tests require root. Tell the framework
         * that all those tests are skipped as non-root.
         */
        ksft_cnt.ksft_xskip += ksft_plan - ksft_test_num();
        ret = 0;
    } else {
        /* Find the current active PID */
        pid = fork();
        if pid == 0 {
            ksft_print_msg(
                b"Child has PID %d\n\0".as_ptr() as *const c_char,
                getpid(),
            );
            child_exit(EXIT_SUCCESS);
        }
        if waitpid(pid, &mut status, 0) < 0 {
            ksft_exit_fail_msg(
                b"Waiting for child %d failed\0".as_ptr() as *const c_char,
                pid,
            );
        }

        /* After the child has finished, its PID should be free. */
        set_tid[0] = pid;
        test_clone3_set_tid(
            b"reallocate child TID\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            0,
            0,
            0,
            false,
        );

        /* This should fail as there is no PID 1 in that namespace */
        test_clone3_set_tid(
            b"duplicate child TID\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            CLONE_NEWPID,
            -EINVAL,
            0,
            false,
        );

        /*
         * Creating a process with PID 1 in the newly created most nested
         * PID namespace and PID 'pid' in the parent PID namespace. This
         * needs to work.
         */
        set_tid[0] = 1;
        set_tid[1] = pid;
        test_clone3_set_tid(
            b"create PID 1 in new NS\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            2,
            CLONE_NEWPID,
            0,
            pid,
            false,
        );

        ksft_print_msg(b"unshare PID namespace\n\0".as_ptr() as *const c_char);
        if unshare(CLONE_NEWPID) == -1 {
            ksft_exit_fail_msg(
                b"unshare(CLONE_NEWPID) failed: %s\n\0".as_ptr() as *const c_char,
                strerror(errno),
            );
        }

        set_tid[0] = pid;

        /* This should fail as there is no PID 1 in that namespace */
        test_clone3_set_tid(
            b"duplicate PID 1\0".as_ptr() as *const c_char,
            set_tid.as_mut_ptr(),
            1,
            0,
            -EINVAL,
            0,
            false,
        );

        /* Let's create a PID 1 */
        ns_pid = fork();
        if ns_pid == 0 {
            /*
             * This and the next test cases check that all pid-s are
             * released on error paths.
             */
            set_tid[0] = 43;
            set_tid[1] = -1;
            test_clone3_set_tid(
                b"check leak on invalid TID -1\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                2,
                0,
                -EINVAL,
                0,
                false,
            );

            set_tid[0] = 43;
            set_tid[1] = pid;
            test_clone3_set_tid(
                b"check leak on invalid specific TID\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                2,
                0,
                0,
                43,
                false,
            );

            ksft_print_msg(
                b"Child in PID namespace has PID %d\n\0".as_ptr() as *const c_char,
                getpid(),
            );
            set_tid[0] = 2;
            test_clone3_set_tid(
                b"create PID 2 in child NS\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                1,
                0,
                0,
                2,
                false,
            );

            set_tid[0] = 1;
            set_tid[1] = -1;
            set_tid[2] = pid;
            /* This should fail as there is invalid PID at level '1'. */
            test_clone3_set_tid(
                b"fail due to invalid TID at level 1\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                3,
                CLONE_NEWPID,
                -EINVAL,
                0,
                false,
            );

            set_tid[0] = 1;
            set_tid[1] = 42;
            set_tid[2] = pid;
            /*
             * This should fail as there are not enough active PID
             * namespaces. Again assuming this is running in the host's
             * PID namespace. Not yet nested.
             */
            test_clone3_set_tid(
                b"fail due to too few active PID NSs\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                4,
                CLONE_NEWPID,
                -EINVAL,
                0,
                false,
            );

            /*
             * This should work and from the parent we should see
             * something like 'NSpid:	pid	42	1'.
             */
            test_clone3_set_tid(
                b"verify that we have 3 PID NSs\0".as_ptr() as *const c_char,
                set_tid.as_mut_ptr(),
                3,
                CLONE_NEWPID,
                0,
                42,
                true,
            );

            child_exit(ksft_cnt.ksft_fail);
        }

        close(pipe_1[1]);
        close(pipe_2[0]);
        while read(pipe_1[0], &mut buf as *mut c_char as *mut c_void, 1) > 0 {
            ksft_print_msg(
                b"[%d] Child is ready and waiting\n\0".as_ptr() as *const c_char,
                getpid(),
            );
            break;
        }

        snprintf(
            proc_path.as_mut_ptr(),
            core::mem::size_of_val(&proc_path),
            b"/proc/%d/status\0".as_ptr() as *const c_char,
            pid,
        );
        f = fopen(
            proc_path.as_mut_ptr() as *const c_char,
            b"r\0".as_ptr() as *const c_char,
        );
        if f.is_null() {
            ksft_exit_fail_msg(
                b"%s - Could not open %s\n\0".as_ptr() as *const c_char,
                strerror(errno),
                proc_path.as_mut_ptr(),
            );
        }

        while getline(&mut line, &mut len, f) != -1 {
            if !strstr(line, b"NSpid\0".as_ptr() as *const c_char).is_null() {
                let mut i: c_int;

                /* Verify that all generated PIDs are as expected. */
                i = sscanf(
                    line,
                    b"NSpid:\t%d\t%d\t%d\0".as_ptr() as *const c_char,
                    &mut ns3,
                    &mut ns2,
                    &mut ns1,
                );
                if i != 3 {
                    ksft_print_msg(
                        b"Unexpected 'NSPid:' entry: %s\0".as_ptr() as *const c_char,
                        line,
                    );
                    ns3 = 0;
                    ns2 = ns3;
                    ns1 = ns2;
                }
                break;
            }
        }
        fclose(f);
        free(line as *mut c_void);
        close(pipe_2[0]);

        /* Tell the clone3()'d child to finish. */
        write(pipe_2[1], &mut buf as *mut c_char as *const c_void, 1);
        close(pipe_2[1]);

        if waitpid(ns_pid, &mut status, 0) < 0 {
            ksft_print_msg(
                b"Child returned %s\n\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            ret = -errno;
        } else {
            if !WIFEXITED(status) {
                ksft_test_result_fail(b"Child error\n\0".as_ptr() as *const c_char);
            }

            ksft_cnt.ksft_pass += 6 - (ksft_cnt.ksft_fail - WEXITSTATUS(status));
            ksft_cnt.ksft_fail = WEXITSTATUS(status);

            ksft_print_msg(
                b"Expecting PIDs %d, 42, 1\n\0".as_ptr() as *const c_char,
                pid,
            );
            ksft_print_msg(
                b"Have PIDs in namespaces: %d, %d, %d\n\0".as_ptr() as *const c_char,
                ns3,
                ns2,
                ns1,
            );
            ksft_test_result(
                ns3 == pid && ns2 == 42 && ns1 == 1,
                b"PIDs in all namespaces as expected\n\0".as_ptr() as *const c_char,
            );

            ret = 0;
        }
    }

    if ret != 0 {
        ksft_exit_fail();
    }
    ksft_exit_pass();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
