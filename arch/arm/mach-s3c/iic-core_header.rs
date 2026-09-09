/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - I2C Controller core functions
 */

/*
 * These functions are only for use with the core support code, such as
 * the cpu specific initialisation code.
 */

use core::ffi::c_char;

/* External device objects supplied by the platform support code. */
extern "C" {
    static mut s3c_device_i2c0: S3cI2cDevice;
    #[cfg(CONFIG_S3C_DEV_I2C1)]
    static mut s3c_device_i2c1: S3cI2cDevice;
}

#[repr(C)]
pub struct S3cI2cDevice {
    pub name: *mut c_char,
}

/* Re-define device name depending on support. */
#[inline]
pub unsafe fn s3c_i2c0_setname(name: *mut c_char) {
    /* Currently this device is always compiled in. */
    s3c_device_i2c0.name = name;
}

#[inline]
pub unsafe fn s3c_i2c1_setname(name: *mut c_char) {
    /* CONFIG_S3C_DEV_I2C1 controls whether this device is compiled in. */
    #[cfg(CONFIG_S3C_DEV_I2C1)]
    {
        s3c_device_i2c1.name = name;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
