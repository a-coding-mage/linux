/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/prom.h
 *
 *  Copyright (C) 2009 Canonical Ltd. <jeremy.kerr@canonical.com>
 */

/* The C header's CONFIG_OF conditional is represented by the `config_of`
 * configuration option.  `machine_desc` is supplied by another dependency. */

#[cfg(feature = "config_of")]
extern "C" {
    pub fn setup_machine_fdt(dt_virt: *mut core::ffi::c_void) -> *const machine_desc;
    pub fn arm_dt_init_cpu_maps();
}

#[cfg(not(feature = "config_of"))]
#[inline]
pub unsafe fn setup_machine_fdt(
    _dt_virt: *mut core::ffi::c_void,
) -> *const machine_desc {
    core::ptr::null()
}

#[cfg(not(feature = "config_of"))]
#[inline]
pub unsafe fn arm_dt_init_cpu_maps() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
