/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* ******************************************************************
 * debug
 * Part of FSE library
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * You can contact the author at :
 * - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
******************************************************************/

/*
 * The purpose of this header is to enable debug functions.
 * They regroup assert(), DEBUGLOG() and RAWLOG() for run-time,
 * and DEBUG_STATIC_ASSERT() for compile-time.
 *
 * By default, DEBUGLEVEL==0, which means run-time debug is disabled.
 */

/* DEBUGLEVEL is expected to be defined externally, typically through compiler
 * command line.  Value must be a number.  The Rust build may provide an
 * equivalent configuration; this fallback preserves the C default. */
#[allow(dead_code)]
pub const DEBUGLEVEL: i32 = 0;

/* Static assert is triggered at compile time, leaving no runtime artefact.
 * Static assert only works with compile-time constants. */
#[macro_export]
macro_rules! DEBUG_STATIC_ASSERT {
    ($c:expr) => {
        const _: [(); 1] = [(); ($c as bool) as usize];
    };
}

/* At DEBUGLEVEL >= 1, the C header imports the dependency-provided assert.
 * Otherwise assert is disabled.  Conditional build-time dependency intent is
 * retained here; the dependency is supplied by the surrounding translation. */

/* At DEBUGLEVEL >= 2, g_debuglevel is declared by debug.c and shared by the
 * whole process.  It is not thread-safe. */
#[cfg(any(feature = "debug_level_2", feature = "debug"))]
extern "C" {
    pub static mut g_debuglevel: i32;
}

/* ZSTD_DEBUG_PRINT is supplied by the dependency layer. */
#[cfg(any(feature = "debug_level_2", feature = "debug"))]
#[macro_export]
macro_rules! RAWLOG {
    ($l:expr, $($arg:tt)*) => {
        unsafe {
            if ($l as i32) <= $crate::g_debuglevel {
                ZSTD_DEBUG_PRINT!($($arg)*);
            }
        }
    };
}

#[cfg(any(feature = "debug_level_2", feature = "debug"))]
#[macro_export]
macro_rules! DEBUGLOG {
    ($l:expr, $($arg:tt)*) => {
        unsafe {
            if ($l as i32) <= $crate::g_debuglevel {
                ZSTD_DEBUG_PRINT!(concat!(file!(), ":", line!(), ": ", $($arg)*));
                ZSTD_DEBUG_PRINT!(" \n");
            }
        }
    };
}

#[cfg(not(any(feature = "debug_level_2", feature = "debug")))]
#[macro_export]
macro_rules! RAWLOG {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(any(feature = "debug_level_2", feature = "debug")))]
#[macro_export]
macro_rules! DEBUGLOG {
    ($($arg:tt)*) => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
