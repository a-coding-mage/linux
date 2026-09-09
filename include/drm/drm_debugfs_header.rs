/*
 * Internal Header for the Direct Rendering Manager
 *
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * Copyright (c) 2009-2010, Code Aurora Forum.
 * All rights reserved.
 *
 * Author: Rickard E. (Rik) Faith <faith@valinux.com>
 * Author: Gareth Hughes <gareth@valinux.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/DRM translation.

/// DRM_DEBUGFS_GPUVA_INFO - drm_info_list entry to dump a GPU VA space.
#[macro_export]
macro_rules! DRM_DEBUGFS_GPUVA_INFO {
    ($show:expr, $data:expr) => {
        $crate::drm_info_list { name: "gpuvas\0", show: $show, driver_features: 0, data: $data }
    };
}

/// debugfs info list entry.
#[repr(C)]
pub struct drm_info_list {
    /// File name.
    pub name: *const core::ffi::c_char,
    /// Show callback.
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
    /// Required driver features for this entry.
    pub driver_features: u32,
    /// Driver-private data, should not be device-specific.
    pub data: *mut core::ffi::c_void,
}

/// Per-minor debugfs node structure.
#[repr(C)]
pub struct drm_info_node {
    /// drm_minor for this node.
    pub minor: *mut drm_minor,
    /// Template for this node.
    pub info_ent: *const drm_info_list,
    // private:
    pub list: list_head,
    pub dent: *mut dentry,
}

/// debugfs info list entry.
#[repr(C)]
pub struct drm_debugfs_info {
    /// File name.
    pub name: *const core::ffi::c_char,
    /// Show callback.
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
    /// Required driver features for this entry.
    pub driver_features: u32,
    /// Driver-private data, should not be device-specific.
    pub data: *mut core::ffi::c_void,
}

/// Per-device debugfs node structure.
#[repr(C)]
pub struct drm_debugfs_entry {
    /// drm_device for this node.
    pub dev: *mut drm_device,
    /// Template for this node.
    pub file: drm_debugfs_info,
    /// Linked list of all device nodes.
    pub list: list_head,
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn drm_debugfs_create_files(files: *const drm_info_list, count: i32, root: *mut dentry, minor: *mut drm_minor);
    pub fn drm_debugfs_remove_files(files: *const drm_info_list, count: i32, root: *mut dentry, minor: *mut drm_minor) -> i32;
    pub fn drm_debugfs_add_file(dev: *mut drm_device, name: *const core::ffi::c_char, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void);
    pub fn drm_debugfs_add_files(dev: *mut drm_device, files: *const drm_debugfs_info, count: i32);
    pub fn drm_debugfs_gpuva_info(m: *mut seq_file, gpuvm: *mut drm_gpuvm) -> i32;
    pub fn drm_debugfs_clients_add(file: *mut drm_file);
    pub fn drm_debugfs_clients_remove(file: *mut drm_file);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_create_files(_files: *const drm_info_list, _count: i32, _root: *mut dentry, _minor: *mut drm_minor) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_remove_files(_files: *const drm_info_list, _count: i32, _root: *mut dentry, _minor: *mut drm_minor) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_add_file(_dev: *mut drm_device, _name: *const core::ffi::c_char, _show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>, _data: *mut core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_add_files(_dev: *mut drm_device, _files: *const drm_debugfs_info, _count: i32) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_gpuva_info(_m: *mut seq_file, _gpuvm: *mut drm_gpuvm) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_clients_add(_file: *mut drm_file) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn drm_debugfs_clients_remove(_file: *mut drm_file) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
