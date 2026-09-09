/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Translated from amdxdna_gem.h. External kernel and driver types/functions
// are supplied by the surrounding translation unit.

#[repr(C)]
pub struct amdxdna_umap {
    pub notifier: mmu_interval_notifier,
    pub range: hmm_range,
    pub hmm_unreg_work: work_struct,
    pub abo: *mut amdxdna_gem_obj,
    pub node: list_head,
    pub refcnt: kref,
    pub invalid: bool,
    pub unmapped: bool,
}

#[repr(C)]
pub struct amdxdna_mem {
    pub kva: *mut core::ffi::c_void,
    pub dma_addr: u64,
    pub size: usize,
    pub umap_list: list_head,
    pub map_invalid: bool,
    /*
     * Cache the first mmap uva as PASID addr, which can be accessed by driver
     * without taking notifier_lock.
     */
    pub uva: u64,
}

#[repr(C)]
pub struct amdxdna_gem_obj {
    pub base: drm_gem_shmem_object,
    pub client: *mut amdxdna_client,
    pub type_: u8,
    pub pinned: bool,
    pub lock: mutex, /* Protects: pinned, mem.kva, open_ref */
    pub mem: amdxdna_mem,
    pub open_ref: i32,

    /* Below members are initialized when needed */
    pub mm_node: drm_mm_node, /* For AMDXDNA_BO_DEV */
    pub heap_start_id: u32,
    pub heap_end_id: u32,
    pub dev_addr: u64, /* For heap bo */
    pub assigned_hwctx: u32,
    pub dma_buf: *mut dma_buf,
    pub attach: *mut dma_buf_attachment,

    /* True, if BO is managed by XRT, not application */
    pub internal: bool,
    /* True, if BO is not exportable */
    pub private_buffer: bool,
}

#[inline]
pub unsafe fn to_gobj(obj: *mut amdxdna_gem_obj) -> *mut drm_gem_object {
    &mut (*obj).base.base
}

#[inline]
pub unsafe fn is_import_bo(obj: *mut amdxdna_gem_obj) -> bool {
    !(*obj).attach.is_null()
}

#[inline]
pub unsafe fn to_xdna_obj(gobj: *mut drm_gem_object) -> *mut amdxdna_gem_obj {
    container_of(gobj, amdxdna_gem_obj, base.base)
}

extern "C" {
    pub fn amdxdna_gem_get_obj(
        client: *mut amdxdna_client,
        bo_hdl: u32,
        bo_type: u8,
    ) -> *mut amdxdna_gem_obj;
}

#[inline]
pub unsafe fn amdxdna_gem_put_obj(abo: *mut amdxdna_gem_obj) {
    drm_gem_object_put(to_gobj(abo));
}

/*
 * Obtain the user virtual address for accessing the BO.
 * It can be used for device to access the BO when PASID is enabled.
 */
#[inline]
pub unsafe fn amdxdna_gem_uva(abo: *mut amdxdna_gem_obj) -> u64 {
    (*abo).mem.uva
}

extern "C" {
    pub fn amdxdna_gem_vmap(abo: *mut amdxdna_gem_obj) -> *mut core::ffi::c_void;
    pub fn amdxdna_gem_dev_addr(abo: *mut amdxdna_gem_obj) -> u64;
}

#[inline]
pub unsafe fn amdxdna_dev_bo_offset(abo: *mut amdxdna_gem_obj) -> u64 {
    amdxdna_gem_dev_addr(abo)
        .wrapping_sub((*to_xdna_dev(to_gobj(abo))).dev_info.dev_mem_base)
}

#[inline]
pub unsafe fn amdxdna_obj_dma_addr(abo: *mut amdxdna_gem_obj) -> u64 {
    /*
     * amdxdna_gem_obj_open() calls amdxdna_dma_map_bo() only when PASID is
     * off, leaving mem.dma_addr at AMDXDNA_INVALID_ADDR when PASID is on.
     * Avoid dereferencing abo->client, which is cleared to NULL by
     * amdxdna_gem_obj_close() while internal kernel references remain.
     */
    if (*abo).mem.dma_addr != AMDXDNA_INVALID_ADDR {
        (*abo).mem.dma_addr
    } else {
        amdxdna_gem_uva(abo)
    }
}

extern "C" {
    pub fn amdxdna_umap_put(mapp: *mut amdxdna_umap);

    pub fn amdxdna_gem_create_shmem_object_cb(
        dev: *mut drm_device,
        size: usize,
    ) -> *mut drm_gem_object;
    pub fn amdxdna_gem_prime_import(
        dev: *mut drm_device,
        dma_buf: *mut dma_buf,
    ) -> *mut drm_gem_object;
    pub fn amdxdna_drm_create_dev_bo(
        dev: *mut drm_device,
        args: *mut amdxdna_drm_create_bo,
        filp: *mut drm_file,
    ) -> *mut amdxdna_gem_obj;

    pub fn amdxdna_gem_pin_nolock(abo: *mut amdxdna_gem_obj) -> i32;
    pub fn amdxdna_gem_pin(abo: *mut amdxdna_gem_obj) -> i32;
    pub fn amdxdna_gem_unpin(abo: *mut amdxdna_gem_obj);

    pub fn amdxdna_drm_create_bo_ioctl(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        filp: *mut drm_file,
    ) -> i32;
    pub fn amdxdna_drm_get_bo_info_ioctl(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        filp: *mut drm_file,
    ) -> i32;
    pub fn amdxdna_drm_sync_bo_ioctl(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        filp: *mut drm_file,
    ) -> i32;
    pub fn amdxdna_drm_get_bo_usage(
        dev: *mut drm_device,
        args: *mut amdxdna_drm_get_array,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
