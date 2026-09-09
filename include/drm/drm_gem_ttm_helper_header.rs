/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header guard: DRM_GEM_TTM_HELPER_H
// Dependencies supplied by the corresponding Linux DRM/TTM headers:
// linux/container_of.h, drm/drm_device.h, drm/drm_gem.h, and drm/ttm/ttm_bo.h.

pub struct iosys_map;

// #define drm_gem_ttm_of_gem(gem_obj) \
//     container_of(gem_obj, struct ttm_buffer_object, base)
#[macro_export]
macro_rules! drm_gem_ttm_of_gem {
    ($gem_obj:expr) => {
        container_of!($gem_obj, ttm_buffer_object, base)
    };
}

extern "C" {
    pub fn drm_gem_ttm_print_info(
        p: *mut drm_printer,
        indent: ::core::ffi::c_uint,
        gem: *const drm_gem_object,
    );

    pub fn drm_gem_ttm_vmap(
        gem: *mut drm_gem_object,
        map: *mut iosys_map,
    ) -> ::core::ffi::c_int;

    pub fn drm_gem_ttm_vunmap(
        gem: *mut drm_gem_object,
        map: *mut iosys_map,
    );

    pub fn drm_gem_ttm_mmap(
        gem: *mut drm_gem_object,
        vma: *mut vm_area_struct,
    ) -> ::core::ffi::c_int;

    pub fn drm_gem_ttm_dumb_map_offset(
        file: *mut drm_file,
        dev: *mut drm_device,
        handle: u32,
        offset: *mut u64,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
