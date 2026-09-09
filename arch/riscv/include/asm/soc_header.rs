/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 * Copyright (C) 2020 Google, Inc
 */

// Dependency supplied by the corresponding Linux/Rust bindings:
// `of_device_id`.

/// Declare an entry in the SoC early-initialization table.
///
/// Rust macro expansion cannot concatenate identifiers without an external
/// identifier-concatenation facility, so the expanded static's name is passed
/// explicitly as `$symbol`.
#[macro_export]
macro_rules! SOC_EARLY_INIT_DECLARE {
    ($symbol:ident, $compat:expr, $fn:expr) => {
        #[used]
        #[link_section = "__soc_early_init_table"]
        static $symbol: of_device_id = of_device_id {
            compatible: $compat,
            data: $fn,
        };
    };
}

extern "C" {
    pub fn soc_early_init();

    pub static mut __soc_early_init_table_start: ::core::ffi::c_ulong;
    pub static mut __soc_early_init_table_end: ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
