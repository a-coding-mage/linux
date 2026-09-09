/*
 * include/asm-xtensa/pci-bridge.h
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 *
 * Copyright (C) 2005 Tensilica Inc.
 */

#[repr(C)]
pub struct pci_controller {
    pub index: ::core::ffi::c_int,
    pub next: *mut pci_controller,
    pub bus: *mut pci_bus,
    pub arch_data: *mut ::core::ffi::c_void,

    pub first_busno: ::core::ffi::c_int,
    pub last_busno: ::core::ffi::c_int,

    pub ops: *mut pci_ops,
    pub cfg_addr: *mut u32,
    pub cfg_data: *mut u8,

    /* Currently, we limit ourselves to 1 IO range and 3 mem
     * ranges since the common pci_bus structure can't handle more
     */
    pub io_resource: resource,
    pub mem_resources: [resource; 3],
    pub mem_resource_count: ::core::ffi::c_int,

    /* Host bridge I/O and Memory space
     * Used for BAR placement algorithms
     */
    pub io_space: pci_space,
    pub mem_space: pci_space,

    /* Return the interrupt number fo a device. */
    pub map_irq: Option<unsafe extern "C" fn(*mut pci_dev, u8, u8) -> ::core::ffi::c_int>,
}

/*
 * pciauto_bus_scan() enumerates the pci space.
 */

unsafe extern "C" {
    pub fn pciauto_bus_scan(
        controller: *mut pci_controller,
        busno: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct pci_space {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub base: ::core::ffi::c_ulong,
}

/*
 * Structure of a PCI controller (host bridge)
 */

#[inline]
pub unsafe fn pcibios_init_resource(
    res: *mut resource,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_int,
    name: *mut ::core::ffi::c_char,
) {
    (*res).start = start;
    (*res).end = end;
    (*res).flags = flags;
    (*res).name = name;
    (*res).parent = core::ptr::null_mut();
    (*res).sibling = core::ptr::null_mut();
    (*res).child = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
