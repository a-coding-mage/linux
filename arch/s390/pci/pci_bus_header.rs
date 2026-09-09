/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2020
 *
 * Author(s):
 *   Pierre Morel <pmorel@linux.ibm.com>
 *
 */

// Dependency: declarations supplied by Linux PCI and the surrounding s390 PCI code.

extern "C" {
    pub fn zpci_bus_device_register(zdev: *mut zpci_dev, ops: *mut pci_ops) -> ::core::ffi::c_int;
    pub fn zpci_bus_device_unregister(zdev: *mut zpci_dev);

    pub fn zpci_bus_scan_bus(zbus: *mut zpci_bus) -> ::core::ffi::c_int;
    pub fn zpci_bus_get_next(pos: *mut *mut zpci_bus);

    pub fn zpci_bus_scan_device(zdev: *mut zpci_dev) -> ::core::ffi::c_int;
    pub fn zpci_bus_remove_device(zdev: *mut zpci_dev, set_error: bool);

    pub fn zpci_release_device(kref: *mut kref);

    pub fn zpci_zdev_put(zdev: *mut zpci_dev);

    pub fn zpci_alloc_domain(domain: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn zpci_free_domain(domain: ::core::ffi::c_int);
    pub fn zpci_setup_bus_resources(zdev: *mut zpci_dev) -> ::core::ffi::c_int;
}

/**
 * zpci_bus_for_each - iterate over all the registered zbus objects
 * @pos: a struct zpci_bus * as cursor
 *
 * Acquires and releases references as the cursor iterates over the registered
 * objects. Is tolerant against concurrent removals of objects.
 *
 * Context: Process context. May sleep.
 */
#[macro_export]
macro_rules! zpci_bus_for_each {
    ($pos:ident) => {
        for $pos in {
            let mut __zpci_bus_for_each_pos: *mut zpci_bus = core::ptr::null_mut();
            unsafe { zpci_bus_get_next(&mut __zpci_bus_for_each_pos) };
            core::iter::from_fn(move || {
                let current = __zpci_bus_for_each_pos;
                if current.is_null() {
                    None
                } else {
                    unsafe { zpci_bus_get_next(&mut __zpci_bus_for_each_pos) };
                    Some(current)
                }
            })
        }
    };
}

#[inline]
pub unsafe fn zpci_zdev_get(zdev: *mut zpci_dev) {
    kref_get(&mut (*zdev).kref);
}

#[inline]
pub unsafe fn zdev_from_bus(bus: *mut pci_bus, devfn: ::core::ffi::c_uint) -> *mut zpci_dev {
    let zbus: *mut zpci_bus = (*bus).sysdata as *mut zpci_bus;

    if devfn >= ZPCI_FUNCTIONS_PER_BUS {
        core::ptr::null_mut()
    } else {
        (*zbus).function[devfn as usize]
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
