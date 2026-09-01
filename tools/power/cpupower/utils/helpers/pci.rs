// SPDX-License-Identifier: GPL-2.0
// Original C conditional: defined(__i386__) || defined(__x86_64__)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct pci_access {
    pub devices: *mut pci_dev,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct pci_dev {
    pub next: *mut pci_dev,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct pci_filter {
    pub domain: ::std::os::raw::c_int,
    pub bus: ::std::os::raw::c_int,
    pub slot: ::std::os::raw::c_int,
    pub func: ::std::os::raw::c_int,
    pub vendor: ::std::os::raw::c_int,
    pub device: ::std::os::raw::c_int,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" {
    pub fn pci_alloc() -> *mut pci_access;
    pub fn pci_filter_init(pacc: *mut pci_access, filter: *mut pci_filter);
    pub fn pci_init(pacc: *mut pci_access);
    pub fn pci_scan_bus(pacc: *mut pci_access);
    pub fn pci_filter_match(filter: *mut pci_filter, dev: *mut pci_dev) -> ::std::os::raw::c_int;
    pub fn pci_cleanup(pacc: *mut pci_access);
}

/*
 * pci_acc_init
 *
 * PCI access helper function depending on libpci
 *
 * **pacc : if a valid pci_dev is returned
 *         *pacc must be passed to pci_acc_cleanup to free it
 *
 * domain: domain
 * bus:    bus
 * slot:   slot
 * func:   func
 * vendor: vendor
 * device: device
 * Pass -1 for one of the six above to match any
 *
 * Returns :
 * struct pci_dev which can be used with pci_{read,write}_* functions
 *                to access the PCI config space of matching pci devices
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn pci_acc_init(
    pacc: *mut *mut pci_access,
    domain: ::std::os::raw::c_int,
    bus: ::std::os::raw::c_int,
    slot: ::std::os::raw::c_int,
    func: ::std::os::raw::c_int,
    vendor: ::std::os::raw::c_int,
    dev: ::std::os::raw::c_int,
) -> *mut pci_dev {
    let mut filter_nb_link: pci_filter = ::std::mem::zeroed();
    let mut device: *mut pci_dev;

    *pacc = pci_alloc();
    if (*pacc).is_null() {
        return ::std::ptr::null_mut();
    }

    pci_filter_init(*pacc, &mut filter_nb_link);
    filter_nb_link.domain = domain;
    filter_nb_link.bus = bus;
    filter_nb_link.slot = slot;
    filter_nb_link.func = func;
    filter_nb_link.vendor = vendor;
    filter_nb_link.device = dev;

    pci_init(*pacc);
    pci_scan_bus(*pacc);

    device = (**pacc).devices;
    while !device.is_null() {
        if pci_filter_match(&mut filter_nb_link, device) != 0 {
            return device;
        }
        device = (*device).next;
    }
    pci_cleanup(*pacc);
    ::std::ptr::null_mut()
}

/* Typically one wants to get a specific slot(device)/func of the root domain
   and bus */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn pci_slot_func_init(
    pacc: *mut *mut pci_access,
    slot: ::std::os::raw::c_int,
    func: ::std::os::raw::c_int,
) -> *mut pci_dev {
    pci_acc_init(pacc, 0, 0, slot, func, -1, -1)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
