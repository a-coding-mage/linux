/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/err.h, linux/bug.h, linux/virtio.h,
// linux/virtio_byteorder.h, linux/compiler_types.h, uapi/linux/virtio_config.h

pub struct irq_affinity;

#[repr(C)]
pub struct virtio_shm_region {
    pub addr: u64,
    pub len: u64,
}

pub type vq_callback_t = unsafe extern "C" fn(*mut virtqueue);

#[repr(C)]
pub struct virtqueue_info {
    pub name: *const core::ffi::c_char,
    pub callback: Option<vq_callback_t>,
    pub ctx: bool,
}

#[repr(C)]
pub struct virtio_config_ops {
    pub get: Option<unsafe extern "C" fn(*mut virtio_device, u32, *mut core::ffi::c_void, u32)>,
    pub set: Option<unsafe extern "C" fn(*mut virtio_device, u32, *const core::ffi::c_void, u32)>,
    pub generation: Option<unsafe extern "C" fn(*mut virtio_device) -> u32>,
    pub get_status: Option<unsafe extern "C" fn(*mut virtio_device) -> u8>,
    pub set_status: Option<unsafe extern "C" fn(*mut virtio_device, u8)>,
    pub reset: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub find_vqs: Option<unsafe extern "C" fn(*mut virtio_device, u32, *mut *mut virtqueue, *mut virtqueue_info, *mut irq_affinity) -> i32>,
    pub del_vqs: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub synchronize_cbs: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub get_features: Option<unsafe extern "C" fn(*mut virtio_device) -> u64>,
    pub get_extended_features: Option<unsafe extern "C" fn(*mut virtio_device, *mut u64)>,
    pub finalize_features: Option<unsafe extern "C" fn(*mut virtio_device) -> i32>,
    pub bus_name: Option<unsafe extern "C" fn(*mut virtio_device) -> *const core::ffi::c_char>,
    pub set_vq_affinity: Option<unsafe extern "C" fn(*mut virtqueue, *const cpumask) -> i32>,
    pub get_vq_affinity: Option<unsafe extern "C" fn(*mut virtio_device, i32) -> *const cpumask>,
    pub get_shm_region: Option<unsafe extern "C" fn(*mut virtio_device, *mut virtio_shm_region, u8) -> bool>,
    pub disable_vq_and_reset: Option<unsafe extern "C" fn(*mut virtqueue) -> i32>,
    pub enable_vq_after_reset: Option<unsafe extern "C" fn(*mut virtqueue) -> i32>,
}

#[repr(C)]
pub struct virtio_map_ops {
    pub map_page: Option<unsafe extern "C" fn(virtio_map, *mut page, usize, usize, dma_data_direction, usize) -> dma_addr_t>,
    pub unmap_page: Option<unsafe extern "C" fn(virtio_map, dma_addr_t, usize, dma_data_direction, usize)>,
    pub sync_single_for_cpu: Option<unsafe extern "C" fn(virtio_map, dma_addr_t, usize, dma_data_direction)>,
    pub sync_single_for_device: Option<unsafe extern "C" fn(virtio_map, dma_addr_t, usize, dma_data_direction)>,
    pub alloc: Option<unsafe extern "C" fn(virtio_map, usize, *mut dma_addr_t, gfp_t) -> *mut core::ffi::c_void>,
    pub free: Option<unsafe extern "C" fn(virtio_map, usize, *mut core::ffi::c_void, dma_addr_t, usize)>,
    pub need_sync: Option<unsafe extern "C" fn(virtio_map, dma_addr_t) -> bool>,
    pub mapping_error: Option<unsafe extern "C" fn(virtio_map, dma_addr_t) -> i32>,
    pub max_mapping_size: Option<unsafe extern "C" fn(virtio_map) -> usize>,
}

extern "C" {
    pub fn virtio_check_driver_offered_feature(vdev: *const virtio_device, fbit: u32);
}

pub unsafe fn __virtio_test_bit(vdev: *const virtio_device, fbit: u32) -> bool {
    virtio_features_test_bit((*vdev).features_array, fbit)
}

pub unsafe fn __virtio_set_bit(vdev: *mut virtio_device, fbit: u32) {
    virtio_features_set_bit((*vdev).features_array, fbit)
}

