/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/mips/include/asm/prom.h
 *
 *  Copyright (C) 2010 Cisco Systems Inc. <dediao@cisco.com>
 */

// CONFIG_USE_OF: the declarations below are present when device-tree support
// is enabled; otherwise `device_tree_init` is an empty inline function.

#[cfg(feature = "CONFIG_USE_OF")]
extern "C" {
    pub fn device_tree_init();
}

#[cfg(not(feature = "CONFIG_USE_OF"))]
#[inline]
pub fn device_tree_init() {}

#[cfg(feature = "CONFIG_USE_OF")]
pub struct boot_param_header;

#[cfg(feature = "CONFIG_USE_OF")]
extern "C" {
    pub fn __dt_setup_arch(bph: *mut core::ffi::c_void);
    pub fn __dt_register_buses(
        bus0: *const core::ffi::c_char,
        bus1: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

extern "C" {
    pub fn mips_get_machine_name() -> *mut core::ffi::c_char;
    pub fn mips_set_machine_name(name: *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
