/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 * pbm.h: PCI bus module pseudo driver software state
 *        Adopted from sparc64 by V. Roganov and G. Raiko
 *
 * Original header:
 * pbm.h: U2P PCI bus module pseudo driver software state.
 *
 * Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 *
 * To put things into perspective, consider sparc64 with a few PCI controllers.
 * Each type would have an own structure, with instances related one to one.
 * We have only pcic on sparc, but we want to be compatible with sparc64 pbm.h.
 * All three represent different abstractions.
 *   pci_bus  - Linux PCI subsystem view of a PCI bus (including bridged buses)
 *   pbm      - Arch-specific view of a PCI bus (sparc or sparc64)
 *   pcic     - Chip-specific information for PCIC.
 */

/* C header dependencies: <linux/pci.h>, <asm/oplib.h>, and <asm/prom.h>. */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct linux_pbm_info {
    pub prom_node: c_int,
    pub prom_name: [c_char; 64],
    /* struct linux_prom_pci_ranges pbm_ranges[PROMREG_MAX]; */
    /* int num_pbm_ranges; */

    /* Now things for the actual PCI bus probes. */
    pub pci_first_busno: core::ffi::c_uint, /* Can it be nonzero? */
    pub pci_bus: *mut pci_bus,              /* Was inline, MJ allocs now */
}

/* PCI devices which are not bridges have this placed in their pci_dev
 * sysdata member.  This makes OBP aware PCI device drivers easier to
 * code.
 */
#[repr(C)]
pub struct pcidev_cookie {
    pub pbm: *mut linux_pbm_info,
    pub prom_node: *mut device_node,
}

/* External types supplied by the translated dependency headers. */
pub type pci_bus = crate::pci_bus;
pub type device_node = crate::device_node;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
