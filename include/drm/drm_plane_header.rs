/* Translated from drm_plane.h. C header dependencies are supplied externally. */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct drm_crtc;
#[repr(C)] pub struct drm_plane_size_hint;
#[repr(C)] pub struct drm_printer;
#[repr(C)] pub struct drm_modeset_acquire_ctx;
#[repr(C)] pub struct drm_framebuffer;
#[repr(C)] pub struct dma_fence;
#[repr(C)] pub struct drm_property_blob;
#[repr(C)] pub struct drm_colorop;
#[repr(C)] pub struct drm_crtc_commit;
#[repr(C)] pub struct drm_atomic_commit;
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_property;
#[repr(C)] pub struct drm_file;
#[repr(C)] pub struct drm_mode_object;
#[repr(C)] pub struct drm_plane_helper_funcs;
#[repr(C)] pub struct drm_object_properties;
#[repr(C)] pub struct drm_modeset_lock;
#[repr(C)] pub struct drm_mode_rect;
#[repr(C)] pub struct drm_prop_enum_list;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct kmsg_dumper;

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_rect { pub x1: i32, pub y1: i32, pub x2: i32, pub y2: i32 }

#[repr(C)] #[derive(Copy, Clone)] pub enum drm_scaling_filter {
    DRM_SCALING_FILTER_DEFAULT,
    DRM_SCALING_FILTER_NEAREST_NEIGHBOR,
}

#[repr(C)] #[derive(Copy, Clone)] pub enum drm_color_encoding { DRM_COLOR_ENCODING_UNSPECIFIED = 0 }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_color_range { DRM_COLOR_RANGE_UNSPECIFIED = 0 }

#[repr(C)]
pub struct drm_plane_state {
    pub plane: *mut drm_plane,
    pub crtc: *mut drm_crtc,
    pub fb: *mut drm_framebuffer,
    pub fence: *mut dma_fence,
    pub crtc_x: i32, pub crtc_y: i32,
    pub crtc_w: u32, pub crtc_h: u32,
    pub src_x: u32, pub src_y: u32, pub src_h: u32, pub src_w: u32,
    pub hotspot_x: i32, pub hotspot_y: i32,
    pub alpha: u16,
    pub pixel_blend_mode: u16,
    pub rotation: u32,
    pub zpos: u32,
    pub normalized_zpos: u32,
    pub color_encoding: drm_color_encoding,
    pub color_range: drm_color_range,
    pub fb_damage_clips: *mut drm_property_blob,
    pub ignore_damage_clips: bool,
    pub src: drm_rect, pub dst: drm_rect,
    pub visible: bool,
    pub scaling_filter: drm_scaling_filter,
    pub color_pipeline: *mut drm_colorop,
    pub commit: *mut drm_crtc_commit,
    pub state: *mut drm_atomic_commit,
    pub color_mgmt_changed: bool,
}

#[inline]
pub unsafe fn drm_plane_state_src(state: *const drm_plane_state) -> drm_rect {
    drm_rect { x1: (*state).src_x as i32, y1: (*state).src_y as i32,
        x2: ((*state).src_x.wrapping_add((*state).src_w)) as i32,
        y2: ((*state).src_y.wrapping_add((*state).src_h)) as i32 }
}

#[inline]
pub unsafe fn drm_plane_state_dest(state: *const drm_plane_state) -> drm_rect {
    drm_rect { x1: (*state).crtc_x, y1: (*state).crtc_y,
        x2: (*state).crtc_x.wrapping_add((*state).crtc_w as i32),
        y2: (*state).crtc_y.wrapping_add((*state).crtc_h as i32) }
}

#[repr(C)]
pub struct drm_plane_funcs {
    pub update_plane: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_crtc, *mut drm_framebuffer, i32, i32, u32, u32, u32, u32, u32, u32, *mut drm_modeset_acquire_ctx) -> i32>,
    pub disable_plane: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_modeset_acquire_ctx) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut drm_plane)>,
    pub reset: Option<unsafe extern "C" fn(*mut drm_plane)>,
    pub set_property: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_property, u64) -> i32>,
    pub atomic_create_state: Option<unsafe extern "C" fn(*mut drm_plane) -> *mut drm_plane_state>,
    pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_plane) -> *mut drm_plane_state>,
    pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state)>,
    pub atomic_set_property: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state, *mut drm_property, u64) -> i32>,
    pub atomic_get_property: Option<unsafe extern "C" fn(*mut drm_plane, *const drm_plane_state, *mut drm_property, *mut u64) -> i32>,
    pub late_register: Option<unsafe extern "C" fn(*mut drm_plane) -> i32>,
    pub early_unregister: Option<unsafe extern "C" fn(*mut drm_plane)>,
    pub atomic_print_state: Option<unsafe extern "C" fn(*mut drm_printer, *const drm_plane_state)>,
    pub format_mod_supported: Option<unsafe extern "C" fn(*mut drm_plane, u32, u64) -> bool>,
    pub format_mod_supported_async: Option<unsafe extern "C" fn(*mut drm_plane, u32, u64) -> bool>,
}

#[repr(C)] #[derive(Copy, Clone)] pub enum drm_plane_type { DRM_PLANE_TYPE_OVERLAY, DRM_PLANE_TYPE_PRIMARY, DRM_PLANE_TYPE_CURSOR }

