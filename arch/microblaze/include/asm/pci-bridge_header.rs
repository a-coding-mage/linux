/* SPDX-License-Identifier: GPL-2.0-or-later */

/* C header dependencies: linux/pci.h, linux/list.h, linux/ioport.h. */

use core::ffi::c_void;

pub struct device_node;

#[cfg(CONFIG_PCI)]
extern "C" {
    pub static mut hose_list: list_head;
    pub fn pcibios_vaddr_is_ioport(address: *mut c_void) -> i32;
}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub unsafe fn pcibios_vaddr_is_ioport(_address: *mut c_void) -> i32 {
    0
}

/*
 * Structure of a PCI controller (host bridge)
 */
#[repr(C)]
pub struct pci_controller {
    pub bus: *mut pci_bus,
    pub list_node: list_head,

    pub io_base_virt: *mut c_void,

    /* Currently, we limit ourselves to 1 IO range and 3 mem
     * ranges since the common pci_bus structure can't handle more
     */
    pub io_resource: resource,
}

#[cfg(CONFIG_PCI)]
#[inline]
pub unsafe fn isa_vaddr_is_ioport(_address: *mut c_void) -> i32 {
    /* No specific ISA handling on ppc32 at this stage, it
     * all goes through PCI
     */
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
