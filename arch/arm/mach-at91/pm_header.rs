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

pub const AT91_MEMCTRL_MC: kernel::ffi::c_uint = 0;
pub const AT91_MEMCTRL_SDRAMC: kernel::ffi::c_uint = 1;
pub const AT91_MEMCTRL_DDRSDR: kernel::ffi::c_uint = 2;

pub const AT91_PM_STANDBY: kernel::ffi::c_uint = 0x00;
pub const AT91_PM_ULP0: kernel::ffi::c_uint = 0x01;
pub const AT91_PM_ULP0_FAST: kernel::ffi::c_uint = 0x02;
pub const AT91_PM_ULP1: kernel::ffi::c_uint = 0x03;
pub const AT91_PM_BACKUP: kernel::ffi::c_uint = 0x04;

#[repr(C)]
pub struct at91_pm_data {
    pub pmc: *mut kernel::ffi::c_void,
    pub ramc: [*mut kernel::ffi::c_void; 2],
    pub ramc_phy: *mut kernel::ffi::c_void,
    pub uhp_udp_mask: kernel::ffi::c_ulong,
    pub memctrl: kernel::ffi::c_uint,
    pub mode: kernel::ffi::c_uint,
    pub shdwc: *mut kernel::ffi::c_void,
    pub sfrbu: *mut kernel::ffi::c_void,
    pub standby_mode: kernel::ffi::c_uint,
    pub suspend_mode: kernel::ffi::c_uint,
    pub pmc_mckr_offset: kernel::ffi::c_uint,
    pub pmc_version: kernel::ffi::c_uint,
    pub pmc_mcks: kernel::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
