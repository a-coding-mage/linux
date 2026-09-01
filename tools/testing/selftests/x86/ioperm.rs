// SPDX-License-Identifier: GPL-2.0
/*
 * ioperm.c - Test case for ioperm(2)
 * Copyright (c) 2015 Andrew Lutomirski
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type pid_t = c_int;

const SIGSEGV: c_int = 11;
const SA_RESETHAND: c_int = 0x80000000u32 as c_int;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
pub struct __jmp_buf_tag {
    _private: [c_ulong; 25],
}

type jmp_buf = [__jmp_buf_tag; 1];

static mut nerrs: c_int = 0;

static mut jmpbuf: jmp_buf = [__jmp_buf_tag { _private: [0; 25] }];

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn setresuid(ruid: c_uint, euid: c_uint, suid: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;

    /*
     * sigsetjmp is a C macro on glibc; this is the underlying entry point
     * corresponding to sigsetjmp(jmpbuf, savesigs).
     */
    fn __sigsetjmp(env: *mut __jmp_buf_tag, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut __jmp_buf_tag, val: c_int) -> !;

    /* From helpers.h. */
    fn sethandler(
        sig: c_int,
        handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        (*set).__bits = [0; 16];
    }
}

fn CPU_SET(cpu: usize, set: *mut cpu_set_t) {
    let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
    unsafe {
        (*set).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn sigsegv(_sig: c_int, _si: *mut siginfo_t, _ctx_void: *mut c_void) {
    unsafe {
        siglongjmp(core::ptr::addr_of_mut!(jmpbuf[0]), 1);
    }
}

fn try_outb(port: u16) -> bool {
    unsafe {
        sethandler(SIGSEGV, sigsegv, SA_RESETHAND);
        if __sigsetjmp(core::ptr::addr_of_mut!(jmpbuf[0]), 1) != 0 {
            false
        } else {
            asm!(
                "out dx, al",
                in("dx") port,
                in("al") 0u8,
                options(nomem, nostack, preserves_flags)
            );
            true
        }
        /*
         * Preserved from the C source: this is unreachable after the returns
         * above.
         */
        /* clearhandler(SIGSEGV); */
    }
}

fn expect_ok(port: u16) {
    unsafe {
        if !try_outb(port) {
            printf(c"[FAIL]\toutb to 0x%02hx failed\n".as_ptr(), port as c_int);
            exit(1);
        }

        printf(c"[OK]\toutb to 0x%02hx worked\n".as_ptr(), port as c_int);
    }
}

fn expect_gp(port: u16) {
    unsafe {
        if try_outb(port) {
            printf(c"[FAIL]\toutb to 0x%02hx worked\n".as_ptr(), port as c_int);
            exit(1);
        }

        printf(c"[OK]\toutb to 0x%02hx failed\n".as_ptr(), port as c_int);
    }
}

fn main() {
    unsafe {
        let mut cpuset: cpu_set_t = core::mem::zeroed();
        CPU_ZERO(&mut cpuset);
        CPU_SET(0, &mut cpuset);
        if sched_setaffinity(0, core::mem::size_of_val(&cpuset), &cpuset) != 0 {
            err(1, c"sched_setaffinity to CPU 0".as_ptr());
        }

        expect_gp(0x80);
        expect_gp(0xed);

        /*
         * Probe for ioperm support.  Note that clearing ioperm bits
         * works even as nonroot.
         */
        printf(c"[RUN]\tenable 0x80\n".as_ptr());
        if ioperm(0x80, 1, 1) != 0 {
            printf(
                c"[OK]\tioperm(0x80, 1, 1) failed (%d) -- try running as root\n".as_ptr(),
                errno(),
            );
            return;
        }
        expect_ok(0x80);
        expect_gp(0xed);

        printf(c"[RUN]\tdisable 0x80\n".as_ptr());
        if ioperm(0x80, 1, 0) != 0 {
            printf(c"[FAIL]\tioperm(0x80, 1, 0) failed (%d)".as_ptr(), errno());
            std::process::exit(1);
        }
        expect_gp(0x80);
        expect_gp(0xed);

        /* Make sure that fork() preserves ioperm. */
        if ioperm(0x80, 1, 1) != 0 {
            printf(c"[FAIL]\tioperm(0x80, 1, 0) failed (%d)".as_ptr(), errno());
            std::process::exit(1);
        }

        let child: pid_t = fork();
        if child == -1 {
            err(1, c"fork".as_ptr());
        }

        if child == 0 {
            printf(c"[RUN]\tchild: check that we inherited permissions\n".as_ptr());
            expect_ok(0x80);
            expect_gp(0xed);
            printf(c"[RUN]\tchild: Extend permissions to 0x81\n".as_ptr());
            if ioperm(0x81, 1, 1) != 0 {
                printf(c"[FAIL]\tioperm(0x81, 1, 1) failed (%d)".as_ptr(), errno());
                std::process::exit(1);
            }
            printf(c"[RUN]\tchild: Drop permissions to 0x80\n".as_ptr());
            if ioperm(0x80, 1, 0) != 0 {
                printf(c"[FAIL]\tioperm(0x80, 1, 0) failed (%d)".as_ptr(), errno());
                std::process::exit(1);
            }
            expect_gp(0x80);
            std::process::exit(0);
        } else {
            let mut status: c_int = 0;
            if waitpid(child, &mut status, 0) != child || !WIFEXITED(status) {
                printf(c"[FAIL]\tChild died\n".as_ptr());
                nerrs += 1;
            } else if WEXITSTATUS(status) != 0 {
                printf(c"[FAIL]\tChild failed\n".as_ptr());
                nerrs += 1;
            } else {
                printf(c"[OK]\tChild succeeded\n".as_ptr());
            }
        }

        /* Verify that the child dropping 0x80 did not affect the parent */
        printf(c"\tVerify that unsharing the bitmap worked\n".as_ptr());
        expect_ok(0x80);

        /* Test the capability checks. */
        printf(c"\tDrop privileges\n".as_ptr());
        if setresuid(1, 1, 1) != 0 {
            printf(c"[WARN]\tDropping privileges failed\n".as_ptr());
            return;
        }

        printf(c"[RUN]\tdisable 0x80\n".as_ptr());
        if ioperm(0x80, 1, 0) != 0 {
            printf(c"[FAIL]\tioperm(0x80, 1, 0) failed (%d)".as_ptr(), errno());
            std::process::exit(1);
        }
        printf(c"[OK]\tit worked\n".as_ptr());

        printf(c"[RUN]\tenable 0x80 again\n".as_ptr());
        if ioperm(0x80, 1, 1) == 0 {
            printf(c"[FAIL]\tit succeeded but should have failed.\n".as_ptr());
            std::process::exit(1);
        }
        printf(c"[OK]\tit failed\n".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
