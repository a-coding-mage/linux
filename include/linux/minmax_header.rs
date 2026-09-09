/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: linux/build_bug.h, linux/compiler.h,
// linux/const.h, and linux/types.h.
// C preprocessor type-checking helpers (__typecheck, __sign_use, __types_ok,
// and BUILD_BUG_ON_MSG) have no direct file-local Rust equivalent.

#[inline]
pub fn in_range64(val: u64, start: u64, len: u64) -> bool {
    val.wrapping_sub(start) < len
}

#[inline]
pub fn in_range32(val: u32, start: u32, len: u32) -> bool {
    val.wrapping_sub(start) < len
}

#[macro_export]
macro_rules! min {
    ($x:expr, $y:expr) => {{
        let __x = $x;
        let __y = $y;
        if __x < __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! max {
    ($x:expr, $y:expr) => {{
        let __x = $x;
        let __y = $y;
        if __x > __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! umin {
    ($x:expr, $y:expr) => {{
        let __x = ($x as u128);
        let __y = ($y as u128);
        if __x < __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! umax {
    ($x:expr, $y:expr) => {{
        let __x = ($x as u128);
        let __y = ($y as u128);
        if __x > __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! min3 {
    ($x:expr, $y:expr, $z:expr) => {{
        let __x = $x;
        let __y = $y;
        let __z = $z;
        if __x < __y { if __x < __z { __x } else { __z } }
        else if __y < __z { __y } else { __z }
    }};
}

#[macro_export]
macro_rules! max3 {
    ($x:expr, $y:expr, $z:expr) => {{
        let __x = $x;
        let __y = $y;
        let __z = $z;
        if __x > __y { if __x > __z { __x } else { __z } }
        else if __y > __z { __y } else { __z }
    }};
}

#[macro_export]
macro_rules! min_t {
    ($ty:ty, $x:expr, $y:expr) => {{
        let __x: $ty = $x as $ty;
        let __y: $ty = $y as $ty;
        if __x < __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! max_t {
    ($ty:ty, $x:expr, $y:expr) => {{
        let __x: $ty = $x as $ty;
        let __y: $ty = $y as $ty;
        if __x > __y { __x } else { __y }
    }};
}

#[macro_export]
macro_rules! min_not_zero {
    ($x:expr, $y:expr) => {{
        let __x = $x;
        let __y = $y;
        if __x == 0 { __y } else if __y == 0 { __x } else { min!(__x, __y) }
    }};
}

#[macro_export]
macro_rules! clamp {
    ($val:expr, $lo:expr, $hi:expr) => {{
        let __val = $val;
        let __lo = $lo;
        let __hi = $hi;
        if __val >= __hi { __hi } else if __val <= __lo { __lo } else { __val }
    }};
}

#[macro_export]
macro_rules! clamp_t {
    ($ty:ty, $val:expr, $lo:expr, $hi:expr) => {{
        let __val: $ty = $val as $ty;
        let __lo: $ty = $lo as $ty;
        let __hi: $ty = $hi as $ty;
        if __val >= __hi { __hi } else if __val <= __lo { __lo } else { __val }
    }};
}

#[macro_export]
macro_rules! clamp_val {
    ($val:expr, $lo:expr, $hi:expr) => {{
        let __val = $val;
        let __lo = $lo;
        let __hi = $hi;
        if __val >= __hi { __hi } else if __val <= __lo { __lo } else { __val }
    }};
}

#[macro_export]
macro_rules! min_array {
    ($array:expr, $len:expr) => {{
        let __array = $array;
        let mut __len = $len;
        __len -= 1;
        let mut __element = __array[__len];
        while __len != 0 {
            __len -= 1;
            __element = min!(__element, __array[__len]);
        }
        __element
    }};
}

#[macro_export]
macro_rules! max_array {
    ($array:expr, $len:expr) => {{
        let __array = $array;
        let mut __len = $len;
        __len -= 1;
        let mut __element = __array[__len];
        while __len != 0 {
            __len -= 1;
            __element = max!(__element, __array[__len]);
        }
        __element
    }};
}

#[macro_export]
macro_rules! in_range {
    ($val:expr, $start:expr, $len:expr) => {{
        let __val = $val;
        let __start = $start;
        let __len = $len;
        in_range64(__val as u64, __start as u64, __len as u64)
    }};
}

#[macro_export]
macro_rules! swap {
    ($a:expr, $b:expr) => {{
        core::mem::swap(&mut $a, &mut $b);
    }};
}

#[macro_export]
macro_rules! MIN { ($a:expr, $b:expr) => { if $a < $b { $a } else { $b } }; }
#[macro_export]
macro_rules! MAX { ($a:expr, $b:expr) => { if $a > $b { $a } else { $b } }; }
#[macro_export]
macro_rules! MIN_T { ($ty:ty, $a:expr, $b:expr) => { min_t!($ty, $a, $b) }; }
#[macro_export]
macro_rules! MAX_T { ($ty:ty, $a:expr, $b:expr) => { max_t!($ty, $a, $b) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
