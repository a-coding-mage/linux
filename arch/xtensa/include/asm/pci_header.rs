/*
 * linux/include/asm-xtensa/pci.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/* Can be used to override the logic in pci_scan_bus for skipping
 * already-configured bus numbers - to be used for buggy BIOSes
 * or architectures with incomplete PCI setup by the loader
 */

pub const PCIBIOS_ASSIGN_ALL_BUSSES: i32 = 0;

/* Assume some values. (We should revise them, if necessary) */

pub const PCIBIOS_MIN_IO: u32 = 0x2000;
pub const PCIBIOS_MIN_MEM: u32 = 0x10000000;

/* Dynamic DMA mapping stuff.
 * Xtensa has everything mapped statically like x86.
 */

/* Dependencies supplied by the surrounding kernel translation:
 * linux/types.h, linux/slab.h, linux/scatterlist.h, linux/string.h,
 * and asm/io.h.
 */

/* The PCI address space does equal the physical memory address space.
 * The networking and block device layers use this boolean for bounce buffer
 * decisions.
 */

/* Tell PCI code what kind of PCI resource mappings we support */
pub const HAVE_PCI_MMAP: i32 = 1;
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: i32 = 1;

#[inline]
pub const fn arch_can_pci_mmap_io() -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
