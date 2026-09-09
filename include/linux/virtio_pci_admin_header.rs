/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/types.h> and <linux/pci.h> is preserved here.

#[cfg(CONFIG_VIRTIO_PCI_ADMIN_LEGACY)]
extern "C" {
    pub fn virtio_pci_admin_has_legacy_io(pdev: *mut pci_dev) -> bool;
    pub fn virtio_pci_admin_legacy_common_io_write(
        pdev: *mut pci_dev,
        offset: u8,
        size: u8,
        buf: *mut u8,
    ) -> i32;
    pub fn virtio_pci_admin_legacy_common_io_read(
        pdev: *mut pci_dev,
        offset: u8,
        size: u8,
        buf: *mut u8,
    ) -> i32;
    pub fn virtio_pci_admin_legacy_device_io_write(
        pdev: *mut pci_dev,
        offset: u8,
        size: u8,
        buf: *mut u8,
    ) -> i32;
    pub fn virtio_pci_admin_legacy_device_io_read(
        pdev: *mut pci_dev,
        offset: u8,
        size: u8,
        buf: *mut u8,
    ) -> i32;
    pub fn virtio_pci_admin_legacy_io_notify_info(
        pdev: *mut pci_dev,
        req_bar_flags: u8,
        bar: *mut u8,
        bar_offset: *mut u64,
    ) -> i32;
}

extern "C" {
    pub fn virtio_pci_admin_has_dev_parts(pdev: *mut pci_dev) -> bool;
    pub fn virtio_pci_admin_mode_set(pdev: *mut pci_dev, mode: u8) -> i32;
    pub fn virtio_pci_admin_obj_create(
        pdev: *mut pci_dev,
        obj_type: u16,
        operation_type: u8,
        obj_id: *mut u32,
    ) -> i32;
    pub fn virtio_pci_admin_obj_destroy(pdev: *mut pci_dev, obj_type: u16, id: u32) -> i32;
    pub fn virtio_pci_admin_dev_parts_metadata_get(
        pdev: *mut pci_dev,
        obj_type: u16,
        id: u32,
        metadata_type: u8,
        out: *mut u32,
    ) -> i32;
    pub fn virtio_pci_admin_dev_parts_get(
        pdev: *mut pci_dev,
        obj_type: u16,
        id: u32,
        get_type: u8,
        res_sg: *mut scatterlist,
        res_size: *mut u32,
    ) -> i32;
    pub fn virtio_pci_admin_dev_parts_set(
        pdev: *mut pci_dev,
        data_sg: *mut scatterlist,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
