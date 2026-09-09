/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */

// Translated from spl_debug.h. The original CONFIG_* symbols are represented
// by Cargo cfg flags; WARN_ON and kgdb_breakpoint are supplied externally.

#[cfg(feature = "CONFIG_KGDB")]
macro_rules! SPL_ASSERT_CRITICAL {
    ($expr:expr) => {{
        if WARN_ON(!($expr)) {
            kgdb_breakpoint();
        }
    }};
}

#[cfg(not(feature = "CONFIG_KGDB"))]
macro_rules! SPL_ASSERT_CRITICAL {
    ($expr:expr) => {{
        if WARN_ON(!($expr)) {
            // Intentionally empty: CONFIG_KGDB is disabled.
        }
    }};
}

#[cfg(feature = "CONFIG_DEBUG_KERNEL_DC")]
macro_rules! SPL_ASSERT {
    ($expr:expr) => {
        SPL_ASSERT_CRITICAL!($expr)
    };
}

#[cfg(not(feature = "CONFIG_DEBUG_KERNEL_DC"))]
macro_rules! SPL_ASSERT {
    ($expr:expr) => {
        WARN_ON(!($expr))
    };
}

macro_rules! SPL_BREAK_TO_DEBUGGER {
    () => {
        SPL_ASSERT!(0)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
