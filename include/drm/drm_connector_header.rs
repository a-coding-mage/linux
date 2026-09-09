/* Direct Rust translation of drm_connector.h. External kernel types are intentionally unresolved. */

// Includes and build-time macros from the C header are represented by external dependencies.

#[repr(C)]
pub struct drm_connector_helper_funcs;
#[repr(C)] pub struct drm_modeset_acquire_ctx;
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_crtc;
#[repr(C)] pub struct drm_display_mode;
#[repr(C)] pub struct drm_encoder;
#[repr(C)] pub struct drm_panel;
#[repr(C)] pub struct drm_property;
#[repr(C)] pub struct drm_property_blob;
#[repr(C)] pub struct drm_printer;
#[repr(C)] pub struct drm_privacy_screen;
#[repr(C)] pub struct drm_edid;
#[repr(C)] pub struct edid;
#[repr(C)] pub struct hdmi_codec_daifmt;
#[repr(C)] pub struct hdmi_codec_params;
#[repr(C)] pub struct i2c_adapter;
#[repr(C)] pub struct drm_file;
#[repr(C)] pub struct drm_atomic_commit;
#[repr(C)] pub struct drm_crtc_commit;
#[repr(C)] pub struct drm_writeback_job;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_attribute;
#[repr(C)] pub struct fwnode_handle;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct llist_node;
#[repr(C)] pub struct drm_mode_object;
#[repr(C)] pub struct drm_object_properties;
#[repr(C)] pub struct drm_tile_group;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct notifier_block;
#[repr(C)] pub struct hdr_sink_metadata;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;

#[repr(C)] #[derive(Copy, Clone)] pub struct hdmi_infoframe { pub _data: [u8; 1] }
#[repr(C)] pub union hdmi_infoframe_union { pub data: hdmi_infoframe }
pub type hdmi_infoframe = hdmi_infoframe_union;

