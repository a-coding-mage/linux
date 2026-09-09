// SPDX-License-Identifier: GPL-2.0
/*
 * ppc64 "iomap" interface implementation.
 *
 * (C) Copyright 2004 Linus Torvalds
 */

use core::ffi::{c_int, c_void};

// Supplied by the architecture and kernel headers in the surrounding build.
extern "C" {
    fn isa_vaddr_is_ioport(addr: *mut c_void) -> c_int;
    fn pcibios_vaddr_is_ioport(addr: *mut c_void) -> c_int;
    fn iounmap(addr: *mut c_void);
}

// Opaque declaration supplied by the PCI headers.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

pub unsafe extern "C" fn ioport_map(port: usize, _len: u32) -> *mut c_void {
    port.wrapping_add(_IO_BASE) as *mut c_void
}

// EXPORT_SYMBOL(ioport_map);

// #ifdef CONFIG_PCI
pub unsafe extern "C" fn pci_iounmap(_dev: *mut pci_dev, addr: *mut c_void) {
    if isa_vaddr_is_ioport(addr) != 0 {
        return;
    }
    if pcibios_vaddr_is_ioport(addr) != 0 {
        return;
    }
    iounmap(addr);
}

// EXPORT_SYMBOL(pci_iounmap);
// #endif /* CONFIG_PCI */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
