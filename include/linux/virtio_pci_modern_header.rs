/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header.  Linux dependencies are supplied externally.

/**
 * struct virtio_pci_modern_device - info for modern PCI virtio
 * @pci_dev: Ptr to the PCI device struct
 * @common: Position of the common capability in the PCI config
 * @device: Device-specific data (non-legacy mode)
 * @notify_base: Base of vq notifications (non-legacy mode)
 * @notify_pa: Physical base of vq notifications
 * @isr: Where to read and clear interrupt
 * @notify_len: So we can sanity-check accesses
 * @device_len: So we can sanity-check accesses
 * @notify_map_cap: Capability for when we need to map notifications per-vq
 * @notify_offset_multiplier: Multiply queue_notify_off by this value
 * @modern_bars: Bitmask of BARs
 * @id: Device and vendor id
 * @device_id_check: Callback used to verify the PCI device
 * @dma_mask: Optional mask instead of the traditional DMA_BIT_MASK(64)
 */
#[repr(C)]
pub struct virtio_pci_modern_device {
    pub pci_dev: *mut pci_dev,
    pub common: *mut virtio_pci_common_cfg,
    pub device: *mut core::ffi::c_void,
    pub notify_base: *mut core::ffi::c_void,
    pub notify_pa: resource_size_t,
    pub isr: *mut u8,
    pub notify_len: usize,
    pub device_len: usize,
    pub common_len: usize,
    pub notify_map_cap: i32,
    pub notify_offset_multiplier: u32,
    pub modern_bars: i32,
    pub id: virtio_device_id,
    pub device_id_check: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
    pub dma_mask: u64,
}

/* Type-safe wrappers for natural-width I/O accesses. */
#[inline]
pub unsafe fn vp_ioread8(addr: *const u8) -> u8 { ioread8(addr) }

#[inline]
pub unsafe fn vp_ioread16(addr: *const __le16) -> u16 { ioread16(addr) }

#[inline]
pub unsafe fn vp_ioread32(addr: *const __le32) -> u32 { ioread32(addr) }

#[inline]
pub unsafe fn vp_iowrite8(value: u8, addr: *mut u8) { iowrite8(value, addr); }

#[inline]
pub unsafe fn vp_iowrite16(value: u16, addr: *mut __le16) { iowrite16(value, addr); }

#[inline]
pub unsafe fn vp_iowrite32(value: u32, addr: *mut __le32) { iowrite32(value, addr); }

#[inline]
pub unsafe fn vp_iowrite64_twopart(val: u64, lo: *mut __le32, hi: *mut __le32) {
    vp_iowrite32(val as u32, lo);
    vp_iowrite32((val >> 32) as u32, hi);
}

extern "C" {
    pub fn vp_modern_get_driver_extended_features(mdev: *mut virtio_pci_modern_device, features: *mut u64);
    pub fn vp_modern_get_extended_features(mdev: *mut virtio_pci_modern_device, features: *mut u64);
    pub fn vp_modern_set_extended_features(mdev: *mut virtio_pci_modern_device, features: *const u64);
}

#[inline]
pub unsafe fn vp_modern_get_features(mdev: *mut virtio_pci_modern_device) -> u64 {
    let mut features_array = [0u64; VIRTIO_FEATURES_U64S];
    vp_modern_get_extended_features(mdev, features_array.as_mut_ptr());
    features_array[0]
}

#[inline]
pub unsafe fn vp_modern_get_driver_features(mdev: *mut virtio_pci_modern_device) -> u64 {
    let mut features_array = [0u64; VIRTIO_FEATURES_U64S];
    vp_modern_get_driver_extended_features(mdev, features_array.as_mut_ptr());
    let mut i = 1;
    while i < VIRTIO_FEATURES_U64S {
        WARN_ON_ONCE(features_array[i]);
        i += 1;
    }
    features_array[0]
}

#[inline]
pub unsafe fn vp_modern_set_features(mdev: *mut virtio_pci_modern_device, features: u64) {
    let mut features_array = [0u64; VIRTIO_FEATURES_U64S];
    virtio_features_from_u64(features_array.as_mut_ptr(), features);
    vp_modern_set_extended_features(mdev, features_array.as_ptr());
}

extern "C" {
    pub fn vp_modern_generation(mdev: *mut virtio_pci_modern_device) -> u32;
    pub fn vp_modern_get_status(mdev: *mut virtio_pci_modern_device) -> u8;
    pub fn vp_modern_set_status(mdev: *mut virtio_pci_modern_device, status: u8);
    pub fn vp_modern_queue_vector(mdev: *mut virtio_pci_modern_device, idx: u16, vector: u16) -> u16;
    pub fn vp_modern_config_vector(mdev: *mut virtio_pci_modern_device, vector: u16) -> u16;
    pub fn vp_modern_queue_address(mdev: *mut virtio_pci_modern_device, index: u16, desc_addr: u64, driver_addr: u64, device_addr: u64);
    pub fn vp_modern_set_queue_enable(mdev: *mut virtio_pci_modern_device, idx: u16, enable: bool);
    pub fn vp_modern_get_queue_enable(mdev: *mut virtio_pci_modern_device, idx: u16) -> bool;
    pub fn vp_modern_set_queue_size(mdev: *mut virtio_pci_modern_device, idx: u16, size: u16);
    pub fn vp_modern_get_queue_size(mdev: *mut virtio_pci_modern_device, idx: u16) -> u16;
    pub fn vp_modern_get_num_queues(mdev: *mut virtio_pci_modern_device) -> u16;
    pub fn vp_modern_map_vq_notify(mdev: *mut virtio_pci_modern_device, index: u16, pa: *mut resource_size_t) -> *mut core::ffi::c_void;
    pub fn vp_modern_probe(mdev: *mut virtio_pci_modern_device) -> i32;
    pub fn vp_modern_remove(mdev: *mut virtio_pci_modern_device);
    pub fn vp_modern_get_queue_reset(mdev: *mut virtio_pci_modern_device, index: u16) -> i32;
    pub fn vp_modern_set_queue_reset(mdev: *mut virtio_pci_modern_device, index: u16);
    pub fn vp_modern_avq_num(mdev: *mut virtio_pci_modern_device) -> u16;
    pub fn vp_modern_avq_index(mdev: *mut virtio_pci_modern_device) -> u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
