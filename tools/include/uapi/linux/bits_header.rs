/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* bits.h: Macros for dealing with bitmasks. */

// C header guard _UAPI_LINUX_BITS_H omitted in Rust.
// Depends on external Rust equivalents of _UL!, _ULL!, _BIT128!,
// __BITS_PER_LONG, and __BITS_PER_LONG_LONG.

#[macro_export]
macro_rules! __GENMASK {
    ($h:expr, $l:expr) => {
        (((!_UL!(0)) << ($l)) & ((!_UL!(0)) >> (__BITS_PER_LONG - 1 - ($h))))
    };
}

#[macro_export]
macro_rules! __GENMASK_ULL {
    ($h:expr, $l:expr) => {
        (((!_ULL!(0)) << ($l)) & ((!_ULL!(0)) >> (__BITS_PER_LONG_LONG - 1 - ($h))))
    };
}

#[macro_export]
macro_rules! __GENMASK_U128 {
    ($h:expr, $l:expr) => {
        ((_BIT128!(($h)) << 1) - (_BIT128!($l)))
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
