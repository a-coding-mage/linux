/*
 * TI DaVinci EMAC platform support
 *
 * Author: Kevin Hilman, Deep Root Systems, LLC
 *
 * 2007 (c) Deep Root Systems, LLC. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// C dependencies:
// - linux/if_ether.h supplies ETH_ALEN (6)
// - linux/nvmem-consumer.h is included by the original header

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct mdio_platform_data {
    pub bus_freq: c_ulong,
}

#[repr(C)]
pub struct emac_platform_data {
    pub mac_addr: [c_char; 6],
    pub ctrl_reg_offset: u32,
    pub ctrl_mod_reg_offset: u32,
    pub ctrl_ram_offset: u32,
    pub hw_ram_addr: u32,
    pub ctrl_ram_size: u32,

    /*
     * phy_id can be one of the following:
     *   - NULL              : use the first phy on the bus,
     *   - ""                : force to 100/full, no mdio control
     *   - "<bus>:<addr>"    : use the specified bus and phy
     */
    pub phy_id: *const c_char,

    pub rmii_en: u8,
    pub version: u8,
    pub no_bd_ram: bool,
    pub interrupt_enable: Option<unsafe extern "C" fn()> ,
    pub interrupt_disable: Option<unsafe extern "C" fn()> ,
}

pub const EMAC_VERSION_1: i32 = 0; /* DM644x */
pub const EMAC_VERSION_2: i32 = 1; /* DM646x */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
