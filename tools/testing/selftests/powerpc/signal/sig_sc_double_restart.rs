// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test that a syscall does not get restarted twice, handled by trap_norestart()
 *
 * Based on Al's description, and a test for the bug fixed in this commit:
 *
 * commit 9a81c16b527528ad307843be5571111aa8d35a80
 * Author: Al Viro <viro@zeniv.linux.org.uk>
 * Date:   Mon Sep 20 21:48:57 2010 +0100
 *
 *  powerpc: fix double syscall restarts
 *
 *  Make sigreturn zero regs->trap, make do_signal() do the same on all
 *  paths.  As it is, signal interrupting e.g. read() from fd 512 (==
 *  ERESTARTSYS) with another signal getting unblocked when the first
 *  handler finishes will lead to restart one insn earlier than it ought
 *  to.  Same for multiple signals with in-kernel handlers interrupting
 *  that sucker at the same time.  Same for multiple signals of any kind
 *  interrupting that sucker on 64bit...
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type sighandler_t = extern "C" fn(c_int);

const SIGUSR1: c_int = 10;
const SIGUSR2: c_int = 12;
const SA_RESTART: c_int = 0x10000000;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const ENOANO: c_int = 55;
const __NR_read: c_long = 3;

const DATA: &[u8] = b"test 123\0";
const DLEN: size_t = DATA.len();

#[repr(C)]
struct sigset_t {
    __val: [usize; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: sighandler_t,
    sa_flags: c_uint,
    sa_restorer: *mut c_void,
    sa_mask: sigset_t,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn dup(oldfd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn _exit(status: c_int) -> !;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;

    static mut stderr: *mut c_void;

    fn test_harness_set_timeout(seconds: u64);
    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn FAIL_IF(cond: bool) {
    if cond {
        unsafe {
            exit(EXIT_FAILURE);
        }
    }
}

extern "C" fn SIGUSR1_handler(_sig: c_int) {
    unsafe {
        kill(getpid(), SIGUSR2);
    }
    /*
     * SIGUSR2 is blocked until the handler exits, at which point it will
     * be raised again and think there is a restart to be done because the
     * pending restarted syscall has 512 (ERESTARTSYS) in r3. The second
     * restart will retreat NIP another 4 bytes to fail case branch.
     */
}

extern "C" fn SIGUSR2_handler(_sig: c_int) {}

unsafe fn raw_read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    let mut nr: c_long = __NR_read;
    let mut _fd: c_long = fd as c_long;
    let mut _buf: *mut c_void = buf;
    let mut _count: size_t = count;

    asm!(
        "b 0f",
        "b 1f",
        "0:",
        "sc 0",
        "bns 2f",
        "neg {0},{0}",
        "b 2f",
        "1:",
        "li {0},{4}",
        "2:",
        inout("r3") _fd,
        inout("r0") nr,
        inout("r4") _buf,
        inout("r5") _count,
        const -ENOANO,
        out("r6") _,
        out("r7") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
        out("r12") _,
        options(nostack)
    );

    if _fd < 0 {
        errno = -_fd as c_int;
        _fd = -1;
    }

    _fd as ssize_t
}

extern "C" fn test_restart() -> c_int {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        let mut pid: pid_t;
        let mut buf: [c_char; 512] = [0; 512];

        if pipe(pipefd.as_mut_ptr()) == -1 {
            perror(c"pipe".as_ptr());
            exit(EXIT_FAILURE);
        }

        pid = fork();
        if pid == -1 {
            perror(c"fork".as_ptr());
            exit(EXIT_FAILURE);
        }

        if pid == 0 {
            /* Child reads from pipe */
            let mut act: sigaction = core::mem::zeroed();
            let mut fd: c_int;

            memset(
                &mut act as *mut sigaction as *mut c_void,
                0,
                core::mem::size_of_val(&act),
            );
            sigaddset(&mut act.sa_mask, SIGUSR2);
            act.sa_handler = SIGUSR1_handler;
            act.sa_flags = SA_RESTART as c_uint;
            if sigaction(SIGUSR1, &act, core::ptr::null_mut()) == -1 {
                perror(c"sigaction".as_ptr());
                exit(EXIT_FAILURE);
            }

            memset(
                &mut act as *mut sigaction as *mut c_void,
                0,
                core::mem::size_of_val(&act),
            );
            act.sa_handler = SIGUSR2_handler;
            act.sa_flags = SA_RESTART as c_uint;
            if sigaction(SIGUSR2, &act, core::ptr::null_mut()) == -1 {
                perror(c"sigaction".as_ptr());
                exit(EXIT_FAILURE);
            }

            /* Let's get ERESTARTSYS into r3 */
            loop {
                fd = dup(pipefd[0]);
                if fd == 512 {
                    break;
                }
                if fd == -1 {
                    perror(c"dup".as_ptr());
                    exit(EXIT_FAILURE);
                }
            }

            if raw_read(fd, buf.as_mut_ptr() as *mut c_void, 512) == -1 {
                if errno == ENOANO {
                    fprintf(
                        stderr,
                        c"Double restart moved restart before sc instruction.\n".as_ptr(),
                    );
                    _exit(EXIT_FAILURE);
                }
                perror(c"read".as_ptr());
                exit(EXIT_FAILURE);
            }

            if strncmp(buf.as_ptr(), DATA.as_ptr() as *const c_char, DLEN) != 0 {
                fprintf(stderr, c"bad test string %s\n".as_ptr(), buf.as_ptr());
                exit(EXIT_FAILURE);
            }

            return 0;
        } else {
            let mut wstatus: c_int = 0;

            usleep(100000); /* Hack to get reader waiting */
            kill(pid, SIGUSR1);
            usleep(100000);
            if write(pipefd[1], DATA.as_ptr() as *const c_void, DLEN) != DLEN as ssize_t {
                perror(c"write".as_ptr());
                exit(EXIT_FAILURE);
            }
            close(pipefd[0]);
            close(pipefd[1]);
            if wait(&mut wstatus) == -1 {
                perror(c"wait".as_ptr());
                exit(EXIT_FAILURE);
            }
            if !WIFEXITED(wstatus) {
                fprintf(stderr, c"child exited abnormally\n".as_ptr());
                exit(EXIT_FAILURE);
            }

            FAIL_IF(WEXITSTATUS(wstatus) != EXIT_SUCCESS);

            return 0;
        }
    }
}

fn main() -> c_int {
    unsafe {
        test_harness_set_timeout(10);
        test_harness(test_restart, c"sig sys restart".as_ptr())
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
