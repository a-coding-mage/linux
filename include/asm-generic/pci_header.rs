/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The C header guards and include directives have no executable Rust
 * equivalent.
 */

/* #ifndef PCIBIOS_MIN_IO */
pub const PCIBIOS_MIN_IO: i32 = 0;

/* #ifndef PCIBIOS_MIN_MEM */
pub const PCIBIOS_MIN_MEM: i32 = 0;

/*
 * For bootloaders that do not initialize the PCI bus.
 * C macro: pcibios_assign_all_busses()
 */
#[inline(always)]
pub const fn pcibios_assign_all_busses() -> i32 {
    1
}

/* Enable generic resource mapping code in drivers/pci/. */
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: bool = true;

/* CONFIG_PCI_DOMAINS condition preserved from the C header. */
#[cfg(feature = "CONFIG_PCI_DOMAINS")]
pub struct pci_bus;

#[cfg(feature = "CONFIG_PCI_DOMAINS")]
#[inline(always)]
pub fn pci_proc_domain(_bus: *mut pci_bus) -> i32 {
    /* always show the domain in /proc */
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
