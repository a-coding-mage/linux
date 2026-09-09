/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/vdpa.h. External kernel types and functions are supplied elsewhere. */

#[repr(C)]
pub struct vdpa_callback {
    pub callback: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void) -> irqreturn_t>,
    pub private: *mut core::ffi::c_void,
    pub trigger: *mut eventfd_ctx,
}

#[repr(C)]
pub struct vdpa_notification_area {
    pub addr: resource_size_t,
    pub size: resource_size_t,
}

#[repr(C)]
pub struct vdpa_vq_state_split {
    pub avail_index: u16,
}

#[repr(C)]
pub struct vdpa_vq_state_packed {
    /* C bitfields: last_avail_counter:1, last_avail_idx:15,
       last_used_counter:1, last_used_idx:15. */
    pub bits: u32,
}

#[repr(C)]
pub union vdpa_vq_state {
    pub split: vdpa_vq_state_split,
    pub packed: vdpa_vq_state_packed,
}

#[repr(C)]
pub struct vdpa_device {
    pub dev: device,
    pub vmap: virtio_map,
    pub config: *const vdpa_config_ops,
    pub map: *const virtio_map_ops,
    pub cf_lock: rw_semaphore,
    pub index: u32,
    pub features_valid: bool,
    pub use_va: bool,
    pub nvqs: u32,
    pub mdev: *mut vdpa_mgmt_dev,
    pub ngroups: u32,
    pub nas: u32,
}

#[repr(C)]
pub struct vdpa_iova_range { pub first: u64, pub last: u64 }

#[repr(C)]
pub struct vdpa_dev_set_config {
    pub device_features: u64,
    pub net: vdpa_dev_set_config_net,
    pub mask: u64,
}
#[repr(C)]
pub struct vdpa_dev_set_config_net {
    pub mac: [u8; ETH_ALEN as usize],
    pub mtu: u16,
    pub max_vq_pairs: u16,
}

#[repr(C)]
pub struct vdpa_map_file { pub file: *mut file, pub offset: u64 }

#[repr(C)]
pub struct vdpa_config_ops {
    pub set_vq_address: Option<unsafe extern "C" fn(*mut vdpa_device, u16, u64, u64, u64) -> i32>,
    pub set_vq_num: Option<unsafe extern "C" fn(*mut vdpa_device, u16, u32)>,
    pub kick_vq: Option<unsafe extern "C" fn(*mut vdpa_device, u16)>,
    pub kick_vq_with_data: Option<unsafe extern "C" fn(*mut vdpa_device, u32)>,
    pub set_vq_cb: Option<unsafe extern "C" fn(*mut vdpa_device, u16, *mut vdpa_callback)>,
    pub set_vq_ready: Option<unsafe extern "C" fn(*mut vdpa_device, u16, bool)>,
    pub get_vq_ready: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> bool>,
    pub set_vq_state: Option<unsafe extern "C" fn(*mut vdpa_device, u16, *const vdpa_vq_state) -> i32>,
    pub get_vq_state: Option<unsafe extern "C" fn(*mut vdpa_device, u16, *mut vdpa_vq_state) -> i32>,
    pub get_vendor_vq_stats: Option<unsafe extern "C" fn(*mut vdpa_device, u16, *mut sk_buff, *mut netlink_ext_ack) -> i32>,
    pub get_vq_notification: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> vdpa_notification_area>,
    pub get_vq_irq: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> i32>,
    pub get_vq_size: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> u16>,
    pub get_vq_align: Option<unsafe extern "C" fn(*mut vdpa_device) -> u32>,
    pub get_vq_group: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> u32>,
    pub get_vq_desc_group: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> u32>,
    pub get_device_features: Option<unsafe extern "C" fn(*mut vdpa_device) -> u64>,
    pub get_backend_features: Option<unsafe extern "C" fn(*const vdpa_device) -> u64>,
    pub set_driver_features: Option<unsafe extern "C" fn(*mut vdpa_device, u64) -> i32>,
    pub get_driver_features: Option<unsafe extern "C" fn(*mut vdpa_device) -> u64>,
    pub set_config_cb: Option<unsafe extern "C" fn(*mut vdpa_device, *mut vdpa_callback)>,
    pub get_vq_num_max: Option<unsafe extern "C" fn(*mut vdpa_device) -> u16>,
    pub get_vq_num_min: Option<unsafe extern "C" fn(*mut vdpa_device) -> u16>,
    pub get_device_id: Option<unsafe extern "C" fn(*mut vdpa_device) -> u32>,
    pub get_vendor_id: Option<unsafe extern "C" fn(*mut vdpa_device) -> u32>,
    pub get_status: Option<unsafe extern "C" fn(*mut vdpa_device) -> u8>,
    pub set_status: Option<unsafe extern "C" fn(*mut vdpa_device, u8)>,
    pub reset: Option<unsafe extern "C" fn(*mut vdpa_device) -> i32>,
    pub compat_reset: Option<unsafe extern "C" fn(*mut vdpa_device, u32) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut vdpa_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut vdpa_device) -> i32>,
    pub get_config_size: Option<unsafe extern "C" fn(*mut vdpa_device) -> usize>,
    pub get_config: Option<unsafe extern "C" fn(*mut vdpa_device, u32, *mut core::ffi::c_void, u32)>,
    pub set_config: Option<unsafe extern "C" fn(*mut vdpa_device, u32, *const core::ffi::c_void, u32)>,
    pub get_generation: Option<unsafe extern "C" fn(*mut vdpa_device) -> u32>,
    pub get_iova_range: Option<unsafe extern "C" fn(*mut vdpa_device) -> vdpa_iova_range>,
    pub set_vq_affinity: Option<unsafe extern "C" fn(*mut vdpa_device, u16, *const cpumask) -> i32>,
    pub get_vq_affinity: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> *const cpumask>,
    pub set_map: Option<unsafe extern "C" fn(*mut vdpa_device, u32, *mut vhost_iotlb) -> i32>,
    pub dma_map: Option<unsafe extern "C" fn(*mut vdpa_device, u32, u64, u64, u64, u32, *mut core::ffi::c_void) -> i32>,
    pub dma_unmap: Option<unsafe extern "C" fn(*mut vdpa_device, u32, u64, u64) -> i32>,
    pub reset_map: Option<unsafe extern "C" fn(*mut vdpa_device, u32) -> i32>,
    pub set_group_asid: Option<unsafe extern "C" fn(*mut vdpa_device, u32, u32) -> i32>,
    pub get_vq_map: Option<unsafe extern "C" fn(*mut vdpa_device, u16) -> virtio_map>,
    pub bind_mm: Option<unsafe extern "C" fn(*mut vdpa_device, *mut mm_struct) -> i32>,
    pub unbind_mm: Option<unsafe extern "C" fn(*mut vdpa_device)>,
    pub free: Option<unsafe extern "C" fn(*mut vdpa_device)>,
}

