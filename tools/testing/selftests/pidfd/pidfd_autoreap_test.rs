// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>

// C dependencies: errno.h, linux/types.h, poll.h, pthread.h, sched.h,
// signal.h, stdio.h, stdlib.h, string.h, syscall.h, sys/ioctl.h,
// sys/prctl.h, sys/socket.h, sys/types.h, sys/wait.h, unistd.h,
// "pidfd.h", and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type pid_t = c_int;
type pthread_t = c_ulong;

const CLONE_AUTOREAP: u64 = 1_u64 << 34;
const CLONE_NNP: u64 = 1_u64 << 35;
const CLONE_PIDFD_AUTOKILL: u64 = 1_u64 << 36;
const _LINUX_CAPABILITY_VERSION_3: __u32 = 0x20080522;

// Constants supplied by the translated includes in the original C file.
const CLONE_PIDFD: u64 = 0x0000_1000;
const CLONE_PARENT: u64 = 0x0000_8000;
const CLONE_THREAD: u64 = 0x0001_0000;
const CLONE_SIGHAND: u64 = 0x0000_0800;
const CLONE_VM: u64 = 0x0000_0100;
const ECHILD: c_int = 10;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const SIGCHLD: c_int = 17;
const SIGKILL: c_int = 9;
const WNOHANG: c_int = 1;
const POLLIN: i16 = 0x0001;
const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const PR_SET_CHILD_SUBREAPER: c_int = 36;
const PR_GET_NO_NEW_PRIVS: c_int = 39;
const __NR_capset: c_long = 126;

// ioctl and pidfd constants are provided by "pidfd.h" in the original source.
extern "C" {
    static PIDFD_GET_INFO: c_ulong;
    static PIDFD_INFO_EXIT: __u64;
}

#[repr(C)]
struct cap_header {
    version: __u32,
    pid: c_int,
}

#[repr(C)]
struct cap_data {
    effective: __u32,
    permitted: __u32,
    inheritable: __u32,
}

#[repr(C)]
struct __clone_args {
    flags: __u64,
    pidfd: __u64,
    child_tid: __u64,
    parent_tid: __u64,
    exit_signal: __u64,
    stack: __u64,
    stack_size: __u64,
    tls: __u64,
    set_tid: __u64,
    set_tid_size: __u64,
    cgroup: __u64,
}

#[repr(C)]
struct pidfd_info {
    mask: __u64,
    cgroupid: __u64,
    pid: __u32,
    tgid: __u32,
    ppid: __u32,
    ruid: __u32,
    rgid: __u32,
    euid: __u32,
    egid: __u32,
    suid: __u32,
    sgid: __u32,
    fsuid: __u32,
    fsgid: __u32,
    exit_code: c_int,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn sys_clone3(args: *mut __clone_args, size: usize) -> pid_t;
    fn sys_pidfd_send_signal(pidfd: c_int, sig: c_int, info: *mut c_void, flags: c_uint) -> c_int;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn pause() -> c_int;
    fn _exit(status: c_int) -> !;
    fn prctl(option: c_int, ...) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn atoi(nptr: *const c_char) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_detach(thread: pthread_t) -> c_int;
    fn geteuid() -> c_uint;
    fn write_nointr(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn __errno_location() -> *mut c_int;
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!(($left) >= ($right))
    };
}
macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!(($left) > ($right))
    };
}
macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $body:block) => {
        assert_eq!($left, $right)
    };
}
macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}
macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}
macro_rules! SKIP {
    (return, $msg:expr) => {
        return
    };
}
macro_rules! TH_LOG {
    ($msg:expr) => {};
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

#[inline]
fn ptr_to_u64<T>(ptr: *mut T) -> __u64 {
    ptr as usize as __u64
}

#[inline]
fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

#[inline]
fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn drop_all_caps() -> c_int {
    let mut hdr = cap_header {
        version: _LINUX_CAPABILITY_VERSION_3,
        pid: 0,
    };
    let mut data: [cap_data; 2] = [
        cap_data {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        },
        cap_data {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        },
    ];

    syscall(__NR_capset, &mut hdr as *mut cap_header, data.as_mut_ptr()) as c_int
}

unsafe fn create_autoreap_child(pidfd: *mut c_int) -> pid_t {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_AUTOREAP;
    args.exit_signal = 0;
    args.pidfd = ptr_to_u64(pidfd);

    sys_clone3(&mut args, size_of::<__clone_args>())
}

/*
 * Test that CLONE_AUTOREAP works without CLONE_PIDFD (fire-and-forget).
 */
unsafe fn autoreap_without_pidfd() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_AUTOREAP;
    args.exit_signal = 0;
    let mut pid: pid_t;
    let mut ret: c_int;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        _exit(0);
    }

    /*
     * Give the child a moment to exit and be autoreaped.
     * Then verify no zombie remains.
     */
    usleep(200000);
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);
}

