// SPDX-License-Identifier: GPL-2.0
/*
 * A ptrace test for testing PTRACE_SYSEMU, PTRACE_SETREGS and
 * PTRACE_GETREG.  This test basically create a child process that executes
 * syscalls and the parent process check if it is being traced appropriated.
 *
 * This test is heavily based on tools/testing/selftests/x86/ptrace_syscall.c
 * test, and it was adapted to run on Powerpc by
 * Breno Leitao <leitao@debian.org>
 */

/* C includes translated as external dependencies:
 * <sys/ptrace.h>, <sys/types.h>, <sys/wait.h>, <sys/syscall.h>,
 * <sys/user.h>, <unistd.h>, <errno.h>, <stddef.h>, <stdio.h>, <err.h>,
 * <string.h>, <sys/auxv.h>, and "utils.h".
 */

use libc::{c_char, c_int, c_long, c_uint, c_void, id_t, pid_t, siginfo_t};

/* Bitness-agnostic defines for user_regs_struct fields. */
/* user_syscall_nr -> gpr[0]
 * user_arg0       -> gpr[3]
 * user_arg1       -> gpr[4]
 * user_arg2       -> gpr[5]
 * user_arg3       -> gpr[6]
 * user_arg4       -> gpr[7]
 * user_arg5       -> gpr[8]
 * user_ip         -> nip
 */

const PTRACE_SYSEMU: c_uint = 0x1d;

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
struct pt_regs {
    gpr: [c_long; 32],
    nip: c_long,
}

static mut nerrs: c_int = 0;

unsafe fn wait_trap(chld: pid_t) {
    let mut si: siginfo_t = unsafe { core::mem::zeroed() };

    if unsafe {
        libc::waitid(
            libc::P_PID,
            chld as id_t,
            &mut si,
            libc::WEXITED | libc::WSTOPPED,
        )
    } != 0
    {
        unsafe { err(1, c"waitid".as_ptr()) };
    }
    if unsafe { si.si_pid() } != chld {
        unsafe { errx(1, c"got unexpected pid in event\n".as_ptr()) };
    }
    if unsafe { si.si_code() } != libc::CLD_TRAPPED {
        unsafe {
            errx(
                1,
                c"got unexpected event type %d\n".as_ptr(),
                si.si_code(),
            )
        };
    }
}

