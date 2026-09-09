/*
 * Copyright 2001 MontaVista Software Inc.
 * Author: MontaVista Software, Inc.
 *         stevel@mvista.com or source@mvista.com
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * THIS SOFTWARE IS PROVIDED "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut IRQ_MAP: [[i32; 12]; 2] = [
    [0, 0, 2, 3, 2, 3, 0, 0, 0, 0, 0, 1],
    [0, 0, 1, 3, 0, 2, 1, 3, 0, 2, 1, 3],
];

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, _slot: u8, _pin: u8) -> i32 {
    let mut irq: i32 = 0;

    if (*(*dev).bus).number < 2 && PCI_SLOT((*dev).devfn) < 12 {
        irq = IRQ_MAP[(*(*dev).bus).number as usize][PCI_SLOT((*dev).devfn) as usize];
    }

    irq + GROUP4_IRQ_BASE + 4
}

unsafe fn rc32434_pci_early_fixup(dev: *mut pci_dev) {
    if PCI_SLOT((*dev).devfn) == 6 && (*(*dev).bus).number == 0 {
        /* disable prefetched memory range */
        pci_write_config_word(dev, PCI_PREF_MEMORY_LIMIT, 0);
        pci_write_config_word(dev, PCI_PREF_MEMORY_BASE, 0x10);

        pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, 4);
    }
}

/*
 * The fixup applies to both the IDT and VIA devices present on the board.
 * C declaration translated as a registration directive for the surrounding
 * PCI fixup framework.
 */
// DECLARE_PCI_FIXUP_HEADER(PCI_ANY_ID, PCI_ANY_ID, rc32434_pci_early_fixup);

/* Do platform specific device initialization at pci_enable_device() time */
pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
