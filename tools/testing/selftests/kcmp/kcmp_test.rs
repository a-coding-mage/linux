// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included Linux/libc/kselftest headers.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const O_RDWR: c_int = 0o2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;

const EPOLLIN: u32 = 0x00000001;
const EPOLLOUT: u32 = 0x00000004;
const EPOLL_CTL_ADD: c_int = 1;

const KCMP_FILE: c_int = 0;
const KCMP_VM: c_int = 1;
const KCMP_FILES: c_int = 2;
const KCMP_FS: c_int = 3;
const KCMP_SIGHAND: c_int = 4;
const KCMP_IO: c_int = 5;
const KCMP_SYSVSEM: c_int = 6;
const KCMP_EPOLL_TFD: c_int = 7;
const KCMP_TYPES: c_int = 8;

const P_ALL: c_int = 0;

#[cfg(target_arch = "x86_64")]
const __NR_kcmp: c_long = 312;

// Build-time syscall number is supplied by <linux/unistd.h> in the C source.
#[cfg(not(target_arch = "x86_64"))]
const __NR_kcmp: c_long = 0;

#[repr(C)]
struct kcmp_epoll_slot {
    efd: u32,
    tfd: u32,
    toff: u32,
}

#[repr(C, packed)]
struct epoll_event {
    events: u32,
    data: u64,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn getpid() -> c_int;
    fn perror(s: *const c_char);
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;

    fn ksft_exit_fail() -> !;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_inc_fail_cnt();
    fn ksft_inc_pass_cnt();
    fn ksft_exit_pass() -> !;

    static mut errno: c_int;
}

unsafe fn sys_kcmp(
    pid1: c_int,
    pid2: c_int,
    type_: c_int,
    fd1: c_ulong,
    fd2: c_ulong,
) -> c_long {
    unsafe { syscall(__NR_kcmp, pid1, pid2, type_, fd1, fd2) }
}

static duped_num: c_uint = 64;

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let kpath: &[u8] = b"kcmp-test-file\0";
    let mut epoll_slot: kcmp_epoll_slot;
    let mut ev: epoll_event = core::mem::zeroed();
    let pid1: c_int;
    let pid2: c_int;
    let mut pipefd: [c_int; 2] = [0; 2];
    let fd1: c_int;
    let mut fd2: c_int;
    let epollfd: c_int;
    let mut status: c_int = 0;
    let fddup: c_int;

    fd1 = open(kpath.as_ptr() as *const c_char, O_RDWR | O_CREAT | O_TRUNC, 0o644);
    pid1 = getpid();

    if fd1 < 0 {
        perror(c"Can't create file".as_ptr());
        ksft_exit_fail();
    }

    if pipe(pipefd.as_mut_ptr()) != 0 {
        perror(c"Can't create pipe".as_ptr());
        ksft_exit_fail();
    }

    epollfd = epoll_create1(0);
    if epollfd < 0 {
        perror(c"epoll_create1 failed".as_ptr());
        ksft_exit_fail();
    }

    memset(
        &mut ev as *mut epoll_event as *mut c_void,
        0xff,
        core::mem::size_of::<epoll_event>(),
    );
    ev.events = EPOLLIN | EPOLLOUT;

    if epoll_ctl(epollfd, EPOLL_CTL_ADD, pipefd[0], &mut ev) != 0 {
        perror(c"epoll_ctl failed".as_ptr());
        ksft_exit_fail();
    }

    fddup = dup2(pipefd[1], duped_num as c_int);
    if fddup < 0 {
        perror(c"dup2 failed".as_ptr());
        ksft_exit_fail();
    }

    if epoll_ctl(epollfd, EPOLL_CTL_ADD, fddup, &mut ev) != 0 {
        perror(c"epoll_ctl failed".as_ptr());
        ksft_exit_fail();
    }
    close(fddup);

    pid2 = fork();
    if pid2 < 0 {
        perror(c"fork failed".as_ptr());
        ksft_exit_fail();
    }

    if pid2 == 0 {
        let pid2: c_int = getpid();
        let mut ret: c_int;

        ksft_print_header();
        ksft_set_plan(3);

        fd2 = open(kpath.as_ptr() as *const c_char, O_RDWR);
        if fd2 < 0 {
            perror(c"Can't open file".as_ptr());
            ksft_exit_fail();
        }

        /* An example of output and arguments */
        printf(
            c"pid1: %6d pid2: %6d FD: %2ld FILES: %2ld VM: %2ld FS: %2ld SIGHAND: %2ld IO: %2ld SYSVSEM: %2ld INV: %2ld\n".as_ptr(),
            pid1,
            pid2,
            sys_kcmp(pid1, pid2, KCMP_FILE, fd1 as c_ulong, fd2 as c_ulong),
            sys_kcmp(pid1, pid2, KCMP_FILES, 0, 0),
            sys_kcmp(pid1, pid2, KCMP_VM, 0, 0),
            sys_kcmp(pid1, pid2, KCMP_FS, 0, 0),
            sys_kcmp(pid1, pid2, KCMP_SIGHAND, 0, 0),
            sys_kcmp(pid1, pid2, KCMP_IO, 0, 0),
            sys_kcmp(pid1, pid2, KCMP_SYSVSEM, 0, 0),
            /* This one should fail */
            sys_kcmp(pid1, pid2, KCMP_TYPES + 1, 0, 0),
        );

        /* This one should return same fd */
        ret = sys_kcmp(pid1, pid2, KCMP_FILE, fd1 as c_ulong, fd1 as c_ulong) as c_int;
        if ret != 0 {
            printf(
                c"FAIL: 0 expected but %d returned (%s)\n".as_ptr(),
                ret,
                strerror(errno),
            );
            ksft_inc_fail_cnt();
            ret = -1;
        } else {
            printf(c"PASS: 0 returned as expected\n".as_ptr());
            ksft_inc_pass_cnt();
        }

        /* Compare with self */
        ret = sys_kcmp(pid1, pid1, KCMP_VM, 0, 0) as c_int;
        if ret != 0 {
            printf(
                c"FAIL: 0 expected but %d returned (%s)\n".as_ptr(),
                ret,
                strerror(errno),
            );
            ksft_inc_fail_cnt();
            ret = -1;
        } else {
            printf(c"PASS: 0 returned as expected\n".as_ptr());
            ksft_inc_pass_cnt();
        }

        /* Compare epoll target */
        epoll_slot = kcmp_epoll_slot {
            efd: epollfd as u32,
            tfd: duped_num,
            toff: 0,
        };
        ret = sys_kcmp(
            pid1,
            pid1,
            KCMP_EPOLL_TFD,
            pipefd[1] as c_ulong,
            &mut epoll_slot as *mut kcmp_epoll_slot as *mut c_void as c_ulong,
        ) as c_int;
        if ret != 0 {
            printf(
                c"FAIL: 0 expected but %d returned (%s)\n".as_ptr(),
                ret,
                strerror(errno),
            );
            ksft_inc_fail_cnt();
            ret = -1;
        } else {
            printf(c"PASS: 0 returned as expected\n".as_ptr());
            ksft_inc_pass_cnt();
        }

        if ret != 0 {
            ksft_exit_fail();
        } else {
            ksft_exit_pass();
        }
    }

    waitpid(pid2, &mut status, P_ALL);

    0
}

fn main() {
    unsafe {
        c_main(0, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
