/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2013, Michael Ellerman, IBM Corporation.
 */

/* These are ordered and tested with <= */
pub const SYNC_STEP_INITIAL: i32 = 0;
pub const SYNC_STEP_UNSPLIT: i32 = 1; /* Set by secondary when it sees unsplit */
pub const SYNC_STEP_REAL_MODE: i32 = 2; /* Set by secondary when in real mode  */
pub const SYNC_STEP_FINISHED: i32 = 3; /* Set by secondary when split/unsplit is done */

/* CONFIG_SMP controls whether the SMP declarations and implementation are available. */
#[cfg(CONFIG_SMP)]
unsafe extern "C" {
    pub fn split_core_secondary_loop(state: *mut u8);
    pub fn update_subcore_sibling_mask();
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn update_subcore_sibling_mask() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
