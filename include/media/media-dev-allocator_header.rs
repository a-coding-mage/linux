/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * media-dev-allocator.h - Media Controller Device Allocator API
 *
 * Copyright (c) 2019 Shuah Khan <shuah@kernel.org>
 *
 * Credits: Suggested by Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

/*
 * This file adds a global ref-counted Media Controller Device Instance API.
 * A system wide global media device list is managed and each media device
 * includes a kref count. The last put on the media device releases the
 * media device instance.
 */

// C header guard: _MEDIA_DEV_ALLOCATOR_H

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

// Equivalent to: defined(CONFIG_MEDIA_CONTROLLER) && IS_ENABLED(CONFIG_USB)
#[cfg(all(feature = "CONFIG_MEDIA_CONTROLLER", feature = "CONFIG_USB"))]
extern "C" {
    /**
     * media_device_usb_allocate() - Allocate and return struct &media device
     *
     * @udev:        struct &usb_device pointer
     * @module_name: should be filled with %KBUILD_MODNAME
     * @owner:       struct module pointer %THIS_MODULE for the driver.
     *               %THIS_MODULE is null for a built-in driver.
     *               It is safe even when %THIS_MODULE is null.
     *
     * This interface should be called to allocate a Media Device when multiple
     * drivers share usb_device and the media device. This interface allocates
     * &media_device structure and calls media_device_usb_init() to initialize
     * it.
     */
    pub fn media_device_usb_allocate(
        udev: *mut usb_device,
        module_name: *const ::core::ffi::c_char,
        owner: *mut module,
    ) -> *mut media_device;

    /**
     * media_device_delete() - Release media device. Calls kref_put().
     *
     * @mdev:        struct &media_device pointer
     * @module_name: should be filled with %KBUILD_MODNAME
     * @owner:       struct module pointer %THIS_MODULE for the driver.
     *               %THIS_MODULE is null for a built-in driver.
     *               It is safe even when %THIS_MODULE is null.
     *
     * This interface should be called to put Media Device Instance kref.
     */
    pub fn media_device_delete(
        mdev: *mut media_device,
        module_name: *const ::core::ffi::c_char,
        owner: *mut module,
    );
}

#[cfg(not(all(feature = "CONFIG_MEDIA_CONTROLLER", feature = "CONFIG_USB")))]
#[inline]
pub unsafe fn media_device_usb_allocate(
    _udev: *mut usb_device,
    _module_name: *const ::core::ffi::c_char,
    _owner: *mut module,
) -> *mut media_device {
    core::ptr::null_mut()
}

#[cfg(not(all(feature = "CONFIG_MEDIA_CONTROLLER", feature = "CONFIG_USB")))]
#[inline]
pub unsafe fn media_device_delete(
    _mdev: *mut media_device,
    _module_name: *const ::core::ffi::c_char,
    _owner: *mut module,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
