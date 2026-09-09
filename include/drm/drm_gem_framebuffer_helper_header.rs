// Translated from drm_gem_framebuffer_helper.h.
// Dependencies supplied by the surrounding DRM and DMA-buf interfaces are
// intentionally referenced but not implemented here.

#[repr(C)]
pub struct drm_afbc_framebuffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_fb_helper_surface_size {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_format_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_framebuffer_funcs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_gem_object {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_mode_fb_cmd2 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iosys_map {
    _private: [u8; 0],
}

// enum dma_data_direction is provided by the DMA-buf interface.
#[repr(C)]
pub enum dma_data_direction {}

pub const AFBC_VENDOR_AND_TYPE_MASK: u64 = 0xfff0_0000_0000_0000;

extern "C" {
    pub fn drm_gem_fb_get_obj(
        fb: *mut drm_framebuffer,
        plane: u32,
    ) -> *mut drm_gem_object;
    pub fn drm_gem_fb_destroy(fb: *mut drm_framebuffer);
    pub fn drm_gem_fb_create_handle(
        fb: *mut drm_framebuffer,
        file: *mut drm_file,
        handle: *mut u32,
    ) -> i32;

    pub fn drm_gem_fb_init_with_funcs(
        dev: *mut drm_device,
        fb: *mut drm_framebuffer,
        file: *mut drm_file,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
        funcs: *const drm_framebuffer_funcs,
    ) -> i32;
    pub fn drm_gem_fb_create_with_funcs(
        dev: *mut drm_device,
        file: *mut drm_file,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
        funcs: *const drm_framebuffer_funcs,
    ) -> *mut drm_framebuffer;
    pub fn drm_gem_fb_create(
        dev: *mut drm_device,
        file: *mut drm_file,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
    ) -> *mut drm_framebuffer;
    pub fn drm_gem_fb_create_with_dirty(
        dev: *mut drm_device,
        file: *mut drm_file,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
    ) -> *mut drm_framebuffer;

    pub fn drm_gem_fb_vmap(
        fb: *mut drm_framebuffer,
        map: *mut iosys_map,
        data: *mut iosys_map,
    ) -> i32;
    pub fn drm_gem_fb_vunmap(fb: *mut drm_framebuffer, map: *mut iosys_map);
    pub fn drm_gem_fb_begin_cpu_access(
        fb: *mut drm_framebuffer,
        dir: dma_data_direction,
    ) -> i32;
    pub fn drm_gem_fb_end_cpu_access(fb: *mut drm_framebuffer, dir: dma_data_direction);

    // DRM_FORMAT_MOD_ARM_AFBC(0), supplied by the DRM format interface.
    pub fn drm_format_mod_arm_afbc(value: u64) -> u64;

    pub fn drm_gem_fb_afbc_init(
        dev: *mut drm_device,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
        afbc_fb: *mut drm_afbc_framebuffer,
    ) -> i32;
}

pub unsafe fn drm_is_afbc(modifier: u64) -> bool {
    (modifier & AFBC_VENDOR_AND_TYPE_MASK) == drm_format_mod_arm_afbc(0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
