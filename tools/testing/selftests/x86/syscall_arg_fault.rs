// SPDX-License-Identifier: GPL-2.0-only
/*
 * syscall_arg_fault.c - tests faults 32-bit fast syscall stack args
 * Copyright (c) 2015 Andrew Lutomirski
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type sig_atomic_t = c_int;
type size_t = usize;

#[repr(C)]
struct sigjmp_buf {
    __private: [c_long; 64],
}

#[repr(C)]
struct stack_t {
    ss_sp: *mut c_void,
    ss_flags: c_int,
    ss_size: size_t,
}

#[repr(C)]
struct siginfo_t {
    __private: [u8; 0],
}

#[repr(C)]
struct mcontext_t {
    gregs: [c_long; 32],
}

#[repr(C)]
struct ucontext_t {
    uc_flags: c_ulong,
    uc_link: *mut ucontext_t,
    uc_stack: stack_t,
    uc_mcontext: mcontext_t,
}

type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

const SIGSEGV: c_int = 11;
const SIGBUS: c_int = 7;
const SIGILL: c_int = 4;
const SIGTRAP: c_int = 5;
const SA_ONSTACK: c_int = 0x08000000;
const SIGSTKSZ: usize = 8192;
const EFAULT: c_int = 14;
const ENOSYS: c_int = 38;

#[cfg(target_arch = "x86_64")]
const REG_AX: usize = 13;
#[cfg(target_arch = "x86_64")]
const REG_IP: usize = 16;

#[cfg(not(target_arch = "x86_64"))]
const REG_AX: usize = 11;
#[cfg(not(target_arch = "x86_64"))]
const REG_IP: usize = 14;

static mut jmpbuf: sigjmp_buf = sigjmp_buf { __private: [0; 64] };

static mut n_errs: sig_atomic_t = 0;

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(format: *const c_char, ...) -> c_int;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;

    /* helpers.h */
    fn sethandler(sig: c_int, handler: sighandler_t, flags: c_int);
    fn get_eflags() -> c_ulong;
    fn set_eflags(eflags: c_ulong);
    static X86_EFLAGS_TF: c_ulong;
}

