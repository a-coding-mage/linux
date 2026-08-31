// SPDX-License-Identifier: GPL-2.0-only

// Translated from cfitests.c. C include dependencies are represented as
// external declarations or constants expected from the surrounding build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type pid_t = c_int;
type size_t = usize;
type bool_t = bool;

const SIGSEGV: c_int = 11;
const SIGSTOP: c_int = 19;
const SA_SIGINFO: c_int = 4;
const SEGV_CPERR: c_int = 10;

const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;

const NT_RISCV_USER_CFI: c_ulong = 0x900;

const __NR_prctl: c_long = 167;
const PR_SET_CFI: c_long = 75;
const PR_GET_CFI: c_long = 76;
const PR_CFI_BRANCH_LANDING_PADS: c_long = 1;
const PR_CFI_ENABLE: c_ulong = 1;
const PR_GET_SHADOW_STACK_STATUS: c_long = 74;
const PR_SHADOW_STACK_ENABLE: c_ulong = 1;

const PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE: u64 = 1 << 0;
const PTRACE_CFI_SHADOW_STACK_EN_STATE: u64 = 1 << 1;
const PTRACE_CFI_SHADOW_STACK_PTR_STATE: u64 = 1 << 2;
const PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE: u64 = 1 << 3;

const CFI_ENABLE_MASK: u64 = PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE
    | PTRACE_CFI_SHADOW_STACK_EN_STATE
    | PTRACE_CFI_SHADOW_STACK_PTR_STATE;

// From cfi_rv_test.h.
const CHILD_EXIT_CODE_SSWRITE: c_int = 207;

#[repr(C)]
pub struct gregset_t {
    pub gregs: [c_ulong; 32],
}

