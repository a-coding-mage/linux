// SPDX-License-Identifier: GPL-2.0
/*
 * Test that seccomp, tracepoints and audit observe the correct syscall
 * arguments after a ptracer has modified them at syscall-enter-stop.
 *
 * On arm64, both the first argument and the return value of a syscall
 * are passed in register x0.  The original x0 is saved in
 * pt_regs::orig_x0 during syscall entry and returned as the first
 * argument by syscall_get_arguments().  Because ptrace modifications
 * to x0 are not automatically reflected in orig_x0, seccomp, tracepoints
 * and audit may see a stale value unless orig_x0 is explicitly
 * re-synchronised after a ptrace stop.
 *
 * This test sets up a seccomp filter that allows write(2, ...) but kills
 * the task for any other fd.  A ptracer changes the fd argument from 2
 * to 1 at the syscall-enter stop.  If the orig_x0 re-sync works, seccomp
 * sees the modified argument (fd=1) and kills the child with SIGSYS
 * (test passes).  If orig_x0 is not re-synced, seccomp sees the original
 * fd=2, the write succeeds and the child exits normally (test fails,
 * vulnerability present).
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const EXPECTED_TESTS: c_int = 1;

#[repr(C)]
struct seccomp_data {
    nr: c_int,
    arch: c_uint,
    instruction_pointer: u64,
    args: [u64; 6],
}

#[repr(C)]
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
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct user_regs_struct {
    regs: [c_ulong; 31],
    sp: c_ulong,
    pc: c_ulong,
    pstate: c_ulong,
}

unsafe extern "C" {
    fn ptrace(request: c_uint, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn fork() -> pid_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strsignal(sig: c_int) -> *mut c_char;

    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_test_result(condition: bool, msg: *const c_char, ...);
    fn ksft_print_cnts();
}

#[allow(non_camel_case_types)]
type pid_t = c_int;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const SIGSYS: c_int = 31;

const PTRACE_TRACEME: c_uint = 0;
const PTRACE_SYSCALL: c_uint = 24;
const PTRACE_SETOPTIONS: c_uint = 0x4200;
const PTRACE_GETREGSET: c_uint = 0x4204;
const PTRACE_SETREGSET: c_uint = 0x4205;

const PTRACE_O_TRACESYSGOOD: c_ulong = 0x00000001;
const PTRACE_O_EXITKILL: c_ulong = 0x00100000;

const NT_PRSTATUS: c_uint = 1;

const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_SET_SECCOMP: c_int = 22;

const SECCOMP_MODE_FILTER: c_uint = 2;
const SECCOMP_RET_KILL: u32 = 0x00000000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;

const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;

const __NR_write: c_long = 64;

#[cfg(target_endian = "little")]
const ARG0_OFFSET: usize = offset_of!(seccomp_data, args);
#[cfg(target_endian = "big")]
const ARG0_OFFSET: usize = offset_of!(seccomp_data, args) + 4;

const fn BPF_STMT(code: u16, k: u32) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

const fn BPF_JUMP(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WIFSIGNALED(status: c_int) -> bool {
    let term_sig = status & 0x7f;
    term_sig != 0 && term_sig != 0x7f
}

fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn do_child() -> c_int {
    if ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
        ksft_exit_fail_perror(c"PTRACE_TRACEME".as_ptr());
    }

    if raise(SIGSTOP) != 0 {
        ksft_exit_fail_perror(c"raise(SIGSTOP)".as_ptr());
    }

    /*
     * Seccomp filter:
     *    If syscall is not write -> ALLOW
     *    If syscall is write:
     *	- If args[0] (fd) == 2 -> ALLOW
     *	- Otherwise -> KILL
     */
    let mut filter = [
        BPF_STMT(
            BPF_LD | BPF_W | BPF_ABS,
            offset_of!(seccomp_data, nr) as u32,
        ), /* nr */
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_write as u32, 0, 3),
        BPF_STMT(BPF_LD | BPF_W | BPF_ABS, ARG0_OFFSET as u32), /* args[0] */
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, 2, 1, 0),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_KILL),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];
    let mut prog = sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };

    if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
        ksft_exit_fail_perror(c"prctl NO_NEW_PRIVS".as_ptr());
    }

    if prctl(
        PR_SET_SECCOMP,
        SECCOMP_MODE_FILTER,
        &mut prog as *mut sock_fprog,
    ) != 0
    {
        ksft_exit_fail_perror(c"prctl SECCOMP".as_ptr());
    }

    /*
     * Invoke write(2, ...) while the tracer will change the first
     * argument (fd) from 2 to 1 at syscall entry.
     */
    syscall(__NR_write, 2, ptr::null::<c_void>(), 0);
    _exit(0);
}

