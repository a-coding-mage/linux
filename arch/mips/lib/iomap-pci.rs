// SPDX-License-Identifier: GPL-2.0
/*
 * Implement the default iomap interfaces
 *
 * (C) Copyright 2004 Linus Torvalds
 * (C) Copyright 2006 Ralf Baechle <ralf@linux-mips.org>
 * (C) Copyright 2007 MIPS Technologies, Inc.
 *     written by Ralf Baechle <ralf@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_PCI_DRIVERS_LEGACY)]
pub unsafe fn __pci_ioport_map(
    dev: *mut pci_dev,
    port: ::core::ffi::c_ulong,
    _nr: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    let ctrl: *mut pci_controller = (*(*dev).bus).sysdata;
    let mut base: ::core::ffi::c_ulong = (*ctrl).io_map_base;

    /* This will eventually become a BUG_ON but for now be gentle */
    if unlikely((*ctrl).io_map_base == 0) {
        let mut bus: *mut pci_bus = (*dev).bus;
        let mut name = [0i8; 8];

        while !(*bus).parent.is_null() {
            bus = (*bus).parent;
        }

        (*ctrl).io_map_base = mips_io_port_base;
        base = mips_io_port_base;

        sprintf(name.as_mut_ptr(), c"%04x:%02x", pci_domain_nr(bus), (*bus).number);
        printk(
            KERN_WARNING,
            c"io_map_base of root PCI bus %s unset.  Trying to continue but you better\nfix this issue or report it to linux-mips@vger.kernel.org or your vendor.\n",
            name.as_ptr(),
        );
        #[cfg(CONFIG_PCI_DOMAINS)]
        panic(c"To avoid data corruption io_map_base MUST be set with multiple PCI domains.");
    }

    (base.wrapping_add(port)) as *mut ::core::ffi::c_void
}

#[cfg(CONFIG_PCI_DRIVERS_LEGACY)]
pub unsafe fn pci_iounmap(dev: *mut pci_dev, addr: *mut ::core::ffi::c_void) {
    let ctrl: *mut pci_controller = (*(*dev).bus).sysdata;
    let base = (*ctrl).io_map_base as *mut ::core::ffi::c_void;

    if (addr as usize) < (base as usize)
        || (addr as usize)
            > (base as usize).wrapping_add(resource_size((*ctrl).io_resource) as usize)
    {
        iounmap(addr);
    }
}

#[cfg(CONFIG_PCI_DRIVERS_LEGACY)]
#[no_mangle]
pub static EXPORT_SYMBOL_pci_iounmap: unsafe extern "C" fn(*mut pci_dev, *mut ::core::ffi::c_void) =
    pci_iounmap;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
