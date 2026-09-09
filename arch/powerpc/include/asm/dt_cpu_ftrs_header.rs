/* SPDX-License-Identifier: GPL-2.0 */

/*
 *  Copyright 2017, IBM Corporation
 *  cpufeatures is the new way to discover CPU features with /cpus/features
 *  devicetree. This supersedes PVR based discovery ("cputable"), and older
 *  device tree feature advertisement.
 */

// Dependency equivalent of <linux/types.h> and the device-tree/cputable UAPI.
use core::ffi::c_void;

/*
 * CONFIG_PPC_DT_CPU_FTRS is a build-time configuration condition.  The
 * declarations below are retained under the corresponding Rust cfg.
 */
#[cfg(CONFIG_PPC_DT_CPU_FTRS)]
unsafe extern "C" {
    pub fn dt_cpu_ftrs_init(fdt: *mut c_void) -> bool;
    pub fn dt_cpu_ftrs_scan();
    pub fn dt_cpu_ftrs_in_use() -> bool;
}

#[cfg(not(CONFIG_PPC_DT_CPU_FTRS))]
#[inline]
pub fn dt_cpu_ftrs_init(_fdt: *mut c_void) -> bool {
    false
}

#[cfg(not(CONFIG_PPC_DT_CPU_FTRS))]
#[inline]
pub fn dt_cpu_ftrs_scan() {}

#[cfg(not(CONFIG_PPC_DT_CPU_FTRS))]
#[inline]
pub fn dt_cpu_ftrs_in_use() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