#[repr(C)]
pub struct drm_plane {
    pub dev: *mut drm_device, pub head: list_head, pub name: *mut c_char,
    pub mutex: drm_modeset_lock, pub base: drm_mode_object, pub possible_crtcs: u32,
    pub format_types: *mut u32, pub format_count: u32, pub format_default: bool,
    pub modifiers: *mut u64, pub modifier_count: u32, pub crtc: *mut drm_crtc,
    pub fb: *mut drm_framebuffer, pub old_fb: *mut drm_framebuffer,
    pub funcs: *const drm_plane_funcs, pub properties: drm_object_properties,
    pub type_: drm_plane_type, pub index: u32, pub helper_private: *const drm_plane_helper_funcs,
    pub state: *mut drm_plane_state, pub alpha_property: *mut drm_property,
    pub zpos_property: *mut drm_property, pub rotation_property: *mut drm_property,
    pub blend_mode_property: *mut drm_property, pub color_encoding_property: *mut drm_property,
    pub color_range_property: *mut drm_property, pub color_pipeline_property: *mut drm_property,
    pub scaling_filter_property: *mut drm_property, pub hotspot_x_property: *mut drm_property,
    pub hotspot_y_property: *mut drm_property, pub kmsg_panic: kmsg_dumper,
}

extern "C" {
    pub fn drm_universal_plane_init(dev: *mut drm_device, plane: *mut drm_plane, possible_crtcs: u32, funcs: *const drm_plane_funcs, formats: *const u32, format_count: u32, format_modifiers: *const u64, type_: drm_plane_type, name: *const c_char, ...) -> i32;
    pub fn drm_plane_cleanup(plane: *mut drm_plane);
    pub fn __drmm_universal_plane_alloc(dev: *mut drm_device, size: usize, offset: usize, possible_crtcs: u32, funcs: *const drm_plane_funcs, formats: *const u32, format_count: u32, format_modifiers: *const u64, plane_type: drm_plane_type, name: *const c_char, ...) -> *mut c_void;
    pub fn __drm_universal_plane_alloc(dev: *mut drm_device, size: usize, offset: usize, possible_crtcs: u32, funcs: *const drm_plane_funcs, formats: *const u32, format_count: u32, format_modifiers: *const u64, plane_type: drm_plane_type, name: *const c_char, ...) -> *mut c_void;
    pub fn drm_plane_from_index(dev: *mut drm_device, idx: i32) -> *mut drm_plane;
    pub fn drm_plane_force_disable(plane: *mut drm_plane);
    pub fn drm_mode_plane_set_obj_prop(plane: *mut drm_plane, property: *mut drm_property, value: u64) -> i32;
    pub fn drm_plane_has_format(plane: *mut drm_plane, format: u32, modifier: u64) -> bool;
    pub fn drm_any_plane_has_format(dev: *mut drm_device, format: u32, modifier: u64) -> bool;
    pub fn drm_plane_enable_fb_damage_clips(plane: *mut drm_plane);
    pub fn drm_plane_get_damage_clips_count(state: *const drm_plane_state) -> u32;
    pub fn drm_plane_get_damage_clips(state: *const drm_plane_state) -> *mut drm_mode_rect;
    pub fn drm_plane_create_scaling_filter_property(plane: *mut drm_plane, supported_filters: u32) -> i32;
    pub fn drm_plane_add_size_hints_property(plane: *mut drm_plane, hints: *const drm_plane_size_hint, num_hints: i32) -> i32;
    pub fn drm_plane_create_color_pipeline_property(plane: *mut drm_plane, pipelines: *const drm_prop_enum_list, num_pipelines: i32) -> i32;
    pub fn drm_mode_object_find(dev: *mut drm_device, file_priv: *mut drm_file, id: u32, type_: u32) -> *mut drm_mode_object;
}

#[inline] pub unsafe fn drm_plane_index(plane: *const drm_plane) -> u32 { (*plane).index }
#[inline] pub unsafe fn drm_plane_mask(plane: *const drm_plane) -> u32 { 1u32.wrapping_shl(drm_plane_index(plane)) }

/* C macros obj_to_plane, allocation helpers, and list iteration helpers retain their source-level intent here. */
#[macro_export] macro_rules! obj_to_plane { ($x:expr) => { ($x as *mut $crate::drm_plane_header::drm_plane) }; }
#[macro_export] macro_rules! drmm_universal_plane_alloc { ($dev:expr, $type:ty, $member:ident, $possible_crtcs:expr, $funcs:expr, $formats:expr, $format_count:expr, $format_modifiers:expr, $plane_type:expr, $name:expr $(, $args:expr)*) => { unsafe { __drmm_universal_plane_alloc($dev, core::mem::size_of::<$type>(), 0, $possible_crtcs, $funcs, $formats, $format_count, $format_modifiers, $plane_type, $name $(, $args)*) as *mut $type } }; }
#[macro_export] macro_rules! drm_universal_plane_alloc { ($dev:expr, $type:ty, $member:ident, $possible_crtcs:expr, $funcs:expr, $formats:expr, $format_count:expr, $format_modifiers:expr, $plane_type:expr, $name:expr $(, $args:expr)*) => { unsafe { __drm_universal_plane_alloc($dev, core::mem::size_of::<$type>(), 0, $possible_crtcs, $funcs, $formats, $format_count, $format_modifiers, $plane_type, $name $(, $args)*) as *mut $type } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
