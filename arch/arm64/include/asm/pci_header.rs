/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/slab.h, linux/dma-mapping.h, asm/io.h,
// and asm-generic/pci.h.

pub const PCIBIOS_MIN_IO: u32 = 0x1000;

/*
 * Set to 1 if the kernel should re-assign all PCI bus numbers
 */
pub fn pcibios_assign_all_busses() -> bool {
    pci_has_flag(PCI_REASSIGN_ALL_BUS)
}

pub const fn arch_can_pci_mmap_wc() -> i32 {
    1
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
