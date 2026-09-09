/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header dependencies are supplied by the surrounding kernel bindings.

pub const DRM_GEM_VRAM_PL_FLAG_SYSTEM: u32 = 1 << 0;
pub const DRM_GEM_VRAM_PL_FLAG_VRAM: u32 = 1 << 1;
pub const DRM_GEM_VRAM_PL_FLAG_TOPDOWN: u32 = 1 << 2;

pub struct drm_mode_create_dumb;
pub struct drm_plane;
pub struct drm_plane_state;
pub struct filp;
pub struct vm_area_struct;
pub struct drm_device;
pub struct drm_file;
pub struct drm_minor;
pub struct drm_display_mode;
pub struct drm_gem_object;
pub struct iosys_map;
pub struct ttm_buffer_object;
pub struct ttm_placement;
pub struct ttm_place;
pub struct ttm_device;

#[repr(C)]
pub struct drm_gem_vram_object {
    pub bo: ttm_buffer_object,
    pub map: iosys_map,
    pub vmap_use_count: u32,
    pub placement: ttm_placement,
    pub placements: [ttm_place; 2],
}

#[inline]
pub unsafe fn drm_gem_vram_of_bo(bo: *mut ttm_buffer_object) -> *mut drm_gem_vram_object {
    (bo as *mut u8).sub(core::mem::offset_of!(drm_gem_vram_object, bo))
        as *mut drm_gem_vram_object
}

#[inline]
pub unsafe fn drm_gem_vram_of_gem(gem: *mut drm_gem_object) -> *mut drm_gem_vram_object {
    // container_of(gem, struct drm_gem_vram_object, bo.base)
    gem as *mut drm_gem_vram_object
}

unsafe extern "C" {
    pub fn drm_gem_vram_create(
        dev: *mut drm_device,
        size: usize,
        pg_align: usize,
    ) -> *mut drm_gem_vram_object;
    pub fn drm_gem_vram_put(gbo: *mut drm_gem_vram_object);
    pub fn drm_gem_vram_offset(gbo: *mut drm_gem_vram_object) -> i64;
    pub fn drm_gem_vram_vmap(gbo: *mut drm_gem_vram_object, map: *mut iosys_map) -> i32;
    pub fn drm_gem_vram_vunmap(gbo: *mut drm_gem_vram_object, map: *mut iosys_map);
    pub fn drm_gem_vram_fill_create_dumb(
        file: *mut drm_file,
        dev: *mut drm_device,
        pg_align: usize,
        pitch_align: usize,
        args: *mut drm_mode_create_dumb,
    ) -> i32;

    pub fn drm_gem_vram_driver_dumb_create(
        file: *mut drm_file,
        dev: *mut drm_device,
        args: *mut drm_mode_create_dumb,
    ) -> i32;

    pub fn drm_gem_vram_plane_helper_prepare_fb(
        plane: *mut drm_plane,
        new_state: *mut drm_plane_state,
    ) -> i32;
    pub fn drm_gem_vram_plane_helper_cleanup_fb(
        plane: *mut drm_plane,
        old_state: *mut drm_plane_state,
    );

    pub fn drm_vram_mm_debugfs_init(minor: *mut drm_minor);

    pub fn drmm_vram_helper_init(
        dev: *mut drm_device,
        vram_base: u64,
        vram_size: usize,
    ) -> i32;

    pub fn drm_vram_helper_mode_valid(
        dev: *mut drm_device,
        mode: *const drm_display_mode,
    ) -> drm_mode_status;
}

// DRM_GEM_VRAM_PLANE_HELPER_FUNCS expands to:
// .prepare_fb = drm_gem_vram_plane_helper_prepare_fb,
// .cleanup_fb = drm_gem_vram_plane_helper_cleanup_fb

// DRM_GEM_VRAM_DRIVER expands to:
// .debugfs_init = drm_vram_mm_debugfs_init,
// .dumb_create = drm_gem_vram_driver_dumb_create,
// .dumb_map_offset = drm_gem_ttm_dumb_map_offset

#[repr(C)]
pub struct drm_vram_mm {
    pub vram_base: u64,
    pub vram_size: usize,
    pub bdev: ttm_device,
}

#[inline]
pub unsafe fn drm_vram_mm_of_bdev(bdev: *mut ttm_device) -> *mut drm_vram_mm {
    (bdev as *mut u8).sub(core::mem::offset_of!(drm_vram_mm, bdev)) as *mut drm_vram_mm
}

pub enum drm_mode_status {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
