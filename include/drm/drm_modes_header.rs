/* Translated from drm_modes.h. */

// Dependencies supplied by the surrounding DRM translation are intentionally
// referenced here and are not reimplemented in this header translation.

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_mode_status {
    MODE_OK = 0,
    MODE_HSYNC, MODE_VSYNC, MODE_H_ILLEGAL, MODE_V_ILLEGAL, MODE_BAD_WIDTH,
    MODE_NOMODE, MODE_NO_INTERLACE, MODE_NO_DBLESCAN, MODE_NO_VSCAN, MODE_MEM,
    MODE_VIRTUAL_X, MODE_VIRTUAL_Y, MODE_MEM_VIRT, MODE_NOCLOCK,
    MODE_CLOCK_HIGH, MODE_CLOCK_LOW, MODE_CLOCK_RANGE, MODE_BAD_HVALUE,
    MODE_BAD_VVALUE, MODE_BAD_VSCAN, MODE_HSYNC_NARROW, MODE_HSYNC_WIDE,
    MODE_HBLANK_NARROW, MODE_HBLANK_WIDE, MODE_VSYNC_NARROW, MODE_VSYNC_WIDE,
    MODE_VBLANK_NARROW, MODE_VBLANK_WIDE, MODE_PANEL, MODE_INTERLACE_WIDTH,
    MODE_ONE_WIDTH, MODE_ONE_HEIGHT, MODE_ONE_SIZE, MODE_NO_REDUCED,
    MODE_NO_STEREO, MODE_NO_420,
    MODE_STALE = -3, MODE_BAD = -2, MODE_ERROR = -1,
}

pub const CRTC_INTERLACE_HALVE_V: u32 = 1 << 0;
pub const CRTC_STEREO_DOUBLE: u32 = 1 << 1;
pub const CRTC_NO_DBLSCAN: u32 = 1 << 2;
pub const CRTC_NO_VSCAN: u32 = 1 << 3;
pub const CRTC_STEREO_DOUBLE_ONLY: u32 = CRTC_STEREO_DOUBLE | CRTC_NO_DBLSCAN | CRTC_NO_VSCAN;
pub const DRM_MODE_MATCH_TIMINGS: u32 = 1 << 0;
pub const DRM_MODE_MATCH_CLOCK: u32 = 1 << 1;
pub const DRM_MODE_MATCH_FLAGS: u32 = 1 << 2;
pub const DRM_MODE_MATCH_3D_FLAGS: u32 = 1 << 3;
pub const DRM_MODE_MATCH_ASPECT_RATIO: u32 = 1 << 4;
pub const DRM_MODE_MATCH_TIMINGS_VRR: u32 = 1 << 5;
pub const DRM_MODE_FMT: &str = "\"%s\": %d %d %d %d %d %d %d %d %d %d 0x%x 0x%x";

#[macro_export]
macro_rules! DRM_MODE_RES_MM { ($res:expr, $dpi:expr) => { (($res) * 254u64) / (($dpi) * 10u64) } }
#[macro_export]
macro_rules! DRM_MODE_INIT { ($hz:expr, $hd:expr, $vd:expr, $hd_mm:expr, $vd_mm:expr) => {
    __DRM_MODE_INIT(($hd) * ($vd) * ($hz) / 1000, $hd, $vd, $hd_mm, $vd_mm)
} }
// C initializer macro; expand at the containing struct-literal site.
#[macro_export]
macro_rules! __DRM_MODE_INIT { ($pix:expr, $hd:expr, $vd:expr, $hd_mm:expr, $vd_mm:expr) => {
    type_: DRM_MODE_TYPE_DRIVER, clock: $pix, hdisplay: $hd, hsync_start: $hd,
    hsync_end: $hd, htotal: $hd, vdisplay: $vd, vsync_start: $vd,
    vsync_end: $vd, vtotal: $vd, width_mm: $hd_mm, height_mm: $vd_mm
} }
#[macro_export]
macro_rules! DRM_SIMPLE_MODE { ($hd:expr, $vd:expr, $hd_mm:expr, $vd_mm:expr) => {
    __DRM_MODE_INIT(1, $hd, $vd, $hd_mm, $vd_mm)
} }

