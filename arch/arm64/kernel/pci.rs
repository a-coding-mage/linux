// SPDX-License-Identifier: GPL-2.0-only
/*
 * Code borrowed from powerpc/kernel/pci-common.c
 *
 * Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
 * Copyright (C) 2014 ARM Ltd.
 */

// Dependency corresponding to: #include <linux/pci.h>

/*
 * raw_pci_read/write - Platform-specific PCI config space access.
 */

pub type U32 = u32;

pub const PCIBIOS_DEVICE_NOT_FOUND: i32 = -1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_bus {
    pub ops: *const pci_ops,
    pub dev: device,
}

#[repr(C)]
pub struct pci_ops {
    pub read: unsafe extern "C" fn(
        bus: *mut pci_bus,
        devfn: u32,
        reg: i32,
        len: i32,
        val: *mut U32,
    ) -> i32,
    pub write: unsafe extern "C" fn(
        bus: *mut pci_bus,
        devfn: u32,
        reg: i32,
        len: i32,
        val: U32,
    ) -> i32,
}

unsafe extern "C" {
    pub fn pci_find_bus(domain: u32, bus: u32) -> *mut pci_bus;
    pub fn dev_to_node(dev: *const device) -> i32;
}

pub unsafe extern "C" fn raw_pci_read(
    domain: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    val: *mut U32,
) -> i32 {
    let b: *mut pci_bus = unsafe { pci_find_bus(domain, bus) };

    if b.is_null() {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    unsafe { ((*(*b).ops).read)(b, devfn, reg, len, val) }
}

pub unsafe extern "C" fn raw_pci_write(
    domain: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    val: U32,
) -> i32 {
    let b: *mut pci_bus = unsafe { pci_find_bus(domain, bus) };

    if b.is_null() {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    unsafe { ((*(*b).ops).write)(b, devfn, reg, len, val) }
}

// CONFIG_NUMA conditional retained from the source build configuration.
#[cfg(feature = "CONFIG_NUMA")]
pub unsafe extern "C" fn pcibus_to_node(bus: *mut pci_bus) -> i32 {
    unsafe { dev_to_node(&(*bus).dev as *const device) }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
