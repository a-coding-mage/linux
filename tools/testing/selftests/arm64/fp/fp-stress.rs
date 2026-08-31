// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 ARM Limited.
 */

/* Translated from C. Original include dependencies:
 * errno.h, getopt.h, poll.h, signal.h, stdbool.h, stddef.h, stdio.h,
 * stdlib.h, string.h, unistd.h, sys/auxv.h, sys/epoll.h, sys/prctl.h,
 * sys/types.h, sys/uio.h, sys/wait.h, asm/hwcap.h, and "kselftest.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const MAX_VLS: usize = 16;

const SIGNAL_INTERVAL_MS: c_int = 25;
const LOG_INTERVALS: c_int = 1000 / SIGNAL_INTERVAL_MS;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

const EXIT_FAILURE: c_int = 1;
const _SC_NPROCESSORS_CONF: c_int = 83;
const EINTR: c_int = 4;

const EPOLLIN: u32 = 0x001;
const EPOLLHUP: u32 = 0x010;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CLOEXEC: c_int = 0o2000000;

const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGCHLD: c_int = 17;
const SIGUSR1: c_int = 10;
const SA_RESTART: c_int = 0x10000000;
const SA_SIGINFO: c_int = 4;

const required_argument: c_int = 1;

const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;

/* Constants supplied by Linux arm64 headers. */
const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP2_SME: c_ulong = 1 << 23;
const HWCAP2_SME2: c_ulong = 1 << 37;
const PR_SVE_SET_VL: c_int = 50;
const PR_SME_SET_VL: c_int = 63;
const PR_SVE_VL_LEN_MASK: c_int = 0xffff;
const PR_SVE_VL_INHERIT: c_int = 1 << 17;
const PR_SME_VL_INHERIT: c_int = 1 << 17;
const SVE_VQ_MAX: c_uint = 512;

#[repr(C)]
struct child_data {
    name: *mut c_char,
    output: *mut c_char,
    pid: pid_t,
    stdout: c_int,
    output_seen: bool,
    exited: bool,
    exit_status: c_int,
}

#[repr(C)]
union epoll_data_t {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data_t,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    si_pid: pid_t,
    si_status: c_int,
}

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

static mut epoll_fd: c_int = 0;
static mut children: *mut child_data = null_mut();
static mut evs: *mut epoll_event = null_mut();
static mut tests: c_int = 0;
static mut num_children: c_int = 0;
static mut terminate: bool = false;

static mut startup_pipe: [c_int; 2] = [0; 2];

