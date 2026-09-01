// SPDX-License-Identifier: GPL-2.0
/*
 * Test for SECCOMP_RET_TRACE argument modification bypass
 * via stale orig_x0 during filter re-evaluation.
 *
 * On arm64, syscall_get_arguments() reads the first argument from
 * regs->orig_x0.  When a seccomp filter returns SECCOMP_RET_TRACE,
 * ptrace may modify regs->regs[0] while orig_x0 remains unchanged.
 * The kernel then re-evaluates the filter; if it sees the stale
 * orig_x0, it may incorrectly allow a syscall that the tracer intended
 * to block.
 *
 * This test installs a filter that:
 *   - TRACEs write() when fd == 2
 *   - returns ERRNO(EPERM) when fd == 1
 *   - allows all other syscalls
 *
 * The child calls write(2, ...).  The parent catches the SECCOMP stop,
 * changes x0 (fd) from 2 to 1, and resumes the child.
 *
 * If re-evaluation sees the old fd=2 (stale orig_x0), the filter
 * returns TRACE again; because recheck_after_trace is true, the kernel
 * allows the syscall to proceed.  write(1, ...) succeeds, child exits 0.
 * -> test FAIL (bypass detected).
 *
 * If re-evaluation sees the new fd=1 (synced orig_x0), the filter
 * returns ERRNO(EPERM), write fails, child exits 1.
 * -> test PASS (no bypass).
 *
 * No special privileges required beyond CAP_SYS_PTRACE.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const EPERM: c_int = 1;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const SIGTRAP: c_int = 5;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const PTRACE_TRACEME: c_uint = 0;
const PTRACE_CONT: c_uint = 7;
const PTRACE_SETOPTIONS: c_uint = 0x4200;
const PTRACE_GETREGSET: c_uint = 0x4204;
const PTRACE_SETREGSET: c_uint = 0x4205;
const PTRACE_O_TRACESECCOMP: c_ulong = 0x0000_0080;
const PTRACE_EVENT_SECCOMP: c_int = 7;

const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_SET_SECCOMP: c_int = 22;
const SECCOMP_MODE_FILTER: c_uint = 2;

const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;

const SECCOMP_RET_KILL_PROCESS: u32 = 0x8000_0000;
const SECCOMP_RET_KILL_THREAD: u32 = 0x0000_0000;
const SECCOMP_RET_TRAP: u32 = 0x0003_0000;
const SECCOMP_RET_ERRNO: u32 = 0x0005_0000;
const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc0_0000;
const SECCOMP_RET_TRACE: u32 = 0x7ff0_0000;
const SECCOMP_RET_LOG: u32 = 0x7ffc_0000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
const SECCOMP_RET_DATA: u32 = 0x0000_ffff;

const NT_PRSTATUS: c_int = 1;
const __NR_write: c_long = 64;

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
struct seccomp_data {
    nr: c_int,
    arch: u32,
    instruction_pointer: u64,
    args: [u64; 6],
}

#[repr(C)]
struct user_pt_regs {
    regs: [u64; 31],
    sp: u64,
    pc: u64,
    pstate: u64,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

unsafe extern "C" {
    fn ptrace(request: c_uint, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn _exit(status: c_int) -> !;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn bpf_stmt(code: u16, k: u32) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

const fn bpf_jump(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

fn ptrace_event_mask(status: c_int) -> c_int {
    status >> 16
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifsignaled(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

fn wifstopped(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn wstopsig(status: c_int) -> c_int {
    wexitstatus(status)
}

#[cfg(target_endian = "little")]
const ARG0_OFFSET: usize = offset_of!(seccomp_data, args);
#[cfg(target_endian = "big")]
const ARG0_OFFSET: usize = offset_of!(seccomp_data, args) + 4;

unsafe fn do_child() -> c_int {
    let ret: c_long;

    if unsafe { ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) } != 0
    {
        unsafe { _exit(2) };
    }

    unsafe { raise(SIGSTOP) }; /* synchronize with parent */

    /*
     * Filter:
     *   if syscall == write:
     *     if fd == 2 -> TRACE
     *     if fd == 1 -> ERRNO(EPERM)
     *     else -> ALLOW
     *   else -> ALLOW
     */
    let mut filter = [
        /* Load syscall number */
        bpf_stmt(
            BPF_LD | BPF_W | BPF_ABS,
            offset_of!(seccomp_data, nr) as u32,
        ),
        /* If not write, allow */
        bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, __NR_write as u32, 0, 5),
        /* Load first argument (fd) */
        bpf_stmt(BPF_LD | BPF_W | BPF_ABS, ARG0_OFFSET as u32),
        /* fd == 2 ? */
        bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, 2, 0, 1),
        /* Yes: TRACE */
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_TRACE),
        /* fd == 1 ? */
        bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, 1, 0, 1),
        /* Yes: ERRNO(EPERM) */
        bpf_stmt(
            BPF_RET | BPF_K,
            SECCOMP_RET_ERRNO | ((EPERM as u32) & SECCOMP_RET_DATA),
        ),
        /* Other fd: ALLOW */
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];

    let mut prog = sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };

    if unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0 {
        unsafe { _exit(3) };
    }
    if unsafe { prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut prog as *mut sock_fprog) } != 0 {
        unsafe { _exit(4) };
    }

    /*
     * write(2, ...) triggers TRACE, parent changes fd to 1.
     * If re-eval sees fd=1 -> ERRNO -> write fails, ret = -EPERM.
     * If re-eval sees fd=2 -> TRACE again -> allowed -> write succeeds.
     */
    ret = unsafe { syscall(__NR_write, 2, c_str!(""), 0) };
    unsafe { _exit(if ret == 0 { 0 } else { 1 }) };
}