#[repr(C)] #[derive(Copy, Clone)]
pub enum drm_connector_force { DRM_FORCE_UNSPECIFIED, DRM_FORCE_OFF, DRM_FORCE_ON, DRM_FORCE_ON_DIGITAL }
#[repr(C)] #[derive(Copy, Clone)]
pub enum drm_connector_status { connector_status_connected = 1, connector_status_disconnected = 2, connector_status_unknown = 3 }
#[repr(C)] #[derive(Copy, Clone)]
pub enum drm_connector_registration_state { DRM_CONNECTOR_INITIALIZING = 0, DRM_CONNECTOR_REGISTERED = 1, DRM_CONNECTOR_UNREGISTERED = 2 }
#[repr(C)] #[derive(Copy, Clone)]
pub enum subpixel_order { SubPixelUnknown = 0, SubPixelHorizontalRGB, SubPixelHorizontalBGR, SubPixelVerticalRGB, SubPixelVerticalBGR, SubPixelNone }
#[repr(C)] #[derive(Copy, Clone)]
pub enum drm_connector_tv_mode { DRM_MODE_TV_MODE_NTSC, DRM_MODE_TV_MODE_NTSC_443, DRM_MODE_TV_MODE_NTSC_J, DRM_MODE_TV_MODE_PAL, DRM_MODE_TV_MODE_PAL_M, DRM_MODE_TV_MODE_PAL_N, DRM_MODE_TV_MODE_SECAM, DRM_MODE_TV_MODE_MONOCHROME, DRM_MODE_TV_MODE_MAX }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_link_status { DRM_LINK_STATUS_GOOD = 0, DRM_LINK_STATUS_BAD = 1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_panel_orientation { DRM_MODE_PANEL_ORIENTATION_UNKNOWN = -1, DRM_MODE_PANEL_ORIENTATION_NORMAL = 0, DRM_MODE_PANEL_ORIENTATION_BOTTOM_UP, DRM_MODE_PANEL_ORIENTATION_LEFT_UP, DRM_MODE_PANEL_ORIENTATION_RIGHT_UP }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_hdmi_broadcast_rgb { DRM_HDMI_BROADCAST_RGB_AUTO, DRM_HDMI_BROADCAST_RGB_FULL, DRM_HDMI_BROADCAST_RGB_LIMITED }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_privacy_screen_status { PRIVACY_SCREEN_DISABLED = 0, PRIVACY_SCREEN_ENABLED, PRIVACY_SCREEN_DISABLED_LOCKED, PRIVACY_SCREEN_ENABLED_LOCKED }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_colorspace { DRM_MODE_COLORIMETRY_DEFAULT = 0, DRM_MODE_COLORIMETRY_NO_DATA = 0, DRM_MODE_COLORIMETRY_SMPTE_170M_YCC = 1, DRM_MODE_COLORIMETRY_BT709_YCC, DRM_MODE_COLORIMETRY_XVYCC_601, DRM_MODE_COLORIMETRY_XVYCC_709, DRM_MODE_COLORIMETRY_SYCC_601, DRM_MODE_COLORIMETRY_OPYCC_601, DRM_MODE_COLORIMETRY_OPRGB, DRM_MODE_COLORIMETRY_BT2020_CYCC, DRM_MODE_COLORIMETRY_BT2020_RGB, DRM_MODE_COLORIMETRY_BT2020_YCC, DRM_MODE_COLORIMETRY_DCI_P3_RGB_D65, DRM_MODE_COLORIMETRY_DCI_P3_RGB_THEATER, DRM_MODE_COLORIMETRY_RGB_WIDE_FIXED, DRM_MODE_COLORIMETRY_RGB_WIDE_FLOAT, DRM_MODE_COLORIMETRY_BT601_YCC, DRM_MODE_COLORIMETRY_COUNT }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_output_color_format { DRM_OUTPUT_COLOR_FORMAT_RGB444 = 0, DRM_OUTPUT_COLOR_FORMAT_YCBCR444, DRM_OUTPUT_COLOR_FORMAT_YCBCR422, DRM_OUTPUT_COLOR_FORMAT_YCBCR420, DRM_OUTPUT_COLOR_FORMAT_COUNT }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_connector_color_format { DRM_CONNECTOR_COLOR_FORMAT_AUTO = 0, DRM_CONNECTOR_COLOR_FORMAT_RGB444, DRM_CONNECTOR_COLOR_FORMAT_YCBCR444, DRM_CONNECTOR_COLOR_FORMAT_YCBCR422, DRM_CONNECTOR_COLOR_FORMAT_YCBCR420, DRM_CONNECTOR_COLOR_FORMAT_COUNT }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_mode_subconnector { DRM_MODE_SUBCONNECTOR_Unknown = 0 }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_picture_aspect { HDMI_PICTURE_ASPECT_NONE = 0 }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_mode_status { MODE_OK = 0 }

#[repr(C)] pub struct drm_scrambling { pub supported: bool, pub low_rates: bool }
#[repr(C)] pub struct drm_scdc { pub supported: bool, pub read_request: bool, pub scrambling: drm_scrambling }
#[repr(C)] pub struct drm_hdmi_dsc_cap { pub v_1p2: bool, pub native_420: bool, pub all_bpp: bool, pub bpc_supported: u8, pub max_slices: u8, pub clk_per_slice: i32, pub max_lanes: u8, pub max_frl_rate_per_lane: u8, pub total_chunk_kbytes: u8 }
#[repr(C)] pub struct drm_hdmi_info { pub scdc: drm_scdc, pub y420_vdb_modes: [usize; 4], pub y420_cmdb_modes: [usize; 4], pub y420_dc_modes: u8, pub max_frl_rate_per_lane: u8, pub max_lanes: u8, pub dsc_cap: drm_hdmi_dsc_cap }
#[repr(C)] pub struct drm_monitor_range_info { pub min_vfreq: u16, pub max_vfreq: u16 }
#[repr(C)] pub struct drm_luminance_range_info { pub min_luminance: u32, pub max_luminance: u32 }
#[repr(C)] pub struct drm_amd_vsdb_info { pub version: u8, pub replay_mode: bool, pub panel_type: u8, pub luminance_range1: drm_luminance_range_info, pub luminance_range2: drm_luminance_range_info }
#[repr(C)] pub struct drm_display_info {
    pub width_mm: u32, pub height_mm: u32, pub bpc: u32, pub subpixel_order: subpixel_order, pub panel_orientation: i32, pub color_formats: u32, pub bus_formats: *const u32, pub num_bus_formats: u32, pub bus_flags: u32, pub max_tmds_clock: i32, pub dvi_dual: bool, pub is_hdmi: bool, pub has_audio: bool, pub has_hdmi_infoframe: bool, pub rgb_quant_range_selectable: bool, pub edid_hdmi_rgb444_dc_modes: u8, pub edid_hdmi_ycbcr444_dc_modes: u8, pub cea_rev: u8, pub hdmi: drm_hdmi_info, pub hdr_sink_metadata: hdr_sink_metadata, pub non_desktop: bool, pub monitor_range: drm_monitor_range_info, pub luminance_range: drm_luminance_range_info, pub mso_stream_count: u8, pub mso_pixel_overlap: u8, pub max_dsc_bpp: u32, pub vics: *mut u8, pub vics_len: i32, pub quirks: u32, pub source_physical_address: u16, pub amd_vsdb: drm_amd_vsdb_info, pub panel_type: u8
}
#[repr(C)] pub struct drm_connector_tv_margins { pub bottom: u32, pub left: u32, pub right: u32, pub top: u32 }
#[repr(C)] pub struct drm_tv_connector_state { pub select_subconnector: drm_mode_subconnector, pub subconnector: drm_mode_subconnector, pub margins: drm_connector_tv_margins, pub legacy_mode: u32, pub mode: u32, pub brightness: u32, pub contrast: u32, pub flicker_reduction: u32, pub overscan: u32, pub saturation: u32, pub hue: u32 }
#[repr(C)] pub struct drm_connector_hdmi_infoframe { pub data: hdmi_infoframe, pub set: bool }
#[repr(C)] pub struct drm_connector_hdmi_state { pub broadcast_rgb: drm_hdmi_broadcast_rgb, pub infoframes: drm_connector_hdmi_infoframes, pub is_limited_range: bool, pub output_bpc: u32, pub output_format: drm_output_color_format, pub tmds_char_rate: u64 }
#[repr(C)] pub struct drm_connector_hdmi_infoframes { pub avi: drm_connector_hdmi_infoframe, pub hdr_drm: drm_connector_hdmi_infoframe, pub spd: drm_connector_hdmi_infoframe, pub hdmi: drm_connector_hdmi_infoframe }

