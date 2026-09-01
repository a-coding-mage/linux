/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * MIPS specific definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* C header guard removed in Rust: _NOLIBC_ARCH_MIPS_H */

/* C includes translated as dependency intent:
 * - <linux/unistd.h>
 * - "compiler.h"
 * - "crt.h"
 * - "std.h"
 */

/* Original C requires one of _ABIO32, _ABIN32, or _ABI64 to be defined.
 * Unsupported MIPS ABI is a preprocessor error in the source header.
 */

/* Syscalls for MIPS ABI O32 :
 *   - WARNING! there's always a delayed slot!
 *   - WARNING again, the syntax is different, registers take a '$' and numbers
 *     do not.
 *   - registers are 32-bit
 *   - stack is 8-byte aligned
 *   - syscall number is passed in v0 (starts at 0xfa0).
 *   - arguments are in a0, a1, a2, a3, then the stack. The caller needs to
 *     leave some room in the stack for the callee to save a0..a3 if needed.
 *   - Many registers are clobbered, in fact only a0..a2 and s0..s8 are
 *     preserved. See: https://www.linux-mips.org/wiki/Syscall as well as
 *     scall32-o32.S in the kernel sources.
 *   - the system call is performed by calling "syscall"
 *   - syscall return comes in v0, and register a3 needs to be checked to know
 *     if an error occurred, in which case errno is in v0.
 *   - the arguments are cast to long and assigned into the target registers
 *     which are then simply passed as registers to the asm code, so that we
 *     don't have to experience issues with register constraints.
 *
 * Syscalls for MIPS ABI N32, same as ABI O32 with the following differences :
 *   - arguments are in a0, a1, a2, a3, t0, t1, t2, t3.
 *     t0..t3 are also known as a4..a7.
 *   - stack is 16-byte aligned
 */

/* Original _NOLIBC_SYSCALL_CLOBBER_HI_LO:
 * - before MIPS ISA revision 6: "hi", "lo"
 * - MIPS ISA revision 6 or later: "$0"
 */

/* Original _NOLIBC_SYSCALL_CLOBBERLIST:
 * - O32: "memory", "cc", "at", "v1",
 *        "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8", "t9",
 *        _NOLIBC_SYSCALL_CLOBBER_HI_LO
 * - N32/64: "memory", "cc", "at", "v1",
 *           "10", "11", "12", "13", "14", "15", "24", "25",
 *           _NOLIBC_SYSCALL_CLOBBER_HI_LO
 */

#[cfg(_ABIO32)]
type _NOLIBC_SYSCALL_REG = core::ffi::c_long;

#[cfg(any(_ABIN32, _ABI64))]
type _NOLIBC_SYSCALL_REG = core::ffi::c_longlong;

unsafe extern "C" {
    fn __nolibc_entrypoint_epilogue() -> !;
    fn _start_c(stack: *mut core::ffi::c_void) -> !;
}

