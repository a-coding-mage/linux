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

/*
 * Rust translation of portability_macros.h.  The original header is shared
 * between C and ASM code and contains portability-detection macros only.
 */

/* Compatibility fallbacks for non-clang compilers.  Rust has no direct
 * equivalent of these preprocessor feature-test operators. */
pub const HAS_ATTRIBUTE_FALLBACK: i32 = 0;
pub const HAS_BUILTIN_FALLBACK: i32 = 0;
pub const HAS_FEATURE_FALLBACK: i32 = 0;

/* The original sanitizer-detection sections contain no definitions. */

/* Mark the internal assembly functions as hidden. */
#[cfg(target_os = "linux")]
#[macro_export]
macro_rules! ZSTD_HIDE_ASM_FUNCTION {
    ($func:ident) => { concat!(".hidden ", stringify!($func)) };
}

#[cfg(target_os = "macos")]
#[macro_export]
macro_rules! ZSTD_HIDE_ASM_FUNCTION {
    ($func:ident) => { concat!(".private_extern ", stringify!($func)) };
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
#[macro_export]
macro_rules! ZSTD_HIDE_ASM_FUNCTION {
    ($func:ident) => { "" };
}

/* Compile-time determination of BMI2 support is supplied by the target/build
 * configuration in Rust; this header did not define a standalone value. */

/* Enable runtime BMI2 dispatch based on the CPU.  This preserves the C
 * default (the exact compiler feature probes are not Rust preprocessor cfgs). */
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "bmi2")
))]
pub const DYNAMIC_BMI2: i32 = 1;
#[cfg(any(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    target_feature = "bmi2"
))]
pub const DYNAMIC_BMI2: i32 = 0;

/* Assembly support is enabled. */
pub const ZSTD_ASM_SUPPORTED: i32 = 1;

/* Assembly for x86-64 with BMI2 is disabled by this translation, matching the
 * source header's effective definition. */
pub const ZSTD_ENABLE_ASM_X86_64_BMI2: i32 = 0;

/* CET endbranch marker.  The original definition is empty when cet.h is not
 * available; Rust has no direct include-probe equivalent here. */
#[macro_export]
macro_rules! ZSTD_CET_ENDBRANCH {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
