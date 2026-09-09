/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _PROBE_ROMS_H_

use core::ffi::c_void;

#[repr(C)]
pub struct pci_dev {
    _unused: [u8; 0],
}

extern "C" {
    pub fn pci_map_biosrom(pdev: *mut pci_dev) -> *mut c_void;
    pub fn pci_unmap_biosrom(rom: *mut c_void);
    pub fn pci_biosrom_size(pdev: *mut pci_dev) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
