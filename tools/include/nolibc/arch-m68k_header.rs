/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * m68k specific definitions for NOLIBC
 * Copyright (C) 2025 Daniel Palmer<daniel@thingy.jp>
 *
 * Roughly based on one or more of the other arch files.
 *
 */

// C dependencies removed from executable Rust:
// #include "compiler.h"
// #include "crt.h"

use core::arch::asm;
use core::ffi::c_long;

pub const _NOLIBC_SYSCALL_CLOBBERLIST: &str = "memory";

#[inline(always)]
pub unsafe fn __nolibc_syscall0(num: c_long) -> c_long {
    let mut _num: c_long = num;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
        );
    }

    _num
}

#[inline(always)]
pub unsafe fn __nolibc_syscall1(num: c_long, arg1: c_long) -> c_long {
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
        );
    }

    _num
}

#[inline(always)]
pub unsafe fn __nolibc_syscall2(num: c_long, arg1: c_long, arg2: c_long) -> c_long {
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
            in("d2") _arg2,
        );
    }

    _num
}

#[inline(always)]
pub unsafe fn __nolibc_syscall3(num: c_long, arg1: c_long, arg2: c_long, arg3: c_long) -> c_long {
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
            in("d2") _arg2,
            in("d3") _arg3,
        );
    }

    _num
}

#[inline(always)]
pub unsafe fn __nolibc_syscall4(
    num: c_long,
    arg1: c_long,
    arg2: c_long,
    arg3: c_long,
    arg4: c_long,
) -> c_long {
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
            in("d2") _arg2,
            in("d3") _arg3,
            in("d4") _arg4,
        );
    }

    _num
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
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let _arg5: c_long = arg5 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
            in("d2") _arg2,
            in("d3") _arg3,
            in("d4") _arg4,
            in("d5") _arg5,
        );
    }

    _num
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
    let mut _num: c_long = num;
    let _arg1: c_long = arg1 as c_long;
    let _arg2: c_long = arg2 as c_long;
    let _arg3: c_long = arg3 as c_long;
    let _arg4: c_long = arg4 as c_long;
    let _arg5: c_long = arg5 as c_long;
    let _arg6: c_long = arg6 as c_long;

    unsafe {
        asm!(
            "trap #0",
            inout("d0") _num,
            in("d1") _arg1,
            in("d2") _arg2,
            in("d3") _arg3,
            in("d4") _arg4,
            in("d5") _arg5,
            in("a0") _arg6,
        );
    }

    _num
}

// Original C condition:
// #ifndef NOLIBC_NO_RUNTIME
unsafe extern "C" {
    fn _start_c(sp: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "movel %sp, %sp@-",
            "jsr _start_c",
        );
        __nolibc_entrypoint_epilogue();
    }
}
// #endif /* NOLIBC_NO_RUNTIME */
