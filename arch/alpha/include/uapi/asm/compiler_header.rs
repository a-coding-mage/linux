/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * These macros describe instruction patterns used by the Alpha kernel ABI.
 * The original header selects compiler builtins when available and otherwise
 * emits the corresponding inline assembly.
 */

/* GCC version condition: __GNUC__ == 3 && __GNUC_MINOR__ >= 4 || __GNUC__ > 3 */
#[macro_export]
macro_rules! __kernel_insbl {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("insbl {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_inswl {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("inswl {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_insql {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("insql {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_inslh {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("inslh {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_extbl {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("extbl {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_extwl {
    ($val:expr, $shift:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("extwl {val},{shift},{out}", val = in(reg) $val, shift = in(reg) $shift, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_cmpbge {
    ($a:expr, $b:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!("cmpbge {a},{b},{out}", a = in(reg) $a, b = in(reg) $b, out = lateout(reg) __kir); }
        __kir
    }};
}

/* __alpha_cix__ / compiler-version conditional: Alpha count instructions. */
#[macro_export]
macro_rules! __kernel_cttz {
    ($x:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!(".arch ev67; cttz {x},{out}", x = in(reg) $x, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_ctlz {
    ($x:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!(".arch ev67; ctlz {x},{out}", x = in(reg) $x, out = lateout(reg) __kir); }
        __kir
    }};
}
#[macro_export]
macro_rules! __kernel_ctpop {
    ($x:expr) => {{
        let mut __kir: ::core::ffi::c_ulong;
        unsafe { ::core::arch::asm!(".arch ev67; ctpop {x},{out}", x = in(reg) $x, out = lateout(reg) __kir); }
        __kir
    }};
}

/* __alpha_bwx__ provides direct memory operations; otherwise these names are absent. */
#[cfg(target_arch = "alpha")]
#[macro_export]
macro_rules! __kernel_ldbu { ($mem:expr) => { $mem }; }
#[cfg(target_arch = "alpha")]
#[macro_export]
macro_rules! __kernel_ldwu { ($mem:expr) => { $mem }; }
#[cfg(target_arch = "alpha")]
#[macro_export]
macro_rules! __kernel_stb { ($val:expr, $mem:expr) => {{ $mem = $val; }}; }
#[cfg(target_arch = "alpha")]
#[macro_export]
macro_rules! __kernel_stw { ($val:expr, $mem:expr) => {{ $mem = $val; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
