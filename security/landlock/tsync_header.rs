// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Cross-thread ruleset enforcement
 *
 * Copyright © 2025 Google LLC
 */

// C dependencies:
// #include <linux/cred.h>
// #include <linux/types.h>

use core::ffi::c_int;

extern "C" {
    pub fn landlock_restrict_sibling_threads(
        old_cred: *const cred,
        new_cred: *const cred,
        restrict_flags: u32,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
