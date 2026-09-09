// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/ops-snapgear.c
 *
 * Author:  David McCullough <davidm@snapgear.com>
 *
 * Ported to new API by Paul Mundt <lethal@linux-sh.org>
 *
 * Highly leveraged from pci-bigsur.c, written by Dustin McIntire.
 *
 * PCI initialization for the SnapGear boards
 */

use core::ffi::{c_char, c_int, c_ulong};

// Supplied by the surrounding kernel dependencies.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn evt2irq(event: c_ulong) -> c_int;
    fn printk(fmt: *const c_char, ...);
}

pub unsafe fn pcibios_map_platform_irq(
    _pdev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> c_int {
    let mut irq: c_int = -1;

    match slot {
        8 => {} // the PCI bridge
        11 => {
            irq = evt2irq(0x300);
        } // USB
        12 => {
            irq = evt2irq(0x360);
        } // PCMCIA
        13 => {
            irq = evt2irq(0x2a0);
        } // eth0
        14 => {
            irq = evt2irq(0x300);
        } // eth1
        15 => {
            irq = evt2irq(0x360);
        } // safenet (unused)
        _ => {}
    }

    let pin_char = pin.wrapping_sub(1).wrapping_add(b'A');
    printk(
        b"PCI: Mapping SnapGear IRQ for slot %d, pin %c to irq %d\n\0".as_ptr()
            as *const c_char,
        slot as c_int,
        pin_char as c_int,
        irq,
    );

    irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
