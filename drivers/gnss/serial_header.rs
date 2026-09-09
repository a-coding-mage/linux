/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generic serial GNSS receiver driver
 *
 * Copyright (C) 2018 Johan Hovold <johan@kernel.org>
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// Supplied by the corresponding kernel headers.
pub enum serdev_device {}
pub enum gnss_device {}
pub enum dev_pm_ops {}

// `speed_t` is supplied by <asm/termbits.h>.
pub type speed_t = c_uint;

#[repr(C)]
pub struct gnss_serial {
    pub serdev: *mut serdev_device,
    pub gdev: *mut gnss_device,
    pub speed: speed_t,
    pub ops: *const gnss_serial_ops,
    pub drvdata: [c_ulong; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gnss_serial_pm_state {
    GNSS_SERIAL_OFF,
    GNSS_SERIAL_ACTIVE,
    GNSS_SERIAL_STANDBY,
}

#[repr(C)]
pub struct gnss_serial_ops {
    pub set_power: Option<unsafe extern "C" fn(
        gserial: *mut gnss_serial,
        state: gnss_serial_pm_state,
    ) -> c_int>,
}

unsafe extern "C" {
    pub static gnss_serial_pm_ops: dev_pm_ops;

    pub fn gnss_serial_allocate(
        gserial: *mut serdev_device,
        data_size: usize,
    ) -> *mut gnss_serial;
    pub fn gnss_serial_free(gserial: *mut gnss_serial);

    pub fn gnss_serial_register(gserial: *mut gnss_serial) -> c_int;
    pub fn gnss_serial_deregister(gserial: *mut gnss_serial);
}

#[inline]
pub unsafe fn gnss_serial_get_drvdata(gserial: *mut gnss_serial) -> *mut c_void {
    unsafe { (*gserial).drvdata.as_mut_ptr().cast::<c_void>() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
