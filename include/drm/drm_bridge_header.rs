/* Direct Rust translation of drm_bridge.h. External kernel/DRM types are supplied elsewhere. */

use core::ffi::c_void;

pub enum cec_msg {}
pub enum device_node {}
pub enum drm_connector {}
pub enum drm_display_info {}
pub enum drm_display_mode {}
pub enum drm_edid {}
pub enum drm_encoder {}
pub enum drm_minor {}
pub enum drm_panel {}
pub enum hdmi_codec_daifmt {}
pub enum hdmi_codec_params {}
pub enum i2c_adapter {}
pub enum drm_atomic_commit {}
pub enum drm_bridge_state {}
pub enum drm_crtc_state {}
pub enum drm_connector_state {}
pub enum dentry {}
pub enum drm_private_obj {}
pub enum drm_device {}
pub enum list_head {}
pub enum kref {}
pub enum mutex {}

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_bridge_attach_flags { DRM_BRIDGE_ATTACH_NO_CONNECTOR = 1 << 0 }

#[repr(C)]
pub struct drm_bridge_funcs {
    pub attach: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_encoder, drm_bridge_attach_flags) -> core::ffi::c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut drm_bridge)>,
    pub detach: Option<unsafe extern "C" fn(*mut drm_bridge)>,
    pub mode_valid: Option<unsafe extern "C" fn(*mut drm_bridge, *const drm_display_info, *const drm_display_mode) -> drm_mode_status>,
    pub mode_fixup: Option<unsafe extern "C" fn(*mut drm_bridge, *const drm_display_mode, *mut drm_display_mode) -> bool>,
    pub mode_set: Option<unsafe extern "C" fn(*mut drm_bridge, *const drm_display_mode, *const drm_display_mode)>,
    pub atomic_pre_enable: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_atomic_commit)>,
    pub atomic_enable: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_atomic_commit)>,
    pub atomic_disable: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_atomic_commit)>,
    pub atomic_post_disable: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_atomic_commit)>,
    pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_bridge) -> *mut drm_bridge_state>,
    pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_bridge_state)>,
    pub atomic_get_output_bus_fmts: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_bridge_state, *mut drm_crtc_state, *mut drm_connector_state, *mut u32) -> *mut u32>,
    pub atomic_get_input_bus_fmts: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_bridge_state, *mut drm_crtc_state, *mut drm_connector_state, u32, *mut u32) -> *mut u32>,
    pub atomic_check: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_bridge_state, *mut drm_crtc_state, *mut drm_connector_state) -> core::ffi::c_int>,
    pub atomic_create_state: Option<unsafe extern "C" fn(*mut drm_bridge) -> *mut drm_bridge_state>,
    pub detect: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> drm_connector_status>,
    pub get_modes: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> core::ffi::c_int>,
    pub edid_read: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> *const drm_edid>,
    pub hpd_notify: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector, drm_connector_status)>,
    pub hpd_enable: Option<unsafe extern "C" fn(*mut drm_bridge)>,
    pub hpd_disable: Option<unsafe extern "C" fn(*mut drm_bridge)>,
    pub hdmi_tmds_char_rate_valid: Option<unsafe extern "C" fn(*const drm_bridge, *const drm_display_mode, u64) -> drm_mode_status>,
    pub hdmi_clear_avi_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge) -> core::ffi::c_int>,
    pub hdmi_write_avi_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge, *const u8, size_t) -> core::ffi::c_int>,
    pub hdmi_clear_hdmi_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge) -> core::ffi::c_int>,
    pub hdmi_write_hdmi_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge, *const u8, size_t) -> core::ffi::c_int>,
    pub hdmi_clear_hdr_drm_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge) -> core::ffi::c_int>,
    pub hdmi_write_hdr_drm_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge, *const u8, size_t) -> core::ffi::c_int>,
    pub hdmi_clear_spd_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge) -> core::ffi::c_int>,
    pub hdmi_write_spd_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge, *const u8, size_t) -> core::ffi::c_int>,
    pub hdmi_clear_audio_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge) -> core::ffi::c_int>,
    pub hdmi_write_audio_infoframe: Option<unsafe extern "C" fn(*mut drm_bridge, *const u8, size_t) -> core::ffi::c_int>,
    pub hdmi_audio_startup: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> core::ffi::c_int>,
    pub hdmi_audio_prepare: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector, *mut hdmi_codec_daifmt, *mut hdmi_codec_params) -> core::ffi::c_int>,
    pub hdmi_audio_shutdown: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector)>,
    pub hdmi_audio_mute_stream: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector, bool, core::ffi::c_int) -> core::ffi::c_int>,
    pub hdmi_cec_init: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> core::ffi::c_int>,
    pub hdmi_cec_enable: Option<unsafe extern "C" fn(*mut drm_bridge, bool) -> core::ffi::c_int>,
    pub hdmi_cec_log_addr: Option<unsafe extern "C" fn(*mut drm_bridge, u8) -> core::ffi::c_int>,
    pub hdmi_cec_transmit: Option<unsafe extern "C" fn(*mut drm_bridge, u8, u32, *mut cec_msg) -> core::ffi::c_int>,
    pub dp_audio_startup: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector) -> core::ffi::c_int>,
    pub dp_audio_prepare: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector, *mut hdmi_codec_daifmt, *mut hdmi_codec_params) -> core::ffi::c_int>,
    pub dp_audio_shutdown: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector)>,
    pub dp_audio_mute_stream: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_connector, bool, core::ffi::c_int) -> core::ffi::c_int>,
    pub debugfs_init: Option<unsafe extern "C" fn(*mut drm_bridge, *mut dentry)>,
}