#[repr(C)] pub struct drm_connector_state { pub connector: *mut drm_connector, pub crtc: *mut drm_crtc, pub best_encoder: *mut drm_encoder, pub link_status: drm_link_status, pub state: *mut drm_atomic_commit, pub commit: *mut drm_crtc_commit, pub tv: drm_tv_connector_state, pub self_refresh_aware: bool, pub picture_aspect_ratio: hdmi_picture_aspect, pub content_type: u32, pub hdcp_content_type: u32, pub scaling_mode: u32, pub content_protection: u32, pub colorspace: drm_colorspace, pub color_format: drm_connector_color_format, pub writeback_job: *mut drm_writeback_job, pub max_requested_bpc: u8, pub max_bpc: u8, pub privacy_screen_sw_state: drm_privacy_screen_status, pub hdr_output_metadata: *mut drm_property_blob, pub hdmi: drm_connector_hdmi_state }

pub type drm_connector_hdmi_audio_startup = unsafe extern "C" fn(*mut drm_connector) -> i32;
#[repr(C)] pub struct drm_connector_hdmi_audio_funcs { pub startup: Option<drm_connector_hdmi_audio_startup>, pub prepare: Option<unsafe extern "C" fn(*mut drm_connector,*mut hdmi_codec_daifmt,*mut hdmi_codec_params)->i32>, pub shutdown: Option<unsafe extern "C" fn(*mut drm_connector)>, pub mute_stream: Option<unsafe extern "C" fn(*mut drm_connector,bool,i32)->i32> }
#[repr(C)] pub struct drm_connector_cec_funcs { pub phys_addr_invalidate: Option<unsafe extern "C" fn(*mut drm_connector)>, pub phys_addr_set: Option<unsafe extern "C" fn(*mut drm_connector,u16)> }
#[repr(C)] pub struct drm_connector_infoframe_funcs { pub clear_infoframe: Option<unsafe extern "C" fn(*mut drm_connector)->i32>, pub write_infoframe: Option<unsafe extern "C" fn(*mut drm_connector,*const u8,usize)->i32> }
#[repr(C)] pub struct drm_connector_hdmi_funcs { pub tmds_char_rate_valid: Option<unsafe extern "C" fn(*const drm_connector,*const drm_display_mode,u64)->drm_mode_status>, pub read_edid: Option<unsafe extern "C" fn(*mut drm_connector)->*const drm_edid>, pub avi: drm_connector_infoframe_funcs, pub hdmi: drm_connector_infoframe_funcs, pub audio: drm_connector_infoframe_funcs, pub hdr_drm: drm_connector_infoframe_funcs, pub spd: drm_connector_infoframe_funcs }

