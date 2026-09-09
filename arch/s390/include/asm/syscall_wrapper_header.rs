/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_wrapper.h - s390 specific wrappers to syscall definitions
 *
 */

// Header guard: _ASM_S390_SYSCALL_WRAPPER_H

/* Mapping of registers to parameters for syscalls */
// The __MAP/__SC_ARGS machinery and the pt_regs type are supplied externally.
macro_rules! SC_S390_REGS_TO_ARGS {
    ($x:tt, $($args:tt)*) => {
        __MAP!($x, __SC_ARGS!,
            , , regs.orig_gpr2, , regs.gprs[3], , regs.gprs[4],
            , , regs.gprs[5], , regs.gprs[6], , regs.gprs[7])
    };
}

macro_rules! SYSCALL_DEFINE0 {
    ($sname:ident) => {
        SYSCALL_METADATA!(_$sname, 0);
        // C token-pasting (`__s390x_sys_##sname`, `__do_sys_##sname`) is
        // retained here as macro intent; Rust identifier concatenation is a
        // build-environment concern.
        extern "C" {
            fn __s390x_sys_$sname(__unused: *mut pt_regs) -> core::ffi::c_long;
        }
        ALLOW_ERROR_INJECTION!(__s390x_sys_$sname, ERRNO);
        // static inline long __do_sys_##sname(void)
    };
}

macro_rules! COND_SYSCALL {
    ($name:ident) => {
        cond_syscall!(__s390x_sys_$name)
    };
}

macro_rules! __S390_SYS_STUBx {
    ($x:tt, $fullname:ident, $name:ident, $($args:tt)*) => {};
}

macro_rules! __SYSCALL_DEFINEx {
    ($x:tt, $name:ident, $($args:tt)*) => {
        // The following declarations and definitions preserve the C wrapper
        // structure. Identifier token-pasting is intentionally left for the
        // surrounding generated-bindings/build environment.
        // long __s390x_sys##name(struct pt_regs *regs);
        ALLOW_ERROR_INJECTION!(__s390x_sys##$name, ERRNO);
        // static inline long __se_sys##name(__MAP(x, __SC_LONG, __VA_ARGS__));
        // static inline long __do_sys##name(__MAP(x, __SC_DECL, __VA_ARGS__));
        __S390_SYS_STUBx!($x, $name, $($args)*);
        // long __s390x_sys##name(struct pt_regs *regs)
        // {
        //     return __se_sys##name(SC_S390_REGS_TO_ARGS(x, __VA_ARGS__));
        // }
        // static inline long __se_sys##name(__MAP(x, __SC_LONG, __VA_ARGS__))
        // {
        //     __MAP(x, __SC_TEST, __VA_ARGS__);
        //     return __do_sys##name(__MAP(x, __SC_CAST, __VA_ARGS__));
        // }
        // static inline long __do_sys##name(__MAP(x, __SC_DECL, __VA_ARGS__))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