pub unsafe fn __virtio_clear_bit(vdev: *mut virtio_device, fbit: u32) {
    virtio_features_clear_bit((*vdev).features_array, fbit)
}

pub unsafe fn virtio_has_feature(vdev: *const virtio_device, fbit: u32) -> bool {
    if fbit < VIRTIO_TRANSPORT_F_START { virtio_check_driver_offered_feature(vdev, fbit); }
    __virtio_test_bit(vdev, fbit)
}

pub unsafe fn virtio_get_features(vdev: *mut virtio_device, features_out: *mut u64) {
    if let Some(f) = (*(*vdev).config).get_extended_features { f(vdev, features_out); return; }
    virtio_features_from_u64(features_out, ((*(*vdev).config).get_features.unwrap())(vdev));
}

pub unsafe fn virtio_has_dma_quirk(vdev: *const virtio_device) -> bool { !virtio_has_feature(vdev, VIRTIO_F_ACCESS_PLATFORM) }

pub unsafe fn virtio_find_vqs(vdev: *mut virtio_device, nvqs: u32, vqs: *mut *mut virtqueue, info: *mut virtqueue_info, desc: *mut irq_affinity) -> i32 {
    ((*(*vdev).config).find_vqs.unwrap())(vdev, nvqs, vqs, info, desc)
}

pub unsafe fn virtio_find_single_vq(vdev: *mut virtio_device, c: Option<vq_callback_t>, n: *const core::ffi::c_char) -> *mut virtqueue {
    let mut info = virtqueue_info { name: n, callback: c, ctx: false };
    let mut vq: *mut virtqueue = core::ptr::null_mut();
    let err = virtio_find_vqs(vdev, 1, &mut vq, &mut info, core::ptr::null_mut());
    if err < 0 { return ERR_PTR(err); } vq
}

pub unsafe fn virtio_synchronize_cbs(dev: *mut virtio_device) {
    if let Some(f) = (*(*dev).config).synchronize_cbs { f(dev); } else { synchronize_rcu(); }
}

pub unsafe fn virtio_device_ready(dev: *mut virtio_device) {
    let status = ((*(*dev).config).get_status.unwrap())(dev);
    WARN_ON(status & VIRTIO_CONFIG_S_DRIVER_OK != 0);
    // CONFIG_VIRTIO_HARDEN_NOTIFICATION: synchronize callbacks and unbreak device.
    ((*(*dev).config).set_status.unwrap())(dev, status | VIRTIO_CONFIG_S_DRIVER_OK);
}

pub unsafe fn virtio_bus_name(vdev: *mut virtio_device) -> *const core::ffi::c_char {
    if (*(*vdev).config).bus_name.is_none() { return b"virtio\0".as_ptr() as *const _; }
    ((*(*vdev).config).bus_name.unwrap())(vdev)
}

pub unsafe fn virtqueue_set_affinity(vq: *mut virtqueue, mask: *const cpumask) -> i32 {
    let vdev = (*vq).vdev;
    if let Some(f) = (*(*vdev).config).set_vq_affinity { f(vq, mask) } else { 0 }
}

pub unsafe fn virtio_get_shm_region(vdev: *mut virtio_device, out: *mut virtio_shm_region, id: u8) -> bool {
    match (*(*vdev).config).get_shm_region { Some(f) => f(vdev, out, id), None => false }
}

pub unsafe fn virtio_is_little_endian(vdev: *mut virtio_device) -> bool { virtio_has_feature(vdev, VIRTIO_F_VERSION_1) || virtio_legacy_is_little_endian() }
pub unsafe fn virtio16_to_cpu(vdev: *mut virtio_device, val: __virtio16) -> u16 { __virtio16_to_cpu(virtio_is_little_endian(vdev), val) }
pub unsafe fn cpu_to_virtio16(vdev: *mut virtio_device, val: u16) -> __virtio16 { __cpu_to_virtio16(virtio_is_little_endian(vdev), val) }
pub unsafe fn virtio32_to_cpu(vdev: *mut virtio_device, val: __virtio32) -> u32 { __virtio32_to_cpu(virtio_is_little_endian(vdev), val) }
pub unsafe fn cpu_to_virtio32(vdev: *mut virtio_device, val: u32) -> __virtio32 { __cpu_to_virtio32(virtio_is_little_endian(vdev), val) }
pub unsafe fn virtio64_to_cpu(vdev: *mut virtio_device, val: __virtio64) -> u64 { __virtio64_to_cpu(virtio_is_little_endian(vdev), val) }
pub unsafe fn cpu_to_virtio64(vdev: *mut virtio_device, val: u64) -> __virtio64 { __cpu_to_virtio64(virtio_is_little_endian(vdev), val) }

