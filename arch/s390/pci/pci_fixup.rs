// SPDX-License-Identifier: GPL-2.0
/*
 * Exceptions for specific devices,
 *
 * Copyright IBM Corp. 2025
 *
 * Author(s):
 *   Niklas Schnelle <schnelle@linux.ibm.com>
 */

// Dependency supplied by the Linux PCI subsystem.

unsafe fn zpci_ism_bar_no_mmap(pdev: *mut pci_dev) {
    /*
     * ISM's BAR is special. Drivers written for ISM know
     * how to handle this but others need to be aware of their
     * special nature e.g. to prevent attempts to mmap() it.
     */
    (*pdev).non_mappable_bars = 1;
}

// Build-time PCI fixup registration, translated from DECLARE_PCI_FIXUP_EARLY.
DECLARE_PCI_FIXUP_EARLY!(
    PCI_VENDOR_ID_IBM,
    PCI_DEVICE_ID_IBM_ISM,
    zpci_ism_bar_no_mmap
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
