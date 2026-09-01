// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


 Array initializer for PCI card IDs

(C) Copyright AudioScience Inc. 1998-2003
*******************************************************************************/

/*NOTE: when adding new lines to this header file
  they MUST be grouped by HPI entry point.
*/

pub const HPIPCIDA: [pci_device_id; 3] = [
    pci_device_id {
        vendor: HPI_PCI_VENDOR_ID_TI,
        device: HPI_PCI_DEV_ID_DSP6205,
        subvendor: HPI_PCI_VENDOR_ID_AUDIOSCIENCE,
        subdevice: PCI_ANY_ID,
        driver_data: HPI_6205 as kernel_ulong_t,
        ..pci_device_id::default()
    },
    pci_device_id {
        vendor: HPI_PCI_VENDOR_ID_TI,
        device: HPI_PCI_DEV_ID_PCI2040,
        subvendor: HPI_PCI_VENDOR_ID_AUDIOSCIENCE,
        subdevice: PCI_ANY_ID,
        driver_data: HPI_6000 as kernel_ulong_t,
        ..pci_device_id::default()
    },
    pci_device_id {
        ..pci_device_id::default()
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
