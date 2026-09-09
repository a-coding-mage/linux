/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Functions for setting up and using a MPC106 northbridge
 *
 * The original declaration is available only when __KERNEL__ is defined.
 */

/// Opaque declaration supplied by the PCI bridge dependency.
#[repr(C)]
pub struct pci_controller {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn setup_grackle(hose: *mut pci_controller);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
