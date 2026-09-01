/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Standard definitions and types for NOLIBC
 * Copyright (C) 2023 Vincent Dagonneau <v@vda.io>
 */

pub type uint8_t = u8;
pub type int8_t = i8;
pub type uint16_t = u16;
pub type int16_t = i16;
pub type uint32_t = u32;
pub type int32_t = i32;
pub type uint64_t = u64;
pub type int64_t = i64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;

pub type int_least8_t = int8_t;
pub type uint_least8_t = uint8_t;
pub type int_least16_t = int16_t;
pub type uint_least16_t = uint16_t;
pub type int_least32_t = int32_t;
pub type uint_least32_t = uint32_t;
pub type int_least64_t = int64_t;
pub type uint_least64_t = uint64_t;

pub type int_fast8_t = int8_t;
pub type uint_fast8_t = uint8_t;
pub type int_fast16_t = ssize_t;
pub type uint_fast16_t = size_t;
pub type int_fast32_t = ssize_t;
pub type uint_fast32_t = size_t;
pub type int_fast64_t = int64_t;
pub type uint_fast64_t = uint64_t;

pub type intmax_t = i64;
pub type uintmax_t = u64;

/* limits of integral types */

pub const INT8_MIN: int8_t = -128;
pub const INT16_MIN: int16_t = -32768;
pub const INT32_MIN: int32_t = -2147483648;
pub const INT64_MIN: int64_t = -9223372036854775808;

pub const INT8_MAX: int8_t = 127;
pub const INT16_MAX: int16_t = 32767;
pub const INT32_MAX: int32_t = 2147483647;
pub const INT64_MAX: int64_t = 9223372036854775807;

pub const UINT8_MAX: uint8_t = 255;
pub const UINT16_MAX: uint16_t = 65535;
pub const UINT32_MAX: uint32_t = 4294967295;
pub const UINT64_MAX: uint64_t = 18446744073709551615;

pub const INT_LEAST8_MIN: int_least8_t = INT8_MIN;
pub const INT_LEAST16_MIN: int_least16_t = INT16_MIN;
pub const INT_LEAST32_MIN: int_least32_t = INT32_MIN;
pub const INT_LEAST64_MIN: int_least64_t = INT64_MIN;

pub const INT_LEAST8_MAX: int_least8_t = INT8_MAX;
pub const INT_LEAST16_MAX: int_least16_t = INT16_MAX;
pub const INT_LEAST32_MAX: int_least32_t = INT32_MAX;
pub const INT_LEAST64_MAX: int_least64_t = INT64_MAX;

pub const UINT_LEAST8_MAX: uint_least8_t = UINT8_MAX;
pub const UINT_LEAST16_MAX: uint_least16_t = UINT16_MAX;
pub const UINT_LEAST32_MAX: uint_least32_t = UINT32_MAX;
pub const UINT_LEAST64_MAX: uint_least64_t = UINT64_MAX;

pub const SIZE_MAX: size_t = usize::MAX;
pub const INTPTR_MIN: intptr_t = isize::MIN;
pub const INTPTR_MAX: intptr_t = isize::MAX;
pub const PTRDIFF_MIN: ptrdiff_t = INTPTR_MIN;
pub const PTRDIFF_MAX: ptrdiff_t = INTPTR_MAX;
pub const UINTPTR_MAX: uintptr_t = SIZE_MAX;

pub const INT_FAST8_MIN: int_fast8_t = INT8_MIN;
pub const INT_FAST16_MIN: int_fast16_t = INTPTR_MIN;
pub const INT_FAST32_MIN: int_fast32_t = INTPTR_MIN;
pub const INT_FAST64_MIN: int_fast64_t = INT64_MIN;

pub const INT_FAST8_MAX: int_fast8_t = INT8_MAX;
pub const INT_FAST16_MAX: int_fast16_t = INTPTR_MAX;
pub const INT_FAST32_MAX: int_fast32_t = INTPTR_MAX;
pub const INT_FAST64_MAX: int_fast64_t = INT64_MAX;

pub const UINT_FAST8_MAX: uint_fast8_t = UINT8_MAX;
pub const UINT_FAST16_MAX: uint_fast16_t = SIZE_MAX;
pub const UINT_FAST32_MAX: uint_fast32_t = SIZE_MAX;
pub const UINT_FAST64_MAX: uint_fast64_t = UINT64_MAX;

pub const INTMAX_MIN: intmax_t = INT64_MIN;
pub const INTMAX_MAX: intmax_t = INT64_MAX;
pub const UINTMAX_MAX: uintmax_t = UINT64_MAX;

/* C source defines these only if not already defined. */
pub const INT_MIN: i32 = i32::MIN;
pub const INT_MAX: i32 = i32::MAX;

pub const LONG_MIN: isize = isize::MIN;
pub const LONG_MAX: isize = isize::MAX;

pub const ULONG_MAX: usize = usize::MAX;

pub const LLONG_MIN: i64 = i64::MIN;
pub const LLONG_MAX: i64 = i64::MAX;

pub const ULLONG_MAX: u64 = u64::MAX;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
