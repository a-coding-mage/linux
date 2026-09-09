/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026 Renesas Electronics Corp.
 * Copyright (C) 2026 Ideas on Board Oy
 * Copyright (C) 2026 Ragnatech AB
 */

//! Rust translation of the C header `rppx1.h`.
//!
//! The referenced kernel/media types are supplied by other translation units.

use core::ffi::c_void;

#[repr(C)]
pub struct rppx1 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct v4l2_mbus_framefmt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vb2_buffer {
    _private: [u8; 0],
}

pub type rppx1_reg_write = unsafe extern "C" fn(
    priv_: *mut c_void,
    offset: u32,
    value: u32,
) -> i32;

extern "C" {
    pub fn rppx1_create(base: *mut c_void, dev: *mut device) -> *mut rppx1;

    pub fn rppx1_destroy(rpp: *mut rppx1);

    pub fn rppx1_start(
        rpp: *mut rppx1,
        input: *const v4l2_mbus_framefmt,
        hv: *const v4l2_mbus_framefmt,
        mv: *const v4l2_mbus_framefmt,
    ) -> i32;

    pub fn rppx1_stop(rpp: *mut rppx1) -> i32;

    pub fn rppx1_interrupt(rpp: *mut rppx1, isc: *mut u32) -> bool;

    pub fn rppx1_params(
        rpp: *mut rppx1,
        vb: *mut vb2_buffer,
        max_size: usize,
        write: rppx1_reg_write,
        priv_: *mut c_void,
    ) -> i32;

    pub fn rppx1_stats_fill_isr(rpp: *mut rppx1, isc: u32, buf: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
