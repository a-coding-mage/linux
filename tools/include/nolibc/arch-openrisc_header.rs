/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * OpenRISC specific definitions for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* Dependencies from the C header:
 * #include "compiler.h"
 * #include "crt.h"
 */

/*
 * Syscalls for OpenRISC:
 *   - syscall number is passed in r11
 *   - arguments are in r3, r4, r5, r6, r7, r8
 *   - the system call is performed by calling l.sys 1
 *   - syscall return value is in r11
 */

/* C clobber list:
 * "r12", "r13", "r15", "r17", "r19", "r21", "r23", "r25", "r27", "r29", "r31", "memory"
 */

#[macro_export]
macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _num: i64 = $num as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                lateout("r3") _,
                lateout("r4") _,
                lateout("r5") _,
                lateout("r6") _,
                lateout("r7") _,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                lateout("r4") _,
                lateout("r5") _,
                lateout("r6") _,
                lateout("r7") _,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                in("r4") _arg2,
                lateout("r5") _,
                lateout("r6") _,
                lateout("r7") _,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                in("r4") _arg2,
                in("r5") _arg3,
                lateout("r6") _,
                lateout("r7") _,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let _arg4: i64 = $arg4 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                in("r4") _arg2,
                in("r5") _arg3,
                in("r6") _arg4,
                lateout("r7") _,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let _arg4: i64 = $arg4 as i64;
        let _arg5: i64 = $arg5 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                in("r4") _arg2,
                in("r5") _arg3,
                in("r6") _arg4,
                in("r7") _arg5,
                lateout("r8") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

#[macro_export]
macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let _arg4: i64 = $arg4 as i64;
        let _arg5: i64 = $arg5 as i64;
        let _arg6: i64 = $arg6 as i64;

        unsafe {
            core::arch::asm!(
                "l.sys 1",
                inout("r11") _num,
                in("r3") _arg1,
                in("r4") _arg2,
                in("r5") _arg3,
                in("r6") _arg4,
                in("r7") _arg5,
                in("r8") _arg6,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r15") _,
                lateout("r17") _,
                lateout("r19") _,
                lateout("r21") _,
                lateout("r23") _,
                lateout("r25") _,
                lateout("r27") _,
                lateout("r29") _,
                lateout("r31") _,
                options(nostack),
            );
        }
        _num
    }};
}

/* The following runtime startup code is omitted when NOLIBC_NO_RUNTIME is defined. */

#[cfg(not(NOLIBC_NO_RUNTIME))]
extern "C" {
    fn __nolibc_entrypoint_epilogue() -> !;
}

/* startup code */
#[cfg(not(NOLIBC_NO_RUNTIME))]
#[no_mangle]
pub unsafe extern "C" fn _start_wrapper() -> ! {
    core::arch::asm!(
        ".global _start",
        ".type _start, @function",
        ".weak _start",
        "_start:",
        "l.jal _start_c",
        "l.or r3,r1,r1",
        ".size _start, .-_start",
        options(noreturn),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