#[repr(C)] pub struct drm_connector_funcs { pub dpms: Option<unsafe extern "C" fn(*mut drm_connector,i32)->i32>, pub reset: Option<unsafe extern "C" fn(*mut drm_connector)>, pub detect: Option<unsafe extern "C" fn(*mut drm_connector,bool)->drm_connector_status>, pub force: Option<unsafe extern "C" fn(*mut drm_connector)>, pub fill_modes: Option<unsafe extern "C" fn(*mut drm_connector,u32,u32)->i32>, pub set_property: Option<unsafe extern "C" fn(*mut drm_connector,*mut drm_property,u64)->i32>, pub late_register: Option<unsafe extern "C" fn(*mut drm_connector)->i32>, pub early_unregister: Option<unsafe extern "C" fn(*mut drm_connector)>, pub destroy: Option<unsafe extern "C" fn(*mut drm_connector)>, pub atomic_create_state: Option<unsafe extern "C" fn(*mut drm_connector)->*mut drm_connector_state>, pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_connector)->*mut drm_connector_state>, pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_connector,*mut drm_connector_state)>, pub atomic_set_property: Option<unsafe extern "C" fn(*mut drm_connector,*mut drm_connector_state,*mut drm_property,u64)->i32>, pub atomic_get_property: Option<unsafe extern "C" fn(*mut drm_connector,*const drm_connector_state,*mut drm_property,*mut u64)->i32>, pub atomic_print_state: Option<unsafe extern "C" fn(*mut drm_printer,*const drm_connector_state)>, pub oob_hotplug_event: Option<unsafe extern "C" fn(*mut drm_connector,drm_connector_status)>, pub debugfs_init: Option<unsafe extern "C" fn(*mut drm_connector,*mut dentry)>, pub color_format: Option<unsafe extern "C" fn(*const drm_connector_state)->drm_connector_color_format> }

pub const DRM_CONNECTOR_HDMI_VENDOR_LEN: usize = 8;
pub const DRM_CONNECTOR_HDMI_PRODUCT_LEN: usize = 16;
#[repr(C)] pub struct drm_connector_hdmi { pub vendor: [u8;8], pub product: [u8;16], pub supported_formats: usize, pub funcs: *const drm_connector_hdmi_funcs, pub infoframes: drm_connector_hdmi_current_infoframes }
#[repr(C)] pub struct drm_connector_hdmi_current_infoframes { pub lock: mutex, pub audio: drm_connector_hdmi_infoframe }
#[repr(C)] pub struct drm_connector_cec { pub mutex: mutex, pub funcs: *const drm_connector_cec_funcs, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct drm_connector_hdmi_audio { pub funcs: *const drm_connector_hdmi_audio_funcs, pub codec_pdev: *mut platform_device, pub lock: mutex, pub plugged_cb: Option<unsafe extern "C" fn(*mut device,bool)>, pub plugged_cb_dev: *mut device, pub last_state: bool, pub dai_port: i32 }

#[repr(C)] pub struct drm_cmdline_mode { pub name: [core::ffi::c_char; 32], pub specified: bool, pub refresh_specified: bool, pub bpp_specified: bool, pub pixel_clock: u32, pub xres: i32, pub yres: i32, pub bpp: i32, pub refresh: i32, pub rb: bool, pub interlace: bool, pub cvt: bool, pub margins: bool, pub force: drm_connector_force, pub rotation_reflection: u32, pub panel_orientation: drm_panel_orientation, pub tv_margins: drm_connector_tv_margins, pub tv_mode: drm_connector_tv_mode, pub tv_mode_specified: bool }

