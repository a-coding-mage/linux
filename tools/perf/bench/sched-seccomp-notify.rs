// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/bench/sched-seccomp-notify.c.
// C dependencies removed from executable Rust:
// <subcmd/parse-options.h>, "bench.h", Linux/seccomp/filter/prctl/ioctl
// headers, and libc/POSIX headers provide the external declarations,
// constants, macros, and layouts referenced below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u64 = u64;
type pid_t = c_int;
type uint64_t = u64;

const LOOPS_DEFAULT: uint64_t = 1000000_u64;
static mut loops: uint64_t = LOOPS_DEFAULT;
static mut sync_mode: bool = false;

#[repr(C)]
struct option {
    _private: [u8; 0],
}

extern "C" {
    static mut bench_format: c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;

    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct seccomp_data {
    nr: c_int,
    arch: u32,
    instruction_pointer: u64,
    args: [u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct seccomp_notif {
    id: u64,
    pid: u32,
    flags: u32,
    data: seccomp_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct seccomp_notif_resp {
    id: u64,
    val: i64,
    error: i32,
    flags: u32,
}

const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;

const SECCOMP_SET_MODE_FILTER: c_uint = 1;
const SECCOMP_FILTER_FLAG_NEW_LISTENER: c_uint = 1 << 3;
const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc00000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;

const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_SET_PDEATHSIG: c_int = 1;
const SIGKILL: c_int = 9;
const EXIT_FAILURE: c_int = 1;
const INT_MAX: c_int = 2147483647;
const USER_NOTIF_MAGIC: c_int = INT_MAX;

const __NR_gettid: c_long = 186;
const __NR_seccomp: c_long = 317;

const USEC_PER_SEC: c_ulong = 1000000;
const USEC_PER_MSEC: c_ulong = 1000;

const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;

// Fallback definitions from the C source's #ifndef
// SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP block.
const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: c_ulong = 1_u64.wrapping_shl(0) as c_ulong;
// SECCOMP_IOCTL_NOTIF_SET_FLAGS is SECCOMP_IOW(4, __u64) in C headers.
const SECCOMP_IOCTL_NOTIF_SET_FLAGS: c_ulong = 0;
const SECCOMP_IOCTL_NOTIF_RECV: c_ulong = 0;
const SECCOMP_IOCTL_NOTIF_SEND: c_ulong = 0;

// C initializer macros OPT_U64, OPT_BOOLEAN, and OPT_END are supplied by
// <subcmd/parse-options.h>. They cannot be expanded from this isolated file.
static options: [option; 1] = [option { _private: [] }];

static BENCH_SECCOMP_USAGE_0: &[u8] = b"perf bench sched secccomp-notify <options>\0";
static bench_seccomp_usage: [*const c_char; 2] = [
    BENCH_SECCOMP_USAGE_0.as_ptr() as *const c_char,
    ptr::null(),
];

#[inline]
const fn BPF_STMT(code: u16, k: u32) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

#[inline]
const fn BPF_JUMP(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

#[inline]
const fn offsetof_seccomp_data_nr() -> u32 {
    0
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) >> 1 > 0
}

unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += USEC_PER_SEC as c_long;
    }
}

unsafe fn seccomp(op: c_uint, flags: c_uint, args: *mut c_void) -> c_int {
    syscall(__NR_seccomp, op, flags, args) as c_int
}

unsafe fn user_notif_syscall(nr: c_int, flags: c_uint) -> c_int {
    let mut filter = [
        BPF_STMT(
            BPF_LD | BPF_W | BPF_ABS,
            offsetof_seccomp_data_nr(),
        ),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, nr as u32, 0, 1),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_USER_NOTIF),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];

    let mut prog = sock_fprog {
        len: (filter.len()) as u16,
        filter: filter.as_mut_ptr(),
    };

    seccomp(
        SECCOMP_SET_MODE_FILTER,
        flags,
        &mut prog as *mut sock_fprog as *mut c_void,
    )
}

