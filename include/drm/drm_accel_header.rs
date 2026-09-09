/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// C header guard: DRM_ACCEL_H_

// C dependency: <drm/drm_file.h>

pub const ACCEL_MAJOR: i32 = 261;
pub const ACCEL_MAX_MINORS: i32 = 256;

/**
 * DRM_ACCEL_FOPS - Default drm accelerators file operations
 *
 * This macro provides a shorthand for setting the accelerator file ops in the
 * &file_operations structure.  If all you need are the default ops, use
 * DEFINE_DRM_ACCEL_FOPS instead.
 */
// C macro DRM_ACCEL_FOPS expands to file_operations initializers:
// .open = accel_open, .release = drm_release, .unlocked_ioctl = drm_ioctl,
// .compat_ioctl = drm_compat_ioctl, .poll = drm_poll, .read = drm_read,
// .llseek = noop_llseek, .mmap = drm_gem_mmap,
// .fop_flags = FOP_UNSIGNED_OFFSET.

/**
 * DEFINE_DRM_ACCEL_FOPS() - macro to generate file operations for accelerators drivers
 * @name: name for the generated structure
 *
 * This macro autogenerates a suitable &struct file_operations for accelerators based
 * drivers, which can be assigned to &drm_driver.fops. Note that this structure
 * cannot be shared between drivers, because it contains a reference to the
 * current module using THIS_MODULE.
 *
 * Note that the declaration is already marked as static - if you need a
 * non-static version of this you're probably doing it wrong and will break the
 * THIS_MODULE reference by accident.
 */
// C macro DEFINE_DRM_ACCEL_FOPS(name) declares a static const struct
// file_operations named `name`, initialized with THIS_MODULE and DRM_ACCEL_FOPS.

// IS_ENABLED(CONFIG_DRM_ACCEL) is a build-time kernel configuration condition.
#[cfg(feature = "CONFIG_DRM_ACCEL")]
extern "C" {
    pub static mut accel_minors_xa: xarray;

    pub fn accel_core_exit();
    pub fn accel_core_init() -> i32;
    pub fn accel_set_device_instance_params(kdev: *mut device, index: i32);
    pub fn accel_open(inode: *mut inode, filp: *mut file) -> i32;
    pub fn accel_debugfs_register(dev: *mut drm_device);
}

#[cfg(not(feature = "CONFIG_DRM_ACCEL"))]
pub unsafe fn accel_core_exit() {
}

#[cfg(not(feature = "CONFIG_DRM_ACCEL"))]
pub unsafe fn accel_core_init() -> i32 {
    /* Return 0 to allow drm_core_init to complete successfully */
    0
}

#[cfg(not(feature = "CONFIG_DRM_ACCEL"))]
pub unsafe fn accel_set_device_instance_params(_kdev: *mut device, _index: i32) {
}

#[cfg(not(feature = "CONFIG_DRM_ACCEL"))]
pub unsafe fn accel_debugfs_register(_dev: *mut drm_device) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