/*
 * Test that CLONE_AUTOREAP with a non-zero exit_signal fails.
 */
unsafe fn autoreap_rejects_exit_signal() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_AUTOREAP;
    args.exit_signal = SIGCHLD as __u64;
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * Test that CLONE_AUTOREAP with CLONE_PARENT fails.
 */
unsafe fn autoreap_rejects_parent() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_AUTOREAP | CLONE_PARENT;
    args.exit_signal = 0;
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * Test that CLONE_AUTOREAP with CLONE_THREAD fails.
 */
unsafe fn autoreap_rejects_thread() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_AUTOREAP | CLONE_THREAD | CLONE_SIGHAND | CLONE_VM;
    args.exit_signal = 0;
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * Basic test: create an autoreap child, let it exit, verify:
 * - pidfd becomes readable (poll returns POLLIN)
 * - PIDFD_GET_INFO returns the correct exit code
 * - waitpid() returns -1/ECHILD (no zombie)
 */
unsafe fn autoreap_basic() {
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    pid = create_autoreap_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        _exit(42);
    }

    ASSERT_GE!(pidfd, 0);

    /* Wait for the child to exit via pidfd poll. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /* Verify exit info via PIDFD_GET_INFO. */
    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    /*
     * exit_code is in waitpid format: for _exit(42),
     * WIFEXITED is true and WEXITSTATUS is 42.
     */
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 42);

    /* Verify no zombie: waitpid should fail with ECHILD. */
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(pidfd);
}

/*
 * Test that an autoreap child killed by a signal reports
 * the correct exit info.
 */
unsafe fn autoreap_signaled() {
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    pid = create_autoreap_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        pause();
        _exit(1);
    }

    ASSERT_GE!(pidfd, 0);

    /* Kill the child. */
    ret = sys_pidfd_send_signal(pidfd, SIGKILL, ptr::null_mut(), 0);
    ASSERT_EQ!(ret, 0);

    /* Wait for exit via pidfd. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /* Verify signal info. */
    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFSIGNALED(info.exit_code));
    ASSERT_EQ!(WTERMSIG(info.exit_code), SIGKILL);

    /* No zombie. */
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(pidfd);
}

/*
 * Test autoreap survives reparenting: middle process creates an
 * autoreap grandchild, then exits. The grandchild gets reparented
 * to us (the grandparent, which is a subreaper). When the grandchild
 * exits, it should still be autoreaped - no zombie under us.
 */
unsafe fn autoreap_reparent() {
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut pidfd: c_int = -1;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut mid_pid: pid_t;
    let mut grandchild_pid: pid_t;
    let mut buf: [c_char; 32] = [0; 32];

    /* Make ourselves a subreaper so reparented children come to us. */
    ret = prctl(PR_SET_CHILD_SUBREAPER, 1);
    ASSERT_EQ!(ret, 0);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    mid_pid = fork();
    ASSERT_GE!(mid_pid, 0);

    if mid_pid == 0 {
        /* Middle child: create an autoreap grandchild. */
        let mut gc_pidfd: c_int = -1;

        close(ipc_sockets[0]);

        grandchild_pid = create_autoreap_child(&mut gc_pidfd);
        if grandchild_pid < 0 {
            write_nointr(ipc_sockets[1], b"E\0".as_ptr() as *const c_void, 1);
            close(ipc_sockets[1]);
            _exit(1);
        }

        if grandchild_pid == 0 {
            /* Grandchild: wait for signal to exit. */
            close(ipc_sockets[1]);
            if gc_pidfd >= 0 {
                close(gc_pidfd);
            }
            pause();
            _exit(0);
        }

        /* Send grandchild PID to grandparent. */
        snprintf(buf.as_mut_ptr(), buf.len(), b"%d\0".as_ptr() as *const c_char, grandchild_pid);
        write_nointr(
            ipc_sockets[1],
            buf.as_ptr() as *const c_void,
            strlen(buf.as_ptr()),
        );
        close(ipc_sockets[1]);
        if gc_pidfd >= 0 {
            close(gc_pidfd);
        }

        /* Middle child exits, grandchild gets reparented. */
        _exit(0);
    }

    close(ipc_sockets[1]);

    /* Read grandchild's PID. */
    ret = read_nointr(ipc_sockets[0], buf.as_mut_ptr() as *mut c_void, buf.len() - 1) as c_int;
    close(ipc_sockets[0]);
    ASSERT_GT!(ret, 0);

    if buf[0] == b'E' as c_char {
        waitpid(mid_pid, ptr::null_mut(), 0);
        prctl(PR_SET_CHILD_SUBREAPER, 0);
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }

    grandchild_pid = atoi(buf.as_ptr());
    ASSERT_GT!(grandchild_pid, 0);

    /* Wait for the middle child to exit. */
    ret = waitpid(mid_pid, ptr::null_mut(), 0);
    ASSERT_EQ!(ret, mid_pid);

    /*
     * Now the grandchild is reparented to us (subreaper).
     * Open a pidfd for the grandchild and kill it.
     */
    pidfd = sys_pidfd_open(grandchild_pid, 0);
    ASSERT_GE!(pidfd, 0);

    ret = sys_pidfd_send_signal(pidfd, SIGKILL, ptr::null_mut(), 0);
    ASSERT_EQ!(ret, 0);

    /* Wait for it to exit via pidfd poll. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /*
     * The grandchild should have been autoreaped even though
     * we (the new parent) haven't set SA_NOCLDWAIT.
     * waitpid should return -1/ECHILD.
     */
    ret = waitpid(grandchild_pid, ptr::null_mut(), WNOHANG);
    EXPECT_EQ!(ret, -1);
    EXPECT_EQ!(errno(), ECHILD);

    close(pidfd);

    /* Clean up subreaper status. */
    prctl(PR_SET_CHILD_SUBREAPER, 0);
}