pub const VDPA_RESET_F_CLEAN_MAP: u32 = 1;

extern "C" {
    pub fn __vdpa_alloc_device(parent: *mut device, config: *const vdpa_config_ops,
        map: *const virtio_map_ops, ngroups: u32, nas: u32, size: usize,
        name: *const core::ffi::c_char, use_va: bool) -> *mut vdpa_device;
    pub fn vdpa_register_device(vdev: *mut vdpa_device, nvqs: u32) -> i32;
    pub fn vdpa_unregister_device(vdev: *mut vdpa_device);
    pub fn _vdpa_register_device(vdev: *mut vdpa_device, nvqs: u32) -> i32;
    pub fn _vdpa_unregister_device(vdev: *mut vdpa_device);
    pub fn __vdpa_register_driver(drv: *mut vdpa_driver, owner: *mut module) -> i32;
    pub fn vdpa_unregister_driver(drv: *mut vdpa_driver);
    pub fn vdpa_get_config(vdev: *mut vdpa_device, offset: u32, buf: *mut core::ffi::c_void, len: u32);
    pub fn vdpa_set_config(dev: *mut vdpa_device, offset: u32, buf: *const core::ffi::c_void, length: u32);
    pub fn vdpa_set_status(vdev: *mut vdpa_device, status: u8);
    pub fn vdpa_mgmtdev_register(mdev: *mut vdpa_mgmt_dev) -> i32;
    pub fn vdpa_mgmtdev_unregister(mdev: *mut vdpa_mgmt_dev);
}

#[repr(C)]
pub struct vdpa_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut vdpa_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut vdpa_device)>,
}

#[repr(C)]
pub struct vdpa_mgmtdev_ops {
    pub dev_add: Option<unsafe extern "C" fn(*mut vdpa_mgmt_dev, *const core::ffi::c_char, *const vdpa_dev_set_config) -> i32>,
    pub dev_del: Option<unsafe extern "C" fn(*mut vdpa_mgmt_dev, *mut vdpa_device)>,
    pub dev_set_attr: Option<unsafe extern "C" fn(*mut vdpa_mgmt_dev, *mut vdpa_device, *const vdpa_dev_set_config) -> i32>,
}

#[repr(C)]
pub struct vdpa_mgmt_dev {
    pub device: *mut device,
    pub ops: *const vdpa_mgmtdev_ops,
    pub id_table: *mut virtio_device_id,
    pub config_attr_mask: u64,
    pub list: list_head,
    pub supported_features: u64,
    pub max_supported_vqs: u32,
}

#[inline]
pub unsafe fn vdpa_reset(vdev: *mut vdpa_device, flags: u32) -> i32 {
    let ops = (*vdev).config;
    down_write(&mut (*vdev).cf_lock);
    (*vdev).features_valid = false;
    let ret = if !(*ops).compat_reset.is_none() && flags != 0 {
        ((*ops).compat_reset.unwrap())(vdev, flags)
    } else { ((*ops).reset.unwrap())(vdev) };
    up_write(&mut (*vdev).cf_lock);
    ret
}

#[inline]
pub unsafe fn vdpa_set_features_unlocked(vdev: *mut vdpa_device, features: u64) -> i32 {
    (*vdev).features_valid = true;
    ((*(*vdev).config).set_driver_features.unwrap())(vdev, features)
}

#[inline]
pub unsafe fn vdpa_set_features(vdev: *mut vdpa_device, features: u64) -> i32 {
    down_write(&mut (*vdev).cf_lock);
    let ret = vdpa_set_features_unlocked(vdev, features);
    up_write(&mut (*vdev).cf_lock);
    ret
}

#[inline]
pub unsafe fn vdpa_get_drvdata(vdev: *const vdpa_device) -> *mut core::ffi::c_void { dev_get_drvdata(&(*vdev).dev) }
#[inline]
pub unsafe fn vdpa_set_drvdata(vdev: *mut vdpa_device, data: *mut core::ffi::c_void) { dev_set_drvdata(&mut (*vdev).dev, data); }
#[inline]
pub unsafe fn vdpa_get_map(vdev: *mut vdpa_device) -> virtio_map { (*vdev).vmap }

#[inline]
pub unsafe fn drv_to_vdpa(driver: *mut device_driver) -> *mut vdpa_driver {
    container_of(driver, vdpa_driver, driver)
}

#[inline]
pub unsafe fn dev_to_vdpa(dev: *mut device) -> *mut vdpa_device {
    container_of(dev, vdpa_device, dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
