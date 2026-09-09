/* Rust translation of drm_modeset_helper_vtables.h. */

// Dependencies supplied by the DRM headers and other translation units.
#[repr(C)] pub struct drm_crtc { pub helper_private: *const drm_crtc_helper_funcs }
#[repr(C)] pub struct drm_encoder { pub helper_private: *const drm_encoder_helper_funcs }
#[repr(C)] pub struct drm_connector { pub helper_private: *const drm_connector_helper_funcs }
#[repr(C)] pub struct drm_plane { pub helper_private: *const drm_plane_helper_funcs }
#[repr(C)] pub struct drm_scanout_buffer;
#[repr(C)] pub struct drm_writeback_connector;
#[repr(C)] pub struct drm_writeback_job;
#[repr(C)] pub struct drm_display_mode;
#[repr(C)] pub struct drm_framebuffer;
#[repr(C)] pub struct drm_atomic_commit;
#[repr(C)] pub struct drm_crtc_state;
#[repr(C)] pub struct drm_connector_state;
#[repr(C)] pub struct drm_plane_state;
#[repr(C)] pub struct drm_modeset_acquire_ctx;
#[repr(C)] pub struct ktime_t;

#[repr(C)] pub enum drm_mode_status { __DRM_MODE_STATUS = 0 }
#[repr(C)] pub enum drm_connector_status { __DRM_CONNECTOR_STATUS = 0 }

pub type CBool = bool;
type CrtcDpms = unsafe extern "C" fn(*mut drm_crtc, i32);
type CrtcVoid = unsafe extern "C" fn(*mut drm_crtc);

#[repr(C)]
pub struct drm_crtc_helper_funcs {
    pub dpms: Option<CrtcDpms>,
    pub prepare: Option<CrtcVoid>,
    pub commit: Option<CrtcVoid>,
    pub mode_valid: Option<unsafe extern "C" fn(*mut drm_crtc, *const drm_display_mode) -> drm_mode_status>,
    pub mode_fixup: Option<unsafe extern "C" fn(*mut drm_crtc, *const drm_display_mode, *mut drm_display_mode) -> bool>,
    pub mode_set: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_display_mode, *mut drm_display_mode, i32, i32, *mut drm_framebuffer) -> i32>,
    pub mode_set_nofb: Option<CrtcVoid>,
    pub mode_set_base: Option<unsafe extern "C" fn(*mut drm_crtc, i32, i32, *mut drm_framebuffer) -> i32>,
    pub disable: Option<CrtcVoid>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_atomic_commit) -> i32>,
    pub atomic_begin: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_atomic_commit)>,
    pub atomic_flush: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_atomic_commit)>,
    pub atomic_enable: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_atomic_commit)>,
    pub atomic_disable: Option<unsafe extern "C" fn(*mut drm_crtc, *mut drm_atomic_commit)>,
    pub get_scanout_position: Option<unsafe extern "C" fn(*mut drm_crtc, bool, *mut i32, *mut i32, *mut ktime_t, *mut ktime_t, *const drm_display_mode) -> bool>,
    pub handle_vblank_timeout: Option<CrtcVoidBool>,
}
type CrtcVoidBool = unsafe extern "C" fn(*mut drm_crtc) -> bool;

pub unsafe fn drm_crtc_helper_add(crtc: *mut drm_crtc, funcs: *const drm_crtc_helper_funcs) { (*crtc).helper_private = funcs; }

