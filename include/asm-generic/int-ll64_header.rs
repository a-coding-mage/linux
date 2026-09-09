/* SPDX-License-Identifier: GPL-2.0 */
/*
 * asm-generic/int-ll64.h
 *
 * Integer declarations for architectures which use "long long"
 * for 64-bit types.
 */

// Dependency supplied by the corresponding UAPI header:
// #include <uapi/asm-generic/int-ll64.h>

pub type s8 = __s8;
pub type u8 = __u8;
pub type s16 = __s16;
pub type u16 = __u16;
pub type s32 = __s32;
pub type u32 = __u32;
pub type s64 = __s64;
pub type u64 = __u64;

macro_rules! S8_C {
    ($x:expr) => { $x };
}

macro_rules! U8_C {
    ($x:expr) => { $x as u8 };
}

macro_rules! S16_C {
    ($x:expr) => { $x };
}

macro_rules! U16_C {
    ($x:expr) => { $x as u16 };
}

macro_rules! S32_C {
    ($x:expr) => { $x };
}

macro_rules! U32_C {
    ($x:expr) => { $x as u32 };
}

macro_rules! S64_C {
    ($x:expr) => { $x as i64 };
}

macro_rules! U64_C {
    ($x:expr) => { $x as u64 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
