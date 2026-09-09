/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * Based on powerpc version
 */

/* C header guard __ASM_MICROBLAZE_PCI_H removed in Rust translation. */
/* The declarations below are kernel-only in the original header. */
/* Original dependencies: linux/types.h, linux/slab.h, linux/string.h,
 * linux/dma-mapping.h, linux/pci.h, linux/scatterlist.h, asm/io.h,
 * and asm/pci-bridge.h.
 */

pub const PCIBIOS_MIN_IO: u32 = 0x1000;
pub const PCIBIOS_MIN_MEM: u32 = 0x1000_0000;

/*
 * Set this to 1 if you want the kernel to re-assign all PCI
 * bus numbers (don't do that on ppc64 yet !)
 */
#[inline]
pub const fn pcibios_assign_all_busses() -> i32 {
    0
}

/* External type supplied by the PCI subsystem. */
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub fn pci_domain_nr(bus: *mut pci_bus) -> i32;
}

/* Decide whether to display the domain number in /proc */
pub unsafe extern "C" {
    pub fn pci_proc_domain(bus: *mut pci_bus) -> i32;
}

/* Tell PCI code what kind of PCI resource mappings we support */
pub const HAVE_PCI_MMAP: i32 = 1;
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: i32 = 1;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn xilinx_pci_init() {
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
