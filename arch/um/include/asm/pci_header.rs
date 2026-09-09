/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: __ASM_UM_PCI_H
// Dependencies: <linux/types.h>, <asm/io.h>, and <asm-generic/pci.h>

/* Generic PCI */

// Preserved from the C source: this declaration is enabled by CONFIG_PCI_MSI.
//
// This is a bit of an annoying hack, and it assumes we only have
// the virt-pci (if anything). Which is true, but still.
extern "C" {
    pub fn pci_root_bus_fwnode(bus: *mut pci_bus) -> *mut core::ffi::c_void;
}

// C macro preserved in intent: #define pci_root_bus_fwnode pci_root_bus_fwnode

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