#[repr(C)]
pub struct drm_bridge_timings { pub input_bus_flags: u32, pub setup_time_ps: u32, pub hold_time_ps: u32, pub dual_link: bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_bridge_ops {
    DRM_BRIDGE_OP_DETECT = 1 << 0, DRM_BRIDGE_OP_EDID = 1 << 1, DRM_BRIDGE_OP_HPD = 1 << 2,
    DRM_BRIDGE_OP_MODES = 1 << 3, DRM_BRIDGE_OP_HDMI = 1 << 4, DRM_BRIDGE_OP_HDMI_AUDIO = 1 << 5,
    DRM_BRIDGE_OP_DP_AUDIO = 1 << 6, DRM_BRIDGE_OP_HDMI_CEC_NOTIFIER = 1 << 7,
    DRM_BRIDGE_OP_HDMI_CEC_ADAPTER = 1 << 8, DRM_BRIDGE_OP_HDMI_HDR_DRM_INFOFRAME = 1 << 9,
    DRM_BRIDGE_OP_HDMI_SPD_INFOFRAME = 1 << 10,
}

#[repr(C)]
pub struct drm_bridge {
    pub base: drm_private_obj, pub dev: *mut drm_device, pub encoder: *mut drm_encoder,
    pub chain_node: list_head, pub of_node: *mut device_node, pub list: list_head,
    pub timings: *const drm_bridge_timings, pub funcs: *const drm_bridge_funcs, pub container: *mut c_void,
    pub refcount: kref, pub unplugged: bool, pub driver_private: *mut c_void, pub ops: drm_bridge_ops,
    pub type_: core::ffi::c_int, pub interlace_allowed: bool, pub ycbcr_420_allowed: bool,
    pub pre_enable_prev_first: bool, pub support_hdcp: bool, pub ddc: *mut i2c_adapter,
    pub vendor: *const core::ffi::c_char, pub product: *const core::ffi::c_char,
    pub supported_formats: core::ffi::c_uint, pub max_bpc: core::ffi::c_uint,
    pub hdmi_cec_dev: *mut c_void, pub hdmi_audio_dev: *mut c_void,
    pub hdmi_audio_max_i2s_playback_channels: core::ffi::c_int, pub hdmi_audio_i2s_formats: u64,
    pub hdmi_audio_spdif_playback: core::ffi::c_uint, pub hdmi_audio_dai_port: core::ffi::c_int,
    pub hdmi_cec_adapter_name: *const core::ffi::c_char, pub hdmi_cec_available_las: u8,
    pub hpd_mutex: mutex, pub hpd_state_mutex: mutex,
    pub hpd_cb: Option<unsafe extern "C" fn(*mut c_void, drm_connector_status)>, pub hpd_data: *mut c_void,
    pub next_bridge: *mut drm_bridge,
}

pub type drm_mode_status = core::ffi::c_int;
pub type drm_connector_status = core::ffi::c_int;

extern "C" {
    pub fn drm_bridge_enter(bridge: *mut drm_bridge, idx: *mut core::ffi::c_int) -> bool;
    pub fn drm_bridge_exit(idx: core::ffi::c_int);
    pub fn drm_bridge_unplug(bridge: *mut drm_bridge);
    pub fn drm_bridge_get(bridge: *mut drm_bridge) -> *mut drm_bridge;
    pub fn drm_bridge_put(bridge: *mut drm_bridge);
    pub fn drm_bridge_clear_and_put(bridge_pp: *mut *mut drm_bridge);
    pub fn __devm_drm_bridge_alloc(dev: *mut c_void, size: size_t, offset: size_t, funcs: *const drm_bridge_funcs) -> *mut c_void;
    pub fn drm_bridge_add(bridge: *mut drm_bridge);
    pub fn devm_drm_bridge_add(dev: *mut c_void, bridge: *mut drm_bridge) -> core::ffi::c_int;
    pub fn drm_bridge_remove(bridge: *mut drm_bridge);
    pub fn drm_bridge_attach(encoder: *mut drm_encoder, bridge: *mut drm_bridge, previous: *mut drm_bridge, flags: drm_bridge_attach_flags) -> core::ffi::c_int;
    pub fn of_drm_find_and_get_bridge(np: *mut device_node) -> *mut drm_bridge;
    pub fn of_drm_find_bridge(np: *mut device_node) -> *mut drm_bridge;
    pub fn of_drm_get_bridge_by_endpoint(np: *const device_node, port: core::ffi::c_int, endpoint: core::ffi::c_int) -> *mut drm_bridge;
    pub fn drm_bridge_chain_mode_valid(bridge: *mut drm_bridge, info: *const drm_display_info, mode: *const drm_display_mode) -> drm_mode_status;
    pub fn drm_bridge_chain_mode_set(bridge: *mut drm_bridge, mode: *const drm_display_mode, adjusted_mode: *const drm_display_mode);
    pub fn drm_atomic_bridge_chain_check(bridge: *mut drm_bridge, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state) -> core::ffi::c_int;
    pub fn drm_atomic_bridge_chain_disable(bridge: *mut drm_bridge, state: *mut drm_atomic_commit);
    pub fn drm_atomic_bridge_chain_post_disable(bridge: *mut drm_bridge, state: *mut drm_atomic_commit);
    pub fn drm_atomic_bridge_chain_pre_enable(bridge: *mut drm_bridge, state: *mut drm_atomic_commit);
    pub fn drm_atomic_bridge_chain_enable(bridge: *mut drm_bridge, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_bridge_propagate_bus_fmt(bridge: *mut drm_bridge, bridge_state: *mut drm_bridge_state, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state, output_fmt: u32, num_input_fmts: *mut core::ffi::c_uint) -> *mut u32;
    pub fn drm_bridge_detect(bridge: *mut drm_bridge, connector: *mut drm_connector) -> drm_connector_status;
    pub fn drm_bridge_get_modes(bridge: *mut drm_bridge, connector: *mut drm_connector) -> core::ffi::c_int;
    pub fn drm_bridge_edid_read(bridge: *mut drm_bridge, connector: *mut drm_connector) -> *const drm_edid;
    pub fn drm_bridge_hpd_enable(bridge: *mut drm_bridge, cb: Option<unsafe extern "C" fn(*mut c_void, drm_connector_status)>, data: *mut c_void);
    pub fn drm_bridge_hpd_disable(bridge: *mut drm_bridge);
    pub fn drm_bridge_hpd_notify(bridge: *mut drm_bridge, status: drm_connector_status);
    pub fn drm_bridge_is_panel(bridge: *const drm_bridge) -> bool;
    pub fn drm_panel_bridge_add(panel: *mut drm_panel) -> *mut drm_bridge;
    pub fn drm_panel_bridge_add_typed(panel: *mut drm_panel, connector_type: u32) -> *mut drm_bridge;
    pub fn drm_panel_bridge_remove(bridge: *mut drm_bridge);
    pub fn drm_panel_bridge_set_orientation(connector: *mut drm_connector, bridge: *mut drm_bridge) -> core::ffi::c_int;
    pub fn devm_drm_panel_bridge_add(dev: *mut c_void, panel: *mut drm_panel) -> *mut drm_bridge;
    pub fn devm_drm_panel_bridge_add_typed(dev: *mut c_void, panel: *mut drm_panel, connector_type: u32) -> *mut drm_bridge;
    pub fn drmm_panel_bridge_add(drm: *mut drm_device, panel: *mut drm_panel) -> *mut drm_bridge;
    pub fn drm_panel_bridge_connector(bridge: *mut drm_bridge) -> *mut drm_connector;
    pub fn devm_drm_of_get_bridge(dev: *mut c_void, node: *mut device_node, port: u32, endpoint: u32) -> *mut drm_bridge;
    pub fn drmm_of_get_bridge(drm: *mut drm_device, node: *mut device_node, port: u32, endpoint: u32) -> *mut drm_bridge;
    pub fn devm_drm_put_bridge(dev: *mut c_void, bridge: *mut drm_bridge);
    pub fn drm_bridge_debugfs_params(root: *mut dentry);
    pub fn drm_bridge_debugfs_encoder_params(root: *mut dentry, encoder: *mut drm_encoder);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
