/* SPDX-License-Identifier: GPL-2.0 */

// `nitrox_device` is supplied by the translated `nitrox_dev.h` dependency.
// The declaration is kept opaque here because this header only uses pointers
// to the type.
#[repr(C)]
pub struct nitrox_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn nitrox_debugfs_init(ndev: *mut nitrox_device);
    pub fn nitrox_debugfs_exit(ndev: *mut nitrox_device);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn nitrox_debugfs_init(_ndev: *mut nitrox_device) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn nitrox_debugfs_exit(_ndev: *mut nitrox_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
