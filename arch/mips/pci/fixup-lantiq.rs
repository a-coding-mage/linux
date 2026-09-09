// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

// Dependency declarations corresponding to <linux/of_pci.h> and <linux/pci.h>.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn of_irq_parse_and_map_pci(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_map_irq(
    dev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    unsafe { of_irq_parse_and_map_pci(dev, slot, pin) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
