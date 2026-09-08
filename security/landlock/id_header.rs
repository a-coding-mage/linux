// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Unique identification number generator
 *
 * Copyright © 2024-2025 Microsoft Corporation
 */

// C header guard omitted in Rust.

// Original C conditional: #ifdef CONFIG_SECURITY_LANDLOCK_LOG
#[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
extern "C" {
    pub fn landlock_init_id();

    pub fn landlock_get_id_range(number_of_ids: size_t) -> u64;
}

// Original C conditional: #else CONFIG_SECURITY_LANDLOCK_LOG
#[cfg(not(CONFIG_SECURITY_LANDLOCK_LOG))]
#[inline]
pub fn landlock_init_id() {}



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
