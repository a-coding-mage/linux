/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Generic System Framebuffers on x86
 * Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 */

/* C dependencies: linux/err.h, linux/platform_data/simplefb.h,
 * linux/screen_info.h, linux/types.h, and video/edid.h. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct screen_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simplefb_platform_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct edid_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MacModel {
    M_I17,
    M_I20,
    M_I20_SR,
    M_I24,
    M_I24_8_1,
    M_I24_10_1,
    M_I27_11_1,
    M_MINI,
    M_MINI_3_1,
    M_MINI_4_1,
    M_MB,
    M_MB_2,
    M_MB_3,
    M_MB_5_1,
    M_MB_6_1,
    M_MB_7_1,
    M_MB_SR,
    M_MBA,
    M_MBA_3,
    M_MBP,
    M_MBP_2,
    M_MBP_2_2,
    M_MBP_SR,
    M_MBP_4,
    M_MBP_5_1,
    M_MBP_5_2,
    M_MBP_5_3,
    M_MBP_6_1,
    M_MBP_6_2,
    M_MBP_7_1,
    M_MBP_8_2,
    M_UNKNOWN,
}

#[repr(C)]
pub struct efifb_dmi_info {
    pub optname: *mut c_char,
    pub base: c_ulong,
    pub stride: c_int,
    pub width: c_int,
    pub height: c_int,
    pub flags: c_int,
}

#[repr(C)]
pub struct sysfb_display_info {
    pub screen: screen_info,
    /* Present only when CONFIG_FIRMWARE_EDID is enabled. */
    #[cfg(feature = "CONFIG_FIRMWARE_EDID")]
    pub edid: edid_info,
}

extern "C" {
    pub static mut sysfb_primary_display: sysfb_display_info;
}

/* CONFIG_SYSFB controls whether the external implementations are present. */
#[cfg(feature = "CONFIG_SYSFB")]
extern "C" {
    pub fn sysfb_disable(dev: *mut device);
    pub fn sysfb_handles_screen_info() -> bool;
}

#[cfg(not(feature = "CONFIG_SYSFB"))]
#[inline]
pub unsafe fn sysfb_disable(_dev: *mut device) {}

#[cfg(not(feature = "CONFIG_SYSFB"))]
#[inline]
pub unsafe fn sysfb_handles_screen_info() -> bool {
    false
}

/* CONFIG_EFI controls whether the external implementations are present. */
#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    pub static mut efifb_dmi_list: efifb_dmi_info;
    pub fn sysfb_apply_efi_quirks(si: *mut screen_info);
    pub fn sysfb_set_efifb_fwnode(si: *const screen_info, pd: *mut platform_device);
}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline]
pub unsafe fn sysfb_apply_efi_quirks(_si: *mut screen_info) {}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline]
pub unsafe fn sysfb_set_efifb_fwnode(_si: *const screen_info, _pd: *mut platform_device) {}

/* CONFIG_SYSFB_SIMPLEFB controls whether the external implementations are present. */
#[cfg(feature = "CONFIG_SYSFB_SIMPLEFB")]
extern "C" {
    pub fn sysfb_parse_mode(si: *const screen_info, mode: *mut simplefb_platform_data) -> bool;
    pub fn sysfb_create_simplefb(
        si: *const screen_info,
        mode: *const simplefb_platform_data,
        parent: *mut device,
    ) -> *mut platform_device;
}

#[cfg(not(feature = "CONFIG_SYSFB_SIMPLEFB"))]
#[inline]
pub unsafe fn sysfb_parse_mode(_si: *const screen_info, _mode: *mut simplefb_platform_data) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SYSFB_SIMPLEFB"))]
#[inline]
pub unsafe fn sysfb_create_simplefb(
    _si: *const screen_info,
    _mode: *const simplefb_platform_data,
    _parent: *mut device,
) -> *mut platform_device {
    (-22isize) as *mut platform_device /* ERR_PTR(-EINVAL) */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
