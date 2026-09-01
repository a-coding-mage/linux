// SPDX-License-Identifier: GPL-2.0-only
/*
 * single_step_syscall.c - single-steps various x86 syscalls
 * Copyright (c) 2014-2015 Andrew Lutomirski
 *
 * This is a very simple series of tests that makes system calls with
 * the TF flag set.  This exercises some nasty kernel code in the
 * SYSENTER case: SYSENTER does not clear TF, so SYSENTER with TF set
 * immediately issues #DB from CPL 0.  This requires special handling in
 * the kernel.
 */

// C dependencies: sys/time.h, time.h, stdlib.h, sys/syscall.h, unistd.h,
// stdio.h, string.h, inttypes.h, sys/mman.h, sys/signal.h, sys/ucontext.h,
// asm/ldt.h, err.h, setjmp.h, stddef.h, stdbool.h, sys/ptrace.h, sys/user.h,
// and "helpers.h".

use core::arch::asm;
use core::ffi::{c_int, c_long, c_ulong, c_void};

type SigAtomicT = c_int;

#[cfg(target_arch = "x86_64")]
const REG_IP: usize = libc::REG_RIP as usize;
#[cfg(target_arch = "x86")]
const REG_IP: usize = libc::REG_EIP as usize;

static mut SIG_TRAPS: volatile SigAtomicT = 0;
static mut SIG_EFLAGS: volatile SigAtomicT = 0;

