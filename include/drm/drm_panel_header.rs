/*
 * Copyright (C) 2013, NVIDIA Corporation.  All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sub license,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

use core::ffi::c_void;

#[repr(C)] pub struct backlight_device { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct display_timing { _private: [u8; 0] }

pub type drm_panel_orientation = i32;

#[repr(C)]
pub struct drm_panel_funcs {
    pub prepare: Option<unsafe extern "C" fn(*mut drm_panel) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut drm_panel) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut drm_panel) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut drm_panel) -> i32>,
    pub get_modes: Option<unsafe extern "C" fn(*mut drm_panel, *mut drm_connector) -> i32>,
    pub get_orientation: Option<unsafe extern "C" fn(*mut drm_panel) -> drm_panel_orientation>,
    pub get_timings: Option<unsafe extern "C" fn(*mut drm_panel, u32, *mut display_timing) -> i32>,
    pub debugfs_init: Option<unsafe extern "C" fn(*mut drm_panel, *mut dentry)>,
}

#[repr(C)]
pub struct drm_panel_follower_funcs {
    pub panel_prepared: Option<unsafe extern "C" fn(*mut drm_panel_follower) -> i32>,
    pub panel_unpreparing: Option<unsafe extern "C" fn(*mut drm_panel_follower) -> i32>,
    pub panel_enabled: Option<unsafe extern "C" fn(*mut drm_panel_follower) -> i32>,
    pub panel_disabling: Option<unsafe extern "C" fn(*mut drm_panel_follower) -> i32>,
}

#[repr(C)]
pub struct drm_panel_follower {
    pub funcs: *const drm_panel_follower_funcs,
    pub list: list_head,
    pub panel: *mut drm_panel,
}

#[repr(C)]
pub struct drm_panel {
    pub dev: *mut device,
    pub backlight: *mut backlight_device,
    pub funcs: *const drm_panel_funcs,
    pub connector_type: i32,
    pub list: list_head,
    pub followers: list_head,
    pub follower_lock: mutex,
    pub prepare_prev_first: bool,
    pub prepared: bool,
    pub enabled: bool,
    pub container: *mut c_void,
    pub refcount: kref,
}

extern "C" {
    pub fn __devm_drm_panel_alloc(dev: *mut device, size: usize, offset: usize,
                                  funcs: *const drm_panel_funcs, connector_type: i32) -> *mut c_void;
    pub fn drm_panel_get(panel: *mut drm_panel) -> *mut drm_panel;
    pub fn drm_panel_put(panel: *mut drm_panel);
    pub fn drm_panel_add(panel: *mut drm_panel);
    pub fn drm_panel_remove(panel: *mut drm_panel);
    pub fn devm_drm_panel_add(dev: *mut device, panel: *mut drm_panel) -> i32;
    pub fn drm_panel_prepare(panel: *mut drm_panel);
    pub fn drm_panel_unprepare(panel: *mut drm_panel);
    pub fn drm_panel_enable(panel: *mut drm_panel);
    pub fn drm_panel_disable(panel: *mut drm_panel);
    pub fn drm_panel_get_modes(panel: *mut drm_panel, connector: *mut drm_connector) -> i32;
    pub fn of_drm_find_panel(np: *const device_node) -> *mut drm_panel;
    pub fn drm_is_panel_follower(dev: *mut device) -> bool;
    pub fn drm_panel_add_follower(dev: *mut device, follower: *mut drm_panel_follower) -> i32;
    pub fn drm_panel_remove_follower(follower: *mut drm_panel_follower);
    pub fn devm_drm_panel_add_follower(dev: *mut device, follower: *mut drm_panel_follower) -> i32;
    pub fn drm_panel_of_backlight(panel: *mut drm_panel) -> i32;
}

#[macro_export]
macro_rules! devm_drm_panel_alloc {
    ($dev:expr, $ty:ty, $member:ident, $funcs:expr, $connector_type:expr) => {{
        __devm_drm_panel_alloc(
            $dev,
            core::mem::size_of::<$ty>(),
            core::mem::offset_of!($ty, $member),
            $funcs,
            $connector_type,
        ) as *mut $ty
    }};
}

// CONFIG_OF && CONFIG_DRM_PANEL: of_drm_find_panel is externally provided;
// otherwise the C inline fallback returns ERR_PTR(-ENODEV).
// CONFIG_DRM_PANEL and backlight configuration select the external helpers;
// otherwise the C inline fallbacks return false, -ENODEV, or 0 as applicable.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
