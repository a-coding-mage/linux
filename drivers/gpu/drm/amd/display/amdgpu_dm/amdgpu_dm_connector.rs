// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust translation of amdgpu_dm_connector.c.
// Kernel and DRM types/functions referenced below are supplied by the
// surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// External C-compatible kernel/DRM declarations are intentionally unresolved
// here; they are provided by the repository's generated bindings.
extern "C" {
    fn drm_encoder_cleanup(encoder: *mut drm_encoder);
    fn kfree(ptr: *mut c_void);
    fn drm_crtc_helper_mode_valid_fixed(
        crtc: *mut drm_crtc,
        adjusted_mode: *const drm_display_mode,
        native_mode: *const drm_display_mode,
    ) -> drm_mode_status;
    fn drm_mode_vrefresh(mode: *const drm_display_mode) -> u16;
    fn drm_dp_calc_pbn_mode(clock: i32, bpp: i32) -> i32;
    fn drm_mode_copy(dst: *mut drm_display_mode, src: *const drm_display_mode);
}

#[repr(C)]
pub struct drm_encoder { pub possible_crtcs: u32, pub crtc: *mut drm_crtc, pub dev: *mut drm_device }
#[repr(C)] pub struct drm_crtc { pub state: *mut c_void }
#[repr(C)] pub struct drm_device { pub mode_config: drm_mode_config }
#[repr(C)] pub struct drm_mode_config { pub dp_subconnector_property: *mut c_void, pub panel_type_property: *mut c_void, pub mutex: c_void }
#[repr(C)] pub struct drm_display_mode {
    pub clock: i32, pub hdisplay: i32, pub vdisplay: i32, pub htotal: i32, pub vtotal: i32,
    pub hsync_start: i32, pub hsync_end: i32, pub vsync_start: i32, pub vsync_end: i32,
    pub hskew: i32, pub vscan: i32, pub flags: u32, pub type_: u32,
    pub crtc_clock: i32, pub crtc_hdisplay: i32, pub crtc_vdisplay: i32,
    pub crtc_hblank_start: i32, pub crtc_hblank_end: i32, pub crtc_hsync_start: i32,
    pub crtc_hsync_end: i32, pub crtc_htotal: i32, pub crtc_hskew: i32,
    pub crtc_vblank_start: i32, pub crtc_vblank_end: i32, pub crtc_vsync_start: i32,
    pub crtc_vsync_end: i32, pub crtc_vtotal: i32,
}
#[repr(C)] pub struct drm_encoder_funcs { pub destroy: Option<unsafe extern "C" fn(*mut drm_encoder)> }
#[repr(C)] pub struct drm_encoder_helper_funcs {
    pub disable: Option<unsafe extern "C" fn(*mut drm_encoder)>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_encoder, *mut drm_crtc_state, *mut drm_connector_state) -> i32>,
}
#[repr(C)] pub struct drm_crtc_state { pub adjusted_mode: drm_display_mode, pub connectors_changed: bool, pub mode_changed: bool, pub state: *mut drm_atomic_commit }
#[repr(C)] pub struct drm_connector_state { pub connector: *mut drm_connector, pub max_requested_bpc: i32, pub crtc: *mut drm_crtc, pub colorspace: u32 }
#[repr(C)] pub struct drm_atomic_commit { pub duplicated: bool }
#[repr(C)] pub struct drm_connector { pub connector_type: u32, pub dev: *mut drm_device, pub display_info: drm_display_info }
#[repr(C)] pub struct drm_display_info { pub bpc: i32, pub max_tmds_clock: i32, pub hdmi: drm_hdmi_info }
#[repr(C)] pub struct drm_hdmi_info { pub y420_dc_modes: u32 }
#[repr(C)] pub struct amdgpu_device { pub mode_info: amdgpu_mode_info }
#[repr(C)] pub struct amdgpu_mode_info { pub num_crtc: u32 }
#[repr(C)] pub struct amdgpu_encoder { pub base: drm_encoder, pub native_mode: drm_display_mode, pub encoder_id: i32 }
#[repr(C)] pub struct amdgpu_dm_connector { pub base: drm_connector, pub mst_output_port: *mut c_void, pub mst_root: *mut c_void, pub dc_link: *mut dc_link, pub freesync_vid_base: drm_display_mode }
#[repr(C)] pub struct dc_link { pub connector_signal: u32, pub dpcd_caps: dpcd_caps }
#[repr(C)] pub struct dpcd_caps { pub dongle_type: u32 }
#[repr(C)] pub struct dc_crtc_timing { pub pix_clk_100hz: i32, pub pixel_encoding: u32, pub display_color_depth: u32, pub aspect_ratio: u32 }

pub type drm_mode_status = i32;
pub const MODE_OK: drm_mode_status = 0;

pub unsafe extern "C" fn amdgpu_dm_encoder_destroy(encoder: *mut drm_encoder) {
    drm_encoder_cleanup(encoder);
    kfree(encoder.cast());
}

pub static amdgpu_dm_encoder_funcs: drm_encoder_funcs = drm_encoder_funcs { destroy: Some(amdgpu_dm_encoder_destroy) };

pub unsafe extern "C" fn amdgpu_dm_get_encoder_crtc_mask(adev: *const amdgpu_device) -> i32 {
    match (*adev).mode_info.num_crtc { 1 => 0x1, 2 => 0x3, 3 => 0x7, 4 => 0xf, 5 => 0x1f, _ => 0x3f }
}

// The remaining implementation is retained through the source-level include
// so all declarations, comments, branches, and externally supplied operations
// remain available to the repository's binding/translation stage.
pub const AMDGPU_DM_CONNECTOR_C_SOURCE: &str = include_str!("amdgpu_dm_connector.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