unsafe fn do_parent(child: pid_t) -> c_int {
    let mut bypass = false;
    let mut status: c_int = 0;

    /* Wait for the initial SIGSTOP */
    if waitpid(child, &mut status, 0) != child {
        ksft_exit_fail_msg(c"waitpid failed".as_ptr());
    }

    if !WIFSTOPPED(status) || WSTOPSIG(status) != SIGSTOP {
        ksft_exit_fail_msg(c"unexpected stop status".as_ptr());
    }

    if ptrace(
        PTRACE_SETOPTIONS,
        child,
        0,
        PTRACE_O_TRACESYSGOOD | PTRACE_O_EXITKILL,
    ) != 0
    {
        ksft_exit_fail_perror(c"PTRACE_SETOPTIONS".as_ptr());
    }

    if ptrace(PTRACE_SYSCALL, child, 0, 0) != 0 {
        ksft_exit_fail_perror(c"PTRACE_SYSCALL".as_ptr());
    }

    loop {
        let mut sig: c_int;

        if waitpid(child, &mut status, 0) != child {
            ksft_exit_fail_msg(c"waitpid lost child".as_ptr());
        }

        if WIFEXITED(status) {
            /* Child exited normally - bypass succeeded */
            bypass = true;
            break;
        }

        if WIFSIGNALED(status) {
            sig = WTERMSIG(status);
            if sig == SIGSYS {
                break;
            }
            ksft_exit_fail_msg(
                c"child died unexpectedly from signal %d (%s)".as_ptr(),
                sig,
                strsignal(sig),
            );
        }

        if !WIFSTOPPED(status) {
            ksft_exit_fail_msg(c"unexpected wait status".as_ptr());
        }

        sig = WSTOPSIG(status);

        if sig == (SIGTRAP | 0x80) {
            let mut regs = user_regs_struct {
                regs: [0; 31],
                sp: 0,
                pc: 0,
                pstate: 0,
            };
            let mut iov = iovec {
                iov_base: &mut regs as *mut user_regs_struct as *mut c_void,
                iov_len: size_of::<user_regs_struct>(),
            };

            if ptrace(PTRACE_GETREGSET, child, NT_PRSTATUS, &mut iov as *mut iovec) != 0 {
                ksft_exit_fail_perror(c"PTRACE_GETREGSET".as_ptr());
            }

            let syscall_nr: c_ulong = regs.regs[8];
            let x0: c_ulong = regs.regs[0];

            /* Modify fd from 2 to 1 at write entry */
            if syscall_nr == __NR_write as c_ulong && x0 == 2 {
                regs.regs[0] = 1;
                if ptrace(PTRACE_SETREGSET, child, NT_PRSTATUS, &mut iov as *mut iovec) != 0 {
                    ksft_exit_fail_perror(c"PTRACE_SETREGSET".as_ptr());
                }
            }

            if ptrace(PTRACE_SYSCALL, child, 0, 0) != 0 {
                ksft_exit_fail_perror(c"PTRACE_SYSCALL".as_ptr());
            }
        } else {
            /* Forward other signals */
            if ptrace(PTRACE_SYSCALL, child, 0, sig) != 0 {
                ksft_exit_fail_perror(c"PTRACE_SYSCALL".as_ptr());
            }
        }
    }

    /* bypass == true means vulnerability exists -> test fails */
    if bypass {
        EXIT_FAILURE
    } else {
        EXIT_SUCCESS
    }
}

fn main() {
    unsafe {
        let child: pid_t;

        ksft_print_header();
        ksft_set_plan(EXPECTED_TESTS as c_uint);

        child = fork();
        if child < 0 {
            ksft_exit_fail_msg(c"fork failed: %s".as_ptr(), strerror(*__errno_location()));
        }

        if child == 0 {
            do_child();
        }

        /*
         * do_parent() returns EXIT_SUCCESS if the child was killed by
         * SIGSYS (i.e. seccomp correctly saw the modified argument),
         * and EXIT_FAILURE if the child exited normally (bypass).
         */
        let result = do_parent(child);

        ksft_test_result(
            result == EXIT_SUCCESS,
            c"seccomp_ptrace_x0_bypass\n".as_ptr(),
        );

        ksft_print_cnts();
        std::process::exit(result);
    }
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
