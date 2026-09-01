/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * LoongArch specific definitions for NOLIBC
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

/* Dependencies from the original header:
 *   #include "compiler.h"
 *   #include "crt.h"
 */

use core::arch::asm;

/* Syscalls for LoongArch :
 *   - stack is 16-byte aligned
 *   - syscall number is passed in a7
 *   - arguments are in a0, a1, a2, a3, a4, a5
 *   - the system call is performed by calling "syscall 0"
 *   - syscall return comes in a0
 *   - the arguments are cast to long and assigned into the target
 *     registers which are then simply passed as registers to the asm code,
 *     so that we don't have to experience issues with register constraints.
 */

/* Original _NOLIBC_SYSCALL_CLOBBERLIST:
 * "memory", "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", "$t8"
 */

pub unsafe fn __nolibc_syscall0(num: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize;

    unsafe {
        asm!(
            "syscall 0",
            lateout("a0") _arg1,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
        );
    }
    _arg1
}

pub unsafe fn __nolibc_syscall1(num: isize, arg1: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
        );
    }
    _arg1
}

pub unsafe fn __nolibc_syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a1") _arg2,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
        );
    }
    _arg1
}

pub unsafe fn __nolibc_syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
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
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            in("a3") _arg4,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
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
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            in("a3") _arg4,
            in("a4") _arg5,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
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
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;
    let _arg6: isize = arg6 as isize;

    unsafe {
        asm!(
            "syscall 0",
            inlateout("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            in("a3") _arg4,
            in("a4") _arg5,
            in("a5") _arg6,
            in("a7") _num,
            lateout("t0") _,
            lateout("t1") _,
            lateout("t2") _,
            lateout("t3") _,
            lateout("t4") _,
            lateout("t5") _,
            lateout("t6") _,
            lateout("t7") _,
            lateout("t8") _,
            options(nostack),
        );
    }
    _arg1
}

/* Original conditional:
 * #ifndef NOLIBC_NO_RUNTIME
 */

unsafe extern "C" {
    fn _start_c(sp: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

/* startup code */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "move          $a0, $sp",
            "bl            _start_c",
            options(noreturn),
        );
    }
}

/* The C attributes weak, noreturn, __nolibc_entrypoint, and
 * __nolibc_no_stack_protector come from the included runtime/compiler headers.
 * If the inline branch returned, the C source calls __nolibc_entrypoint_epilogue().
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
