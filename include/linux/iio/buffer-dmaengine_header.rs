/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2014-2015 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

/* Translated from linux/iio/buffer-dmaengine.h. */

use core::ffi::c_char;

/* Types supplied by linux/iio/buffer.h and other dependencies. */
#[repr(C)]
pub struct iio_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

pub type iio_buffer_direction = i32;

pub const IIO_BUFFER_DIRECTION_IN: iio_buffer_direction = 0;

unsafe extern "C" {
    pub fn iio_dmaengine_buffer_teardown(buffer: *mut iio_buffer);

    pub fn iio_dmaengine_buffer_setup_ext(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        channel: *const c_char,
        dir: iio_buffer_direction,
    ) -> *mut iio_buffer;

    pub fn devm_iio_dmaengine_buffer_setup_ext(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        channel: *const c_char,
        dir: iio_buffer_direction,
    ) -> i32;

    pub fn devm_iio_dmaengine_buffer_setup_with_handle(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        chan: *mut dma_chan,
        dir: iio_buffer_direction,
    ) -> i32;
}

#[inline]
pub unsafe fn iio_dmaengine_buffer_setup(
    dev: *mut device,
    indio_dev: *mut iio_dev,
    channel: *const c_char,
) -> *mut iio_buffer {
    unsafe {
        iio_dmaengine_buffer_setup_ext(
            dev,
            indio_dev,
            channel,
            IIO_BUFFER_DIRECTION_IN,
        )
    }
}

#[inline]
pub unsafe fn devm_iio_dmaengine_buffer_setup(
    dev: *mut device,
    indio_dev: *mut iio_dev,
    channel: *const c_char,
) -> i32 {
    unsafe {
        devm_iio_dmaengine_buffer_setup_ext(
            dev,
            indio_dev,
            channel,
            IIO_BUFFER_DIRECTION_IN,
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
