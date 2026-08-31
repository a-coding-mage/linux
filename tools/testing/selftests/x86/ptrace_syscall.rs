// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included Linux/x86 ptrace, syscall, signal,
// wait, auxv, and local helpers.h declarations.

use core::arch::asm;
use core::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

use libc::{
    _exit, err, errx, fork, getauxval, getpid, kill, pid_t, ptrace, sigaction, sigemptyset,
    siginfo_t, syscall, waitid, waitpid, CLD_TRAPPED, P_PID, SA_RESTART, SIGKILL, SIGSTOP,
    SIGUSR1, SIG_IGN, SYS_getpid, SYS_gettid, SYS_pause, SYS_tgkill, WEXITED, WSTOPPED,
};

extern "C" {
    fn sethandler(
        sig: c_int,
        handler: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
}

// Bitness-agnostic accessors for user_regs_struct fields.
#[cfg(target_arch = "x86_64")]
unsafe fn user_syscall_nr(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).orig_rax
}
#[cfg(target_arch = "x86")]
unsafe fn user_syscall_nr(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).orig_eax as c_ulong
}
#[cfg(target_arch = "x86_64")]
unsafe fn set_user_ax(regs: *mut libc::user_regs_struct, val: c_ulong) {
    (*regs).rax = val;
}
#[cfg(target_arch = "x86")]
unsafe fn set_user_ax(regs: *mut libc::user_regs_struct, val: c_ulong) {
    (*regs).eax = val as _;
}
#[cfg(target_arch = "x86_64")]
unsafe fn user_ax(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).rax
}
#[cfg(target_arch = "x86")]
unsafe fn user_ax(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).eax as c_ulong
}
#[cfg(target_arch = "x86_64")]
unsafe fn user_ip(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).rip
}
#[cfg(target_arch = "x86")]
unsafe fn user_ip(regs: *const libc::user_regs_struct) -> c_ulong {
    (*regs).eip as c_ulong
}
#[cfg(target_arch = "x86_64")]
unsafe fn sub_user_ip(regs: *mut libc::user_regs_struct, val: c_ulong) {
    (*regs).rip = (*regs).rip.wrapping_sub(val);
}
#[cfg(target_arch = "x86")]
unsafe fn sub_user_ip(regs: *mut libc::user_regs_struct, val: c_ulong) {
    (*regs).eip = ((*regs).eip as c_ulong).wrapping_sub(val) as _;
}

#[cfg(target_arch = "x86_64")]
unsafe fn user_args(regs: *const libc::user_regs_struct) -> [c_ulong; 6] {
    [
        (*regs).rdi,
        (*regs).rsi,
        (*regs).rdx,
        (*regs).r10,
        (*regs).r8,
        (*regs).r9,
    ]
}
#[cfg(target_arch = "x86")]
unsafe fn user_args(regs: *const libc::user_regs_struct) -> [c_ulong; 6] {
    [
        (*regs).ebx as c_ulong,
        (*regs).ecx as c_ulong,
        (*regs).edx as c_ulong,
        (*regs).esi as c_ulong,
        (*regs).edi as c_ulong,
        (*regs).ebp as c_ulong,
    ]
}
#[cfg(target_arch = "x86_64")]
unsafe fn set_user_args(regs: *mut libc::user_regs_struct, vals: [c_ulong; 6]) {
    (*regs).rdi = vals[0];
    (*regs).rsi = vals[1];
    (*regs).rdx = vals[2];
    (*regs).r10 = vals[3];
    (*regs).r8 = vals[4];
    (*regs).r9 = vals[5];
}
#[cfg(target_arch = "x86")]
unsafe fn set_user_args(regs: *mut libc::user_regs_struct, vals: [c_ulong; 6]) {
    (*regs).ebx = vals[0] as _;
    (*regs).ecx = vals[1] as _;
    (*regs).edx = vals[2] as _;
    (*regs).esi = vals[3] as _;
    (*regs).edi = vals[4] as _;
    (*regs).ebp = vals[5] as _;
}

static mut nerrs: c_int = 0;

#[repr(C)]
struct syscall_args32 {
    nr: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
}

#[cfg(target_arch = "x86")]
extern "C" {
    fn sys32_helper(args: *mut syscall_args32, func: *mut c_void);
    fn int80_and_ret();
}

/*
 * Helper to invoke int80 with controlled regs and capture the final regs.
 */
