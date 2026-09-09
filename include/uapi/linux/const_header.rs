/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* const_header.rs: Rust translation of Linux constant macros. */

/*
 * The C header provides macros usable from both assembler and C.  Rust has
 * no assembler preprocessor mode here, so these are represented as Rust
 * expression macros.
 */

#[macro_export]
macro_rules! _AC {
    ($x:expr, $y:ident) => { $x };
}

#[macro_export]
macro_rules! _AT {
    ($t:ty, $x:expr) => { $x as $t };
}

#[macro_export]
macro_rules! _UL {
    ($x:expr) => { $x };
}

#[macro_export]
macro_rules! _ULL {
    ($x:expr) => { $x };
}

#[macro_export]
macro_rules! _BITUL {
    ($x:expr) => { (1usize << ($x)) };
}

#[macro_export]
macro_rules! _BITULL {
    ($x:expr) => { (1u64 << ($x)) };
}

/* Missing assembler support for 128-bit constants, as in the C header. */
#[macro_export]
macro_rules! _BIT128 {
    ($x:expr) => { (1u128 << ($x)) };
}

#[macro_export]
macro_rules! __ALIGN_KERNEL_MASK {
    ($x:expr, $mask:expr) => {
        (($x + $mask) & !($mask))
    };
}

#[macro_export]
macro_rules! __ALIGN_KERNEL {
    ($x:expr, $a:expr) => {
        $crate::__ALIGN_KERNEL_MASK!($x, (($a) as _ - 1))
    };
}

#[macro_export]
macro_rules! __KERNEL_DIV_ROUND_UP {
    ($n:expr, $d:expr) => {
        (($n + $d - 1) / $d)
    };
}

/*
 * Divide a positive or negative dividend by a positive or negative divisor
 * and round to the closest integer.  As in C, the result is undefined for
 * the unsigned/sign combinations described by the original macro.
 */
#[macro_export]
macro_rules! __KERNEL_DIV_ROUND_CLOSEST {
    ($x:expr, $divisor:expr) => {{
        let __x = $x;
        let __d = $divisor;
        if ((__x as _ - 1) > 0)
            || ((__d as _ - 1) > 0)
            || ((__x > 0) == (__d > 0))
        {
            (__x + (__d / 2)) / __d
        } else {
            (__x - (__d / 2)) / __d
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