static mut thread_sock_fd: c_int = 0;

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        /* Signal parent we're running. */
        write_nointr(thread_sock_fd, b"1\0".as_ptr() as *const c_void, 1);

        /* Give main thread time to call _exit() first. */
        usleep(200000);
    }

    ptr::null_mut()
}

/*
 * Test that an autoreap child with multiple threads is properly
 * autoreaped only after all threads have exited.
 */
unsafe fn autoreap_multithreaded() {
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut pidfd: c_int = -1;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;
    let mut c: c_char = 0;

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid = create_autoreap_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        close(ipc_sockets[0]);
        close(ipc_sockets[1]);
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut thread: pthread_t = 0;

        close(ipc_sockets[0]);

        /*
         * Create a sub-thread that outlives the main thread.
         * The thread signals readiness, then sleeps.
         * The main thread waits briefly, then calls _exit().
         */
        thread_sock_fd = ipc_sockets[1];
        pthread_create(&mut thread, ptr::null(), thread_func, ptr::null_mut());
        pthread_detach(thread);

        /* Wait for thread to be running. */
        usleep(100000);

        /* Main thread exits; sub-thread is still alive. */
        _exit(99);
    }

    close(ipc_sockets[1]);

    /* Wait for the sub-thread to signal readiness. */
    ret = read_nointr(ipc_sockets[0], &mut c as *mut c_char as *mut c_void, 1) as c_int;
    close(ipc_sockets[0]);
    ASSERT_EQ!(ret, 1);

    /* Wait for the process to fully exit via pidfd poll. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /* Verify exit info. */
    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 99);

    /* No zombie. */
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(pidfd);
}

/*
 * Test that autoreap is NOT inherited by grandchildren.
 */
unsafe fn autoreap_no_inherit() {
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut pidfd: c_int = -1;
    let mut pid: pid_t;
    let mut buf: [c_char; 2] = [0; 2];
    let mut pfd: pollfd = core::mem::zeroed();

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid = create_autoreap_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        close(ipc_sockets[0]);
        close(ipc_sockets[1]);
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut gc: pid_t;
        let mut status: c_int = 0;

        close(ipc_sockets[0]);

        /* Autoreap child forks a grandchild (without autoreap). */
        gc = fork();
        if gc < 0 {
            write_nointr(ipc_sockets[1], b"E\0".as_ptr() as *const c_void, 1);
            _exit(1);
        }
        if gc == 0 {
            /* Grandchild: exit immediately. */
            close(ipc_sockets[1]);
            _exit(77);
        }

        /*
         * The grandchild should become a regular zombie
         * since it was NOT created with CLONE_AUTOREAP.
         * Wait for it to verify.
         */
        ret = waitpid(gc, &mut status, 0);
        if ret == gc && WIFEXITED(status) && WEXITSTATUS(status) == 77 {
            write_nointr(ipc_sockets[1], b"P\0".as_ptr() as *const c_void, 1);
        } else {
            write_nointr(ipc_sockets[1], b"F\0".as_ptr() as *const c_void, 1);
        }
        close(ipc_sockets[1]);
        _exit(0);
    }

    close(ipc_sockets[1]);

    ret = read_nointr(ipc_sockets[0], buf.as_mut_ptr() as *mut c_void, 1) as c_int;
    close(ipc_sockets[0]);
    ASSERT_EQ!(ret, 1);

    /*
     * 'P' means the autoreap child was able to waitpid() its
     * grandchild (correct - grandchild should be a normal zombie,
     * not autoreaped).
     */
    ASSERT_EQ!(buf[0], b'P' as c_char);

    /* Wait for the autoreap child to exit. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);

    /* Autoreap child itself should be autoreaped. */
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(pidfd);
}

