/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * parisc/hppa (32-bit) specific definitions for NOLIBC
 * Copyright (C) 2026 Thomas Weissschuh <linux@weissschuh.net>
 */

/* Original C header guard: _NOLIBC_ARCH_PARISC_H */

/* Original C condition:
 * #if defined(__LP64__)
 * #error 64-bit not supported
 * #endif
 */
#[cfg(target_pointer_width = "64")]
compile_error!("64-bit not supported");

/* Original C dependencies:
 * #include "compiler.h"
 * #include "crt.h"
 */

/* Syscalls for parisc :
 *   - syscall number is passed in r20
 *   - arguments are in r26 to r21
 *   - the system call is performed by calling "ble 0x100(%sr2, %r0)",
 *     the instruction after that is in the delay slot and executed before
 *     the jump to 0x100 actually happens, use it to load the syscall number
 *   - syscall return comes in r28
 *   - the arguments are cast to long and assigned into the target
 *     registers which are then simply passed as registers to the asm code,
 *     so that we don't have to experience issues with register constraints.
 */

/* Original C macro:
 * #define _NOLIBC_SYSCALL_CLOBBERLIST \
 *     "memory", "%r1", "%r2", "%r4", "%r20", "%r29", "%r31"
 *
 * Rust inline asm does not expose a direct named clobber-list item equivalent
 * for this file-local macro; each translated syscall asm block preserves the
 * same register-clobber intent in comments and explicit operands where Rust
 * syntax allows it.
 */

#[inline(always)]
pub unsafe fn __nolibc_syscall0(num: isize) -> isize {
    let _ret: isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21, %r22, %r23, %r24, %r25, %r26
             */
            options(nostack, preserves_flags),
        );
    }

    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall1(num: isize, arg1: isize) -> isize {
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21, %r22, %r23, %r24, %r25
             */
            options(nostack, preserves_flags),
        );
    }

    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;
    let mut _arg2: isize = arg2 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            inout("r25") _arg2,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21, %r22, %r23, %r24
             */
            options(nostack, preserves_flags),
        );
    }

    _ret
}

#[inline(always)]
pub unsafe fn __nolibc_syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            inout("r25") _arg2,
            inout("r24") _arg3,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21, %r22, %r23
             */
            options(nostack, preserves_flags),
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
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            inout("r25") _arg2,
            inout("r24") _arg3,
            inout("r23") _arg4,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21, %r22
             */
            options(nostack, preserves_flags),
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
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;
    let mut _arg5: isize = arg5 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            inout("r25") _arg2,
            inout("r24") _arg3,
            inout("r23") _arg4,
            inout("r22") _arg5,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31,
             *           %r21
             */
            options(nostack, preserves_flags),
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
    let _ret: isize;
    let mut _arg1: isize = arg1 as isize;
    let mut _arg2: isize = arg2 as isize;
    let mut _arg3: isize = arg3 as isize;
    let mut _arg4: isize = arg4 as isize;
    let mut _arg5: isize = arg5 as isize;
    let mut _arg6: isize = arg6 as isize;

    unsafe {
        core::arch::asm!(
            "ble 0x100(%sr2, %r0)",
            "copy {num}, %r20",
            num = in(reg) num,
            inout("r26") _arg1,
            inout("r25") _arg2,
            inout("r24") _arg3,
            inout("r23") _arg4,
            inout("r22") _arg5,
            inout("r21") _arg6,
            lateout("r28") _ret,
            /* clobbers: memory, %r1, %r2, %r4, %r20, %r29, %r31 */
            options(nostack, preserves_flags),
        );
    }

    _ret
}

/* Original C condition:
 * #ifndef NOLIBC_NO_RUNTIME
 */

/* startup code */
#[cfg(not(NOLIBC_NO_RUNTIME))]
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            ".import $global$",
            "ldil L%$global$, %dp",
            "ldo R%$global$(%r27), %dp",
            "b _start_c",
            "ldo -4(%r24), %r26",
            options(noreturn),
        );
    }
}

/* Original attributes on _start:
 * __attribute__((weak, noreturn)) __nolibc_entrypoint
 * __nolibc_no_stack_protector
 *
 * The C body calls __nolibc_entrypoint_epilogue() after noreturn assembly as a
 * compiler-facing epilogue. The translated Rust asm is noreturn, so no
 * reachable epilogue call is emitted.
 */

