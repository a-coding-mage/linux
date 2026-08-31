/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * SPARC (32bit and 64bit) specific definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

/* C header guard removed in Rust translation. */

/* Dependencies from the original header:
 * - <linux/unistd.h>
 * - "compiler.h"
 * - "crt.h"
 */

/*
 * Syscalls for SPARC:
 *   - registers are native word size
 *   - syscall number is passed in g1
 *   - arguments are in o0-o5
 *   - the system call is performed by calling a trap instruction
 *   - syscall return value is in o0
 *   - syscall error flag is in the carry bit of the processor status register
 */

/* Original C selected the trap sequence with #ifdef __arch64__. */
#[cfg(target_pointer_width = "64")]
macro_rules! _NOLIBC_SYSCALL {
    () => {
        "t 0x6d",
        "bcs,a %xcc, 1f",
        "sub %g0, %o0, %o0",
        "1:",
    };
}

/* Original C selected the trap sequence with #else of #ifdef __arch64__. */
#[cfg(not(target_pointer_width = "64"))]
macro_rules! _NOLIBC_SYSCALL {
    () => {
        "t 0x10",
        "bcs,a 1f",
        "sub %g0, %o0, %o0",
        "1:",
    };
}

pub unsafe fn __nolibc_syscall0(num: isize) -> isize {
    let mut _arg1: isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            lateout("o0") _arg1,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall1(num: isize, arg1: isize) -> isize {
    let mut _arg1: isize = arg1 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("o1") _arg2,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("o1") _arg2,
            in("o2") _arg3,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall4(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
) -> isize {
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("o1") _arg2,
            in("o2") _arg3,
            in("o3") _arg4,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall5(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
) -> isize {
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("o1") _arg2,
            in("o2") _arg3,
            in("o3") _arg4,
            in("o4") _arg5,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall6(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    arg6: isize,
) -> isize {
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;
    let _arg6: isize = arg6 as isize;

    unsafe {
        core::arch::asm!(
            _NOLIBC_SYSCALL!(),
            inout("o0") _arg1,
            in("o1") _arg2,
            in("o2") _arg3,
            in("o3") _arg4,
            in("o4") _arg5,
            in("o5") _arg6,
            in("g1") num,
            options(nostack, preserves_flags),
        );
    }

    _arg1
}

/* Original C omits this runtime code when NOLIBC_NO_RUNTIME is defined. */
#[cfg(not(NOLIBC_NO_RUNTIME))]
/* startup code */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            /*
             * Save argc pointer to o0, as arg1 of _start_c.
             * Account for the window save area, which is 16 registers wide.
             */
            #[cfg(target_pointer_width = "64")]
            "add %sp, 128 + 2047, %o0",
            /* on sparc64 / v9 the stack is offset by 2047 */
            #[cfg(not(target_pointer_width = "64"))]
            "add %sp, 64, %o0",
            "b,a _start_c",
            /* transfer to c runtime */
            options(noreturn),
        );
    }
}

unsafe extern "C" {
    fn getpid() -> pid_t;
}

pub unsafe fn _sys_fork() -> pid_t {
    let parent: pid_t;
    let ret: pid_t;

    unsafe {
        parent = getpid();
        ret = __nolibc_syscall0(__NR_fork as isize) as pid_t;
    }

    /* The syscall returns the parent pid in the child instead of 0 */
    if ret == parent {
        return 0;
    } else {
        return ret;
    }
}
/* #define _sys_fork _sys_fork */

pub unsafe fn _sys_vfork() -> pid_t {
    let parent: pid_t;
    let ret: pid_t;

    unsafe {
        parent = getpid();
        ret = __nolibc_syscall0(__NR_vfork as isize) as pid_t;
    }

    /* The syscall returns the parent pid in the child instead of 0 */
    if ret == parent {
        return 0;
    } else {
        return ret;
    }
}
/* #define _sys_vfork _sys_vfork */
