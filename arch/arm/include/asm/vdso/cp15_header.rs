/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

// The following declarations are enabled when CONFIG_CPU_CP15 is configured.
// The original C header also excludes them during assembly builds.

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! __ACCESS_CP15 {
    ($crn:ident, $op1:literal, $crm:ident, $op2:literal) => {
        ("mrc", "mcr", stringify!(p15, $op1, %0, $crn, $crm, $op2), u32)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! __ACCESS_CP15_64 {
    ($op1:literal, $crm:ident) => {
        ("mrrc", "mcrr", stringify!(p15, $op1, %Q0, %R0, $crm), u64)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! __read_sysreg {
    ($r:expr, $w:expr, $c:expr, $t:ty) => {{
        let mut __val: $t;
        unsafe {
            core::arch::asm!(concat!($r, " ", $c), out(reg) __val);
        }
        __val
    }};
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! read_sysreg {
    ($($args:tt)*) => {
        __read_sysreg!($($args)*)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! __write_sysreg {
    ($v:expr, $r:expr, $w:expr, $c:expr, $t:ty) => {
        unsafe {
            core::arch::asm!(concat!($w, " ", $c), in(reg) (($v) as $t));
        }
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! write_sysreg {
    ($v:expr, $($args:tt)*) => {
        __write_sysreg!($v, $($args)*)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! BPIALL {
    () => {
        __ACCESS_CP15!(c7, 0, c5, 6)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! ICIALLU {
    () => {
        __ACCESS_CP15!(c7, 0, c5, 0)
    };
}

#[cfg(feature = "CONFIG_CPU_CP15")]
macro_rules! CNTVCT {
    () => {
        __ACCESS_CP15_64!(1, c14)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