unsafe fn do_full_int80(args: *mut syscall_args32) {
    #[cfg(target_arch = "x86_64")]
    {
        let mut bp: c_ulong = (*args).arg5 as c_ulong;
        asm!(
            "int $0x80",
            inout("eax") (*args).nr,
            inout("ebx") (*args).arg0,
            inout("ecx") (*args).arg1,
            inout("edx") (*args).arg2,
            inout("esi") (*args).arg3,
            inout("edi") (*args).arg4,
            inout("rbp") bp,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
        );
        (*args).arg5 = bp as u32;
    }
    #[cfg(target_arch = "x86")]
    {
        sys32_helper(args, int80_and_ret as *mut c_void);
    }
}

#[cfg(target_arch = "x86")]
static mut vsyscall32: Option<unsafe extern "C" fn()> = None;

/*
 * Nasty helper to invoke AT_SYSINFO (i.e. __kernel_vsyscall) with
 * controlled regs and capture the final regs.  This is so nasty that it
 * crashes my copy of gdb :)
 */
#[cfg(target_arch = "x86")]
unsafe fn do_full_vsyscall32(args: *mut syscall_args32) {
    sys32_helper(args, mem::transmute(vsyscall32));
}

unsafe fn wait_trap(chld: pid_t) -> siginfo_t {
    let mut si: siginfo_t = mem::zeroed();
    if waitid(P_PID, chld as _, &mut si, WEXITED | WSTOPPED) != 0 {
        err(1, b"waitid\0".as_ptr() as *const _);
    }
    if si.si_pid() != chld {
        errx(1, b"got unexpected pid in event\n\0".as_ptr() as *const _);
    }
    if si.si_code != CLD_TRAPPED {
        errx(
            1,
            b"got unexpected event type %d\n\0".as_ptr() as *const _,
            si.si_code,
        );
    }
    si
}

unsafe fn setsigign(sig: c_int, flags: c_int) {
    let mut sa: sigaction = mem::zeroed();
    sa.sa_sigaction = SIG_IGN;
    sa.sa_flags = flags;
    sigemptyset(&mut sa.sa_mask);
    if sigaction(sig, &sa, ptr::null_mut()) != 0 {
        err(1, b"sigaction\0".as_ptr() as *const _);
    }
}

extern "C" fn empty_handler(_sig: c_int, _si: *mut siginfo_t, _ctx_void: *mut c_void) {}

unsafe fn test_sys32_regs(do_syscall: unsafe fn(*mut syscall_args32)) {
    let mut args = syscall_args32 {
        nr: 224, /* gettid */
        arg0: 10,
        arg1: 11,
        arg2: 12,
        arg3: 13,
        arg4: 14,
        arg5: 15,
    };

    do_syscall(&mut args);

    if args.nr != getpid() as u32
        || args.arg0 != 10
        || args.arg1 != 11
        || args.arg2 != 12
        || args.arg3 != 13
        || args.arg4 != 14
        || args.arg5 != 15
    {
        libc::printf(b"[FAIL]\tgetpid() failed to preserve regs\n\0".as_ptr() as *const _);
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tgetpid() preserves regs\n\0".as_ptr() as *const _);
    }

    sethandler(SIGUSR1, empty_handler, 0);

    args.nr = 37; /* kill */
    args.arg0 = getpid() as u32;
    args.arg1 = SIGUSR1 as u32;
    do_syscall(&mut args);
    if args.nr != 0
        || args.arg0 != getpid() as u32
        || args.arg1 != SIGUSR1 as u32
        || args.arg2 != 12
        || args.arg3 != 13
        || args.arg4 != 14
        || args.arg5 != 15
    {
        libc::printf(
            b"[FAIL]\tkill(getpid(), SIGUSR1) failed to preserve regs\n\0".as_ptr() as *const _,
        );
        nerrs += 1;
    } else {
        libc::printf(
            b"[OK]\tkill(getpid(), SIGUSR1) preserves regs\n\0".as_ptr() as *const _,
        );
    }
    clearhandler(SIGUSR1);
}

unsafe fn ptrace0(request: c_int, pid: pid_t) -> c_long {
    ptrace(request as _, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>())
}