/*
 * Test that CLONE_NNP sets no_new_privs on the child.
 * The child checks via prctl(PR_GET_NO_NEW_PRIVS) and reports back.
 * The parent must NOT have no_new_privs set afterwards.
 */
unsafe fn nnp_sets_no_new_privs() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_AUTOREAP | CLONE_NNP;
    args.exit_signal = 0;
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    /* Ensure parent does not already have no_new_privs. */
    ret = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0);
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("Parent already has no_new_privs set, cannot run test");
    });

    args.pidfd = ptr_to_u64(&mut pidfd);

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_NNP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /*
         * Child: check no_new_privs. Exit 0 if set, 1 if not.
         */
        ret = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0);
        _exit(if ret == 1 { 0 } else { 1 });
    }

    ASSERT_GE!(pidfd, 0);

    /* Parent must still NOT have no_new_privs. */
    ret = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0);
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("Parent got no_new_privs after creating CLONE_NNP child");
    });

    /* Wait for child to exit. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);

    /* Verify child exited with 0 (no_new_privs was set). */
    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0, {
        TH_LOG!("Child did not have no_new_privs set");
    });

    close(pidfd);
}

/*
 * Test that CLONE_NNP with CLONE_THREAD fails with EINVAL.
 */
unsafe fn nnp_rejects_thread() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_NNP | CLONE_THREAD | CLONE_SIGHAND | CLONE_VM;
    args.exit_signal = 0;
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * Test that a plain CLONE_AUTOREAP child does NOT get no_new_privs.
 * Only CLONE_NNP should set it.
 */
unsafe fn autoreap_no_new_privs_unset() {
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    pid = create_autoreap_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_AUTOREAP not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /*
         * Child: check no_new_privs. Exit 0 if NOT set, 1 if set.
         */
        ret = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0);
        _exit(if ret == 0 { 0 } else { 1 });
    }

    ASSERT_GE!(pidfd, 0);

    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);

    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0, {
        TH_LOG!("Plain autoreap child unexpectedly has no_new_privs");
    });

    close(pidfd);
}

/*
 * Helper: create a child with CLONE_PIDFD | CLONE_PIDFD_AUTOKILL | CLONE_AUTOREAP | CLONE_NNP.
 */
unsafe fn create_autokill_child(pidfd: *mut c_int) -> pid_t {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_PIDFD_AUTOKILL | CLONE_AUTOREAP | CLONE_NNP;
    args.exit_signal = 0;
    args.pidfd = ptr_to_u64(pidfd);

    sys_clone3(&mut args, size_of::<__clone_args>())
}

/*
 * Basic autokill test: child blocks in pause(), parent closes the
 * clone3 pidfd, child should be killed and autoreaped.
 */
unsafe fn autokill_basic() {
    let mut pidfd: c_int = -1;
    let mut pollfd_fd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    pid = create_autokill_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_PIDFD_AUTOKILL not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        pause();
        _exit(1);
    }

    ASSERT_GE!(pidfd, 0);

    /*
     * Open a second pidfd via pidfd_open() so we can observe the
     * child's death after closing the clone3 pidfd.
     */
    pollfd_fd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pollfd_fd, 0);

    /* Close the clone3 pidfd — this should trigger autokill. */
    close(pidfd);

    /* Wait for the child to die via the pidfd_open'd fd. */
    pfd.fd = pollfd_fd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /* Child should be autoreaped — no zombie. */
    usleep(100000);
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(pollfd_fd);
}

/*
 * CLONE_PIDFD_AUTOKILL without CLONE_PIDFD must fail with EINVAL.
 */