#[repr(C)]
pub struct drm_encoder_helper_funcs {
    pub dpms: Option<unsafe extern "C" fn(*mut drm_encoder, i32)>,
    pub mode_valid: Option<unsafe extern "C" fn(*mut drm_encoder, *const drm_display_mode) -> drm_mode_status>,
    pub mode_fixup: Option<unsafe extern "C" fn(*mut drm_encoder, *const drm_display_mode, *mut drm_display_mode) -> bool>,
    pub prepare: Option<unsafe extern "C" fn(*mut drm_encoder)>,
    pub commit: Option<unsafe extern "C" fn(*mut drm_encoder)>,
    pub mode_set: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_display_mode, *mut drm_display_mode)>,
    pub atomic_mode_set: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_crtc_state, *mut drm_connector_state)>,
    pub detect: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_connector) -> drm_connector_status>,
    pub atomic_disable: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_atomic_commit)>,
    pub atomic_enable: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_atomic_commit)>,
    pub disable: Option<unsafe extern "C" fn(*mut drm_encoder)>,
    pub enable: Option<unsafe extern "C" fn(*mut drm_encoder)>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_crtc_state, *mut drm_connector_state) -> i32>,
}
pub unsafe fn drm_encoder_helper_add(encoder: *mut drm_encoder, funcs: *const drm_encoder_helper_funcs) { (*encoder).helper_private = funcs; }

#[repr(C)]
pub struct drm_connector_helper_funcs {
    pub get_modes: Option<unsafe extern "C" fn(*mut drm_connector) -> i32>,
    pub detect_ctx: Option<unsafe extern "C" fn(*mut drm_connector, *mut drm_modeset_acquire_ctx, bool) -> i32>,
    pub mode_valid: Option<unsafe extern "C" fn(*mut drm_connector, *const drm_display_mode) -> drm_mode_status>,
    pub mode_valid_ctx: Option<unsafe extern "C" fn(*mut drm_connector, *const drm_display_mode, *mut drm_modeset_acquire_ctx, *mut drm_mode_status) -> i32>,
    pub best_encoder: Option<unsafe extern "C" fn(*mut drm_connector) -> *mut drm_encoder>,
    pub atomic_best_encoder: Option<unsafe extern "C" fn(*mut drm_connector, *mut drm_atomic_commit) -> *mut drm_encoder>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_connector, *mut drm_atomic_commit) -> i32>,
    pub atomic_commit: Option<unsafe extern "C" fn(*mut drm_connector, *mut drm_atomic_commit)>,
    pub prepare_writeback_job: Option<unsafe extern "C" fn(*mut drm_writeback_connector, *mut drm_writeback_job) -> i32>,
    pub cleanup_writeback_job: Option<unsafe extern "C" fn(*mut drm_writeback_connector, *mut drm_writeback_job)>,
    pub enable_hpd: Option<unsafe extern "C" fn(*mut drm_connector)>,
    pub disable_hpd: Option<unsafe extern "C" fn(*mut drm_connector)>,
}
pub unsafe fn drm_connector_helper_add(connector: *mut drm_connector, funcs: *const drm_connector_helper_funcs) { (*connector).helper_private = funcs; }

#[repr(C)]
pub struct drm_plane_helper_funcs {
    pub prepare_fb: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state) -> i32>,
    pub cleanup_fb: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state)>,
    pub begin_fb_access: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state) -> i32>,
    pub end_fb_access: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_plane_state)>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit) -> i32>,
    pub atomic_update: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit)>,
    pub atomic_enable: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit)>,
    pub atomic_disable: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit)>,
    pub atomic_async_check: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit, bool) -> i32>,
    pub atomic_async_update: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_atomic_commit)>,
    pub get_scanout_buffer: Option<unsafe extern "C" fn(*mut drm_plane, *mut drm_scanout_buffer) -> i32>,
    pub panic_flush: Option<unsafe extern "C" fn(*mut drm_plane)>,
}
pub unsafe fn drm_plane_helper_add(plane: *mut drm_plane, funcs: *const drm_plane_helper_funcs) { (*plane).helper_private = funcs; }

#[repr(C)]
pub struct drm_mode_config_helper_funcs {
    pub atomic_commit_tail: Option<unsafe extern "C" fn(*mut drm_atomic_commit)>,
    pub atomic_commit_setup: Option<unsafe extern "C" fn(*mut drm_atomic_commit) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
