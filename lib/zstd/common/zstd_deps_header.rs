/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

/*
 * This file provides common libc dependencies that zstd requires.
 * The purpose is to allow replacing this file with a custom implementation
 * to compile zstd without libc support.
 *
 * C header includes and header guards are intentionally omitted.  The
 * corresponding kernel-supplied names remain external dependencies.
 */

/* Need:
 * NULL
 * INT_MAX
 * UINT_MAX
 * ZSTD_memcpy()
 * ZSTD_memset()
 * ZSTD_memmove()
 */

/* Equivalent to the C __builtin_memcpy/__builtin_memmove/__builtin_memset
 * macros.  The raw-pointer operations preserve the original low-level API. */
#[macro_export]
macro_rules! ZSTD_memcpy {
    ($d:expr, $s:expr, $n:expr) => {{
        unsafe { core::ptr::copy_nonoverlapping($s as *const u8, $d as *mut u8, $n as usize) }
    }};
}

#[macro_export]
macro_rules! ZSTD_memmove {
    ($d:expr, $s:expr, $n:expr) => {{
        unsafe { core::ptr::copy($s as *const u8, $d as *mut u8, $n as usize) }
    }};
}

#[macro_export]
macro_rules! ZSTD_memset {
    ($d:expr, $s:expr, $n:expr) => {{
        unsafe { core::ptr::write_bytes($d as *mut u8, $s as u8, $n as usize) }
    }};
}

/* Define malloc as always failing.  Enable this section when the C build
 * defines ZSTD_DEPS_NEED_MALLOC. */
#[cfg(feature = "ZSTD_DEPS_NEED_MALLOC")]
#[macro_export]
macro_rules! ZSTD_malloc {
    ($s:expr) => {{ let _ = $s; core::ptr::null_mut() }};
}

#[cfg(feature = "ZSTD_DEPS_NEED_MALLOC")]
#[macro_export]
macro_rules! ZSTD_free {
    ($p:expr) => {{ let _ = $p; }};
}

#[cfg(feature = "ZSTD_DEPS_NEED_MALLOC")]
#[macro_export]
macro_rules! ZSTD_calloc {
    ($n:expr, $s:expr) => {{ let _ = $n; let _ = $s; core::ptr::null_mut() }};
}

/* Provides 64-bit math support.  Enable this section when the C build
 * defines ZSTD_DEPS_NEED_MATH64; div_u64 is supplied externally. */
#[cfg(feature = "ZSTD_DEPS_NEED_MATH64")]
#[allow(non_snake_case)]
pub unsafe fn ZSTD_div64(dividend: u64, divisor: u32) -> u64 {
    div_u64(dividend, divisor)
}

/* This is only requested when DEBUGLEVEL >= 1.  Enable when the C build
 * defines ZSTD_DEPS_NEED_ASSERT; WARN_ON is supplied externally. */
#[cfg(feature = "ZSTD_DEPS_NEED_ASSERT")]
#[macro_export]
macro_rules! assert {
    ($x:expr) => {{ WARN_ON(!$x) }};
}

/* This is only requested when DEBUGLEVEL >= 2.  Enable when the C build
 * defines ZSTD_DEPS_NEED_IO; pr_debug is supplied externally. */
#[cfg(feature = "ZSTD_DEPS_NEED_IO")]
#[macro_export]
macro_rules! ZSTD_DEBUG_PRINT {
    ($($arg:tt)*) => {{ pr_debug!($($arg)*) }};
}

/* Only requested when MSAN is enabled.  intptr_t is already provided by the
 * common dependencies in the original header and is likewise external here. */
#[cfg(feature = "ZSTD_DEPS_NEED_STDINT")]
const _ZSTD_DEPS_STDINT: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
