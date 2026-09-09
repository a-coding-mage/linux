/* SPDX-License-Identifier: GPL-2.0 */
/*
 * asm/leon_pci.h
 *
 * Copyright (C) 2011 Aeroflex Gaisler AB, Daniel Hellstrom
 */

/* PCI related definitions */

/* Forward declarations supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct pci_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct leon_pci_info {
    pub ops: *mut pci_ops,
    pub io_space: resource,
    pub mem_space: resource,
    pub busn: resource,
    pub map_irq:
        Option<unsafe extern "C" fn(dev: *const pci_dev, slot: u8, pin: u8) -> core::ffi::c_int>,
}

extern "C" {
    pub fn leon_pci_init(ofdev: *mut platform_device, info: *mut leon_pci_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
