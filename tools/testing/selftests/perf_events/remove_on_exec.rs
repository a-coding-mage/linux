// SPDX-License-Identifier: GPL-2.0
/*
 * Test for remove_on_exec.
 *
 * Copyright (C) 2021, Google LLC.
 */

/* Original C dependencies:
 * _GNU_SOURCE, sys/types.h, asm/siginfo.h, stdbool.h, stddef.h, stdint.h,
 * stdio.h, linux/perf_event.h, pthread.h, signal.h, sys/ioctl.h,
 * sys/syscall.h, unistd.h, and "kselftest_harness.h".
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

static mut signal_count: c_int = 0;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
const PERF_FLAG_FD_CLOEXEC: u64 = 8;
const PERF_EVENT_IOC_ENABLE: c_ulong = 9216;
const SA_SIGINFO: c_int = 4;
const SA_NODEFER: c_int = 0x40000000;
const SIGTRAP: c_int = 5;
const SIGKILL: c_int = 9;
const STDOUT_FILENO: c_int = 1;
const WNOHANG: c_int = 1;
const TRAP_PERF: c_int = 6;
const __NR_perf_event_open: c_long = 298;

type c_ulong = u64;
type pid_t = c_int;

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    _rest: [u8; 128 - 3 * size_of::<c_int>()],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct perf_event_attr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
}

const PERF_EVENT_ATTR_DISABLED: u64 = 1 << 0;
const PERF_EVENT_ATTR_INHERIT: u64 = 1 << 1;
const PERF_EVENT_ATTR_EXCLUDE_KERNEL: u64 = 1 << 5;
const PERF_EVENT_ATTR_EXCLUDE_HV: u64 = 1 << 6;
const PERF_EVENT_ATTR_REMOVE_ON_EXEC: u64 = 1 << 19;
const PERF_EVENT_ATTR_SIGTRAP: u64 = 1 << 20;

unsafe extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn usleep(usec: u32) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn test_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn signal_count_read() -> c_int {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(signal_count)) }
}

unsafe fn signal_count_write(value: c_int) {
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(signal_count), value) }
}

unsafe fn signal_count_increment() {
    let value = unsafe { signal_count_read() };
    unsafe { signal_count_write(value + 1) };
}

fn make_event_attr() -> perf_event_attr {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_HARDWARE,
        size: size_of::<perf_event_attr>() as u32,
        config: PERF_COUNT_HW_INSTRUCTIONS,
        sample_period: 1000,
        sample_type: 0,
        read_format: 0,
        flags: 0,
    };

    attr.flags |= PERF_EVENT_ATTR_EXCLUDE_KERNEL;
    attr.flags |= PERF_EVENT_ATTR_EXCLUDE_HV;
    attr.flags |= PERF_EVENT_ATTR_DISABLED;
    attr.flags |= PERF_EVENT_ATTR_INHERIT;
    /*
     * Children normally retain their inherited event on exec; with
     * remove_on_exec, we'll remove their event, but the parent and
     * any other non-exec'd children will keep their events.
     */
    attr.flags |= PERF_EVENT_ATTR_REMOVE_ON_EXEC;
    attr.flags |= PERF_EVENT_ATTR_SIGTRAP;

    attr
}

unsafe extern "C" fn sigtrap_handler(
    _signum: c_int,
    info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    unsafe {
        if (*info).si_code != TRAP_PERF {
            fprintf(
                stderr,
                c"%s: unexpected si_code %d\n".as_ptr(),
                c"sigtrap_handler".as_ptr(),
                (*info).si_code,
            );
            return;
        }

        signal_count_increment();
    }
}

struct remove_on_exec {
    oldact: sigaction,
    fd: c_int,
}