#[repr(C)] pub struct drm_connector { pub dev:*mut drm_device, pub kdev:*mut device, pub attr:*mut device_attribute, pub fwnode:*mut fwnode_handle, pub head:list_head, pub global_connector_list_entry:list_head, pub base:drm_mode_object, pub name:*mut core::ffi::c_char, pub mutex:mutex, pub index:u32, pub connector_type:i32, pub connector_type_id:i32, pub interlace_allowed:bool, pub doublescan_allowed:bool, pub stereo_allowed:bool, pub ycbcr_420_allowed:bool, pub registration_state:drm_connector_registration_state, pub modes:list_head, pub status:drm_connector_status, pub probed_modes:list_head, pub display_info:drm_display_info, pub funcs:*const drm_connector_funcs, pub edid_blob_ptr:*mut drm_property_blob, pub properties:drm_object_properties, pub scaling_mode_property:*mut drm_property, pub vrr_capable_property:*mut drm_property, pub colorspace_property:*mut drm_property, pub color_format_property:*mut drm_property, pub path_blob_ptr:*mut drm_property_blob, pub max_bpc:u32, pub max_bpc_property:*mut drm_property, pub privacy_screen:*mut drm_privacy_screen, pub privacy_screen_notifier:notifier_block, pub privacy_screen_sw_state_property:*mut drm_property, pub privacy_screen_hw_state_property:*mut drm_property, pub broadcast_rgb_property:*mut drm_property, pub polled:u8, pub dpms:i32, pub helper_private:*const drm_connector_helper_funcs, pub cmdline_mode:drm_cmdline_mode, pub force:drm_connector_force, pub edid_override:*const drm_edid, pub edid_override_mutex:mutex, pub epoch_counter:u64, pub possible_encoders:u32, pub encoder:*mut drm_encoder, pub eld:[u8;128], pub eld_mutex:mutex, pub latency_present:[bool;2], pub video_latency:[i32;2], pub audio_latency:[i32;2], pub ddc:*mut i2c_adapter, pub null_edid_counter:i32, pub bad_edid_counter:u32, pub edid_corrupt:bool, pub real_edid_checksum:u8, pub debugfs_entry:*mut dentry, pub state:*mut drm_connector_state, pub tile_blob_ptr:*mut drm_property_blob, pub has_tile:bool, pub tile_group:*mut drm_tile_group, pub tile_is_single_monitor:bool, pub num_h_tile:u8, pub num_v_tile:u8, pub tile_h_loc:u8, pub tile_v_loc:u8, pub tile_h_size:u16, pub tile_v_size:u16, pub free_node:llist_node, pub hdmi:drm_connector_hdmi, pub hdmi_audio:drm_connector_hdmi_audio, pub cec:drm_connector_cec }

#[repr(C)] pub struct drm_tile_group { pub refcount:kref, pub dev:*mut drm_device, pub id:i32, pub group_data:[u8;9] }
#[repr(C)] pub struct drm_connector_list_iter { pub dev:*mut drm_device, pub conn:*mut drm_connector }

extern "C" {
    pub fn drm_display_info_set_bus_formats(info:*mut drm_display_info, formats:*const u32, num_formats:u32)->i32;
    pub fn drm_connector_init(dev:*mut drm_device, connector:*mut drm_connector, funcs:*const drm_connector_funcs, connector_type:i32)->i32;
    pub fn drm_connector_dynamic_init(dev:*mut drm_device, connector:*mut drm_connector, funcs:*const drm_connector_funcs, connector_type:i32, ddc:*mut i2c_adapter)->i32;
    pub fn drm_connector_init_with_ddc(dev:*mut drm_device, connector:*mut drm_connector, funcs:*const drm_connector_funcs, connector_type:i32, ddc:*mut i2c_adapter)->i32;
    pub fn drmm_connector_init(dev:*mut drm_device, connector:*mut drm_connector, funcs:*const drm_connector_funcs, connector_type:i32, ddc:*mut i2c_adapter)->i32;
    pub fn drm_connector_cleanup(connector:*mut drm_connector);
    pub fn drm_connector_register(connector:*mut drm_connector)->i32;
    pub fn drm_connector_unregister(connector:*mut drm_connector);
    pub fn drm_connector_attach_encoder(connector:*mut drm_connector, encoder:*mut drm_encoder)->i32;
    pub fn drm_connector_list_iter_begin(dev:*mut drm_device, iter:*mut drm_connector_list_iter);
    pub fn drm_connector_list_iter_next(iter:*mut drm_connector_list_iter)->*mut drm_connector;
    pub fn drm_connector_list_iter_end(iter:*mut drm_connector_list_iter);
    pub fn drm_connector_oob_hotplug_event(fwnode:*mut fwnode_handle, status:drm_connector_status);
    pub fn drm_get_connector_type_name(t:u32)->*const core::ffi::c_char;
    pub fn drm_get_connector_status_name(s:drm_connector_status)->*const core::ffi::c_char;
    pub fn drm_get_colorspace_name(c:drm_colorspace)->*const core::ffi::c_char;
}

#[inline] pub unsafe fn drm_connector_index(c:*const drm_connector)->u32 { (*c).index }
#[inline] pub unsafe fn drm_connector_mask(c:*const drm_connector)->u32 { 1u32.wrapping_shl((*c).index) }
#[inline] pub unsafe fn obj_to_connector(x:*mut drm_mode_object)->*mut drm_connector { x as *mut drm_connector }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
