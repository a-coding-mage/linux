/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/* Note : this module is expected to remain private, do not expose it */

/* Dependencies supplied by the surrounding translation unit. */

/* Compiler-specific: ERR_STATIC is static and unused in the C source. */

/* Customization (error_public.h). */
pub type ERR_enum = ZSTD_ErrorCode;

/* Error codes handling. */
#[inline]
pub unsafe fn ERR_isError(code: usize) -> u32 {
    (code > ERROR!(maxCode)) as u32
}

#[inline]
pub unsafe fn ERR_getErrorCode(code: usize) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return 0 as ERR_enum;
    }
    (0usize.wrapping_sub(code)) as ERR_enum
}

/* The error-code enum and ERROR!/ZSTD_error_* names are supplied externally. */
extern "C" {
    pub fn ERR_getErrorString(code: ERR_enum) -> *const ::core::ffi::c_char;
}

#[inline]
pub unsafe fn ERR_getErrorName(code: usize) -> *const ::core::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}

/*
 * Ignore: this is an internal helper.
 *
 * This is a helper function to help force C99-correctness during compilation.
 * Under strict compilation modes, variadic macro arguments can't be empty.
 * However, variadic function arguments can be. Using a function therefore lets
 * us statically check that at least one (string) argument was passed,
 * independent of the compilation flags.
 */
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _force_has_format_string(
    _format: *const ::core::ffi::c_char,
    ...,
) {
}

/* Local Rust equivalents of the C helper macros. */
#[macro_export]
macro_rules! CHECK_V_F {
    ($e:ident, $f:expr) => {
        let $e: usize = $f;
        if unsafe { $crate::ERR_isError($e) } != 0 {
            return $e;
        }
    };
}

#[macro_export]
macro_rules! CHECK_F {
    ($f:expr) => {{
        $crate::CHECK_V_F!(_var_err__, $f);
    }};
}

#[macro_export]
macro_rules! _FORCE_HAS_FORMAT_STRING {
    ($($args:tt)*) => {{
        if false {
            let _ = ($($args)*);
        }
    }};
}

#[macro_export]
macro_rules! ERR_QUOTE {
    ($str:ident) => { stringify!($str) };
    ($str:expr) => { stringify!($str) };
}

/*
 * The following C variadic logging macros retain their control flow and return
 * behavior. RAWLOG and ERROR! are supplied by the surrounding translation.
 */
#[macro_export]
macro_rules! RETURN_ERROR_IF {
    ($cond:expr, $err:ident $(, $args:tt)*) => {{
        if $cond {
            $crate::_FORCE_HAS_FORMAT_STRING!($($args)*);
            return ERROR!($err);
        }
    }};
}

#[macro_export]
macro_rules! RETURN_ERROR {
    ($err:ident $(, $args:tt)*) => {{
        $crate::_FORCE_HAS_FORMAT_STRING!($($args)*);
        return ERROR!($err);
    }};
}

#[macro_export]
macro_rules! FORWARD_IF_ERROR {
    ($err:expr $(, $args:tt)*) => {{
        let err_code: usize = $err;
        if unsafe { $crate::ERR_isError(err_code) } != 0 {
            $crate::_FORCE_HAS_FORMAT_STRING!($($args)*);
            return err_code;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
