/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: __VDSO_LIMITS_H

pub const USHRT_MAX: u16 = !0u16;
pub const SHRT_MAX: i16 = (USHRT_MAX >> 1) as i16;
pub const SHRT_MIN: i16 = -SHRT_MAX - 1;
pub const INT_MAX: i32 = (u32::MAX >> 1) as i32;
pub const INT_MIN: i32 = -INT_MAX - 1;
pub const UINT_MAX: u32 = u32::MAX;
pub const LONG_MAX: i64 = (u64::MAX >> 1) as i64;
pub const LONG_MIN: i64 = -LONG_MAX - 1;
pub const ULONG_MAX: u64 = u64::MAX;
pub const LLONG_MAX: i64 = (u64::MAX >> 1) as i64;
pub const LLONG_MIN: i64 = -LLONG_MAX - 1;
pub const ULLONG_MAX: u64 = u64::MAX;
pub const UINTPTR_MAX: usize = ULONG_MAX as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
