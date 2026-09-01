/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * PowerPC specific definitions for NOLIBC
 * Copyright (C) 2023 Zhangjin Wu <falcon@tinylab.org>
 */

/* Original C dependencies:
 *   <linux/unistd.h>
 *   "compiler.h"
 *   "crt.h"
 *   "std.h"
 */

use core::arch::asm;

/* Syscalls for PowerPC :
 *   - stack is 16-byte aligned
 *   - syscall number is passed in r0
 *   - arguments are in r3, r4, r5, r6, r7, r8, r9
 *   - the system call is performed by calling "sc"
 *   - syscall return comes in r3, and the summary overflow bit is checked
 *     to know if an error occurred, in which case errno is in r3.
 *   - the arguments are cast to long and assigned into the target
 *     registers which are then simply passed as registers to the asm code,
 *     so that we don't have to experience issues with register constraints.
 */

/* _NOLIBC_SYSCALL_CLOBBERLIST:
 *   "memory", "cr0", "ctr", "xer", "r12", "r11", "r10", "r9"
 */

#[inline(always)]
pub unsafe fn __nolibc_syscall0(num: isize) -> isize {
    let mut _ret: isize;
    let mut _num: isize = num;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = lateout(reg) _ret,
            inout("r0") _num,
            lateout("r8") _,
            lateout("r7") _,
            lateout("r6") _,
            lateout("r5") _,
            lateout("r4") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall1(num: isize, arg1: isize) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            lateout("r8") _,
            lateout("r7") _,
            lateout("r6") _,
            lateout("r5") _,
            lateout("r4") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;
    let mut _arg2: isize = arg2 as isize;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            inout("r4") _arg2,
            lateout("r8") _,
            lateout("r7") _,
            lateout("r6") _,
            lateout("r5") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            inout("r4") _arg2,
            inout("r5") _arg3,
            lateout("r8") _,
            lateout("r7") _,
            lateout("r6") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall4(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            inout("r4") _arg2,
            inout("r5") _arg3,
            inout("r6") _arg4,
            lateout("r8") _,
            lateout("r7") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall5(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;
    let mut _arg5: isize = arg5 as isize;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            inout("r4") _arg2,
            inout("r5") _arg3,
            inout("r6") _arg4,
            inout("r7") _arg5,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall6(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    arg6: isize,
) -> isize {
    let mut _ret: isize = arg1 as isize;
    let mut _num: isize = num;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;
    let mut _arg5: isize = arg5 as isize;
    let mut _arg6: isize = arg6 as isize;

    unsafe {
        asm!(
            "sc",
            "bns+ 1f",
            "neg {ret}, {ret}",
            "1:",
            ret = inlateout("r3") _ret,
            inout("r0") _num,
            inout("r4") _arg2,
            inout("r5") _arg3,
            inout("r6") _arg4,
            inout("r7") _arg5,
            inout("r8") _arg6,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
            lateout("r12") _,
            clobber_abi("C"),
        );
    }
    _ret
}

/* C preprocessor condition preserved:
 *   #if !defined(__powerpc64__) && !defined(__clang__)
 *
 * For 32-bit PowerPC, with newer gcc compilers (e.g. gcc 13.1.0),
 * "omit-frame-pointer" fails with __attribute__((no_stack_protector)) but
 * works with __attribute__((__optimize__("-fno-stack-protector"))).
 *
 * If __nolibc_no_stack_protector was defined, C undefines it and redefines it
 * as __attribute__((__optimize__("-fno-stack-protector"))).
 */

#[cfg(not(NOLIBC_NO_RUNTIME))]
unsafe extern "C" {
    fn _start_c(sp: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

#[cfg(not(NOLIBC_NO_RUNTIME))]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    #[cfg(all(target_arch = "powerpc64", _CALL_ELF_2))]
    unsafe {
        /* with -mabi=elfv2, save TOC/GOT pointer to r2
         * r12 is global entry pointer, we use it to compute TOC from r12
         * https://www.llvm.org/devmtg/2014-04/PDFs/Talks/Euro-LLVM-2014-Weigand.pdf
         * https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.pdf
         */
        asm!(
            "addis  2, 12, .TOC. - _start@ha",
            "addi   2,  2, .TOC. - _start@l",
        );
    }

    #[cfg(target_arch = "powerpc64")]
    unsafe {
        asm!(
            "mr     3, 1",
            "li     0, 0",
            "stdu   1, -32(1)",
            "bl     _start_c",
            options(noreturn),
        );
    }

    #[cfg(not(target_arch = "powerpc64"))]
    unsafe {
        asm!(
            "mr     3, 1",
            "li     0, 0",
            "stwu   1, -16(1)",
            "bl     _start_c",
            options(noreturn),
        );
    }

    unsafe { __nolibc_entrypoint_epilogue() }
}

#[cfg(not(target_arch = "powerpc64"))]
#[allow(dead_code)]
pub unsafe fn _sys_ftruncate64(fd: i32, length0: u32, length1: u32) -> i32 {
    unsafe { __nolibc_syscall4(__NR_ftruncate64 as isize, fd as isize, 0, length0 as isize, length1 as isize) as i32 }
}

#[cfg(not(target_arch = "powerpc64"))]
unsafe extern "C" {
    static __NR_ftruncate64: core::ffi::c_long;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
