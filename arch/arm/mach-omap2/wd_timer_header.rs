/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP2+ MPU WD_TIMER-specific function prototypes
 */

// Translated from the C header guard and include. The definition of
// `omap_hwmod` is supplied by the corresponding dependency.

#[repr(C)]
pub struct omap_hwmod {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn omap2_wd_timer_disable(oh: *mut omap_hwmod) -> i32;
    pub fn omap2_wd_timer_reset(oh: *mut omap_hwmod) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
