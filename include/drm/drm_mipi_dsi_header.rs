/* SPDX-License-Identifier: GPL-2.0-only */
/* MIPI DSI Bus -- Rust translation of drm_mipi_dsi.h */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type size_t = usize;
pub type ssize_t = isize;

#[repr(C)] pub struct mipi_dsi_host { pub dev: *mut device, pub ops: *const mipi_dsi_host_ops, pub list: list_head }
#[repr(C)] pub struct mipi_dsi_device { pub host: *mut mipi_dsi_host, pub dev: device, pub attached: bool, pub name: [c_char; DSI_DEV_NAME_SIZE], pub channel: c_uint, pub lanes: c_uint, pub format: mipi_dsi_pixel_format, pub mode_flags: c_ulong, pub hs_rate: c_ulong, pub lp_rate: c_ulong, pub dsc: *mut drm_dsc_config }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct drm_dsc_config { _private: [u8; 0] }
#[repr(C)] pub struct drm_dsc_picture_parameter_set { _private: [u8; 0] }

pub const MIPI_DSI_MSG_REQ_ACK: u16 = 1 << 0;
pub const MIPI_DSI_MSG_USE_LPM: u16 = 1 << 1;

#[repr(C)] pub struct mipi_dsi_msg { pub channel: u8, pub type_: u8, pub flags: u16, pub tx_len: size_t, pub tx_buf: *const c_void, pub rx_len: size_t, pub rx_buf: *mut c_void }
extern "C" { pub fn mipi_dsi_packet_format_is_short(type_: u8) -> bool; pub fn mipi_dsi_packet_format_is_long(type_: u8) -> bool; }
#[repr(C)] pub struct mipi_dsi_packet { pub size: size_t, pub header: [u8; 4], pub payload_length: size_t, pub payload: *const u8 }
extern "C" { pub fn mipi_dsi_create_packet(packet: *mut mipi_dsi_packet, msg: *const mipi_dsi_msg) -> c_int; }

#[repr(C)] pub struct mipi_dsi_host_ops { pub attach: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *mut mipi_dsi_device) -> c_int>, pub detach: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *mut mipi_dsi_device) -> c_int>, pub transfer: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *const mipi_dsi_msg) -> ssize_t> }
extern "C" { pub fn mipi_dsi_host_register(host: *mut mipi_dsi_host) -> c_int; pub fn mipi_dsi_host_unregister(host: *mut mipi_dsi_host); pub fn of_find_mipi_dsi_host_by_node(node: *mut device_node) -> *mut mipi_dsi_host; }

pub const MIPI_DSI_MODE_VIDEO: c_ulong = 1 << 0;
pub const MIPI_DSI_MODE_VIDEO_BURST: c_ulong = 1 << 1;
pub const MIPI_DSI_MODE_VIDEO_SYNC_PULSE: c_ulong = 1 << 2;
pub const MIPI_DSI_MODE_VIDEO_AUTO_VERT: c_ulong = 1 << 3;
pub const MIPI_DSI_MODE_VIDEO_HSE: c_ulong = 1 << 4;
pub const MIPI_DSI_MODE_VIDEO_NO_HFP: c_ulong = 1 << 5;
pub const MIPI_DSI_MODE_VIDEO_NO_HBP: c_ulong = 1 << 6;
pub const MIPI_DSI_MODE_VIDEO_NO_HSA: c_ulong = 1 << 7;
pub const MIPI_DSI_MODE_NO_EOT_PACKET: c_ulong = 1 << 9;
pub const MIPI_DSI_CLOCK_NON_CONTINUOUS: c_ulong = 1 << 10;
pub const MIPI_DSI_MODE_LPM: c_ulong = 1 << 11;
pub const MIPI_DSI_HS_PKT_END_ALIGNED: c_ulong = 1 << 12;
pub const MIPI_DSI_MODE_DSC_ALL_SLICES_IN_PKT: c_ulong = 1 << 13;

#[repr(C)] #[derive(Copy, Clone)] pub enum mipi_dsi_pixel_format { MIPI_DSI_FMT_RGB888, MIPI_DSI_FMT_RGB666, MIPI_DSI_FMT_RGB666_PACKED, MIPI_DSI_FMT_RGB565, MIPI_DSI_FMT_RGB101010 }
pub const DSI_DEV_NAME_SIZE: usize = 20;
#[repr(C)] pub struct mipi_dsi_device_info { pub type_: [c_char; DSI_DEV_NAME_SIZE], pub channel: u32, pub node: *mut device_node }
#[repr(C)] pub struct mipi_dsi_multi_context { pub dsi: *mut mipi_dsi_device, pub accum_err: c_int }
pub const MIPI_DSI_MODULE_PREFIX: &[u8] = b"mipi-dsi:\0";

