/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AT91 Power Management
 *
 * Copyright (C) 2005 David Brownell
 */

// Dependency supplied by asm/proc-fns.h.
// Dependency supplied by linux/mfd/syscon/atmel-mc.h.
// Dependency supplied by soc/at91/at91sam9_ddrsdr.h.
// Dependency supplied by soc/at91/at91sam9_sdramc.h.
// Dependency supplied by soc/at91/sama7-ddr.h.
// Dependency supplied by soc/at91/sama7-sfrbu.h.

pub const AT91_MEMCTRL_MC: core::ffi::c_uint = 0;
pub const AT91_MEMCTRL_SDRAMC: core::ffi::c_uint = 1;
pub const AT91_MEMCTRL_DDRSDR: core::ffi::c_uint = 2;

pub const AT91_PM_STANDBY: core::ffi::c_uint = 0x00;
pub const AT91_PM_ULP0: core::ffi::c_uint = 0x01;
pub const AT91_PM_ULP0_FAST: core::ffi::c_uint = 0x02;
pub const AT91_PM_ULP1: core::ffi::c_uint = 0x03;
pub const AT91_PM_BACKUP: core::ffi::c_uint = 0x04;

#[repr(C)]
pub struct at91_pm_data {
    pub pmc: *mut core::ffi::c_void,
    pub ramc: [*mut core::ffi::c_void; 2],
    pub ramc_phy: *mut core::ffi::c_void,
    pub uhp_udp_mask: core::ffi::c_ulong,
    pub memctrl: core::ffi::c_uint,
    pub mode: core::ffi::c_uint,
    pub shdwc: *mut core::ffi::c_void,
    pub sfrbu: *mut core::ffi::c_void,
    pub standby_mode: core::ffi::c_uint,
    pub suspend_mode: core::ffi::c_uint,
    pub pmc_mckr_offset: core::ffi::c_uint,
    pub pmc_version: core::ffi::c_uint,
    pub pmc_mcks: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