#[inline(always)]
pub unsafe fn __nolibc_syscall0(num: _NOLIBC_SYSCALL_REG) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let mut _arg4: _NOLIBC_SYSCALL_REG;

    unsafe {
        #[cfg(_ABIO32)]
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            lateout("a3") _arg4,
        );
        #[cfg(any(_ABIN32, _ABI64))]
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            lateout("a3") _arg4,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[inline(always)]
pub unsafe fn __nolibc_syscall1(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let mut _arg4: _NOLIBC_SYSCALL_REG;

    unsafe {
        #[cfg(_ABIO32)]
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            lateout("a3") _arg4,
        );
        #[cfg(any(_ABIN32, _ABI64))]
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("a0") _arg1,
            lateout("a3") _arg4,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[inline(always)]
pub unsafe fn __nolibc_syscall2(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let mut _arg4: _NOLIBC_SYSCALL_REG;

    unsafe {
        #[cfg(_ABIO32)]
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            lateout("a3") _arg4,
        );
        #[cfg(any(_ABIN32, _ABI64))]
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            lateout("a3") _arg4,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[inline(always)]
pub unsafe fn __nolibc_syscall3(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG;

    unsafe {
        #[cfg(_ABIO32)]
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            lateout("a3") _arg4,
        );
        #[cfg(any(_ABIN32, _ABI64))]
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            lateout("a3") _arg4,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[inline(always)]
pub unsafe fn __nolibc_syscall4(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
    arg4: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG = arg4;

    unsafe {
        #[cfg(_ABIO32)]
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            inout("a3") _arg4,
        );
        #[cfg(any(_ABIN32, _ABI64))]
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            inout("a3") _arg4,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[cfg(_ABIO32)]
#[inline(always)]
pub unsafe fn __nolibc_syscall5(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
    arg4: _NOLIBC_SYSCALL_REG,
    arg5: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG = arg4;
    let _arg5: _NOLIBC_SYSCALL_REG = arg5;

    unsafe {
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "sw {arg5}, 16($sp)\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            inout("a3") _arg4,
            arg5 = in(reg) _arg5,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[cfg(_ABIO32)]
#[inline(always)]
pub unsafe fn __nolibc_syscall6(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
    arg4: _NOLIBC_SYSCALL_REG,
    arg5: _NOLIBC_SYSCALL_REG,
    arg6: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG = arg4;
    let _arg5: _NOLIBC_SYSCALL_REG = arg5;
    let _arg6: _NOLIBC_SYSCALL_REG = arg6;

    unsafe {
        core::arch::asm!(
            "addiu $sp, $sp, -32\n",
            "sw {arg5}, 16($sp)\n",
            "sw {arg6}, 20($sp)\n",
            "syscall\n",
            "addiu $sp, $sp, 32\n",
            inout("v0") _num,
            in("a0") _arg1,
            in("a1") _arg2,
            in("a2") _arg3,
            inout("a3") _arg4,
            arg5 = in(reg) _arg5,
            arg6 = in(reg) _arg6,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[cfg(any(_ABIN32, _ABI64))]
#[inline(always)]
pub unsafe fn __nolibc_syscall5(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
    arg4: _NOLIBC_SYSCALL_REG,
    arg5: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG = arg4;
    let _arg5: _NOLIBC_SYSCALL_REG = arg5;

    unsafe {
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("$4") _arg1,
            in("$5") _arg2,
            in("$6") _arg3,
            inout("$7") _arg4,
            in("$8") _arg5,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[cfg(any(_ABIN32, _ABI64))]
#[inline(always)]
pub unsafe fn __nolibc_syscall6(
    num: _NOLIBC_SYSCALL_REG,
    arg1: _NOLIBC_SYSCALL_REG,
    arg2: _NOLIBC_SYSCALL_REG,
    arg3: _NOLIBC_SYSCALL_REG,
    arg4: _NOLIBC_SYSCALL_REG,
    arg5: _NOLIBC_SYSCALL_REG,
    arg6: _NOLIBC_SYSCALL_REG,
) -> _NOLIBC_SYSCALL_REG {
    let mut _num: _NOLIBC_SYSCALL_REG = num;
    let _arg1: _NOLIBC_SYSCALL_REG = arg1;
    let _arg2: _NOLIBC_SYSCALL_REG = arg2;
    let _arg3: _NOLIBC_SYSCALL_REG = arg3;
    let mut _arg4: _NOLIBC_SYSCALL_REG = arg4;
    let _arg5: _NOLIBC_SYSCALL_REG = arg5;
    let _arg6: _NOLIBC_SYSCALL_REG = arg6;

    unsafe {
        core::arch::asm!(
            "syscall\n",
            inout("v0") _num,
            in("$4") _arg1,
            in("$5") _arg2,
            in("$6") _arg3,
            inout("$7") _arg4,
            in("$8") _arg5,
            in("$9") _arg6,
        );
    }
    if _arg4 != 0 { -_num } else { _num }
}

#[cfg(not(NOLIBC_NO_RUNTIME))]
/* startup code, note that it's called __start on MIPS */
#[unsafe(no_mangle)]
#[linkage = "weak"]
pub unsafe extern "C" fn __start() -> ! {
    unsafe {
        core::arch::asm!(
            "move  $a0, $sp\n",       /* save stack pointer to $a0, as arg1 of _start_c */
            #[cfg(_ABIO32)]
            "addiu $sp, $sp, -16\n",  /* the callee expects to save a0..a3 there        */
            "lui $t9, %hi(_start_c)\n", /* ABI requires current function address in $t9 */
            "ori $t9, %lo(_start_c)\n",
            #[cfg(_ABI64)]
            "lui  $t0, %highest(_start_c)\n",
            #[cfg(_ABI64)]
            "ori  $t0, %higher(_start_c)\n",
            #[cfg(_ABI64)]
            "dsll $t0, 0x20\n",
            #[cfg(_ABI64)]
            "or   $t9, $t0\n",
            "jalr $t9\n",             /* transfer to c runtime                          */
            options(noreturn),
        );
    }
}

#[cfg(_ABIO32)]
#[allow(dead_code)]
unsafe fn _sys_ftruncate64(
    fd: core::ffi::c_int,
    length0: u32,
    length1: u32,
) -> core::ffi::c_int {
    unsafe { __nolibc_syscall4(__NR_ftruncate64, fd, 0, length0, length1) as core::ffi::c_int }
}

#[cfg(_ABIO32)]
const _sys_ftruncate64_macro_alias: &str = "_sys_ftruncate64";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
