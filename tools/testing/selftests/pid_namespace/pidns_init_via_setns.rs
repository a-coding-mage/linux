// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included:
// <fcntl.h>, <sched.h>, <stdio.h>, <sys/types.h>, <unistd.h>
// "kselftest_harness.h", "../pidfd/pidfd.h"

use libc::{
    c_char, c_int, c_long, c_void, close, fopen, fork, fscanf, free, getpid, geteuid, open, pipe,
    setns, snprintf, sscanf, strncmp, strrchr, unshare, FILE, CLONE_NEWPID, CLONE_NEWUSER,
    O_RDONLY, SIGCHLD,
};
use std::mem;
use std::ptr;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __clone_args {
    pub flags: u64,
    pub pidfd: u64,
    pub child_tid: u64,
    pub parent_tid: u64,
    pub exit_signal: u64,
    pub stack: u64,
    pub stack_size: u64,
    pub tls: u64,
    pub set_tid: u64,
    pub set_tid_size: u64,
    pub cgroup: u64,
}

extern "C" {
    fn read_nointr(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write_nointr(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn wait_for_pid(pid: libc::pid_t) -> c_int;
    fn sys_clone3(args: *mut __clone_args, size: usize) -> libc::pid_t;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize;
}

unsafe fn ptr_to_u64<T>(ptr: *mut T) -> u64 {
    ptr as u64
}

unsafe fn _exit(status: c_int) -> ! {
    libc::_exit(status)
}

pub unsafe fn pidns_init_via_setns(_metadata: *mut __test_metadata) {
    let mut child: libc::pid_t;
    let parent_pid: libc::pid_t;
    let mut pipe_fd: [c_int; 2] = [0; 2];
    let mut buf: c_char = 0;

    if geteuid() != 0 {
        assert_eq!(0, unshare(CLONE_NEWUSER));
    }

    parent_pid = getpid();

    assert_eq!(0, pipe(pipe_fd.as_mut_ptr()));

    child = fork();
    assert!(child >= 0);

    if child == 0 {
        let mut path: [c_char; 256] = [0; 256];
        let nsfd: c_int;
        let mut grandchild: libc::pid_t;

        close(pipe_fd[1]);

        /* Wait for parent to complete unshare */
        assert_eq!(
            1,
            read_nointr(pipe_fd[0], &mut buf as *mut c_char as *mut c_void, 1)
        );
        close(pipe_fd[0]);

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"/proc/%d/ns/pid_for_children\0".as_ptr() as *const c_char,
            parent_pid,
        );
        nsfd = open(path.as_ptr(), O_RDONLY);
        assert!(nsfd >= 0);

        assert_eq!(0, setns(nsfd, CLONE_NEWPID));
        close(nsfd);

        grandchild = fork();
        assert!(grandchild >= 0);

        if grandchild == 0 {
            /* Should be init (PID 1) in the new namespace */
            if getpid() != 1 {
                _exit(1);
            }
            _exit(0);
        }

        assert_eq!(0, wait_for_pid(grandchild));
        _exit(0);
    }

    close(pipe_fd[0]);

    assert_eq!(0, unshare(CLONE_NEWPID));

    /* Signal child that the new PID namespace is ready */
    buf = 0;
    assert_eq!(
        1,
        write_nointr(pipe_fd[1], &buf as *const c_char as *const c_void, 1)
    );
    close(pipe_fd[1]);

    assert_eq!(0, wait_for_pid(child));
}

/*
 * Similar to pidns_init_via_setns, but:
 *  1. Parent enters a new PID namespace right from the start to be able to
 *     later freely use pid 1001 in it.
 *  2. After forking child, parent also calls unshare(CLONE_NEWUSER)
 *     before unshare(CLONE_NEWPID) so that new old and new pid namespaces have
 *     different user namespace owners.
 *  3. Child uses clone3() with set_tid={1, 1001} instead of fork() and
 *     grandchild checks that it gets desired pids .
 *
 * Flow:
 *  1. Test process creates a new PID namespace and forks a wrapper
 *     (PID 1 in the outer namespace).
 *  2. Wrapper forks a child.
 *  3. Wrapper calls unshare(CLONE_NEWUSER) + unshare(CLONE_NEWPID)
 *     to create an inner PID namespace.
 *  4. Wrapper signals the child via pipe.
 *  5. Child opens wrapper's /proc/<pid>/ns/pid_for_children and calls
 *     setns(fd, CLONE_NEWPID) to join the inner namespace.
 *  6. Child calls clone3() with set_tid={1, 1001}.
 *  7. Grandchild verifies its NSpid ends with "1001 1".
 */

static mut set_tid: [libc::pid_t; 2] = [1, 1001];

unsafe fn pidns_init_via_setns_set_tid_grandchild(
    _metadata: *mut __test_metadata,
) -> c_int {
    let mut line: *mut c_char = ptr::null_mut();
    let mut len: usize = 0;
    let mut found: bool = false;
    let gf: *mut FILE;

    gf = fopen(b"/proc/self/status\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    assert_ne!(gf, ptr::null_mut());

    while getline(&mut line, &mut len, gf) != -1 {
        if strncmp(line, b"NSpid:\0".as_ptr() as *const c_char, 6) != 0 {
            continue;
        }

        for i in 0..2 {
            let last: *mut c_char = strrchr(line, b'\t' as c_int);
            let mut pid: libc::pid_t = 0;

            assert_ne!(last, ptr::null_mut());
            assert_eq!(
                sscanf(last, b"%d\0".as_ptr() as *const c_char, &mut pid),
                1
            );
            assert_eq!(pid, set_tid[i]);
            *last = b'\0' as c_char;
        }

        found = true;
        break;
    }

    free(line as *mut c_void);
    libc::fclose(gf);
    assert!(found);
    return 0;
}

unsafe fn pidns_init_via_setns_set_tid_child(
    _metadata: *mut __test_metadata,
    parent_pid: libc::pid_t,
    pipe_fd: *mut c_int,
) -> c_int {
    let mut args: __clone_args = mem::zeroed();
    args.exit_signal = SIGCHLD as u64;
    args.set_tid = ptr_to_u64(set_tid.as_mut_ptr());
    args.set_tid_size = 2;

    let mut grandchild: libc::pid_t;
    let mut path: [c_char; 256] = [0; 256];
    let mut buf: c_char = 0;
    let nsfd: c_int;

    close(*pipe_fd.add(1));

    assert_eq!(
        1,
        read_nointr(*pipe_fd.add(0), &mut buf as *mut c_char as *mut c_void, 1)
    );
    close(*pipe_fd.add(0));

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"/proc/%d/ns/pid_for_children\0".as_ptr() as *const c_char,
        parent_pid,
    );
    nsfd = open(path.as_ptr(), O_RDONLY);
    assert!(nsfd >= 0);

    assert_eq!(0, setns(nsfd, CLONE_NEWPID));
    close(nsfd);

    grandchild = sys_clone3(&mut args, mem::size_of_val(&args));
    assert!(grandchild >= 0);

    if grandchild == 0 {
        _exit(pidns_init_via_setns_set_tid_grandchild(_metadata));
    }

    assert_eq!(0, wait_for_pid(grandchild));
    return 0;
}

unsafe fn pidns_init_via_setns_set_tid_wrapper(
    _metadata: *mut __test_metadata,
) -> c_int {
    let mut pipe_fd: [c_int; 2] = [0; 2];
    let mut child: libc::pid_t;
    let mut parent_pid: libc::pid_t = 0;
    let mut buf: c_char = 0;
    let f: *mut FILE;

    /*
     * We are PID 1 inside the new namespace, but /proc is
     * mounted from the host.  Read our host-visible PID so
     * the child can reach our pid_for_children via /proc.
     */
    f = fopen(b"/proc/self/stat\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    assert_ne!(f, ptr::null_mut());
    assert_eq!(
        fscanf(f, b"%d\0".as_ptr() as *const c_char, &mut parent_pid),
        1
    );
    assert_eq!(0, pipe(pipe_fd.as_mut_ptr()));

    child = fork();
    assert!(child >= 0);

    if child == 0 {
        _exit(pidns_init_via_setns_set_tid_child(
            _metadata,
            parent_pid,
            pipe_fd.as_mut_ptr(),
        ));
    }

    close(pipe_fd[0]);

    assert_eq!(0, unshare(CLONE_NEWUSER));
    assert_eq!(0, unshare(CLONE_NEWPID));

    buf = 0;
    assert_eq!(
        1,
        write_nointr(pipe_fd[1], &buf as *const c_char as *const c_void, 1)
    );
    close(pipe_fd[1]);

    assert_eq!(0, wait_for_pid(child));

    libc::fclose(f);
    return 0;
}

pub unsafe fn pidns_init_via_setns_set_tid(_metadata: *mut __test_metadata) {
    let mut wrapper: libc::pid_t;

    if geteuid() != 0 {
        /* SKIP(return, "This test needs root to run!"); */
        return;
    }

    assert_eq!(0, unshare(CLONE_NEWPID));

    wrapper = fork();
    assert!(wrapper >= 0);

    if wrapper == 0 {
        _exit(pidns_init_via_setns_set_tid_wrapper(_metadata));
    }

    assert_eq!(0, wait_for_pid(wrapper));
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