#[inline] pub fn mipi_dsi_pixel_format_to_bpp(fmt: mipi_dsi_pixel_format) -> c_int { match fmt { mipi_dsi_pixel_format::MIPI_DSI_FMT_RGB101010 => 30, mipi_dsi_pixel_format::MIPI_DSI_FMT_RGB888 | mipi_dsi_pixel_format::MIPI_DSI_FMT_RGB666 => 24, mipi_dsi_pixel_format::MIPI_DSI_FMT_RGB666_PACKED => 18, mipi_dsi_pixel_format::MIPI_DSI_FMT_RGB565 => 16, } }
#[repr(C)] #[derive(Copy, Clone)] pub enum mipi_dsi_compression_algo { MIPI_DSI_COMPRESSION_DSC = 0, MIPI_DSI_COMPRESSION_VENDOR = 3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum mipi_dsi_dcs_tear_mode { MIPI_DSI_DCS_TEAR_MODE_VBLANK, MIPI_DSI_DCS_TEAR_MODE_VHBLANK }
pub const MIPI_DSI_DCS_POWER_MODE_DISPLAY: u8 = 1 << 2; pub const MIPI_DSI_DCS_POWER_MODE_NORMAL: u8 = 1 << 3; pub const MIPI_DSI_DCS_POWER_MODE_SLEEP: u8 = 1 << 4; pub const MIPI_DSI_DCS_POWER_MODE_PARTIAL: u8 = 1 << 5; pub const MIPI_DSI_DCS_POWER_MODE_IDLE: u8 = 1 << 6;

