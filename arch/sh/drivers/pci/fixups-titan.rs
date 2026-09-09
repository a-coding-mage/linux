// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/ops-titan.c
 *
 * Ported to new API by Paul Mundt <lethal@linux-sh.org>
 *
 * Modified from ops-snapgear.c written by  David McCullough
 * Highly leveraged from pci-bigsur.c, written by Dustin McIntire.
 *
 * PCI initialization for the Titan boards
 */

// Dependencies supplied by the Linux kernel and the machine-specific headers:
// linux/kernel.h, linux/types.h, linux/init.h, linux/pci.h, linux/io.h,
// mach/titan.h, and pci-sh4.h.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

// TITAN_IRQ_* are supplied by mach/titan.h.
static mut TITAN_IRQ_TAB: [i32; 5] = [
    TITAN_IRQ_WAN,
    TITAN_IRQ_LAN,
    TITAN_IRQ_MPCIA,
    TITAN_IRQ_MPCIB,
    TITAN_IRQ_USB,
];

extern "C" {
    fn printk(fmt: *const u8, ...) -> i32;
}

pub unsafe fn pcibios_map_platform_irq(
    _pdev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    let irq = TITAN_IRQ_TAB[slot as usize];

    let fmt = b"PCI: Mapping TITAN IRQ for slot %d, pin %c to irq %d\n\0";
    printk(
        fmt.as_ptr(),
        slot as i32,
        (pin as i32 - 1 + 'A' as i32),
        irq,
    );

    irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
