/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Procfs support for lockd
 *
 * Copyright (c) 2014 Jeff Layton <jlayton@primarydata.com>
 */

// Equivalent of: #if IS_ENABLED(CONFIG_PROC_FS)
#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn lockd_create_procfs() -> ::std::os::raw::c_int;
    pub fn lockd_remove_procfs();
}

// Equivalent of the !IS_ENABLED(CONFIG_PROC_FS) branch.
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub unsafe fn lockd_create_procfs() -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub unsafe fn lockd_remove_procfs() {
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
