/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Platform data for the Intel TCO Watchdog
 */

/* Watchdog resources */
pub const ICH_RES_IO_TCO: i32 = 0;
pub const ICH_RES_IO_SMI: i32 = 1;
pub const ICH_RES_MEM_OFF: i32 = 2;
pub const ICH_RES_MEM_GCS_PMC: i32 = 0;

/**
 * struct itco_wdt_platform_data - iTCO_wdt platform data
 * @name: Name of the platform
 * @version: iTCO version
 * @no_reboot_use_pmc: Use PMC BXT API to set and clear NO_REBOOT bit
 */
#[repr(C)]
pub struct itco_wdt_platform_data {
    pub name: [::core::ffi::c_char; 32],
    pub version: u32,
    pub no_reboot_use_pmc: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
