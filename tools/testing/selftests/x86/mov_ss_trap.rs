/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mov_ss_trap.c: Exercise the bizarre side effects of a watchpoint on MOV SS
 *
 * This does MOV SS from a watchpointed address followed by various
 * types of kernel entries.  A MOV SS that hits a watchpoint will queue
 * up a #DB trap but will not actually deliver that trap.  The trap
 * will be delivered after the next instruction instead.  The CPU's logic
 * seems to be:
 *
 *  - Any fault: drop the pending #DB trap.
 *  - INT $N, INT3, INTO, SYSCALL, SYSENTER: enter the kernel and then
 *    deliver #DB.
 *  - ICEBP: enter the kernel but do not deliver the watchpoint trap
 *  - breakpoint: only one #DB is delivered (phew!)
 *
 * There are plenty of ways for a kernel to handle this incorrectly.  This
 * test tries to exercise all the cases.
 *
 * This should mostly cover CVE-2018-1087 and CVE-2018-8897.
 */

use core::arch::asm;
use core::ffi::{c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

#[cfg(target_arch = "x86_64")]
const REG_IP: usize = libc::REG_RIP as usize;
#[cfg(not(target_arch = "x86_64"))]
const REG_IP: usize = libc::REG_EIP as usize;

static mut ss: u16 = 0;

unsafe extern "C" {
    static breakpoint_insn: u8;
}

type sigjmp_buf = [libc::c_long; 32];
static mut jmpbuf: sigjmp_buf = [0; 32];

unsafe extern "C" {
    fn sethandler(
        sig: c_int,
        handler: unsafe extern "C" fn(c_int, *mut libc::siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn __sigsetjmp(env: *mut sigjmp_buf, savemask: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;
}

unsafe fn enable_watchpoint() {
    let parent: libc::pid_t = libc::getpid();
    let mut status: c_int = 0;

    let child: libc::pid_t = libc::fork();
    if child < 0 {
        libc::err(1, c"fork".as_ptr());
    }

    if child != 0 {
        if libc::waitpid(child, &mut status, 0) != child {
            libc::err(1, c"waitpid for child".as_ptr());
        }
    } else {
        let dr0: c_ulong = (&raw mut ss) as c_ulong;
        let dr1: c_ulong = (&raw const breakpoint_insn) as c_ulong;
        let dr7: c_ulong = ((1u64 << 1) |     /* G0 */
                            (3u64 << 16) |    /* RW0 = read or write */
                            (1u64 << 18) |    /* LEN0 = 2 bytes */
                            (1u64 << 3)) as c_ulong; /* G1, RW1 = insn */

        if libc::ptrace(libc::PTRACE_ATTACH, parent, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
            libc::err(1, c"PTRACE_ATTACH".as_ptr());
        }

        if libc::waitpid(parent, &mut status, 0) != parent {
            libc::err(1, c"waitpid for child".as_ptr());
        }

        if libc::ptrace(
            libc::PTRACE_POKEUSER,
            parent,
            memoffset::offset_of!(libc::user, u_debugreg[0]) as *mut c_void,
            dr0 as *mut c_void,
        ) != 0 {
            libc::err(1, c"PTRACE_POKEUSER DR0".as_ptr());
        }

        if libc::ptrace(
            libc::PTRACE_POKEUSER,
            parent,
            memoffset::offset_of!(libc::user, u_debugreg[1]) as *mut c_void,
            dr1 as *mut c_void,
        ) != 0 {
            libc::err(1, c"PTRACE_POKEUSER DR1".as_ptr());
        }

        if libc::ptrace(
            libc::PTRACE_POKEUSER,
            parent,
            memoffset::offset_of!(libc::user, u_debugreg[7]) as *mut c_void,
            dr7 as *mut c_void,
        ) != 0 {
            libc::err(1, c"PTRACE_POKEUSER DR7".as_ptr());
        }

        libc::printf(c"\tDR0 = %lx, DR1 = %lx, DR7 = %lx\n".as_ptr(), dr0, dr1, dr7);

        if libc::ptrace(libc::PTRACE_DETACH, parent, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
            libc::err(1, c"PTRACE_DETACH".as_ptr());
        }

        libc::exit(0);
    }
}

static signames: [*const libc::c_char; 32] = {
    let mut names = [ptr::null(); 32];
    names[libc::SIGSEGV as usize] = c"SIGSEGV".as_ptr();
    names[libc::SIGBUS as usize] = c"SIBGUS".as_ptr();
    names[libc::SIGTRAP as usize] = c"SIGTRAP".as_ptr();
    names[libc::SIGILL as usize] = c"SIGILL".as_ptr();
    names
};

unsafe extern "C" fn sigtrap(_sig: c_int, _si: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut libc::ucontext_t = ctx_void.cast();

    libc::printf(
        c"\tGot SIGTRAP with RIP=%lx, EFLAGS.RF=%d\n".as_ptr(),
        (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
        (((*ctx).uc_mcontext.gregs[libc::REG_EFL as usize] & X86_EFLAGS_RF as libc::greg_t) != 0) as c_int,
    );
}

unsafe extern "C" fn handle_and_return(sig: c_int, _si: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut libc::ucontext_t = ctx_void.cast();

    libc::printf(
        c"\tGot %s with RIP=%lx\n".as_ptr(),
        signames[sig as usize],
        (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
    );
}

unsafe extern "C" fn handle_and_longjmp(sig: c_int, _si: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut libc::ucontext_t = ctx_void.cast();

    libc::printf(
        c"\tGot %s with RIP=%lx\n".as_ptr(),
        signames[sig as usize],
        (*ctx).uc_mcontext.gregs[REG_IP] as c_ulong,
    );

    siglongjmp(&raw mut jmpbuf, 1);
}

const X86_EFLAGS_RF: c_ulong = 0x0001_0000;

unsafe fn sigsetjmp(env: *mut sigjmp_buf, savemask: c_int) -> c_int {
    __sigsetjmp(env, savemask)
}

unsafe fn main_impl() -> c_int {
    let mut nr: c_ulong;

    asm!("mov {0:x}, ss", out(reg) ss, options(att_syntax));
    libc::printf(c"\tSS = 0x%hx, &SS = 0x%p\n".as_ptr(), ss as c_int, (&raw mut ss).cast::<c_void>());

    if libc::prctl(libc::PR_SET_PTRACER, libc::PR_SET_PTRACER_ANY, 0, 0, 0) == 0 {
        libc::printf(c"\tPR_SET_PTRACER_ANY succeeded\n".as_ptr());
    }

    libc::printf(c"\tSet up a watchpoint\n".as_ptr());
    sethandler(libc::SIGTRAP, sigtrap, 0);
    enable_watchpoint();

    libc::printf(c"[RUN]\tRead from watched memory (should get SIGTRAP)\n".as_ptr());
    asm!("mov {tmp}, {ss}", tmp = out(reg) nr, ss = sym ss, options(att_syntax));

    libc::printf(c"[RUN]\tMOV SS; INT3\n".as_ptr());
    asm!("mov {ss}, %ss; int3", ss = sym ss, options(att_syntax));

    libc::printf(c"[RUN]\tMOV SS; INT 3\n".as_ptr());
    asm!("mov {ss}, %ss; .byte 0xcd, 0x3", ss = sym ss, options(att_syntax));

    libc::printf(c"[RUN]\tMOV SS; CS CS INT3\n".as_ptr());
    asm!("mov {ss}, %ss; .byte 0x2e, 0x2e; int3", ss = sym ss, options(att_syntax));

    libc::printf(c"[RUN]\tMOV SS; CSx14 INT3\n".as_ptr());
    asm!("mov {ss}, %ss; .fill 14,1,0x2e; int3", ss = sym ss, options(att_syntax));

    libc::printf(c"[RUN]\tMOV SS; INT 4\n".as_ptr());
    sethandler(libc::SIGSEGV, handle_and_return, libc::SA_RESETHAND);
    asm!("mov {ss}, %ss; int $4", ss = sym ss, options(att_syntax));

    #[cfg(target_arch = "x86")]
    {
        libc::printf(c"[RUN]\tMOV SS; INTO\n".as_ptr());
        sethandler(libc::SIGSEGV, handle_and_return, libc::SA_RESETHAND);
        nr = c_ulong::MAX;
        asm!(
            "add $1, {tmp}; mov {ss}, %ss; into",
            tmp = inout(reg) nr,
            ss = sym ss,
            options(att_syntax),
        );
    }

    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; ICEBP\n".as_ptr());

        /* Some emulators (e.g. QEMU TCG) don't emulate ICEBP. */
        sethandler(libc::SIGILL, handle_and_longjmp, libc::SA_RESETHAND);

        asm!("mov {ss}, %ss; .byte 0xf1", ss = sym ss, options(att_syntax));
    }

    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; CLI\n".as_ptr());
        sethandler(libc::SIGSEGV, handle_and_longjmp, libc::SA_RESETHAND);
        asm!("mov {ss}, %ss; cli", ss = sym ss, options(att_syntax));
    }

    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; #PF\n".as_ptr());
        sethandler(libc::SIGSEGV, handle_and_longjmp, libc::SA_RESETHAND);
        asm!("mov {ss}, %ss; mov (-1), {tmp}", tmp = out(reg) nr, ss = sym ss, options(att_syntax));
    }

    /*
     * INT $1: if #DB has DPL=3 and there isn't special handling,
     * then the kernel will die.
     */
    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; INT 1\n".as_ptr());
        sethandler(libc::SIGSEGV, handle_and_longjmp, libc::SA_RESETHAND);
        asm!("mov {ss}, %ss; int $1", ss = sym ss, options(att_syntax));
    }

    #[cfg(target_arch = "x86_64")]
    {
        /*
         * In principle, we should test 32-bit SYSCALL as well, but
         * the calling convention is so unpredictable that it's
         * not obviously worth the effort.
         */
        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            libc::printf(c"[RUN]\tMOV SS; SYSCALL\n".as_ptr());
            sethandler(libc::SIGILL, handle_and_longjmp, libc::SA_RESETHAND);
            nr = libc::SYS_getpid as c_ulong;
            /*
             * Toggle the high bit of RSP to make it noncanonical to
             * strengthen this test on non-SMAP systems.
             */
            asm!(
                "btc $63, %rsp\n\tmov {ss}, %ss; syscall\n\tbtc $63, %rsp",
                inout("rax") nr,
                ss = sym ss,
                out("rcx") _,
                out("r11") _,
                options(att_syntax),
            );
        }
    }

    libc::printf(c"[RUN]\tMOV SS; breakpointed NOP\n".as_ptr());
    asm!(".global breakpoint_insn\nmov {ss}, %ss; breakpoint_insn: nop", ss = sym ss, options(att_syntax));

    /*
     * Invoking SYSENTER directly breaks all the rules.  Just handle
     * the SIGSEGV.
     */
    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; SYSENTER\n".as_ptr());
        let mut stack = libc::stack_t {
            ss_sp: libc::malloc(mem::size_of::<libc::c_char>() * libc::SIGSTKSZ),
            ss_flags: 0,
            ss_size: libc::SIGSTKSZ,
        };
        if libc::sigaltstack(&stack, ptr::null_mut()) != 0 {
            libc::err(1, c"sigaltstack".as_ptr());
        }
        sethandler(libc::SIGSEGV, handle_and_longjmp, libc::SA_RESETHAND | libc::SA_ONSTACK);
        nr = libc::SYS_getpid as c_ulong;
        libc::free(stack.ss_sp);
        /* Clear EBP first to make sure we segfault cleanly. */
        asm!(
            "xorl %ebp, %ebp; mov {ss}, %ss; SYSENTER",
            inout("rax") nr,
            ss = sym ss,
            out("rcx") _,
            #[cfg(target_arch = "x86_64")]
            out("r11") _,
            options(att_syntax),
        );

        /* We're unreachable here.  SYSENTER forgets RIP. */
    }

    if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
        libc::printf(c"[RUN]\tMOV SS; INT $0x80\n".as_ptr());
        sethandler(libc::SIGSEGV, handle_and_longjmp, libc::SA_RESETHAND);
        nr = 20; /* compat getpid */
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mov {ss}, %ss; int $0x80",
            inout("rax") nr,
            ss = sym ss,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            options(att_syntax),
        );
        #[cfg(not(target_arch = "x86_64"))]
        asm!(
            "mov {ss}, %ss; int $0x80",
            inout("eax") nr,
            ss = sym ss,
            options(att_syntax),
        );
    }

    libc::printf(c"[OK]\tI aten't dead\n".as_ptr());
    0
}

fn main() {
    unsafe {
        main_impl();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
