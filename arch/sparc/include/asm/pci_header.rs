/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Can be used to override the logic in pci_scan_bus for skipping
 * already-configured bus numbers - to be used for buggy BIOSes
 * or architectures with incomplete PCI setup by the loader.
 */
#[inline]
pub const fn pcibios_assign_all_busses() -> i32 {
    0
}

pub const PCIBIOS_MIN_IO: usize = 0;
pub const PCIBIOS_MIN_MEM: usize = 0;

pub const PCI_IRQ_NONE: u32 = 0xffff_ffff;

/* The following declarations are conditional on CONFIG_SPARC64 in C. */
#[cfg(CONFIG_SPARC64)]
pub const PCI64_REQUIRED_MASK: u64 = !0u64;

#[cfg(CONFIG_SPARC64)]
pub const PCI64_ADDR_BASE: u64 = 0xfffc_0000_0000_0000u64;

#[cfg(CONFIG_SPARC64)]
pub struct pci_bus;

/* Return the index of the PCI controller for device PDEV. */
#[cfg(CONFIG_SPARC64)]
extern "C" {
    pub fn pci_domain_nr(bus: *mut pci_bus) -> i32;
}

#[cfg(CONFIG_SPARC64)]
#[inline]
pub unsafe fn pci_proc_domain(_bus: *mut pci_bus) -> i32 {
    1
}

/* Platform support for /proc/bus/pci/X/Y mmap()s. */
#[cfg(CONFIG_SPARC64)]
pub const HAVE_PCI_MMAP: bool = true;

#[cfg(CONFIG_SPARC64)]
#[inline]
pub const fn arch_can_pci_mmap_io() -> i32 {
    1
}

#[cfg(CONFIG_SPARC64)]
pub const HAVE_ARCH_PCI_GET_UNMAPPED_AREA: bool = true;

#[cfg(CONFIG_SPARC64)]
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: bool = true;

/* C macro: get_pci_unmapped_area is an alias for get_fb_unmapped_area. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
