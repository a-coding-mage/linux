/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2017 IBM Corp.
 */

/* CONFIG_PPC_POWERNV selects the declaration versus no-op inline definitions. */
#[cfg(feature = "CONFIG_PPC_POWERNV")]
unsafe extern "C" {
    pub fn powernv_set_nmmu_ptcr(ptcr: ::core::ffi::c_ulong);

    pub fn pnv_program_cpu_hotplug_lpcr(cpu: ::core::ffi::c_uint, lpcr_val: u64);

    pub fn pnv_tm_init();
}

#[cfg(not(feature = "CONFIG_PPC_POWERNV"))]
#[inline]
pub fn powernv_set_nmmu_ptcr(_ptcr: ::core::ffi::c_ulong) {}

#[cfg(not(feature = "CONFIG_PPC_POWERNV"))]
#[inline]
pub fn pnv_tm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