#[repr(C)]
pub struct drm_display_mode {
    pub clock: i32,
    pub hdisplay: u16, pub hsync_start: u16, pub hsync_end: u16, pub htotal: u16,
    pub hskew: u16, pub vdisplay: u16, pub vsync_start: u16, pub vsync_end: u16,
    pub vtotal: u16, pub vscan: u16,
    pub flags: u32,
    pub crtc_clock: i32,
    pub crtc_hdisplay: u16, pub crtc_hblank_start: u16, pub crtc_hblank_end: u16,
    pub crtc_hsync_start: u16, pub crtc_hsync_end: u16, pub crtc_htotal: u16,
    pub crtc_hskew: u16, pub crtc_vdisplay: u16, pub crtc_vblank_start: u16,
    pub crtc_vblank_end: u16, pub crtc_vsync_start: u16, pub crtc_vsync_end: u16,
    pub crtc_vtotal: u16,
    pub width_mm: u16, pub height_mm: u16,
    pub r#type: u8,
    pub expose_to_userspace: bool,
    pub head: list_head,
    pub name: [::std::os::raw::c_char; DRM_DISPLAY_MODE_LEN as usize],
    pub status: drm_mode_status,
    pub picture_aspect_ratio: hdmi_picture_aspect,
}

#[inline]
pub unsafe fn drm_mode_is_stereo(mode: *const drm_display_mode) -> bool {
    ((*mode).flags & DRM_MODE_FLAG_3D_MASK) != 0
}

