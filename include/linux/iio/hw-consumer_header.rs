/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Industrial I/O in kernel hardware consumer interface
 *
 * Copyright 2017 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependency: `struct device` is supplied by the surrounding kernel code.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_hw_consumer {
    _private: [u8; 0],
}

extern "C" {
    pub fn iio_hw_consumer_alloc(dev: *mut device) -> *mut iio_hw_consumer;
    pub fn iio_hw_consumer_free(hwc: *mut iio_hw_consumer);
    pub fn devm_iio_hw_consumer_alloc(dev: *mut device) -> *mut iio_hw_consumer;
    pub fn iio_hw_consumer_enable(hwc: *mut iio_hw_consumer) -> i32;
    pub fn iio_hw_consumer_disable(hwc: *mut iio_hw_consumer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
