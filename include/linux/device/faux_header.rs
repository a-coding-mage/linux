/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
 * Copyright (c) 2025 The Linux Foundation
 *
 * A "simple" faux bus that allows devices to be created and added
 * automatically to it.  This is to be used whenever you need to create a
 * device that is not associated with any "real" system resources, and do
 * not want to have to deal with a bus/driver binding logic.  It is
 * intended to be very simple, with only a create and a destroy function
 * available.
 */

// Dependencies supplied by the surrounding kernel translation.

/// A simple faux device that can be created/destroyed.
#[repr(C)]
pub struct faux_device {
    pub dev: device,
}

/// Equivalent to container_of_const(x, struct faux_device, dev).
#[inline]
pub unsafe fn to_faux_device(x: *const device) -> *const faux_device {
    x.cast::<u8>().sub(std::mem::offset_of!(faux_device, dev)).cast()
}

/// A set of callbacks for a faux_device.
#[repr(C)]
pub struct faux_device_ops {
    pub probe: Option<unsafe extern "C" fn(faux_dev: *mut faux_device) -> ::std::os::raw::c_int>,
    pub remove: Option<unsafe extern "C" fn(faux_dev: *mut faux_device)>,
}

unsafe extern "C" {
    pub fn faux_device_create(
        name: *const ::std::os::raw::c_char,
        parent: *mut device,
        faux_ops: *const faux_device_ops,
    ) -> *mut faux_device;
    pub fn faux_device_create_with_groups(
        name: *const ::std::os::raw::c_char,
        parent: *mut device,
        faux_ops: *const faux_device_ops,
        groups: *const *const attribute_group,
    ) -> *mut faux_device;
    pub fn faux_device_destroy(faux_dev: *mut faux_device);

    fn dev_get_drvdata(dev: *const device) -> *mut ::std::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut ::std::ffi::c_void);
}

#[inline]
pub unsafe fn faux_device_get_drvdata(faux_dev: *const faux_device) -> *mut ::std::ffi::c_void {
    dev_get_drvdata(&raw const (*faux_dev).dev)
}

#[inline]
pub unsafe fn faux_device_set_drvdata(
    faux_dev: *mut faux_device,
    data: *mut ::std::ffi::c_void,
) {
    dev_set_drvdata(&raw mut (*faux_dev).dev, data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
