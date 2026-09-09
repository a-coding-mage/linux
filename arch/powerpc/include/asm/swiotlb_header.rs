/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Becky Bruce, Freescale Semiconductor
 */

// Dependency: declarations supplied by <linux/swiotlb.h>.

unsafe extern "C" {
    pub static mut ppc_swiotlb_enable: core::ffi::c_uint;
    pub static mut ppc_swiotlb_flags: core::ffi::c_uint;
}

// CONFIG_SWIOTLB selects the external implementation in the build.
#[cfg(CONFIG_SWIOTLB)]
unsafe extern "C" {
    pub fn swiotlb_detect_4g();
}

#[cfg(not(CONFIG_SWIOTLB))]
#[inline]
pub unsafe fn swiotlb_detect_4g() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
