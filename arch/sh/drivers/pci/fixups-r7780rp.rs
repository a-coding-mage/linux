// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/fixups-r7780rp.c
 *
 * Highlander R7780RP-1 PCI fixups
 *
 * Copyright (C) 2003  Lineo uSolutions, Inc.
 * Copyright (C) 2004 - 2006  Paul Mundt
 */
// Dependencies supplied by the Linux PCI, I/O, SH interrupt-controller, and
// SH4 PCI headers are referenced here but not defined in this translation unit.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn evt2irq(event: i32) -> i32;
}

pub unsafe fn pcibios_map_platform_irq(
    pdev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    let _ = pdev;
    let _ = pin;
    evt2irq(0xa20) + slot as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
