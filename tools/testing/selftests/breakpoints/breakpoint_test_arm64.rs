// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Google, Inc.
 *
 * Original Code by Pavel Labath <labath@google.com>
 *
 * Code modified by Pratyush Anand <panand@redhat.com>
 * for testing different byte select for each access size.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type uintptr_t = usize;

const PTRACE_TRACEME: c_uint = 0;
const PTRACE_CONT: c_uint = 7;
const PTRACE_GETSIGINFO: c_uint = 0x4202;
const PTRACE_SETREGSET: c_uint = 0x4204;
const NT_ARM_HW_WATCH: c_uint = 0x403;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const SIGKILL: c_int = 9;
const SIGALRM: c_int = 14;
const EIO: c_int = 5;
const __WALL: c_int = 0x40000000;
const TRAP_HWBKPT: c_int = 4;

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct user_hwdebug_state_reg {
    addr: u64,
    ctrl: u32,
    pad: u32,
}

#[repr(C)]
struct user_hwdebug_state {
    dbg_info: u32,
    pad: u32,
    dbg_regs: [user_hwdebug_state_reg; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    _data: [u8; 128 - 3 * size_of::<c_int>()],
}

#[repr(align(32))]
struct AlignedVar([u8; 96]);

static mut var: AlignedVar = AlignedVar([0; 96]);

unsafe extern "C" {
    fn ptrace(request: c_uint, ...) -> c_long;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn alarm(seconds: c_uint) -> c_uint;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail() -> !;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

fn MIN(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn child(size: c_int, wr: c_int) {
    let addr = var.0.as_mut_ptr().offset((32 + wr) as isize);

    if ptrace(
        PTRACE_TRACEME,
        0 as c_long,
        ptr::null_mut::<c_void>(),
        ptr::null_mut::<c_void>(),
    ) != 0
    {
        ksft_print_msg(
            c"ptrace(PTRACE_TRACEME) failed: %s\n".as_ptr(),
            strerror(errno()),
        );
        _exit(1);
    }

    if raise(SIGSTOP) != 0 {
        ksft_print_msg(c"raise(SIGSTOP) failed: %s\n".as_ptr(), strerror(errno()));
        _exit(1);
    }

    if (addr as uintptr_t) % (size as uintptr_t) != 0 {
        ksft_print_msg(
            c"Wrong address write for the given size: %s\n".as_ptr(),
            strerror(errno()),
        );
        _exit(1);
    }

    match size {
        1 => {
            ptr::write_volatile(addr, 47);
        }
        2 => {
            ptr::write_volatile(addr as *mut u16, 47);
        }
        4 => {
            ptr::write_volatile(addr as *mut u32, 47);
        }
        8 => {
            ptr::write_volatile(addr as *mut u64, 47);
        }
        16 => {
            asm!("stp x29, x30, [{0}]", in(reg) addr, options(nostack, preserves_flags));
        }
        32 => {
            asm!("stp q29, q30, [{0}]", in(reg) addr, options(nostack, preserves_flags));
        }
        _ => {}
    }

    _exit(0);
}

unsafe fn set_watchpoint(pid: pid_t, size: c_int, wp: c_int) -> bool {
    let addr = var.0.as_ptr().offset((32 + wp) as isize);
    let offset = (addr as uintptr_t % 8) as c_int;
    let byte_mask = (((1u32 << size) - 1) << offset) as c_uint;
    let type_ = 2u32; /* Write */
    let enable = 1u32;
    let control = byte_mask << 5 | type_ << 3 | enable;
    let mut dreg_state = user_hwdebug_state {
        dbg_info: 0,
        pad: 0,
        dbg_regs: [user_hwdebug_state_reg {
            addr: 0,
            ctrl: 0,
            pad: 0,
        }; 16],
    };
    let mut iov = iovec {
        iov_base: ptr::null_mut(),
        iov_len: 0,
    };

    dreg_state.dbg_regs[0].addr = addr.offset(-(offset as isize)) as uintptr_t as u64;
    dreg_state.dbg_regs[0].ctrl = control;
    iov.iov_base = &mut dreg_state as *mut user_hwdebug_state as *mut c_void;
    iov.iov_len =
        offset_of!(user_hwdebug_state, dbg_regs) + size_of::<user_hwdebug_state_reg>();
    if ptrace(PTRACE_SETREGSET, pid, NT_ARM_HW_WATCH, &mut iov) == 0 {
        return true;
    }

    if errno() == EIO {
        ksft_print_msg(
            c"ptrace(PTRACE_SETREGSET, NT_ARM_HW_WATCH) not supported on this hardware: %s\n"
                .as_ptr(),
            strerror(errno()),
        );
    }

    ksft_print_msg(
        c"ptrace(PTRACE_SETREGSET, NT_ARM_HW_WATCH) failed: %s\n".as_ptr(),
        strerror(errno()),
    );
    false
}

unsafe fn run_test(wr_size: c_int, wp_size: c_int, wr: c_int, wp: c_int) -> bool {
    let mut status: c_int = 0;
    let mut siginfo = siginfo_t {
        si_signo: 0,
        si_errno: 0,
        si_code: 0,
        _data: [0; 128 - 3 * size_of::<c_int>()],
    };
    let pid = fork();
    let mut wpid: pid_t;

    if pid < 0 {
        ksft_test_result_fail(c"fork() failed: %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    if pid == 0 {
        child(wr_size, wr);
    }

    wpid = waitpid(pid, &mut status, __WALL);
    if wpid != pid {
        ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    if !WIFSTOPPED(status) {
        ksft_print_msg(c"child did not stop: %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    if WSTOPSIG(status) != SIGSTOP {
        ksft_print_msg(c"child did not stop with SIGSTOP\n".as_ptr());
        return false;
    }

    if !set_watchpoint(pid, wp_size, wp) {
        return false;
    }

    if ptrace(
        PTRACE_CONT,
        pid,
        ptr::null_mut::<c_void>(),
        ptr::null_mut::<c_void>(),
    ) < 0
    {
        ksft_print_msg(c"ptrace(PTRACE_CONT) failed: %s\n".as_ptr(), strerror(errno()));
        return false;
    }

    alarm(3);
    wpid = waitpid(pid, &mut status, __WALL);
    if wpid != pid {
        ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    alarm(0);
    if WIFEXITED(status) {
        ksft_print_msg(c"child exited prematurely\n".as_ptr());
        return false;
    }
    if !WIFSTOPPED(status) {
        ksft_print_msg(c"child did not stop\n".as_ptr());
        return false;
    }
    if WSTOPSIG(status) != SIGTRAP {
        ksft_print_msg(c"child did not stop with SIGTRAP\n".as_ptr());
        return false;
    }
    if ptrace(
        PTRACE_GETSIGINFO,
        pid,
        ptr::null_mut::<c_void>(),
        &mut siginfo,
    ) != 0
    {
        ksft_print_msg(c"ptrace(PTRACE_GETSIGINFO): %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    if siginfo.si_code != TRAP_HWBKPT {
        ksft_print_msg(c"Unexpected si_code %d\n".as_ptr(), siginfo.si_code);
        return false;
    }

    kill(pid, SIGKILL);
    wpid = waitpid(pid, &mut status, 0);
    if wpid != pid {
        ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
        return false;
    }
    true
}

extern "C" fn sigalrm(_sig: c_int) {}

fn main() {
    unsafe {
        let mut succeeded = true;
        let mut act = sigaction {
            sa_handler: sigalrm,
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: ptr::null_mut(),
        };
        let mut wr: c_int;
        let mut wp: c_int;
        let mut size: c_int;
        let mut result: bool;

        ksft_print_header();
        ksft_set_plan(213);

        act.sa_handler = sigalrm;
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = 0;
        sigaction(SIGALRM, &act, ptr::null_mut());
        size = 1;
        while size <= 32 {
            wr = 0;
            while wr <= 32 {
                wp = wr - size;
                while wp <= wr + size {
                    result = run_test(size, MIN(size, 8), wr, wp);
                    if (result && wr == wp) || (!result && wr != wp) {
                        ksft_test_result_pass(
                            c"Test size = %d write offset = %d watchpoint offset = %d\n"
                                .as_ptr(),
                            size,
                            wr,
                            wp,
                        );
                    } else {
                        ksft_test_result_fail(
                            c"Test size = %d write offset = %d watchpoint offset = %d\n"
                                .as_ptr(),
                            size,
                            wr,
                            wp,
                        );
                        succeeded = false;
                    }
                    wp += size;
                }
                wr += size;
            }
            size *= 2;
        }

        size = 1;
        while size <= 32 {
            if run_test(size, 8, -size, -8) {
                ksft_test_result_pass(
                    c"Test size = %d write offset = %d watchpoint offset = -8\n".as_ptr(),
                    size,
                    -size,
                );
            } else {
                ksft_test_result_fail(
                    c"Test size = %d write offset = %d watchpoint offset = -8\n".as_ptr(),
                    size,
                    -size,
                );
                succeeded = false;
            }
            size *= 2;
        }

        if succeeded {
            ksft_exit_pass();
        } else {
            ksft_exit_fail();
        }
    }
}
