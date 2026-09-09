/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * i2c-mux.h - functions for the i2c-bus mux support
 *
 * Copyright (c) 2008-2009 Rodolfo Giometti <giometti@linux.it>
 * Copyright (c) 2008-2009 Eurotech S.p.A. <info@eurotech.it>
 * Michael Lawnick <michael.lawnick.ext@nsn.com>
 */

/* Original header guard: _LINUX_I2C_MUX_H */
/* Original declarations are kernel-only (__KERNEL__). */

use core::ffi::c_void;

/* External types supplied by other headers. */
#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct i2c_mux_core {
    pub parent: *mut i2c_adapter,
    pub dev: *mut device,
    /* C bit-fields: mux_locked:1, arbitrator:1, gate:1. */
    pub mux_locked: u32,
    pub arbitrator: u32,
    pub gate: u32,
    pub priv_: *mut c_void,
    pub select: Option<unsafe extern "C" fn(*mut i2c_mux_core, u32) -> i32>,
    pub deselect: Option<unsafe extern "C" fn(*mut i2c_mux_core, u32) -> i32>,
    pub num_adapters: i32,
    pub max_adapters: i32,
    pub adapter: [*mut i2c_adapter; 0],
}

unsafe extern "C" {
    pub fn i2c_mux_alloc(
        parent: *mut i2c_adapter,
        dev: *mut device,
        max_adapters: i32,
        sizeof_priv: i32,
        flags: u32,
        select: Option<unsafe extern "C" fn(*mut i2c_mux_core, u32) -> i32>,
        deselect: Option<unsafe extern "C" fn(*mut i2c_mux_core, u32) -> i32>,
    ) -> *mut i2c_mux_core;
}

/* flags for i2c_mux_alloc */
pub const I2C_MUX_LOCKED: u32 = 1 << 0;
pub const I2C_MUX_ARBITRATOR: u32 = 1 << 1;
pub const I2C_MUX_GATE: u32 = 1 << 2;

pub unsafe fn i2c_mux_priv(muxc: *mut i2c_mux_core) -> *mut c_void {
    unsafe { (*muxc).priv_ }
}

unsafe extern "C" {
    pub fn i2c_root_adapter(dev: *mut device) -> *mut i2c_adapter;
}

/*
 * Called to create an i2c bus on a multiplexed bus segment.
 * The chan_id parameter is passed to the select and deselect
 * callback functions to perform hardware-specific mux control.
 */
unsafe extern "C" {
    pub fn i2c_mux_add_adapter(
        muxc: *mut i2c_mux_core,
        force_nr: u32,
        chan_id: u32,
    ) -> i32;

    pub fn i2c_mux_del_adapters(muxc: *mut i2c_mux_core);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
