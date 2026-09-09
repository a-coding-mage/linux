/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Compile time versions of __arch_hweightN().
 *
 * The C __builtin_constant_p and BUILD_BUG_ON_ZERO facilities are retained
 * as dependency-provided macros in the translated interfaces below.
 */
macro_rules! __const_hweight8 {
    ($w:expr) => {
        (($w as u64 & (1u64 << 0) != 0) as u32
            + ($w as u64 & (1u64 << 1) != 0) as u32
            + ($w as u64 & (1u64 << 2) != 0) as u32
            + ($w as u64 & (1u64 << 3) != 0) as u32
            + ($w as u64 & (1u64 << 4) != 0) as u32
            + ($w as u64 & (1u64 << 5) != 0) as u32
            + ($w as u64 & (1u64 << 6) != 0) as u32
            + ($w as u64 & (1u64 << 7) != 0) as u32)
    };
}

macro_rules! __const_hweight16 {
    ($w:expr) => {
        (__const_hweight8!($w) + __const_hweight8!(($w) >> 8))
    };
}

macro_rules! __const_hweight32 {
    ($w:expr) => {
        (__const_hweight16!($w) + __const_hweight16!(($w) >> 16))
    };
}

macro_rules! __const_hweight64 {
    ($w:expr) => {
        (__const_hweight32!($w) + __const_hweight32!(($w) >> 32))
    };
}

/* Generic interface. */
macro_rules! hweight8 {
    ($w:expr) => {
        if __builtin_constant_p!($w) {
            __const_hweight8!($w)
        } else {
            __arch_hweight8($w)
        }
    };
}

macro_rules! hweight16 {
    ($w:expr) => {
        if __builtin_constant_p!($w) {
            __const_hweight16!($w)
        } else {
            __arch_hweight16($w)
        }
    };
}

macro_rules! hweight32 {
    ($w:expr) => {
        if __builtin_constant_p!($w) {
            __const_hweight32!($w)
        } else {
            __arch_hweight32($w)
        }
    };
}

macro_rules! hweight64 {
    ($w:expr) => {
        if __builtin_constant_p!($w) {
            __const_hweight64!($w)
        } else {
            __arch_hweight64($w)
        }
    };
}

/* Interface for known constant arguments. */
macro_rules! HWEIGHT8 {
    ($w:expr) => { (BUILD_BUG_ON_ZERO!(!__builtin_constant_p!($w)) + __const_hweight8!($w)) };
}

macro_rules! HWEIGHT16 {
    ($w:expr) => { (BUILD_BUG_ON_ZERO!(!__builtin_constant_p!($w)) + __const_hweight16!($w)) };
}

macro_rules! HWEIGHT32 {
    ($w:expr) => { (BUILD_BUG_ON_ZERO!(!__builtin_constant_p!($w)) + __const_hweight32!($w)) };
}

macro_rules! HWEIGHT64 {
    ($w:expr) => { (BUILD_BUG_ON_ZERO!(!__builtin_constant_p!($w)) + __const_hweight64!($w)) };
}

/* Type invariant interface to the compile time constant hweight functions. */
macro_rules! HWEIGHT {
    ($w:expr) => { HWEIGHT64!($w as u64) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
