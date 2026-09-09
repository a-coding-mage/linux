/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// C dependency: struct pci_bus is supplied by the surrounding translation.
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

extern "C" {
    pub static mut ltq_pci_mapped_cfg: *mut core::ffi::c_void;

    pub fn ltq_pci_read_config_dword(
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: *mut u32,
    ) -> i32;

    pub fn ltq_pci_write_config_dword(
        bus: *mut pci_bus,
        devfn: u32,
        where_: i32,
        size: i32,
        val: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
