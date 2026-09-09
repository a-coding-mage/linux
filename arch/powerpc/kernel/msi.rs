// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2006-2007, Michael Ellerman, IBM Corporation.
 */

use core::ffi::c_int;

// Types and symbols supplied by the surrounding PowerPC/Linux translation.
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub bus: *mut pci_bus,
}

#[repr(C)]
pub struct pci_controller_ops {
    pub setup_msi_irqs:
        Option<unsafe extern "C" fn(*mut pci_dev, c_int, c_int) -> c_int>,
    pub teardown_msi_irqs: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

#[repr(C)]
pub struct pci_controller {
    pub controller_ops: pci_controller_ops,
}

extern "C" {
    fn pci_bus_to_host(bus: *mut pci_bus) -> *mut pci_controller;
    fn pr_debug(format: *const u8, ...);
}

// PCI_CAP_ID_MSI and ENOSYS are supplied by the surrounding translation.
extern "C" {
    static PCI_CAP_ID_MSI: c_int;
    static ENOSYS: c_int;
}

pub unsafe extern "C" fn arch_setup_msi_irqs(
    dev: *mut pci_dev,
    nvec: c_int,
    type_: c_int,
) -> c_int {
    let phb: *mut pci_controller = pci_bus_to_host((*dev).bus);

    if (*phb).controller_ops.setup_msi_irqs.is_none()
        || (*phb).controller_ops.teardown_msi_irqs.is_none()
    {
        pr_debug(b"msi: Platform doesn't provide MSI callbacks.\n\0".as_ptr());
        return -ENOSYS;
    }

    /* PowerPC doesn't support multiple MSI yet */
    if type_ == PCI_CAP_ID_MSI && nvec > 1 {
        return 1;
    }

    ((*phb).controller_ops.setup_msi_irqs.unwrap())(dev, nvec, type_)
}

pub unsafe extern "C" fn arch_teardown_msi_irqs(dev: *mut pci_dev) {
    let phb: *mut pci_controller = pci_bus_to_host((*dev).bus);

    /*
     * We can be called even when arch_setup_msi_irqs() returns -ENOSYS,
     * so check the pointer again.
     */
    if let Some(teardown_msi_irqs) = (*phb).controller_ops.teardown_msi_irqs {
        teardown_msi_irqs(dev);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
