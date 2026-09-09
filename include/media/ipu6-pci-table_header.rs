/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Intel Corporation
 */

/* Dependency intent: pci_device_id, PCI_VENDOR_ID_INTEL, and PCI_ANY_ID are
 * supplied by the Linux PCI bindings. */

pub const PCI_DEVICE_ID_INTEL_IPU6: u16 = 0x9a19;
pub const PCI_DEVICE_ID_INTEL_IPU6SE: u16 = 0x4e19;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_ADLP: u16 = 0x465d;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_ADLN: u16 = 0x462e;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_RPLP: u16 = 0xa75d;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_MTL: u16 = 0x7d19;

/* Equivalent of the Linux PCI_VDEVICE(INTEL, device) initializer. */
macro_rules! pci_vdevice {
    ($device:expr) => {
        pci_device_id {
            vendor: PCI_VENDOR_ID_INTEL,
            device: $device,
            subvendor: PCI_ANY_ID,
            subdevice: PCI_ANY_ID,
            class: 0,
            class_mask: 0,
            driver_data: 0,
        }
    };
}

pub static ipu6_pci_tbl: [pci_device_id; 7] = [
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6),
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6SE),
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6EP_ADLP),
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6EP_ADLN),
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6EP_RPLP),
    pci_vdevice!(PCI_DEVICE_ID_INTEL_IPU6EP_MTL),
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
