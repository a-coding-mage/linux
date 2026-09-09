/* SPDX-License-Identifier: GPL-2.0 or MIT */
/* Copyright 2025 Arm, Ltd. */

// Dependency intent preserved from ethosu_device.h and drm_gem_dma_helper.h.

#[repr(C)]
pub struct ethosu_validated_cmdstream_info {
    pub cmd_size: u32,
    pub region_size: [u64; NPU_BASEP_REGION_MAX],
    pub output_region: [bool; NPU_BASEP_REGION_MAX],
}

/**
 * struct ethosu_gem_object - Driver specific GEM object.
 */
#[repr(C)]
pub struct ethosu_gem_object {
    /** @base: Inherit from drm_gem_shmem_object. */
    pub base: drm_gem_dma_object,

    pub info: *mut ethosu_validated_cmdstream_info,

    /** @flags: Combination of drm_ethosu_bo_flags flags. */
    pub flags: u32,
}

pub unsafe fn to_ethosu_bo(obj: *mut drm_gem_object) -> *mut ethosu_gem_object {
    // `base` is the first field, so this is the Rust equivalent of
    // container_of(to_drm_gem_dma_obj(obj), struct ethosu_gem_object, base).
    to_drm_gem_dma_obj(obj) as *mut ethosu_gem_object
}

pub unsafe extern "C" {
    pub fn ethosu_gem_create_object(
        ddev: *mut drm_device,
        size: usize,
    ) -> *mut drm_gem_object;

    pub fn ethosu_gem_create_with_handle(
        file: *mut drm_file,
        ddev: *mut drm_device,
        size: *mut u64,
        flags: u32,
        handle: *mut u32,
    ) -> i32;

    pub fn ethosu_gem_cmdstream_create(
        file: *mut drm_file,
        ddev: *mut drm_device,
        size: u32,
        data: u64,
        flags: u32,
        handle: *mut u32,
    ) -> i32;
}

// Supplied by the DRM and device dependencies.
extern "C" {
    fn to_drm_gem_dma_obj(obj: *mut drm_gem_object) -> *mut drm_gem_dma_object;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
