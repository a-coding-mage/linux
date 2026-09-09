/*
 * arch/arm/plat-orion/include/plat/pcie.h
 *
 * Marvell Orion SoC PCIe handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

extern "C" {
    pub fn orion_pcie_dev_id(base: *mut c_void) -> u32;
    pub fn orion_pcie_rev(base: *mut c_void) -> u32;
    pub fn orion_pcie_link_up(base: *mut c_void) -> i32;
    pub fn orion_pcie_x4_mode(base: *mut c_void) -> i32;
    pub fn orion_pcie_get_local_bus_nr(base: *mut c_void) -> i32;
    pub fn orion_pcie_set_local_bus_nr(base: *mut c_void, nr: i32);
    pub fn orion_pcie_reset(base: *mut c_void);
    pub fn orion_pcie_setup(base: *mut c_void);
    pub fn orion_pcie_rd_conf(
        base: *mut c_void,
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: *mut u32,
    ) -> i32;
    pub fn orion_pcie_rd_conf_tlp(
        base: *mut c_void,
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: *mut u32,
    ) -> i32;
    pub fn orion_pcie_rd_conf_wa(
        wa_base: *mut c_void,
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: *mut u32,
    ) -> i32;
    pub fn orion_pcie_wr_conf(
        base: *mut c_void,
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
