/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/platform_data/dsa.h>.

#[repr(C)]
pub struct dsa_mv88e6xxx_pdata {
    /* Must be first, such that dsa_register_switch() can access this
     * without gory pointer manipulations
     */
    pub cd: dsa_chip_data,
    pub compatible: *const core::ffi::c_char,
    pub enabled_ports: u32,
    pub netdev: *mut net_device,
    pub eeprom_len: u32,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
