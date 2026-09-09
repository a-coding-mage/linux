/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header guard FSL_EDAC_H.

// Opaque external type supplied by the surrounding dependency set.
pub enum device_node {}

#[repr(C)]
pub struct mpc85xx_edac_pci_plat_data {
    pub of_node: *mut device_node,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
