/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_wrapper.h - powerpc specific wrappers to syscall definitions
 *
 * Based on arch/{x86,arm64}/include/asm/syscall_wrapper.h
 */

// C header guard: __ASM_POWERPC_SYSCALL_WRAPPER_H

/// Opaque declaration corresponding to `struct pt_regs`.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/*
 * C macro equivalent.  The external `__MAP` and `__SC_ARGS` macros are
 * supplied by the syscall infrastructure in the containing translation unit.
 */
#[macro_export]
macro_rules! SC_POWERPC_REGS_TO_ARGS {
    ($x:tt, $($args:tt)*) => {
        __MAP!($x, __SC_ARGS!, , regs.gpr[3], , regs.gpr[4], , regs.gpr[5],
               , regs.gpr[6], , regs.gpr[7], , regs.gpr[8])
    };
}

/*
 * __SYSCALL_DEFINEx is retained as a Rust macro.  Rust has no direct token
 * pasting equivalent for the C `sys##name`, `__se_sys##name`, and
 * `__do_sys##name` identifiers; callers provide the corresponding identifiers
 * explicitly while the external syscall helper macros remain dependencies.
 */
#[macro_export]
macro_rules! __SYSCALL_DEFINEx {
    (
        $x:tt,
        $sys_name:ident,
        $se_name:ident,
        $do_name:ident,
        ($($decl:tt)*),
        ($($args:tt)*)
    ) => {
        unsafe extern "C" {
            fn $sys_name(regs: *const $crate::pt_regs) -> isize;
        }
        #[allow(non_snake_case)]
        unsafe fn $se_name($($decl)*) -> isize {
            let ret = $do_name($($args)*);
            ret
        }
        #[allow(non_snake_case)]
        unsafe fn $do_name($($decl)*) -> isize;
    };
}

#[macro_export]
macro_rules! SYSCALL_DEFINE0 {
    ($sname:ident, $sys_name:ident) => {
        SYSCALL_METADATA!(_$sname, 0);
        unsafe extern "C" {
            fn $sys_name(__unused: *const $crate::pt_regs) -> isize;
        }
        ALLOW_ERROR_INJECTION!($sys_name, ERRNO);
    };
}

#[macro_export]
macro_rules! COND_SYSCALL {
    ($name:ident, $sys_name:ident) => {
        unsafe extern "C" {
            fn $sys_name(regs: *const $crate::pt_regs) -> isize;
        }
        #[no_mangle]
        pub unsafe extern "C" fn $sys_name(_regs: *const $crate::pt_regs) -> isize {
            sys_ni_syscall()
        }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
