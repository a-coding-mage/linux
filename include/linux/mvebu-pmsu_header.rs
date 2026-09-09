/*
 * Copyright (C) 2012 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// CONFIG_MACH_MVEBU_V7 selects the external implementation.
#[cfg(feature = "CONFIG_MACH_MVEBU_V7")]
extern "C" {
    pub fn mvebu_pmsu_dfs_request(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// When CONFIG_MACH_MVEBU_V7 is not enabled, the C header provides this
// static inline fallback. `ENODEV` is supplied by the surrounding system.
#[cfg(not(feature = "CONFIG_MACH_MVEBU_V7"))]
#[inline]
pub fn mvebu_pmsu_dfs_request(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let _ = cpu;
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
