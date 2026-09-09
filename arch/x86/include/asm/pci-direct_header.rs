/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Direct PCI access. This is used for PCI accesses in early boot before
 * the PCI subsystem works.
 */

unsafe extern "C" {
    pub fn read_pci_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32;
    pub fn read_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8) -> u8;
    pub fn read_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8) -> u16;
    pub fn write_pci_config(bus: u8, slot: u8, func: u8, offset: u8, val: u32);
    pub fn write_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8, val: u8);
    pub fn write_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8, val: u16);

    pub fn early_pci_allowed() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