unsafe fn autokill_requires_pidfd() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD_AUTOKILL | CLONE_AUTOREAP;
    args.exit_signal = 0;
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * CLONE_PIDFD_AUTOKILL without CLONE_AUTOREAP must fail with EINVAL.
 */
unsafe fn autokill_requires_autoreap() {
    let mut pidfd: c_int = -1;
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_PIDFD_AUTOKILL;
    args.exit_signal = 0;
    args.pidfd = ptr_to_u64(&mut pidfd);
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * CLONE_PIDFD_AUTOKILL with CLONE_THREAD must fail with EINVAL.
 */
unsafe fn autokill_rejects_thread() {
    let mut pidfd: c_int = -1;
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD
        | CLONE_PIDFD_AUTOKILL
        | CLONE_AUTOREAP
        | CLONE_THREAD
        | CLONE_SIGHAND
        | CLONE_VM;
    args.exit_signal = 0;
    args.pidfd = ptr_to_u64(&mut pidfd);
    let mut pid: pid_t;

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EINVAL);
}

/*
 * Test that only the clone3 pidfd triggers autokill, not pidfd_open().
 * Close the pidfd_open'd fd first — child should survive.
 * Then close the clone3 pidfd — child should be killed and autoreaped.
 */
unsafe fn autokill_pidfd_open_no_effect() {
    let mut pidfd: c_int = -1;
    let mut open_fd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    pid = create_autokill_child(&mut pidfd);
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_PIDFD_AUTOKILL not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        pause();
        _exit(1);
    }

    ASSERT_GE!(pidfd, 0);

    /* Open a second pidfd via pidfd_open(). */
    open_fd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(open_fd, 0);

    /*
     * Close the pidfd_open'd fd — child should survive because
     * only the clone3 pidfd has autokill.
     */
    close(open_fd);
    usleep(200000);

    /* Verify child is still alive by polling the clone3 pidfd. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 0);
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("Child died after closing pidfd_open fd — should still be alive");
    });

    /* Open another observation fd before triggering autokill. */
    open_fd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(open_fd, 0);

    /* Now close the clone3 pidfd — this triggers autokill. */
    close(pidfd);

    pfd.fd = open_fd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);
    ASSERT_TRUE!((pfd.revents & POLLIN) != 0);

    /* Child should be autoreaped — no zombie. */
    usleep(100000);
    ret = waitpid(pid, ptr::null_mut(), WNOHANG);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno(), ECHILD);

    close(open_fd);
}

/*
 * Test that CLONE_PIDFD_AUTOKILL without CLONE_NNP fails with EPERM
 * for an unprivileged caller.
 */
unsafe fn autokill_requires_cap_sys_admin() {
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_PIDFD_AUTOKILL | CLONE_AUTOREAP;
    args.exit_signal = 0;
    args.pidfd = ptr_to_u64(&mut pidfd);
    let mut pid: pid_t;

    /* Drop all capabilities so we lack CAP_SYS_ADMIN. */
    ret = drop_all_caps();
    ASSERT_EQ!(ret, 0);

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    ASSERT_EQ!(pid, -1);
    ASSERT_EQ!(errno(), EPERM);
}

/*
 * Test that CLONE_PIDFD_AUTOKILL without CLONE_NNP succeeds with
 * CAP_SYS_ADMIN.
 */
unsafe fn autokill_without_nnp_with_cap() {
    let mut args: __clone_args = core::mem::zeroed();
    args.flags = CLONE_PIDFD | CLONE_PIDFD_AUTOKILL | CLONE_AUTOREAP;
    args.exit_signal = 0;
    let mut info: pidfd_info = core::mem::zeroed();
    info.mask = PIDFD_INFO_EXIT;
    let mut pidfd: c_int = -1;
    let mut ret: c_int;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pid: pid_t;

    if geteuid() != 0 {
        SKIP!(return, "Need root/CAP_SYS_ADMIN");
    }

    args.pidfd = ptr_to_u64(&mut pidfd);

    pid = sys_clone3(&mut args, size_of::<__clone_args>());
    if pid < 0 && errno() == EINVAL {
        SKIP!(return, "CLONE_PIDFD_AUTOKILL not supported");
    }
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        _exit(0);
    }

    ASSERT_GE!(pidfd, 0);

    /* Wait for child to exit. */
    pfd.fd = pidfd;
    pfd.events = POLLIN;
    ret = poll(&mut pfd, 1, 5000);
    ASSERT_EQ!(ret, 1);

    ret = ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info);
    ASSERT_EQ!(ret, 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0);

    close(pidfd);
}

// TEST_HARNESS_MAIN
