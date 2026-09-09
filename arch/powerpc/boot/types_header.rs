/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <stdbool.h>

macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        core::mem::size_of_val(&$x) / core::mem::size_of_val(&$x[0])
    };
}

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type s8 = ::core::ffi::c_schar;
pub type s16 = ::core::ffi::c_short;
pub type s32 = ::core::ffi::c_int;
pub type s64 = ::core::ffi::c_longlong;

/* required for opal-api.h */
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int8_t = s8;
pub type int16_t = s16;
pub type int32_t = s32;
pub type int64_t = s64;

macro_rules! min {
    ($x:expr, $y:expr) => {{
        let _x = $x;
        let _y = $y;
        let _ = (&_x as *const _, &_y as *const _);
        if _x < _y { _x } else { _y }
    }};
}

macro_rules! max {
    ($x:expr, $y:expr) => {{
        let _x = $x;
        let _y = $y;
        let _ = (&_x as *const _, &_y as *const _);
        if _x > _y { _x } else { _y }
    }};
}

macro_rules! min_t {
    ($type:ty, $a:expr, $b:expr) => {
        min!($a as $type, $b as $type)
    };
}

macro_rules! max_t {
    ($type:ty, $a:expr, $b:expr) => {
        max!($a as $type, $b as $type)
    };
}

pub type bool = ::core::ffi::c_int;

pub const true: bool = 1;
pub const false: bool = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
