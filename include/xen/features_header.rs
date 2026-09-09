/* SPDX-License-Identifier: GPL-2.0 */
/******************************************************************************
 * features.h
 *
 * Query the features reported by Xen.
 *
 * Copyright (c) 2006, Ian Campbell
 */

/* Dependency corresponding to: #include <xen/interface/features.h> */

unsafe extern "C" {
    pub fn xen_setup_features();

    pub static mut xen_features: [u8; XENFEAT_NR_SUBMAPS * 32];
}

#[inline]
pub unsafe fn xen_feature(flag: i32) -> i32 {
    xen_features[flag as usize] as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