unsafe fn user_notification_sync_loop(listener: c_int) {
    let mut resp: seccomp_notif_resp;
    let mut req: seccomp_notif;
    let mut nr: uint64_t;

    nr = 0;
    while nr < loops {
        req = zeroed();
        memset(
            &mut req as *mut seccomp_notif as *mut c_void,
            0,
            size_of::<seccomp_notif>(),
        );
        if ioctl(
            listener,
            SECCOMP_IOCTL_NOTIF_RECV,
            &mut req as *mut seccomp_notif,
        ) != 0
        {
            err(
                EXIT_FAILURE,
                b"SECCOMP_IOCTL_NOTIF_RECV failed\0".as_ptr() as *const c_char,
            );
        }

        if req.data.nr as c_long != __NR_gettid {
            errx(
                EXIT_FAILURE,
                b"unexpected syscall: %d\0".as_ptr() as *const c_char,
                req.data.nr,
            );
        }

        resp = zeroed();
        resp.id = req.id;
        resp.error = 0;
        resp.val = USER_NOTIF_MAGIC as i64;
        resp.flags = 0;
        if ioctl(
            listener,
            SECCOMP_IOCTL_NOTIF_SEND,
            &mut resp as *mut seccomp_notif_resp,
        ) != 0
        {
            err(
                EXIT_FAILURE,
                b"SECCOMP_IOCTL_NOTIF_SEND failed\0".as_ptr() as *const c_char,
            );
        }
        nr += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn bench_sched_seccomp_notify(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut start: timeval = zeroed();
    let mut stop: timeval = zeroed();
    let mut diff: timeval = zeroed();
    let mut result_usec: c_ulong = 0;
    let mut status: c_int = 0;
    let listener: c_int;
    let pid: pid_t;
    let mut ret: c_long;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_seccomp_usage.as_ptr(),
        0,
    );
    let _ = argc;

    gettimeofday(&mut start, ptr::null_mut());

    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
    listener = user_notif_syscall(__NR_gettid as c_int, SECCOMP_FILTER_FLAG_NEW_LISTENER);
    if listener < 0 {
        err(
            EXIT_FAILURE,
            b"can't create a notification descriptor\0".as_ptr() as *const c_char,
        );
    }

    pid = fork();
    if pid < 0 {
        err(EXIT_FAILURE, b"fork\0".as_ptr() as *const c_char);
    }
    if pid == 0 {
        if prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0) != 0 {
            err(
                EXIT_FAILURE,
                b"can't set the parent death signal\0".as_ptr() as *const c_char,
            );
        }
        loop {
            ret = syscall(__NR_gettid);
            if ret == USER_NOTIF_MAGIC as c_long {
                continue;
            }
            break;
        }
        _exit(1);
    }

    if sync_mode {
        if ioctl(
            listener,
            SECCOMP_IOCTL_NOTIF_SET_FLAGS,
            SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP,
            0,
        ) != 0
        {
            err(
                EXIT_FAILURE,
                b"can't set SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP\0".as_ptr() as *const c_char,
            );
        }
    }
    user_notification_sync_loop(listener);

    kill(pid, SIGKILL);
    if waitpid(pid, &mut status, 0) != pid {
        err(
            EXIT_FAILURE,
            b"waitpid(%d) failed\0".as_ptr() as *const c_char,
            pid,
        );
    }
    if !WIFSIGNALED(status) || WTERMSIG(status) != SIGKILL {
        errx(
            EXIT_FAILURE,
            b"unexpected exit code: %d\0".as_ptr() as *const c_char,
            status,
        );
    }

    gettimeofday(&mut stop, ptr::null_mut());
    timersub(&stop, &start, &mut diff);

    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            printf(
                b"# Executed %lu system calls\n\n\0".as_ptr() as *const c_char,
                loops,
            );

            result_usec = (diff.tv_sec as c_ulong).wrapping_mul(USEC_PER_SEC);
            result_usec = result_usec.wrapping_add(diff.tv_usec as c_ulong);

            printf(
                b" %14s: %lu.%03lu [sec]\n\n\0".as_ptr() as *const c_char,
                b"Total time\0".as_ptr() as *const c_char,
                diff.tv_sec as c_ulong,
                (diff.tv_usec as c_ulong) / USEC_PER_MSEC,
            );

            printf(
                b" %14lf usecs/op\n\0".as_ptr() as *const c_char,
                result_usec as f64 / loops as f64,
            );
            printf(
                b" %14d ops/sec\n\0".as_ptr() as *const c_char,
                (loops as f64 / (result_usec as f64 / USEC_PER_SEC as f64)) as c_int,
            );
        }

        BENCH_FORMAT_SIMPLE => {
            printf(
                b"%lu.%03lu\n\0".as_ptr() as *const c_char,
                diff.tv_sec as c_ulong,
                (diff.tv_usec as c_ulong) / USEC_PER_MSEC,
            );
        }

        _ => {
            /* reaching here is something disaster */
            fprintf(
                stderr,
                b"Unknown format:%d\n\0".as_ptr() as *const c_char,
                bench_format,
            );
            exit(1);
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
