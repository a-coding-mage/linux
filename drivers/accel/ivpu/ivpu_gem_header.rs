/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2025 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel/Rust translation.
use core::ffi::c_void;

#[repr(C)]
pub struct ivpu_bo {
    pub base: drm_gem_shmem_object,
    pub ctx: *mut ivpu_mmu_context,
    pub bo_list_node: list_head,
    pub mm_node: drm_mm_node,
    pub vpu_addr: u64,
    pub flags: u32,
    pub job_status: u32, /* Valid only for command buffer */
    pub ctx_id: u32,
    pub mmu_mapped: bool,
}

extern "C" {
    pub fn ivpu_bo_bind(bo: *mut ivpu_bo) -> i32;
    pub fn ivpu_bo_unbind_all_bos_from_context(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
    );

    pub fn ivpu_gem_create_object(
        dev: *mut drm_device,
        size: usize,
    ) -> *mut drm_gem_object;
    pub fn ivpu_gem_prime_import(
        dev: *mut drm_device,
        dma_buf: *mut dma_buf,
    ) -> *mut drm_gem_object;
    pub fn ivpu_bo_create(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
        range: *mut ivpu_addr_range,
        size: u64,
        flags: u32,
    ) -> *mut ivpu_bo;
    pub fn ivpu_bo_create_runtime(
        vdev: *mut ivpu_device,
        addr: u64,
        size: u64,
        flags: u32,
    ) -> *mut ivpu_bo;
    pub fn ivpu_bo_create_global(
        vdev: *mut ivpu_device,
        size: u64,
        flags: u32,
    ) -> *mut ivpu_bo;
    pub fn ivpu_bo_free(bo: *mut ivpu_bo);

    pub fn ivpu_bo_create_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_bo_info_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_bo_wait_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_bo_create_from_userptr_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;

    pub fn ivpu_bo_list(dev: *mut drm_device, p: *mut drm_printer);
    pub fn ivpu_bo_list_print(dev: *mut drm_device);
}

pub unsafe fn to_ivpu_bo(obj: *mut drm_gem_object) -> *mut ivpu_bo {
    container_of(obj, ivpu_bo, base.base)
}

pub unsafe fn ivpu_bo_vaddr(bo: *mut ivpu_bo) -> *mut c_void {
    (*bo).base.vaddr
}

pub unsafe fn ivpu_bo_size(bo: *mut ivpu_bo) -> usize {
    (*bo).base.base.size
}

pub unsafe fn ivpu_bo_cache_mode(bo: *mut ivpu_bo) -> u32 {
    (*bo).flags & DRM_IVPU_BO_CACHE_MASK
}

pub unsafe fn ivpu_bo_to_vdev(bo: *mut ivpu_bo) -> *mut ivpu_device {
    to_ivpu_device((*bo).base.base.dev)
}

pub unsafe fn ivpu_bo_is_snooped(bo: *mut ivpu_bo) -> bool {
    if ivpu_is_force_snoop_enabled(ivpu_bo_to_vdev(bo)) {
        return true;
    }

    ivpu_bo_cache_mode(bo) == DRM_IVPU_BO_CACHED
}

pub unsafe fn ivpu_bo_is_read_only(bo: *mut ivpu_bo) -> bool {
    ((*bo).flags & DRM_IVPU_BO_READ_ONLY) != 0
}

pub unsafe fn ivpu_bo_is_resident(bo: *mut ivpu_bo) -> bool {
    !(*bo).base.pages.is_null()
}

pub unsafe fn ivpu_to_cpu_addr(bo: *mut ivpu_bo, vpu_addr: u32) -> *mut c_void {
    if vpu_addr < (*bo).vpu_addr as u32 {
        return core::ptr::null_mut();
    }

    if vpu_addr as u64 >= (*bo).vpu_addr.wrapping_add(ivpu_bo_size(bo) as u64) {
        return core::ptr::null_mut();
    }

    (ivpu_bo_vaddr(bo) as *mut u8).add((vpu_addr as u64 - (*bo).vpu_addr) as usize)
        as *mut c_void
}

pub unsafe fn cpu_to_vpu_addr(bo: *mut ivpu_bo, cpu_addr: *mut c_void) -> u32 {
    let base = ivpu_bo_vaddr(bo) as *mut u8;
    let addr = cpu_addr as *mut u8;
    if addr < base {
        return 0;
    }

    if addr >= base.add(ivpu_bo_size(bo)) {
        return 0;
    }

    ((*bo).vpu_addr + addr.offset_from(base) as u64) as u32
}

pub unsafe fn ivpu_bo_is_mappable(bo: *mut ivpu_bo) -> bool {
    ((*bo).flags & DRM_IVPU_BO_MAPPABLE) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
