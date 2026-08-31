/* SPDX-License-Identifier: GPL-2.0-only */

/* Translated from the C header guard SELFTEST_RISCV_CFI_H. */
/* C dependencies: <stddef.h>, <sys/types.h>, and "shadowstack.h". */

pub const CHILD_EXIT_CODE_SSWRITE: i32 = 10;
pub const CHILD_EXIT_CODE_SIG_TEST: i32 = 11;

#[macro_export]
macro_rules! my_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a3") _arg4,
                in("a4") _arg5,
                in("a7") _num,
            );
        }

        _arg1
    }};
}

#[macro_export]
macro_rules! my_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let _num: isize = $num as isize;
        let mut _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;

        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") _arg1,
                in("a1") _arg2,
                in("a2") _arg3,
                in("a7") _num,
            );
        }

        _arg1
    }};
}

/* C fallback when __NR_prctl is not already defined. */
pub const __NR_prctl: isize = 167;

/* C fallback when __NR_map_shadow_stack is not already defined. */
pub const __NR_map_shadow_stack: isize = 453;

pub const CSR_SSP: usize = 0x011;

/* The C __ASM_STR macro stringifies CSR names outside __ASSEMBLY__. */

#[macro_export]
macro_rules! csr_read {
    ($csr:tt) => {{
        let __v: usize;
        unsafe {
            core::arch::asm!(
                concat!("csrr {}, ", stringify!($csr)),
                out(reg) __v,
                options(nostack),
            );
        }
        __v
    }};
}

#[macro_export]
macro_rules! csr_write {
    ($csr:tt, $val:expr) => {{
        let __v: usize = $val as usize;
        unsafe {
            core::arch::asm!(
                concat!("csrw ", stringify!($csr), ", {}"),
                in(reg) __v,
                options(nostack),
            );
        }
    }};
}
