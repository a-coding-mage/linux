/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/pci.h, and bcm63xx_cpu.h.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bcm63xx_get_irq_number(irq: i32) -> i32;
}

pub unsafe extern "C" fn pcibios_map_irq(
    dev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    let _ = (dev, slot, pin);
    bcm63xx_get_irq_number(IRQ_PCI)
}

pub unsafe extern "C" fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
    let _ = dev;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