// External declarations supplied by libc and helpers.h.
unsafe extern "C" {
    static mut jmpbuf: libc::sigjmp_buf;

    fn get_eflags() -> c_ulong;
    fn set_eflags(eflags: c_ulong);
    fn sethandler(
        sig: c_int,
        handler: unsafe extern "C" fn(c_int, *mut libc::siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
    fn sigsetjmp(env: libc::sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: libc::sigjmp_buf, val: c_int) -> !;
    fn err(eval: c_int, fmt: *const libc::c_char, ...) -> !;
}

unsafe extern "C" fn sigtrap(sig: c_int, info: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void as *mut libc::ucontext_t;

    if (unsafe { get_eflags() } & X86_EFLAGS_TF) != 0 {
        unsafe {
            set_eflags(get_eflags() & !X86_EFLAGS_TF);
            libc::printf(c"[WARN]\tSIGTRAP handler had TF set\n".as_ptr());
            libc::_exit(1);
        }
    }

    unsafe {
        SIG_TRAPS += 1;
    }

    if unsafe { SIG_TRAPS == 10000 || SIG_TRAPS == 10001 } {
        unsafe {
            libc::printf(
                c"[WARN]\tHit %d SIGTRAPs with si_addr 0x%lx, ip 0x%lx\n".as_ptr(),
                SIG_TRAPS as c_int,
                (*info).si_addr() as c_ulong,
                (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
            );
        }
    }
}

static SIGNAMES: [*const libc::c_char; libc::SIGTRAP as usize + 1] = {
    let mut signames = [core::ptr::null(); libc::SIGTRAP as usize + 1];
    signames[libc::SIGSEGV as usize] = c"SIGSEGV".as_ptr();
    signames[libc::SIGBUS as usize] = c"SIBGUS".as_ptr();
    signames[libc::SIGTRAP as usize] = c"SIGTRAP".as_ptr();
    signames[libc::SIGILL as usize] = c"SIGILL".as_ptr();
    signames
};

unsafe extern "C" fn print_and_longjmp(
    sig: c_int,
    _si: *mut libc::siginfo_t,
    ctx_void: *mut c_void,
) {
    let ctx = ctx_void as *mut libc::ucontext_t;

    unsafe {
        libc::printf(
            c"\tGot %s with RIP=%lx, TF=%ld\n".as_ptr(),
            SIGNAMES[sig as usize],
            (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
            ((*ctx).uc_mcontext.gregs[libc::REG_EFL as usize] as c_ulong & X86_EFLAGS_TF)
                as c_ulong,
        );

        SIG_EFLAGS = (*ctx).uc_mcontext.gregs[libc::REG_EFL as usize] as c_ulong as SigAtomicT;
        siglongjmp(jmpbuf, 1);
    }
}

unsafe fn check_result() {
    let new_eflags = unsafe { get_eflags() };
    unsafe {
        set_eflags(new_eflags & !X86_EFLAGS_TF);
    }

    if unsafe { SIG_TRAPS == 0 } {
        unsafe {
            libc::printf(c"[FAIL]\tNo SIGTRAP\n".as_ptr());
            libc::exit(1);
        }
    }

    if (new_eflags & X86_EFLAGS_TF) == 0 {
        unsafe {
            libc::printf(c"[FAIL]\tTF was cleared\n".as_ptr());
            libc::exit(1);
        }
    }

    unsafe {
        libc::printf(
            c"[OK]\tSurvived with TF set and %d traps\n".as_ptr(),
            SIG_TRAPS as c_int,
        );
        SIG_TRAPS = 0;
    }
}

unsafe fn fast_syscall_no_tf() {
    unsafe {
        SIG_TRAPS = 0;
        libc::printf(c"[RUN]\tFast syscall with TF cleared\n".as_ptr());
        libc::fflush(libc::stdout); /* Force a syscall */
    }
    if (unsafe { get_eflags() } & X86_EFLAGS_TF) != 0 {
        unsafe {
            libc::printf(c"[FAIL]\tTF is now set\n".as_ptr());
            libc::exit(1);
        }
    }
    if unsafe { SIG_TRAPS != 0 } {
        unsafe {
            libc::printf(c"[FAIL]\tGot SIGTRAP\n".as_ptr());
            libc::exit(1);
        }
    }
    unsafe {
        libc::printf(c"[OK]\tNothing unexpected happened\n".as_ptr());
    }
}

const X86_EFLAGS_TF: c_ulong = 0x0000_0100;

fn main() {
    unsafe {
        #[cfg(CAN_BUILD_32)]
        let mut tmp: c_long;

        sethandler(libc::SIGTRAP, sigtrap, 0);

        libc::printf(c"[RUN]\tSet TF and check nop\n".as_ptr());
        set_eflags(get_eflags() | X86_EFLAGS_TF);
        asm!("nop", options(nostack, preserves_flags));
        check_result();

        #[cfg(target_arch = "x86_64")]
        {
            libc::printf(c"[RUN]\tSet TF and check syscall-less opportunistic sysret\n".as_ptr());
            set_eflags(get_eflags() | X86_EFLAGS_TF);
            unsafe extern "C" {
                static post_nop: [u8; 0];
            }
            asm!(
                "pushfq",
                "pop r11",
                "nop",
                "post_nop:",
                in("rcx") post_nop.as_ptr(),
                out("r11") _,
            );
            check_result();
        }

        // Original C condition: #ifdef CAN_BUILD_32.
        #[cfg(CAN_BUILD_32)]
        {
            libc::printf(c"[RUN]\tSet TF and check int80\n".as_ptr());
            set_eflags(get_eflags() | X86_EFLAGS_TF);
            #[cfg(target_arch = "x86_64")]
            asm!(
                "int $0x80",
                inlateout("rax") libc::SYS_getpid => tmp,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
            );
            #[cfg(target_arch = "x86")]
            asm!(
                "int $0x80",
                inlateout("eax") libc::SYS_getpid => tmp,
            );
            check_result();
        }

        /*
         * This test is particularly interesting if fast syscalls use
         * SYSENTER: it triggers a nasty design flaw in SYSENTER.
         * Specifically, SYSENTER does not clear TF, so either SYSENTER
         * or the next instruction traps at CPL0.  (Of course, Intel
         * mostly forgot to document exactly what happens here.)  So we
         * get a CPL0 fault with usergs (on 64-bit kernels) and possibly
         * no stack.  The only sane way the kernel can possibly handle
         * it is to clear TF on return from the #DB handler, but this
         * happens way too early to set TF in the saved pt_regs, so the
         * kernel has to do something clever to avoid losing track of
         * the TF bit.
         *
         * Needless to say, we've had bugs in this area.
         */
        libc::syscall(libc::SYS_getpid); /* Force symbol binding without TF set. */
        libc::printf(c"[RUN]\tSet TF and check a fast syscall\n".as_ptr());
        set_eflags(get_eflags() | X86_EFLAGS_TF);
        libc::syscall(libc::SYS_getpid);
        check_result();

        /* Now make sure that another fast syscall doesn't set TF again. */
        fast_syscall_no_tf();

        /*
         * And do a forced SYSENTER to make sure that this works even if
         * fast syscalls don't use SYSENTER.
         *
         * Invoking SYSENTER directly breaks all the rules.  Just handle
         * the SIGSEGV.
         */
        if sigsetjmp(jmpbuf, 1) == 0 {
            let mut nr: c_ulong = libc::SYS_getpid as c_ulong;
            libc::printf(c"[RUN]\tSet TF and check SYSENTER\n".as_ptr());
            let mut stack = libc::stack_t {
                ss_sp: libc::malloc(core::mem::size_of::<libc::c_char>() * libc::SIGSTKSZ),
                ss_flags: 0,
                ss_size: libc::SIGSTKSZ,
            };
            if libc::sigaltstack(&stack, core::ptr::null_mut()) != 0 {
                err(1, c"sigaltstack".as_ptr());
            }
            sethandler(
                libc::SIGSEGV,
                print_and_longjmp,
                libc::SA_RESETHAND | libc::SA_ONSTACK,
            );
            sethandler(libc::SIGILL, print_and_longjmp, libc::SA_RESETHAND);
            set_eflags(get_eflags() | X86_EFLAGS_TF);
            libc::free(stack.ss_sp);
            /* Clear EBP first to make sure we segfault cleanly. */
            #[cfg(target_arch = "x86_64")]
            asm!(
                "xorl %ebp, %ebp; SYSENTER",
                inout("rax") nr,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack),
            );
            #[cfg(target_arch = "x86")]
            asm!(
                "xorl %ebp, %ebp; SYSENTER",
                inout("eax") nr,
                lateout("ecx") _,
                options(nostack),
            );

            /* We're unreachable here.  SYSENTER forgets RIP. */
        }
        clearhandler(libc::SIGSEGV);
        clearhandler(libc::SIGILL);
        if (SIG_EFLAGS as c_ulong & X86_EFLAGS_TF) == 0 {
            libc::printf(c"[FAIL]\tTF was cleared\n".as_ptr());
            libc::exit(1);
        }

        /* Now make sure that another fast syscall doesn't set TF again. */
        fast_syscall_no_tf();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
