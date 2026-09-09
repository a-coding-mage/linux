/* SPDX-License-Identifier: MIT */

// C dependencies: linux/dma-direction.h, linux/hmm.h, linux/memremap.h,
// linux/types.h

pub const NR_PAGES: fn(u32) -> u32 = |order| 1u32 << order;

pub enum dma_data_direction {}
pub type dma_addr_t = u64;

pub enum dma_fence {}
pub enum drm_pagemap_cache {}
pub enum drm_pagemap_dev_hold {}
pub enum drm_pagemap_zdd {}
pub enum device {}
pub enum page {}
pub enum mm_struct {}
pub enum drm_device {}
pub enum dev_pagemap {}
pub enum dev_pagemap_ops {}
pub enum kref {}
pub enum list_head {}
pub enum completion {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_interconnect_protocol {
    DRM_INTERCONNECT_SYSTEM,
    DRM_INTERCONNECT_DRIVER,
    // A driver can add private values beyond DRM_INTERCONNECT_DRIVER.
}

#[repr(C)]
pub struct drm_pagemap_addr {
    pub addr: dma_addr_t,
    // C bit-fields: proto:54, order:8, dir:2.
    pub proto: u64,
    pub order: u64,
    pub dir: u64,
}

#[inline]
pub fn drm_pagemap_addr_encode(
    addr: dma_addr_t,
    proto: drm_interconnect_protocol,
    order: u32,
    dir: dma_data_direction,
) -> drm_pagemap_addr {
    drm_pagemap_addr { addr, proto: proto as u64, order: order as u64, dir: dir as u64 }
}

#[repr(C)]
pub struct drm_pagemap_ops {
    pub device_map: Option<unsafe extern "C" fn(*mut drm_pagemap, *mut device, *mut page, u32, dma_data_direction) -> drm_pagemap_addr>,
    pub device_unmap: Option<unsafe extern "C" fn(*mut drm_pagemap, *mut device, *const drm_pagemap_addr)>,
    pub populate_mm: Option<unsafe extern "C" fn(*mut drm_pagemap, usize, usize, *mut mm_struct, usize) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut drm_pagemap, bool)>,
}

#[repr(C)]
pub struct drm_pagemap {
    pub ops: *const drm_pagemap_ops,
    pub ref_: kref,
    pub drm: *mut drm_device,
    pub pagemap: *mut dev_pagemap,
    pub dev_hold: *mut drm_pagemap_dev_hold,
    pub cache: *mut drm_pagemap_cache,
    pub shrink_link: list_head,
}

#[repr(C)]
pub struct drm_pagemap_devmem_ops {
    pub devmem_release: Option<unsafe extern "C" fn(*mut drm_pagemap_devmem)>,
    pub populate_devmem_pfn: Option<unsafe extern "C" fn(*mut drm_pagemap_devmem, usize, *mut usize) -> i32>,
    pub copy_to_devmem: Option<unsafe extern "C" fn(*mut *mut page, *mut drm_pagemap_addr, usize, *mut dma_fence) -> i32>,
    pub copy_to_ram: Option<unsafe extern "C" fn(*mut *mut page, *mut drm_pagemap_addr, usize, *mut dma_fence) -> i32>,
}

// The following declarations are present when CONFIG_ZONE_DEVICE is enabled.
unsafe extern "C" {
    pub fn drm_pagemap_init(dpagemap: *mut drm_pagemap, pagemap: *mut dev_pagemap, drm: *mut drm_device, ops: *const drm_pagemap_ops) -> i32;
    pub fn drm_pagemap_create(drm: *mut drm_device, pagemap: *mut dev_pagemap, ops: *const drm_pagemap_ops) -> *mut drm_pagemap;
    pub fn drm_pagemap_page_to_dpagemap(page: *mut page) -> *mut drm_pagemap;
    pub fn drm_pagemap_put(dpagemap: *mut drm_pagemap);
}

#[inline]
pub unsafe fn drm_pagemap_get(dpagemap: *mut drm_pagemap) -> *mut drm_pagemap {
    if !dpagemap.is_null() { kref_get(&mut (*dpagemap).ref_); }
    dpagemap
}

#[inline]
pub unsafe fn drm_pagemap_get_unless_zero(dpagemap: *mut drm_pagemap) -> *mut drm_pagemap {
    if !dpagemap.is_null() && kref_get_unless_zero(&mut (*dpagemap).ref_) { dpagemap } else { core::ptr::null_mut() }
}

#[repr(C)]
pub struct drm_pagemap_devmem {
    pub dev: *mut device,
    pub mm: *mut mm_struct,
    pub detached: completion,
    pub ops: *const drm_pagemap_devmem_ops,
    pub dpagemap: *mut drm_pagemap,
    pub size: usize,
    pub timeslice_expiration: u64,
    pub pre_migrate_fence: *mut dma_fence,
}

#[repr(C)]
pub struct drm_pagemap_migrate_details {
    pub timeslice_ms: usize,
    // C bit-field: can_migrate_same_pagemap:1.
    pub can_migrate_same_pagemap: u32,
}

unsafe extern "C" {
    pub fn drm_pagemap_migrate_to_devmem(a: *mut drm_pagemap_devmem, mm: *mut mm_struct, start: usize, end: usize, details: *const drm_pagemap_migrate_details) -> i32;
    pub fn drm_pagemap_evict_to_ram(a: *mut drm_pagemap_devmem) -> i32;
    pub fn drm_pagemap_pagemap_ops_get() -> *const dev_pagemap_ops;
    pub fn drm_pagemap_devmem_init(a: *mut drm_pagemap_devmem, dev: *mut device, mm: *mut mm_struct, ops: *const drm_pagemap_devmem_ops, dpagemap: *mut drm_pagemap, size: usize, fence: *mut dma_fence);
    pub fn drm_pagemap_populate_mm(dpagemap: *mut drm_pagemap, start: usize, end: usize, mm: *mut mm_struct, timeslice_ms: usize) -> i32;
    pub fn drm_pagemap_destroy(dpagemap: *mut drm_pagemap, is_atomic_or_reclaim: bool);
    pub fn drm_pagemap_reinit(dpagemap: *mut drm_pagemap) -> i32;
}

#[inline]
pub unsafe fn drm_pagemap_page_zone_device_data(_page: *mut page) -> *mut drm_pagemap_zdd { core::ptr::null_mut() }

extern "C" {
    fn kref_get(kref: *mut kref);
    fn kref_get_unless_zero(kref: *mut kref) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
