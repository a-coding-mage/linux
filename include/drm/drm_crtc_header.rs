/* Direct Rust translation of drm_crtc.h. C dependencies are supplied externally. */

pub unsafe fn U642I64(val: u64) -> i64 { core::mem::transmute::<u64, i64>(val) }
pub unsafe fn I642U64(val: i64) -> u64 { core::mem::transmute::<i64, u64>(val) }

pub enum drm_connector {}
pub enum drm_device {}
pub enum drm_framebuffer {}
pub enum drm_file {}
pub enum drm_printer {}
pub enum drm_self_refresh_data {}
pub enum device_node {}
pub enum edid {}
pub enum drm_pending_vblank_event {}
pub enum drm_plane {}
pub enum drm_bridge {}
pub enum drm_atomic_commit {}
pub enum drm_crtc_helper_funcs {}
pub enum drm_plane_helper_funcs {}
pub enum drm_property_blob {}
pub enum drm_crtc_commit {}
pub enum drm_property {}
pub enum drm_event {}
pub enum drm_modeset_acquire_ctx {}
pub enum drm_mode_object {}
pub enum drm_display_mode {}
pub enum drm_object_properties {}
pub enum drm_crtc_crc {}
pub enum list_head {}
pub enum dentry {}
pub enum drm_modeset_lock {}
pub enum ktime_t {}
pub type spinlock_t = core::ffi::c_void;
pub type drm_scaling_filter = u32;

#[repr(C)]
pub struct drm_crtc_state {
    pub crtc: *mut drm_crtc,
    pub enable: bool,
    pub active: bool,
    pub planes_changed: bool,
    pub mode_changed: bool,
    pub active_changed: bool,
    pub connectors_changed: bool,
    pub zpos_changed: bool,
    pub color_mgmt_changed: bool,
    pub no_vblank: bool,
    pub plane_mask: u32,
    pub connector_mask: u32,
    pub encoder_mask: u32,
    pub adjusted_mode: drm_display_mode,
    pub mode: drm_display_mode,
    pub mode_blob: *mut drm_property_blob,
    pub degamma_lut: *mut drm_property_blob,
    pub ctm: *mut drm_property_blob,
    pub gamma_lut: *mut drm_property_blob,
    pub background_color: u64,
    pub target_vblank: u32,
    pub async_flip: bool,
    pub vrr_enabled: bool,
    pub self_refresh_active: bool,
    pub scaling_filter: drm_scaling_filter,
    pub sharpness_strength: u8,
    pub event: *mut drm_pending_vblank_event,
    pub commit: *mut drm_crtc_commit,
    pub state: *mut drm_atomic_commit,
}

#[repr(C)]
pub struct drm_crtc_funcs {
    pub reset: Option<unsafe extern "C" fn(*mut drm_crtc)>,
    pub cursor_set: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_file, u32, u32, u32) -> i32>,
    pub cursor_set2: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_file, u32, u32, u32, i32, i32) -> i32>,
    pub cursor_move: Option<unsafe extern "C" fn(*mut drm_crtc, i32, i32) -> i32>,
    pub gamma_set: Option<unsafe extern "C" fn(*mut drm_crtc, *mut u16, *mut u16, *mut u16, u32, *mut drm_modeset_acquire_ctx) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut drm_crtc)>,
    pub set_config: Option<unsafe extern "C" fn(*mut drm_mode_set, *mut drm_modeset_acquire_ctx) -> i32>,
    pub page_flip: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_framebuffer, *mut drm_pending_vblank_event, u32, *mut drm_modeset_acquire_ctx) -> i32>,
    pub page_flip_target: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_framebuffer, *mut drm_pending_vblank_event, u32, u32, *mut drm_modeset_acquire_ctx) -> i32>,
    pub set_property: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_property, u64) -> i32>,
    pub atomic_create_state: Option<unsafe extern "C" fn(*mut drm_crtc) -> *mut drm_crtc_state>,
    pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_crtc) -> *mut drm_crtc_state>,
    pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_crtc_state)>,
    pub atomic_set_property: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_crtc_state, *mut drm_property, u64) -> i32>,
    pub atomic_get_property: Option<unsafe extern "C" fn(*mut drm_crtc, *const drm_crtc_state, *mut drm_property, *mut u64) -> i32>,
    pub late_register: Option<unsafe extern "C" fn(*mut drm_crtc) -> i32>,
    pub early_unregister: Option<unsafe extern "C" fn(*mut drm_crtc)>,
    pub set_crc_source: Option<unsafe extern "C" fn(*mut drm_crtc, *const core::ffi::c_char) -> i32>,
    pub verify_crc_source: Option<unsafe extern "C" fn(*mut drm_crtc, *const core::ffi::c_char, *mut usize) -> i32>,
    pub get_crc_sources: Option<unsafe extern "C" fn(*mut drm_crtc, *mut usize) -> *const *const core::ffi::c_char>,
    pub atomic_print_state: Option<unsafe extern "C" fn(*mut drm_printer, *const drm_crtc_state)>,
    pub get_vblank_counter: Option<unsafe extern "C" fn(*mut drm_crtc) -> u32>,
    pub enable_vblank: Option<unsafe extern "C" fn(*mut drm_crtc) -> i32>,
    pub disable_vblank: Option<unsafe extern "C" fn(*mut drm_crtc)>,
    pub get_vblank_timestamp: Option<unsafe extern "C" fn(*mut drm_crtc, *mut i32, *mut ktime_t, bool) -> bool>,
}

