/* SPDX-License-Identifier: GPL-2.0 */

// Macros to assist the sharing of assembler code between 32-bit and
// 64-bit sparc.
//
// CONFIG_SPARC64 is represented as the Rust cfg feature of the same name.

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH32 {
    ($type:tt, $predict:tt, $dest:tt) => {
        concat!(stringify!($type), ",", stringify!($predict), " %icc, ", stringify!($dest))
    };
}

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH32_ANNUL {
    ($type:tt, $predict:tt, $dest:tt) => {
        concat!(stringify!($type), ",a,", stringify!($predict), " %icc, ", stringify!($dest))
    };
}

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH_REG_ZERO {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("brz,", stringify!($predict), " ", stringify!($reg), ", ", stringify!($dest))
    };
}

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH_REG_ZERO_ANNUL {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("brz,a,", stringify!($predict), " ", stringify!($reg), ", ", stringify!($dest))
    };
}

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH_REG_NOT_ZERO {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("brnz,", stringify!($predict), " ", stringify!($reg), ", ", stringify!($dest))
    };
}

#[cfg(feature = "CONFIG_SPARC64")]
macro_rules! BRANCH_REG_NOT_ZERO_ANNUL {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("brnz,a,", stringify!($predict), " ", stringify!($reg), ", ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH32 {
    ($type:tt, $predict:tt, $dest:tt) => {
        concat!(stringify!($type), " ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH32_ANNUL {
    ($type:tt, $predict:tt, $dest:tt) => {
        concat!(stringify!($type), ",a ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH_REG_ZERO {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("cmp ", stringify!($reg), ", 0; be ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH_REG_ZERO_ANNUL {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("cmp ", stringify!($reg), ", 0; be,a ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH_REG_NOT_ZERO {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("cmp ", stringify!($reg), ", 0; bne ", stringify!($dest))
    };
}

#[cfg(not(feature = "CONFIG_SPARC64"))]
macro_rules! BRANCH_REG_NOT_ZERO_ANNUL {
    ($predict:tt, $reg:tt, $dest:tt) => {
        concat!("cmp ", stringify!($reg), ", 0; bne,a ", stringify!($dest))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
