/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005-2009 Cavium Networks
 */

// Dependency supplied by the surrounding kernel translation: `pci_dev` and
// the PCI integer types.

/*
 * The physical memory base mapped by BAR1.  256MB at the end of the
 * first 4GB.
 */
pub const CVMX_PCIE_BAR1_PHYS_BASE: u64 = (1u64 << 32) - (1u64 << 28);
pub const CVMX_PCIE_BAR1_PHYS_SIZE: u64 = 1u64 << 28;

/*
 * The RC base of BAR1.  gen1 has a 39-bit BAR2, gen2 has 41-bit BAR2,
 * place BAR1 so it is the same for both.
 */
pub const CVMX_PCIE_BAR1_RC_BASE: u64 = 1u64 << 41;

/*
 * pcibios_map_irq() is defined inside pci-octeon.c. All it does is
 * call the Octeon specific version pointed to by this variable. This
 * function needs to change for PCI or PCIe based hosts.
 */
extern "C" {
    pub static mut octeon_pcibios_map_irq:
        Option<unsafe extern "C" fn(dev: *const pci_dev, slot: u8, pin: u8) -> i32>;
}

/*
 * For PCI (not PCIe) the BAR2 base address.
 */
pub const OCTEON_BAR2_PCI_ADDRESS: u64 = 0x8000000000u64;

/*
 * For PCI (not PCIe) the base of the memory mapped by BAR1
 */
extern "C" {
    pub static mut octeon_bar1_pci_phys: u64;
}

/*
 * The following defines are used when octeon_dma_bar_type =
 * OCTEON_DMA_BAR_TYPE_BIG
 */
pub const OCTEON_PCI_BAR1_HOLE_BITS: u32 = 5;
pub const OCTEON_PCI_BAR1_HOLE_SIZE: u64 =
    1u64 << (OCTEON_PCI_BAR1_HOLE_BITS + 3);

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum octeon_dma_bar_type {
    OCTEON_DMA_BAR_TYPE_INVALID,
    OCTEON_DMA_BAR_TYPE_SMALL,
    OCTEON_DMA_BAR_TYPE_BIG,
    OCTEON_DMA_BAR_TYPE_PCIE,
    OCTEON_DMA_BAR_TYPE_PCIE2,
}

/*
 * This tells the DMA mapping system in dma-octeon.c how to map PCI
 * DMA addresses.
 */
extern "C" {
    pub static mut octeon_dma_bar_type: octeon_dma_bar_type;

    pub fn octeon_pci_dma_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
