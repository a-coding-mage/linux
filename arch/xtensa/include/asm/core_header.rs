/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 Cadence Design Systems Inc. */

// Dependency supplied by the variant-specific core configuration.

// These configuration constants default to zero when not supplied by the
// target configuration, matching the C preprocessor definitions.
pub const XCHAL_HAVE_DIV32: i32 = 0;
pub const XCHAL_HAVE_EXCLUSIVE: i32 = 0;
pub const XCHAL_HAVE_EXTERN_REGS: i32 = 0;
pub const XCHAL_HAVE_MPU: i32 = 0;
pub const XCHAL_HAVE_VECBASE: i32 = 0;
pub const XCHAL_SPANNING_WAY: i32 = 0;
pub const XCHAL_HAVE_TRAX: i32 = 0;
pub const XCHAL_NUM_PERF_COUNTERS: i32 = 0;

// The following C preprocessor conditions are build-time configuration:
//
// #if XCHAL_HAVE_WINDOWED
// #if defined(CONFIG_USER_ABI_DEFAULT) || defined(CONFIG_USER_ABI_CALL0_PROBE)
// Whether windowed ABI is supported in userspace.
// #define USER_SUPPORT_WINDOWED
// #endif
// #if defined(__XTENSA_WINDOWED_ABI__) || defined(USER_SUPPORT_WINDOWED)
// Whether windowed ABI is supported either in userspace or in the kernel.
// #define SUPPORT_WINDOWED
// #endif
// #endif

/* Xtensa ABI requires stack alignment to be at least 16 */
// If XCHAL_DATA_WIDTH > 16, use XCHAL_DATA_WIDTH; otherwise use 16.
pub const XTENSA_STACK_ALIGNMENT: i32 = 16;

// XCHAL_HW_MIN_VERSION defaults to the encoded major/minor hardware version
// when both components are supplied by the target configuration, and to zero
// otherwise.
pub const XCHAL_HW_MIN_VERSION: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