static mut options: [option; 2] = [
    option {
        name: b"timeout\0".as_ptr() as *const c_char,
        has_arg: required_argument,
        flag: null_mut(),
        val: b't' as c_int,
    },
    option {
        name: null(),
        has_arg: 0,
        flag: null_mut(),
        val: 0,
    },
];

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn __errno_location() -> *mut c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn free(ptr: *mut c_void);
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn sysconf(name: c_int) -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_finished();
    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(pass: bool, fmt: *const c_char, ...);
    fn sve_vq_from_vl(vl: c_int) -> c_uint;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn num_processors() -> c_int {
    let nproc = sysconf(_SC_NPROCESSORS_CONF);
    if nproc < 0 {
        perror(b"Unable to read number of processors\n\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    nproc as c_int
}

unsafe fn child_start(child: *mut child_data, program: *const c_char) {
    let mut ret: c_int;
    let mut pipefd: [c_int; 2] = [0; 2];
    let mut i: c_int;
    let mut ev: epoll_event = zeroed();

    ret = pipe(pipefd.as_mut_ptr());
    if ret != 0 {
        ksft_exit_fail_msg(
            b"Failed to create stdout pipe: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }

    (*child).pid = fork();
    if (*child).pid == -1 {
        ksft_exit_fail_msg(
            b"fork() failed: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }

    if (*child).pid == 0 {
        /*
         * In child, replace stdout with the pipe, errors to
         * stderr from here as kselftest prints to stdout.
         */
        ret = dup2(pipefd[1], 1);
        if ret == -1 {
            printf(b"dup2() %d\n\0".as_ptr() as *const c_char, errno());
            exit(EXIT_FAILURE);
        }

        /*
         * Duplicate the read side of the startup pipe to
         * FD 3 so we can close everything else.
         */
        ret = dup2(startup_pipe[0], 3);
        if ret == -1 {
            printf(b"dup2() %d\n\0".as_ptr() as *const c_char, errno());
            exit(EXIT_FAILURE);
        }

        /*
         * Very dumb mechanism to clean open FDs other than
         * stdio. We don't want O_CLOEXEC for the pipes...
         */
        i = 4;
        while i < 8192 {
            close(i);
            i += 1;
        }

        /*
         * Read from the startup pipe, there should be no data
         * and we should block until it is closed. We just
         * carry-on on error since this isn't super critical.
         */
        ret = read(3, &mut i as *mut c_int as *mut c_void, size_of::<c_int>()) as c_int;
        if ret < 0 {
            printf(
                b"read(startp pipe) failed: %s (%d)\n\0".as_ptr() as *const c_char,
                strerror(errno()),
                errno(),
            );
        }
        if ret > 0 {
            printf(
                b"%d bytes of data on startup pipe\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        close(3);

        ret = execl(program, program, null::<c_char>());
        printf(
            b"execl(%s) failed: %d (%s)\n\0".as_ptr() as *const c_char,
            program,
            errno(),
            strerror(errno()),
        );

        exit(EXIT_FAILURE);
    } else {
        /*
         * In parent, remember the child and close our copy of the
         * write side of stdout.
         */
        close(pipefd[1]);
        (*child).stdout = pipefd[0];
        (*child).output = null_mut();
        (*child).exited = false;
        (*child).output_seen = false;

        ev.events = EPOLLIN | EPOLLHUP;
        ev.data.ptr = child as *mut c_void;

        ret = epoll_ctl(epoll_fd, EPOLL_CTL_ADD, (*child).stdout, &mut ev);
        if ret < 0 {
            ksft_exit_fail_msg(
                b"%s EPOLL_CTL_ADD failed: %s (%d)\n\0".as_ptr() as *const c_char,
                (*child).name,
                strerror(errno()),
                errno(),
            );
        }
    }
}

unsafe fn child_output_read(child: *mut child_data) -> bool {
    let mut read_data: [c_char; 1024] = [0; 1024];
    let mut work: [c_char; 1024] = [0; 1024];
    let mut ret: c_int;
    let len: c_int;
    let mut cur_work: c_int;
    let mut cur_read: c_int;

    ret = read(
        (*child).stdout,
        read_data.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 1024]>(),
    ) as c_int;
    if ret < 0 {
        if errno() == EINTR {
            return true;
        }

        ksft_print_msg(
            b"%s: read() failed: %s (%d)\n\0".as_ptr() as *const c_char,
            (*child).name,
            strerror(errno()),
            errno(),
        );
        return false;
    }
    len = ret;

    (*child).output_seen = true;

    /* Pick up any partial read */
    if !(*child).output.is_null() {
        strncpy(
            work.as_mut_ptr(),
            (*child).output,
            size_of::<[c_char; 1024]>() - 1,
        );
        cur_work = strnlen(work.as_ptr(), size_of::<[c_char; 1024]>()) as c_int;
        free((*child).output as *mut c_void);
        (*child).output = null_mut();
    } else {
        cur_work = 0;
    }

    cur_read = 0;
    while cur_read < len {
        work[cur_work as usize] = read_data[cur_read as usize];
        cur_read += 1;

        if work[cur_work as usize] == b'\n' as c_char {
            work[cur_work as usize] = b'\0' as c_char;
            ksft_print_msg(
                b"%s: %s\n\0".as_ptr() as *const c_char,
                (*child).name,
                work.as_ptr(),
            );
            cur_work = 0;
        } else {
            cur_work += 1;
        }
    }

    if cur_work != 0 {
        work[cur_work as usize] = b'\0' as c_char;
        ret = asprintf(
            &mut (*child).output,
            b"%s\0".as_ptr() as *const c_char,
            work.as_ptr(),
        );
        if ret == -1 {
            ksft_exit_fail_msg(b"Out of memory\n\0".as_ptr() as *const c_char);
        }
    }

    false
}

unsafe fn child_output(child: *mut child_data, events: u32, mut flush: bool) {
    let mut read_more: bool;

    if (events & EPOLLIN) != 0 {
        loop {
            read_more = child_output_read(child);
            if !read_more {
                break;
            }
        }
    }

    if (events & EPOLLHUP) != 0 {
        close((*child).stdout);
        (*child).stdout = -1;
        flush = true;
    }

    if flush && !(*child).output.is_null() {
        ksft_print_msg(
            b"%s: %s<EOF>\n\0".as_ptr() as *const c_char,
            (*child).name,
            (*child).output,
        );
        free((*child).output as *mut c_void);
        (*child).output = null_mut();
    }
}

unsafe fn child_tickle(child: *mut child_data) {
    if (*child).output_seen && !(*child).exited {
        kill((*child).pid, SIGUSR1);
    }
}

unsafe fn child_stop(child: *mut child_data) {
    if !(*child).exited {
        kill((*child).pid, SIGTERM);
    }
}

unsafe fn child_cleanup(child: *mut child_data) {
    let mut ret: pid_t;
    let mut status: c_int = 0;
    let mut fail: bool = false;

    if !(*child).exited {
        loop {
            ret = waitpid((*child).pid, &mut status, 0);
            if ret == -1 && errno() == EINTR {
                continue;
            }

            if ret == -1 {
                ksft_print_msg(
                    b"waitpid(%d) failed: %s (%d)\n\0".as_ptr() as *const c_char,
                    (*child).pid,
                    strerror(errno()),
                    errno(),
                );
                fail = true;
                break;
            }
            if WIFEXITED(status) {
                break;
            }
        }
        (*child).exit_status = WEXITSTATUS(status);
    }

    if !(*child).output_seen {
        ksft_print_msg(
            b"%s no output seen\n\0".as_ptr() as *const c_char,
            (*child).name,
        );
        fail = true;
    }

    if (*child).exit_status != 0 {
        ksft_print_msg(
            b"%s exited with error code %d\n\0".as_ptr() as *const c_char,
            (*child).name,
            (*child).exit_status,
        );
        fail = true;
    }

    ksft_test_result(!fail, b"%s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe extern "C" fn handle_child_signal(_sig: c_int, info: *mut siginfo_t, _context: *mut c_void) {
    let mut i: c_int;
    let mut found: bool = false;

    i = 0;
    while i < num_children {
        let child = children.add(i as usize);
        if (*child).pid == (*info).si_pid {
            (*child).exited = true;
            (*child).exit_status = (*info).si_status;
            found = true;
            break;
        }
        i += 1;
    }

    if !found {
        ksft_print_msg(
            b"SIGCHLD for unknown PID %d with status %d\n\0".as_ptr() as *const c_char,
            (*info).si_pid,
            (*info).si_status,
        );
    }
}

unsafe extern "C" fn handle_exit_signal(_sig: c_int, _info: *mut siginfo_t, _context: *mut c_void) {
    let mut i: c_int;

    /* If we're already exiting then don't signal again */
    if terminate {
        return;
    }

    ksft_print_msg(b"Got signal, exiting...\n\0".as_ptr() as *const c_char);

    terminate = true;

    /*
     * This should be redundant, the main loop should clean up
     * after us, but for safety stop everything we can here.
     */
    i = 0;
    while i < num_children {
        child_stop(children.add(i as usize));
        i += 1;
    }
}

unsafe fn start_fpsimd(child: *mut child_data, cpu: c_int, copy: c_int) {
    let mut ret: c_int;

    ret = asprintf(
        &mut (*child).name,
        b"FPSIMD-%d-%d\0".as_ptr() as *const c_char,
        cpu,
        copy,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    child_start(child, b"./fpsimd-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn start_kernel(child: *mut child_data, cpu: c_int, copy: c_int) {
    let mut ret: c_int;

    ret = asprintf(
        &mut (*child).name,
        b"KERNEL-%d-%d\0".as_ptr() as *const c_char,
        cpu,
        copy,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    child_start(child, b"./kernel-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn start_sve(child: *mut child_data, vl: c_int, cpu: c_int) {
    let mut ret: c_int;

    ret = prctl(PR_SVE_SET_VL, vl | PR_SVE_VL_INHERIT);
    if ret < 0 {
        ksft_exit_fail_msg(b"Failed to set SVE VL %d\n\0".as_ptr() as *const c_char, vl);
    }

    ret = asprintf(
        &mut (*child).name,
        b"SVE-VL-%d-%d\0".as_ptr() as *const c_char,
        vl,
        cpu,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    child_start(child, b"./sve-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn start_ssve(child: *mut child_data, vl: c_int, cpu: c_int) {
    let mut ret: c_int;

    ret = asprintf(
        &mut (*child).name,
        b"SSVE-VL-%d-%d\0".as_ptr() as *const c_char,
        vl,
        cpu,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    ret = prctl(PR_SME_SET_VL, vl | PR_SME_VL_INHERIT);
    if ret < 0 {
        ksft_exit_fail_msg(b"Failed to set SME VL %d\n\0".as_ptr() as *const c_char, ret);
    }

    child_start(child, b"./ssve-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn start_za(child: *mut child_data, vl: c_int, cpu: c_int) {
    let mut ret: c_int;

    ret = prctl(PR_SME_SET_VL, vl | PR_SVE_VL_INHERIT);
    if ret < 0 {
        ksft_exit_fail_msg(b"Failed to set SME VL %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret = asprintf(
        &mut (*child).name,
        b"ZA-VL-%d-%d\0".as_ptr() as *const c_char,
        vl,
        cpu,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    child_start(child, b"./za-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn start_zt(child: *mut child_data, cpu: c_int) {
    let mut ret: c_int;

    ret = asprintf(
        &mut (*child).name,
        b"ZT-%d\0".as_ptr() as *const c_char,
        cpu,
    );
    if ret == -1 {
        ksft_exit_fail_msg(b"asprintf() failed\n\0".as_ptr() as *const c_char);
    }

    child_start(child, b"./zt-test\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Started %s\n\0".as_ptr() as *const c_char, (*child).name);
}

unsafe fn probe_vls(vls: *mut c_int, vl_count: *mut c_int, set_vl: c_int) {
    let mut vq: c_uint;
    let mut vl: c_int;

    *vl_count = 0;

    vq = SVE_VQ_MAX;
    while vq > 0 {
        vl = prctl(set_vl, vq * 16);
        if vl == -1 {
            ksft_exit_fail_msg(
                b"SET_VL failed: %s (%d)\n\0".as_ptr() as *const c_char,
                strerror(errno()),
                errno(),
            );
        }

        vl &= PR_SVE_VL_LEN_MASK;

        if *vl_count != 0 && vl == *vls.add((*vl_count - 1) as usize) {
            break;
        }

        vq = sve_vq_from_vl(vl);

        *vls.add(*vl_count as usize) = vl;
        *vl_count += 1;
        vq /= 2;
    }
}

/* Handle any pending output without blocking */
unsafe fn drain_output(flush: bool) {
    let mut ret: c_int = 1;
    let mut i: c_int;

    while ret > 0 {
        ret = epoll_wait(epoll_fd, evs, tests, 0);
        if ret < 0 {
            if errno() == EINTR {
                continue;
            }
            ksft_print_msg(
                b"epoll_wait() failed: %s (%d)\n\0".as_ptr() as *const c_char,
                strerror(errno()),
                errno(),
            );
        }

        i = 0;
        while i < ret {
            let ev = evs.add(i as usize);
            child_output((*ev).data.ptr as *mut child_data, (*ev).events, flush);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut timeout: c_int = 10 * (1000 / SIGNAL_INTERVAL_MS);
    let mut poll_interval: c_int = 5000;
    let cpus: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut c: c_int;
    let mut sve_vl_count: c_int = 0;
    let mut sme_vl_count: c_int = 0;
    let mut all_children_started: bool = false;
    let mut seen_children: c_int;
    let mut sve_vls: [c_int; MAX_VLS] = [0; MAX_VLS];
    let mut sme_vls: [c_int; MAX_VLS] = [0; MAX_VLS];
    let have_sme2: bool;
    let mut sa: sigaction = zeroed();

    loop {
        c = getopt_long(
            argc,
            argv,
            b"t:\0".as_ptr() as *const c_char,
            options.as_ptr(),
            null_mut(),
        );
        if c == -1 {
            break;
        }
        match c {
            x if x == b't' as c_int => {
                ret = sscanf(
                    optarg,
                    b"%d\0".as_ptr() as *const c_char,
                    &mut timeout,
                );
                if ret != 1 {
                    ksft_exit_fail_msg(
                        b"Failed to parse timeout %s\n\0".as_ptr() as *const c_char,
                        optarg,
                    );
                }
            }
            _ => {
                ksft_exit_fail_msg(b"Unknown argument\n\0".as_ptr() as *const c_char);
            }
        }
    }

    cpus = num_processors();
    tests = 0;

    if (getauxval(AT_HWCAP) & HWCAP_SVE) != 0 {
        probe_vls(sve_vls.as_mut_ptr(), &mut sve_vl_count, PR_SVE_SET_VL);
        tests += sve_vl_count * cpus;
    } else {
        sve_vl_count = 0;
    }

    if (getauxval(AT_HWCAP2) & HWCAP2_SME) != 0 {
        probe_vls(sme_vls.as_mut_ptr(), &mut sme_vl_count, PR_SME_SET_VL);
        tests += sme_vl_count * cpus * 2;
    } else {
        sme_vl_count = 0;
    }

    if (getauxval(AT_HWCAP2) & HWCAP2_SME2) != 0 {
        tests += cpus;
        have_sme2 = true;
    } else {
        have_sme2 = false;
    }

    tests += cpus * 2;

    ksft_print_header();
    ksft_set_plan(tests);

    ksft_print_msg(
        b"%d CPUs, %d SVE VLs, %d SME VLs, SME2 %s\n\0".as_ptr() as *const c_char,
        cpus,
        sve_vl_count,
        sme_vl_count,
        if have_sme2 {
            b"present\0".as_ptr() as *const c_char
        } else {
            b"absent\0".as_ptr() as *const c_char
        },
    );

    if timeout > 0 {
        ksft_print_msg(b"Will run for %d\n\0".as_ptr() as *const c_char, timeout);
    } else {
        ksft_print_msg(b"Will run until terminated\n\0".as_ptr() as *const c_char);
    }

    children = calloc(size_of::<child_data>(), tests as size_t) as *mut child_data;
    if children.is_null() {
        ksft_exit_fail_msg(b"Unable to allocate child data\n\0".as_ptr() as *const c_char);
    }

    ret = epoll_create1(EPOLL_CLOEXEC);
    if ret < 0 {
        ksft_exit_fail_msg(
            b"epoll_create1() failed: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            ret,
        );
    }
    epoll_fd = ret;

    /* Create a pipe which children will block on before execing */
    ret = pipe(startup_pipe.as_mut_ptr());
    if ret != 0 {
        ksft_exit_fail_msg(
            b"Failed to create startup pipe: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }

    /* Get signal handers ready before we start any children */
    sa = zeroed();
    sa.sa_sigaction = Some(handle_exit_signal);
    sa.sa_flags = SA_RESTART | SA_SIGINFO;
    sigemptyset(&mut sa.sa_mask);
    ret = sigaction(SIGINT, &sa, null_mut());
    if ret < 0 {
        ksft_print_msg(
            b"Failed to install SIGINT handler: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }
    ret = sigaction(SIGTERM, &sa, null_mut());
    if ret < 0 {
        ksft_print_msg(
            b"Failed to install SIGTERM handler: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }
    sa.sa_sigaction = Some(handle_child_signal);
    ret = sigaction(SIGCHLD, &sa, null_mut());
    if ret < 0 {
        ksft_print_msg(
            b"Failed to install SIGCHLD handler: %s (%d)\n\0".as_ptr() as *const c_char,
            strerror(errno()),
            errno(),
        );
    }

    evs = calloc(tests as size_t, size_of::<epoll_event>()) as *mut epoll_event;
    if evs.is_null() {
        ksft_exit_fail_msg(
            b"Failed to allocate %d epoll events\n\0".as_ptr() as *const c_char,
            tests,
        );
    }

    i = 0;
    while i < cpus {
        start_fpsimd(children.add(num_children as usize), i, 0);
        num_children += 1;
        start_kernel(children.add(num_children as usize), i, 0);
        num_children += 1;

        j = 0;
        while j < sve_vl_count {
            start_sve(children.add(num_children as usize), sve_vls[j as usize], i);
            num_children += 1;
            j += 1;
        }

        j = 0;
        while j < sme_vl_count {
            start_ssve(children.add(num_children as usize), sme_vls[j as usize], i);
            num_children += 1;
            start_za(children.add(num_children as usize), sme_vls[j as usize], i);
            num_children += 1;
            j += 1;
        }

        if have_sme2 {
            start_zt(children.add(num_children as usize), i);
            num_children += 1;
        }
        i += 1;
    }

    /*
     * All children started, close the startup pipe and let them
     * run.
     */
    close(startup_pipe[0]);
    close(startup_pipe[1]);

    loop {
        /* Did we get a signal asking us to exit? */
        if terminate {
            break;
        }

        /*
         * Timeout is counted in poll intervals with no
         * output, the tests print during startup then are
         * silent when running so this should ensure they all
         * ran enough to install the signal handler, this is
         * especially useful in emulation where we will both
         * be slow and likely to have a large set of VLs.
         */
        ret = epoll_wait(epoll_fd, evs, tests, poll_interval);
        if ret < 0 {
            if errno() == EINTR {
                continue;
            }
            ksft_exit_fail_msg(
                b"epoll_wait() failed: %s (%d)\n\0".as_ptr() as *const c_char,
                strerror(errno()),
                errno(),
            );
        }

        /* Output? */
        if ret > 0 {
            i = 0;
            while i < ret {
                let ev = evs.add(i as usize);
                child_output((*ev).data.ptr as *mut child_data, (*ev).events, false);
                i += 1;
            }
            continue;
        }

        /* Otherwise epoll_wait() timed out */

        /*
         * If the child processes have not produced output they
         * aren't actually running the tests yet .
         */
        if !all_children_started {
            seen_children = 0;

            i = 0;
            while i < num_children {
                let child = children.add(i as usize);
                if (*child).output_seen || (*child).exited {
                    seen_children += 1;
                }
                i += 1;
            }

            if seen_children != num_children {
                ksft_print_msg(
                    b"Waiting for %d children\n\0".as_ptr() as *const c_char,
                    num_children - seen_children,
                );
                continue;
            }

            all_children_started = true;
            poll_interval = SIGNAL_INTERVAL_MS;
        }

        if (timeout % LOG_INTERVALS) == 0 {
            ksft_print_msg(
                b"Sending signals, timeout remaining: %d\n\0".as_ptr() as *const c_char,
                timeout,
            );
        }

        i = 0;
        while i < num_children {
            child_tickle(children.add(i as usize));
            i += 1;
        }

        /* Negative timeout means run indefinitely */
        if timeout < 0 {
            continue;
        }
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }

    ksft_print_msg(b"Finishing up...\n\0".as_ptr() as *const c_char);
    terminate = true;

    i = 0;
    while i < tests {
        child_stop(children.add(i as usize));
        i += 1;
    }

    drain_output(false);

    i = 0;
    while i < tests {
        child_cleanup(children.add(i as usize));
        i += 1;
    }

    drain_output(true);

    ksft_finished();
    0
}
