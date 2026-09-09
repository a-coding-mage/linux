/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/args.h> supplies CONCATENATE and COUNT_ARGS.

pub const KSTRTOX_OVERFLOW: u32 = 1u32 << 31;

extern "C" {
    pub fn _parse_integer_fixup_radix(
        s: *const core::ffi::c_char,
        base: *mut core::ffi::c_uint,
    ) -> *const core::ffi::c_char;

    pub fn _parse_integer_limit(
        s: *const core::ffi::c_char,
        base: core::ffi::c_uint,
        res: *mut core::ffi::c_ulonglong,
        max_chars: usize,
        init: core::ffi::c_ulonglong,
    ) -> core::ffi::c_uint;
}

#[macro_export]
macro_rules! _parse_integer0 {
    ($s:expr, $base:expr, $res:expr $(, $args:expr)*) => {
        unsafe {
            $crate::_parse_integer_limit(
                $s,
                $base,
                $res,
                ::core::ffi::c_int::MAX as usize,
                0,
            )
        }
    };
}

#[macro_export]
macro_rules! _parse_integer1 {
    ($s:expr, $base:expr, $res:expr, $max_chars:expr $(, $args:expr)*) => {
        unsafe {
            $crate::_parse_integer_limit($s, $base, $res, $max_chars, 0)
        }
    };
}

#[macro_export]
macro_rules! _parse_integer2 {
    ($s:expr, $base:expr, $res:expr, $max_chars:expr, $init:expr $(, $args:expr)*) => {
        unsafe {
            $crate::_parse_integer_limit($s, $base, $res, $max_chars, $init)
        }
    };
}

// Equivalent to CONCATENATE(_parse_integer, COUNT_ARGS(__VA_ARGS__))(...).
// The supported arities are the declaration forms provided above.
#[macro_export]
macro_rules! _parse_integer {
    ($s:expr, $base:expr, $res:expr) => {
        $crate::_parse_integer0!($s, $base, $res)
    };
    ($s:expr, $base:expr, $res:expr, $max_chars:expr) => {
        $crate::_parse_integer1!($s, $base, $res, $max_chars)
    };
    ($s:expr, $base:expr, $res:expr, $max_chars:expr, $init:expr) => {
        $crate::_parse_integer2!($s, $base, $res, $max_chars, $init)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