unsafe fn remove_on_exec_setup(self_: *mut remove_on_exec) {
    let attr = make_event_attr();
    let mut action: sigaction = unsafe { zeroed() };

    unsafe { signal_count_write(0) };

    /* Initialize sigtrap handler. */
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = Some(sigtrap_handler);
    unsafe { sigemptyset(&mut action.sa_mask) };
    ASSERT_EQ!(
        unsafe { sigaction(SIGTRAP, &action, &mut (*self_).oldact) },
        0
    );

    /* Initialize perf event. */
    unsafe {
        (*self_).fd = syscall(
            __NR_perf_event_open,
            &attr as *const perf_event_attr,
            0,
            -1,
            -1,
            PERF_FLAG_FD_CLOEXEC,
        ) as c_int;
        ASSERT_NE!((*self_).fd, -1);
    }
}

unsafe fn remove_on_exec_teardown(self_: *mut remove_on_exec) {
    unsafe {
        close((*self_).fd);
        sigaction(SIGTRAP, &(*self_).oldact, ptr::null_mut());
    }
}

/* Verify event propagates to fork'd child. */
unsafe fn remove_on_exec_fork_only(self_: *mut remove_on_exec) {
    let mut status: c_int = 0;
    let pid: pid_t = unsafe { fork() };

    if pid == 0 {
        ASSERT_EQ!(unsafe { signal_count_read() }, 0);
        ASSERT_EQ!(unsafe { ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0) }, 0);
        while unsafe { signal_count_read() } == 0 {}
        unsafe { _exit(42) };
    }

    while unsafe { signal_count_read() } == 0 {} /* Child enables event. */
    EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    EXPECT_EQ!(WEXITSTATUS(status), 42);
}

/*
 * Verify that event does _not_ propagate to fork+exec'd child; event enabled
 * after fork+exec.
 */
unsafe fn remove_on_exec_fork_exec_then_enable(self_: *mut remove_on_exec) {
    let pid_exec: pid_t;
    let pid_only_fork: pid_t;
    let mut pipefd: [c_int; 2] = [0; 2];
    let mut tmp: c_int = 0;

    /*
     * Non-exec child, to ensure exec does not affect inherited events of
     * other children.
     */
    pid_only_fork = unsafe { fork() };
    if pid_only_fork == 0 {
        /* Block until parent enables event. */
        while unsafe { signal_count_read() } == 0 {}
        unsafe { _exit(42) };
    }

    ASSERT_NE!(unsafe { pipe(pipefd.as_mut_ptr()) }, -1);
    pid_exec = unsafe { fork() };
    if pid_exec == 0 {
        ASSERT_NE!(unsafe { dup2(pipefd[1], STDOUT_FILENO) }, -1);
        unsafe { close(pipefd[0]) };
        unsafe { execl(c"/proc/self/exe".as_ptr(), c"exec_child".as_ptr(), ptr::null::<c_char>()) };
        unsafe {
            perror(c"exec failed".as_ptr());
            _exit(1);
        }
    }
    unsafe { close(pipefd[1]) };

    ASSERT_EQ!(unsafe { waitpid(pid_exec, &mut tmp, WNOHANG) }, 0); /* Child is running. */
    /* Wait for exec'd child to start spinning. */
    EXPECT_EQ!(
        unsafe {
            read(
                pipefd[0],
                &mut tmp as *mut c_int as *mut c_void,
                size_of::<c_int>(),
            )
        },
        size_of::<c_int>() as isize
    );
    EXPECT_EQ!(tmp, 42);
    unsafe { close(pipefd[0]) };
    /* Now we can enable the event, knowing the child is doing work. */
    EXPECT_EQ!(unsafe { ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0) }, 0);
    /* If the event propagated to the exec'd child, it will exit normally... */
    unsafe { usleep(100000) }; /* ... give time for event to trigger (in case of bug). */
    EXPECT_EQ!(unsafe { waitpid(pid_exec, &mut tmp, WNOHANG) }, 0); /* Should still be running. */
    EXPECT_EQ!(unsafe { kill(pid_exec, SIGKILL) }, 0);

    /* Verify removal from child did not affect this task's event. */
    tmp = unsafe { signal_count_read() };
    while unsafe { signal_count_read() } == tmp {} /* Should not hang! */
    /* Nor should it have affected the first child. */
    EXPECT_EQ!(unsafe { waitpid(pid_only_fork, &mut tmp, 0) }, pid_only_fork);
    EXPECT_EQ!(WEXITSTATUS(tmp), 42);
}

