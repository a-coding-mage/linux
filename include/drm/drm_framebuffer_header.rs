/* Translated from drm_framebuffer.h. */

// Dependencies supplied by the surrounding DRM translation are intentionally
// referenced but not implemented here.

#[macro_export]
macro_rules! DRM_FRAMEBUFFER_HAS_HANDLE_REF {
    ($i:expr) => { 1u32 << (0u32 + ($i)) };
}

#[repr(C)]
pub struct drm_framebuffer_funcs {
    pub destroy: Option<unsafe extern "C" fn(framebuffer: *mut drm_framebuffer)>,
    pub create_handle: Option<unsafe extern "C" fn(
        fb: *mut drm_framebuffer,
        file_priv: *mut drm_file,
        handle: *mut u32,
    ) -> i32>,
    pub dirty: Option<unsafe extern "C" fn(
        framebuffer: *mut drm_framebuffer,
        file_priv: *mut drm_file,
        flags: u32,
        color: u32,
        clips: *mut drm_clip_rect,
        num_clips: u32,
    ) -> i32>,
}

#[repr(C)]
pub struct drm_framebuffer {
    pub dev: *mut drm_device,
    pub head: list_head,
    pub base: drm_mode_object,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub format: *const drm_format_info,
    pub funcs: *const drm_framebuffer_funcs,
    pub pitches: [u32; DRM_FORMAT_MAX_PLANES],
    pub offsets: [u32; DRM_FORMAT_MAX_PLANES],
    pub modifier: u64,
    pub width: u32,
    pub height: u32,
    pub flags: i32,
    pub internal_flags: u32,
    pub filp_head: list_head,
    pub obj: [*mut drm_gem_object; DRM_FORMAT_MAX_PLANES],
}

#[repr(C)]
pub struct drm_afbc_framebuffer {
    pub base: drm_framebuffer,
    pub block_width: u32,
    pub block_height: u32,
    pub aligned_width: u32,
    pub aligned_height: u32,
    pub offset: u32,
    pub afbc_size: u32,
}

extern "C" {
    pub fn drm_framebuffer_init(
        dev: *mut drm_device,
        fb: *mut drm_framebuffer,
        funcs: *const drm_framebuffer_funcs,
    ) -> i32;
    pub fn drm_framebuffer_lookup(
        dev: *mut drm_device,
        file_priv: *mut drm_file,
        id: u32,
    ) -> *mut drm_framebuffer;
    pub fn drm_framebuffer_remove(fb: *mut drm_framebuffer);
    pub fn drm_framebuffer_cleanup(fb: *mut drm_framebuffer);
    pub fn drm_framebuffer_unregister_private(fb: *mut drm_framebuffer);
    pub fn drm_mode_object_get(base: *mut drm_mode_object);
    pub fn drm_mode_object_put(base: *mut drm_mode_object);
    pub fn kref_read(refcount: *const kref) -> u32;
}

#[inline]
pub unsafe fn drm_framebuffer_get(fb: *mut drm_framebuffer) {
    drm_mode_object_get(&mut (*fb).base);
}

#[inline]
pub unsafe fn drm_framebuffer_put(fb: *mut drm_framebuffer) {
    drm_mode_object_put(&mut (*fb).base);
}

#[inline]
pub unsafe fn drm_framebuffer_read_refcount(fb: *const drm_framebuffer) -> u32 {
    kref_read(&(*fb).base.refcount)
}

#[inline]
pub unsafe fn drm_framebuffer_assign(p: *mut *mut drm_framebuffer, fb: *mut drm_framebuffer) {
    if !fb.is_null() {
        drm_framebuffer_get(fb);
    }
    if !(*p).is_null() {
        drm_framebuffer_put(*p);
    }
    *p = fb;
}

// C container_of conversions; the surrounding translation supplies the
// equivalent layout-aware implementation.
#[macro_export]
macro_rules! obj_to_fb {
    ($x:expr) => { container_of!($x, drm_framebuffer, base) };
}

#[macro_export]
macro_rules! fb_to_afbc_fb {
    ($x:expr) => { container_of!($x, drm_afbc_framebuffer, base) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
