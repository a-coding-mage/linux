/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 1995-2003 Russell King
 */

unsafe extern "C" {
    pub fn check_writebuffer_bugs();
}

#[cfg(CONFIG_MMU)]
unsafe extern "C" {
    pub fn check_other_bugs();
}

#[cfg(not(CONFIG_MMU))]
#[inline(always)]
pub unsafe fn check_other_bugs() {
    // C equivalent: do { } while (0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
