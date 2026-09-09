/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependency supplied by the DRM headers: drm_gem_shmem_helper.h
use crate::{drm_device, drm_file, drm_gem_object, drm_gem_shmem_object, drm_mm_node};

#[repr(C)]
pub struct rocket_gem_object {
    pub base: drm_gem_shmem_object,
    pub driver_priv: *mut rocket_file_priv,
    pub domain: *mut rocket_iommu_domain,
    pub mm: drm_mm_node,
    pub size: usize,
    pub offset: u32,
}

// Types supplied by other parts of the driver.
pub enum rocket_file_priv {}
pub enum rocket_iommu_domain {}

extern "C" {
    pub fn rocket_gem_create_object(
        dev: *mut drm_device,
        size: usize,
    ) -> *mut drm_gem_object;

    pub fn rocket_ioctl_create_bo(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file: *mut drm_file,
    ) -> core::ffi::c_int;

    pub fn rocket_ioctl_prep_bo(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file: *mut drm_file,
    ) -> core::ffi::c_int;

    pub fn rocket_ioctl_fini_bo(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file: *mut drm_file,
    ) -> core::ffi::c_int;

    // C macro supplied by the DRM headers.
    pub fn to_drm_gem_shmem_obj(obj: *mut drm_gem_object) -> *mut drm_gem_shmem_object;
}

#[inline]
pub unsafe fn to_rocket_bo(obj: *mut drm_gem_object) -> *mut rocket_gem_object {
    let shmem = to_drm_gem_shmem_obj(obj);
    (shmem as *mut u8).sub(core::mem::offset_of!(rocket_gem_object, base))
        as *mut rocket_gem_object
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