pub unsafe fn __virtio_cread_many(vdev: *mut virtio_device, offset: u32, buf: *mut u8, count: usize, bytes: usize) {
    let mut old; let mut gen = (*(*vdev).config).generation.map_or(0, |f| f(vdev));
    loop { old = gen; for i in 0..count { ((*(*vdev).config).get.unwrap())(vdev, offset + (bytes * i) as u32, buf.add(i * bytes) as *mut _, bytes as u32); } gen = (*(*vdev).config).generation.map_or(0, |f| f(vdev)); if gen == old { break; } }
}

pub unsafe fn virtio_cread_bytes(vdev: *mut virtio_device, offset: u32, buf: *mut u8, len: usize) { __virtio_cread_many(vdev, offset, buf, len, 1); }
pub unsafe fn virtio_cread8(vdev: *mut virtio_device, offset: u32) -> u8 { let mut r=0; ((*(*vdev).config).get.unwrap())(vdev, offset, &mut r as *mut _ as *mut _, 1); r }
pub unsafe fn virtio_cwrite8(vdev: *mut virtio_device, offset: u32, val: u8) { ((*(*vdev).config).set.unwrap())(vdev, offset, &val as *const _ as *const _, 1); }
pub unsafe fn virtio_cread16(vdev: *mut virtio_device, offset: u32) -> u16 { let mut r=core::mem::zeroed(); ((*(*vdev).config).get.unwrap())(vdev, offset, &mut r, 2); virtio16_to_cpu(vdev,r) }
pub unsafe fn virtio_cwrite16(vdev: *mut virtio_device, offset: u32, val: u16) { let v=cpu_to_virtio16(vdev,val); ((*(*vdev).config).set.unwrap())(vdev,offset,&v,2); }
pub unsafe fn virtio_cread32(vdev: *mut virtio_device, offset: u32) -> u32 { let mut r=core::mem::zeroed(); ((*(*vdev).config).get.unwrap())(vdev,offset,&mut r,4); virtio32_to_cpu(vdev,r) }
pub unsafe fn virtio_cwrite32(vdev: *mut virtio_device, offset: u32, val: u32) { let v=cpu_to_virtio32(vdev,val); ((*(*vdev).config).set.unwrap())(vdev,offset,&v,4); }
pub unsafe fn virtio_cread64(vdev: *mut virtio_device, offset: u32) -> u64 { let mut r=core::mem::zeroed(); __virtio_cread_many(vdev,offset,&mut r as *mut _ as *mut u8,1,8); virtio64_to_cpu(vdev,r) }
pub unsafe fn virtio_cwrite64(vdev: *mut virtio_device, offset: u32, val: u64) { let v=cpu_to_virtio64(vdev,val); ((*(*vdev).config).set.unwrap())(vdev,offset,&v,8); }

// C _Generic config-space accessors are retained as Rust macro call-throughs.
#[macro_export] macro_rules! virtio_cread { ($v:expr, $s:ty, $m:ident, $p:expr) => {{ $crate::__virtio_cread_many($v, core::mem::offset_of!($s, $m) as u32, $p as *mut u8, 1, core::mem::size_of::<$s>()); }} }
#[macro_export] macro_rules! virtio_cwrite { ($v:expr, $s:ty, $m:ident, $p:expr) => {{ ((*(*$v).config).set.unwrap())($v, core::mem::offset_of!($s, $m) as u32, $p as *const _, core::mem::size_of_val(&$p) as u32); }} }
#[macro_export] macro_rules! virtio_cread_feature { ($v:expr, $f:expr, $s:ty, $m:ident, $p:expr) => {{ if !$crate::virtio_has_feature($v,$f) { -ENOENT } else { $crate::virtio_cread!($v,$s,$m,$p); 0 } }} }
#[macro_export] macro_rules! virtio_cread_le_feature { ($v:expr, $f:expr, $s:ty, $m:ident, $p:expr) => {{ $crate::virtio_cread_feature!($v,$f,$s,$m,$p) }} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
