// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2018 John Crispin <john@phrozen.org>
 */

// Dependency: <linux/pci.h>
// Dependency: <linux/of_pci.h>
// #include <linux/of_irq.h> was disabled in the source.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

extern "C" {
    fn of_irq_parse_and_map_pci(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
}

pub unsafe fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
    let _ = dev;
    0
}

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    of_irq_parse_and_map_pci(dev, slot, pin)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
