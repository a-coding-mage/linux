/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STMicroelectronics sensors spi library driver
 *
 * Copyright 2012-2013 STMicroelectronics Inc.
 *
 * Denis Ciocca <denis.ciocca@st.com>
 */

// Dependencies supplied by the Linux IIO and SPI subsystems.
#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

pub unsafe extern "C" fn st_sensors_spi_configure(
    indio_dev: *mut iio_dev,
    spi: *mut spi_device,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
