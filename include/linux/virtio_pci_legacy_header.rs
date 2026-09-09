/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/mod_devicetable.h, linux/pci.h, and linux/virtio_pci.h.

use core::ffi::c_void;

#[repr(C)]
pub struct virtio_pci_legacy_device {
    pub pci_dev: *mut pci_dev,

    /* Where to read and clear interrupt */
    pub isr: *mut u8,
    /* The IO mapping for the PCI config space (legacy mode only) */
    pub ioaddr: *mut c_void,

    pub id: virtio_device_id,
}

extern "C" {
    pub fn vp_legacy_get_features(ldev: *mut virtio_pci_legacy_device) -> u64;
    pub fn vp_legacy_get_driver_features(ldev: *mut virtio_pci_legacy_device) -> u64;
    pub fn vp_legacy_set_features(ldev: *mut virtio_pci_legacy_device, features: u32);
    pub fn vp_legacy_get_status(ldev: *mut virtio_pci_legacy_device) -> u8;
    pub fn vp_legacy_set_status(ldev: *mut virtio_pci_legacy_device, status: u8);
    pub fn vp_legacy_queue_vector(
        ldev: *mut virtio_pci_legacy_device,
        idx: u16,
        vector: u16,
    ) -> u16;
    pub fn vp_legacy_config_vector(ldev: *mut virtio_pci_legacy_device, vector: u16) -> u16;
    pub fn vp_legacy_set_queue_address(
        ldev: *mut virtio_pci_legacy_device,
        index: u16,
        queue_pfn: u32,
    );
    pub fn vp_legacy_get_queue_enable(ldev: *mut virtio_pci_legacy_device, idx: u16) -> bool;
    pub fn vp_legacy_get_queue_size(ldev: *mut virtio_pci_legacy_device, idx: u16) -> u16;
    pub fn vp_legacy_probe(ldev: *mut virtio_pci_legacy_device) -> i32;
    pub fn vp_legacy_remove(ldev: *mut virtio_pci_legacy_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