extern "C" {
    pub fn drm_mode_create(dev: *mut drm_device) -> *mut drm_display_mode;
    pub fn drm_mode_destroy(dev: *mut drm_device, mode: *mut drm_display_mode);
    pub fn drm_mode_convert_to_umode(out: *mut drm_mode_modeinfo, input: *const drm_display_mode);
    pub fn drm_mode_convert_umode(dev: *mut drm_device, out: *mut drm_display_mode, input: *const drm_mode_modeinfo) -> i32;
    pub fn drm_mode_probed_add(connector: *mut drm_connector, mode: *mut drm_display_mode);
    pub fn drm_mode_debug_printmodeline(mode: *const drm_display_mode);
    pub fn drm_mode_is_420_only(display: *const drm_display_info, mode: *const drm_display_mode) -> bool;
    pub fn drm_mode_is_420_also(display: *const drm_display_info, mode: *const drm_display_mode) -> bool;
    pub fn drm_mode_is_420(display: *const drm_display_info, mode: *const drm_display_mode) -> bool;
    pub fn drm_set_preferred_mode(connector: *mut drm_connector, hpref: i32, vpref: i32);
    pub fn drm_analog_tv_mode(dev: *mut drm_device, mode: drm_connector_tv_mode, pixel_clock_hz: usize, hdisplay: u32, vdisplay: u32, interlace: bool) -> *mut drm_display_mode;
    pub fn drm_cvt_mode(dev: *mut drm_device, hdisplay: i32, vdisplay: i32, vrefresh: i32, reduced: bool, interlaced: bool, margins: bool) -> *mut drm_display_mode;
    pub fn drm_gtf_mode(dev: *mut drm_device, hdisplay: i32, vdisplay: i32, vrefresh: i32, interlaced: bool, margins: i32) -> *mut drm_display_mode;
    pub fn drm_gtf_mode_complex(dev: *mut drm_device, hdisplay: i32, vdisplay: i32, vrefresh: i32, interlaced: bool, margins: i32, gtf_m: i32, gtf_2c: i32, gtf_k: i32, gtf_2j: i32) -> *mut drm_display_mode;
    pub fn drm_display_mode_from_videomode(vm: *const videomode, dmode: *mut drm_display_mode);
    pub fn drm_display_mode_to_videomode(dmode: *const drm_display_mode, vm: *mut videomode);
    pub fn drm_bus_flags_from_videomode(vm: *const videomode, bus_flags: *mut u32);
    pub fn drm_mode_set_name(mode: *mut drm_display_mode);
    pub fn drm_mode_vrefresh(mode: *const drm_display_mode) -> i32;
    pub fn drm_mode_get_hv_timing(mode: *const drm_display_mode, hdisplay: *mut i32, vdisplay: *mut i32);
    pub fn drm_mode_set_crtcinfo(p: *mut drm_display_mode, adjust_flags: i32);
    pub fn drm_mode_copy(dst: *mut drm_display_mode, src: *const drm_display_mode);
    pub fn drm_mode_init(dst: *mut drm_display_mode, src: *const drm_display_mode);
    pub fn drm_mode_duplicate(dev: *mut drm_device, mode: *const drm_display_mode) -> *mut drm_display_mode;
    pub fn drm_mode_match(mode1: *const drm_display_mode, mode2: *const drm_display_mode, match_flags: u32) -> bool;
    pub fn drm_mode_equal(mode1: *const drm_display_mode, mode2: *const drm_display_mode) -> bool;
    pub fn drm_mode_equal_no_clocks(mode1: *const drm_display_mode, mode2: *const drm_display_mode) -> bool;
    pub fn drm_mode_equal_no_clocks_no_stereo(mode1: *const drm_display_mode, mode2: *const drm_display_mode) -> bool;
    pub fn drm_mode_validate_driver(dev: *mut drm_device, mode: *const drm_display_mode) -> drm_mode_status;
    pub fn drm_mode_validate_size(mode: *const drm_display_mode, max_x: i32, max_y: i32) -> drm_mode_status;
    pub fn drm_mode_validate_ycbcr420(mode: *const drm_display_mode, connector: *mut drm_connector) -> drm_mode_status;
    pub fn drm_mode_prune_invalid(dev: *mut drm_device, mode_list: *mut list_head, verbose: bool);
    pub fn drm_mode_sort(mode_list: *mut list_head);
    pub fn drm_connector_list_update(connector: *mut drm_connector);
    pub fn drm_mode_parse_command_line_for_connector(mode_option: *const ::std::os::raw::c_char, connector: *const drm_connector, mode: *mut drm_cmdline_mode) -> bool;
    pub fn drm_mode_create_from_cmdline_mode(dev: *mut drm_device, cmd: *mut drm_cmdline_mode) -> *mut drm_display_mode;
}

#[inline]
pub unsafe fn drm_mode_analog_ntsc_480i(dev: *mut drm_device) -> *mut drm_display_mode {
    drm_analog_tv_mode(dev, DRM_MODE_TV_MODE_NTSC, 13500000, 720, 480, true)
}

#[inline]
pub unsafe fn drm_mode_analog_pal_576i(dev: *mut drm_device) -> *mut drm_display_mode {
    drm_analog_tv_mode(dev, DRM_MODE_TV_MODE_PAL, 13500000, 720, 576, true)
}

// The CONFIG_OF branch is supplied by the target kernel configuration.
#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_get_drm_display_mode(_np: *mut device_node, _dmode: *mut drm_display_mode,
                                      _bus_flags: *mut u32, _index: i32) -> i32 { -EINVAL }

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_get_drm_panel_display_mode(_np: *mut device_node, _dmode: *mut drm_display_mode,
                                            _bus_flags: *mut u32) -> i32 { -EINVAL }

extern "C" {
    pub type drm_device; pub type drm_mode_modeinfo; pub type drm_connector;
    pub type drm_display_info; pub type drm_cmdline_mode; pub type videomode;
    pub type list_head; pub type hdmi_picture_aspect; pub type drm_connector_tv_mode;
    pub type device_node; const EINVAL: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
