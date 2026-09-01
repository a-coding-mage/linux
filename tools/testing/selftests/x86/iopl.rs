// SPDX-License-Identifier: GPL-2.0
/*
 * iopl.c - Test case for a Linux on Xen 64-bit bug
 * Copyright (c) 2015 Andrew Lutomirski
 */

// C dependencies: err.h, stdio.h, stdint.h, signal.h, setjmp.h, stdlib.h,
// string.h, errno.h, unistd.h, sys/types.h, sys/wait.h, stdbool.h, sched.h,
// sys/io.h, and "helpers.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const SIGSEGV: c_int = 11;
const SA_RESETHAND: c_int = 0x8000_0000u32 as c_int;
const ENOSYS: c_int = 38;

const RET_FAULTED: c_int = 0;
const RET_FAIL: c_int = 1;
const RET_EMUL: c_int = 2;

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
struct jmp_buf {
    // Platform-owned storage for sigsetjmp/siglongjmp, corresponding to C jmp_buf.
    _storage: [c_ulong; 25],
}

type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);
type pid_t = c_int;

unsafe extern "C" {
    static mut errno: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn err(eval: c_int, format: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn setresuid(ruid: c_uint, euid: c_uint, suid: c_uint) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn iopl(level: c_int) -> c_int;
    fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int;
    fn sigsetjmp(env: *mut jmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut jmp_buf, val: c_int) -> !;

    fn sethandler(sig: c_int, handler: sighandler_t, flags: c_int);
    fn clearhandler(sig: c_int);
}

static mut nerrs: c_int = 0;

static mut jmpbuf: jmp_buf = jmp_buf { _storage: [0; 25] };

unsafe extern "C" fn sigsegv(_sig: c_int, _si: *mut siginfo_t, _ctx_void: *mut c_void) {
    unsafe {
        siglongjmp(&raw mut jmpbuf, 1);
    }
}

unsafe fn cpu_zero(cpuset: *mut cpu_set_t) {
    unsafe {
        (*cpuset).__bits = [0; 16];
    }
}

unsafe fn cpu_set(cpu: usize, cpuset: *mut cpu_set_t) {
    let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
    unsafe {
        (*cpuset).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn try_outb(port: u16) -> bool {
    unsafe {
        sethandler(SIGSEGV, sigsegv, SA_RESETHAND);
        if sigsetjmp(&raw mut jmpbuf, 1) != 0 {
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
        // Unreachable in the C source after both branches return.
        // clearhandler(SIGSEGV);
    }
}

unsafe fn expect_ok_outb(port: u16) {
    unsafe {
        if !try_outb(port) {
            printf(c"[FAIL]\toutb to 0x%02hx failed\n".as_ptr(), port as c_int);
            exit(1);
        }

        printf(c"[OK]\toutb to 0x%02hx worked\n".as_ptr(), port as c_int);
    }
}

unsafe fn expect_gp_outb(port: u16) {
    unsafe {
        if try_outb(port) {
            printf(c"[FAIL]\toutb to 0x%02hx worked\n".as_ptr(), port as c_int);
            nerrs += 1;
        }

        printf(c"[OK]\toutb to 0x%02hx failed\n".as_ptr(), port as c_int);
    }
}

unsafe fn try_cli() -> c_int {
    let flags: c_ulong;

    unsafe {
        sethandler(SIGSEGV, sigsegv, SA_RESETHAND);
        if sigsetjmp(&raw mut jmpbuf, 1) != 0 {
            RET_FAULTED
        } else {
            asm!(
                "cli",
                "pushfq",
                "pop {flags}",
                flags = lateout(reg) flags,
                options(nomem)
            );

            /* X86_FLAGS_IF */
            if (flags & (1 << 9)) == 0 {
                RET_FAIL
            } else {
                RET_EMUL
            }
        }
        // Unreachable in the C source after both branches return.
        // clearhandler(SIGSEGV);
    }
}

unsafe fn try_sti(irqs_off: bool) -> c_int {
    let flags: c_ulong;

    unsafe {
        sethandler(SIGSEGV, sigsegv, SA_RESETHAND);
        if sigsetjmp(&raw mut jmpbuf, 1) != 0 {
            RET_FAULTED
        } else {
            asm!(
                "sti",
                "pushfq",
                "pop {flags}",
                flags = lateout(reg) flags,
                options(nomem)
            );

            /* X86_FLAGS_IF */
            if irqs_off && (flags & (1 << 9)) != 0 {
                RET_FAIL
            } else {
                RET_EMUL
            }
        }
        // Unreachable in the C source after both branches return.
        // clearhandler(SIGSEGV);
    }
}

unsafe fn expect_gp_sti(irqs_off: bool) {
    let ret = unsafe { try_sti(irqs_off) };

    unsafe {
        match ret {
            RET_FAULTED => {
                printf(c"[OK]\tSTI faulted\n".as_ptr());
            }
            RET_EMUL => {
                printf(c"[OK]\tSTI NOPped\n".as_ptr());
            }
            _ => {
                printf(c"[FAIL]\tSTI worked\n".as_ptr());
                nerrs += 1;
            }
        }
    }
}

/*
 * Returns whether it managed to disable interrupts.
 */
unsafe fn test_cli() -> bool {
    let ret = unsafe { try_cli() };

    unsafe {
        match ret {
            RET_FAULTED => {
                printf(c"[OK]\tCLI faulted\n".as_ptr());
            }
            RET_EMUL => {
                printf(c"[OK]\tCLI NOPped\n".as_ptr());
            }
            _ => {
                printf(c"[FAIL]\tCLI worked\n".as_ptr());
                nerrs += 1;
                return true;
            }
        }
    }

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe {
        let mut cpuset = cpu_set_t { __bits: [0; 16] };

        cpu_zero(&mut cpuset);
        cpu_set(0, &mut cpuset);
        if sched_setaffinity(0, core::mem::size_of_val(&cpuset), &cpuset) != 0 {
            err(1, c"sched_setaffinity to CPU 0".as_ptr());
        }

        /* Probe for iopl support.  Note that iopl(0) works even as nonroot. */
        match iopl(3) {
            0 => {}
            x if x == -ENOSYS => {
                printf(c"[OK]\tiopl() nor supported\n".as_ptr());
                return 0;
            }
            _ => {
                printf(
                    c"[OK]\tiopl(3) failed (%d) -- try running as root\n".as_ptr(),
                    errno,
                );
                return 0;
            }
        }

        /* Make sure that CLI/STI are blocked even with IOPL level 3 */
        let cli_disabled = test_cli();
        expect_gp_sti(cli_disabled);
        expect_ok_outb(0x80);

        /* Establish an I/O bitmap to test the restore */
        if ioperm(0x80, 1, 1) != 0 {
            err(1, c"ioperm(0x80, 1, 1) failed\n".as_ptr());
        }

        /* Restore our original state prior to starting the fork test. */
        if iopl(0) != 0 {
            err(1, c"iopl(0)".as_ptr());
        }

        /*
         * Verify that IOPL emulation is disabled and the I/O bitmap still
         * works.
         */
        expect_ok_outb(0x80);
        expect_gp_outb(0xed);
        /* Drop the I/O bitmap */
        if ioperm(0x80, 1, 0) != 0 {
            err(1, c"ioperm(0x80, 1, 0) failed\n".as_ptr());
        }

        let child: pid_t = fork();
        if child == -1 {
            err(1, c"fork".as_ptr());
        }

        if child == 0 {
            printf(c"\tchild: set IOPL to 3\n".as_ptr());
            if iopl(3) != 0 {
                err(1, c"iopl".as_ptr());
            }

            printf(c"[RUN]\tchild: write to 0x80\n".as_ptr());
            asm!(
                "out 0x80, al",
                in("al") 0u8,
                options(nomem, nostack, preserves_flags)
            );

            return 0;
        } else {
            let mut status: c_int = 0;
            if waitpid(child, &mut status, 0) != child || !wifexited(status) {
                printf(c"[FAIL]\tChild died\n".as_ptr());
                nerrs += 1;
            } else if wexitstatus(status) != 0 {
                printf(c"[FAIL]\tChild failed\n".as_ptr());
                nerrs += 1;
            } else {
                printf(c"[OK]\tChild succeeded\n".as_ptr());
            }
        }

        printf(c"[RUN]\tparent: write to 0x80 (should fail)\n".as_ptr());

        expect_gp_outb(0x80);
        let cli_disabled = test_cli();
        expect_gp_sti(cli_disabled);

        /* Test the capability checks. */
        printf(c"\tiopl(3)\n".as_ptr());
        if iopl(3) != 0 {
            err(1, c"iopl(3)".as_ptr());
        }

        printf(c"\tDrop privileges\n".as_ptr());
        if setresuid(1, 1, 1) != 0 {
            printf(c"[WARN]\tDropping privileges failed\n".as_ptr());
            return if nerrs != 0 { 1 } else { 0 };
        }

        printf(c"[RUN]\tiopl(3) unprivileged but with IOPL==3\n".as_ptr());
        if iopl(3) != 0 {
            printf(c"[FAIL]\tiopl(3) should work if iopl is already 3 even if unprivileged\n".as_ptr());
            nerrs += 1;
        }

        printf(c"[RUN]\tiopl(0) unprivileged\n".as_ptr());
        if iopl(0) != 0 {
            printf(c"[FAIL]\tiopl(0) should work if iopl is already 3 even if unprivileged\n".as_ptr());
            nerrs += 1;
        }

        printf(c"[RUN]\tiopl(3) unprivileged\n".as_ptr());
        if iopl(3) == 0 {
            printf(c"[FAIL]\tiopl(3) should fail if when unprivileged if iopl==0\n".as_ptr());
            nerrs += 1;
        } else {
            printf(c"[OK]\tFailed as expected\n".as_ptr());
        }

        if nerrs != 0 {
            1
        } else {
            0
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
