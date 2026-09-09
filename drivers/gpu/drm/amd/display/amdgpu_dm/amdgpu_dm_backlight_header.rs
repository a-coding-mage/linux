/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Opaque declarations corresponding to the C forward declarations.
#[repr(C)] pub struct amdgpu_display_manager { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _private: [u8; 0] }
#[repr(C)] pub struct backlight_device { _private: [u8; 0] }
#[repr(C)] pub struct backlight_properties { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_backlight_caps { _private: [u8; 0] }

pub const AMDGPU_DM_DEFAULT_MIN_BACKLIGHT: i32 = 12;
pub const AMDGPU_DM_DEFAULT_MAX_BACKLIGHT: i32 = 255;
pub const AMDGPU_DM_MIN_SPREAD: i32 =
    (AMDGPU_DM_DEFAULT_MAX_BACKLIGHT - AMDGPU_DM_DEFAULT_MIN_BACKLIGHT) / 2;
pub const AUX_BL_DEFAULT_TRANSITION_TIME_MS: i32 = 50;

pub type u32 = core::ffi::c_uint;
pub type uint = core::ffi::c_uint;
pub type ssize_t = isize;

unsafe extern "C" {
    pub fn amdgpu_dm_update_backlight_caps(dm: *mut amdgpu_display_manager, bl_idx: i32);
    pub fn amdgpu_dm_backlight_set_level(dm: *mut amdgpu_display_manager, bl_idx: i32, user_brightness: u32);
    pub fn amdgpu_dm_register_backlight_device(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_setup_backlight_device(dm: *mut amdgpu_display_manager, aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_update_connector_ext_caps(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_should_create_sysfs(aconnector: *mut amdgpu_dm_connector) -> bool;

    pub static amdgpu_group: attribute_group;

    // Conditional on CONFIG_DRM_AMD_DC_KUNIT_TEST in the C source.
    pub fn dm_find_stream_with_link(dm: *mut amdgpu_display_manager, link: *mut dc_link) -> *mut dc_stream_state;
    pub fn amdgpu_dm_backlight_update_status(bd: *mut backlight_device) -> i32;
    pub fn amdgpu_dm_backlight_get_level(dm: *mut amdgpu_display_manager, bl_idx: i32) -> u32;
    pub fn amdgpu_dm_backlight_get_brightness(bd: *mut backlight_device) -> i32;
    pub fn panel_power_savings_show(device: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn panel_power_savings_store(device: *mut device, attr: *mut device_attribute, buf: *const i8, count: usize) -> ssize_t;
    pub fn get_brightness_range(caps: *const amdgpu_dm_backlight_caps, min: *mut u32, max: *mut u32) -> i32;
    pub fn convert_custom_brightness(caps: *const amdgpu_dm_backlight_caps, min: u32, max: u32, user_brightness: *mut u32);
    pub fn convert_brightness_from_user(caps: *const amdgpu_dm_backlight_caps, brightness: u32) -> u32;
    pub fn convert_brightness_to_user(caps: *const amdgpu_dm_backlight_caps, brightness: u32) -> u32;
    pub fn amdgpu_dm_backlight_get_device_index(dm: *mut amdgpu_display_manager, bd: *mut backlight_device) -> i32;
    pub fn amdgpu_dm_backlight_fill_props(caps: *const amdgpu_dm_backlight_caps, is_system_supplied: bool, custom_curve_enabled: bool, props: *mut backlight_properties);
    pub fn amdgpu_dm_get_dc_debug_mask() -> uint;
    pub fn amdgpu_dm_set_dc_debug_mask(val: uint);
    pub fn amdgpu_dm_get_abm_level_param() -> i32;
    pub fn amdgpu_dm_set_abm_level_param(val: i32);
    pub fn amdgpu_dm_get_backlight_param() -> i32;
    pub fn amdgpu_dm_set_backlight_param(val: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
