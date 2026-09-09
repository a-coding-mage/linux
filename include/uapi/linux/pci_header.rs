/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *	pci.h
 *
 *	PCI defines and function prototypes
 *	Copyright 1994, Drew Eckhardt
 *	Copyright 1997--1999 Martin Mares <mj@ucw.cz>
 *
 *	For more information, please consult the following manuals (look at
 *	http://www.pcisig.com/ for how to get them):
 *
 *	PCI BIOS Specification
 *	PCI Local Bus Specification
 *	PCI to PCI Bridge Specification
 *	PCI System Design Guide
 */

// Dependency intent: the original header includes <linux/pci_regs.h>.

/*
 * The PCI interface treats multi-function devices as independent
 * devices.  The slot/function address of each device is encoded
 * in a single byte as follows:
 *
 *	7:3 = slot
 *	2:0 = function
 */
#[inline]
pub const fn pci_devfn(slot: u32, func: u32) -> u32 {
    ((slot & 0x1f) << 3) | (func & 0x07)
}

#[inline]
pub const fn pci_slot(devfn: u32) -> u32 {
    (devfn >> 3) & 0x1f
}

#[inline]
pub const fn pci_func(devfn: u32) -> u32 {
    devfn & 0x07
}

/* Ioctls for /proc/bus/pci/X/Y nodes. */
pub const PCIIOC_BASE: u32 = ('P' as u32) << 24 | ('C' as u32) << 16 | ('I' as u32) << 8;
pub const PCIIOC_CONTROLLER: u32 = PCIIOC_BASE | 0x00; // Get controller for PCI device.
pub const PCIIOC_MMAP_IS_IO: u32 = PCIIOC_BASE | 0x01; // Set mmap state to I/O space.
pub const PCIIOC_MMAP_IS_MEM: u32 = PCIIOC_BASE | 0x02; // Set mmap state to MEM space.
pub const PCIIOC_WRITE_COMBINE: u32 = PCIIOC_BASE | 0x03; // Enable/disable write-combining.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pci_hotplug_event {
    PCI_HOTPLUG_LINK_UP,
    PCI_HOTPLUG_LINK_DOWN,
    PCI_HOTPLUG_CARD_PRESENT,
    PCI_HOTPLUG_CARD_NOT_PRESENT,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
