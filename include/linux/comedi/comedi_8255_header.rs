/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi_8255.h
 * Generic 8255 digital I/O subdevice support
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

// C header dependency: <linux/errno.h>

pub const I8255_SIZE: u32 = 0x04;

pub const I8255_DATA_A_REG: u32 = 0x00;
pub const I8255_DATA_B_REG: u32 = 0x01;
pub const I8255_DATA_C_REG: u32 = 0x02;
pub const I8255_CTRL_REG: u32 = 0x03;
pub const I8255_CTRL_C_LO_IO: u32 = 1u32 << 0;
pub const I8255_CTRL_B_IO: u32 = 1u32 << 1;
pub const I8255_CTRL_B_MODE: u32 = 1u32 << 2;
pub const I8255_CTRL_C_HI_IO: u32 = 1u32 << 3;
pub const I8255_CTRL_A_IO: u32 = 1u32 << 4;

#[inline]
pub const fn I8255_CTRL_A_MODE(x: u32) -> u32 {
    x << 5
}

pub const I8255_CTRL_CW: u32 = 1u32 << 7;

#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_subdevice {
    _private: [u8; 0],
}

// CONFIG_HAS_IOPORT controls whether the external implementation is provided.
#[cfg(CONFIG_HAS_IOPORT)]
unsafe extern "C" {
    pub fn subdev_8255_io_init(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        regbase: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_HAS_IOPORT))]
#[inline]
pub unsafe fn subdev_8255_io_init(
    _dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _regbase: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    // -ENXIO from Linux errno.h.
    -6
}

unsafe extern "C" {
    pub fn subdev_8255_mm_init(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        regbase: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn subdev_8255_cb_init(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        io: Option<unsafe extern "C" fn(
            dev: *mut comedi_device,
            dir: ::core::ffi::c_int,
            port: ::core::ffi::c_int,
            data: ::core::ffi::c_int,
            context: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int>,
        context: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn subdev_8255_regbase(s: *mut comedi_subdevice) -> ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
