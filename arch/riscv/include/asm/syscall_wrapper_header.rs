/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_wrapper.h - riscv specific wrappers to syscall definitions
 *
 * Based on arch/arm64/include/syscall_wrapper.h
 */

// The C header includes <asm/ptrace.h>; `pt_regs` is supplied by that dependency.

extern "C" {
    pub fn __riscv_sys_ni_syscall(regs: *const pt_regs) -> ::core::ffi::c_long;
}

#[cfg(target_pointer_width = "64")]
macro_rules! __SYSCALL_SE_DEFINEx {
    ($x:expr, $prefix:ident, $name:ident, $($args:tt)*) => {
        // C declaration/definition pair: static long __se_##prefix##name(...).
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! SC_RISCV_REGS_TO_ARGS {
    ($x:expr, $regs:expr $(, $args:tt)*) => {
        $regs.orig_a0, $regs.a1, $regs.a2, $regs.a3, $regs.a4, $regs.a5, $regs.a6
    };
}

#[cfg(not(target_pointer_width = "64"))]
/*
 * On 32-bit RISC-V, C uses type aliasing and an alias symbol to sanitize
 * syscall arguments when arguments wider than a word are present.
 */
macro_rules! __SYSCALL_SE_DEFINEx {
    ($x:expr, $prefix:ident, $name:ident, $($args:tt)*) => {
        // The C __attribute__((alias(...))), diagnostics, noinline, and used
        // declarations are represented by the generated implementation.
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! SC_RISCV_REGS_TO_ARGS {
    ($x:expr, $regs:expr $(, $args:tt)*) => {
        $regs.orig_a0, $regs.a1, $regs.a2, $regs.a3, $regs.a4, $regs.a5, $regs.a6
    };
}

#[cfg(feature = "compat")]
macro_rules! COMPAT_SYSCALL_DEFINEx {
    ($x:expr, $name:ident, $($args:tt)*) => {
        // C declares __riscv_compat_sys$name, its error-injection metadata,
        // the argument conversion wrapper, and the inline implementation.
    };
}

#[cfg(feature = "compat")]
macro_rules! COMPAT_SYSCALL_DEFINE0 {
    ($sname:ident) => {
        // C declares asmlinkage long __riscv_compat_sys_$sname.
    };
}

#[cfg(feature = "compat")]
macro_rules! COND_SYSCALL_COMPAT {
    ($name:ident) => {
        // C weakly defines __riscv_compat_sys_$name to call sys_ni_syscall().
    };
}

macro_rules! __SYSCALL_DEFINEx {
    ($x:expr, $name:ident, $($args:tt)*) => {
        // C declares __riscv_sys$name, error-injection metadata, the
        // argument conversion wrapper, and the inline syscall implementation.
    };
}

macro_rules! SYSCALL_DEFINE0 {
    ($sname:ident) => {
        // C emits SYSCALL_METADATA(_$sname, 0) and declares
        // asmlinkage long __riscv_sys_$sname.
    };
}

macro_rules! COND_SYSCALL {
    ($name:ident) => {
        // C weakly defines __riscv_sys_$name to call sys_ni_syscall().
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