#[repr(C)]
pub struct ucontext {
    pub uc_mcontext: gregset_t,
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct user_cfi_status {
    pub cfi_state: u64,
}

#[repr(C)]
pub struct user_cfi_state {
    pub cfi_status: user_cfi_status,
    pub shstk_ptr: u64,
}

unsafe extern "C" {
    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;

    fn exit(status: c_int) -> !;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn fork() -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn my_syscall5(
        number: c_long,
        arg1: c_long,
        arg2: c_long,
        arg3: c_ulong,
        arg4: c_ulong,
        arg5: c_ulong,
    ) -> c_int;
    fn execute_shadow_stack_tests();

    static mut errno: c_int;
}

unsafe extern "C" {
    fn ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_long;
}

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

/* do not optimize cfi related test functions */
/* Original C used:
 * #pragma GCC push_options
 * #pragma GCC optimize("O0")
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigsegv_handler(
    _signum: c_int,
    si: *mut siginfo_t,
    uc: *mut c_void,
) {
    let ctx = uc as *mut ucontext;

    if unsafe { (*si).si_code == SEGV_CPERR } {
        unsafe {
            ksft_print_msg(c"Control flow violation happened somewhere\n".as_ptr());
            ksft_print_msg(
                c"PC where violation happened %lx\n".as_ptr(),
                (*ctx).uc_mcontext.gregs[0],
            );
            exit(-1);
        }
    }

    /* all other cases are expected to be of shadow stack write case */
    unsafe {
        exit(CHILD_EXIT_CODE_SSWRITE);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_signal_handler() -> bool_t {
    let sa = sigaction {
        sa_sigaction: sigsegv_handler,
        sa_flags: SA_SIGINFO,
    };

    if unsafe { sigaction(SIGSEGV, &sa, core::ptr::null_mut()) } != 0 {
        unsafe {
            ksft_print_msg(
                c"Registering signal handler for landing pad violation failed\n".as_ptr(),
            );
        }
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfi_ptrace_test() -> bool_t {
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut ret: c_int = 0;
    let mut ptrace_test_num: c_ulong = 0;
    let total_ptrace_tests: c_ulong = 2;

    let mut cfi_reg = user_cfi_state {
        cfi_status: user_cfi_status { cfi_state: 0 },
        shstk_ptr: 0,
    };
    let mut iov = iovec {
        iov_base: core::ptr::null_mut(),
        iov_len: 0,
    };

    pid = unsafe { fork() };

    if pid == -1 {
        unsafe {
            ksft_exit_fail_msg(c"%s: fork failed\n".as_ptr(), c"cfi_ptrace_test".as_ptr());
        }
    }

    if pid == 0 {
        /* allow to be traced */
        unsafe {
            ptrace(
                PTRACE_TRACEME,
                0,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            raise(SIGSTOP);
            core::arch::asm!(
                "la a5, 2f",
                "jalr a5",
                "nop",
                "nop",
                "2: nop",
                lateout("a5") _,
                options(nostack)
            );
            exit(11);
        }
        /* child shouldn't go beyond here */
    }

    /* parent's code goes here */
    iov.iov_base = (&mut cfi_reg as *mut user_cfi_state).cast::<c_void>();
    iov.iov_len = core::mem::size_of::<user_cfi_state>();

    while ptrace_test_num < total_ptrace_tests {
        unsafe {
            memset(
                (&mut cfi_reg as *mut user_cfi_state).cast::<c_void>(),
                0,
                core::mem::size_of::<user_cfi_state>(),
            );
            waitpid(pid, &mut status, 0);
        }
        if WIFSTOPPED(status) {
            unsafe {
                errno = 0;
                ret = ptrace(
                    PTRACE_GETREGSET,
                    pid,
                    NT_RISCV_USER_CFI as *mut c_void,
                    (&mut iov as *mut iovec).cast::<c_void>(),
                ) as c_int;
                if ret == -1 && errno != 0 {
                    ksft_exit_fail_msg(
                        c"%s: PTRACE_GETREGSET failed\n".as_ptr(),
                        c"cfi_ptrace_test".as_ptr(),
                    );
                }
            }
        } else {
            unsafe {
                ksft_exit_fail_msg(
                    c"%s: child didn't stop, failed\n".as_ptr(),
                    c"cfi_ptrace_test".as_ptr(),
                );
            }
        }

        match ptrace_test_num {
            0 => {
                if (cfi_reg.cfi_status.cfi_state & CFI_ENABLE_MASK) != CFI_ENABLE_MASK {
                    unsafe {
                        ksft_exit_fail_msg(
                            c"%s: ptrace_getregset failed, %llu\n".as_ptr(),
                            c"cfi_ptrace_test".as_ptr(),
                            cfi_reg.cfi_status.cfi_state,
                        );
                    }
                }
                if cfi_reg.shstk_ptr == 0 {
                    unsafe {
                        ksft_exit_fail_msg(
                            c"%s: NULL shadow stack pointer, test failed\n".as_ptr(),
                            c"cfi_ptrace_test".as_ptr(),
                        );
                    }
                }
            }
            1 => {
                if (cfi_reg.cfi_status.cfi_state
                    & PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE)
                    == 0
                {
                    unsafe {
                        ksft_exit_fail_msg(
                            c"%s: elp must have been set\n".as_ptr(),
                            c"cfi_ptrace_test".as_ptr(),
                        );
                    }
                }
                /* clear elp state. not interested in anything else */
                cfi_reg.cfi_status.cfi_state = 0;

                unsafe {
                    ret = ptrace(
                        PTRACE_SETREGSET,
                        pid,
                        NT_RISCV_USER_CFI as *mut c_void,
                        (&mut iov as *mut iovec).cast::<c_void>(),
                    ) as c_int;
                    if ret == -1 && errno != 0 {
                        ksft_exit_fail_msg(
                            c"%s: PTRACE_GETREGSET failed\n".as_ptr(),
                            c"cfi_ptrace_test".as_ptr(),
                        );
                    }
                }
            }
            _ => unsafe {
                ksft_exit_fail_msg(
                    c"%s: unreachable switch case\n".as_ptr(),
                    c"cfi_ptrace_test".as_ptr(),
                );
            },
        }
        unsafe {
            ptrace(
                PTRACE_CONT,
                pid,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        }
        ptrace_test_num += 1;
    }

    unsafe {
        waitpid(pid, &mut status, 0);
    }
    if WEXITSTATUS(status) != 11 {
        unsafe {
            ksft_print_msg(
                c"%s, bad return code from child\n".as_ptr(),
                c"cfi_ptrace_test".as_ptr(),
            );
        }
    }

    unsafe {
        ksft_print_msg(
            c"%s, ptrace test succeeded\n".as_ptr(),
            c"cfi_ptrace_test".as_ptr(),
        );
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut lpad_status: c_ulong = 0;
    let mut ss_status: c_ulong = 0;

    unsafe {
        ksft_print_header();

        ksft_print_msg(c"Starting risc-v tests\n".as_ptr());
    }

    /* Test unknown PR_CFI bits */
    ret = unsafe {
        my_syscall5(
            __NR_prctl,
            PR_SET_CFI,
            PR_CFI_BRANCH_LANDING_PADS,
            PR_CFI_ENABLE | 0xffff0,
            0,
            0,
        )
    };
    if ret == 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"PR_SET_CFI accepted reserved branch landing pad bits\n".as_ptr(),
            );
        }
    }

    /*
     * Landing pad test. Not a lot of kernel changes to support landing
     * pads for user mode except lighting up a bit in senvcfg via a prctl.
     * Enable landing pad support throughout the execution of the test binary.
     */
    ret = unsafe {
        my_syscall5(
            __NR_prctl,
            PR_GET_CFI,
            PR_CFI_BRANCH_LANDING_PADS,
            (&mut lpad_status as *mut c_ulong) as c_ulong,
            0,
            0,
        )
    };
    if ret != 0 {
        unsafe {
            ksft_exit_fail_msg(c"Get landing pad status failed with %d\n".as_ptr(), ret);
        }
    }

    if (lpad_status & PR_CFI_ENABLE) == 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"Landing pad is not enabled, should be enabled via glibc\n".as_ptr(),
            );
        }
    }

    ret = unsafe {
        my_syscall5(
            __NR_prctl,
            PR_GET_SHADOW_STACK_STATUS,
            (&mut ss_status as *mut c_ulong) as c_long,
            0,
            0,
            0,
        )
    };
    if ret != 0 {
        unsafe {
            ksft_exit_fail_msg(c"Get shadow stack failed with %d\n".as_ptr(), ret);
        }
    }

    if (ss_status & PR_SHADOW_STACK_ENABLE) == 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"Shadow stack is not enabled, should be enabled via glibc\n".as_ptr(),
            );
        }
    }

    if !unsafe { register_signal_handler() } {
        unsafe {
            ksft_exit_fail_msg(c"Registering signal handler for SIGSEGV failed\n".as_ptr());
        }
    }

    unsafe {
        ksft_print_msg(c"Landing pad and shadow stack are enabled for binary\n".as_ptr());
        cfi_ptrace_test();

        execute_shadow_stack_tests();
    }

    0
}

/* Original C used: #pragma GCC pop_options */
