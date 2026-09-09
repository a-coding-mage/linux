/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) IBM Corporation 2023 */

// Dependencies supplied by the Linux and FSI master headers:
// linux/i2c.h, linux/mutex.h, and fsi-master.h.

use core::ffi::c_void;

/// Forward declaration supplied by the I2C subsystem.
pub struct i2c_client;

/// `struct fsi_master_i2cr`.
#[repr(C)]
pub struct fsi_master_i2cr {
    pub master: fsi_master,
    pub lock: mutex, /* protect HW access */
    pub client: *mut i2c_client,
}

/// Equivalent of `container_of(m, struct fsi_master_i2cr, master)`.
#[macro_export]
macro_rules! to_fsi_master_i2cr {
    ($m:expr) => {{
        unsafe {
            ($m as *mut u8).sub(core::mem::offset_of!(fsi_master_i2cr, master))
                as *mut fsi_master_i2cr
        }
    }};
}

extern "C" {
    pub fn fsi_master_i2cr_read(
        i2cr: *mut fsi_master_i2cr,
        addr: u32,
        data: *mut u64,
    ) -> i32;
    pub fn fsi_master_i2cr_write(
        i2cr: *mut fsi_master_i2cr,
        addr: u32,
        data: u64,
    ) -> i32;
}

#[inline]
pub unsafe fn is_fsi_master_i2cr(master: *mut fsi_master) -> bool {
    if !(*master).dev.parent.is_null()
        && (*(*master).dev.parent).type_ == &i2c_client_type as *const _
    {
        return true;
    }

    false
}

// Names below are provided by the included FSI master and Linux headers.
extern "C" {
    static i2c_client_type: c_void;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