unsafe extern "C" fn sigsegv_or_sigbus(
    _sig: c_int,
    _info: *mut siginfo_t,
    ctx_void: *mut c_void,
) {
    let ctx: *mut ucontext_t = ctx_void as *mut ucontext_t;
    let ax: c_long = (*ctx).uc_mcontext.gregs[REG_AX] as c_long;

    if ax != -(EFAULT as c_long) && ax != -(ENOSYS as c_long) {
        printf(
            c"[FAIL]\tAX had the wrong value: 0x%lx\n".as_ptr(),
            ax as c_ulong,
        );
        printf(
            c"\tIP = 0x%lx\n".as_ptr(),
            (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
        );
        n_errs += 1;
    } else {
        printf(c"[OK]\tSeems okay\n".as_ptr());
    }

    siglongjmp(&raw mut jmpbuf, 1);
}

static mut sigtrap_consecutive_syscalls: sig_atomic_t = 0;

unsafe extern "C" fn sigtrap(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    /*
     * KVM has some bugs that can cause us to stop making progress.
     * detect them and complain, but don't infinite loop or fail the
     * test.
     */

    let ctx: *mut ucontext_t = ctx_void as *mut ucontext_t;
    let ip: *mut u16 = (*ctx).uc_mcontext.gregs[REG_IP] as *mut u16;

    if *ip == 0x340f || *ip == 0x050f {
        /* The trap was on SYSCALL or SYSENTER */
        sigtrap_consecutive_syscalls += 1;
        if sigtrap_consecutive_syscalls > 3 {
            printf(
                c"[WARN]\tGot stuck single-stepping -- you probably have a KVM bug\n".as_ptr(),
            );
            siglongjmp(&raw mut jmpbuf, 1);
        }
    } else {
        sigtrap_consecutive_syscalls = 0;
    }
}

unsafe extern "C" fn sigill(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut ucontext_t = ctx_void as *mut ucontext_t;
    let ip: *mut u16 = (*ctx).uc_mcontext.gregs[REG_IP] as *mut u16;

    if *ip == 0x0b0f {
        /* one of the ud2 instructions faulted */
        printf(c"[OK]\tSYSCALL returned normally\n".as_ptr());
    } else {
        printf(c"[SKIP]\tIllegal instruction\n".as_ptr());
    }
    siglongjmp(&raw mut jmpbuf, 1);
}

unsafe fn invalid_state_sysenter() {
    asm!(
        "mov eax, -1",
        "mov ebx, -1",
        "mov ecx, -1",
        "mov edx, -1",
        "mov esi, -1",
        "mov edi, -1",
        "mov ebp, -1",
        "mov esp, -1",
        "sysenter",
        options(nostack, preserves_flags),
    );
}

unsafe fn invalid_state_syscall_ud2() {
    asm!(
        "mov eax, -1",
        "mov ebx, -1",
        "mov ecx, -1",
        "mov edx, -1",
        "mov esi, -1",
        "mov edi, -1",
        "mov ebp, -1",
        "mov esp, -1",
        "syscall",
        "ud2",
        /* make sure we recover cleanly */
        options(nostack, preserves_flags),
    );
}

fn main() {
    unsafe {
        let stack = stack_t {
            /* Our sigaltstack scratch space. */
            ss_sp: malloc(core::mem::size_of::<c_char>() * SIGSTKSZ),
            ss_flags: 0,
            ss_size: SIGSTKSZ,
        };
        if sigaltstack(&stack, core::ptr::null_mut()) != 0 {
            err(1, c"sigaltstack".as_ptr());
        }

        sethandler(SIGSEGV, sigsegv_or_sigbus, SA_ONSTACK);
        /*
         * The actual exception can vary.  On Atom CPUs, we get #SS
         * instead of #PF when the vDSO fails to access the stack when
         * ESP is too close to 2^32, and #SS causes SIGBUS.
         */
        sethandler(SIGBUS, sigsegv_or_sigbus, SA_ONSTACK);
        sethandler(SIGILL, sigill, SA_ONSTACK);

        /*
         * Exercise another nasty special case.  The 32-bit SYSCALL
         * and SYSENTER instructions (even in compat mode) each
         * clobber one register.  A Linux system call has a syscall
         * number and six arguments, and the user stack pointer
         * needs to live in some register on return.  That means
         * that we need eight registers, but SYSCALL and SYSENTER
         * only preserve seven registers.  As a result, one argument
         * ends up on the stack.  The stack is user memory, which
         * means that the kernel can fail to read it.
         *
         * The 32-bit fast system calls don't have a defined ABI:
         * we're supposed to invoke them through the vDSO.  So we'll
         * fudge it: we set all regs to invalid pointer values and
         * invoke the entry instruction.  The return will fail no
         * matter what, and we completely lose our program state,
         * but we can fix it up with a signal handler.
         */

        printf(c"[RUN]\tSYSENTER with invalid state\n".as_ptr());
        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            invalid_state_sysenter();
        }

        printf(c"[RUN]\tSYSCALL with invalid state\n".as_ptr());
        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            invalid_state_syscall_ud2();
        }

        printf(c"[RUN]\tSYSENTER with TF and invalid state\n".as_ptr());
        sethandler(SIGTRAP, sigtrap, SA_ONSTACK);

        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            sigtrap_consecutive_syscalls = 0;
            set_eflags(get_eflags() | X86_EFLAGS_TF);
            invalid_state_sysenter();
        }
        set_eflags(get_eflags() & !X86_EFLAGS_TF);

        printf(c"[RUN]\tSYSCALL with TF and invalid state\n".as_ptr());
        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            sigtrap_consecutive_syscalls = 0;
            set_eflags(get_eflags() | X86_EFLAGS_TF);
            invalid_state_syscall_ud2();
        }
        set_eflags(get_eflags() & !X86_EFLAGS_TF);

        #[cfg(target_arch = "x86_64")]
        {
            printf(c"[RUN]\tSYSENTER with TF, invalid state, and GSBASE < 0\n".as_ptr());

            if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
                sigtrap_consecutive_syscalls = 0;

                asm!("wrgsbase rax", in("rax") 0xffffffffffff0000_u64);

                set_eflags(get_eflags() | X86_EFLAGS_TF);
                invalid_state_sysenter();
            }
            set_eflags(get_eflags() & !X86_EFLAGS_TF);
        }

        free(stack.ss_sp);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
