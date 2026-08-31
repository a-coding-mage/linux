/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * SuperH specific definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

// Dependencies from the original header:
// #include "compiler.h"
// #include "crt.h"

use core::arch::asm;
use core::ffi::c_long;

/*
 * Syscalls for SuperH:
 *   - registers are 32bit wide
 *   - syscall number is passed in r3
 *   - arguments are in r4, r5, r6, r7, r0, r1, r2
 *   - the system call is performed by calling trapa #31
 *   - syscall return value is in r0
 */

#[inline]
pub unsafe fn __nolibc_syscall0(num: c_long) -> c_long {
    let _num: c_long = num;
    let _ret: c_long;

    unsafe {
        asm!(
            "trapa #31",
            lateout("r0") _ret,
            in("r3") _num,
            options(nostack),
        );
    }
    _ret
}

#[inline]
pub unsafe fn __nolibc_syscall1(num: c_long, arg1: c_long) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _ret: c_long;

    unsafe {
        asm!(
            "trapa #31",
            lateout("r0") _ret,
            in("r3") _num,
            in("r4") _arg1,
            options(nostack),
        );
    }
    _ret
}

#[inline]
pub unsafe fn __nolibc_syscall2(num: c_long, arg1: c_long, arg2: c_long) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _ret: c_long;

    unsafe {
        asm!(
            "trapa #31",
            lateout("r0") _ret,
            in("r3") _num,
            in("r4") _arg1,
            in("r5") _arg2,
            options(nostack),
        );
    }
    _ret
}

#[inline]
pub unsafe fn __nolibc_syscall3(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _ret: c_long;

    unsafe {
        asm!(
            "trapa #31",
            lateout("r0") _ret,
            in("r3") _num,
            in("r4") _arg1,
            in("r5") _arg2,
            in("r6") _arg3,
            options(nostack),
        );
    }
    _ret
}

#[inline]
pub unsafe fn __nolibc_syscall4(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let _ret: c_long;

    unsafe {
        asm!(
            "trapa #31",
            lateout("r0") _ret,
            in("r3") _num,
            in("r4") _arg1,
            in("r5") _arg2,
            in("r6") _arg3,
            in("r7") _arg4,
            options(nostack),
        );
    }
    _ret
}

#[inline]
pub unsafe fn __nolibc_syscall5(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
    arg5: c_long,
) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let mut _arg5: c_long = arg5 as c_long;

    unsafe {
        asm!(
            "trapa #31",
            in("r3") _num,
            in("r4") _arg1,
            in("r5") _arg2,
            in("r6") _arg3,
            in("r7") _arg4,
            inlateout("r0") _arg5,
            options(nostack),
        );
    }
    _arg5
}

#[inline]
pub unsafe fn __nolibc_syscall6(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
    arg5: c_long,
    arg6: c_long,
) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let mut _arg5: c_long = arg5 as c_long;
    let _arg6: c_long = arg6 as c_long;

    unsafe {
        asm!(
            "trapa #31",
            in("r3") _num,
            in("r4") _arg1,
            in("r5") _arg2,
            in("r6") _arg3,
            in("r7") _arg4,
            inlateout("r0") _arg5,
            in("r1") _arg6,
            options(nostack),
        );
    }
    _arg5
}

// Original condition: #ifndef NOLIBC_NO_RUNTIME
/* startup code */
unsafe extern "C" {
    pub fn __nolibc_entrypoint_epilogue() -> !;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start_wrapper() -> ! {
    unsafe {
        asm!(
            ".global _start",
            ".type _start, @function",
            ".weak _start",
            "_start:",
            "mov sp, r4",
            "bsr _start_c",
            "nop",
            ".size _start, .-_start",
            options(noreturn),
        );
    }
}
// End original condition: #endif /* NOLIBC_NO_RUNTIME */
