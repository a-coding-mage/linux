/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_wrapper.h - arm64 specific wrappers to syscall definitions
 *
 * Based on arch/x86/include/asm_syscall_wrapper.h
 */

// C header guard: __ASM_SYSCALL_WRAPPER_H
// Dependency supplied by the surrounding kernel translation: <asm/ptrace.h>

/* The following items are enabled only when CONFIG_COMPAT is defined. */
#[cfg(CONFIG_COMPAT)]
macro_rules! compat_sc_arm64_regs_to_args {
    ($x:tt, $($args:tt)*) => {
        __MAP!($x, __SC_ARGS, , , regs->regs[0], , regs->regs[1], , regs->regs[2],
               , regs->regs[3], , regs->regs[4], , regs->regs[5])
    };
}

#[cfg(CONFIG_COMPAT)]
macro_rules! compat_syscall_definex {
    ($x:tt, $name:ident, $($args:tt)*) => {
        // asmlinkage long __arm64_compat_sys$name(const struct pt_regs *regs);
        // ALLOW_ERROR_INJECTION(__arm64_compat_sys$name, ERRNO);
        // static long __se_compat_sys$name(__MAP($x,__SC_LONG,$($args)*));
        // static inline long __do_compat_sys$name(__MAP($x,__SC_DECL,$($args)*));
        // The generated C entry point invokes __se_compat_sys$name with the
        // six argument registers, which in turn invokes __do_compat_sys$name.
        $crate::__compat_syscall_definition!($x, $name, $($args)*);
    };
}

#[cfg(CONFIG_COMPAT)]
macro_rules! compat_syscall_define0 {
    ($sname:ident) => {
        // asmlinkage long __arm64_compat_sys_$sname(const struct pt_regs *__unused);
        // ALLOW_ERROR_INJECTION(__arm64_compat_sys_$sname, ERRNO);
        $crate::__compat_syscall_zero!($sname);
    };
}

#[cfg(CONFIG_COMPAT)]
macro_rules! cond_syscall_compat {
    ($name:ident) => {
        // asmlinkage long __arm64_compat_sys_$name(const struct pt_regs *regs);
        // Weak implementation returns sys_ni_syscall().
        $crate::__conditional_compat_syscall!($name);
    };
}

macro_rules! sc_arm64_regs_to_args {
    ($x:tt, $($args:tt)*) => {
        __MAP!($x, __SC_ARGS, , , regs->orig_x0, , regs->regs[1], , regs->regs[2],
               , regs->regs[3], , regs->regs[4], , regs->regs[5])
    };
}

macro_rules! __syscall_definex {
    ($x:tt, $name:ident, $($args:tt)*) => {
        // asmlinkage long __arm64_sys$name(const struct pt_regs *regs);
        // ALLOW_ERROR_INJECTION(__arm64_sys$name, ERRNO);
        // static long __se_sys$name(__MAP($x,__SC_LONG,$($args)*));
        // static inline long __do_sys$name(__MAP($x,__SC_DECL,$($args)*));
        // The entry point passes SC_ARM64_REGS_TO_ARGS to __se_sys$name.
        // __se_sys$name calls __do_sys$name, performs __SC_TEST and __PROTECT,
        // then returns ret.
        $crate::__syscall_definition!($x, $name, $($args)*);
    };
}

macro_rules! syscall_define0 {
    ($sname:ident) => {
        // SYSCALL_METADATA(_$sname, 0);
        // asmlinkage long __arm64_sys_$sname(const struct pt_regs *__unused);
        // ALLOW_ERROR_INJECTION(__arm64_sys_$sname, ERRNO);
        $crate::__syscall_zero!($sname);
    };
}

macro_rules! cond_syscall {
    ($name:ident) => {
        // asmlinkage long __arm64_sys_$name(const struct pt_regs *regs);
        // Weak implementation returns sys_ni_syscall().
        $crate::__conditional_syscall!($name);
    };
}

// asmlinkage long __arm64_sys_ni_syscall(const struct pt_regs *__unused);
extern "C" {
    pub fn __arm64_sys_ni_syscall(__unused: *const crate::pt_regs) -> libc::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
