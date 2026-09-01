/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * ARM specific definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* C header dependencies:
 *   <linux/unistd.h>
 *   "compiler.h"
 *   "crt.h"
 *   "std.h"
 */

use core::arch::asm;

/* Syscalls for ARM in ARM or Thumb modes :
 *   - registers are 32-bit
 *   - stack is 8-byte aligned
 *     ( http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.faqs/ka4127.html)
 *   - syscall number is passed in r7
 *   - arguments are in r0, r1, r2, r3, r4, r5
 *   - the system call is performed by calling svc #0
 *   - syscall return comes in r0.
 *   - only lr is clobbered.
 *   - the arguments are cast to long and assigned into the target registers
 *     which are then simply passed as registers to the asm code, so that we
 *     don't have to experience issues with register constraints.
 *   - the syscall number is always specified last in order to allow to force
 *     some registers before (gcc refuses a %-register at the last position).
 *   - in thumb mode without -fomit-frame-pointer, r7 is also used to store the
 *     frame pointer, and we cannot directly assign it as a register variable,
 *     nor can we clobber it. Instead we assign the r6 register and swap it
 *     with r7 before calling svc, and r6 is marked as clobbered.
 *     We're just using any regular register which we assign to r7 after saving
 *     it.
 */

/* C preprocessor condition:
 *   (defined(__THUMBEB__) || defined(__THUMBEL__)) &&
 *   !defined(NOLIBC_OMIT_FRAME_POINTER)
 *
 * In that Thumb mode case, the C macros use r6 as _NOLIBC_SYSCALL_REG and
 * exchange r6/r7 around svc. Otherwise they use r7 directly.
 */

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                lateout("r0") _arg1,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize;
        unsafe {
            asm!(
                "svc #0",
                lateout("r0") _arg1,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                in("r1") _arg2,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                in("r1") _arg2,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                in("r4") _arg5,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                in("r4") _arg5,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER)))]
#[macro_export]
macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        let _arg6: isize = $arg6 as isize;
        unsafe {
            asm!(
                "eor r7, r6",
                "eor r6, r7",
                "eor r7, r6",
                "svc #0",
                "mov r7, r6",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                in("r4") _arg5,
                in("r5") _arg6,
                inout("r6") _num,
                lateout("r6") _,
                options(nostack),
            );
        }
        _arg1
    }};
}

#[cfg(not(all(target_feature = "thumb-mode", not(NOLIBC_OMIT_FRAME_POINTER))))]
#[macro_export]
macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        let _arg6: isize = $arg6 as isize;
        unsafe {
            asm!(
                "svc #0",
                inout("r0") _arg1,
                in("r1") _arg2,
                in("r2") _arg3,
                in("r3") _arg4,
                in("r4") _arg5,
                in("r5") _arg6,
                inout("r7") _num,
                options(nostack),
            );
        }
        _arg1
    }};
}

/* C preprocessor condition: #ifndef NOLIBC_NO_RUNTIME */
#[cfg(not(NOLIBC_NO_RUNTIME))]
extern "C" {
    fn _start_c(stack: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

#[cfg(not(NOLIBC_NO_RUNTIME))]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    /* startup code */
    asm!(
        "mov r0, sp",          /* save stack pointer to %r0, as arg1 of _start_c */
        "bl  _start_c",        /* transfer to c runtime                          */
        options(noreturn),
    );
}

static __nolibc_sys_ftruncate64_marker: () = ();

#[allow(non_snake_case)]
pub unsafe fn _sys_ftruncate64(fd: i32, length0: u32, length1: u32) -> i32 {
    __nolibc_syscall4!(__NR_ftruncate64, fd, 0, length0, length1) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
