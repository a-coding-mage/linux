/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* CONFIG_DEBUG_FS condition from the original C header. */
#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn cc_debugfs_global_init();
    pub fn cc_debugfs_global_fini();

    pub fn cc_debugfs_init(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;
    pub fn cc_debugfs_fini(drvdata: *mut cc_drvdata);
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn cc_debugfs_global_init() {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn cc_debugfs_global_fini() {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn cc_debugfs_init(_drvdata: *mut cc_drvdata) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn cc_debugfs_fini(_drvdata: *mut cc_drvdata) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