/*
 * Verify that event does _not_ propagate to fork+exec'd child; event enabled
 * before fork+exec.
 */
unsafe fn remove_on_exec_enable_then_fork_exec(self_: *mut remove_on_exec) {
    let pid_exec: pid_t;
    let mut tmp: c_int = 0;

    EXPECT_EQ!(unsafe { ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0) }, 0);

    pid_exec = unsafe { fork() };
    if pid_exec == 0 {
        unsafe { execl(c"/proc/self/exe".as_ptr(), c"exec_child".as_ptr(), ptr::null::<c_char>()) };
        unsafe {
            perror(c"exec failed".as_ptr());
            _exit(1);
        }
    }

    /*
     * The child may exit abnormally at any time if the event propagated and
     * a SIGTRAP is sent before the handler was set up.
     */
    unsafe { usleep(100000) }; /* ... give time for event to trigger (in case of bug). */
    EXPECT_EQ!(unsafe { waitpid(pid_exec, &mut tmp, WNOHANG) }, 0); /* Should still be running. */
    EXPECT_EQ!(unsafe { kill(pid_exec, SIGKILL) }, 0);

    /* Verify removal from child did not affect this task's event. */
    tmp = unsafe { signal_count_read() };
    while unsafe { signal_count_read() } == tmp {} /* Should not hang! */
}

unsafe fn remove_on_exec_exec_stress(self_: *mut remove_on_exec) {
    let mut pids: [pid_t; 30] = [0; 30];
    let mut i: usize;
    let mut tmp: c_int = 0;

    i = 0;
    while i < size_of::<[pid_t; 30]>() / size_of::<pid_t>() {
        pids[i] = unsafe { fork() };
        if pids[i] == 0 {
            unsafe { execl(c"/proc/self/exe".as_ptr(), c"exec_child".as_ptr(), ptr::null::<c_char>()) };
            unsafe {
                perror(c"exec failed".as_ptr());
                _exit(1);
            }
        }

        /* Some forked with event disabled, rest with enabled. */
        if i > 10 {
            EXPECT_EQ!(unsafe { ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0) }, 0);
        }
        i += 1;
    }

    unsafe { usleep(100000) }; /* ... give time for event to trigger (in case of bug). */

    i = 0;
    while i < size_of::<[pid_t; 30]>() / size_of::<pid_t>() {
        /* All children should still be running. */
        EXPECT_EQ!(unsafe { waitpid(pids[i], &mut tmp, WNOHANG) }, 0);
        EXPECT_EQ!(unsafe { kill(pids[i], SIGKILL) }, 0);
        i += 1;
    }

    /* Verify event is still alive. */
    tmp = unsafe { signal_count_read() };
    while unsafe { signal_count_read() } == tmp {}
}

/* For exec'd child. */
unsafe fn exec_child() {
    let mut action: sigaction = unsafe { zeroed() };
    let val: c_int = 42;

    /* Set up sigtrap handler in case we erroneously receive a trap. */
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = Some(sigtrap_handler);
    unsafe { sigemptyset(&mut action.sa_mask) };
    if unsafe { sigaction(SIGTRAP, &action, ptr::null_mut()) } != 0 {
        unsafe {
            perror(c"sigaction failed".as_ptr());
            _exit(1);
        }
    }

    /* Signal parent that we're starting to spin. */
    if unsafe {
        write(
            STDOUT_FILENO,
            &val as *const c_int as *const c_void,
            size_of::<c_int>(),
        )
    } == -1
    {
        unsafe {
            perror(c"write failed".as_ptr());
            _exit(1);
        }
    }

    /* Should hang here until killed. */
    while unsafe { signal_count_read() } == 0 {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        if strcmp(*argv, c"exec_child".as_ptr()) == 0 {
            exec_child();
            return 1;
        }

        test_main(argc, argv)
    }
}
