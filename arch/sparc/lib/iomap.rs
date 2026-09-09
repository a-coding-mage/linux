// SPDX-License-Identifier: GPL-2.0
/*
 * Implement the sparc iomap interfaces
 */

use core::ffi::c_void;

/* Create a virtual mapping cookie for an IO port range */
pub unsafe fn ioport_map(port: usize, _nr: u32) -> *mut c_void {
    port as usize as *mut c_void
}

pub unsafe fn ioport_unmap(_addr: *mut c_void) {
    /* Nothing to do */
}

// EXPORT_SYMBOL(ioport_map);
// EXPORT_SYMBOL(ioport_unmap);

#[cfg(CONFIG_PCI)]
pub unsafe fn pci_iounmap(_dev: *mut pci_dev, _addr: *mut c_void) {
    /* nothing to do */
}

// EXPORT_SYMBOL(pci_iounmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
