/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 SiFive
 */

// Translated from the Linux RISC-V PCI header.
// Dependencies supplied by the corresponding Linux/Rust environment are
// intentionally referenced but not defined here.

pub const PCIBIOS_MIN_IO: i32 = 4;
pub const PCIBIOS_MIN_MEM: i32 = 16;

// Equivalent of: defined(CONFIG_PCI) && defined(CONFIG_NUMA)
// This block is available when PCI and NUMA support are enabled.
#[cfg(all(feature = "CONFIG_PCI", feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn pcibus_to_node(bus: *mut pci_bus) -> i32 {
    dev_to_node(&(*bus).dev)
}

// Equivalent of the conditional cpumask_of_pcibus macro.  The referenced
// types and functions are provided by the surrounding kernel bindings.
#[cfg(all(feature = "CONFIG_PCI", feature = "CONFIG_NUMA"))]
#[macro_export]
macro_rules! cpumask_of_pcibus {
    ($bus:expr) => {{
        if unsafe { $crate::pcibus_to_node($bus) } == -1 {
            cpu_all_mask
        } else {
            cpumask_of_node(unsafe { $crate::pcibus_to_node($bus) })
        }
    }};
}

/* Generic PCI: equivalent of <asm-generic/pci.h>. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
