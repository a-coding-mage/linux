// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/* _GNU_SOURCE was defined in C for CPU_ZERO etc. */
/* C dependencies: errno.h, sched.h, setjmp.h, stdlib.h, sys/wait.h, utils.h, lib.h */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type pid_t = c_int;
pub type bool = core::primitive::bool;

const PARENT_TOKEN: c_char = 0xAAu8 as c_char;
const CHILD_TOKEN: c_char = 0x55u8 as c_char;
const SIGTERM: c_int = 15;
const SIGKILL: c_int = 9;
const SIG_DFL: usize = 0;
const BIND_CPU_ANY: c_int = -1;
const PARANOID_PATH: &[u8] = b"/proc/sys/kernel/perf_event_paranoid\0";

#[repr(C)]
pub union pipe {
    pub fds: [c_int; 2],
    pub read_fd: c_int,
    pub write_fd: c_int,
}

#[repr(C)]
pub struct addr_range {
    pub first: c_ulong,
    pub last: c_ulong,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn getpid() -> pid_t;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn bind_to_cpu(cpu: c_int) -> c_int;
    fn read_long(path: *const c_char, value: *mut c_long, base: c_int) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[no_mangle]
pub unsafe extern "C" fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int {
    let mut c: c_char = PARENT_TOKEN;

    FAIL_IF!(write(write_pipe.write_fd, &c as *const c_char as *const c_void, 1) != 1);
    FAIL_IF!(read(read_pipe.read_fd, &mut c as *mut c_char as *mut c_void, 1) != 1);
    if c != CHILD_TOKEN {
        /* sometimes expected */
        return 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn wait_for_parent(read_pipe: pipe) -> c_int {
    let mut c: c_char = 0;

    FAIL_IF!(read(read_pipe.read_fd, &mut c as *mut c_char as *mut c_void, 1) != 1);
    FAIL_IF!(c != PARENT_TOKEN);

    0
}

#[no_mangle]
pub unsafe extern "C" fn notify_parent(write_pipe: pipe) -> c_int {
    let c: c_char = CHILD_TOKEN;

    FAIL_IF!(write(write_pipe.write_fd, &c as *const c_char as *const c_void, 1) != 1);

    0
}

#[no_mangle]
pub unsafe extern "C" fn notify_parent_of_error(write_pipe: pipe) -> c_int {
    let c: c_char = !CHILD_TOKEN;

    FAIL_IF!(write(write_pipe.write_fd, &c as *const c_char as *const c_void, 1) != 1);

    0
}

#[no_mangle]
pub unsafe extern "C" fn wait_for_child(child_pid: pid_t) -> c_int {
    let mut rc: c_int = 0;

    if waitpid(child_pid, &mut rc, 0) == -1 {
        perror(c"waitpid".as_ptr());
        return 1;
    }

    if WIFEXITED(rc) {
        rc = WEXITSTATUS(rc);
    } else {
        rc = 1; /* Signal or other */
    }

    rc
}

#[no_mangle]
pub unsafe extern "C" fn kill_child_and_wait(child_pid: pid_t) -> c_int {
    kill(child_pid, SIGTERM);

    wait_for_child(child_pid)
}

unsafe extern "C" fn eat_cpu_child(read_pipe: pipe, write_pipe: pipe) -> c_int {
    let mut i: c_int = 0;

    /*
     * We are just here to eat cpu and die. So make sure we can be killed,
     * and also don't do any custom SIGTERM handling.
     */
    signal(SIGTERM, SIG_DFL);

    notify_parent(write_pipe);
    wait_for_parent(read_pipe);

    /* Soak up cpu forever */
    loop {
        core::ptr::write_volatile(&mut i, core::ptr::read_volatile(&i).wrapping_add(1));
    }
}

#[no_mangle]
pub unsafe extern "C" fn eat_cpu(test_function: unsafe extern "C" fn() -> c_int) -> pid_t {
    let mut read_pipe = pipe { fds: [0; 2] };
    let mut write_pipe = pipe { fds: [0; 2] };
    let rc: c_int;
    let pid: pid_t;

    FAIL_IF!(bind_to_cpu(BIND_CPU_ANY) < 0);

    if pipe(read_pipe.fds.as_mut_ptr()) == -1 {
        return -1;
    }

    if pipe(write_pipe.fds.as_mut_ptr()) == -1 {
        return -1;
    }

    pid = fork();
    if pid == 0 {
        exit(eat_cpu_child(write_pipe, read_pipe));
    }

    if sync_with_child(read_pipe, write_pipe) != 0 {
        rc = -1;
    } else {
        printf(c"main test running as pid %d\n".as_ptr(), getpid());

        rc = test_function();
    }

    kill(pid, SIGKILL);

    rc
}

#[no_mangle]
pub static mut libc: addr_range = addr_range { first: 0, last: 0 };
#[no_mangle]
pub static mut vdso: addr_range = addr_range { first: 0, last: 0 };

#[no_mangle]
pub unsafe extern "C" fn parse_proc_maps() -> c_int {
    let mut start: c_ulong = 0;
    let mut end: c_ulong = 0;
    let mut execute: c_char = 0;
    let mut name: [c_char; 128] = [0; 128];
    let f: *mut FILE;
    let mut rc: c_int;

    f = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if f.is_null() {
        perror(c"fopen".as_ptr());
        return -1;
    }

    loop {
        /* This skips line with no executable which is what we want */
        rc = fscanf(
            f,
            c"%lx-%lx %*c%*c%c%*c %*x %*d:%*d %*d %127s\n".as_ptr(),
            &mut start,
            &mut end,
            &mut execute,
            name.as_mut_ptr(),
        );
        if rc <= 0 {
            break;
        }

        if execute != b'x' as c_char {
            continue;
        }

        if !strstr(name.as_ptr(), c"libc".as_ptr()).is_null() {
            libc.first = start;
            libc.last = end - 1;
        } else if !strstr(name.as_ptr(), c"[vdso]".as_ptr()).is_null() {
            vdso.first = start;
            vdso.last = end - 1;
        }
    }

    fclose(f);

    0
}

#[no_mangle]
pub unsafe extern "C" fn require_paranoia_below(level: c_int) -> bool {
    let err: c_int;
    let mut current: c_long = 0;

    err = read_long(PARANOID_PATH.as_ptr() as *const c_char, &mut current, 10);
    if err != 0 {
        printf(c"Couldn't parse /proc/sys/kernel/perf_event_paranoid?\n".as_ptr());
        return false;
    }

    current < level as c_long
}
