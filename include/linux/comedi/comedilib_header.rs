/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedilib.h
 * Header file for kcomedilib
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998-2001 David A. Schleef <ds@schleef.org>
 */

// C dependency declaration: `struct comedi_device` is supplied externally.
#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn comedi_open_from(path: *const ::core::ffi::c_char, from: ::core::ffi::c_int)
        -> *mut comedi_device;
}

/**
 * comedi_open() - Open a COMEDI device from the kernel
 * @filename: Fake pathname of the form "/dev/comediN".
 *
 * Converts @filename to a COMEDI device number and "opens" it if it exists
 * and is attached to a low-level COMEDI driver.
 *
 * Return: A pointer to the COMEDI device on success.
 * Return %NULL on failure.
 */
#[inline]
pub unsafe fn comedi_open(path: *const ::core::ffi::c_char) -> *mut comedi_device {
    comedi_open_from(path, -1)
}

unsafe extern "C" {
    pub fn comedi_close_from(dev: *mut comedi_device, from: ::core::ffi::c_int)
        -> ::core::ffi::c_int;

    pub fn comedi_dio_get_config(
        dev: *mut comedi_device,
        subdev: ::core::ffi::c_uint,
        chan: ::core::ffi::c_uint,
        io: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn comedi_dio_config(
        dev: *mut comedi_device,
        subdev: ::core::ffi::c_uint,
        chan: ::core::ffi::c_uint,
        io: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn comedi_dio_bitfield2(
        dev: *mut comedi_device,
        subdev: ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
        bits: *mut ::core::ffi::c_uint,
        base_channel: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn comedi_find_subdevice_by_type(
        dev: *mut comedi_device,
        type_: ::core::ffi::c_int,
        subd: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn comedi_get_n_channels(
        dev: *mut comedi_device,
        subdevice: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

/**
 * comedi_close() - Close a COMEDI device from the kernel
 * @dev: COMEDI device.
 *
 * Closes a COMEDI device previously opened by comedi_open().
 *
 * Returns: 0
 */
#[inline]
pub unsafe fn comedi_close(dev: *mut comedi_device) -> ::core::ffi::c_int {
    comedi_close_from(dev, -1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