unsafe fn test_ptrace_syscall_restart() {
    libc::printf(b"[RUN]\tptrace-induced syscall restart\n\0".as_ptr() as *const _);
    let chld = fork();
    if chld < 0 {
        err(1, b"fork\0".as_ptr() as *const _);
    }

    if chld == 0 {
        if ptrace(
            libc::PTRACE_TRACEME,
            0,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        ) != 0
        {
            err(1, b"PTRACE_TRACEME\0".as_ptr() as *const _);
        }

        let pid = getpid();
        let tid = syscall(SYS_gettid) as pid_t;

        libc::printf(b"\tChild will make one syscall\n\0".as_ptr() as *const _);
        syscall(SYS_tgkill, pid, tid, SIGSTOP);

        syscall(SYS_gettid, 10, 11, 12, 13, 14, 15);
        _exit(0);
    }

    let mut status: c_int = 0;

    /* Wait for SIGSTOP. */
    if waitpid(chld, &mut status, 0) != chld || !libc::WIFSTOPPED(status) {
        err(1, b"waitpid\0".as_ptr() as *const _);
    }

    let mut regs: libc::user_regs_struct = mem::zeroed();

    libc::printf(b"[RUN]\tSYSEMU\n\0".as_ptr() as *const _);
    if ptrace0(libc::PTRACE_SYSEMU, chld) != 0 {
        err(1, b"PTRACE_SYSEMU\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_gettid as c_ulong
        || a[0] != 10
        || a[1] != 11
        || a[2] != 12
        || a[3] != 13
        || a[4] != 14
        || a[5] != 15
    {
        libc::printf(
            b"[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tInitial nr and args are correct\n\0".as_ptr() as *const _);
    }

    libc::printf(
        b"[RUN]\tRestart the syscall (ip = 0x%lx)\n\0".as_ptr() as *const _,
        user_ip(&regs),
    );

    /*
     * This does exactly what it appears to do if syscall is int80 or
     * SYSCALL64.  For SYSCALL32 or SYSENTER, though, this is highly
     * magical.  It needs to work so that ptrace and syscall restart
     * work as expected.
     */
    set_user_ax(&mut regs, user_syscall_nr(&regs));
    sub_user_ip(&mut regs, 2);
    if ptrace(
        libc::PTRACE_SETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_SETREGS\0".as_ptr() as *const _);
    }

    if ptrace0(libc::PTRACE_SYSEMU, chld) != 0 {
        err(1, b"PTRACE_SYSEMU\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_gettid as c_ulong
        || a[0] != 10
        || a[1] != 11
        || a[2] != 12
        || a[3] != 13
        || a[4] != 14
        || a[5] != 15
    {
        libc::printf(
            b"[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tRestarted nr and args are correct\n\0".as_ptr() as *const _);
    }

    libc::printf(
        b"[RUN]\tChange nr and args and restart the syscall (ip = 0x%lx)\n\0".as_ptr()
            as *const _,
        user_ip(&regs),
    );

    set_user_ax(&mut regs, SYS_getpid as c_ulong);
    set_user_args(&mut regs, [20, 21, 22, 23, 24, 25]);
    sub_user_ip(&mut regs, 2);

    if ptrace(
        libc::PTRACE_SETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_SETREGS\0".as_ptr() as *const _);
    }

    if ptrace0(libc::PTRACE_SYSEMU, chld) != 0 {
        err(1, b"PTRACE_SYSEMU\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_getpid as c_ulong
        || a[0] != 20
        || a[1] != 21
        || a[2] != 22
        || a[3] != 23
        || a[4] != 24
        || a[5] != 25
    {
        libc::printf(
            b"[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tReplacement nr and args are correct\n\0".as_ptr() as *const _);
    }

    if ptrace0(libc::PTRACE_CONT, chld) != 0 {
        err(1, b"PTRACE_CONT\0".as_ptr() as *const _);
    }
    if waitpid(chld, &mut status, 0) != chld {
        err(1, b"waitpid\0".as_ptr() as *const _);
    }
    if !libc::WIFEXITED(status) || libc::WEXITSTATUS(status) != 0 {
        libc::printf(b"[FAIL]\tChild failed\n\0".as_ptr() as *const _);
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tChild exited cleanly\n\0".as_ptr() as *const _);
    }
}

unsafe fn test_restart_under_ptrace() {
    libc::printf(b"[RUN]\tkernel syscall restart under ptrace\n\0".as_ptr() as *const _);
    let chld = fork();
    if chld < 0 {
        err(1, b"fork\0".as_ptr() as *const _);
    }

    if chld == 0 {
        if ptrace(
            libc::PTRACE_TRACEME,
            0,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        ) != 0
        {
            err(1, b"PTRACE_TRACEME\0".as_ptr() as *const _);
        }

        let pid = getpid();
        let tid = syscall(SYS_gettid) as pid_t;

        libc::printf(b"\tChild will take a nap until signaled\n\0".as_ptr() as *const _);
        setsigign(SIGUSR1, SA_RESTART);
        syscall(SYS_tgkill, pid, tid, SIGSTOP);

        syscall(SYS_pause, 0, 0, 0, 0, 0, 0);
        _exit(0);
    }

    let mut status: c_int = 0;

    /* Wait for SIGSTOP. */
    if waitpid(chld, &mut status, 0) != chld || !libc::WIFSTOPPED(status) {
        err(1, b"waitpid\0".as_ptr() as *const _);
    }

    let mut regs: libc::user_regs_struct = mem::zeroed();

    libc::printf(b"[RUN]\tSYSCALL\n\0".as_ptr() as *const _);
    if ptrace0(libc::PTRACE_SYSCALL, chld) != 0 {
        err(1, b"PTRACE_SYSCALL\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    /* We should be stopped at pause(2) entry. */

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_pause as c_ulong
        || a[0] != 0
        || a[1] != 0
        || a[2] != 0
        || a[3] != 0
        || a[4] != 0
        || a[5] != 0
    {
        libc::printf(
            b"[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tInitial nr and args are correct\n\0".as_ptr() as *const _);
    }

    /* Interrupt it. */
    kill(chld, SIGUSR1);

    /* Advance.  We should be stopped at exit. */
    libc::printf(b"[RUN]\tSYSCALL\n\0".as_ptr() as *const _);
    if ptrace0(libc::PTRACE_SYSCALL, chld) != 0 {
        err(1, b"PTRACE_SYSCALL\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_pause as c_ulong
        || a[0] != 0
        || a[1] != 0
        || a[2] != 0
        || a[3] != 0
        || a[4] != 0
        || a[5] != 0
    {
        libc::printf(
            b"[FAIL]\tArgs after SIGUSR1 are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(
            b"[OK]\tArgs after SIGUSR1 are correct (ax = %ld)\n\0".as_ptr() as *const _,
            user_ax(&regs) as c_long,
        );
    }

    /* Poke the regs back in.  This must not break anything. */
    if ptrace(
        libc::PTRACE_SETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_SETREGS\0".as_ptr() as *const _);
    }

    /* Catch the (ignored) SIGUSR1. */
    if ptrace0(libc::PTRACE_CONT, chld) != 0 {
        err(1, b"PTRACE_CONT\0".as_ptr() as *const _);
    }
    if waitpid(chld, &mut status, 0) != chld {
        err(1, b"waitpid\0".as_ptr() as *const _);
    }
    if !libc::WIFSTOPPED(status) {
        libc::printf(
            b"[FAIL]\tChild was stopped for SIGUSR1 (status = 0x%x)\n\0".as_ptr() as *const _,
            status,
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tChild got SIGUSR1\n\0".as_ptr() as *const _);
    }

    /* The next event should be pause(2) again. */
    libc::printf(b"[RUN]\tStep again\n\0".as_ptr() as *const _);
    if ptrace0(libc::PTRACE_SYSCALL, chld) != 0 {
        err(1, b"PTRACE_SYSCALL\0".as_ptr() as *const _);
    }
    wait_trap(chld);

    /* We should be stopped at pause(2) entry. */

    if ptrace(
        libc::PTRACE_GETREGS,
        chld,
        ptr::null_mut::<c_void>(),
        &mut regs as *mut _ as *mut c_void,
    ) != 0
    {
        err(1, b"PTRACE_GETREGS\0".as_ptr() as *const _);
    }

    let a = user_args(&regs);
    if user_syscall_nr(&regs) != SYS_pause as c_ulong
        || a[0] != 0
        || a[1] != 0
        || a[2] != 0
        || a[3] != 0
        || a[4] != 0
        || a[5] != 0
    {
        libc::printf(
            b"[FAIL]\tpause did not restart (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n\0"
                .as_ptr() as *const _,
            user_syscall_nr(&regs),
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
        );
        nerrs += 1;
    } else {
        libc::printf(b"[OK]\tpause(2) restarted correctly\n\0".as_ptr() as *const _);
    }

    /* Kill it. */
    kill(chld, SIGKILL);
    if waitpid(chld, &mut status, 0) != chld {
        err(1, b"waitpid\0".as_ptr() as *const _);
    }
}

fn main() {
    unsafe {
        libc::printf(b"[RUN]\tCheck int80 return regs\n\0".as_ptr() as *const _);
        test_sys32_regs(do_full_int80);

        // Original condition:
        // #if defined(__i386__) && (!defined(__GLIBC__) || __GLIBC__ > 2 || __GLIBC_MINOR__ >= 16)
        #[cfg(target_arch = "x86")]
        {
            const AT_SYSINFO: c_ulong = 32;
            vsyscall32 = mem::transmute(getauxval(AT_SYSINFO));
            if vsyscall32.is_some() {
                libc::printf(b"[RUN]\tCheck AT_SYSINFO return regs\n\0".as_ptr() as *const _);
                test_sys32_regs(do_full_vsyscall32);
            } else {
                libc::printf(b"[SKIP]\tAT_SYSINFO is not available\n\0".as_ptr() as *const _);
            }
        }

        test_ptrace_syscall_restart();

        test_restart_under_ptrace();
    }
}