extern "C" {
 pub static mipi_dsi_bus_type: bus_type;
 pub fn mipi_dsi_device_register_full(*mut mipi_dsi_host, *const mipi_dsi_device_info) -> *mut mipi_dsi_device; pub fn mipi_dsi_device_unregister(*mut mipi_dsi_device);
 pub fn mipi_dsi_attach(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_detach(*mut mipi_dsi_device) -> c_int;
 pub fn mipi_dsi_generic_write(*mut mipi_dsi_device, *const c_void, size_t) -> ssize_t; pub fn mipi_dsi_generic_read(*mut mipi_dsi_device, *const c_void, size_t, *mut c_void, size_t) -> ssize_t;
 pub fn drm_mipi_dsi_get_input_bus_fmt(mipi_dsi_pixel_format) -> u32;
 pub fn mipi_dsi_dcs_write_buffer(*mut mipi_dsi_device, *const c_void, size_t) -> ssize_t; pub fn mipi_dsi_dcs_write(*mut mipi_dsi_device, u8, *const c_void, size_t) -> ssize_t; pub fn mipi_dsi_dcs_read(*mut mipi_dsi_device, u8, *mut c_void, size_t) -> ssize_t;
}

#[repr(C)] pub struct mipi_dsi_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut mipi_dsi_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut mipi_dsi_device)>, pub shutdown: Option<unsafe extern "C" fn(*mut mipi_dsi_device)> }
extern "C" { pub fn mipi_dsi_driver_register_full(*mut mipi_dsi_driver, *mut module) -> c_int; pub fn mipi_dsi_driver_unregister(*mut mipi_dsi_driver); }

extern "C" {
 pub fn devm_mipi_dsi_device_register_full(*mut device, *mut mipi_dsi_host, *const mipi_dsi_device_info) -> *mut mipi_dsi_device; pub fn of_find_mipi_dsi_device_by_node(*mut device_node) -> *mut mipi_dsi_device; pub fn devm_mipi_dsi_attach(*mut device, *mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_shutdown_peripheral(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_turn_on_peripheral(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_set_maximum_return_packet_size(*mut mipi_dsi_device, u16) -> c_int; pub fn mipi_dsi_compression_mode(*mut mipi_dsi_device, bool) -> c_int; pub fn mipi_dsi_compression_mode_ext(*mut mipi_dsi_device, bool, mipi_dsi_compression_algo, c_uint) -> c_int; pub fn mipi_dsi_picture_parameter_set(*mut mipi_dsi_device, *const drm_dsc_picture_parameter_set) -> c_int;
 pub fn mipi_dsi_compression_mode_ext_multi(*mut mipi_dsi_multi_context, bool, mipi_dsi_compression_algo, c_uint); pub fn mipi_dsi_compression_mode_multi(*mut mipi_dsi_multi_context, bool); pub fn mipi_dsi_picture_parameter_set_multi(*mut mipi_dsi_multi_context, *const drm_dsc_picture_parameter_set); pub fn mipi_dsi_generic_write_multi(*mut mipi_dsi_multi_context, *const c_void, size_t); pub fn mipi_dsi_dual_generic_write_multi(*mut mipi_dsi_multi_context, *mut mipi_dsi_device, *mut mipi_dsi_device, *const c_void, size_t);
 pub fn mipi_dsi_dcs_write_buffer_chatty(*mut mipi_dsi_device, *const c_void, size_t) -> c_int; pub fn mipi_dsi_dcs_write_buffer_multi(*mut mipi_dsi_multi_context, *const c_void, size_t); pub fn mipi_dsi_dual_dcs_write_buffer_multi(*mut mipi_dsi_multi_context, *mut mipi_dsi_device, *mut mipi_dsi_device, *const c_void, size_t); pub fn mipi_dsi_dcs_read_multi(*mut mipi_dsi_multi_context, u8, *mut c_void, size_t);
 pub fn mipi_dsi_dcs_nop(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_dcs_soft_reset(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_dcs_get_power_mode(*mut mipi_dsi_device, *mut u8) -> c_int; pub fn mipi_dsi_dcs_get_pixel_format(*mut mipi_dsi_device, *mut u8) -> c_int; pub fn mipi_dsi_dcs_enter_sleep_mode(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_dcs_exit_sleep_mode(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_dcs_set_display_off(*mut mipi_dsi_device) -> c_int; pub fn mipi_dsi_dcs_set_display_on(*mut mipi_dsi_device) -> c_int;
 pub fn mipi_dsi_dcs_set_column_address(*mut mipi_dsi_device, u16, u16) -> c_int; pub fn mipi_dsi_dcs_set_page_address(*mut mipi_dsi_device, u16, u16) -> c_int; pub fn mipi_dsi_dcs_set_tear_on(*mut mipi_dsi_device, mipi_dsi_dcs_tear_mode) -> c_int; pub fn mipi_dsi_dcs_set_pixel_format(*mut mipi_dsi_device, u8) -> c_int; pub fn mipi_dsi_dcs_set_tear_scanline(*mut mipi_dsi_device, u16) -> c_int; pub fn mipi_dsi_dcs_set_display_brightness(*mut mipi_dsi_device, u16) -> c_int; pub fn mipi_dsi_dcs_get_display_brightness(*mut mipi_dsi_device, *mut u16) -> c_int; pub fn mipi_dsi_dcs_set_display_brightness_large(*mut mipi_dsi_device, u16) -> c_int; pub fn mipi_dsi_dcs_get_display_brightness_large(*mut mipi_dsi_device, *mut u16) -> c_int;
 pub fn mipi_dsi_dcs_nop_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_enter_sleep_mode_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_exit_sleep_mode_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_set_display_off_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_set_display_on_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_set_tear_on_multi(*mut mipi_dsi_multi_context, mipi_dsi_dcs_tear_mode); pub fn mipi_dsi_turn_on_peripheral_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_soft_reset_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_dcs_set_display_brightness_multi(*mut mipi_dsi_multi_context, u16); pub fn mipi_dsi_dcs_set_pixel_format_multi(*mut mipi_dsi_multi_context, u8); pub fn mipi_dsi_dcs_set_column_address_multi(*mut mipi_dsi_multi_context, u16, u16); pub fn mipi_dsi_dcs_set_page_address_multi(*mut mipi_dsi_multi_context, u16, u16); pub fn mipi_dsi_dcs_set_tear_scanline_multi(*mut mipi_dsi_multi_context, u16); pub fn mipi_dsi_dcs_set_tear_off_multi(*mut mipi_dsi_multi_context); pub fn mipi_dsi_shutdown_peripheral_multi(*mut mipi_dsi_multi_context);
}

/* The remaining C variadic helper macros are represented as Rust macros. */
#[macro_export] macro_rules! mipi_dsi_msleep { ($ctx:expr, $delay:expr) => { if unsafe { (*$ctx).accum_err } == 0 { unsafe { msleep($delay) } } }; }
#[macro_export] macro_rules! mipi_dsi_usleep_range { ($ctx:expr, $min:expr, $max:expr) => { if unsafe { (*$ctx).accum_err } == 0 { unsafe { usleep_range($min, $max) } } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