#[repr(C)]
pub struct drm_crtc {
    pub dev: *mut drm_device, pub port: *mut device_node, pub head: list_head,
    pub name: *mut core::ffi::c_char, pub mutex: drm_modeset_lock, pub base: drm_mode_object,
    pub primary: *mut drm_plane, pub cursor: *mut drm_plane, pub index: u32,
    pub cursor_x: i32, pub cursor_y: i32, pub enabled: bool, pub mode: drm_display_mode,
    pub hwmode: drm_display_mode, pub x: i32, pub y: i32,
    pub funcs: *const drm_crtc_funcs, pub gamma_size: u32, pub gamma_store: *mut u16,
    pub helper_private: *const drm_crtc_helper_funcs, pub properties: drm_object_properties,
    pub scaling_filter_property: *mut drm_property, pub sharpness_strength_property: *mut drm_property,
    pub state: *mut drm_crtc_state, pub commit_list: list_head, pub commit_lock: spinlock_t,
    pub debugfs_entry: *mut dentry, pub crc: drm_crtc_crc, pub fence_context: u32,
    pub fence_lock: spinlock_t, pub fence_seqno: usize, pub timeline_name: [core::ffi::c_char; 32],
    pub self_refresh_data: *mut drm_self_refresh_data,
}

#[repr(C)]
pub struct drm_mode_set {
    pub fb: *mut drm_framebuffer, pub crtc: *mut drm_crtc, pub mode: *mut drm_display_mode,
    pub x: u32, pub y: u32, pub connectors: *mut *mut drm_connector, pub num_connectors: usize,
}

extern "C" {
    pub fn drm_crtc_init_with_planes(dev: *mut drm_device, crtc: *mut drm_crtc, primary: *mut drm_plane, cursor: *mut drm_plane, funcs: *const drm_crtc_funcs, name: *const core::ffi::c_char, ...) -> i32;
    pub fn drmm_crtc_init_with_planes(dev: *mut drm_device, crtc: *mut drm_crtc, primary: *mut drm_plane, cursor: *mut drm_plane, funcs: *const drm_crtc_funcs, name: *const core::ffi::c_char, ...) -> i32;
    pub fn drm_crtc_cleanup(crtc: *mut drm_crtc);
    pub fn __drmm_crtc_alloc_with_planes(dev: *mut drm_device, size: usize, offset: usize, primary: *mut drm_plane, cursor: *mut drm_plane, funcs: *const drm_crtc_funcs, name: *const core::ffi::c_char, ...) -> *mut core::ffi::c_void;
    pub fn drm_mode_set_config_internal(set: *mut drm_mode_set) -> i32;
    pub fn drm_crtc_from_index(dev: *mut drm_device, idx: i32) -> *mut drm_crtc;
    pub fn drm_crtc_create_scaling_filter_property(crtc: *mut drm_crtc, supported_filters: u32) -> i32;
    pub fn drm_crtc_in_clone_mode(state: *mut drm_crtc_state) -> bool;
    pub fn drm_crtc_create_sharpness_strength_property(crtc: *mut drm_crtc) -> i32;
}

#[inline]
pub unsafe fn drm_crtc_index(crtc: *const drm_crtc) -> u32 { (*crtc).index }
#[inline]
pub unsafe fn drm_crtc_mask(crtc: *const drm_crtc) -> u32 { 1u32 << drm_crtc_index(crtc) }

// C container_of and list iteration macros are intentionally represented as external dependencies.
#[macro_export]
macro_rules! obj_to_crtc { ($x:expr) => { unsafe { &mut *((($x as *mut u8).sub(core::mem::offset_of!($crate::drm_crtc, base))) as *mut $crate::drm_crtc) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
