/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * ARM64 specific definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* Original C dependencies:
 * #include "compiler.h"
 * #include "crt.h"
 */

use core::arch::asm;
use core::ffi::c_long;

/* Syscalls for ARM64 :
 *   - registers are 64-bit
 *   - stack is 16-byte aligned
 *   - syscall number is passed in x8
 *   - arguments are in x0, x1, x2, x3, x4, x5
 *   - the system call is performed by calling svc 0
 *   - syscall return comes in x0.
 *   - the arguments are cast to long and assigned into the target registers
 *     which are then simply passed as registers to the asm code, so that we
 *     don't have to experience issues with register constraints.
 */

#[inline(always)]
pub unsafe fn __nolibc_syscall0(num: c_long) -> c_long {
    let _num: c_long = num;
    let _arg1: c_long;

    unsafe {
        asm!(
            "svc #0",
            lateout("x0") _arg1,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
pub unsafe fn __nolibc_syscall1(num: c_long, arg1: c_long) -> c_long {
    let _num: c_long = num;
    let mut _arg1: c_long = arg1 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
pub unsafe fn __nolibc_syscall2(num: c_long, arg1: c_long, arg2: c_long) -> c_long {
    let _num: c_long = num;
    let mut _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
pub unsafe fn __nolibc_syscall3(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
) -> c_long {
    let _num: c_long = num;
    let mut _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
pub unsafe fn __nolibc_syscall4(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
) -> c_long {
    let _num: c_long = num;
    let mut _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
pub unsafe fn __nolibc_syscall5(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
    arg5: c_long,
) -> c_long {
    let _num: c_long = num;
    let mut _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let _arg5: c_long = arg5 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x4") _arg5,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

#[inline(always)]
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
    let mut _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let _arg5: c_long = arg5 as c_long;
    let _arg6: c_long = arg6 as c_long;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x4") _arg5,
            in("x5") _arg6,
            in("x8") _num,
            options(nostack),
        );
    }

    _arg1
}

/* startup code */
/* Original C condition: #ifndef NOLIBC_NO_RUNTIME */
unsafe extern "C" {
    fn _start_c(sp: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

/* Original C attributes:
 * void __attribute__((weak, noreturn)) __nolibc_entrypoint
 * __nolibc_no_stack_protector _start(void)
 */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov x0, sp",
            "bl  _start_c",
            options(noreturn),
        );
    }

    unsafe {
        __nolibc_entrypoint_epilogue();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