unsafe fn test_ptrace_syscall_restart() {
    let mut status: c_int = 0;
    let mut regs: pt_regs = unsafe { core::mem::zeroed() };
    let chld: pid_t;

    unsafe { printf(c"[RUN]\tptrace-induced syscall restart\n".as_ptr()) };

    chld = unsafe { libc::fork() };
    if chld < 0 {
        unsafe { err(1, c"fork".as_ptr()) };
    }

    /*
     * Child process is running 4 syscalls after ptrace.
     *
     * 1) getpid()
     * 2) gettid()
     * 3) tgkill() -> Send SIGSTOP
     * 4) gettid() -> Where the tests will happen essentially
     */
    if chld == 0 {
        if unsafe {
            libc::ptrace(
                libc::PTRACE_TRACEME,
                0,
                0 as *mut c_void,
                0 as *mut c_void,
            )
        } != 0
        {
            unsafe { err(1, c"PTRACE_TRACEME".as_ptr()) };
        }

        let pid: pid_t = unsafe { libc::getpid() };
        let tid: c_long = unsafe { libc::syscall(libc::SYS_gettid) };

        unsafe { printf(c"\tChild will make one syscall\n".as_ptr()) };
        unsafe { libc::syscall(libc::SYS_tgkill, pid, tid, libc::SIGSTOP) };

        unsafe { libc::syscall(libc::SYS_gettid, 10, 11, 12, 13, 14, 15) };
        unsafe { libc::_exit(0) };
    }
    /* Parent process below */

    /* Wait for SIGSTOP sent by tgkill above. */
    if unsafe { libc::waitpid(chld, &mut status, 0) } != chld || !libc::WIFSTOPPED(status) {
        unsafe { err(1, c"waitpid".as_ptr()) };
    }

    unsafe { printf(c"[RUN]\tSYSEMU\n".as_ptr()) };
    if unsafe {
        libc::ptrace(
            PTRACE_SYSEMU,
            chld,
            0 as *mut c_void,
            0 as *mut c_void,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_SYSEMU".as_ptr()) };
    }
    unsafe { wait_trap(chld) };

    if unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            chld,
            0 as *mut c_void,
            &mut regs as *mut pt_regs,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_GETREGS".as_ptr()) };
    }

    /*
     * Ptrace trapped prior to executing the syscall, thus r3 still has
     * the syscall number instead of the sys_gettid() result
     */
    if regs.gpr[0] != libc::SYS_gettid
        || regs.gpr[3] != 10
        || regs.gpr[4] != 11
        || regs.gpr[5] != 12
        || regs.gpr[6] != 13
        || regs.gpr[7] != 14
        || regs.gpr[8] != 15
    {
        unsafe {
            printf(
                c"[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n"
                    .as_ptr(),
                regs.gpr[0] as libc::c_ulong,
                regs.gpr[3] as libc::c_ulong,
                regs.gpr[4] as libc::c_ulong,
                regs.gpr[5] as libc::c_ulong,
                regs.gpr[6] as libc::c_ulong,
                regs.gpr[7] as libc::c_ulong,
                regs.gpr[8] as libc::c_ulong,
            );
            nerrs += 1;
        }
    } else {
        unsafe { printf(c"[OK]\tInitial nr and args are correct\n".as_ptr()) };
    }

    unsafe {
        printf(
            c"[RUN]\tRestart the syscall (ip = 0x%lx)\n".as_ptr(),
            regs.nip as libc::c_ulong,
        )
    };

    /*
     * Rewind to retry the same syscall again. This will basically test
     * the rewind process together with PTRACE_SETREGS and PTRACE_GETREGS.
     */
    regs.nip -= 4;
    if unsafe {
        libc::ptrace(
            libc::PTRACE_SETREGS,
            chld,
            0 as *mut c_void,
            &mut regs as *mut pt_regs,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_SETREGS".as_ptr()) };
    }

    if unsafe {
        libc::ptrace(
            PTRACE_SYSEMU,
            chld,
            0 as *mut c_void,
            0 as *mut c_void,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_SYSEMU".as_ptr()) };
    }
    unsafe { wait_trap(chld) };

    if unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            chld,
            0 as *mut c_void,
            &mut regs as *mut pt_regs,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_GETREGS".as_ptr()) };
    }

    if regs.gpr[0] != libc::SYS_gettid
        || regs.gpr[3] != 10
        || regs.gpr[4] != 11
        || regs.gpr[5] != 12
        || regs.gpr[6] != 13
        || regs.gpr[7] != 14
        || regs.gpr[8] != 15
    {
        unsafe {
            printf(
                c"[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n"
                    .as_ptr(),
                regs.gpr[0] as libc::c_ulong,
                regs.gpr[3] as libc::c_ulong,
                regs.gpr[4] as libc::c_ulong,
                regs.gpr[5] as libc::c_ulong,
                regs.gpr[6] as libc::c_ulong,
                regs.gpr[7] as libc::c_ulong,
                regs.gpr[8] as libc::c_ulong,
            );
            nerrs += 1;
        }
    } else {
        unsafe { printf(c"[OK]\tRestarted nr and args are correct\n".as_ptr()) };
    }

    unsafe {
        printf(
            c"[RUN]\tChange nr and args and restart the syscall (ip = 0x%lx)\n".as_ptr(),
            regs.nip as libc::c_ulong,
        )
    };

    /*
     * Inject a new syscall (getpid) in the same place the previous
     * syscall (gettid), rewind and re-execute.
     */
    regs.gpr[0] = libc::SYS_getpid;
    regs.gpr[3] = 20;
    regs.gpr[4] = 21;
    regs.gpr[5] = 22;
    regs.gpr[6] = 23;
    regs.gpr[7] = 24;
    regs.gpr[8] = 25;
    regs.nip -= 4;

    if unsafe {
        libc::ptrace(
            libc::PTRACE_SETREGS,
            chld,
            0 as *mut c_void,
            &mut regs as *mut pt_regs,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_SETREGS".as_ptr()) };
    }

    if unsafe {
        libc::ptrace(
            PTRACE_SYSEMU,
            chld,
            0 as *mut c_void,
            0 as *mut c_void,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_SYSEMU".as_ptr()) };
    }
    unsafe { wait_trap(chld) };

    if unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            chld,
            0 as *mut c_void,
            &mut regs as *mut pt_regs,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_GETREGS".as_ptr()) };
    }

    /* Check that ptrace stopped at the new syscall that was
     * injected, and guarantee that it haven't executed, i.e, user_args
     * contain the arguments and not the syscall return value, for
     * instance.
     */
    if regs.gpr[0] != libc::SYS_getpid
        || regs.gpr[3] != 20
        || regs.gpr[4] != 21
        || regs.gpr[5] != 22
        || regs.gpr[6] != 23
        || regs.gpr[7] != 24
        || regs.gpr[8] != 25
    {
        unsafe {
            printf(
                c"[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n"
                    .as_ptr(),
                regs.gpr[0] as libc::c_ulong,
                regs.gpr[3] as libc::c_ulong,
                regs.gpr[4] as libc::c_ulong,
                regs.gpr[5] as libc::c_ulong,
                regs.gpr[6] as libc::c_ulong,
                regs.gpr[7] as libc::c_ulong,
                regs.gpr[8] as libc::c_ulong,
            );
            nerrs += 1;
        }
    } else {
        unsafe { printf(c"[OK]\tReplacement nr and args are correct\n".as_ptr()) };
    }

    if unsafe {
        libc::ptrace(
            libc::PTRACE_CONT,
            chld,
            0 as *mut c_void,
            0 as *mut c_void,
        )
    } != 0
    {
        unsafe { err(1, c"PTRACE_CONT".as_ptr()) };
    }

    if unsafe { libc::waitpid(chld, &mut status, 0) } != chld {
        unsafe { err(1, c"waitpid".as_ptr()) };
    }

    /* Guarantee that the process executed properly, returning 0 */
    if !libc::WIFEXITED(status) || libc::WEXITSTATUS(status) != 0 {
        unsafe { printf(c"[FAIL]\tChild failed\n".as_ptr()) };
        unsafe {
            nerrs += 1;
        }
    } else {
        unsafe { printf(c"[OK]\tChild exited cleanly\n".as_ptr()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ptrace_syscall() -> c_int {
    unsafe { test_ptrace_syscall_restart() };

    unsafe { nerrs }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            ptrace_syscall,
            c"ptrace_syscall".as_ptr(),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
