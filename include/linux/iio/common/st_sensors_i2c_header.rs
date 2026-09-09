/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STMicroelectronics sensors i2c library driver
 *
 * Copyright 2012-2013 STMicroelectronics Inc.
 *
 * Denis Ciocca <denis.ciocca@st.com>
 */

// C dependencies: <linux/i2c.h>, <linux/iio/common/st_sensors.h>

use core::ffi::c_int;

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn st_sensors_i2c_configure(
        indio_dev: *mut iio_dev,
        client: *mut i2c_client,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