fn main() {
    let mut regs = user_pt_regs {
        regs: [0; 31],
        sp: 0,
        pc: 0,
        pstate: 0,
    };
    let mut iov = iovec {
        iov_base: (&mut regs as *mut user_pt_regs).cast::<c_void>(),
        iov_len: size_of::<user_pt_regs>(),
    };
    let child: c_int;
    let mut status: c_int = 0;

    unsafe {
        ksft_print_header();
        ksft_set_plan(1);

        child = fork();
        if child < 0 {
            ksft_exit_fail_msg(c_str!("fork failed: %s"), strerror(*__errno_location()));
        }

        if child == 0 {
            do_child();
        }

        /* 1. Wait for initial SIGSTOP */
        if waitpid(child, &mut status, 0) != child {
            ksft_exit_fail_msg(c_str!("waitpid SIGSTOP"));
        }
        if !wifstopped(status) || wstopsig(status) != SIGSTOP {
            ksft_exit_fail_msg(c_str!("unexpected initial stop"));
        }

        /* 2. Enable SECCOMP ptrace events */
        if ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESECCOMP) != 0 {
            ksft_exit_fail_msg(c_str!("PTRACE_SETOPTIONS"));
        }

        /* 3. Continue child to hit SECCOMP stop */
        if ptrace(PTRACE_CONT, child, 0, 0) != 0 {
            ksft_exit_fail_msg(c_str!("PTRACE_CONT"));
        }

        /* 4. Wait for SECCOMP stop */
        loop {
            if waitpid(child, &mut status, 0) != child {
                ksft_exit_fail_msg(c_str!("waitpid SECCOMP"));
            }
            if wifexited(status) {
                ksft_test_result_fail(c_str!("child exited before SECCOMP stop\n"));
                break;
            }
            if wifsignaled(status) {
                ksft_test_result_fail(c_str!("child killed unexpectedly\n"));
                break;
            }
            if wifstopped(status)
                && wstopsig(status) == SIGTRAP
                && ptrace_event_mask(status) == PTRACE_EVENT_SECCOMP
            {
                break;
            }
            ptrace(PTRACE_CONT, child, 0, wstopsig(status));
        }

        if !(wifstopped(status)
            && wstopsig(status) == SIGTRAP
            && ptrace_event_mask(status) == PTRACE_EVENT_SECCOMP)
        {
            if child > 0 {
                kill(child, SIGKILL);
                waitpid(child, ptr::null_mut(), 0);
            }
            ksft_print_cnts();
            std::process::exit(if ksft_get_fail_cnt() != 0 {
                EXIT_FAILURE
            } else {
                EXIT_SUCCESS
            });
        }

        /* 5. Modify x0 (fd) from 2 to 1 */
        if ptrace(
            PTRACE_GETREGSET,
            child,
            NT_PRSTATUS,
            &mut iov as *mut iovec,
        ) != 0
        {
            ksft_exit_fail_perror(c_str!("GETREGSET"));
        }
        if regs.regs[8] != __NR_write as u64 || regs.regs[0] != 2 {
            ksft_test_result_fail(
                c_str!("unexpected regs: syscall=%llu, x0=%llu\n"),
                regs.regs[8],
                regs.regs[0],
            );
        } else {
            regs.regs[0] = 1;
            if ptrace(
                PTRACE_SETREGSET,
                child,
                NT_PRSTATUS,
                &mut iov as *mut iovec,
            ) != 0
            {
                ksft_exit_fail_perror(c_str!("SETREGSET"));
            }

            /* 6. Resume child */
            if ptrace(PTRACE_CONT, child, 0, 0) != 0 {
                ksft_exit_fail_perror(c_str!("PTRACE_CONT"));
            }

            /* 7. Reap child - must exit normally */
            if waitpid(child, &mut status, 0) != child {
                ksft_exit_fail_msg(c_str!("final waitpid"));
            }

            if !wifexited(status) {
                ksft_test_result_fail(c_str!("child did not exit normally\n"));
            } else if wexitstatus(status) != 0 {
                ksft_test_result_pass(c_str!("seccomp correctly denied modified syscall\n"));
            } else {
                ksft_test_result_fail(c_str!("write succeeded, orig_x0 bypass likely\n"));
            }
        }

        if child > 0 {
            kill(child, SIGKILL);
            waitpid(child, ptr::null_mut(), 0);
        }
        ksft_print_cnts();
        std::process::exit(if ksft_get_fail_cnt() != 0 {
            EXIT_FAILURE
        } else {
            EXIT_SUCCESS
        });
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
