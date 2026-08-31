/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * RISCV (32 and 64) specific definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* Dependencies in the original header:
 * #include "compiler.h"
 * #include "crt.h"
 */

/* Syscalls for RISCV :
 *   - stack is 16-byte aligned
 *   - syscall number is passed in a7
 *   - arguments are in a0, a1, a2, a3, a4, a5
 *   - the system call is performed by calling ecall
 *   - syscall return comes in a0
 *   - the arguments are cast to long and assigned into the target
 *     registers which are then simply passed as registers to the asm code,
 *     so that we don't have to experience issues with register constraints.
 */

macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let _num: isize = $num as isize;
        let _arg1: isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                lateout("a0") _arg1,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a3") _arg4,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a3") _arg4,
                in("a4") _arg5,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        let _arg6: isize = $arg6 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a3") _arg4,
                in("a4") _arg5,
                in("a5") _arg6,
                in("a7") _num,
                options(nostack, preserves_flags),
            );
        }
        _arg1
    }};
}

/* Original condition: #ifndef NOLIBC_NO_RUNTIME */

/* startup code */
extern "C" {
    fn _start_c(stack: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn _start() -> ! {
    let stack: *mut core::ffi::c_void;

    core::arch::asm!(
        ".option push",
        ".option norelax",
        "lla  gp, __global_pointer$",
        ".option pop",
        "mv   {0}, sp",
        lateout(reg) stack,
        options(nostack, preserves_flags),
    );

    _start_c(stack);
}

/* End of original condition: #endif / NOLIBC_NO_RUNTIME */
