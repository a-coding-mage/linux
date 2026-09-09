/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct acpi_video_brightness_flags {
    pub _BCL_no_ac_battery_levels: u8,
    pub _BCL_reversed: u8,
    pub _BQC_use_index: u8,
}

#[repr(C)]
pub struct acpi_video_device_brightness {
    pub curr: i32,
    pub count: i32,
    pub levels: *mut i32,
    pub flags: acpi_video_brightness_flags,
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

pub const ACPI_VIDEO_CLASS: &str = "video";

pub const ACPI_VIDEO_DISPLAY_CRT: i32 = 1;
pub const ACPI_VIDEO_DISPLAY_TV: i32 = 2;
pub const ACPI_VIDEO_DISPLAY_DVI: i32 = 3;
pub const ACPI_VIDEO_DISPLAY_LCD: i32 = 4;

pub const ACPI_VIDEO_DISPLAY_LEGACY_MONITOR: i32 = 0x0100;
pub const ACPI_VIDEO_DISPLAY_LEGACY_PANEL: i32 = 0x0110;
pub const ACPI_VIDEO_DISPLAY_LEGACY_TV: i32 = 0x0200;

pub const ACPI_VIDEO_NOTIFY_SWITCH: i32 = 0x80;
pub const ACPI_VIDEO_NOTIFY_PROBE: i32 = 0x81;
pub const ACPI_VIDEO_NOTIFY_CYCLE: i32 = 0x82;
pub const ACPI_VIDEO_NOTIFY_NEXT_OUTPUT: i32 = 0x83;
pub const ACPI_VIDEO_NOTIFY_PREV_OUTPUT: i32 = 0x84;
pub const ACPI_VIDEO_NOTIFY_CYCLE_BRIGHTNESS: i32 = 0x85;
pub const ACPI_VIDEO_NOTIFY_INC_BRIGHTNESS: i32 = 0x86;
pub const ACPI_VIDEO_NOTIFY_DEC_BRIGHTNESS: i32 = 0x87;
pub const ACPI_VIDEO_NOTIFY_ZERO_BRIGHTNESS: i32 = 0x88;
pub const ACPI_VIDEO_NOTIFY_DISPLAY_OFF: i32 = 0x89;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum acpi_backlight_type {
    acpi_backlight_undef = -1,
    acpi_backlight_none = 0,
    acpi_backlight_video,
    acpi_backlight_vendor,
    acpi_backlight_native,
    acpi_backlight_nvidia_wmi_ec,
    acpi_backlight_apple_gmux,
    acpi_backlight_dell_uart,
}

// Preserves the build-time CONFIG_ACPI_VIDEO condition from the C header.
#[cfg(feature = "CONFIG_ACPI_VIDEO")]
extern "C" {
    pub fn acpi_video_register() -> i32;
    pub fn acpi_video_unregister();
    pub fn acpi_video_register_backlight();
    pub fn acpi_video_get_edid(
        device: *mut acpi_device,
        type_: i32,
        device_id: i32,
        edid: *mut *mut c_void,
    ) -> i32;
    pub fn acpi_video_handles_brightness_key_presses() -> bool;
    pub fn acpi_video_get_levels(
        device: *mut acpi_device,
        dev_br: *mut *mut acpi_video_device_brightness,
        pmax_level: *mut i32,
    ) -> i32;
    pub fn __acpi_video_get_backlight_type(
        native: bool,
        auto_detect: *mut bool,
    ) -> acpi_backlight_type;
}

#[cfg(feature = "CONFIG_ACPI_VIDEO")]
#[inline]
pub unsafe fn acpi_video_get_backlight_type() -> acpi_backlight_type {
    __acpi_video_get_backlight_type(false, core::ptr::null_mut())
}

#[cfg(feature = "CONFIG_ACPI_VIDEO")]
#[inline]
pub unsafe fn acpi_video_backlight_use_native() -> bool {
    __acpi_video_get_backlight_type(true, core::ptr::null_mut())
        == acpi_backlight_type::acpi_backlight_native
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_register() -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_unregister() {}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_register_backlight() {}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_get_edid(
    _device: *mut acpi_device,
    _type: i32,
    _device_id: i32,
    _edid: *mut *mut c_void,
) -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_get_backlight_type() -> acpi_backlight_type {
    acpi_backlight_type::acpi_backlight_vendor
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_backlight_use_native() -> bool {
    true
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_handles_brightness_key_presses() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_ACPI_VIDEO"))]
#[inline]
pub fn acpi_video_get_levels(
    _device: *mut acpi_device,
    _dev_br: *mut *mut acpi_video_device_brightness,
    _pmax_level: *mut i32,
) -> i32 {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
